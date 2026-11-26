// SPDX-License-Identifier: GPL-2.0-only
//! Validated tapered profile extrusion (`extrude_tapered`) with cancellation and
//! exact frustum volume certification (#463).
use super::admission::admit_resolution;
use super::decline::{
    DeclineCause, Decliner, OperationDecline, OperationKind, OperationStage as Stage,
};
use super::extrude::{ExtrudeOptions, extrude};
use super::plane_math::{dot3, norm3};
use super::regions::PlanarRegion;
use super::resolution::Resolution;
use super::solid_validation::check_solid;
use crate::body_check::ValidationReport;
use crate::measured::{IntegrationMethod, Measured};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shell, Solid, vertex};
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPISewing,
};
use std::f64::consts::FRAC_PI_2;
use std::sync::atomic::{AtomicBool, Ordering};

const DECLINE: Decliner = Decliner {
    operation: OperationKind::Extrude,
};

/// Options for tapered profile extrusion (#463).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TaperedExtrudeOptions {
    /// Explicit modelling resolution and tolerance cap.
    pub resolution: Resolution,
    /// Draft / taper angle in radians; positive widens outward, negative narrows inward.
    pub taper_angle: f64,
}

/// Validated tapered extrusion result (#463).
#[derive(Clone, Debug)]
pub struct TaperedExtrudeResult {
    solid: Solid,
    validation: ValidationReport,
    volume: Measured,
    taper_angle: f64,
    resolution: Resolution,
}

impl TaperedExtrudeResult {
    /// Detached, validated solid produced by tapered extrusion.
    pub fn solid(&self) -> &Solid {
        &self.solid
    }
    /// Ordered B-rep validation report.
    pub fn validation(&self) -> &ValidationReport {
        &self.validation
    }
    /// Certified frustum volume.
    pub fn volume(&self) -> Measured {
        self.volume
    }
    /// Applied taper angle in radians.
    pub fn taper_angle(&self) -> f64 {
        self.taper_angle
    }
    /// Modelling resolution used.
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }
}

/// Sweeps a planar polygonal `region` along `direction` with a uniform `taper_angle`,
/// checking cancellation, self-intersection collapse, and B-rep validity (#463).
pub fn extrude_tapered(
    region: &PlanarRegion,
    direction: [f64; 3],
    options: &TaperedExtrudeOptions,
    cancel: Option<&AtomicBool>,
) -> Result<TaperedExtrudeResult, OperationDecline> {
    if cancel.is_some_and(|flag| flag.load(Ordering::Acquire)) {
        return Err(DECLINE.at(
            Stage::Admission,
            DeclineCause::InvalidDirection {
                reason: "cancelled before tapered extrusion",
                direction,
            },
        ));
    }
    let resolution = options.resolution;
    admit_resolution(&resolution).map_err(|c| DECLINE.at(Stage::Admission, c))?;
    if !options.taper_angle.is_finite()
        || options.taper_angle.abs() >= FRAC_PI_2 - resolution.angular
    {
        return Err(DECLINE.at(
            Stage::Admission,
            DeclineCause::InvalidResolution {
                field: "taper_angle",
                value: options.taper_angle,
            },
        ));
    }
    let length = norm3(direction);
    if !length.is_finite() || length < resolution.weld {
        return Err(DECLINE.at(
            Stage::Admission,
            DeclineCause::InvalidDirection {
                reason: "direction shorter than weld tolerance",
                direction,
            },
        ));
    }
    let height = dot3(direction, region.normal());
    if height.abs() < length * martensite_cmath::sin(resolution.angular) {
        return Err(DECLINE.at(
            Stage::Admission,
            DeclineCause::InvalidDirection {
                reason: "direction within angular tolerance of region plane",
                direction,
            },
        ));
    }
    if options.taper_angle.abs() <= resolution.angular {
        let straight = extrude(region, direction, &ExtrudeOptions { resolution })?;
        return Ok(TaperedExtrudeResult {
            solid: straight.solid,
            validation: straight.validation,
            volume: straight.volume,
            taper_angle: 0.0,
            resolution,
        });
    }

    let bottom_pts = ordered_outer_vertices(region, &resolution)?;
    let (centroid, min_radius) = polygon_centroid_and_min_radius(&bottom_pts);
    let radial_delta = height.abs() * martensite_cmath::tan(options.taper_angle);
    if min_radius + radial_delta <= resolution.weld {
        return Err(DECLINE.at(
            Stage::Admission,
            DeclineCause::InvalidDirection {
                reason: "inward taper collapses profile before reaching top cap",
                direction,
            },
        ));
    }

    let scale = (min_radius + radial_delta) / min_radius;
    let top_pts: Vec<[f64; 3]> = bottom_pts
        .iter()
        .map(|p| {
            [
                centroid[0] + (p[0] - centroid[0]) * scale + direction[0],
                centroid[1] + (p[1] - centroid[1]) * scale + direction[1],
                centroid[2] + (p[2] - centroid[2]) * scale + direction[2],
            ]
        })
        .collect();

    if cancel.is_some_and(|flag| flag.load(Ordering::Acquire)) {
        return Err(DECLINE.at(
            Stage::Construction,
            DeclineCause::InvalidDirection {
                reason: "cancelled during tapered extrusion",
                direction,
            },
        ));
    }

    let solid = build_frustum_solid(&bottom_pts, &top_pts, resolution.sew)?;
    let checked = check_solid(&solid, resolution, DECLINE)?;
    let a0 = region.area().value.abs();
    let a1 = a0 * scale * scale;
    let frustum_vol = (height.abs() / 3.0) * (a0 + a1 + (a0 * a1).sqrt());
    let bound = (frustum_vol * 1e-12).max(resolution.sew);
    let volume = Measured {
        value: frustum_vol,
        error_bound: bound,
        method: IntegrationMethod::ClosedForm,
    };
    Ok(TaperedExtrudeResult {
        solid,
        validation: checked.validation,
        volume,
        taper_angle: options.taper_angle,
        resolution,
    })
}

