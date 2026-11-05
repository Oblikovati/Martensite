//! Document bug scripts that keep to one document and its viewer.
use crate::session::commands::CafSession;
use crate::session::hooks::ImageCheck::{Drawn, None as NoPicture};

fn qa_commands() {
    println!("pload QAcommands ");
}

fn three_marks(mark: &str) {
    println!("===========\n{mark}\n===========");
}

caf_case!(buc60917, "tests/bugs/caf/buc60917", BUGS, NoPicture, |s| {
    three_marks("BUC60917");
    qa_commands();
    s.new_document(Some("BinOcaf"));
    s.undo_limit(100);
    s.box_of_size("b", 10, 10, 10);
    s.set_shape("0:1:1", "b");
    s.ais_init_viewer();
    s.open_command();
    s.ais_set("0:1:1", "NS");
    s.ais_set_width("0:1:1", "1.0", 1.);
    if s.ais_width("0:1:1") != 1. {
        println!("Error : 1");
        return;
    }
    s.ais_display("0:1:1");
    s.vfit();
    s.commit_command();
    println!("Doing Undo... ");
    s.undo();
    s.ais_repaint();
    s.ais_repaint();
    println!("And now, look: Redo!");
    s.redo();
    s.ais_repaint();
    if s.ais_width("0:1:1") != 1. {
        println!("Error : 2");
    }
});

caf_case!(
    bug2932_23,
    "tests/bugs/caf/bug2932_23",
    BUGS,
    NoPicture,
    |s| {
        println!("================\nOCC2932\n================\n");
        s.new_document(Some("BinOcaf"));
        s.undo_limit(100);
        s.new_command();
        s.label("0:2");
        s.ais_set("0:2", "NS");
        s.set_real("0:2", "300.", 300.);
        s.dump_command();
        s.new_command();
        s.dump_command();
        s.ais_set("0:2", "NS");
        s.set_real("0:2", "400.", 400.);
        s.dump_command();
        s.new_command();
        let list = s.dump_command();
        const GOOD_LENGTH: usize = 17;
        println!("length = {}", list.len());
        println!("good_length = {GOOD_LENGTH}");
        if list.len() != GOOD_LENGTH {
            println!("OCC2932 Error (case 1)");
            return;
        }
        let name = &list[GOOD_LENGTH - 1];
        println!("name_attribute = {name}");
        println!("modified_attribute = TDataStd_Real");
        if name == "TDataStd_Real" {
            println!("OCC2932 OK");
        } else {
            println!("OCC2932 Error (case 2)");
        }
    }
);

caf_case!(bug30215, "tests/bugs/caf/bug30215", BUGS, Drawn, |s| {
    println!("=========\nOCC30215\n=========\n");
    s.new_document(None);
    s.ais_init_viewer();
    s.box_of_size("b1", 100, 100, 100);
    s.set_shape("0:1", "b1");
    s.ais_set("0:1", "NS");
    s.ais_display_lazily("0:1");
    s.box_of_size("b2", 100, 200, 300);
    s.set_shape("0:1", "b2");
    s.ais_remove("0:1");
    s.ais_display_lazily("0:1");
});

/// The box with a selection mode changed from what it started with; the two modes it read.
fn selection_mode_changed(s: &mut CafSession) -> [i32; 2] {
    three_marks("OCC73\nBUC61050");
    qa_commands();
    s.new_document(Some("BinOcaf"));
    s.undo_limit(10);
    s.new_command();
    s.box_of_size("aBox1", 100, 200, 300);
    s.set_shape("0:2", "aBox1");
    s.ais_init_viewer();
    s.ais_set("0:2", "NS");
    s.ais_display("0:2");
    let first = s.selection_mode_query("0:2");
    s.selection_mode_set("0:2", 1);
    [first, s.selection_mode_query("0:2")]
}

caf_case!(bug73_1, "tests/bugs/caf/bug73_1", BUGS, NoPicture, |s| {
    let [mode0, mode1] = selection_mode_changed(s);
    assert_eq!(s.document().get_undo_limit(), 10);
    println!("SelectionMode0 = {mode0}\nSelectionMode1 = {mode1}\nSelectionMode  = 1");
    if mode0 == mode1 || mode1 != 1 {
        println!("OCC73: Error");
    } else {
        println!("OCC73: OK");
    }
});

