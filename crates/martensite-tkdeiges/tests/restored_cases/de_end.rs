//! `tests/de/end`, once: the whole of every `de/iges_*` case. It reads the file into a document, analyses
//! it, writes it, reads that back, analyses it again, then compares both with the row's reference under
//! the case's own tolerances. `tests/de/parse.rules` then grades the stream.
use crate::iges_session::{IgesSession, image_folder, keep_written_file, locate_data_file};
use crate::reports::{
    CheckCounts, ContentCounts, DocumentStatistics, check_counts, contents_of, statistics_of,
    whole_tolerance,
};
use crate::script_text::{
    general_14_10, list_words, lsort_text, numbers_of_line, sum_before_marker,
};
use crate::support::{
    checkview::screenshot,
    nbshapes::{Counting, nbshapes},
    session::{Session, ViewStep},
};
use crate::table::DeRow;
use std::path::Path;

/// What the script sets for one read: its analyses of that document.
#[derive(Clone, Debug, Default)]
struct DocumentFacts {
    data_faulties: i64,
    data_warnings: i64,
    transfer_faulties: i64,
    transfer_warnings: i64,
    statistics: DocumentStatistics,
    max_tolerance: f64,
    avg_tolerance: f64,
    contents: ContentCounts,
    solids: i64,
    shells: i64,
    faces: i64,
    checks: CheckCounts,
}

impl DocumentFacts {
    fn data_sum(&self) -> i64 {
        self.data_faulties + self.data_warnings
    }
    fn transfer_sum(&self) -> i64 {
        self.transfer_faulties + self.transfer_warnings
    }
}

/// `tests/de/begin` as far as it writes: its document, undo limit and transaction.
fn run_begin(session: &mut IgesSession) {
    println!("cpulimit 1000 ");
    println!("CPU and elapsed time limit set to 1000 seconds");
    println!("NewDocument D BinXCAF ");
    session.new_document("D");
    println!("0:1");
    println!("UndoLimit D 100 ");
    println!("100 0 0");
    println!("NewCommand D ");
}

/// Runs a row's script: `de/begin`, its `TODO`s, then `de/end`.
/// Example: `run(&mut IgesSession::new("de_iges_1_B2"), &row)`.
pub fn run(session: &mut IgesSession, row: &DeRow) {
    run_begin(session);
    for todo in row.todos {
        println!("{todo}");
    }
    End::new(session, row).run();
}

struct End<'a> {
    session: &'a mut IgesSession,
    row: &'a DeRow,
    mist: i32,
    mist_write: bool,
    mist_second: bool,
    todo: bool,
    first: DocumentFacts,
    second: DocumentFacts,
}

impl<'a> End<'a> {
    fn new(session: &'a mut IgesSession, row: &'a DeRow) -> Self {
        Self {
            session,
            row,
            mist: 0,
            mist_write: false,
            mist_second: false,
            todo: false,
            first: DocumentFacts::default(),
            second: DocumentFacts::default(),
        }
    }

    fn run(mut self) {
        let row = self.row;
        let folder = image_folder(row.source);
        self.read_original();
        if self.mist < 1 {
            self.first = self.analyse("D_First", "First", "res_1");
        }
        if self.mist < 1 {
            self.write_and_read_back(&folder);
        }
        if self.mist_write || self.mist_second {
            self.mist = 2;
        }
        if self.mist < 1 {
            self.second = self.analyse("D_Second", "Second", "res_2");
        }
        let checks = self.collect_checks();
        let report = (self.mist != 1).then(|| self.report());
        self.print_results(&checks, report.as_deref());
        self.compare_with_reference(report.as_deref());
        self.end_view();
        println!("TEST COMPLETED");
    }

    fn print_error(&mut self, message: &str) {
        println!("{message}");
        self.todo = true;
    }

    fn read_original(&mut self) {
        let row = self.row;
        println!("XNewDoc D_First ");
        self.session.new_document("D_First");
        let Some(path) = locate_data_file(row.file) else {
            crate::support::script_error::script_error(&format!(
                "File {} could not be found",
                row.file
            ));
        };
        let path = path.to_string_lossy().into_owned();
        println!("ReadIges D_First {path} ");
        if self.session.read_iges("D_First", &path).is_err() {
            self.print_error("Error: First - file was not read - exception ");
            self.mist = 1;
        }
    }

