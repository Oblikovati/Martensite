use super::*;
pub(super) fn keep(
    pair: &Pairing<'_>,
    line: &Line,
    transition: &mut Transition,
) -> Result<bool, Failure> {
    let previous = pair.previous()?;
    let defined = !previous.is_unknown();
    let departing = leaving(transition);
    if !defined {
        return Ok(first_endpoint(pair, line, transition, departing));
    }
    let previous_enters = !leaving(&previous);
    if !pair.oriented_edge() && pair.point.index() == pair.last {
        *transition = exiting();
    } else if pair.oriented_edge()
        && !last_endpoint_correction(pair, line, previous_enters, transition)
        && !(previous_enters && departing)
    {
        return Ok(false);
    }
    if pair.same_parameter && pair.seam {
        return Ok(false);
    }
    duplicate_closing_endpoint(pair, line, previous_enters, departing)
}
fn first_endpoint(
    pair: &Pairing<'_>,
    line: &Line,
    transition: &mut Transition,
    departing: bool,
) -> bool {
    let index = pair.point.index();
    let correction =
        pair.found.is_null() && !line.is_v_closed() && index == 1 && index == pair.first;
    let mut keep = !departing || correction;
    if departing && correction {
        *transition = entering();
    }
    if index == pair.last {
        keep = false;
    }
    if !pair.oriented_edge() && index == pair.first {
        *transition = entering();
    }
    keep
}
fn last_endpoint_correction(
    pair: &Pairing<'_>,
    line: &Line,
    previous_enters: bool,
    transition: &mut Transition,
) -> bool {
    let correction = pair.found.is_null()
        && !line.is_v_closed()
        && previous_enters
        && pair.point.index() == pair.last;
    if correction {
        *transition = exiting();
    }
    correction
}
fn duplicate_closing_endpoint(
    pair: &Pairing<'_>,
    line: &Line,
    previous_enters: bool,
    departing: bool,
) -> Result<bool, Failure> {
    if pair.found.is_null() || !pair.same_parameter || !previous_enters {
        return Ok(true);
    }
    let parameter = InterferenceTool::parameter(pair.found)?;
    // PRO12107's source intentionally omits the departing condition in its open-line branch.
    Ok(!((!line.is_v_closed() || departing)
        && parameter == 1.0
        && pair.point.index() == pair.first + 1
        && pair.point.index() != pair.last))
}
