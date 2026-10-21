//! Tests of the BRepGProp properties. The check of a retained cut result is in
//! brep_gprop_cut_cylinder_fixture.rs.
mod gprop_builders;
use gprop_builders::{plane_rectangle, quarter_disk, wide_spline_surface};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Edge, Face};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dBSplineCurve, Geom2dCurve};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve, GeomPlane, GeomSurface};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::gcpnts::AbscissaPoint;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Direction3, Plane3, Point2, Point3};
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use martensite_tktopalgo::brep_gprop::{
    BRepGProp, BRepGPropDomain, BRepGPropFace, BRepGPropSinert, BRepGPropVinert,
};
use std::f64::consts::PI;

#[path = "common/floating.rs"]
mod floating;
use floating::within_absolute_error as near;

#[path = "common/property_assertions.rs"]
mod property_assertions;
use property_assertions::expect_properties_near as assert_gprops_near;

// BRepGPropTest.LinearProperties_EdgeLength
#[test]
fn linear_properties_edge_length() {
    let maker =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(0.0, 0.0, 0.0), Point3::new(3.0, 4.0, 0.0))
            .unwrap();
    assert!(maker.is_done());
    let mut props = GProps::new();
    BRepGProp::linear_properties(&maker.edge().unwrap(), &mut props, false, false).unwrap();
    assert!(near(props.mass(), 5.0, precision::confusion()));
}

// BRepGPropTest.AdaptiveNaturalBSplineSurface_ResizesIntervalArrays
#[test]
fn adaptive_natural_bspline_surface_resizes_interval_arrays() {
    let intervals = martensite_tkmath::math::gauss_points_max() + 1;
    let mut f = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(
            &mut f,
            &wide_spline_surface(intervals),
            precision::confusion(),
        )
        .unwrap();
    assert_eq!(f.nb_children(), 0);
    let fixed_face = BRepGPropFace::with_face(&f, false).unwrap();
    assert_eq!(fixed_face.su_int_subs().unwrap(), intervals);
    let fixed = BRepGPropSinert::with_face(&fixed_face, Point3::new(0.0, 0.0, 0.0)).unwrap();
    let mut adaptive_face = BRepGPropFace::with_face(&f, false).unwrap();
    let adaptive =
        BRepGPropSinert::with_face_eps(&mut adaptive_face, Point3::new(0.0, 0.0, 0.0), 1.0e-8)
            .unwrap();
    let n = f64::from(intervals);
    assert!(near(adaptive.mass(), 2.0 * n, 1.0e-10));
    assert!(near(adaptive.centre_of_mass().x(), 0.5 * n, 1.0e-10));
    assert!(near(adaptive.centre_of_mass().y(), 1.0, 1.0e-12));
    assert!(near(adaptive.centre_of_mass().z(), 0.0, 1.0e-12));
    assert_gprops_near(&adaptive, &fixed, 1.0e-9);
}

// BRepGPropTest.AdaptiveTrimmedQuarterDisk_ReusesBoundaryBuffers
#[test]
fn adaptive_trimmed_quarter_disk_reuses_boundary_buffers() {
    let radius = 4.0;
    let origin = Point3::new(0.0, 0.0, 0.0);
    let f = quarter_disk(radius);
    let mut order_face = BRepGPropFace::with_face(&f, false).unwrap();
    let mut domain = BRepGPropDomain::with_face(&f).unwrap();
    let mut orders = Vec::new();
    while domain.more() {
        assert!(order_face.load_edge(&domain.value().unwrap()).unwrap());
        orders.push(order_face.integration_order().unwrap());
        domain.next().unwrap();
    }
    assert_eq!(orders.len(), 3);
    assert!(orders.iter().any(|order| *order != orders[0]));
    let sinert = |eps: Option<f64>| {
        let mut face = BRepGPropFace::with_face(&f, false).unwrap();
        let mut domain = BRepGPropDomain::with_face(&f).unwrap();
        match eps {
            None => BRepGPropSinert::with_domain(&mut face, &mut domain, origin).unwrap(),
            Some(eps) => {
                BRepGPropSinert::with_domain_eps(&mut face, &mut domain, origin, eps).unwrap()
            }
        }
    };
    let fixed = sinert(None);
    let mut adaptive = sinert(Some(1.0e-8));
    assert!(adaptive.get_epsilon() >= 0.0 && adaptive.get_epsilon() <= 1.0e-8);
    assert!(near(adaptive.mass(), PI * radius * radius / 4.0, 1.0e-10));
    let expected = 4.0 * radius / (3.0 * PI);
    assert!(near(adaptive.centre_of_mass().x(), expected, 1.0e-7));
    assert!(near(adaptive.centre_of_mass().y(), expected, 1.0e-7));
    assert!(near(adaptive.centre_of_mass().z(), 0.0, 1.0e-12));
    assert_gprops_near(&adaptive, &fixed, 5.0e-5);
    let mut fallback = sinert(Some(1.0e-2));
    floating::expect_double_eq(fallback.get_epsilon(), 1.0e-2);
    assert_gprops_near(&fallback, &fixed, 5.0e-5);
}

