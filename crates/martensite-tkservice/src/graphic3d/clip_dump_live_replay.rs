//! Child callbacks edit real next nodes while each DumpJson receiver owns one snapshot (#592).
use super::{Graphic3dClipPlane, Graphic3dClipPlaneSource, Graphic3dSequenceOfHClipPlane};
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Handle, SharedState, WeakHandle};
use std::{
    io,
    sync::{Arc, OnceLock},
};
struct ClipDumpLiveEdit {
    remove: WeakHandle<dyn Graphic3dClipPlaneSource>,
    append: WeakHandle<dyn Graphic3dClipPlaneSource>,
}
struct ClipDumpLivePlaybackPlane {
    base: Graphic3dClipPlane,
    identity: OnceLock<WeakHandle<ClipDumpLivePlaybackPlane>>,
    tag: i32,
    collection: WeakHandle<Graphic3dSequenceOfHClipPlane>,
    rows: Arc<SharedState<Vec<String>>>,
    edit: SharedState<Option<ClipDumpLiveEdit>>,
}
martensite_tkernel::impl_transient!(ClipDumpLivePlaybackPlane,
    "Martensite_ClipDumpLivePlaybackPlane", Graphic3dClipPlane,
    interfaces: [dyn Graphic3dClipPlaneSource]);
impl Graphic3dClipPlaneSource for ClipDumpLivePlaybackPlane {
    fn clip_plane_base(&self) -> &Graphic3dClipPlane {
        &self.base
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        let sequence = self.collection.upgrade();
        self.rows.write().push(format!(
            "visit {} {depth} {} {}",
            self.tag,
            sequence.size(),
            self.identity.get().unwrap().ref_count()
        ));
        let edit = self.edit.write().take();
        if let Some(edit) = edit {
            sequence.remove_reference(&edit.remove.upgrade());
            sequence.append_reference(&edit.append.upgrade());
        }
        stream.append(&format!("\"tag\": {}", self.tag));
        Ok(())
    }
}
fn live_dump_plane(
    sequence: &Handle<Graphic3dSequenceOfHClipPlane>,
    rows: &Arc<SharedState<Vec<String>>>,
    tag: i32,
) -> Handle<ClipDumpLivePlaybackPlane> {
    let plane = Handle::new(ClipDumpLivePlaybackPlane {
        base: Graphic3dClipPlane::default(),
        identity: OnceLock::new(),
        tag,
        collection: sequence.downgrade(),
        rows: rows.clone(),
        edit: SharedState::default(),
    });
    plane.identity.set(plane.downgrade()).unwrap();
    plane
}
#[test]
fn null_dump_children_and_depth_zero_replay_native_json_without_projection() {
    let sequence = Graphic3dSequenceOfHClipPlane::default();
    sequence.append_reference(&Handle::<dyn Graphic3dClipPlaneSource>::default());
    let rows: Vec<String> = [-1, 0]
        .into_iter()
        .map(|depth| {
            let mut stream = DumpStream::default();
            sequence.dump_json(&mut stream, depth).unwrap();
            format!("null {depth} {} {}", sequence.size(), stream.text())
        })
        .collect();
    let expected: Vec<&str> = include_str!("../../tests/fixtures/clip-dump-live-native.txt")
        .lines()
        .take(2)
        .collect();
    assert_eq!(rows, expected);
}
struct ClipDumpLiveActors {
    sequence: Handle<Graphic3dSequenceOfHClipPlane>,
    rows: Arc<SharedState<Vec<String>>>,
    first: Handle<ClipDumpLivePlaybackPlane>,
    second: Handle<dyn Graphic3dClipPlaneSource>,
    third: Handle<dyn Graphic3dClipPlaneSource>,
}
fn live_dump_actors() -> ClipDumpLiveActors {
    let sequence = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    let rows = Arc::new(SharedState::default());
    let first = live_dump_plane(&sequence, &rows, 1);
    let second = live_dump_plane(&sequence, &rows, 2).into_cast();
    let third = live_dump_plane(&sequence, &rows, 3).into_cast();
    ClipDumpLiveActors {
        sequence,
        rows,
        first,
        second,
        third,
    }
}
fn prepare_live_dump(actors: &ClipDumpLiveActors) {
    *actors.first.edit.write() = Some(ClipDumpLiveEdit {
        remove: actors.second.downgrade(),
        append: actors.third.downgrade(),
    });
    actors
        .sequence
        .append_reference(&actors.first.upcast::<dyn Graphic3dClipPlaneSource>());
    actors.sequence.append_reference(&actors.second);
    actors.sequence.set_override_global(true);
}
fn live_dump_count_row(actors: &ClipDumpLiveActors) -> String {
    format!(
        "counts {} {} {} {}",
        actors.sequence.ref_count(),
        actors.first.ref_count(),
        actors.second.ref_count(),
        actors.third.ref_count()
    )
}
#[test]
fn virtual_dump_edits_follow_real_nodes_with_one_owned_receiver_snapshot() {
    let actors = live_dump_actors();
    prepare_live_dump(&actors);
    let mut stream = DumpStream::default();
    actors.sequence.dump_json(&mut stream, -1).unwrap();
    actors.rows.write().push(format!("json {}", stream.text()));
    actors.rows.write().push(live_dump_count_row(&actors));
    let expected: Vec<&str> = include_str!("../../tests/fixtures/clip-dump-live-native.txt")
        .lines()
        .skip(2)
        .collect();
    let actual = actors.rows.read();
    // #592: each virtual call owns one Arc snapshot; traversal, JSON and final owners stay native.
    assert_eq!(&expected[..2], &["visit 1 -2 2 2", "visit 3 -2 2 2"]);
    assert_eq!(&actual[..2], &["visit 1 -2 2 3", "visit 3 -2 2 3"]);
    assert_eq!(&actual[2..], &expected[2..]);
}