fn ordered_outer_vertices(
    region: &PlanarRegion,
    resolution: &Resolution,
) -> Result<Vec<[f64; 3]>, OperationDecline> {
    let mut pts = Vec::new();
    let mut explorer = Explorer::new(region.face(), ShapeEnum::Vertex, ShapeEnum::Shape)
        .map_err(|f| DECLINE.native(Stage::Admission, "explore region vertices", f))?;
    while explorer.more() {
        let v = vertex(explorer.current())
            .map_err(|f| DECLINE.native(Stage::Admission, "cast vertex", f))?;
        let p =
            BRepTool::pnt(&v).map_err(|f| DECLINE.native(Stage::Admission, "vertex point", f))?;
        let coords = [p.x(), p.y(), p.z()];
        let duplicate = pts.iter().any(|existing: &[f64; 3]| {
            let dx = existing[0] - coords[0];
            let dy = existing[1] - coords[1];
            let dz = existing[2] - coords[2];
            (dx * dx + dy * dy + dz * dz).sqrt() <= resolution.weld
        });
        if !duplicate {
            pts.push(coords);
        }
        explorer
            .next()
            .map_err(|f| DECLINE.native(Stage::Admission, "next vertex", f))?;
    }
    if pts.len() < 3 {
        return Err(DECLINE.at(
            Stage::Admission,
            DeclineCause::InvalidResolution {
                field: "profile.vertex_count",
                value: pts.len() as f64,
            },
        ));
    }
    // Sort polygon vertices counter-clockwise around their centroid in the region plane.
    let (centroid, _) = polygon_centroid_and_min_radius(&pts);
    let n = region.normal();
    let u = unit_perp(n);
    let v = cross3(n, u);
    pts.sort_by(|a, b| {
        let da = [a[0] - centroid[0], a[1] - centroid[1], a[2] - centroid[2]];
        let db = [b[0] - centroid[0], b[1] - centroid[1], b[2] - centroid[2]];
        let ang_a = martensite_cmath::atan2(dot3(da, v), dot3(da, u));
        let ang_b = martensite_cmath::atan2(dot3(db, v), dot3(db, u));
        ang_a.total_cmp(&ang_b)
    });
    Ok(pts)
}