    fn analyse(&mut self, document_name: &str, which: &str, shape_name: &str) -> DocumentFacts {
        let mut facts = DocumentFacts::default();
        println!();
        self.read_data_check(&mut facts);
        self.read_transfer_check(&mut facts);
        println!();
        println!("XStat {document_name} ");
        facts.statistics = statistics_of(&self.session.documents[document_name]);
        if !facts.statistics.printed {
            println!(" DECAF information was NOT provided");
        }
        println!();
        println!("XGetOneShape {shape_name} {document_name} ");
        if self
            .session
            .get_one_shape(shape_name, document_name)
            .is_err()
        {
            let message = format!("Error : {which} - cannot get shape from document - exception");
            self.print_error(&message);
            self.mist = 1;
            return facts;
        }
        self.analyse_shape(shape_name, &mut facts);
        facts
    }

    fn analyse_shape(&mut self, shape_name: &str, facts: &mut DocumentFacts) {
        let shape = self.session.bound_shape(shape_name);
        println!();
        println!("tolerance {shape_name} ");
        (facts.max_tolerance, facts.avg_tolerance) = whole_tolerance(&shape);
        println!();
        println!("statshape {shape_name} ");
        facts.contents = contents_of(&shape);
        println!();
        println!("nbshapes {shape_name} ");
        let counts = nbshapes(&self.session.raster, shape_name, Counting::Located);
        (facts.faces, facts.shells, facts.solids) =
            (counts[3] as i64, counts[4] as i64, counts[5] as i64);
        println!("checkshape {shape_name} ff ");
        facts.checks = check_counts(&shape, shape_name);
    }

    fn read_data_check(&mut self, facts: &mut DocumentFacts) {
        let text = self.session.pilot_command("data c");
        if list_words_are_empty(&text) {
            println!(" DATA c checking was NOT provided");
            return;
        }
        facts.data_warnings = sum_before_marker(&text, "W:");
        facts.data_faulties = sum_before_marker(&text, "F:");
    }

    fn read_transfer_check(&mut self, facts: &mut DocumentFacts) {
        println!();
        let text = self.session.pilot_command("tpstat c");
        if list_words_are_empty(&text) {
            println!(" TPSTAT c checking was NOT provided");
            return;
        }
        self.todo |= reports_failed_message(&text);
        facts.transfer_warnings = sum_before_marker(&text, "W:");
        facts.transfer_faulties = sum_before_marker(&text, "F:");
    }

    fn write_and_read_back(&mut self, folder: &Path) {
        println!(" \n-----------------------------WRITING FILE ------------------------------");
        let extension = self.row.file.rsplit_once('.').map_or("", |parts| parts.1);
        let file = folder.join(format!("{}_D_First.{extension}", case_of(self.row)));
        let _ = std::fs::remove_file(&file);
        let file = file.to_string_lossy().into_owned();
        println!("WriteIges D_First {file} ");
        if self.session.write_iges("D_First", &file).is_err() {
            self.print_error("Error: First - file was not written - exception");
            self.mist_write = true;
        }
        keep_written_file(&file);
        if !self.mist_write {
            self.read_back(&file);
        }
        let _ = std::fs::remove_file(&file);
        println!("Close D_First ");
        self.session.close_document("D_First");
    }

    fn read_back(&mut self, file: &str) {
        println!("XNewDoc D_Second ");
        self.session.new_document("D_Second");
        println!("ReadIges D_Second {file} ");
        if self.session.read_iges("D_Second", file).is_err() {
            self.print_error("Error: Second - file was not read - exception");
            self.mist_second = true;
        }
    }

