//! Fields serialized after a curve dumper observe mutations made by that callback.
use super::curve_update_rebinding::rebinding_curve::RebindingParameterCurve;
use martensite_tkbrep::brep::{BRepCurveOnClosedSurface, BRepCurveRepresentation};
use martensite_tkernel::standard::{Handle, dump::DumpStream};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkmath::{
    geom_abs::Shape,
    gp::{Axis2d, Point2},
    toploc::Location,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

fn seam_with_dump_curve(curve: &Handle<dyn Geom2dCurve>) -> Handle<BRepCurveOnClosedSurface> {
    let first: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::new(Axis2d::default())).cast();
    Handle::new(
        BRepCurveOnClosedSurface::new(
            &first,
            curve,
            &Handle::default(),
            &Location::default(),
            Shape::C0,
        )
        .unwrap(),
    )
}

fn dump_fixture() -> (Handle<BRepCurveOnClosedSurface>, Arc<AtomicUsize>) {
    let curve = Handle::new(RebindingParameterCurve::new(
        Arc::new(AtomicUsize::new(0)),
        Arc::new(|u| Ok(Point2::new(u, 0.))),
    ));
    let source = seam_with_dump_curve(&curve.cast());
    let owner = source.downgrade();
    let count = Arc::new(AtomicUsize::new(0));
    let calls = count.clone();
    curve.set_dump_hook(Arc::new(move || {
        calls.fetch_add(1, Ordering::SeqCst);
        owner.upgrade().set_continuity(Shape::C1).unwrap();
        owner
            .upgrade()
            .set_uv_points2(Point2::new(17., 19.), Point2::new(23., 29.));
    }));
    (source, count)
}

#[test]
fn seam_dump_late_fields_observe_prior_curve_callback() {
    let (source, count) = dump_fixture();
    let mut stream = DumpStream::default();
    source.dump_json(&mut stream, -1).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert_eq!(source.continuity().unwrap(), Shape::C1);
    let text: String = String::from_utf8(stream.bytes().to_vec())
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    assert!(
        text.contains("\"UV21\":{\"gp_Pnt2d\":[17,19]}"),
        "dump={text}"
    );
    assert!(
        text.contains("\"UV22\":{\"gp_Pnt2d\":[23,29]}"),
        "dump={text}"
    );
    assert!(
        text.contains(&format!("\"Continuity\":{}", Shape::C1 as i32)),
        "dump={text}; expected changed continuity"
    );
}

#[test]
fn shallow_seam_dump_skips_curve_callback() {
    let (source, count) = dump_fixture();
    source.dump_json(&mut DumpStream::default(), 0).unwrap();
    assert_eq!(count.load(Ordering::SeqCst), 0);
    assert_eq!(source.continuity().unwrap(), Shape::C0);
    assert_eq!(source.uv_points2(), (Point2::default(), Point2::default()));
}
