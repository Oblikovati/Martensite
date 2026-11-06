//! The Boolean check commands, with the text each writes and the value a script reads back.
use crate::case_bopcheck::{complete_response, configured_checker};
use crate::support::{
    bop_session::BopOptions,
    commands::bound_shape,
    nbshapes::{Counting, nbshapes},
    session::Session,
};
use martensite_tkbo::{
    bop_algo::{BOPAlgoArgumentAnalyzer, CheckStatus, Operation},
    brep_algo_api::BRepAlgoAPICheck,
};
use martensite_tkbrep::topods::Shape;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPICopy, BRepBuilderAPIMakeShape};

/// `bopargcheck name /modes #F`: the full report: each self-interference bound as `s1si_N`, then the
/// table of faults, which `modes_off` marks `DISABLED`. A fault of another kind is not this script's.
/// Example: `bopargcheck_full(s, "b", "RFPCS").contains("Self-Intersections")`.
pub fn bopargcheck_full(s: &mut dyn Session, name: &str, modes_off: &str) -> String {
    let mut checker = BOPAlgoArgumentAnalyzer::new();
    checker.options_mut().set_run_parallel(false);
    checker
        .options_mut()
        .set_fuzzy_value(martensite_tkernel::precision::confusion());
    checker.set_shape1(&bound_shape(s, name));
    apply_modes(&mut checker, modes_off, false);
    checker.perform();
    let (mut cases, mut shapes) = (0, 0);
    for result in checker.check_result().iter() {
        assert_eq!(
            result.check_status(),
            CheckStatus::SelfIntersect,
            "bopargcheck {name} #F"
        );
        cases += 1;
        let faulty = result.faulty_shapes1();
        shapes += faulty.iter().count();
        let parts: Vec<Shape> = faulty.iter().cloned().collect();
        s.bind(
            &format!("s1si_{cases}"),
            &super::repair::compound_of(&parts),
        );
        println!("Made faulty shape: s1si_{cases}");
    }
    let state = |disabled: &str| {
        if modes_off.contains(disabled) {
            "DISABLED"
        } else {
            "NO"
        }
    };
    let report = format!(
        "Faulties for FIRST  shape found : {cases}\n---------------------------------\n\
         Shapes are not suppotrted by BOP: {}\n\
         Self-Intersections              : YES  Cases({cases})  Total shapes({shapes})\n\
         Check for SI has been aborted   : NO\n\
         Too small edges                 : {}\n\
         Bad faces                       : {}\n\
         Too close vertices              : DISABLED\n\
         Too close edges                 : DISABLED\n\
         Shapes with Continuity C0       : {}\n\
         Invalid Curve on Surface        : {}\n\n\
         Faulties for SECOND  shape found : 0\n",
        state("P"),
        state("R"),
        state("F"),
        state("C"),
        state("S"),
    );
    print!("{report}");
    report
}

/// `bopcheck name` under the session's options: writes the response and returns it.
/// Example: `bopcheck_text(s, "r0", &bop.options).contains("OK")`.
pub fn bopcheck_text(s: &dyn Session, name: &str, options: &BopOptions) -> String {
    let shape = bound_shape(s, name);
    let mut checker = configured_checker(&shape);
    checker.options_mut().set_run_parallel(options.run_parallel);
    checker.options_mut().set_fuzzy_value(options.fuzzy);
    checker.perform();
    let response = complete_response(&checker);
    println!("{response}");
    response
}

/// `bopcheck name`, binding the shapes of each interference as `xN`, as the command does.
/// Example: `bopcheck_binding(s, "b1")`.
pub fn bopcheck_binding(s: &mut dyn Session, name: &str) -> String {
    let shape = bound_shape(s, name);
    let mut checker = configured_checker(&shape);
    checker.perform();
    let response = complete_response(&checker);
    println!("{response}");
    let structure = checker
        .data_structure()
        .expect("bopcheck: expected the performed structure");
    for pair in structure.interferences() {
        let (first, second) = pair.indices();
        if structure.is_new_shape(first) || structure.is_new_shape(second) {
            continue;
        }
        for index in [first, second] {
            s.bind(&format!("x{index}"), structure.shape(index).unwrap());
        }
    }
    response
}

