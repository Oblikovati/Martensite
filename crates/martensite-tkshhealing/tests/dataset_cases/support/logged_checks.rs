//! The DrawResources/CheckCommands.tcl checks as the scripts run them: each prints its
//! `Error` or `OK` line to the case's log ([`super::log`]), which gets the verdict.
use super::analysis::trinfo;
use super::checks::{NBSHAPES_ENTITIES, nbshapes};
use super::log;
use super::text::numbers_after;
use martensite_tkbrep::topods::Shape;

/// `checkreal name value expected tol_abs tol_rel`.
pub fn checkreal(name: &str, value: f64, expected: f64, tol_abs: f64, tol_rel: f64) {
    if (value - expected).abs() > tol_abs + tol_rel * expected.abs() {
        log::puts(&format!(
            "Error: {name} = {value} is not equal to expected {expected}"
        ));
    } else {
        log::puts(&format!(
            "Check of {name} OK: value = {value}, expected = {expected}"
        ));
    }
}

/// The count of `entity` in an `nbshapes` text (`ENTITY +: +(n)`).
pub fn nbshapes_count(nb_info: &str, entity: &str) -> Option<usize> {
    numbers_after(nb_info, &format!("{entity} "), 1, "")
        .or_else(|| {
            let at = nb_info.find(&format!("{entity} "))?;
            let rest = nb_info[at + entity.len()..].trim_start_matches(' ');
            let rest = rest.strip_prefix(':')?;
            numbers_after(rest, "", 1, "")
        })
        .map(|values| values[0] as usize)
}

/// `checknbshapes shape [-vertex n] ... [-shape n] [-ref text]`: each entity given an
/// expected count, or found in the `-ref` text, must have it (the proc's `message` is
/// empty, hence its two blanks in `Error :  is WRONG`).
pub fn checknbshapes(shape: &Shape, expected: [Option<usize>; 9], reference: Option<&str>) {
    log::puts("checknbshapes");
    let nb_info = nbshapes(shape);
    log::result(&nb_info);
    for (index, entity) in NBSHAPES_ENTITIES.iter().enumerate() {
        let found = nbshapes_count(&nb_info, entity)
            .unwrap_or_else(|| panic!("Error : command \"nbshapes\" gives an empty result"));
        let mut compare = vec![found];
        compare.extend(expected[index]);
        compare.extend(reference.and_then(|text| nbshapes_count(text, entity)));
        if compare.len() == 1 {
            continue;
        }
        compare.sort_unstable();
        compare.dedup();
        if compare.len() != 1 {
            log::puts(&format!(
                "Error :  is WRONG because number of {entity} entities in shape \"shape\" is {found}"
            ));
        } else {
            log::puts(&format!(
                "OK :  is GOOD because number of {entity} entities is equal to number of expected {entity} entities"
            ));
        }
    }
}

/// `checknbshapes shape -vertex v -edge e ...` with only some entities given (`None`
/// for an option the script does not pass), in VERTEX..SHAPE order.
pub fn checknbshapes_some(shape: &Shape, expected: [Option<usize>; 9]) {
    checknbshapes(shape, expected, None);
}

/// The face, triangle and node counts of a `trinfo` text.
pub fn trinfo_counts(text: &str) -> (usize, usize, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let before = |word: &str| -> usize {
        let at = words
            .iter()
            .position(|w| w.trim_end_matches('.') == word)
            .unwrap_or_else(|| panic!("trinfo prints no {word}"));
        words[at - 1].parse().unwrap()
    };
    (before("faces"), before("triangles"), before("nodes"))
}

/// `trinfo shape`, logged.
pub fn trinfo_logged(shape: &Shape) -> String {
    let text = trinfo(shape);
    log::result(&text);
    text
}

/// `checktrinfo shape -face f -nod n -tri t`: each count equal (checkreal, tolerance 0).
pub fn checktrinfo(shape: &Shape, faces: usize, nodes: usize, triangles: usize) {
    log::puts("checktrinfo");
    let (cur_faces, cur_triangles, cur_nodes) = trinfo_counts(&trinfo_logged(shape));
    checkreal("Number of faces", cur_faces as f64, faces as f64, 0.0, 0.0);
    checkreal(
        "Number of triangles",
        cur_triangles as f64,
        triangles as f64,
        0.0,
        0.0,
    );
    checkreal("Number of nodes", cur_nodes as f64, nodes as f64, 0.0, 0.0);
}
