//! Drawer dump calls and the authored weak literal depth check (bug30949).
use super::Outcome;
use crate::session_run::{SessionRun, banner};
use martensite_tkernel::standard::dump::{DumpStream, format_json, text};
use martensite_tkservice::graphic3d::Graphic3dPresentationAttributesSource;
use martensite_tkv3d::prs3d::Prs3dDrawer;

/// Execute the full original command sequence; `default_dump()?`.
pub(crate) fn default_dump() -> Outcome {
    banner(&[
        "=============",
        "0030949: Foundation Classes - Dump improvement for classes",
        "=============",
    ]);
    let mut run = SessionRun::new()?;
    let drawer = run.session().context().default_drawer().clone();
    execute_default_dumps(&drawer)?;
    run.finish_without_image()
}

fn execute_default_dumps(drawer: &Prs3dDrawer) -> Outcome {
    // ViewerCommands -dumpcompact uses >= instead of <; valid on/off flags
    // leave toCompactDump=false. Preserve the pinned parser defect.
    let compact = dump(drawer, -1, false)?;
    check_rgba(&compact)?;
    let _deep = dump(drawer, 2, false)?;
    // Reference omits '$': this regexp tests this literal, not the depth-2 dump.
    if four_numbers("dump_aligned_off_deep").is_some() {
        println!("Error in Dump. Wrong deep of output");
    }
    println!("Aligned Dump of vaspects:");
    let _aligned = dump(drawer, -1, false)?;
    println!("Aligned Dump of vaspects (deep = 2):");
    let _aligned_deep = dump(drawer, 2, false)?;
    Ok(())
}

fn dump(drawer: &Prs3dDrawer, depth: i32, compact: bool) -> Result<String, std::io::Error> {
    let mut stream = DumpStream::default();
    drawer.dump_json(&mut stream, depth)?;
    let value = if compact {
        text(&stream)
    } else {
        format_json(&stream, 3)
    };
    Ok(String::from_utf8(value.as_bytes().to_vec()).expect("dump is ASCII JSON"))
}

fn check_rgba(value: &str) -> Outcome {
    let rgba = value
        .match_indices("\"RGBA\": ")
        .find_map(|(start, prefix)| number_array(&value[start + prefix.len()..]));
    let rgba = rgba.ok_or("Error in Dump. Wrong format of output.")?;
    for ((label, actual), expected) in ["Cred", "Cgreen", "Cblue"]
        .into_iter()
        .zip(rgba)
        .zip([0.088428, 0.041081, 0.00209])
    {
        check_component(label, actual, expected)?;
    }
    Ok(())
}

fn check_component(label: &str, actual: f64, expected: f64) -> Outcome {
    if !actual.is_finite() || (actual - expected).abs() > 1e-7 {
        return Err(
            format!("Error: {label} value {actual}, expected {expected} within 1e-7").into(),
        );
    }
    Ok(())
}

// The authored regexp accepts repeated '[' and numeric [-0-9.+eE] tokens,
// comma+one-or-more spaces, and an immediate final ']'. No JSON normalization.
fn four_numbers(value: &str) -> Option<[f64; 4]> {
    value
        .match_indices('[')
        .find_map(|(start, _)| number_array(&value[start..]))
}

fn number_array(value: &str) -> Option<[f64; 4]> {
    let mut tail = value.strip_prefix('[')?.trim_start_matches('[');
    let mut numbers = [0.; 4];
    for (index, number) in numbers.iter_mut().enumerate() {
        let end = tail.find(|ch: char| !ch.is_ascii_digit() && !"-.+eE".contains(ch))?;
        *number = tail[..end].parse().ok()?;
        tail = &tail[end..];
        if index == 3 {
            return tail.starts_with(']').then_some(numbers);
        }
        tail = tail.strip_prefix(',')?.strip_prefix(' ')?;
        tail = tail.trim_start_matches(' ');
    }
    None
}

#[test]
fn authored_dump_numeric_shape_and_literal_depth_operand() {
    assert_eq!(
        four_numbers("RGBA: [[1e-2, -2, +3, 1]"),
        Some([0.01, -2., 3., 1.])
    );
    for invalid in [
        "[1,2, 3, 4]",
        "[1, 2, 3, 4 ]",
        "[nan, 2, 3, 4]",
        "dump_aligned_off_deep",
    ] {
        assert!(
            four_numbers(invalid).is_none(),
            "unexpected accepted operand {invalid}"
        );
    }
    assert!(check_rgba("\"RGBA\": [0.088428, 0.041081, 0.00209, 1]").is_ok());
    assert!(check_rgba("\"RGBA\": [0.088429, 0.041081, 0.00209, 1]").is_err());
}

#[test]
fn actual_default_drawer_executes_all_dump_depth_and_alignment_calls() {
    use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let drawer = session.context().default_drawer().clone();
    for depth in [-1, 2] {
        for compact in [true, false] {
            let actual = dump(&drawer, depth, compact).unwrap();
            assert!(
                !actual.is_empty(),
                "empty drawer dump for depth {depth}, compact {compact}"
            );
            if depth == -1 && compact {
                check_rgba(&actual).unwrap();
            }
        }
    }
    execute_default_dumps(&drawer).unwrap();
    session.close();
}

#[test]
fn rgba_match_starts_at_array_but_searches_later_rgba_fields() {
    let numbers = "[0.088428, 0.041081, 0.00209, 1]";
    assert!(check_rgba(&format!("\"RGBA\": garbage {numbers}")).is_err());
    assert!(check_rgba("\"RGBA\": 0.088428, 0.041081, 0.00209, 1]").is_err());
    assert!(check_rgba(&format!("\"RGBA\": malformed, \"RGBA\": {numbers}")).is_ok());
}