    /// `Collect results of loop back test`: the ten check lines; empty when a read failed.
    fn collect_checks(&mut self) -> Vec<String> {
        if self.mist >= 1 {
            return Vec::new();
        }
        let (a, b) = (&self.first, &self.second);
        let (sa, sb) = (&a.statistics, &b.statistics);
        let lines = [
            (
                "DATA",
                a.data_sum() < b.data_sum() || a.data_faulties < b.data_faulties,
            ),
            (
                "TPSTAT",
                a.transfer_faulties < b.transfer_faulties
                    || a.transfer_warnings < b.transfer_warnings,
            ),
            (
                "CHECKSHAPE",
                a.checks.wires < b.checks.wires
                    || a.checks.faces < b.checks.faces
                    || a.checks.shells < b.checks.shells
                    || a.checks.solids < b.checks.solids,
            ),
            (
                "NBSHAPES",
                a.solids < b.solids || a.shells < b.shells || a.faces < b.faces,
            ),
            (
                "STATSHAPE",
                a.contents.solids < b.contents.solids
                    || a.contents.shells < b.contents.shells
                    || a.contents.faces < b.contents.faces
                    || a.contents.free_wires < b.contents.free_wires,
            ),
            (
                "TOLERANCE",
                tolerance_grew(a.max_tolerance, b.max_tolerance),
            ),
            (
                "LABELS",
                sa.levels[..3] != sb.levels[..3]
                    || sa.total != sb.total
                    || sa.names != sb.names
                    || sa.with_color != sb.with_color
                    || sa.with_layer != sb.with_layer,
            ),
            (
                "PROPS",
                sa.centroids != sb.centroids || sa.volumes != sb.volumes || sa.areas != sb.areas,
            ),
            ("COLORS", sa.colors != sb.colors),
            ("LAYERS", sa.layers != sb.layers),
        ];
        self.todo |= lines.iter().any(|(_, faulty)| *faulty);
        lines
            .iter()
            .map(|(name, faulty)| format!("{name} : {}", if *faulty { "Faulty" } else { "OK" }))
            .collect()
    }

    /// The 12 report lines, each `current ( first )`.
    fn report(&self) -> String {
        report_text(&self.first, &self.second)
    }

    fn print_results(&mut self, checks: &[String], report: Option<&str>) {
        println!("\n\n########################### RESULTS ###############################\n");
        println!("========================== Loop Back test =========================\n");
        if self.mist == 1 {
            self.print_error("Error : Here is reading problem");
            return;
        }
        println!("File was read PROPERLY - OK\n");
        if self.mist_write {
            self.print_error("Error : Here is writing problem");
            return;
        }
        println!("File was write PROPERLY - OK\n");
        if self.mist_second {
            self.print_error("Error : Here is reading problem for writing file");
            return;
        }
        println!("Writing file was read PROPERLY - OK\n");
        for check in checks {
            println!("{check}");
        }
        println!();
        if self.todo {
            println!("{}\n", report.unwrap_or_default());
        }
    }

    fn compare_with_reference(&mut self, report: Option<&str>) {
        let Some(report) = report else {
            self.print_comparison(&[]);
            return;
        };
        println!("========================== Comparison with reference data ========\n");
        let lines: Vec<&str> = report.split('\n').collect();
        let mut differing = Vec::new();
        for (index, current) in lines.iter().enumerate() {
            let reference = self.row.reference.get(index + 1).copied().unwrap_or("");
            if !line_agrees(reference, current) {
                differing.push(format!(
                    " Reference data - {reference}\n Current data   - {current}\n----------------------------------------------\n"
                ));
            }
        }
        self.print_comparison(&differing);
    }

    fn print_comparison(&self, differing: &[String]) {
        if differing.is_empty() {
            println!("Comparison of current result with reference data - OK\n");
        } else {
            println!(
                "Error : {} differences with reference data found :\n{}",
                differing.len(),
                differing.concat()
            );
        }
        println!("--------------------------------------------------------------------\n");
    }

    fn end_view(&mut self) {
        let raster = &mut self.session.raster;
        raster.view(ViewStep::Clear);
        raster.view(ViewStep::SmallView);
        if self.mist < 1 && raster.shape("res_2").is_some() {
            raster.view(ViewStep::Donly(&["res_2"]));
        }
        raster.view(ViewStep::Fit);
        screenshot(raster, case_of(self.row));
        if self.mist < 1 {
            println!("Close D_Second ");
            self.session.close_document("D_Second");
        }
    }
}

