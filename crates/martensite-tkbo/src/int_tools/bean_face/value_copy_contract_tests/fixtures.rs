use super::*;

fn curve_edge(curve: &Handle<GeomBezierCurve>) -> Edge {
    let mut edge = Edge::default();
    let builder = BRepBuilder::default();
    builder
        .make_edge_with_curve(&mut edge, &curve.cast::<dyn GeomCurve>(), 1e-7)
        .unwrap();
    builder.range(&edge, 0., 1., true).unwrap();
    edge
}

pub(super) fn cached_curve() -> (Handle<GeomBezierCurve>, BRepAdaptorCurve) {
    let points = [Point3::default(), Point3::new(2., 0., 0.)];
    let curve = Handle::new(GeomBezierCurve::new(&Array1::from_shared(&points, 1)).unwrap());
    curve.clear_eval_representation();
    let adaptor = BRepAdaptorCurve::with_edge(&curve_edge(&curve)).unwrap();
    assert_eq!(adaptor.value(0.25).unwrap(), Point3::new(0.5, 0., 0.));
    (curve, adaptor)
}

fn lattice_point(u: i32, v: i32, z: f64) -> Point3 {
    Point3::new(2. * f64::from(u - 1), 2. * f64::from(v - 1), z)
}

fn patch_poles() -> Array2<'static, Point3> {
    let mut poles = Array2::with_bounds(1, 2, 1, 2).unwrap();
    for u in 1..=2 {
        for v in 1..=2 {
            poles.set_value(u, v, lattice_point(u, v, 0.)).unwrap();
        }
    }
    poles
}

pub(super) fn cached_surface() -> (Handle<GeomBezierSurface>, BRepAdaptorSurface) {
    let surface = Handle::new(GeomBezierSurface::new(&patch_poles()).unwrap());
    surface.clear_eval_representation();
    let mut face = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut face, &surface.cast::<dyn GeomSurface>(), 1e-7)
        .unwrap();
    let adaptor = BRepAdaptorSurface::with_face(&face, false).unwrap();
    assert_eq!(adaptor.value(0.25, 0.5).unwrap(), Point3::new(0.5, 1., 0.));
    (surface, adaptor)
}

pub(super) fn raise_patch(surface: &GeomBezierSurface) {
    for u in 1..=2 {
        for v in 1..=2 {
            surface.set_pole(u, v, lattice_point(u, v, 4.)).unwrap();
        }
    }
}

fn bounded_constructor(
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
) -> BeanFaceIntersector {
    BeanFaceIntersector::from_adaptors_with_bounds(
        curve,
        surface,
        Range::new(0., 1.),
        [0., 1., 0., 1.],
        [0.125, 0.25],
    )
    .unwrap()
}

fn initialize(
    mode: usize,
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
) -> BeanFaceIntersector {
    let mut bean = BeanFaceIntersector::new();
    if mode == 2 {
        bean.init_adaptors(curve, surface, 0.125, 0.25).unwrap();
    } else {
        bounded_initialize(&mut bean, curve, surface);
    }
    bean
}

pub(super) fn constructed(
    mode: usize,
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
) -> BeanFaceIntersector {
    match mode {
        0 => BeanFaceIntersector::from_adaptors(curve, surface, 0.125, 0.25).unwrap(),
        1 => bounded_constructor(curve, surface),
        _ => initialize(mode, curve, surface),
    }
}

pub(super) fn raise_curve(curve: &GeomBezierCurve) {
    curve.set_pole(1, Point3::new(0., 0., 4.)).unwrap();
    curve.set_pole(2, Point3::new(2., 0., 4.)).unwrap();
}

fn bounded_initialize(
    bean: &mut BeanFaceIntersector,
    curve: &BRepAdaptorCurve,
    surface: &BRepAdaptorSurface,
) {
    bean.init_adaptors_with_bounds(
        curve,
        surface,
        Range::new(0., 1.),
        [0., 1., 0., 1.],
        [0.125, 0.25],
    )
    .unwrap();
}
