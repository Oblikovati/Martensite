//! A metadata edit retains its original exclusion of walking-line readers (#294).
use martensite_tkernel::standard::Handle;
use martensite_tkgeomalgo::int_patch::{IntPatchPoint, IntPatchPointLine, IntPatchWLine};
use martensite_tkgeomalgo::int_surf::{IntSurfLineOn2S, IntSurfPntOn2S};
use martensite_tkmath::gp::Point3;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, channel};
use std::time::Duration;

type WalkingRead = fn(&IntPatchWLine) -> IntSurfPntOn2S;

fn sample(index: i32) -> IntSurfPntOn2S {
    let x = f64::from(index);
    let mut point = IntSurfPntOn2S::new();
    point.set_value_all(Point3::new(x, 2. * x, -x), x, -x, 2. * x, 4. * x);
    point
}

fn shared_line() -> (Handle<IntSurfLineOn2S>, Handle<IntPatchWLine>) {
    let curve = Handle::new(IntSurfLineOn2S::new(None));
    curve.add(&sample(1));
    let line = Handle::new(IntPatchWLine::new_undecided(&curve, false));
    line.add_vertex(&IntPatchPoint::new(), false);
    (curve, line)
}

fn held_edit(
    line: &IntPatchWLine,
    curve: &IntSurfLineOn2S,
    entered: Sender<()>,
    release: Receiver<()>,
) {
    line.change_vertex(1, &mut |_| {
        curve.set_value(1, &sample(7)).unwrap();
        entered.send(()).unwrap();
        release.recv().unwrap();
        curve.set_value(1, &sample(9)).unwrap();
    })
    .unwrap();
}

fn point_read(line: &IntPatchWLine) -> IntSurfPntOn2S {
    // A point reader must wait for edits coordinated by the metadata writer.
    line.point(1).unwrap()
}

fn public_curve_read(line: &IntPatchWLine) -> IntSurfPntOn2S {
    // Obtaining the public owner participates in the same ordering gate.
    line.curve().value(1).unwrap()
}

fn read_attempt(
    line: &IntPatchWLine,
    read: WalkingRead,
    attempted: Sender<()>,
    observed: Sender<IntSurfPntOn2S>,
) {
    attempted.send(()).unwrap();
    observed.send(read(line)).unwrap();
}

fn observe_held_writer(read: WalkingRead) -> (bool, IntSurfPntOn2S) {
    let (curve, line) = shared_line();
    let (entered_tx, entered_rx) = channel();
    let (release_tx, release_rx) = channel();
    std::thread::scope(|scope| {
        let writer = scope.spawn(|| held_edit(&line, &curve, entered_tx, release_rx));
        entered_rx.recv().unwrap();
        let outcome = read_during_edit(scope, &line, read, release_tx);
        writer.join().unwrap();
        outcome
    })
}

fn read_during_edit<'scope, 'env: 'scope>(
    scope: &'scope std::thread::Scope<'scope, 'env>,
    line: &'scope IntPatchWLine,
    read: WalkingRead,
    release: Sender<()>,
) -> (bool, IntSurfPntOn2S) {
    let (attempted_tx, attempted_rx) = channel();
    let (observed_tx, observed_rx) = channel();
    let reader = scope.spawn(move || read_attempt(line, read, attempted_tx, observed_tx));
    attempted_rx.recv().unwrap();
    let intermediate = observed_rx.recv_timeout(Duration::from_millis(100));
    let blocked = matches!(intermediate, Err(RecvTimeoutError::Timeout));
    release.send(()).unwrap(); // Release before any assertion, including a regression failure.
    reader.join().unwrap();
    (
        blocked,
        intermediate.unwrap_or_else(|_| observed_rx.recv().unwrap()),
    )
}

#[test]
fn metadata_writer_excludes_point_reads_until_its_edit_finishes() {
    let (blocked, observed) = observe_held_writer(point_read);
    assert!(
        blocked,
        "point read exposed an in-flight metadata edit: {observed:?}"
    );
    assert_eq!(observed, sample(9));
}

#[test]
fn metadata_writer_excludes_public_curve_reads_until_its_edit_finishes() {
    let (blocked, observed) = observe_held_writer(public_curve_read);
    assert!(
        blocked,
        "curve read exposed an in-flight metadata edit: {observed:?}"
    );
    assert_eq!(observed, sample(9));
}
