//! The orientation a selection keeps. Only the checks before storage run here: the same scripts
//! then store and reopen the document in a native format, which is out of scope and is not run.
use crate::session::commands::CafSession;
use crate::session::hooks::ImageCheck::None as NoPicture;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The check each selection's orientation gets, with the script's two answers.
fn orientation_check(s: &mut CafSession, entry: &str, name: &str, expected: &str) {
    s.get_shape(entry, name);
    let what = s.whatis(name);
    if what.contains(expected) {
        println!("OK: orientation of {name} is correct");
    } else {
        println!("Error: orientation of {name} is incorrect");
    }
}

/// Steps 1 to 3: compute the box, attach two edges keeping orientation, check them.
fn checks_before_storage(s: &mut CafSession, format: &str) {
    println!("==========\nOCC23766\n==========\n");
    s.erase();
    s.new_document(Some(format));
    s.new_command();
    let object = s.add_object();
    let function = s.add_box_function(&object);
    s.box_size("BoxDX", &object, 1, 190);
    s.box_size("BoxDY", &object, 2, 290);
    s.box_size("BoxDZ", &object, 3, 390);
    s.init_log_book();
    s.add_box_driver();
    s.compute_fun(&function);
    s.get_shape(&format!("{function}:2"), "Box2");
    s.explode_listed("Box2", ShapeEnum::Face, "F");
    s.explode_listed("Box2_1", ShapeEnum::Edge, "E");
    s.explode_listed("Box2_3", ShapeEnum::Edge, "E");
    s.show_only("Box2_1_1", "Box2_3_4");
    s.attach_keeping_orientation("Box2_1_1", &object);
    s.attach_keeping_orientation("Box2_3_4", &object);
    orientation_check(s, "0:1:2:1:2", "E1", "FORWARD");
    orientation_check(s, "0:1:3:1:2", "E2", "REVERSED");
}

caf_case!(
    bug23766_4,
    "tests/bugs/caf/bug23766_4",
    BUGS,
    NoPicture,
    |s| {
        checks_before_storage(s, "BinOcaf");
    }
);

caf_case!(
    bug23766_5,
    "tests/bugs/caf/bug23766_5",
    BUGS,
    NoPicture,
    |s| {
        checks_before_storage(s, "BinOcaf");
    }
);

caf_case!(
    bug23766_6,
    "tests/bugs/caf/bug23766_6",
    BUGS,
    NoPicture,
    |s| {
        println!("TODO OCC24156 MacOS: Error: orientation of E2 is incorrect");
        checks_before_storage(s, "XmlOcaf");
    }
);
