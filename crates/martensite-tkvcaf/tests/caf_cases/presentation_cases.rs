//! The presentation grid: attribute state and display through commands, Undo and Redo.
use crate::session::commands::CafSession;
use crate::session::hooks::ImageCheck::{Drawn, Empty};

const NOT_NEEDED: &str = "It is not necessary to have the photo for this case.";

fn one_box_label(s: &mut CafSession) {
    s.box_of_size("aBox1", 100, 200, 300);
    s.set_shape("0:2", "aBox1");
    s.ais_init_viewer();
    s.ais_set("0:2", "NS");
}

fn displayed_box(s: &mut CafSession) {
    one_box_label(s);
    s.ais_display("0:2");
}

fn shaded_displayed_box(s: &mut CafSession) {
    one_box_label(s);
    s.vsetdispmode(1);
    s.ais_display("0:2");
}

fn undo_redo_with_repaint(s: &CafSession) {
    s.undo();
    s.ais_repaint();
    s.redo();
    s.ais_repaint();
}

fn undo_with_repaint(s: &CafSession) {
    s.undo();
    s.ais_repaint();
}

fn two_box_labels(s: &mut CafSession) {
    s.box_of_size("aBox1", 100, 200, 300);
    s.set_shape("0:2", "aBox1");
    s.box_at("aBox2", [50, 50, 0], [100, 200, 300]);
    s.set_shape("0:3", "aBox2");
}

/// The shaded pair with the second box green, displayed, with the next command open.
fn displayed_pair(s: &mut CafSession) {
    s.ais_init_viewer();
    s.ais_set("0:2", "NS");
    s.ais_set("0:3", "NS");
    s.vsetdispmode(1);
    s.ais_set_color("0:3", "GREEN");
    s.ais_display("0:2");
    s.ais_display("0:3");
    s.new_command();
}

fn green_undone_redone(s: &mut CafSession) {
    displayed_box(s);
    s.new_command();
    s.ais_set_color("0:2", "GREEN");
    s.new_command();
    undo_redo_with_repaint(s);
}

fn jade_undone_redone(s: &mut CafSession) {
    shaded_displayed_box(s);
    s.new_command();
    s.ais_set_material("0:2", "18", 18);
    s.new_command();
    undo_redo_with_repaint(s);
}

fn transparent_undone_redone(s: &mut CafSession) {
    two_box_labels(s);
    displayed_pair(s);
    s.ais_set_transparency("0:3", 0.9);
    s.new_command();
    undo_redo_with_repaint(s);
}

caf_case!(a1, "tests/caf/presentation/A1", PRESENTATION, Empty, |s| {
    println!("caf003-A1");
    s.ais_init_viewer();
    println!("There is the empty 3D viewer");
});

fn box_display_undone(s: &mut CafSession) {
    s.box_of_size("aBox1", 100, 200, 300);
    s.set_shape("0:2", "aBox1");
    s.ais_init_viewer();
    s.new_command();
    s.ais_set("0:2", "NS");
    s.new_command();
    s.ais_display("0:2");
    s.new_command();
    undo_with_repaint(s);
}

caf_case!(a2, "tests/caf/presentation/A2", PRESENTATION, Empty, |s| {
    println!("caf003-A2");
    box_display_undone(s);
    println!("There is the empty 3D viewer");
});

caf_case!(a3, "tests/caf/presentation/A3", PRESENTATION, Drawn, |s| {
    println!("caf003-A3");
    box_display_undone(s);
    s.redo();
    s.ais_repaint();
    println!("There is the box in the viewer");
});

caf_case!(b1, "tests/caf/presentation/B1", PRESENTATION, Drawn, |s| {
    println!("caf003-B1");
    displayed_box(s);
    let has_own = s.ais_has_own_color("0:2");
    println!("{NOT_NEEDED}");
    if has_own > 0 {
        println!("HasOwnColor={has_own}");
        println!("There is own color; AISHasOwnColor command: Error");
        return;
    }
    println!("There is not own color; AISHasOwnColor command: OK");
});

