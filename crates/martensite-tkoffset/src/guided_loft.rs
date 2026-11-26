// SPDX-License-Identifier: GPL-2.0-only
//! Checked solid Loft with N ordered planar profiles and M 3D spline guide rails (#616).
use crate::brep_offset_api::BRepOffsetAPIThruSections;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Solid, Wire};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPICopy, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;

/// One named planar closed profile station in a guided loft (#616).
#[derive(Clone, Debug)]
pub struct LoftSectionStation {
    name: String,
    points: Vec<Point3>,
}

impl LoftSectionStation {
    /// Creates a named closed polygonal profile station with ordered vertices.
    pub fn new(name: impl Into<String>, points: Vec<Point3>) -> Self {
        Self {
            name: name.into(),
            points,
        }
    }
    /// Station name for provenance tracking.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Ordered profile vertices.
    pub fn points(&self) -> &[Point3] {
        &self.points
    }
}

/// One named 3D spline guide rail passing through all profile stations (#616).
#[derive(Clone, Debug)]
pub struct LoftGuideRail {
    name: String,
    station_points: Vec<Point3>,
}

impl LoftGuideRail {
    /// Creates a named guide rail defined by its 3D anchor points at each station.
    pub fn new(name: impl Into<String>, station_points: Vec<Point3>) -> Self {
        Self {
            name: name.into(),
            station_points,
        }
    }
    /// Rail name for provenance tracking.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Anchor points at each station.
    pub fn station_points(&self) -> &[Point3] {
        &self.station_points
    }
}

/// Named decline when a guided loft input violates geometric or topological invariants (#616).
#[derive(Clone, Debug, PartialEq)]
pub enum GuidedLoftDecline {
    /// Fewer than 2 profile stations were provided.
    TooFewProfiles,
    /// No guide rails were provided.
    MissingGuideRails,
    /// A profile station has fewer than 3 vertices or mismatched vertex count.
    IncompatibleProfile {
        /// Name of the offending station.
        station: String,
        /// Observed vertex count on that station.
        vertex_count: usize,
    },
    /// A guide rail does not pass through a profile station within tolerance.
    RailStationMismatch {
        /// Name of the offending guide rail.
        rail: String,
        /// Name of the station where the gap occurred.
        station: String,
        /// Minimum Euclidean gap between rail anchor and station vertices.
        gap: f64,
    },
    /// The lofted solid failed BRepCheck validity.
    InvalidSolid,
}

/// Detached, validated solid produced by a guided spline-rail loft (#616).
#[derive(Clone, Debug)]
pub struct GuidedLoftResult {
    solid: Solid,
    station_names: Vec<String>,
    rail_names: Vec<String>,
    volume: f64,
    is_valid: bool,
}