fn polygon_centroid_and_min_radius(pts: &[[f64; 3]]) -> ([f64; 3], f64) {
    let n = pts.len() as f64;
    let mut c = [0.0; 3];
    for p in pts {
        c[0] += p[0] / n;
        c[1] += p[1] / n;
        c[2] += p[2] / n;
    }
    let min_r = pts
        .iter()
        .map(|p| {
            let dx = p[0] - c[0];
            let dy = p[1] - c[1];
            let dz = p[2] - c[2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        })
        .fold(f64::INFINITY, f64::min);
    (c, min_r)
}

fn unit_perp(n: [f64; 3]) -> [f64; 3] {
    let ref_axis = if n[0].abs() < 0.9 {
        [1.0, 0.0, 0.0]
    } else {
        [0.0, 1.0, 0.0]
    };
    let c = cross3(n, ref_axis);
    let len = norm3(c);
    [c[0] / len, c[1] / len, c[2] / len]
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn make_planar_face(
    points: &[[f64; 3]],
) -> Result<martensite_tkbrep::topods::Face, OperationDecline> {
    let native = |f| DECLINE.native(Stage::Construction, "tapered face", f);
    let mut poly = BRepBuilderAPIMakePolygon::new();
    for p in points {
        poly.add_point(Point3::new(p[0], p[1], p[2]))
            .map_err(native)?;
    }
    poly.close().map_err(native)?;
    let wire = poly.wire().map_err(native)?;
    let face_maker = BRepBuilderAPIMakeFace::from_wire(&wire, true).map_err(native)?;
    face_maker.face().map_err(native)
}

fn build_frustum_solid(
    bottom: &[[f64; 3]],
    top: &[[f64; 3]],
    sew_tol: f64,
) -> Result<Solid, OperationDecline> {
    let native = |f| DECLINE.native(Stage::Construction, "sew tapered frustum", f);
    let mut sewing = BRepBuilderAPISewing::new(sew_tol, true, true, true, false);
    let bottom_rev: Vec<[f64; 3]> = bottom.iter().copied().rev().collect();
    sewing
        .add(&make_planar_face(&bottom_rev)?.into_shape())
        .map_err(native)?;
    sewing
        .add(&make_planar_face(top)?.into_shape())
        .map_err(native)?;
    let count = bottom.len();
    for i in 0..count {
        let next = (i + 1) % count;
        let quad = [bottom[i], bottom[next], top[next], top[i]];
        sewing
            .add(&make_planar_face(&quad)?.into_shape())
            .map_err(native)?;
    }
    sewing.perform(&ProgressRange::default()).map_err(native)?;
    let sewed = sewing.sewed_shape();
    let shell_explorer =
        Explorer::new(sewed, ShapeEnum::Shell, ShapeEnum::Shape).map_err(native)?;
    if !shell_explorer.more() {
        return Err(DECLINE.at(
            Stage::Construction,
            DeclineCause::InvalidDirection {
                reason: "sewing tapered faces did not produce a closed shell",
                direction: [0.0; 3],
            },
        ));
    }
    let shell: Shell = topods::shell(shell_explorer.current()).map_err(native)?;
    let mut solid = Solid::default();
    let builder = BRepBuilder::default();
    builder.make_solid(&mut solid);
    let mut solid_shape = solid.into_shape();
    builder
        .add(&mut solid_shape, &shell.into_shape())
        .map_err(native)?;
    topods::solid(&solid_shape).map_err(native)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile_extrusion::{
        DisplayTolerance, ProfileCurve, ProfileCurveId, ProfilePlane, ProfileVertex,
        ProfileVertexId, SketchProfile, ToleranceCap, build_planar_regions,
    };

    fn square_profile(half: f64) -> SketchProfile {
        SketchProfile {
            plane: ProfilePlane::world_xy(),
            vertices: vec![
                ProfileVertex {
                    id: ProfileVertexId(1),
                    point: [-half, -half],
                },
                ProfileVertex {
                    id: ProfileVertexId(2),
                    point: [half, -half],
                },
                ProfileVertex {
                    id: ProfileVertexId(3),
                    point: [half, half],
                },
                ProfileVertex {
                    id: ProfileVertexId(4),
                    point: [-half, half],
                },
            ],
            curves: vec![
                ProfileCurve::Line {
                    id: ProfileCurveId(1),
                    start: ProfileVertexId(1),
                    end: ProfileVertexId(2),
                },
                ProfileCurve::Line {
                    id: ProfileCurveId(2),
                    start: ProfileVertexId(2),
                    end: ProfileVertexId(3),
                },
                ProfileCurve::Line {
                    id: ProfileCurveId(3),
                    start: ProfileVertexId(3),
                    end: ProfileVertexId(4),
                },
                ProfileCurve::Line {
                    id: ProfileCurveId(4),
                    start: ProfileVertexId(4),
                    end: ProfileVertexId(1),
                },
            ],
        }
    }

    #[test]
    fn tapered_extrude_builds_positive_and_negative_frustums_and_declines_collapse() {
        let display = DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        };
        let resolution = Resolution::new(1e-6, 1e-5, 1e-9, display, ToleranceCap::Refuse(1e-3));
        let regions = build_planar_regions(&square_profile(1.0), resolution).expect("regions");
        let region = &regions.regions()[0];

        let outward = extrude_tapered(
            region,
            [0.0, 0.0, 2.0],
            &TaperedExtrudeOptions {
                resolution,
                taper_angle: 0.1,
            },
            None,
        )
        .expect("outward taper");
        assert!(outward.volume().value > 8.0);

        let inward = extrude_tapered(
            region,
            [0.0, 0.0, 2.0],
            &TaperedExtrudeOptions {
                resolution,
                taper_angle: -0.1,
            },
            None,
        )
        .expect("inward taper");
        assert!(inward.volume().value < 8.0);

        let collapsed = extrude_tapered(
            region,
            [0.0, 0.0, 10.0],
            &TaperedExtrudeOptions {
                resolution,
                taper_angle: -0.6,
            },
            None,
        );
        assert!(collapsed.is_err());

        let cancelled = AtomicBool::new(true);
        assert!(
            extrude_tapered(
                region,
                [0.0, 0.0, 2.0],
                &TaperedExtrudeOptions {
                    resolution,
                    taper_angle: 0.1,
                },
                Some(&cancelled),
            )
            .is_err()
        );
    }
}
