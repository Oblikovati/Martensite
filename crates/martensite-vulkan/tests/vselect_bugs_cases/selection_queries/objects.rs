//! Object information comes from live objects and actual detected membership.
use super::selected::{detected_objects, local_rows, selected_object, selected_operands};
use super::*;
use martensite_tkv3d::ais::AisKindOfInteractive;

/// Report live state and actual runtime descriptors; `object_info(session, object, detected)?`.
pub(super) fn object_info(
    session: &ViewerSession,
    object: &Handle<dyn AisInteractiveObject>,
    detected: &[Handle<dyn AisInteractiveObject>],
) -> Result<String, Failure> {
    let value = object
        .get()
        .ok_or_else(|| program_error("null presentation; expected a live AIS object"))?;
    let kind = value.kind_of_interactive();
    let signature = value.signature();
    let runtime = value.dynamic_type().name().to_owned();
    let label = supported_label(kind, signature, &runtime)?;
    let flags = display_flags(session, object, detected);
    Ok(format!("{flags} Type:  {label} ({runtime})"))
}

fn display_flags(
    session: &ViewerSession,
    object: &Handle<dyn AisInteractiveObject>,
    detected: &[Handle<dyn AisInteractiveObject>],
) -> String {
    let displayed = session.context().is_displayed(object);
    let selected = session.context().is_selected_object(object);
    flag_text(displayed, selected, detected.contains(object))
}

fn flag_text(displayed: bool, selected: bool, detected: bool) -> String {
    let shown = if displayed { "Displayed" } else { "Hidden   " };
    let chosen = if selected { " Selected" } else { "         " };
    let hit = if detected { " Detected" } else { "         " };
    format!("{shown}{chosen}{hit}")
}

fn supported_label(
    kind: AisKindOfInteractive,
    signature: i32,
    runtime: &str,
) -> Result<&'static str, Failure> {
    match (kind, signature, runtime) {
        (AisKindOfInteractive::Shape, 0, "AisShape") => Ok("AisShape"),
        (AisKindOfInteractive::Datum, 5, "AisLine") => Ok("AisLine"),
        (AisKindOfInteractive::Datum, 1, "AisPoint") => Ok("AisPoint"),
        actual => Err(program_error(&format!(
            "query class {actual:?}; expected actual AisShape/Line/Point kind, signature and descriptor"
        ))),
    }
}

/// Keep the native missing-name newline and named row; `named_text(session, "aSelected")?`.
pub(super) fn named_text(session: &ViewerSession, name: &str) -> Result<String, Failure> {
    let detected = detected_objects(session.context())?;
    let Some(object) = session.object(name).cloned() else {
        return Ok(format!("{name} doesn't exist!\n"));
    };
    Ok(format!(
        "  {name:<20} {}\n",
        object_info(session, &object, &detected)?
    ))
}

/// Use source count/neutral branching without a nonempty assertion; `state_text(session, names, false)?`.
pub(crate) fn state_text(
    session: &ViewerSession,
    names: &[&str],
    all: bool,
) -> Result<String, Failure> {
    let detected = detected_objects(session.context())?;
    let has_selection = session.context().nb_selected() > 0;
    let selected = selected_operands(session.context())?;
    validate_selected_objects(session, &selected, &detected)?;
    let mut text = String::new();
    if all || !has_selection {
        text.push_str("Neutral-point state:\n");
        append_named(session, names, &detected, &mut text)?;
    } else {
        append_global(session, names, &selected, &detected, &mut text)?;
    }
    text.push_str(&local_rows(session, names, &selected)?);
    Ok(text)
}

fn validate_selected_objects(
    session: &ViewerSession,
    operands: &[super::selected::SelectedOperand],
    detected: &[Handle<dyn AisInteractiveObject>],
) -> Result<(), Failure> {
    for operand in operands {
        let object = selected_object(operand)?;
        let _info = object_info(session, &object, detected)?;
    }
    for object in detected {
        let _info = object_info(session, object, detected)?;
    }
    Ok(())
}

fn append_named(
    session: &ViewerSession,
    names: &[&str],
    detected: &[Handle<dyn AisInteractiveObject>],
    text: &mut String,
) -> Result<(), Failure> {
    for name in names {
        let object = query_named_object(session, name)?;
        text.push_str(&format!(
            "  {name:<20} {}\n",
            object_info(session, &object, detected)?
        ));
    }
    Ok(())
}

fn append_global(
    session: &ViewerSession,
    names: &[&str],
    operands: &[super::selected::SelectedOperand],
    detected: &[Handle<dyn AisInteractiveObject>],
    text: &mut String,
) -> Result<(), Failure> {
    for operand in operands {
        let object = selected_object(operand)?;
        if operand.owner == object.global_sel_owner() {
            let name = names.iter().find(|name| session.object(name) == Some(&object)).copied().ok_or_else(|| program_error(&format!("selected AIS object{:?} not in names{names:?}; expected a declared live binding", object.object_address())))?;
            text.push_str(&format!(
                "{name:<20} {}\n",
                object_info(session, &object, detected)?
            ));
        }
    }
    Ok(())
}
