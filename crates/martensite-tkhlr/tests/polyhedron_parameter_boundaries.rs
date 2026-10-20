//! Explicit parameter grids retain the distinct hidden-line and intersection contracts.
use martensite_tkbrep::{brep::BRepBuilder, topods::Face};
use martensite_tkernel::{ncollection::Array1, standard::Handle};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkgeomalgo::int_curve_surface::{SamplePolyhedron, ThePolyhedronOfHInter};
use martensite_tkhlr::hlr_brep::{Surface, SurfaceTool, ThePolyhedronOfInterCSurf};
use martensite_tkmath::gp::{Axis3, Point3};

fn plane_at_height_three() -> Surface {
    let mut frame = Axis3::default();
    frame.set_location(Point3::new(0.0, 0.0, 3.0));
    let geometry: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &geometry, 1e-7)
        .unwrap();
    let mut surface = Surface::new();
    surface.set_surface(&face).unwrap();
    surface
}

fn parameters(lower: i32, count: i32) -> Array1<'static, f64> {
    if count == 0 {
        return Array1::default();
    }
    let mut values = Array1::with_bounds(lower, lower + count - 1).unwrap();
    for index in 0..count {
        values[lower + index] = f64::from(index);
    }
    values
}

#[test]
fn hidden_line_grids_accept_singleton_axes_and_keep_array_lower_bounds() {
    let surface = plane_at_height_three();
    for (nu, nv) in [(1, 1), (1, 2), (2, 1), (2, 2)] {
        let u = parameters(-3, nu);
        let v = parameters(7, nv);
        let poly =
            <ThePolyhedronOfInterCSurf as SamplePolyhedron<SurfaceTool<'_>>>::with_parameters(
                &&surface, &u, &v,
            )
            .unwrap();
        assert_eq!(poly.nb_triangles(), 2 * (nu - 1) * (nv - 1));
        assert_eq!(poly.components_bounding().length(), poly.nb_triangles());
        assert_grid_points(&poly, nu, nv);
    }
}

fn assert_grid_points(poly: &ThePolyhedronOfInterCSurf, nu: i32, nv: i32) {
    assert_eq!(poly.nb_points(), nu * nv);
    for row in 0..nu {
        for column in 0..nv {
            let index = row * nv + column + 1;
            let (point, u, v) = poly.point_with_parameters(index).unwrap();
            assert_eq!((u, v), (f64::from(row), f64::from(column)));
            assert_eq!(point, Point3::new(u, v, 3.0));
        }
    }
}

#[test]
fn empty_hidden_line_parameter_axes_raise_the_array_access_failure() {
    let surface = plane_at_height_three();
    for (nu, nv) in [(0, 1), (1, 0), (0, 0), (0, 2), (2, 0)] {
        let result =
            <ThePolyhedronOfInterCSurf as SamplePolyhedron<SurfaceTool<'_>>>::with_parameters(
                &&surface,
                &parameters(-3, nu),
                &parameters(7, nv),
            );
        let error = result.unwrap_err();
        assert_eq!(error.exception_type(), "Standard_OutOfRange");
        assert_eq!(error.message(), "NCollection_Array1::Value");
    }
}

#[test]
fn ordinary_intersection_parameter_grids_still_require_two_values_on_each_axis() {
    let surface = plane_at_height_three();
    for (nu, nv) in [(0, 0), (0, 2), (2, 0), (1, 1), (1, 2), (2, 1)] {
        let result = ThePolyhedronOfHInter::on_adaptor_with_parameters(
            surface.surface(),
            &parameters(-3, nu),
            &parameters(7, nv),
        );
        assert_eq!(result.unwrap_err().exception_type(), "Standard_OutOfRange");
    }
}
