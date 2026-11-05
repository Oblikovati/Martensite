use super::*;
pub(super) fn same_parameter(
    line: &Line,
    found: &Handle<Interference>,
    parameter: f64,
    seam: bool,
) -> Result<bool, Failure> {
    if found.is_null() {
        return Ok(false);
    }
    Ok((seam && line.is_periodic())
        || (parameter - InterferenceTool::parameter(found)?).abs() < precision::p_confusion())
}
pub(super) fn periodize(
    line: &mut Line,
    found: &Handle<Interference>,
    parameter: &mut f64,
) -> Result<(), Failure> {
    let mut previous = InterferenceTool::parameter(found)?;
    if !line.is_periodic() {
        *parameter = source_min(*parameter, previous);
        previous = *parameter;
    } else {
        periodic_pair(line, found, &mut previous, parameter)?;
    }
    InterferenceTool::set_parameter(found, previous)
}
fn periodic_pair(
    line: &mut Line,
    found: &Handle<Interference>,
    previous: &mut f64,
    parameter: &mut f64,
) -> Result<(), Failure> {
    let (first, last) = line.bounds()?;
    // Unordered distances follow the native not-close branch, just like equality.
    if (*parameter - first)
        .abs()
        .partial_cmp(&precision::p_confusion())
        != Some(std::cmp::Ordering::Less)
        && (*parameter - last)
            .abs()
            .partial_cmp(&precision::p_confusion())
            != Some(std::cmp::Ordering::Less)
    {
        *parameter = source_min(*parameter, *previous);
        *previous = *parameter;
        return Ok(());
    }
    let orientation = found
        .get()
        .ok_or_else(null_interference)?
        .transition()
        .orientation(State::In, ShapeEnum::Face);
    let pair = oriented_bounds(orientation, first, last)?;
    *previous = pair.0;
    *parameter = pair.1;
    Ok(())
}
fn oriented_bounds(orientation: Orientation, first: f64, last: f64) -> Result<(f64, f64), Failure> {
    match orientation {
        Orientation::Forward => Ok((first, last)),
        Orientation::Reversed => Ok((last, first)),
        _ => Err(Failure::with_message(
            "Standard_ProgramError",
            &format!("FUNBREP_Periodize: {orientation:?}; expected Forward or Reversed"),
        )),
    }
}

fn source_min(first: f64, second: f64) -> f64 {
    // std::min preserves its first operand for unordered values and equal signed zeroes.
    if second < first { second } else { first }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parameter_min_preserves_unordered_values_and_signed_zero() {
        assert!(source_min(f64::NAN, 1.0).is_nan());
        assert_eq!(source_min(1.0, f64::NAN), 1.0);
        assert_eq!(source_min(0.0, -0.0).to_bits(), 0.0_f64.to_bits());
        assert_eq!(source_min(2.0, 1.0), 1.0);
    }
}