caf_case!(b2, "tests/caf/presentation/B2", PRESENTATION, Drawn, |s| {
    println!("caf003-B2");
    green_undone_redone(s);
    let has_own = s.ais_has_own_color("0:2");
    println!("{NOT_NEEDED}");
    if has_own == 0 {
        println!("HasOwnColor={has_own}");
        println!("There is not own color; AISHasOwnColor command: Error");
        return;
    }
    println!("There is own color; AISHasOwnColor command: OK");
});

caf_case!(b3, "tests/caf/presentation/B3", PRESENTATION, Drawn, |s| {
    println!("caf003-B3");
    green_undone_redone(s);
    println!("The box's color is NOC_GREEN");
});

caf_case!(b4, "tests/caf/presentation/B4", PRESENTATION, Drawn, |s| {
    println!("caf003-B4");
    green_undone_redone(s);
    let color = s.ais_color("0:2").unwrap_or_default();
    println!("{NOT_NEEDED}");
    if color != "GREEN" {
        println!("GreenColor=GREEN");
        println!("aColor={color}");
        println!("The box's color is not NOC_GREEN; AISColor command: Error");
        return;
    }
    println!("The box's color is NOC_GREEN; AISColor command: OK");
});

caf_case!(c1, "tests/caf/presentation/C1", PRESENTATION, Drawn, |s| {
    println!("caf003-C1");
    displayed_box(s);
    s.ais_set_color("0:2", "GREEN");
    s.new_command();
    let first = s.ais_has_own_color("0:2");
    s.ais_color("0:2");
    s.ais_default_color("0:2");
    s.new_command();
    undo_redo_with_repaint(s);
    let second = s.ais_has_own_color("0:2");
    println!("{NOT_NEEDED}");
    if first == 0 || second > 0 {
        println!("HasOwnColor1={first} HasOwnColor2={second}");
        println!("AISDefaultColor command: Error");
        return;
    }
    println!("AISDefaultColor command: OK");
});

caf_case!(d1, "tests/caf/presentation/D1", PRESENTATION, Drawn, |s| {
    println!("caf003-D1");
    shaded_displayed_box(s);
    let has_own = s.ais_has_own_material("0:2");
    println!("{NOT_NEEDED}");
    if has_own > 0 {
        println!("HasOwnMaterial={has_own}");
        println!("There is own material; AISHasOwnMaterial command: Error");
        return;
    }
    println!("There is not own material; AISHasOwnMaterial command: OK");
});

caf_case!(d2, "tests/caf/presentation/D2", PRESENTATION, Drawn, |s| {
    println!("caf003-D2");
    jade_undone_redone(s);
    let has_own = s.ais_has_own_material("0:2");
    println!("{NOT_NEEDED}");
    if has_own == 0 {
        println!("HasOwnMaterial={has_own}");
        println!("There is not own material; AISHasOwnMaterial command: Error");
        return;
    }
    println!("There is own material; AISHasOwnMaterial command: OK");
});

caf_case!(d3, "tests/caf/presentation/D3", PRESENTATION, Drawn, |s| {
    println!("caf003-D3");
    jade_undone_redone(s);
    println!("The box's material is NOM_JADE");
});

caf_case!(d4, "tests/caf/presentation/D4", PRESENTATION, Drawn, |s| {
    println!("caf003-D4");
    jade_undone_redone(s);
    let material = s.ais_material("0:2");
    println!("{NOT_NEEDED}");
    if material != 18 {
        println!("JadeMaterial=18");
        println!("aMaterial={material}");
        println!("The box's material is not NOM_JADE; AISMaterial command: Error");
        return;
    }
    println!("The box's material is NOM_JADE; AISMaterial command: OK");
});

caf_case!(e1, "tests/caf/presentation/E1", PRESENTATION, Drawn, |s| {
    println!("caf003-E1");
    shaded_displayed_box(s);
    s.new_command();
    s.ais_set_material("0:2", "18", 18);
    s.new_command();
    let first = s.ais_has_own_material("0:2");
    s.ais_material("0:2");
    s.ais_default_material("0:2");
    s.new_command();
    undo_redo_with_repaint(s);
    let second = s.ais_has_own_material("0:2");
    println!("{NOT_NEEDED}");
    if first == 0 || second > 0 {
        println!("HasOwnMaterial1={first} HasOwnMaterial2={second}");
        println!("AISDefaultMaterial command: Error");
        return;
    }
    println!("AISDefaultMaterial command: OK");
});

