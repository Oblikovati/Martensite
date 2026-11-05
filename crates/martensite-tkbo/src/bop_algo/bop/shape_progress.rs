//! A Boolean shape keeps the explicit unbound open-solid branch and real split/container polls.
use super::BopHooks;
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use crate::{bop_algo::Operation, bop_ds::DataStructure};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::Failure;
/// Keep actual virtual open-solid checks inside the native shape scope; `build_boolean_shape_progress(h, ds, range, observed)?`.
pub(super) fn build_boolean_shape_progress<H: BopHooks>(
    hooks: &mut H,
    structure: &DataStructure,
    range: &ProgressRange<'_>,
    observed: &ProgressCancellation,
) -> Result<(), Failure> {
    let scope = ProgressScope::unnamed(range, 10., false);
    let open = hooks.boolean().dims == [3, 3] && hooks.check_args_for_open_solid(structure)?;
    let bop = hooks.boolean();
    if open && bop.try_open_solid_result(structure)? {
        return Ok(());
    }
    bop.build_rc_progress_raw(structure, &scope.next(2.), observed)?;
    if bop.operation == Operation::Fuse && bop.dims[0] == 3 {
        return bop.build_solid_progress_raw(&scope.next(8.), observed);
    }
    if observe_user_break(bop.builder().options(), &scope, observed) {
        return Ok(());
    }
    bop.build_containers_progress(&scope, observed)
}