// BRepGPropTest.AdaptivePlaneLimitedInertia
#[test]
fn adaptive_plane_limited_inertia() {
    let normal = Direction3::new(0.0, 0.0, 1.0).unwrap();
    let f = plane_rectangle(
        Plane3::new(Point3::new(0.0, 0.0, 2.0), normal).unwrap(),
        (0.0, 3.0, 0.0, 4.0),
    );
    let reference = Plane3::new(Point3::new(0.0, 0.0, 0.0), normal).unwrap();
    let location = Point3::new(0.0, 0.0, 0.0);
    let vinert = |eps: Option<f64>| {
        let mut face = BRepGPropFace::with_face(&f, false).unwrap();
        let mut domain = BRepGPropDomain::with_face(&f).unwrap();
        match eps {
            None => BRepGPropVinert::with_domain_plane(&mut face, &mut domain, reference, location)
                .unwrap(),
            Some(eps) => BRepGPropVinert::with_domain_plane_eps(
                &mut face,
                &mut domain,
                reference,
                location,
                eps,
            )
            .unwrap(),
        }
    };
    let (fixed, adaptive) = (vinert(None), vinert(Some(1.0e-8)));
    let m = adaptive.matrix_of_inertia();
    let expected = [
        ((1, 1), 160.0),
        ((2, 2), 104.0),
        ((3, 3), 200.0),
        ((1, 2), -72.0),
        ((1, 3), -36.0),
        ((2, 3), -48.0),
    ];
    for ((row, column), value) in expected {
        assert!(near(m.value(row, column).unwrap(), value, 1.0e-10));
    }
    assert_gprops_near(&adaptive, &fixed, 1.0e-10);
}

// BRepGPropTest.NaturalInfinitePlane_UsesInfiniteArithmetic
#[test]
fn natural_infinite_plane_uses_infinite_arithmetic() {
    let plane = Plane3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::from_pln(plane)).cast();
    let mut f = Face::default();
    BRepBuilder::default()
        .make_face_on_surface(&mut f, &surface, precision::confusion())
        .unwrap();
    let tool = BRepGPropFace::with_face(&f, false).unwrap();
    let props = BRepGPropSinert::with_face(&tool, Point3::new(0.0, 0.0, 0.0)).unwrap();
    assert!(precision::is_positive_infinite(props.mass()));
}

// BSplineLengthConsistencyAbscissaVsLinearProperties
#[test]
fn occ8797_bspline_length_consistency() {
    let points = [
        (0.0, 0.0),
        (1.0, 1.0),
        (2.0, 1.0),
        (3.0, 0.0),
        (4.0, 1.0),
        (5.0, 1.0),
        (6.0, 0.0),
    ];
    let mut poles = Array1::with_bounds(0, 6).unwrap();
    for (i, (x, y)) in points.into_iter().enumerate() {
        poles[i as i32] = Point3::new(x, y, 0.0);
    }
    let mut knots = Array1::with_bounds(0, 2).unwrap();
    let mut mults = Array1::with_bounds(0, 2).unwrap();
    for (i, (knot, mult)) in [(0.0, 4), (0.5, 3), (1.0, 4)].into_iter().enumerate() {
        knots[i as i32] = knot;
        mults[i as i32] = mult;
    }
    let spline = GeomBSplineCurve::new(&poles, &knots, &mults, 3, false).unwrap();
    assert_eq!((spline.nb_poles(), spline.nb_knots()), (7, 3));
    let curve: Handle<dyn GeomCurve> = Handle::new(spline).cast();
    assert!(!curve.is_null());
    let length_abscissa =
        AbscissaPoint::length(&GeomAdaptorCurve::with_curve(&curve).unwrap()).unwrap();
    assert!(length_abscissa > 0.0);
    let e = BRepBuilderAPIMakeEdge::from_curve(&curve)
        .unwrap()
        .edge()
        .unwrap();
    let mut props = GProps::new();
    BRepGProp::linear_properties(&e, &mut props, false, false).unwrap();
    assert!(props.mass() > 0.0);
    assert!(near(length_abscissa, props.mass(), props.mass() * 1e-3));
}

// BRepGPropTest.LinearProperties_DegenerateEdgeWithBSplinePCurveNoCrash
#[test]
fn linear_properties_degenerate_edge_with_bspline_pcurve_no_crash() {
    let plane = Plane3::new(
        Point3::new(0.0, 0.0, 0.0),
        Direction3::new(0.0, 0.0, 1.0).unwrap(),
    )
    .unwrap();
    let surface: Handle<dyn GeomSurface> = Handle::new(GeomPlane::from_pln(plane)).cast();
    let mut poles = Array1::with_bounds(1, 4).unwrap();
    for (i, (u, v)) in [(0.0, 0.0), (0.1, 0.05), (0.2, -0.05), (0.3, 0.0)]
        .into_iter()
        .enumerate()
    {
        poles[i as i32 + 1] = Point2::new(u, v);
    }
    let mut knots = Array1::with_bounds(1, 2).unwrap();
    knots[2] = 1.0;
    let mut mults = Array1::with_bounds(1, 2).unwrap();
    mults.init(4);
    let pcurve = Geom2dBSplineCurve::new(&poles, &knots, &mults, 3, false).unwrap();
    let (first, last) = (pcurve.first_parameter(), pcurve.last_parameter());
    let pcurve: Handle<dyn Geom2dCurve> = Handle::new(pcurve).cast();
    let b = BRepBuilder::default();
    let mut e = Edge::default();
    b.make_edge(&mut e).unwrap();
    b.update_edge_pcurve(
        &e,
        &pcurve,
        &surface,
        &Location::default(),
        precision::confusion(),
        None,
    )
    .unwrap();
    b.range_on_surface(&e, &surface, &Location::default(), first, last)
        .unwrap();
    b.degenerated(&e, true).unwrap();
    let mut props = GProps::new();
    BRepGProp::linear_properties(&e, &mut props, false, false).unwrap();
    assert!(props.mass().is_finite());
    assert!(props.mass() >= 0.0);
}