/// The script's name: `tests/de/iges_1/B2` is `B2`.
fn case_of(row: &DeRow) -> &str {
    row.source.rsplit('/').next().unwrap_or(row.source)
}

fn list_words_are_empty(text: &str) -> bool {
    list_words(text).is_empty()
}

/// `$err_list`: `[^\r\n:]*failed[^\r\n]*` found anywhere, which puts a line in `todo_msg`.
fn reports_failed_message(text: &str) -> bool {
    text.contains("failed")
}

/// The script's `TOLERANCE` test: a growth above 1e-4 relative and 1e-7 absolute, or a value lost.
fn tolerance_grew(first: f64, second: f64) -> bool {
    let grew = second - first > 0.0001 * first && (first - second).abs() > 0.000_000_1;
    grew || (second == 0.0 && first > 0.0)
}

/// `$result`: one line per fact, the read-back's value then the first read's.
fn report_text(a: &DocumentFacts, b: &DocumentFacts) -> String {
    let (sa, sb) = (&a.statistics, &b.statistics);
    let mut text = String::new();
    text += &format!(
        "DATA        : Faulties = {}  ( {} )  Warnings = {}  ( {} )  Summary  = {}  ( {} )\n",
        b.data_faulties,
        a.data_faulties,
        b.data_warnings,
        a.data_warnings,
        b.data_sum(),
        a.data_sum()
    );
    text += &format!(
        "TPSTAT      : Faulties = {}  ( {} )  Warnings = {}  ( {} )  Summary  = {}  ( {} )\n",
        b.transfer_faulties,
        a.transfer_faulties,
        b.transfer_warnings,
        a.transfer_warnings,
        b.transfer_sum(),
        a.transfer_sum()
    );
    text += &format!(
        "CHECKSHAPE  : Wires    = {}  ( {} )  Faces    = {}  ( {} )  Shells   = {}  ( {} )   Solids   = {} ( {} )\n",
        b.checks.wires,
        a.checks.wires,
        b.checks.faces,
        a.checks.faces,
        b.checks.shells,
        a.checks.shells,
        b.checks.solids,
        a.checks.solids
    );
    text += &format!(
        "NBSHAPES    : Solid    = {}  ( {} )  Shell    = {}  ( {} )  Face     = {}  ( {} ) \n",
        b.solids, a.solids, b.shells, a.shells, b.faces, a.faces
    );
    text += &format!(
        "STATSHAPE   : Solid    = {}  ( {} )  Shell    = {}  ( {} )  Face     = {}  ( {} )   FreeWire = {}  ( {} ) \n",
        b.contents.solids,
        a.contents.solids,
        b.contents.shells,
        a.contents.shells,
        b.contents.faces,
        a.contents.faces,
        b.contents.free_wires,
        a.contents.free_wires
    );
    text += &format!(
        "TOLERANCE   : MaxTol   = {}  ( {} )  AvgTol   =  {}  (  {} )\n",
        general_14_10(b.max_tolerance),
        general_14_10(a.max_tolerance),
        general_14_10(b.avg_tolerance),
        general_14_10(a.avg_tolerance)
    );
    text += &format!(
        "LABELS      : N0Labels = {}  ( {} )  N1Labels = {}  ( {} )  N2Labels = {}  ( {} )   TotalLabels = {}  ( {} )   NameLabels = {}  ( {} )   ColorLabels = {}  ( {} )   LayerLabels = {}  ( {} )\n",
        sb.levels[0],
        sa.levels[0],
        sb.levels[1],
        sa.levels[1],
        sb.levels[2],
        sa.levels[2],
        sb.total,
        sa.total,
        sb.names,
        sa.names,
        sb.with_color,
        sa.with_color,
        sb.with_layer,
        sa.with_layer
    );
    text += &format!(
        "PROPS       : Centroid = {}  ( {} )  Volume   = {}  ( {} )  Area     = {}  ( {} )\n",
        sb.centroids, sa.centroids, sb.volumes, sa.volumes, sb.areas, sa.areas
    );
    text += &format!(
        "NCOLORS     : NColors  = {}  ( {} )\n",
        sb.colors, sa.colors
    );
    text += &format!(
        "COLORS      : Colors   = {}  ( {} )\n",
        lsort_text(&sb.color_names),
        lsort_text(&sa.color_names)
    );
    text += &format!(
        "NLAYERS     : NLayers  = {}  ( {} )\n",
        sb.layers, sa.layers
    );
    text += &format!(
        "LAYERS      : Layers   = {}  ( {} )\n",
        lsort_text(&sb.layer_names),
        lsort_text(&sa.layer_names)
    );
    text
}