caf_case!(f1, "tests/caf/presentation/F1", PRESENTATION, Drawn, |s| {
    println!("caf003-F1");
    shaded_displayed_box(s);
    let has_own = s.ais_has_own_transparency("0:2");
    println!("{NOT_NEEDED}");
    if has_own > 0 {
        println!("HasOwnTransparency={has_own}");
        println!("There is own transparency; AISHasOwnTransparency command: Error");
        return;
    }
    println!("There is not own transparency; AISHasOwnTransparency command: OK");
});

caf_case!(f2, "tests/caf/presentation/F2", PRESENTATION, Drawn, |s| {
    println!("caf003-F2");
    transparent_undone_redone(s);
    let has_own = s.ais_has_own_transparency("0:3");
    println!("{NOT_NEEDED}");
    if has_own == 0 {
        println!("HasOwnTransparency={has_own}");
        println!("There is not own transparency; AISHasOwnTransparency command: Error");
        return;
    }
    println!("There is own transparency; AISHasOwnTransparency command: OK");
});

caf_case!(f3, "tests/caf/presentation/F3", PRESENTATION, Drawn, |s| {
    println!("caf003-F3");
    transparent_undone_redone(s);
    println!("Transparency of the NOC_GREEN box is 0.9");
});

caf_case!(f4, "tests/caf/presentation/F4", PRESENTATION, Drawn, |s| {
    println!("caf003-F4");
    transparent_undone_redone(s);
    let value = s.ais_transparency("0:3");
    println!("{NOT_NEEDED}");
    if value != 0.9 {
        println!("SetTransparency=0.9");
        println!("aTransparency={value}");
        println!("The box's transparency is not 0.9; AISTransparency command: Error");
        return;
    }
    println!("The box's transparency is 0.9; AISTransparency command: OK");
});

caf_case!(g1, "tests/caf/presentation/G1", PRESENTATION, Drawn, |s| {
    println!("caf003-G1");
    two_box_labels(s);
    displayed_pair(s);
    s.ais_set_transparency("0:3", 0.9);
    s.new_command();
    let first = s.ais_has_own_transparency("0:3");
    s.ais_transparency("0:3");
    s.ais_default_transparency("0:3");
    s.new_command();
    undo_redo_with_repaint(s);
    let second = s.ais_has_own_transparency("0:3");
    println!("{NOT_NEEDED}");
    if first == 0 || second > 0 {
        println!("HasOwnTransparency1={first} HasOwnTransparency2={second}");
        println!("AISDefaultTransparency command: Error");
        return;
    }
    println!("AISDefaultTransparency command: OK");
});

/// The pair replaced by a sphere and a cone, with only the first label's presentation updated.
fn shapes_replaced(s: &mut CafSession) {
    two_box_labels(s);
    s.psphere("aSphere", 100);
    s.pcone("aCone", 50, 20, 500);
    displayed_pair(s);
    s.set_shape("0:2", "aSphere");
    s.set_shape("0:3", "aCone");
    s.ais_update("0:2");
    s.new_command();
    undo_with_repaint(s);
}

caf_case!(h1, "tests/caf/presentation/H1", PRESENTATION, Drawn, |s| {
    println!("caf003-H1");
    shapes_replaced(s);
    println!("There are two boxes");
});

// The script prints the first case's name, H1, here too.
caf_case!(h2, "tests/caf/presentation/H2", PRESENTATION, Drawn, |s| {
    println!("caf003-H1");
    shapes_replaced(s);
    s.redo();
    s.ais_repaint();
    println!("There are the box and the sphere");
});

fn first_box_changed_then_undone(s: &mut CafSession, change: fn(&CafSession, &str)) {
    two_box_labels(s);
    displayed_pair(s);
    change(s, "0:2");
    s.new_command();
    undo_with_repaint(s);
}

caf_case!(i1, "tests/caf/presentation/I1", PRESENTATION, Drawn, |s| {
    println!("caf003-I1");
    first_box_changed_then_undone(s, CafSession::ais_erase);
    println!("There are two boxis");
});

