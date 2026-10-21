//! Surface dump callbacks must precede serialization of regularity continuity.
#[path = "regularity_dump_callbacks/dump_surface.rs"]
pub(super) mod dump_surface;
use dump_surface::CallbackDumpPlane;
use martensite_tkbrep::brep::{BRepCurveOn2Surfaces, BRepCurveRepresentation};
use martensite_tkernel::standard::{Handle, SharedState, dump::DumpStream};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::{geom_abs::Shape, toploc::Location};
use std::sync::Arc;

type DumpFixture = (Handle<BRepCurveOn2Surfaces>, Arc<SharedState<Vec<Shape>>>);

fn regularity_fixture() -> DumpFixture {
    let first = Handle::new(CallbackDumpPlane::new());
    let second = Handle::new(CallbackDumpPlane::new());
    let a: Handle<dyn GeomSurface> = first.cast();
    let b: Handle<dyn GeomSurface> = second.cast();
    let source = Handle::new(BRepCurveOn2Surfaces::new(
        &a,
        &b,
        &Location::default(),
        &Location::default(),
        Shape::C0,
    ));
    let history = Arc::new(SharedState::new(Vec::new()));
    set_callback(&first, &source, &history, Shape::C1);
    set_callback(&second, &source, &history, Shape::C2);
    (source, history)
}

fn set_callback(
    plane: &Handle<CallbackDumpPlane>,
    source: &Handle<BRepCurveOn2Surfaces>,
    history: &Arc<SharedState<Vec<Shape>>>,
    next: Shape,
) {
    let owner = source.downgrade();
    let history = history.clone();
    plane.set_dump_hook(Arc::new(move || {
        let source = owner.upgrade();
        history.write().push(source.continuity().unwrap());
        source.set_continuity(next).unwrap();
    }));
}

#[test]
fn regularity_dump_continuity_follows_both_surface_callbacks() {
    let (source, history) = regularity_fixture();
    let mut stream = DumpStream::default();
    source.dump_json(&mut stream, -1).unwrap();
    assert_eq!(*history.read(), vec![Shape::C0, Shape::C1]);
    assert_eq!(source.continuity().unwrap(), Shape::C2);
    let text: String = String::from_utf8(stream.bytes().to_vec())
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    assert!(
        text.contains("\"Continuity\":4"),
        "expected C2 after both dumps: {text}"
    );
}

#[test]
fn shallow_regularity_dump_does_not_invoke_surface_callbacks() {
    let (source, history) = regularity_fixture();
    source.dump_json(&mut DumpStream::default(), 0).unwrap();
    assert!(history.read().is_empty());
    assert_eq!(source.continuity().unwrap(), Shape::C0);
}
