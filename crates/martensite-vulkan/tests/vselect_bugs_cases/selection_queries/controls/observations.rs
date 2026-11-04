//! Raw identities are process-local observations, never cross-process topology identity.
use super::*;

/// Retain process-local live observations; `record_scene("mixed", session, names)`.
pub(super) fn record_scene(stage: &str, session: &ViewerSession, names: &[&str]) {
    let operands = selected::selected_operands(session.context()).unwrap();
    let owner_ids: Vec<usize> = operands
        .iter()
        .map(|value| value.owner.object_address() as usize)
        .collect();
    let detected = selected::detected_objects(session.context()).unwrap();
    let detected_ids: Vec<usize> = detected
        .iter()
        .map(|object| object.object_address() as usize)
        .collect();
    let context_id = session.context().object_address() as usize;
    let current_id = current_owner_id(session);
    eprintln!(
        "{{\"control\":{stage:?},\"context_id\":{context_id},\"owner_ids\":{owner_ids:?},\"detected_ids\":{detected_ids:?},\"current_owner_id\":{current_id}}}"
    );
    for name in names {
        record_object(stage, session, name, &detected);
    }
}

fn current_owner_id(session: &ViewerSession) -> usize {
    // Clone the owner before releasing the actual context read lease.
    let current: AisSelectedOwner = {
        let lease = session.context().detected_owner();
        (*lease).clone()
    };
    current.object_address() as usize
}

fn record_object(
    stage: &str,
    session: &ViewerSession,
    name: &str,
    detected: &[Handle<dyn AisInteractiveObject>],
) {
    let object = query_named_object(session, name).unwrap();
    let object_id = object.object_address() as usize;
    let displayed = session.context().is_displayed(&object);
    let selected = session.context().is_selected_object(&object);
    let detected = detected.contains(&object);
    let runtime = object.get().unwrap().dynamic_type().name().to_owned();
    eprintln!(
        "{{\"control\":{stage:?},\"name\":{name:?},\"object_id\":{object_id},\"displayed\":{displayed},\"selected\":{selected},\"detected\":{detected},\"runtime\":{runtime:?}}}"
    );
}

/// Retain the actual owner/rank join; `record_rank(operand, actual_rank)`.
pub(super) fn record_rank(owner: &selected::SelectedOperand, rank: usize) {
    let owner_id = owner.owner.object_address() as usize;
    let object_id = selected::selected_object(owner).unwrap().object_address() as usize;
    eprintln!(
        "{{\"control\":\"actual-face-rank\",\"owner_id\":{owner_id},\"object_id\":{object_id},\"rank\":{rank}}}"
    );
}
