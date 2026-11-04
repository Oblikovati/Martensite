//! The point cloud of a shape's triangulation nodes, as an interactive object.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::precision;
use martensite_tkernel::standard::exceptions::program_error;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point2, Point3, Vector3, resolution};
use martensite_tkservice::graphic3d::Graphic3dArrayOfPoints;
use martensite_tktopalgo::brep_lib::{BRepLibPointCloudShape, BRepLibPointCloudShapeBase};
use martensite_tkv3d::ais::{AisPointCloud, AisPointCloudSource as _};

/// Fills a point array with each generated point and, when present, its normal.
struct NodeFiller {
    base: BRepLibPointCloudShapeBase,
    points: Handle<Graphic3dArrayOfPoints>,
    failure: Option<Failure>,
}

impl NodeFiller {
    fn try_add(&self, point: Point3, normal: Vector3, uv: Point2) -> Result<(), Failure> {
        let points = self.points.get().ok_or_else(|| {
            program_error("a null point array; expected one allocated before generation")
        })?;
        let index = points.add_vertex_pnt_texel(&point, &uv)?;
        if normal.square_magnitude() > resolution() {
            points.set_vertex_normal(index, normal.x(), normal.y(), normal.z())?;
        }
        point_color(points, index)?;
        Ok(())
    }
}

impl BRepLibPointCloudShape for NodeFiller {
    fn point_cloud_base(&self) -> &BRepLibPointCloudShapeBase {
        &self.base
    }
    fn point_cloud_base_mut(&mut self) -> &mut BRepLibPointCloudShapeBase {
        &mut self.base
    }
    fn add_point(&mut self, point: Point3, normal: Vector3, uv: Point2, _: &Shape) {
        if self.failure.is_none()
            && let Err(failure) = self.try_add(point, normal, uv)
        {
            self.failure = Some(failure);
        }
    }
}

/// The cloud of the shape's triangulation nodes with the array `flags` (normals, colours,
/// texels), printing the generated count as the cloud command does.
/// Example: `node_cloud(&face, GRAPHIC3D_ARRAY_FLAGS_NONE)?`.
pub(super) fn node_cloud(shape: &Shape, flags: i32) -> Result<Handle<AisPointCloud>, Failure> {
    let mut filler = NodeFiller {
        base: BRepLibPointCloudShapeBase::new(Shape::default(), precision::confusion()),
        points: Handle::default(),
        failure: None,
    };
    filler.set_shape(shape);
    filler.set_distance(0.0);
    let count = filler.nb_points_by_triangulation()?;
    println!("Number of the generated points : {count}");
    filler.points = Handle::new(Graphic3dArrayOfPoints::new(count, flags)?);
    let generated = filler.generate_points_by_triangulation()?;
    require_generation(&mut filler, generated, count)?;
    let cloud = AisPointCloud::new()?;
    cloud.set_points(&filler.points)?;
    Ok(cloud)
}

/// Shape-cloud colors are indexed HLS, not a random generator; `point_color(points, index)?`.
fn point_color(points: &Graphic3dArrayOfPoints, index: i32) -> Result<(), Failure> {
    if !points.has_vertex_colors() {
        return Ok(());
    }
    use martensite_tkernel::quantity::{Color, ColorSpace};
    let hue = 360.0 * f64::from(index) / f64::from(points.vertex_number_allocated());
    points.set_vertex_color(index, Color::new(hue, 1.0, 0.5, ColorSpace::Hls)?)
}

#[cfg(test)]
mod controls {
    use super::*;
    use martensite_tkservice::graphic3d::{
        GRAPHIC3D_ARRAY_FLAGS_NONE, GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR,
    };

