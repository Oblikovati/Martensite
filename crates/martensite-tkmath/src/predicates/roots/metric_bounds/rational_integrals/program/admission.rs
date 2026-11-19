// SPDX-License-Identifier: GPL-2.0-only
use super::run::State;
use super::*;

pub(super) fn admit(
    loops: &[IntegralProgramLoop<'_>],
    state: &mut State,
) -> Result<usize, IntegralProgramError> {
    let options = state.options;
    super::super::publish::admit_options(options.integration)
        .map_err(|cause| state.numerical(cause))?;
    admit_policy(state)?;
    if loops.is_empty() {
        return Err(state.error(IntegralProgramFailure::Empty));
    }
    admit_count(loops, state)?;
    admit_loops(loops, state)
}
fn admit_loops(
    loops: &[IntegralProgramLoop<'_>],
    state: &mut State,
) -> Result<usize, IntegralProgramError> {
    let mut occurrences = 0;
    for (index, current) in loops.iter().enumerate() {
        state.position = (Some(index), None, None);
        if current.components.is_empty() {
            return Err(state.error(IntegralProgramFailure::Empty));
        }
        occurrences += admit_components(current, state, occurrences)?;
    }
    state.position = (None, None, None);
    Ok(occurrences)
}
fn admit_components(
    current: &IntegralProgramLoop<'_>,
    state: &mut State,
    mut offset: usize,
) -> Result<usize, IntegralProgramError> {
    let initial = offset;
    for (index, component) in current.components.iter().enumerate() {
        state.position.1 = Some(index);
        state.position.2 = Some(offset);
        super::super::publish::admit_boundary(component, state.options.integration)
            .map_err(|cause| state.numerical(cause))?;
        offset += component.len();
    }
    Ok(offset - initial)
}

fn admit_count(
    loops: &[IntegralProgramLoop<'_>],
    state: &State,
) -> Result<(), IntegralProgramError> {
    let mut actual = 0;
    for current in loops {
        actual += current.components.len();
        if actual > usize::from(state.options.max_components) {
            return Err(state.error(IntegralProgramFailure::ComponentCount {
                actual,
                limit: state.options.max_components,
            }));
        }
    }
    Ok(())
}

fn admit_policy(state: &State) -> Result<(), IntegralProgramError> {
    let options = state.options;
    match options.version {
        IntegralProgramVersion::V1 => {}
    }
    let count = u64::from(options.max_components);
    if !(1..=64).contains(&count) {
        return Err(state.error(IntegralProgramFailure::Policy {
            field: "max_components",
            actual: count,
            minimum: 1,
            maximum: 64,
        }));
    }
    admit_cell_cap(state)
}
fn admit_cell_cap(state: &State) -> Result<(), IntegralProgramError> {
    let options = state.options;
    let cap =
        u64::from(options.integration.max_curves) * u64::from(options.integration.max_subintervals);
    if options.max_total_subintervals > cap {
        return Err(state.error(IntegralProgramFailure::Policy {
            field: "max_total_subintervals",
            actual: options.max_total_subintervals,
            minimum: 0,
            maximum: cap,
        }));
    }
    Ok(())
}
