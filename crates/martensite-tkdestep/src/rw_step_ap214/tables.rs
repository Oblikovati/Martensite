//! The shape of a row of the AP214 tables. Each row names an entity class and its RW class, once.
use crate::step_rw::*;

/// `read_step` of a case.
pub(super) type ReadFn = fn(
    &Handle<StepDataStepReaderData>,
    i32,
    &Handle<InterfaceCheck>,
    &Handle<StandardTransient>,
) -> Result<(), Failure>;
/// `WriteStep` of a case.
pub(super) type WriteFn =
    fn(&mut StepDataStepWriter, &Handle<StandardTransient>) -> Result<(), Failure>;
/// `share` of a case.
pub(super) type ShareFn =
    fn(&Handle<StandardTransient>, &mut InterfaceEntityIterator) -> Result<(), Failure>;
/// `Check` of a case.
pub(super) type CheckFn = fn(
    &Handle<StandardTransient>,
    &InterfaceShareTool,
    &Handle<InterfaceCheck>,
) -> Result<(), Failure>;
/// The entity a case creates, before it is read.
pub(super) type NewFn = fn() -> Handle<StandardTransient>;

/// A case and its reader.
pub(super) type ReadRow = (i32, ReadFn);
/// A case and its writer.
pub(super) type WriteRow = (i32, WriteFn);
/// A case and its sharing.
pub(super) type ShareRow = (i32, ShareFn);
/// A case and its check.
pub(super) type CheckRow = (i32, CheckFn);
/// A case and its new entity.
pub(super) type NewRow = (i32, NewFn);
/// A case and its category.
pub(super) type CategoryRow = (i32, StepCategory);

/// The categories of the entity types: Shape, Drawing, Structure, Description, Auxiliary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum StepCategory {
    Sh,
    Dr,
    Str,
    Dsc,
    Aux,
}

// In each row the entity is cast to its type, then converted to the type its RW class takes,
// which can be a parent, as the handle converts implicitly.

/// A row of the read table: the `DeclareAndCast` then `tool.ReadStep`.
macro_rules! read_row {
    ($cn:literal, $entity:ty, $tool:ty) => {
        ($cn, |data, num, ach, ent| {
            <$tool>::new().read_step(data, num, ach, &ent.cast::<$entity>().cast())
        })
    };
}
pub(super) use read_row;

/// A row of the write table: the `DeclareAndCast` then `tool.WriteStep`.
macro_rules! write_row {
    ($cn:literal, $entity:ty, $tool:ty) => {
        ($cn, |sw, ent| {
            <$tool>::new().write_step(sw, &ent.cast::<$entity>().cast())
        })
    };
}
pub(super) use write_row;

/// A row of the sharing table: the `DeclareAndCast` then `tool.Share`.
macro_rules! share_row {
    ($cn:literal, $entity:ty, $tool:ty) => {
        ($cn, |ent, iter| {
            <$tool>::new().share(&ent.cast::<$entity>().cast(), iter)
        })
    };
}
pub(super) use share_row;

/// A row of the check table: the `DeclareAndCast` then `tool.Check`.
macro_rules! check_row {
    ($cn:literal, $entity:ty, $tool:ty) => {
        ($cn, |ent, shares, ach| {
            <$tool>::new().check(&ent.cast::<$entity>().cast(), shares, ach)
        })
    };
}
pub(super) use check_row;

/// A row of the new entity table: the `ent = new T`.
macro_rules! new_row {
    ($cn:literal, $entity:ty) => {
        ($cn, || Handle::new(<$entity>::new()).cast())
    };
}
pub(super) use new_row;