    fn three_points(flags: i32) -> Handle<Graphic3dArrayOfPoints> {
        let points = Handle::new(Graphic3dArrayOfPoints::new(3, flags).unwrap());
        let filler = NodeFiller {
            base: BRepLibPointCloudShapeBase::new(Shape::default(), precision::confusion()),
            points: points.clone(),
            failure: None,
        };
        for index in 0..3 {
            filler
                .try_add(
                    Point3::new(f64::from(index), 0., 0.),
                    Vector3::default(),
                    Point2::default(),
                )
                .unwrap();
        }
        points
    }

    #[test]
    fn source_index_hues_span_green_blue_red_for_three_allocated_points() {
        let points = three_points(GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR);
        assert!(points.has_vertex_colors());
        // HLS(120/240/360,1,.5) gives sRGB .5 on non-dominant channels; inverse sRGB is .21404114.
        for (index, expected) in [
            (1, [0.21404114, 1., 0.21404114]),
            (2, [0.21404114, 0.21404114, 1.]),
            (3, [1., 0.21404114, 0.21404114]),
        ] {
            let actual = points.vertex_color_rgb(index).unwrap();
            for (actual, expected) in actual.into_iter().zip(expected) {
                assert!(
                    (actual - expected).abs() <= 1. / 255.,
                    "actual{actual}; expected HLS-derived{expected} rounded to one packed byte"
                );
            }
        }
    }

    #[test]
    fn generation_refusal_preserves_callback_error_before_not_done() {
        let mut filler = NodeFiller {
            base: BRepLibPointCloudShapeBase::new(Shape::default(), precision::confusion()),
            points: Handle::default(),
            failure: Some(Failure::with_message(
                "Standard_Failure",
                "callback sentinel",
            )),
        };
        let failure = require_generation(&mut filler, false, 17).unwrap_err();
        assert_eq!(failure.message(), "callback sentinel");
        let failure = require_generation(&mut filler, false, 17).unwrap_err();
        assert!(
            failure
                .message()
                .contains("17 triangulation nodes not generated")
        );
        assert!(require_generation(&mut filler, true, 17).is_ok());
    }

    #[test]
    fn unchanged_uncolored_cloud_path_keeps_positions_without_color_storage() {
        let points = three_points(GRAPHIC3D_ARRAY_FLAGS_NONE);
        assert!(!points.has_vertex_colors());
        for index in 1..=3 {
            assert_eq!(
                points.vertice_xyz(index).unwrap(),
                [f64::from(index - 1), 0., 0.]
            );
        }
    }
}

#[cfg(test)]
mod sphere_controls {
    use super::*;
    use martensite_tkservice::graphic3d::{
        GRAPHIC3D_ARRAY_FLAGS_NONE, GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL,
        GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL,
    };

    #[test]
    fn actual_sphere_cloud_modes_keep_identical_points_and_requested_channels() {
        let shape = super::super::sensitive_box::meshed_sphere_face().unwrap();
        let plain = node_cloud(&shape, GRAPHIC3D_ARRAY_FLAGS_NONE)
            .unwrap()
            .get_points();
        let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL | GRAPHIC3D_ARRAY_FLAGS_VERTEX_TEXEL;
        let textured = node_cloud(&shape, flags).unwrap().get_points();
        assert!(plain.vertex_number() > 0);
        assert_eq!(plain.vertex_number(), textured.vertex_number());
        assert!(!plain.has_vertex_normals() && !plain.has_vertex_texels());
        assert!(textured.has_vertex_normals() && textured.has_vertex_texels());
        for index in 1..=plain.vertex_number() {
            assert_eq!(
                plain.vertice_xyz(index).unwrap(),
                textured.vertice_xyz(index).unwrap()
            );
        }
    }
}

fn require_generation(filler: &mut NodeFiller, generated: bool, count: i32) -> Result<(), Failure> {
    if let Some(failure) = filler.failure.take() {
        return Err(failure);
    }
    if !generated {
        return Err(program_error(&format!(
            "point cloud of {count} triangulation nodes not generated; expected generated points"
        )));
    }
    Ok(())
}
