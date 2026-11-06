//! `tests/gdt/begin`, the grid's `begin` and `end`, `tests/gdt/end`: the first dump against `ref_data`.
use crate::de_end::locate_data_file;
use crate::support::script_error::script_error;
use crate::table::{GdtGrid, GdtRow};
use crate::xs_session::XsSession;

fn begin(session: &mut XsSession, row: &GdtRow) {
    println!("cpulimit 1000 ");
    println!("CPU and elapsed time limit set to 1000 seconds");
    session.pilot_command("param write.step.schema 5");
    session.pilot_command("newmodel");
    if matches!(row.grid, GdtGrid::Import | GdtGrid::Export) {
        println!("NewDocument D ");
        session.new_named_document("D");
        println!("document (not handled by application)  D created");
        println!("0:1");
    }
}

/// Runs a row's script: the group's and grid's `begin`, its `TODO`s, the end scripts.
/// Example: `run(&mut session, &row)`.
pub fn run(session: &mut XsSession, row: &GdtRow) {
    begin(session, row);
    for todo in row.todos {
        println!("{todo}");
    }
    let Some(path) = locate_data_file(row.file) else {
        script_error(&format!("File {} could not be found", row.file));
    };
    let path = path.to_string_lossy().into_owned();
    println!("ReadStep D_First {path} ");
    let mist = session.read_step("D_First", &path).is_err();
    if mist {
        println!("Error: First - file was not read - exception ");
    }
    let mut result = String::new();
    if !mist {
        println!();
        result = session.dump_nb_dgts("D_First");
        result.push('\n');
        result += &session.dump_dgts("D_First");
        println!();
    }
    println!("Close D_First ");
    session.close_document("D_First");
    if !mist {
        compare(row, &result);
    }
    println!("--------------------------------------------------------------------");
    println!();
    println!("TEST COMPLETED");
    session.pilot_command("param write.step.schema 4");
    session.pilot_command("newmodel");
}

/// The script's comparison: each current line against the reference line one further on.
fn compare(row: &GdtRow, result: &str) {
    println!("========================== Comparison with reference data ========");
    println!();
    let mut differences = 0;
    let mut report = String::new();
    for (at, current) in result.split('\n').enumerate() {
        let reference = row.reference.get(at + 1).copied().unwrap_or("");
        if reference != current {
            differences += 1;
            report += &format!(
                " Reference data - {reference}\n Current data   - {current}\n--------------------------------------------------------------------\n"
            );
        }
    }
    if differences > 0 {
        println!("Error : {differences} differences with reference data found :\n{report}");
    } else {
        println!("Comparison of current result with reference data - OK\n");
    }
}
