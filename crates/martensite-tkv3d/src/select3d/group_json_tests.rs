//! Native JSON preserves exact diagnostic fragments, depth and NaNs; only addresses vary by process.
use super::group_reference_geometry::group_children;
use super::{Select3DSensitiveEntity, Select3DSensitiveGroup};
use crate::select_mgr::SelectMgrEntityOwner;
use martensite_tkernel::standard::dump::DumpStream;
fn group_json_case(kind: i32, depth: i32) -> String {
    let owner = SelectMgrEntityOwner::new(13).cast();
    let children = group_children(if kind == 4 { 1 } else { kind }, &owner);
    let group = if kind == 0 {
        Select3DSensitiveGroup::new(&owner, true)
    } else {
        Select3DSensitiveGroup::with_entities(&owner, &children, true)
    };
    if kind == 4 {
        group.remove(&children[0]);
    }
    let mut text = DumpStream::default();
    group.dump_json(&mut text, depth).unwrap();
    normalize_group_json_pointers(text.text())
}
pub(super) fn normalize_group_json_pointers(text: &str) -> String {
    let mut normalized = String::new();
    let mut remaining = text;
    // The platform C library decides the spelling: `0xx1f` on Linux and macOS, `0x1F` on Windows.
    while let Some(position) = remaining.find("0x") {
        normalized.push_str(&remaining[..position]);
        normalized.push_str("@pointer");
        let pointer = remaining[position + 2..].trim_start_matches('x');
        let length = pointer.bytes().take_while(u8::is_ascii_hexdigit).count();
        remaining = &pointer[length..];
    }
    normalized.push_str(remaining);
    normalized
}
/// The recording is from x86-64, whose 0/0 sets the NaN sign bit; other CPUs leave it clear.
fn expected_on_this_cpu(recorded: &str) -> String {
    if cfg!(target_arch = "x86_64") {
        recorded.to_owned()
    } else {
        recorded.replace("-nan", "nan")
    }
}
#[test]
fn native_group_json_depth_nan_and_cached_structural_state_are_preserved() {
    let native = include_str!("../../tests/fixtures/sensitive-group-json.txt");
    for line in native.lines() {
        let mut fields = line.splitn(4, ' ');
        let kind = fields.next().unwrap().parse().unwrap();
        let depth = fields.next().unwrap().parse().unwrap();
        assert_eq!(fields.next().unwrap(), "0");
        assert_eq!(
            group_json_case(kind, depth),
            expected_on_this_cpu(fields.next().unwrap()),
            "kind {kind} depth {depth}"
        );
    }
    assert_eq!(native.lines().count(), 30);
}
