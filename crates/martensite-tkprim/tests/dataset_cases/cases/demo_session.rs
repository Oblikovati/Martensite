//! tests/demo/draw case bodies (their inputs are public dataset members).
use crate::support::{
    analysis::trinfo,
    bounding::{Bounding, bounding},
    checks::nbshapes,
    checkshape::checkshape,
    curves::{imagedir, shape_set},
    log,
    logged_checks::{checknbshapes, checkreal},
    misc::{binrestore, binsave},
    props_view::{Expected, checkprops},
    restore::restore_as,
    session::Session,
};
use martensite_tkbrep::topods::Shape;

/// The `checktrinfo` pattern: faces, (empty faces), triangles, nodes, maximal deflection.
fn trinfo_values(text: &str) -> [f64; 5] {
    let words: Vec<&str> = text.split_whitespace().collect();
    let before = |word: &str| -> f64 {
        let at = words
            .iter()
            .position(|w| w.trim_end_matches('.') == word)
            .unwrap_or_else(|| panic!("Error: command trinfo prints empty info ({word})"));
        words[at - 1].parse().unwrap()
    };
    let empty = words
        .windows(2)
        .position(|pair| pair[0] == "empty" && pair[1].starts_with("faces"))
        .map_or(0., |at| words[at - 1].parse().unwrap());
    let deflection = words
        .windows(3)
        .find(|w| w[0] == "Maximal" && w[1] == "deflection")
        .map(|w| w[2].parse().unwrap())
        .expect("Maximal deflection");
    [
        before("faces"),
        empty,
        before("triangles"),
        before("nodes"),
        deflection,
    ]
}

/// `checktrinfo shape -ref [trinfo other]`: faces, empty faces, triangles, nodes and the
/// maximal deflection equal to the reference (`checkreal`, tolerance 0).
fn checktrinfo_ref(shape: &Shape, reference: &str) {
    log::puts("checktrinfo bb -ref");
    let current = trinfo(shape);
    log::result(&current);
    let [faces, empty, triangles, nodes, deflection] = trinfo_values(&current);
    let [rf, re, rt, rn, rd] = trinfo_values(reference);
    checkreal("Number of faces", faces, rf, 0., 0.);
    checkreal("Number of empty faces", empty, re, 0., 0.);
    checkreal("Number of triangles", triangles, rt, 0., 0.);
    checkreal("Number of nodes", nodes, rn, 0., 0.);
    checkreal("Maximal deflection", deflection, rd, 0., 0.);
}

/// The common body of binpersist_2 and binpersist_3: `binsave`, `binrestore` and the
/// checks that the restored shape is the saved one.
fn binpersist(
    s: &mut dyn Session,
    case: &str,
    input: &str,
    expected: [f64; 3],
    with_triangulation: bool,
) {
    let file = imagedir().join(format!("{case}.bin"));
    let b = restore_as(s, input, "b").shape();
    let saved = binsave("b", &b, &file);
    if saved.contains(&format!("Cannot write to the file {}", file.display())) {
        log::puts("Error: binsave");
        return;
    }
    let (restored, bb) = binrestore("bb", &file);
    if restored.contains(&format!("Cannot read from the file {}", file.display())) {
        log::puts("Error: binrestore");
        return;
    }
    let bb = shape_set(s, "bb", bb);
    std::fs::remove_file(&file).unwrap();
    let first = bounding(
        s,
        Bounding {
            print: true,
            ..Bounding::of(&b)
        },
    )
    .text;
    let second = bounding(
        s,
        Bounding {
            print: true,
            ..Bounding::of(&bb)
        },
    )
    .text;
    if first != second {
        log::puts("Error: restored shape has another bounding box");
    }
    checkshape(&bb);
    let reference = nbshapes(&b);
    log::result(&reference);
    checknbshapes(&bb, [None; 9], Some(&reference));
    let [length, area, volume] = expected;
    checkprops(s, "bb", &bb, Expected::l(length));
    checkprops(s, "bb", &bb, Expected::s(area));
    checkprops(s, "bb", &bb, Expected::v(volume));
    if with_triangulation {
        let reference = trinfo(&b);
        log::result(&reference);
        checktrinfo_ref(&bb, &reference);
    }
}

/// `binsave`/`binrestore` of the restored segment keep its box, sub-shapes and properties.
pub fn binpersist_2(s: &mut dyn Session) {
    binpersist(
        s,
        "binpersist_2",
        "bug23849_segment_2.brep",
        [7703.49, 10678.2, 67245.],
        false,
    );
    log::puts("TEST COMPLETED");
}

/// The same for OCC615.brep, with its triangulation.
pub fn binpersist_3(s: &mut dyn Session) {
    binpersist(
        s,
        "binpersist_3",
        "OCC615.brep",
        [5501.4, 201237., 4.68509e+006],
        true,
    );
    log::puts("TEST COMPLETED");
}
