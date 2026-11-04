//! Real cursor snapshots own their handles; no context/selector lease crosses virtual calls.
use super::*;
use martensite_tkv3d::ais::{AisShapeParameters, AisShapeSource};

pub(super) struct SelectedOperand {
    pub owner: AisSelectedOwner,
    pub shape: Option<Shape>,
}

/// Own cursor operands without retaining a context lease; `selected_operands(context)?`.
pub(super) fn selected_operands(
    context: &AisInteractiveContext,
) -> Result<Vec<SelectedOperand>, Failure> {
    let mut operands = Vec::new();
    context.init_selected();
    while context.more_selected() {
        let owner = context.selected_owner()?;
        let shape = context
            .has_selected_shape()
            .then(|| context.selected_shape())
            .transpose()?;
        operands.push(SelectedOperand { owner, shape });
        context.next_selected();
    }
    Ok(operands)
}

/// Resolve every actual detected cursor entry, not only last-picked; `detected_objects(context)?`.
pub(super) fn detected_objects(
    context: &AisInteractiveContext,
) -> Result<Vec<Handle<dyn AisInteractiveObject>>, Failure> {
    let mut owners = Vec::new();
    context.init_detected();
    while context.more_detected() {
        owners.push(context.detected_current_owner()?);
        context.next_detected();
    }
    // Cursor/selector API returns owned handles; callbacks run after cursor acquisition ends.
    owners.iter().map(owner_object).collect()
}

/// Resolve a retained owner after acquisition; `selected_object(operand)?`.
pub(super) fn selected_object(
    operand: &SelectedOperand,
) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    // The owned operand remains alive while its selectable callback executes.
    owner_object(&operand.owner)
}

fn owner_object(owner: &AisSelectedOwner) -> Result<Handle<dyn AisInteractiveObject>, Failure> {
    let value = owner
        .get()
        .ok_or_else(|| program_error("owner0; expected a live selected/detected owner"))?;
    let object: Handle<dyn AisInteractiveObject> = value.selectable().cast();
    if object.is_null() {
        return Err(program_error(&format!(
            "owner{:?} has no AIS selectable; expected AisShape/Line/Point",
            owner.object_address()
        )));
    }
    Ok(object)
}

/// Call the genuine InitSelected/HasSelectedShape pair; `has_selected_text(context)`.
pub(super) fn has_selected_text(context: &AisInteractiveContext) -> String {
    context.init_selected();
    let value = i32::from(context.has_selected_shape());
    format!("Check if context has selected shape: {value}\n")
}

/// Use the one shared distinct Explorer/IsSame rank; `local_rows(session, names, operands)?`.
pub(super) fn local_rows(
    session: &ViewerSession,
    names: &[&str],
    operands: &[SelectedOperand],
) -> Result<String, Failure> {
    let mut previous = String::new();
    let mut text = String::new();
    for operand in operands {
        if let Some((name, kind, rank)) = local_rank(session, names, operand)? {
            append_local(&mut text, &mut previous, name, kind, rank);
        }
    }
    Ok(text)
}

fn append_local(text: &mut String, previous: &mut String, name: String, kind: &str, rank: usize) {
    if name != *previous {
        text.push_str(&format!("Locally selected sub-shapes within {name}:\n"));
        *previous = name;
    }
    text.push_str(&format!("  {kind} #{rank}\n"));
}

fn local_rank(
    session: &ViewerSession,
    names: &[&str],
    operand: &SelectedOperand,
) -> Result<Option<(String, &'static str, usize)>, Failure> {
    let object = selected_object(operand)?;
    let shape_object = object.cast::<dyn AisShapeSource>();
    if shape_object.is_null() || operand.owner == object.global_sel_owner() {
        return Ok(None);
    }
    let Some(selected) = &operand.shape else {
        return Ok(None);
    };
    let parent = shape_object.shape();
    let rank = crate::vselect_clipping_cases::occurrence_rank(&parent, selected)?;
    let Some(rank) = rank else { return Ok(None) };
    let name = local_parent_name(session, names, &object)?;
    Ok(Some((name, local_kind(selected)?, rank)))
}

fn local_parent_name(
    session: &ViewerSession,
    names: &[&str],
    object: &Handle<dyn AisInteractiveObject>,
) -> Result<String, Failure> {
    let name = names
        .iter()
        .find(|name| session.object(name) == Some(object));
    name.map(|name| (*name).to_owned()).ok_or_else(|| {
        program_error(&format!(
            "local AIS object{:?} not in names{names:?}; expected a source-declared named parent",
            object.object_address()
        ))
    })
}

fn local_kind(shape: &Shape) -> Result<&'static str, Failure> {
    use martensite_tkg3d::top_abs::ShapeEnum::*;
    Ok(match shape.shape_type()? {
        Compound => " Compound",
        CompSolid => "CompSolid",
        Solid => "    Solid",
        Shell => "    Shell",
        Face => "     Face",
        Wire => "     Wire",
        Edge => "     Edge",
        Vertex => "   Vertex",
        Shape => "    Shape",
    })
}