/// The script's comparison of one report line: `DATA`, `TPSTAT` and `CHECKSHAPE` fail only when the
/// current value exceeds the reference; `TOLERANCE` only above it by more than 0.0001 and more than 1%;
/// every other line compares exactly.
fn line_agrees(reference: &str, current: &str) -> bool {
    if reference.contains("DATA        :") || reference.contains("TPSTAT      :") {
        return integers_not_above(reference, current, 2);
    }
    if reference.contains("CHECKSHAPE  :") {
        return integers_not_above(reference, current, 8);
    }
    if reference.contains("TOLERANCE   :") {
        return tolerances_not_above(reference, current);
    }
    reference == current
}

fn integers_not_above(reference: &str, current: &str, count: usize) -> bool {
    let parse = |word: &String| word.parse::<i64>().expect("an integer word");
    let (old, new) = (
        numbers_of_line(reference, count, false),
        numbers_of_line(current, count, false),
    );
    old.iter()
        .zip(&new)
        .all(|(old, new)| parse(new) <= parse(old))
}

fn tolerances_not_above(reference: &str, current: &str) -> bool {
    let parse = |word: &String| word.parse::<f64>().expect("a real word");
    let (old, new) = (
        numbers_of_line(reference, 4, true),
        numbers_of_line(current, 4, true),
    );
    old.iter().zip(&new).all(|(old, new)| {
        let (old, new) = (parse(old), parse(new));
        !(new - old > 0.0001 && new - old > 0.01 * old)
    })
}

#[test]
fn the_comparison_keeps_the_scripts_tolerances() {
    let line = |new: &str, old: &str| {
        format!("TOLERANCE   : MaxTol   = {new}  ( {old} )  AvgTol   =  {new}  (  {old} )")
    };
    let reference = line("0.001", "0.001");
    assert!(
        tolerances_not_above(&reference, &line("0.00109", "0.00109")),
        "below 0.0001 above"
    );
    assert!(
        !tolerances_not_above(&reference, &line("0.00111", "0.001")),
        "above both"
    );
    assert!(
        tolerances_not_above(&line("10", "10"), &line("10.05", "10.05")),
        "above 0.0001, below 1%"
    );
    assert!(tolerances_not_above(
        &line("1e-007", "1e-007"),
        &line("1e-07", "1e-07")
    ));
    let data = |faulty: i32, warn: i32| {
        format!(
            "DATA        : Faulties = {faulty}  ( {faulty} )  Warnings = {warn}  ( {warn} )  Summary  = 9  ( 9 )"
        )
    };
    assert!(integers_not_above(&data(1, 1), &data(0, 1), 2));
    // The script's loop reads the first two integers of the line: the Faulties, new and old.
    assert!(
        integers_not_above(&data(1, 1), &data(1, 2), 2),
        "warnings are not compared"
    );
    assert!(
        !integers_not_above(&data(1, 1), &data(2, 1), 2),
        "more faulties"
    );
    assert!(line_agrees(
        "NCOLORS     : NColors  = 1  ( 1 )",
        "NCOLORS     : NColors  = 1  ( 1 )"
    ));
    assert!(!line_agrees(
        "NCOLORS     : NColors  = 1  ( 1 )",
        "NCOLORS     : NColors  = 0  ( 1 )"
    ));
}