/// `if {![regexp "OK" [bopcheck name]]} {puts message}`.
/// Example: `bopcheck_ok(s, "r0", &bop.options, "Error: the r0 is a self-interfering shape")`.
pub fn bopcheck_ok(s: &dyn Session, name: &str, options: &BopOptions, message: &str) {
    if !bopcheck_text(s, name, options).contains("OK") {
        println!("{message}");
    }
}

/// `bopcheck name level`: the intersections up to `level` only.
/// Example: `bopcheck_level(s, "r10", &options, 4)`.
pub fn bopcheck_level(s: &dyn Session, name: &str, options: &BopOptions, level: i32) -> String {
    let shape = bound_shape(s, name);
    println!(
        "Info:\nThe level of check is set to E/F, i.e. intersection(s)\nF/F, V/Z, E/Z, F/Z, Z/Z will not be checked.\n"
    );
    let mut checker = configured_checker(&shape);
    checker.set_level_of_check(level);
    checker.options_mut().set_run_parallel(options.run_parallel);
    checker.options_mut().set_fuzzy_value(options.fuzzy);
    checker.perform();
    let response = complete_response(&checker);
    println!("{response}");
    response
}

/// `bopargcheck first [second] [-F|-O|-C|-T|-S] [/modes]`: writes and returns the verdict line.
/// `modes_off` is the script's `/` or `#` letters. Example: `bopargcheck(s, ("a", None), None, "")`.
pub fn bopargcheck(
    s: &dyn Session,
    (first, second): (&str, Option<&str>),
    operation: Option<Operation>,
    modes_off: &str,
) -> String {
    let mut checker = BOPAlgoArgumentAnalyzer::new();
    checker.options_mut().set_run_parallel(false);
    checker
        .options_mut()
        .set_fuzzy_value(martensite_tkernel::precision::confusion());
    checker.set_shape1(&bound_shape(s, first));
    if let Some(second) = second {
        let mut copy =
            BRepBuilderAPICopy::with_shape(&bound_shape(s, second), true, false).unwrap();
        checker.set_shape2(copy.shape().unwrap());
        *checker.operation_type() = operation.unwrap_or(Operation::Section);
    }
    apply_modes(&mut checker, modes_off, second.is_some());
    checker.perform();
    let line = if checker.has_faulty() {
        "Faulties, that can not be treated by BOP, are detected.\n"
    } else {
        "Shape(s) seem(s) to be valid for BOP.\n"
    };
    print!("{line}");
    line.to_owned()
}

/// The default modes, then the script's `/` letters turning some off; a second shape adds three.
fn apply_modes(checker: &mut BOPAlgoArgumentAnalyzer, off: &str, two: bool) {
    *checker.argument_type_mode() = true;
    *checker.self_inter_mode() = true;
    *checker.small_edge_mode() = true;
    *checker.rebuild_face_mode() = true;
    *checker.continuity_mode() = true;
    *checker.curve_on_surface_mode() = true;
    if two {
        *checker.tangent_mode() = true;
        *checker.merge_vertex_mode() = true;
        *checker.merge_edge_mode() = true;
    }
    for letter in off.chars() {
        match letter {
            'R' => *checker.small_edge_mode() = false,
            'F' => *checker.rebuild_face_mode() = false,
            'T' => *checker.tangent_mode() = false,
            'V' => *checker.merge_vertex_mode() = false,
            'E' => *checker.merge_edge_mode() = false,
            'I' => *checker.self_inter_mode() = false,
            'P' => *checker.argument_type_mode() = false,
            'C' => *checker.continuity_mode() = false,
            'S' => *checker.curve_on_surface_mode() = false,
            other => panic!("bopargcheck: expected a mode letter, got {other:?}"),
        }
    }
}