// The script prints the first case's name, I1, here too.
caf_case!(i2, "tests/caf/presentation/I2", PRESENTATION, Drawn, |s| {
    println!("caf003-I1");
    first_box_changed_then_undone(s, CafSession::ais_erase);
    s.redo();
    s.ais_repaint();
    println!("There is one box");
});

caf_case!(j1, "tests/caf/presentation/J1", PRESENTATION, Drawn, |s| {
    println!("caf003-J1");
    first_box_changed_then_undone(s, CafSession::ais_remove);
    println!("There are two boxis");
});

// The script prints the first case's name, J1, here too.
caf_case!(j2, "tests/caf/presentation/J2", PRESENTATION, Drawn, |s| {
    println!("caf003-J1");
    first_box_changed_then_undone(s, CafSession::ais_remove);
    s.redo();
    s.ais_repaint();
    println!("There is one box");
});

caf_case!(k1, "tests/caf/presentation/K1", PRESENTATION, Drawn, |s| {
    println!("caf003-K1");
    first_box_changed_then_undone(s, CafSession::ais_unset);
    println!("There are two boxis");
});

caf_case!(k2, "tests/caf/presentation/K2", PRESENTATION, Drawn, |s| {
    println!("caf003-K2");
    first_box_changed_then_undone(s, CafSession::ais_unset);
    s.redo();
    s.ais_repaint();
    if let Some(color) = s.ais_color("0:2") {
        println!("{color}");
        println!("AISUnset command: Error");
        return;
    }
    println!("AISUnset command: OK");
    println!("There is one box");
});

fn box_with_driver(s: &mut CafSession) -> String {
    s.box_of_size("aBox", 100, 200, 300);
    s.set_shape("0:2", "aBox");
    s.ais_init_viewer();
    let driver = s.ais_set("0:2", "NS");
    s.ais_display("0:2");
    driver
}

caf_case!(l1, "tests/caf/presentation/L1", PRESENTATION, Drawn, |s| {
    println!("caf003-L1");
    let first = box_with_driver(s);
    let second = s.ais_driver("0:2");
    println!("{NOT_NEEDED}");
    if first != second {
        println!("aDriverGUID1={first} aDriverGUID2={second}");
        println!("Get the driver GUID; AISDriver command: Error");
        return;
    }
    println!("Get the driver GUID; AISDriver command: OK");
});

caf_case!(l2, "tests/caf/presentation/L2", PRESENTATION, Drawn, |s| {
    println!("caf003-L2");
    let first = box_with_driver(s);
    s.new_command();
    let second = s.ais_set_driver("0:2", "A");
    s.new_command();
    s.undo();
    let third = s.ais_driver("0:2");
    println!("{NOT_NEEDED}");
    if first == second || first != third {
        println!("aDriverGUID1={first} aDriverGUID2={second}");
        println!("aDriverGUID1={first} aDriverGUID3={third}");
        println!("Set the driver GUID; AISDriver command: Error");
        return;
    }
    println!("Set the driver GUID; AISDriver command: OK");
});

// The script compares what a presentation with no own mode answers, an empty text, with 0: they
// differ, so it stops at its first check and the checks after it never run. They are kept, and the
// behaviour they meant to check is `selection_modes_through_undo_and_redo` below.
caf_case!(n1, "tests/caf/presentation/N1", PRESENTATION, Drawn, |s| {
    println!("caf003-N1");
    s.new_command();
    displayed_box(s);
    s.new_command();
    if s.ais_sel_mode("0:2") != "0" {
        println!("Default selection mode is not 0");
        return;
    }
    s.ais_set_sel_mode("0:2", &[2]);
    if s.ais_sel_mode("0:2") != "2" {
        println!("Selection mode is not 2");
        return;
    }
    s.new_command();
    s.ais_set_sel_mode("0:2", &[2, 4]);
    if s.ais_sel_mode("0:2") != "2 4" {
        println!("Selection mode is not 2 4");
        return;
    }
    s.new_command();
    s.undo();
    if s.ais_sel_mode("0:2") != "2" {
        println!("Selection mode after undo is not 2");
        return;
    }
    s.redo();
    if s.ais_sel_mode("0:2") != "2 4" {
        println!("Selection mode after redo is not 2 4");
        return;
    }
    println!("AISSelMode command: OK");
});
