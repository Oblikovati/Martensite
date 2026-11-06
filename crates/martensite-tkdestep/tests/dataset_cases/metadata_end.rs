//! `tests/metadata/begin` and `tests/metadata/step/end`: properties and file size through the provider.
//! The keys are `provider.STEP.Martensite.*`.
use crate::de_end::locate_data_file;
use crate::script_commands::image_file;
use crate::support::script_error::script_error;
use crate::table::MetadataRow;
use crate::xs_session::XsSession;

const READ_METADATA: &str = "provider.STEP.Martensite.read.productmetadata : 1";
const WRITE_METADATA: &str = "provider.STEP.Martensite.write.metadata : 1";

fn must(step: Result<(), crate::documents::CommandFailed>, command: &str) {
    if step.is_err() {
        script_error(command);
    }
}

/// `expr {min(max(0.001 * $ref_size, 1), 100)}`.
pub fn size_tolerance(ref_size: i64) -> f64 {
    (0.001 * ref_size as f64).max(1.0).min(100.0)
}

/// Runs a row's script. Example: `run(&mut session, &row)`.
pub fn run(session: &mut XsSession, row: &MetadataRow) {
    println!("cpulimit 1000 ");
    println!("CPU and elapsed time limit set to 1000 seconds");
    let Some(path) = locate_data_file(row.file) else {
        script_error(&format!("File {} could not be found", row.file));
    };
    let path = path.to_string_lossy().into_owned();
    must(
        session.read_file("anDocInput_1", &path, Some(READ_METADATA)),
        "ReadFile anDocInput_1",
    );
    let case = row.source.rsplit('/').next().unwrap_or(row.source);
    let written = image_file(row.source, &format!("{case}_written.step"))
        .to_string_lossy()
        .into_owned();
    must(
        session.write_file("anDocInput_1", &written, Some(WRITE_METADATA)),
        "WriteFile anDocInput_1",
    );
    let size = std::fs::metadata(&written)
        .unwrap_or_else(|cause| script_error(&format!("file size {written}: {cause}")))
        .len() as i64;
    crate::de_end::keep_written_file(&written);
    must(
        session.read_file("anDocInput_2", &written, Some(READ_METADATA)),
        "ReadFile anDocInput_2",
    );
    let metadata = session.get_properties("anDocInput_2");
    if (size - row.ref_size).abs() as f64 > size_tolerance(row.ref_size) {
        println!("Error: Wrong file size {size} instead of {}", row.ref_size);
    }
    if row.check_metadata {
        compare(&metadata, row.reference);
    }
    println!("Close anDocInput_1 ");
    session.close_document("anDocInput_1");
    println!("Close anDocInput_2 ");
    session.close_document("anDocInput_2");
    let _ = std::fs::remove_file(&written);
    println!("TEST COMPLETED");
}

/// `split $aMetaData "\n"`, as the script compares: the reference's last line is the empty one after the
/// last newline, so both are split the same way.
fn compare(metadata: &str, reference: &[&str]) {
    let lines: Vec<&str> = metadata.split('\n').collect();
    for (at, (current, expected)) in lines.iter().zip(reference).enumerate() {
        if current != expected {
            println!("Error: Incorrect metadata at line {at}");
            println!("Expected: \"{expected}\"");
            println!("Actual:   \"{current}\"");
            break;
        }
    }
    if lines.len() != reference.len() {
        println!("Error: Line count is metadata doesn't match the expected value.");
        println!("Expected: {}", reference.len());
        println!("Actual:   {}", lines.len());
    }
}
