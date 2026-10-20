//! External composition of the geometric checking step with independent line/plane distances.
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurveOnSurface, Adaptor3dSurface};
use martensite_tkg3d::geom::{GeomPlane, GeomSurface};
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkmath::gp::{Axis3, Direction2, Plane3, Point2, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};
use martensite_tktopalgo::brep_lib::BRepLibCheckCurveOnSurface;

struct ComposedSurfaceCheck {
    check: BRepLibCheckCurveOnSurface,
    edge: Edge,
    face: Face,
}

impl ComposedSurfaceCheck {
    fn new() -> Result<Self, Failure> {
        let edge =
            BRepBuilderAPIMakeEdge::from_points(Point3::new(0., 0., 0.), Point3::new(1., 0., 0.))?
                .edge()?;
        let face =
            BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 1., -10., 10.))?
                .face()?;
        let check = BRepLibCheckCurveOnSurface::with_edge_face(&edge, &face)?;
        Ok(Self { check, edge, face })
    }

    fn evaluate(&mut self, offset: f64, first: f64) -> Result<(), Failure> {
        self.check.compute(&offset_curve(offset, first)?);
        Ok(())
    }

    fn reinitialize(&mut self) -> Result<(), Failure> {
        self.check.init(&self.edge, &self.face)?;
        assert!(self.check.is_done());
        assert_eq!(self.check.max_distance(), f64::MAX);
        assert_eq!(self.check.max_parameter(), 0.);
        Ok(())
    }

    fn measured_result(&self) -> (f64, f64) {
        (self.check.max_distance(), self.check.max_parameter())
    }
}

fn offset_curve(offset: f64, first: f64) -> Result<Handle<Adaptor3dCurveOnSurface>, Failure> {
    let curve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
        Point2::new(0., offset),
        Direction2::new(1., 0.)?,
    ))
    .cast();
    let adaptor: Handle<dyn Adaptor2dCurve2d> =
        Handle::new(Geom2dAdaptorCurve::with_range(&curve, first, 1.)?).cast();
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(Axis3::default())).cast();
    let surface: Handle<dyn Adaptor3dSurface> =
        Handle::new(GeomAdaptorSurface::with_surface(&plane)?).cast();
    Ok(Handle::new(Adaptor3dCurveOnSurface::with_curve(
        &adaptor, &surface,
    )?))
}

#[test]
fn composed_check_measures_constant_line_plane_offsets_in_both_modes() -> Result<(), Failure> {
    for parallel in [false, true] {
        let mut composed = ComposedSurfaceCheck::new()?;
        composed.check.set_parallel(parallel);
        for offset in [3., -5., 0.] {
            composed.evaluate(offset, 0.)?;
            assert!(composed.check.is_done());
            assert_eq!(composed.check.error_status(), 0);
            assert!((composed.check.max_distance() - offset.abs()).abs() <= 1e-12);
            assert!((0.0..=1.0).contains(&composed.check.max_parameter()));
            assert_eq!(composed.check.is_parallel(), parallel);
        }
    }
    Ok(())
}

#[test]
fn null_computation_preserves_results_and_error_until_reinitialization() -> Result<(), Failure> {
    let mut composed = ComposedSurfaceCheck::new()?;
    composed.evaluate(3., 0.)?;
    let previous = composed.measured_result();
    composed.check.compute(&Handle::default());
    assert!(!composed.check.is_done());
    assert_eq!(composed.check.error_status(), 1);
    assert_eq!(composed.measured_result(), previous);
    composed.evaluate(5., 0.)?;
    assert_eq!(composed.check.error_status(), 1);
    assert!((composed.check.max_distance() - 5.).abs() <= 1e-12);
    composed.reinitialize()?;
    composed.evaluate(3., 0.)?;
    assert!(composed.check.is_done());
    Ok(())
}

#[test]
fn mismatched_range_preserves_previous_geometric_results() -> Result<(), Failure> {
    let mut composed = ComposedSurfaceCheck::new()?;
    composed.evaluate(3., 0.)?;
    let previous = composed.measured_result();
    composed.evaluate(5., -1.)?;
    assert!(!composed.check.is_done());
    assert_eq!(composed.check.error_status(), 2);
    assert_eq!(composed.measured_result(), previous);
    composed.reinitialize()?;
    composed.evaluate(5., 0.)?;
    assert!(composed.check.is_done());
    assert!((composed.check.max_distance() - 5.).abs() <= 1e-12);
    Ok(())
}

#[test]
fn empty_reference_reports_null_error_without_changing_initial_results() -> Result<(), Failure> {
    let mut check = BRepLibCheckCurveOnSurface::new();
    check.compute(&offset_curve(3., 0.)?);
    assert!(!check.is_done());
    assert_eq!(check.error_status(), 1);
    assert_eq!(check.max_distance(), f64::MAX);
    assert_eq!(check.max_parameter(), 0.);
    Ok(())
}
