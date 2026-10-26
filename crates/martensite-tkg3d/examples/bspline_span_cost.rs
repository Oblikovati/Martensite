//! A generated straight spline separates construction from cross-span evaluation costs (#309).
#[cfg(test)]
#[path = "span_cost/contracts/mod.rs"]
mod contracts;
#[path = "span_cost/fixture.rs"]
mod fixture;
#[path = "span_cost/samples.rs"]
mod samples;
use martensite_tkernel::osd::Chronometer;
use std::{
    error::Error,
    io::{self, BufWriter, Write},
    time::Instant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvaluationMode {
    Closed,
    Cached,
}

fn arguments(arguments: &[String]) -> Result<(EvaluationMode, i32), io::Error> {
    let [mode, spans] = arguments else {
        return Err(invalid(arguments, "closed|cached and span count2..32768"));
    };
    Ok((mode_value(mode)?, span_count(spans)?))
}

fn invalid(value: impl std::fmt::Debug, expected: &str) -> io::Error {
    let message = format!("argument {value:?}: expected {expected}");
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn mode_value(mode: &str) -> Result<EvaluationMode, io::Error> {
    match mode {
        "closed" => Ok(EvaluationMode::Closed),
        "cached" => Ok(EvaluationMode::Cached),
        _ => Err(invalid(mode, "mode closed or cached")),
    }
}

fn span_count(spans: &str) -> Result<i32, io::Error> {
    let count = spans
        .parse::<i32>()
        .map_err(|_| invalid(spans, "integer2..32768"))?;
    if !(2..=32768).contains(&count) {
        return Err(invalid(count, "integer2..32768"));
    }
    Ok(count)
}

fn stage<T>(
    label: &str,
    body: impl FnOnce() -> Result<T, Box<dyn Error>>,
) -> Result<T, Box<dyn Error>> {
    let start = Instant::now();
    let (user, system) = Chronometer::get_process_cpu();
    let result = body()?;
    let (last_user, last_system) = Chronometer::get_process_cpu();
    eprintln!(
        "{{\"phase\":\"{label}\",\"wall_seconds\":{},\"user_seconds\":{},\"system_seconds\":{}}}",
        start.elapsed().as_secs_f64(),
        last_user - user,
        last_system - system
    );
    Ok(result)
}

fn run(mode: EvaluationMode, spans: i32, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let adaptor = stage("construction", || fixture::axis_adaptor(spans))?;
    output.write_all(b"MSPL309\0")?;
    output.write_all(&spans.to_le_bytes())?;
    stage("evaluation", || {
        for index in 0..spans {
            let parameter = f64::from(index) + 0.5;
            let actual = match mode {
                EvaluationMode::Closed => samples::closed_values(parameter),
                EvaluationMode::Cached => samples::cached_values(&adaptor, parameter)?,
            };
            samples::checked_record(output, parameter, actual)?;
        }
        Ok(())
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let values: Vec<String> = std::env::args().skip(1).collect();
    let (mode, spans) = arguments(&values)?;
    eprintln!("{{\"mode\":\"{mode:?}\",\"spans\":{spans}}}");
    let mut output = BufWriter::new(io::stdout().lock());
    run(mode, spans, &mut output)?;
    output.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_both_equal_length_cost_modes() {
        assert_eq!(
            arguments(&["closed".into(), "512".into()]).unwrap(),
            (EvaluationMode::Closed, 512)
        );
        assert_eq!(
            arguments(&["cached".into(), "2048".into()]).unwrap(),
            (EvaluationMode::Cached, 2048)
        );
    }

    #[test]
    fn refuses_invalid_modes_shapes_and_counts() {
        for input in [
            vec![],
            vec!["bad".into(), "512".into()],
            vec!["cached".into(), "0".into()],
            vec!["cached".into(), "32769".into()],
        ] {
            assert!(arguments(&input).is_err());
        }
    }

    #[test]
    fn closed_and_cached_modes_preserve_the_complete_small_raw_bitstream() {
        let (mut closed, mut cached) = (Vec::new(), Vec::new());
        run(EvaluationMode::Closed, 8, &mut closed).unwrap();
        run(EvaluationMode::Cached, 8, &mut cached).unwrap();
        assert_eq!(cached, closed);
        assert_eq!(cached.len(), 12 + 8 * 31 * 8);
    }
}
