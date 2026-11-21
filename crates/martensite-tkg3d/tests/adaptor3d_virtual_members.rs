//! Vertex dispatch, base behavior and caller-supplied sampling counts.
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::{impl_transient, standard::RuntimeTyped};
use martensite_tkg2d::adaptor2d::{Adaptor2dCurve2d, Adaptor2dLine2d};
use martensite_tkg3d::adaptor3d::{
    Adaptor3dCurve, Adaptor3dHVertex, Adaptor3dHVertexBase, Adaptor3dSurface, Adaptor3dTopolTool,
};
use martensite_tkg3d::geom::{GeomCurve, GeomLine, GeomPlane, GeomSurface};
use martensite_tkg3d::geom_adaptor::{
    GeomAdaptorCurve, GeomAdaptorSurface, GeomAdaptorSurfaceOfLinearExtrusion,
    GeomAdaptorSurfaceOfRevolution, GeomSurfaceAdaptor,
};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::{Axis1, Axis3, Direction2, Direction3, Point2, Point3};

/// A subclass of the vertex whose members all differ from the base ones.
struct FakeOverridingVertex;
impl_transient!(
    FakeOverridingVertex,
    "FakeOverridingVertex",
    dyn Adaptor3dHVertex,
    interfaces: [dyn Adaptor3dHVertex]
);
impl Adaptor3dHVertex for FakeOverridingVertex {
    fn value(&self) -> Point2 {
        Point2::new(7.0, 8.0)
    }
    fn parameter(&self, _curve: &Handle<dyn Adaptor2dCurve2d>) -> Result<f64, Failure> {
        Ok(42.0)
    }
    fn resolution(&self, _curve: &Handle<dyn Adaptor2dCurve2d>) -> Result<f64, Failure> {
        Err(Failure::with_message("Standard_NullObject", "fake"))
    }
    fn orientation(&self) -> Orientation {
        Orientation::Internal
    }
    fn is_same(&self, _other: &Handle<dyn Adaptor3dHVertex>) -> Result<bool, Failure> {
        Ok(false)
    }
}

fn line() -> Handle<dyn Adaptor2dCurve2d> {
    let d = Direction2::new(1.0, 0.0).unwrap();
    Handle::new(Adaptor2dLine2d::new(Point2::new(0.0, 0.0), d, 0.0, 10.0)).cast()
}

fn plane_adaptor() -> Handle<dyn Adaptor3dSurface> {
    let z = Direction3::new(0.0, 0.0, 1.0).unwrap();
    let frame = Axis3::from_normal(Point3::new(0.0, 0.0, 0.0), z).unwrap();
    let plane: Handle<dyn GeomSurface> = Handle::new(GeomPlane::new(frame)).cast();
    Handle::new(GeomAdaptorSurface::with_surface(&plane).unwrap()).cast()
}

#[test]
fn base_vertex_keeps_its_base_members() {
    let base = Adaptor3dHVertexBase::new(Point2::new(3.0, 0.0), Orientation::Reversed, 0.25);
    let vertex: Handle<dyn Adaptor3dHVertex> = Handle::new(base).cast();
    assert_eq!(vertex.value(), Point2::new(3.0, 0.0));
    assert_eq!(vertex.parameter(&line()).unwrap(), 3.0);
    assert_eq!(vertex.resolution(&line()).unwrap(), 0.25);
    assert_eq!(vertex.orientation(), Orientation::Reversed);
    assert!(vertex.is_same(&vertex).unwrap());
    assert!(vertex.is_kind_name("Adaptor3d_HVertex"));
    assert!(vertex.is_instance(<dyn Adaptor3dHVertex>::static_type()));
}

#[test]
fn trait_handles_dispatch_to_the_overrides() {
    let fake: Handle<dyn Adaptor3dHVertex> = Handle::new(FakeOverridingVertex).into_upcast();
    assert_eq!(fake.value(), Point2::new(7.0, 8.0));
    assert_eq!(fake.parameter(&line()).unwrap(), 42.0);
    assert!(fake.resolution(&line()).is_err());
    assert!(fake.is_kind_name("Adaptor3d_HVertex"));
    assert!(fake.is_instance(FakeOverridingVertex::static_type()));
    // The base IsSame reads the other vertex through its own (overridden) Value.
    let base: Handle<dyn Adaptor3dHVertex> = Handle::new(Adaptor3dHVertexBase::new(
        Point2::new(7.0, 8.0),
        Orientation::Forward,
        0.0,
    ))
    .cast();
    assert!(base.is_same(&fake).unwrap());
    // Adaptor3dTopolTool::Orientation(V) and Identical call the vertex members.
    let tool = Adaptor3dTopolTool::default();
    assert_eq!(
        tool.vertex_orientation(&fake).unwrap(),
        Orientation::Internal
    );
    assert!(!tool.identical(&fake, &base).unwrap());
    assert!(tool.identical(&base, &fake).unwrap());
}

