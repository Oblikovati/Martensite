//! Preserve source read/configure/write/read/assert/delete order and inherited end.
mod exchange;
mod files;
use files::CaseBrepFiles;
use martensite_tkbrep::toptools::THE_ASCII_VERSIONS;

/// The version line of the input file.
const INPUT_VERSION_LINE: &str = "CASCADE Topology V3, (c) Open Cascade";

/// Run the original commands on the BREP file operations; `commands_without_end()?`.
pub(super) fn commands_without_end() -> Result<(), Box<dyn std::error::Error>> {
    emit_brep_heading()?;
    let files = CaseBrepFiles::new();
    let string1 = files.read();
    let shape = exchange::read(files.input_path(), &string1);
    exchange::write(files.output_path(), &shape);
    let string2 = files.read_written();
    let newstr = string2.trim();
    // What is written carries the current version line; the input carries the older one.
    let expected = string1.replacen(INPUT_VERSION_LINE, THE_ASCII_VERSIONS[3], 1);
    assert!(
        expected.trim().contains(newstr),
        "Error: strings is not equal"
    );
    files.capture_written();
    files.remove_written();
    Ok(())
}

/// Preserve the inherited completion after all real commands; `case_body()?`.
pub(super) fn case_body() -> Result<(), Box<dyn std::error::Error>> {
    commands_without_end()?;
    // bugs/begin sets rel_tol=0, so bugs/end only emits its completion marker.
    crate::source_process::emit_source("TEST COMPLETED\n")?;
    Ok(())
}

fn emit_brep_heading() -> std::io::Result<()> {
    crate::source_process::emit_source(
        "===========================================================\n",
    )?;
    crate::source_process::emit_source(
        "0033318: Data Exchange - Modifying the BRep flag after exporting the shape\n",
    )?;
    crate::source_process::emit_source(
        "===========================================================\n",
    )?;
    Ok(())
}

/// The predicate has to see the fault it was written for: a flag that changes between read and write.
#[test]
fn a_flag_changed_after_reading_leaves_the_case_text() {
    let files = CaseBrepFiles::new();
    let original = files.read();
    let shape = exchange::read(files.input_path(), &original);
    let tshape = shape.tshape().get().unwrap();
    tshape.set_checked(!tshape.checked());
    exchange::write(files.output_path(), &shape);
    assert!(!original.trim().contains(files.read_written().trim()));
}
