//! topology-command-style topology operands survive AIS removal; extraction reuses the actual viewer command.
use super::selected::{selected_object, selected_operands};
use super::*;
use martensite_tkv3d::ais::{AisShapeParameters, AisShapeSource};

#[derive(Default)]
pub(super) struct TopologyOperands(Vec<(String, Shape)>);

impl TopologyOperands {
    /// Retain actual constructor/picked topology separately; `operands.bind("b", shape)`.
    pub(super) fn bind(&mut self, name: &str, shape: Shape) {
        self.0.retain(|(bound, _)| bound != name);
        self.0.push((name.to_owned(), shape));
    }

    /// Refuse an absent topology-command operand rather than substituting geometry; `operands.shape("b")?`.
    pub(super) fn shape(&self, name: &str) -> Result<&Shape, Failure> {
        self.0
            .iter()
            .find(|(bound, _)| bound == name)
            .map(|(_, shape)| shape)
            .ok_or_else(|| {
                program_error(&format!(
                    "topology {name:?}; expected an actual constructor/pick binding"
                ))
            })
    }

    /// Reuse genuine pick_selected and fetch its resulting AIS shapes; `operands.pick(session, "aSelected")?`.
    pub(super) fn pick(
        &mut self,
        session: &mut ViewerSession,
        prefix: &str,
    ) -> Result<Vec<String>, Failure> {
        let operands = selected_operands(session.context())?;
        validate_pick_owners(&operands)?;
        session.pick_selected(prefix)?;
        let names: Vec<String> = (0..operands.len()).map(|i| pick_name(prefix, i)).collect();
        for name in &names {
            self.bind(name, displayed_shape(session, name)?);
        }
        Ok(names)
    }
}

fn validate_pick_owners(operands: &[super::selected::SelectedOperand]) -> Result<(), Failure> {
    for operand in operands {
        let object = selected_object(operand)?.cast::<dyn AisShapeSource>();
        if operand.shape.is_none() && object.is_null() {
            return Err(program_error(&format!(
                "selected owner{:?} has no topology/AisShape; expected actual selected topology or AisShape",
                operand.owner.object_address()
            )));
        }
    }
    Ok(())
}

/// Read the actual displayed shape binding; `displayed_shape(session, name)?`.
pub(super) fn displayed_shape(session: &ViewerSession, name: &str) -> Result<Shape, Failure> {
    let object = query_named_object(session, name)?.cast::<dyn AisShapeSource>();
    let value = object.get().ok_or_else(|| {
        program_error(&format!(
            "picked object {name:?}; expected an actual AisShape"
        ))
    })?;
    let shape = value.shape();
    if shape.is_null() {
        return Err(program_error(&format!(
            "picked shape {name:?}; expected non-null actual topology"
        )));
    }
    Ok(shape)
}

/// Keep first/subsequent native pick names; `pick_name("aSelected", 1)`.
pub(super) fn pick_name(prefix: &str, index: usize) -> String {
    if index == 0 {
        return prefix.to_owned();
    }
    format!("{prefix}{index}")
}

/// Keep IsSame/location and optional IsEqual output; `compare_text(left, right)`.
pub(super) fn compare_text(left: &Shape, right: &Shape) -> &'static str {
    if !left.is_same(right) {
        return "shapes are not same\n";
    }
    if left.is_equal(right) {
        return "same shapes\nequal shapes\n";
    }
    "same shapes\n"
}

/// Preserve a missing-name failure before the session removal; `remove_named(session, name)?`.
pub(super) fn remove_named(session: &mut ViewerSession, name: &str) -> Result<(), Failure> {
    // ViewerSession::remove ignores missing names; the original command does not.
    let _object = query_named_object(session, name)?;
    session.remove(&[name])
}