/// `checknbshapes name ... -m message`: the verdicts carry the message.
/// Example: `checknbshapes_message(s, "result", Counting::Typed, &[(0, 16)], "FUSE")`.
pub fn checknbshapes_message(
    s: &dyn Session,
    name: &str,
    counting: Counting,
    expected: &[usize; 9],
    message: &str,
) {
    println!("checknbshapes {name}");
    let counts = nbshapes(s, name, counting);
    const NAMES: [&str; 9] = [
        "VERTEX",
        "EDGE",
        "WIRE",
        "FACE",
        "SHELL",
        "SOLID",
        "COMPSOLID",
        "COMPOUND",
        "SHAPE",
    ];
    for (row, word) in NAMES.iter().enumerate() {
        if counts[row] == expected[row] {
            println!(
                "OK : {message} is GOOD because number of {word} entities is equal to number of expected {word} entities"
            );
        } else {
            println!(
                "Error : {message} is WRONG because number of {word} entities in shape \"{name}\" is {}",
                counts[row]
            );
        }
    }
}

/// `checknbshapes name -kind n ... -m message`: `rows` pairs an index of the dump with its count.
/// Example: `checknbshapes_selected(s, "r", Counting::Typed, &[(5, 1)], "NbShapes in result")`.
pub fn checknbshapes_selected(
    s: &dyn Session,
    name: &str,
    counting: Counting,
    rows: &[(usize, usize)],
    message: &str,
) {
    const NAMES: [&str; 9] = [
        "VERTEX",
        "EDGE",
        "WIRE",
        "FACE",
        "SHELL",
        "SOLID",
        "COMPSOLID",
        "COMPOUND",
        "SHAPE",
    ];
    println!("checknbshapes {name}");
    let counts = nbshapes(s, name, counting);
    let mut sorted = rows.to_vec();
    sorted.sort_unstable();
    for (row, wanted) in sorted {
        let word = NAMES[row];
        if counts[row] == wanted {
            println!(
                "OK : {message} is GOOD because number of {word} entities is equal to number of expected {word} entities"
            );
        } else {
            println!(
                "Error : {message} is WRONG because number of {word} entities in shape \"{name}\" is {}",
                counts[row]
            );
        }
    }
}

/// `bopapicheck first [second] [-op kind] [-se] [-si]`: writes and returns the answer.
/// Example: `bopapicheck(s, ("c", None), None, [true, false])`.
pub fn bopapicheck(
    s: &dyn Session,
    (first, second): (&str, Option<&str>),
    operation: Option<Operation>,
    [small_edges, self_inter]: [bool; 2],
) -> String {
    let shape = bound_shape(s, first);
    let other = second.map_or_else(Shape::default, |name| bound_shape(s, name));
    let checker = BRepAlgoAPICheck::from_shapes(
        &shape,
        &other,
        operation.unwrap_or(Operation::Unknown),
        small_edges,
        self_inter,
    )
    .unwrap_or_else(|failure| panic!("bopapicheck {first}: {failure:?}"));
    let text = api_check_text(&checker, other.is_null());
    print!("{text}");
    text
}

fn api_check_text(checker: &BRepAlgoAPICheck, one: bool) -> String {
    use martensite_tkbo::bop_algo::CheckStatus;
    if checker.is_valid() {
        return if one {
            "The shape seems to be valid\n"
        } else {
            "The shapes seem to be valid\n"
        }
        .to_owned();
    }
    let (mut first, mut second, mut bad_type) = (false, false, false);
    for faulty in checker.result().iter() {
        if faulty.check_status() == CheckStatus::BadType {
            bad_type = true;
        } else {
            first |= !faulty.shape1().is_null();
            second |= !faulty.shape2().is_null();
        }
    }
    let mut text = String::new();
    if first {
        text += if one {
            "The shape is invalid\n"
        } else {
            "The first shape is invalid\n"
        };
    }
    if second {
        text += "The second shape is invalid\n";
    }
    if bad_type {
        text += if one {
            "The shape is empty\n"
        } else {
            "The shapes are not valid for Boolean operation\n"
        };
    }
    text
}
