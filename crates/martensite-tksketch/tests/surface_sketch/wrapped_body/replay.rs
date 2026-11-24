//! Native graph bytes bind every tool-parent history image across processes and scheduling.
use super::*;
use martensite_tkbrep::brep_tools::History;
type NativeBinding = ((WrappedHistoryKey, &'static str, usize), Shape);

fn signature() -> String {
    [Operation::Fuse, Operation::Cut]
        .into_iter()
        .map(operation_signature)
        .collect()
}
fn operation_signature(kind: Operation) -> String {
    let attachment = fixture::attachment(true);
    let source = attachment.export_native(SupportRevision(3)).unwrap();
    let tool = normal_extrude_profile(&fixture::request(-1., true)).unwrap();
    let native = tool.export_native().unwrap();
    let operation = boolean(&source.body, &native.bodies, kind);
    let rebound = checks::rebound(&attachment, &source, &operation);
    format!(
        "{kind:?}|{:?}|{:?}|{:?}|{:?}|{}",
        rebound.record(),
        rebound.record().brep,
        rebound.validation(),
        tool.history(),
        history_signature(&operation, &native)
    )
}
fn history_signature(
    operation: &BRepAlgoAPIBooleanOperation,
    tool: &DetachedWrappedBodies,
) -> String {
    let history = operation.history().unwrap();
    let bindings: Vec<_> = tool
        .entities
        .iter()
        .flat_map(|(key, entity)| parent_bindings(history, *key, entity))
        .collect();
    let deleted = deletion_signature(history, tool);
    let bytes = super::super::replay::geometry_bytes::bound_signature(
        operation.shape(),
        bindings.iter().map(|(key, shape)| (key, shape)),
    );
    format!("{deleted}{bytes}")
}
fn deletion_signature(history: &History, tool: &DetachedWrappedBodies) -> String {
    tool.entities
        .iter()
        .map(|(parent, entity)| format!("{parent:?}={}|", history.is_removed(entity).unwrap()))
        .collect::<String>()
}
fn parent_bindings(
    history: &History,
    parent: WrappedHistoryKey,
    entity: &Shape,
) -> Vec<NativeBinding> {
    let mut bindings = Vec::new();
    let modified = history.modified(entity).unwrap();
    append(&mut bindings, parent, "modified", modified);
    let generated = history.generated(entity).unwrap();
    append(&mut bindings, parent, "generated", generated);
    if !history.is_removed(entity).unwrap() && bindings.is_empty() {
        bindings.push(((parent, "unchanged", 0), entity.clone()));
    }
    bindings
}
fn append(
    bindings: &mut Vec<NativeBinding>,
    parent: WrappedHistoryKey,
    relation: &'static str,
    images: Vec<Shape>,
) {
    // Native list ordinals measure replay ordering here; they do not assign persistent identity.
    for (index, image) in images.into_iter().enumerate() {
        bindings.push(((parent, relation, index), image));
    }
}
#[test]
#[ignore = "subprocess worker; explicitly run by the process replay test"]
fn child_signature() {
    println!("WRAPPED_BODY_REPLAY={}", signature());
}
fn child_output() -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "wrapped_body::replay::child_signature",
            "--ignored",
            "--nocapture",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    replay_line(output.stdout)
}
fn replay_line(stdout: Vec<u8>) -> String {
    String::from_utf8(stdout)
        .unwrap()
        .lines()
        .find(|line| line.starts_with("WRAPPED_BODY_REPLAY="))
        .unwrap()
        .to_owned()
}

#[test]
fn separate_processes_replay_oblique_seam_crossing_boss_and_pocket() {
    assert_eq!(child_output(), child_output());
}
#[test]
fn concurrent_operations_replay_sequential_history_and_geometry() {
    let expected = signature();
    let actual = std::thread::scope(|scope| {
        let first = scope.spawn(signature);
        let second = scope.spawn(signature);
        [first.join().unwrap(), second.join().unwrap()]
    });
    assert!(actual.iter().all(|value| value == &expected));
}
