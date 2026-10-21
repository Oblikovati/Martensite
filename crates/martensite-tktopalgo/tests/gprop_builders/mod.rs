//! Original builder-created geometry used by the property regressions.
use martensite_tkbrep::topods::{Edge, Face, Wire};
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::{GeomBSplineSurface, GeomCurve, GeomSurface};
use martensite_tkgeombase::gc::MakeArcOfCircle;
use martensite_tkmath::gp::{Axis2, Circle3, Direction3, Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape, BRepBuilderAPIMakeWire,
};
use std::f64::consts::FRAC_PI_2;

/// The degree 1 B-spline surface with GaussPointsMax + 1 U intervals of the original case.
pub fn wide_spline_surface(intervals: i32) -> Handle<dyn GeomSurface> {
    let nb_poles = intervals + 1;
    let mut poles = Array2::with_bounds(1, nb_poles, 1, 2).unwrap();
    let mut knots = Array1::with_bounds(1, nb_poles).unwrap();
    let mut mults = Array1::with_bounds(1, nb_poles).unwrap();
    for i in 1..=nb_poles {
        *poles.change_value(i, 1).unwrap() = Point3::new(f64::from(i - 1), 0.0, 0.0);
        *poles.change_value(i, 2).unwrap() = Point3::new(f64::from(i - 1), 2.0, 0.0);
        knots[i] = f64::from(i - 1);
        mults[i] = if i == 1 || i == nb_poles { 2 } else { 1 };
    }
    let mut v_knots = Array1::with_bounds(1, 2).unwrap();
    v_knots[2] = 1.0;
    let mut v_mults = Array1::with_bounds(1, 2).unwrap();
    v_mults.init(2);
    let surface = GeomBSplineSurface::new(
        &poles,
        (&knots, &mults),
        (&v_knots, &v_mults),
        (1, 1),
        (false, false),
    );
    Handle::new(surface.unwrap()).cast()
}

/// Builds the quarter disk with the original arc, wire and plane-face makers.
pub fn quarter_disk(radius: f64) -> Face {
    let origin = Point3::new(0., 0., 0.);
    let normal = Direction3::new(0., 0., 1.).unwrap();
    let wire = quarter_disk_wire(radius, origin, normal);
    let maker =
        BRepBuilderAPIMakeFace::from_plane_wire(Plane3::new(origin, normal).unwrap(), &wire, true)
            .unwrap();
    assert!(maker.is_done());
    maker.face().unwrap()
}

fn quarter_disk_arc(radius: f64, origin: Point3, normal: Direction3) -> Edge {
    let circle = Circle3::new(Axis2::new(origin, normal).unwrap(), radius).unwrap();
    let arc: Handle<dyn GeomCurve> = MakeArcOfCircle::from_parameters(circle, 0., FRAC_PI_2, true)
        .unwrap()
        .value()
        .unwrap()
        .cast();
    BRepBuilderAPIMakeEdge::from_curve(&arc)
        .unwrap()
        .edge()
        .unwrap()
}

fn segment_edge(first: Point3, second: Point3) -> Edge {
    let maker = BRepBuilderAPIMakeEdge::from_points(first, second).unwrap();
    maker.edge().unwrap()
}

fn quarter_disk_wire(radius: f64, origin: Point3, normal: Direction3) -> Wire {
    let mut wire = BRepBuilderAPIMakeWire::new();
    wire.add_edge(&quarter_disk_arc(radius, origin, normal))
        .unwrap();
    wire.add_edge(&segment_edge(Point3::new(0., radius, 0.), origin))
        .unwrap();
    wire.add_edge(&segment_edge(origin, Point3::new(radius, 0., 0.)))
        .unwrap();
    assert!(wire.is_done());
    wire.wire().unwrap()
}

/// Builds the original bounded planar face and checks completion before reading it.
pub fn plane_rectangle(plane: Plane3, bounds: (f64, f64, f64, f64)) -> Face {
    let maker = BRepBuilderAPIMakeFace::from_plane_bounds(plane, bounds).unwrap();
    assert!(maker.is_done());
    maker.face().unwrap()
}
