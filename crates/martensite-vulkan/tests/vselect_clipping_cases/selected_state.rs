//! Native local-selection ranks use distinct explored identities, never geometric order.
use super::prelude::*;
pub(crate) use crate::selection_occurrence_rank::occurrence_rank;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::ais::{AisInteractiveContext, AisShapeParameters, AisShapeSource};

pub(super) fn local_selected_faces(
    context: &AisInteractiveContext,
) -> Result<Vec<String>, Failure> {
    let mut lines = Vec::new();
    context.init_selected();
    while context.more_selected() {
        if let Some(line) = selected_face_line(context)? {
            lines.push(line);
        }
        context.next_selected();
    }
    Ok(lines)
}

fn selected_face_line(context: &AisInteractiveContext) -> Result<Option<String>, Failure> {
    let object = context.selected_interactive()?.cast::<dyn AisShapeSource>();
    let Some(object) = object.get() else {
        return Ok(None);
    };
    if context.selected_owner()? == object.global_sel_owner() || !context.has_selected_shape() {
        return Ok(None);
    }
    let selected = context.selected_shape()?;
    if selected.shape_type()? != ShapeEnum::Face {
        return Ok(None);
    }
    Ok(occurrence_rank(&object.shape(), &selected)?.map(|rank| format!("Face #{rank}")))
}
