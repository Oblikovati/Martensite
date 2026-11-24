use super::{GenExtPS, GridDistance, Point3};
use crate::extrema::point_surface::gen_ext_ps::{ParamGrid, Sampling};
use crate::extrema::pon::POnSurfParams;
use martensite_tkernel::ncollection::Array2;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::geom::{GeomBezierSurface, GeomPlane, GeomSurface};
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkmath::gp::Axis3;

#[test]
fn row_distances_match_scalar_cells_in_asymmetric_grids() {
    let plane = Handle::new(GeomPlane::new(Axis3::default())).into_upcast();
    let surfaces = [plane, rational_patch()];
    for surface in &surfaces {
        let surface = GeomAdaptorSurface::with_surface(surface).unwrap();
        for (u, v) in [(2, 3), (7, 5)] {
            check_reused_grid(&surface, u, v);
        }
    }
}

fn rational_patch() -> Handle<dyn GeomSurface> {
    let mut poles = Array2::with_bounds(1, 3, 1, 2).unwrap();
    let mut weights = Array2::with_bounds(1, 3, 1, 2).unwrap();
    for u in 1..=3 {
        for v in 1..=2 {
            let x = f64::from(u - 1);
            let y = f64::from(v - 1);
            *poles.change_value(u, v).unwrap() = Point3::new(x, y, (x * (1.0 - 0.25 * x)) * y);
            *weights.change_value(u, v).unwrap() = 1.0 + 0.25 * x + 0.125 * y;
        }
    }
    Handle::new(GeomBezierSurface::with_weights(&poles, &weights).unwrap()).into_upcast()
}

fn check_reused_grid(surface: &dyn Adaptor3dSurface, u: i32, v: i32) {
    let mut rows = initialized(surface, u, v);
    let mut scalar = initialized(surface, u, v);
    for point in [
        Point3::new(0.375, 0.625, 0.0625),
        Point3::new(-0.0625, 0.875, 0.25),
        Point3::new(2.25, -0.125, 1.0),
    ] {
        rows.build_grid(point).unwrap();
        scalar.scalar_node_distances(point).unwrap();
        let projection = GridDistance {
            surface,
            point,
            diff_tol: scalar.tolu + scalar.tolv,
        };
        scalar.scalar_edge_distances(&projection).unwrap();
        scalar.scalar_face_distances(&projection).unwrap();
        scalar.reset_face_distances().unwrap();
        compare_grid(&rows, &scalar);
    }
}

fn initialized(surface: &dyn Adaptor3dSurface, u: i32, v: i32) -> GenExtPS<'_> {
    let mut extrema = GenExtPS::new();
    let sampling = Sampling {
        nb_u: u,
        nb_v: v,
        tol_u: 1e-10,
        tol_v: 1e-9,
    };
    extrema
        .initialize_range(surface, sampling, (0.0, 1.0), (0.0, 1.0))
        .unwrap();
    extrema.init_grid().unwrap();
    assert!(extrema.init);
    extrema
}

fn compare_grid(rows: &GenExtPS<'_>, scalar: &GenExtPS<'_>) {
    let (u, v) = (rows.usample, rows.vsample);
    for (row, original, u_bounds, v_bounds) in [
        (&rows.points, &scalar.points, (0, u + 1), (0, v + 1)),
        (
            &rows.u_edge_pnt_params,
            &scalar.u_edge_pnt_params,
            (1, u - 1),
            (1, v),
        ),
        (
            &rows.v_edge_pnt_params,
            &scalar.v_edge_pnt_params,
            (1, u),
            (1, v - 1),
        ),
        (
            &rows.face_pnt_params,
            &scalar.face_pnt_params,
            (0, u),
            (0, v),
        ),
    ] {
        compare_cells(row, original, u_bounds, v_bounds);
    }
}

fn compare_cells(rows: &ParamGrid, scalar: &ParamGrid, (u0, u1): (i32, i32), (v0, v1): (i32, i32)) {
    for u in u0..=u1 {
        for v in v0..=v1 {
            let row = rows.value(u, v).unwrap();
            let original = scalar.value(u, v).unwrap();
            assert_eq!(
                row.get_element_type(),
                original.get_element_type(),
                "{u}, {v}"
            );
            assert_eq!(row.get_indices(), original.get_indices(), "{u}, {v}");
            assert_eq!(coordinates(row), coordinates(original), "{u}, {v}");
        }
    }
}

fn coordinates(param: &POnSurfParams) -> [u64; 6] {
    let (u, v) = param.parameter();
    let xyz = param.value();
    [u, v, xyz.x(), xyz.y(), xyz.z(), param.get_sqr_distance()].map(f64::to_bits)
}

#[test]
fn refused_reinitialize_retains_scalar_grid_results_and_errors() {
    let plane = Handle::new(GeomPlane::new(Axis3::default())).into_upcast();
    let surface = GeomAdaptorSurface::with_surface(&plane).unwrap();
    let point = Point3::new(0.25, 0.75, 0.125);
    for (u, v) in [(1, 5), (1, 9)] {
        let mut retained = initialized(&surface, 7, 5);
        let mut scalar = initialized(&surface, 7, 5);
        refuse_reinitialize(&mut retained, &surface, u, v);
        refuse_reinitialize(&mut scalar, &surface, u, v);
        let actual = retained.perform(point);
        let expected = perform_scalar(&mut scalar, &surface, point);
        let diagnostics = |failure: martensite_tkernel::standard::Failure| {
            (
                failure.exception_type().to_owned(),
                failure.message().to_owned(),
            )
        };
        assert_eq!(actual.map_err(diagnostics), expected.map_err(diagnostics));
        assert_eq!(retained.is_done(), scalar.is_done());
        compare_extrema(&retained, &scalar);
    }
}

fn refuse_reinitialize<'a>(
    extrema: &mut GenExtPS<'a>,
    surface: &'a dyn Adaptor3dSurface,
    u: i32,
    v: i32,
) {
    let bad = Sampling {
        nb_u: u,
        nb_v: v,
        tol_u: 1e-10,
        tol_v: 1e-9,
    };
    let failure = extrema
        .initialize_range(surface, bad, (0.0, 1.0), (0.0, 1.0))
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert!(extrema.init);
}

fn perform_scalar(
    extrema: &mut GenExtPS<'_>,
    surface: &dyn Adaptor3dSurface,
    point: Point3,
) -> Result<(), martensite_tkernel::standard::Failure> {
    extrema.done = false;
    extrema.f.set_point(point);
    extrema.scalar_node_distances(point)?;
    let projection = GridDistance {
        surface,
        point,
        diff_tol: extrema.tolu + extrema.tolv,
    };
    extrema.scalar_edge_distances(&projection)?;
    extrema.scalar_face_distances(&projection)?;
    extrema.reset_face_distances()?;
    extrema.find_grid_minima()?;
    extrema.find_grid_maxima()
}

fn compare_extrema(actual: &GenExtPS<'_>, expected: &GenExtPS<'_>) {
    if !actual.is_done() {
        return;
    }
    let count = actual.nb_ext().unwrap();
    assert_eq!(count, expected.nb_ext().unwrap());
    for i in 1..=count {
        let a = actual.point(i).unwrap();
        let b = expected.point(i).unwrap();
        assert_eq!(a, b);
        assert_eq!(
            actual.square_distance(i).unwrap().to_bits(),
            expected.square_distance(i).unwrap().to_bits()
        );
    }
}