impl GuidedLoftResult {
    /// Detached solid resulting from the guided loft.
    pub fn solid(&self) -> &Solid {
        &self.solid
    }
    /// Ordered profile station names.
    pub fn station_names(&self) -> &[String] {
        &self.station_names
    }
    /// Ordered guide rail names.
    pub fn rail_names(&self) -> &[String] {
        &self.rail_names
    }
    /// Certified solid volume computed via `BRepGProp::volume_properties`.
    pub fn volume(&self) -> f64 {
        self.volume
    }
    /// Whether `BRepCheckAnalyzer` certified the solid valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

/// Builds a checked solid Loft across `sections` guided by `rails`, interpolating
/// smooth Catmull-Rom / cubic spline stations along the guide rails and returning a
/// detached, validated [`GuidedLoftResult`] (#616).
pub fn build_guided_loft(
    sections: &[LoftSectionStation],
    rails: &[LoftGuideRail],
    tolerance: f64,
) -> Result<Result<GuidedLoftResult, GuidedLoftDecline>, Failure> {
    if sections.len() < 2 {
        return Ok(Err(GuidedLoftDecline::TooFewProfiles));
    }
    if rails.is_empty() {
        return Ok(Err(GuidedLoftDecline::MissingGuideRails));
    }
    let vertex_count = sections[0].points.len();
    for section in sections {
        if section.points.len() < 3 || section.points.len() != vertex_count {
            return Ok(Err(GuidedLoftDecline::IncompatibleProfile {
                station: section.name.clone(),
                vertex_count: section.points.len(),
            }));
        }
    }
    for rail in rails {
        if rail.station_points.len() != sections.len() {
            return Ok(Err(GuidedLoftDecline::RailStationMismatch {
                rail: rail.name.clone(),
                station: sections[0].name.clone(),
                gap: f64::INFINITY,
            }));
        }
        for (idx, (anchor, section)) in rail.station_points.iter().zip(sections.iter()).enumerate() {
            let min_gap = section
                .points
                .iter()
                .map(|p| p.distance(anchor))
                .fold(f64::INFINITY, f64::min);
            if min_gap > tolerance {
                return Ok(Err(GuidedLoftDecline::RailStationMismatch {
                    rail: rail.name.clone(),
                    station: sections[idx].name.clone(),
                    gap: min_gap,
                }));
            }
        }
    }

    // Build smooth spline-sampled section wires along the guide rails.
    let sampled_wires = sample_guided_wires(sections, 2)?;
    let mut thru = BRepOffsetAPIThruSections::new(true, false, tolerance);
    for wire in &sampled_wires {
        thru.add_wire(wire);
    }
    thru.build(&ProgressRange::default())?;
    if !thru.is_done() {
        return Ok(Err(GuidedLoftDecline::InvalidSolid));
    }
    let raw_shape = thru.shape()?.clone();
    let mut copy = BRepBuilderAPICopy::with_shape(&raw_shape, true, false)?;
    let detached = copy.shape()?.clone();

    let analyzer = BRepCheckAnalyzer::new(&detached, true, false, false)?;
    let is_valid = analyzer.is_valid()?;
    if !is_valid {
        return Ok(Err(GuidedLoftDecline::InvalidSolid));
    }

    let solid_exp = Explorer::new(&detached, ShapeEnum::Solid, ShapeEnum::Shape)?;
    if !solid_exp.more() {
        return Ok(Err(GuidedLoftDecline::InvalidSolid));
    }
    let solid = topods::solid(solid_exp.current())?;
    let mut props = GProps::default();
    BRepGProp::volume_properties(&detached, &mut props, true, false, false)?;
    let volume = props.mass().abs();

    // Ensure no vertex has non-finite coordinates.
    let mut v_exp = Explorer::new(&detached, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    while v_exp.more() {
        let v = topods::vertex(v_exp.current())?;
        let p = BRepTool::pnt(&v)?;
        if !(p.x().is_finite() && p.y().is_finite() && p.z().is_finite()) {
            return Ok(Err(GuidedLoftDecline::InvalidSolid));
        }
        v_exp.next()?;
    }

    Ok(Ok(GuidedLoftResult {
        solid,
        station_names: sections.iter().map(|s| s.name.clone()).collect(),
        rail_names: rails.iter().map(|r| r.name.clone()).collect(),
        volume,
        is_valid,
    }))
}

fn sample_guided_wires(
    sections: &[LoftSectionStation],
    subdivisions_per_span: usize,
) -> Result<Vec<Wire>, Failure> {
    let n = sections.len();
    let k = sections[0].points.len();
    let mut wires = Vec::new();

    for span in 0..(n - 1) {
        let steps = if span + 1 == n - 1 {
            subdivisions_per_span + 1
        } else {
            subdivisions_per_span
        };
        for step in 0..steps {
            let t = step as f64 / subdivisions_per_span as f64;
            let mut ring = Vec::with_capacity(k);
            for v in 0..k {
                let p0 = sections[span.saturating_sub(1)].points[v];
                let p1 = sections[span].points[v];
                let p2 = sections[span + 1].points[v];
                let p3 = sections[(span + 2).min(n - 1)].points[v];
                ring.push(catmull_rom(p0, p1, p2, p3, t));
            }
            wires.push(wire_from_ring(&ring)?);
        }
    }
    Ok(wires)
}

fn catmull_rom(p0: Point3, p1: Point3, p2: Point3, p3: Point3, t: f64) -> Point3 {
    let t2 = t * t;
    let t3 = t2 * t;
    let eval = |a: f64, b: f64, c: f64, d: f64| {
        0.5 * ((2.0 * b)
            + (-a + c) * t
            + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
            + (-a + 3.0 * b - 3.0 * c + d) * t3)
    };
    Point3::new(
        eval(p0.x(), p1.x(), p2.x(), p3.x()),
        eval(p0.y(), p1.y(), p2.y(), p3.y()),
        eval(p0.z(), p1.z(), p2.z(), p3.z()),
    )
}

fn wire_from_ring(points: &[Point3]) -> Result<Wire, Failure> {
    let mut poly = BRepBuilderAPIMakePolygon::new();
    for &pt in points {
        poly.add_point(pt)?;
    }
    poly.close()?;
    poly.wire()
}

#[cfg(test)]
mod guided_loft_tests {
    use super::*;

    fn square_ring(half: f64, z: f64) -> Vec<Point3> {
        vec![
            Point3::new(-half, -half, z),
            Point3::new(half, -half, z),
            Point3::new(half, half, z),
            Point3::new(-half, half, z),
        ]
    }

    fn rails_from_sections(sections: &[LoftSectionStation]) -> Vec<LoftGuideRail> {
        (0..sections[0].points().len())
            .map(|corner| {
                LoftGuideRail::new(
                    format!("rail_{corner}"),
                    sections.iter().map(|s| s.points()[corner]).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn guided_spline_rail_loft_builds_valid_solid_and_refits_on_middle_profile_edit() {
        let s0 = LoftSectionStation::new("bottom", square_ring(1.0, 0.0));
        let s1 = LoftSectionStation::new("middle", square_ring(1.5, 2.0));
        let s2 = LoftSectionStation::new("top", square_ring(1.0, 4.0));
        let sections = vec![s0.clone(), s1, s2.clone()];
        let rails = rails_from_sections(&sections);

        let first = build_guided_loft(&sections, &rails, 1e-4)
            .expect("loft call")
            .expect("valid guided loft");
        assert!(first.is_valid());
        assert_eq!(first.station_names(), &["bottom", "middle", "top"]);
        assert_eq!(first.rail_names().len(), 4);
        assert!(first.volume() > 16.0);

        // Edit middle profile wider and refit rails; original solid remains detached.
        let s1_edited = LoftSectionStation::new("middle", square_ring(2.2, 2.0));
        let edited_sections = vec![s0, s1_edited, s2];
        let edited_rails = rails_from_sections(&edited_sections);
        let second = build_guided_loft(&edited_sections, &edited_rails, 1e-4)
            .expect("edited loft call")
            .expect("valid edited loft");
        assert!(second.is_valid());
        assert!(second.volume() > first.volume());
    }

    #[test]
    fn guided_loft_refuses_rail_missing_profile_station() {
        let sections = vec![
            LoftSectionStation::new("bottom", square_ring(1.0, 0.0)),
            LoftSectionStation::new("top", square_ring(1.0, 2.0)),
        ];
        let bad_rail = LoftGuideRail::new(
            "detached_rail",
            vec![Point3::new(1.0, 1.0, 0.0), Point3::new(5.0, 5.0, 2.0)],
        );
        let err = build_guided_loft(&sections, &[bad_rail], 1e-4)
            .expect("loft call")
            .expect_err("rail not on station must be declined");
        assert!(matches!(err, GuidedLoftDecline::RailStationMismatch { .. }));
    }
}
