//! What the `transfer_entity` overloads of `StepControlActorRead` share.
use crate::step_entity::step_live;
use crate::step_repr::StepReprRepresentation;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkmath::gp::{Transform3, TransformForm};
use martensite_tkmath::toploc::Location;
use martensite_tkxsbase::interface::InterfaceEntityIterator;
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// `InterfaceEntityIterator` as a list.
fn entities_of(
    iterator: InterfaceEntityIterator,
) -> Result<Vec<Handle<StandardTransient>>, Failure> {
    let mut entities = Vec::new();
    iterator.start();
    while iterator.more() {
        entities.push(iterator.value()?);
        iterator.next();
    }
    Ok(entities)
}

/// `TP->Graph().Sharings(ent)`.
pub(super) fn sharing_entities(
    tp: &Handle<TransferTransientProcess>,
    ent: &Handle<StandardTransient>,
) -> Result<Vec<Handle<StandardTransient>>, Failure> {
    let guard = step_live(tp, "StepControlActorRead")?.graph()?;
    let graph = guard.get();
    entities_of(graph.sharings(ent)?)
}

/// `TP->Graph().Shareds(ent)`.
pub(super) fn shared_entities(
    tp: &Handle<TransferTransientProcess>,
    ent: &Handle<StandardTransient>,
) -> Result<Vec<Handle<StandardTransient>>, Failure> {
    let guard = step_live(tp, "StepControlActorRead")?.graph()?;
    let graph = guard.get();
    entities_of(graph.shareds(ent)?)
}

/// `ent->DynamicType() == STANDARD_TYPE(T)`.
pub(super) fn is_exactly<T: RuntimeTyped>(ent: &Handle<StandardTransient>) -> bool {
    ent.dynamic_type()
        .is_some_and(|kind| std::ptr::eq(kind, T::static_type()))
}

/// `apply_transformation`: moves the shape, unless the transformation is the identity.
pub(super) fn apply_transformation(shape: &mut Shape, trsf: &Transform3) -> Result<(), Failure> {
    if trsf.form() == TransformForm::Identity {
        return Ok(());
    }
    shape.move_by(&Location::new(*trsf), false)
}

/// `find_context`: the representation that shares `start`, at most `level` levels up.
pub(super) fn find_context(
    start: &Handle<StandardTransient>,
    tp: &Handle<TransferTransientProcess>,
    level: i32,
) -> Result<Handle<StepReprRepresentation>, Failure> {
    let mut rep = Handle::<StepReprRepresentation>::default();
    for sharing in sharing_entities(tp, start)? {
        if rep.is_some() {
            break;
        }
        rep = sharing.cast();
        if rep.is_null() && level > 0 {
            rep = find_context(&sharing, tp, level - 1)?;
        }
    }
    Ok(rep)
}
