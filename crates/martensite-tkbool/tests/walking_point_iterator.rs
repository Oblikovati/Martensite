//! Native walking-point traversal state, exact coordinates and restart behavior.
use martensite_tkbool::top_ope_brep::{TopOpeBRepLineInter, TopOpeBRepWPointInterIterator};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkernel::standard::Handle;
use martensite_tkgeomalgo::{
    int_patch::{IntPatchLine, IntPatchWLine},
    int_surf::{IntSurfLineOn2S, IntSurfPntOn2S},
};
use martensite_tkmath::gp::Point3;

fn walking_line(count: i32) -> TopOpeBRepLineInter {
    let samples = Handle::new(IntSurfLineOn2S::new(None));
    for index in 1..=count {
        samples.add(&walking_sample(index));
    }
    let source: Handle<dyn IntPatchLine> =
        Handle::new(IntPatchWLine::new_undecided(&samples, false)).cast();
    let mut line = TopOpeBRepLineInter::new();
    line.set_line(
        &source,
        &BRepAdaptorSurface::default(),
        &BRepAdaptorSurface::default(),
    )
    .unwrap();
    line
}

fn walking_sample(index: i32) -> IntSurfPntOn2S {
    let x = f64::from(index);
    let mut sample = IntSurfPntOn2S::new();
    sample.set_value_all(Point3::new(x, 2. * x, 3. * x), x, x + 1., x + 2., x + 3.);
    sample
}

fn assert_current(cursor: &mut TopOpeBRepWPointInterIterator<'_>, index: i32) {
    assert!(cursor.more());
    let x = f64::from(index);
    let sample = cursor.current_wp().unwrap();
    assert_eq!(sample.value().unwrap(), Point3::new(x, 2. * x, 3. * x));
    assert_eq!(sample.parameters().unwrap(), (x, x + 1., x + 2., x + 3.));
}

#[test]
fn default_cursor_keeps_native_zero_index_and_defined_unbound_errors() {
    let mut cursor = TopOpeBRepWPointInterIterator::new();
    assert!(cursor.more());
    assert!(cursor.line_inter().is_none());
    assert_eq!(
        cursor.current_wp().err().unwrap().exception_type(),
        "Standard_NullObject"
    );
    cursor.next();
    assert!(!cursor.more());
    assert_eq!(
        cursor.current_wp().err().unwrap().message(),
        "TopOpeBRepWPointInterIterator::Current"
    );
    assert!(cursor.restart().is_err());
}

#[test]
fn walking_cursor_visits_source_samples_once_and_restarts_one_based() {
    let mut line = walking_line(3);
    let mut cursor = TopOpeBRepWPointInterIterator::with_line(&mut line).unwrap();
    assert_eq!(cursor.line_inter().unwrap().nb_wpoint().unwrap(), 3);
    for index in 1..=3 {
        assert_current(&mut cursor, index);
        cursor.next();
    }
    assert!(!cursor.more());
    assert_eq!(
        cursor.current_wp().err().unwrap().exception_type(),
        "Standard_ProgramError"
    );
    cursor.next();
    cursor.restart().unwrap();
    assert_current(&mut cursor, 1);
}

#[test]
fn empty_and_rebound_sources_reset_native_walk_count() {
    let mut empty = walking_line(0);
    let mut source = walking_line(1);
    let mut cursor = TopOpeBRepWPointInterIterator::with_line(&mut empty).unwrap();
    assert!(!cursor.more());
    cursor.init(&mut source).unwrap();
    assert!(cursor.more());
    assert_eq!(
        cursor.current_wp().unwrap().value().unwrap(),
        Point3::new(1., 2., 3.)
    );
    cursor.next();
    assert!(!cursor.more());
}

#[test]
fn walking_cursor_is_send_and_sync_when_its_line_is_owned_by_the_operation() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<TopOpeBRepWPointInterIterator<'static>>();
}