#[test]
fn sample_pnts_body_spreads_the_counts_of_the_given_computation() {
    let surface = plane_adaptor();
    let (u, v) =
        Adaptor3dTopolTool::sample_pnts_parameters(&*surface, (0.01, 2, 2), || Ok((3, 5))).unwrap();
    assert_eq!((u.length(), v.length()), (3, 5));
    // The plane adaptor spans +-Precision::Infinite(), which is not RealFirst/RealLast,
    // so the box keeps it (natively too): the parameters spread over [-2e100, 2e100].
    assert_eq!(
        (*u.value(1).unwrap(), *u.value(3).unwrap()),
        (-2.0e100, 2.0e100)
    );
    assert_eq!(*v.value(2).unwrap(), -2.0e100 + 1.0e100);
    let failing = Adaptor3dTopolTool::sample_pnts_parameters(&*surface, (0.01, 2, 2), || {
        Err(Failure::with_message("Standard_NullObject", "no counts"))
    });
    assert!(failing.is_err());
    assert!(Adaptor3dTopolTool::uniform_sampling_of(&*surface).unwrap());
    // The base tool passes its own counts: 6 x 6 for a plane.
    let tool = Adaptor3dTopolTool::with_surface(&surface).unwrap();
    tool.sample_pnts(0.01, 2, 2).unwrap();
    assert_eq!(tool.u_parameters().unwrap().length(), 6);
    assert_eq!(tool.nb_samples_v().unwrap(), 6);
}

fn meridian() -> Handle<dyn Adaptor3dCurve> {
    let x = Point3::new(2., 0., 0.);
    let z = Direction3::new(0., 0., 1.).unwrap();
    let line: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(x, z)).into_upcast();
    Handle::new(GeomAdaptorCurve::with_curve(&line).unwrap()).into_upcast()
}

fn check_swept_dispatch(adaptor: &Handle<dyn Adaptor3dSurface>, expected: Point3) {
    let owners = adaptor.ref_count();
    let view = adaptor.view::<dyn GeomSurfaceAdaptor>();
    let surface = view.get().unwrap();
    assert_eq!(adaptor.ref_count(), owners);
    assert_eq!(view.object_address(), adaptor.object_address());
    assert!(adaptor.is_kind(GeomAdaptorSurface::static_type()));
    assert!(surface.surface().is_null());
    assert_eq!(surface.value(0., 1.5).unwrap(), expected);
    let owning = adaptor.cast::<dyn GeomSurfaceAdaptor>();
    assert_eq!(owning.object_address(), adaptor.object_address());
    assert_eq!(adaptor.ref_count(), owners + 1);
    let upcast = owning.into_upcast::<dyn Adaptor3dSurface>();
    assert_eq!(upcast.value(0., 1.5).unwrap(), expected);
    assert_eq!(adaptor.ref_count(), owners + 1);
    assert!(adaptor.downcast_ref::<GeomAdaptorSurface>().is_none());
}

#[test]
fn swept_adaptor_views_keep_identity_and_dispatch() {
    let meridian = meridian();
    let x = Direction3::new(1., 0., 0.).unwrap();
    let z = Direction3::new(0., 0., 1.).unwrap();
    let axis = Axis1::new(Point3::default(), z);
    let extrusion = GeomAdaptorSurfaceOfLinearExtrusion::with_curve_and_direction(&meridian, x);
    let revolution = GeomAdaptorSurfaceOfRevolution::with_curve_and_axis(&meridian, axis).unwrap();
    check_swept_dispatch(
        &Handle::new(extrusion).into_upcast(),
        Point3::new(3.5, 0., 0.),
    );
    check_swept_dispatch(
        &Handle::new(revolution).into_upcast(),
        Point3::new(2., 0., 1.5),
    );
}