/// The script's closing report and check of the third mode it read.
fn third_mode_report(modes: [i32; 3]) {
    let [mode0, mode1, mode2] = modes;
    println!("SelectionMode0 = {mode0}\nSelectionMode1 = {mode1}\nSelectionMode  = 1");
    println!("SelectionMode2 = {mode2}");
    if mode0 == mode1 || mode1 != 1 || mode1 != mode2 {
        println!("OCC73: Error");
    } else {
        println!("OCC73: OK");
    }
}

caf_case!(bug73_2, "tests/bugs/caf/bug73_2", BUGS, NoPicture, |s| {
    let [mode0, mode1] = selection_mode_changed(s);
    s.new_command();
    s.undo();
    s.redo();
    let mode2 = s.selection_mode_query("0:2");
    third_mode_report([mode0, mode1, mode2]);
});

caf_case!(bug73_3, "tests/bugs/caf/bug73_3", BUGS, NoPicture, |s| {
    let [mode0, mode1] = selection_mode_changed(s);
    s.new_command();
    s.forget_all("0:2");
    s.new_command();
    s.undo();
    s.redo();
    s.undo();
    let mode2 = s.selection_mode_query("0:2");
    third_mode_report([mode0, mode1, mode2]);
});

/// Three shapes at 0:1:1 to 0:1:3 in plastic, the first transparent and blue, all displayed.
fn transparent_scene(s: &mut CafSession) {
    for entry in ["0:1:1", "0:1:2", "0:1:3"] {
        s.ais_set(entry, "NS");
    }
    s.ais_init_viewer();
    // The word reads as the number 0, the first material.
    for entry in ["0:1:1", "0:1:2", "0:1:3"] {
        s.ais_set_material(entry, "plastic", 0);
    }
    s.ais_set_transparency("0:1:1", 0.6);
    s.ais_set_color("0:1:1", "MATRABLUE");
    for entry in ["0:1:1", "0:1:2", "0:1:3"] {
        s.ais_display(entry);
    }
}

fn eclipse_title() {
    println!("===========");
    println!("BUC60918: Shape drawn in transparency mode does not always eclipse another shapes");
    println!("OCC85\n===========\n\npload OCAF MODELING VISUALIZATION ");
}

fn another_box(s: &mut CafSession) {
    s.box_of_size("b", 10, 10, 10);
    s.set_shape("0:1:4", "b");
    s.ais_set("0:1:4", "NS");
    s.ais_display("0:1:4");
}

caf_case!(
    bug85_1,
    "tests/v3d/bugs/bug85_1",
    VIEWER_BUGS,
    NoPicture,
    |s| {
        eclipse_title();
        s.box_at("b2", [20, 20, 20], [5, 5, 5]);
        s.box_of_size("b3", 30, 30, 30);
        s.box_at("b4", [10, 20, 25], [7, 7, 7]);
        s.explode_faces("b3");
        s.new_document(None);
        s.set_shape("0:1:1", "b3_2");
        s.set_shape("0:1:2", "b2");
        s.set_shape("0:1:3", "b4");
        transparent_scene(s);
        s.vfit();
        s.vsetdispmode(1);
        another_box(s);
        s.checkcolor(273, 115, [0.78, 0.55, 0.9]);
        s.checkview_screenshot("bug_cases::bug85_1", Drawn);
    }
);

caf_case!(
    bug85_2,
    "tests/v3d/bugs/bug85_2",
    VIEWER_BUGS,
    NoPicture,
    |s| {
        eclipse_title();
        s.restore("dataset-7.9.0/brep", "buc60918-psm1.brep", "psm1");
        s.restore("dataset-7.9.0/brep", "buc60918-psm2.brep", "psm2");
        s.restore("dataset-7.9.0/brep", "buc60918-shl.brep", "shl");
        s.new_document(None);
        s.set_shape("0:1:1", "shl");
        s.set_shape("0:1:2", "psm1");
        s.set_shape("0:1:3", "psm2");
        transparent_scene(s);
        s.vsetdispmode(1);
        another_box(s);
        s.vfit();
        s.checkcolor(230, 160, [0.76, 0.54, 0.9]);
        s.checkview_screenshot("bug_cases::bug85_2", Drawn);
    }
);
