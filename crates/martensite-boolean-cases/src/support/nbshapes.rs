//! `checknbshapes name -vertex n ... [-t]`: dumps `nbshapes` and compares each given count.
use super::{script_error::script_error, session::Session};
use martensite_tkbrep::{brep_tools::BRepToolsShapeSet, topods::Shape, toptools::ShapeSet};
use martensite_tkg3d::top_abs::ShapeEnum;

#[path = "../boolean_shape_counts.rs"]
pub mod typed_counts;

/// How the entities are counted: `-t` maps each type alone, without it the shape set counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Counting {
    /// `-t`.
    Typed,
    /// No `-t`: one entry per topological shape, whatever its location.
    Located,
}

/// One expected count of `checknbshapes`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NbCount {
    Vertices(usize),
    Edges(usize),
    Wires(usize),
    Faces(usize),
    Shells(usize),
    Solids(usize),
    CompSolids(usize),
    Compounds(usize),
    Shapes(usize),
}

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

impl NbCount {
    /// Its position in the dump and its value; `Edges(3).row() == (1, 3)`.
    fn row(self) -> (usize, usize) {
        match self {
            NbCount::Vertices(n) => (0, n),
            NbCount::Edges(n) => (1, n),
            NbCount::Wires(n) => (2, n),
            NbCount::Faces(n) => (3, n),
            NbCount::Shells(n) => (4, n),
            NbCount::Solids(n) => (5, n),
            NbCount::CompSolids(n) => (6, n),
            NbCount::Compounds(n) => (7, n),
            NbCount::Shapes(n) => (8, n),
        }
    }
}

const KINDS: [ShapeEnum; 8] = [
    ShapeEnum::Vertex,
    ShapeEnum::Edge,
    ShapeEnum::Wire,
    ShapeEnum::Face,
    ShapeEnum::Shell,
    ShapeEnum::Solid,
    ShapeEnum::CompSolid,
    ShapeEnum::Compound,
];

fn located_counts(shape: &Shape) -> [usize; 9] {
    let mut entries = BRepToolsShapeSet::default();
    entries.add(shape).unwrap();
    let mut counts = [0; 9];
    for index in 1..=entries.nb_shapes() {
        let kind = entries.shape(index).unwrap().shape_type().unwrap();
        let row = KINDS.iter().position(|known| *known == kind).unwrap();
        counts[row] += 1;
    }
    counts[8] = usize::try_from(entries.nb_shapes()).unwrap();
    counts
}

fn dump_text(name: &str, counts: &[usize; 9]) -> String {
    let mut text = format!("Number of shapes in {name}\n");
    for (word, count) in NAMES.iter().zip(counts) {
        text += &format!(" {word:<9} : {count}\n");
    }
    text
}

fn dumped_counts(name: &str, shape: &Shape, counting: Counting) -> [usize; 9] {
    let counts = match counting {
        Counting::Typed => typed_counts::counts(shape),
        Counting::Located => located_counts(shape),
    };
    print!("{}", dump_text(name, &counts));
    counts
}

/// The verdict lines, in the order of the dump; the script's `-m` message is empty.
fn verdicts(name: &str, counts: &[usize; 9], expected: &[NbCount]) -> Vec<String> {
    let mut rows: Vec<(usize, usize)> = expected.iter().map(|count| count.row()).collect();
    rows.sort_unstable();
    rows.iter()
        .map(|(row, wanted)| {
            let word = NAMES[*row];
            if counts[*row] == *wanted {
                return format!(
                    "OK :  is GOOD because number of {word} entities is equal to number of expected {word} entities"
                );
            }
            let found = counts[*row];
            format!("Error :  is WRONG because number of {word} entities in shape \"{name}\" is {found}")
        })
        .collect()
}

/// The counts `nbshapes name` would dump, writing nothing; `shape_counts(s, "result", Located)`.
pub fn shape_counts(s: &dyn Session, name: &str, counting: Counting) -> [usize; 9] {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("nbshapes {name}: expected a bound shape"));
    match counting {
        Counting::Typed => typed_counts::counts(shape),
        Counting::Located => located_counts(shape),
    }
}

/// `nbshapes name`: writes the dump and returns its counts; `nbshapes(s, "result", Located)`.
pub fn nbshapes(s: &dyn Session, name: &str, counting: Counting) -> [usize; 9] {
    let shape = s
        .shape(name)
        .unwrap_or_else(|| panic!("nbshapes {name}: expected a bound shape"));
    dumped_counts(name, shape, counting)
}

/// `checknbshapes name -ref [nbshapes other]`: every count against the other shape's.
/// Example: `checknbshapes_like(s, "r1_0", "r_0")`.
pub fn checknbshapes_like(s: &dyn Session, name: &str, other: &str) {
    checknbshapes_like_counting(s, name, other, Counting::Located);
}

/// `checknbshapes name -ref [nbshapes other -t] -t`, or without `-t` for [`Counting::Located`].
/// Example: `checknbshapes_like_counting(s, "ru1", "r", Counting::Typed)`.
pub fn checknbshapes_like_counting(s: &dyn Session, name: &str, other: &str, counting: Counting) {
    let reference = nbshapes(s, other, counting);
    let typed = if counting == Counting::Typed {
        " -t"
    } else {
        ""
    };
    println!(
        "checknbshapes {name} -ref {{{}}}{typed}",
        dump_text(other, &reference)
    );
    let Some(shape) = s.shape(name) else {
        script_error(&format!(
            "Error : command \"nbshapes {name}\" gives an empty result"
        ));
    };
    let counts = dumped_counts(name, shape, counting);
    let expected = reference_rows(&reference);
    for line in verdicts(name, &counts, &expected) {
        println!("{line}");
    }
}

fn reference_rows(counts: &[usize; 9]) -> [NbCount; 9] {
    [
        NbCount::Vertices(counts[0]),
        NbCount::Edges(counts[1]),
        NbCount::Wires(counts[2]),
        NbCount::Faces(counts[3]),
        NbCount::Shells(counts[4]),
        NbCount::Solids(counts[5]),
        NbCount::CompSolids(counts[6]),
        NbCount::Compounds(counts[7]),
        NbCount::Shapes(counts[8]),
    ]
}

/// `checknbshapes name ... -m message`: the same check, naming what it checks in each verdict.
/// Example: `checknbshapes_named(s, "r", Located, &[Solids(1)], "Number of SOLIDs")`.
pub fn checknbshapes_named(
    s: &dyn Session,
    name: &str,
    counting: Counting,
    expected: &[NbCount],
    message: &str,
) {
    println!("checknbshapes {name}");
    let Some(shape) = s.shape(name) else {
        script_error(&format!(
            "Error : command \"nbshapes {name}\" gives an empty result"
        ));
    };
    let counts = dumped_counts(name, shape, counting);
    for line in verdicts(name, &counts, expected) {
        println!("{}", line.replacen(":  is", &format!(": {message} is"), 1));
    }
}

/// `checknbshapes name ...`; `checknbshapes(s, "result", Typed, &[Vertices(4), Edges(3)])`.
pub fn checknbshapes(s: &dyn Session, name: &str, counting: Counting, expected: &[NbCount]) {
    println!("checknbshapes {name}");
    let Some(shape) = s.shape(name) else {
        script_error(&format!(
            "Error : command \"nbshapes {name}\" gives an empty result"
        ));
    };
    let counts = dumped_counts(name, shape, counting);
    for line in verdicts(name, &counts, expected) {
        println!("{line}");
    }
}

#[test]
fn verdicts_follow_the_dump_order_and_name_the_wrong_count() {
    let counts = [4, 3, 0, 0, 0, 0, 0, 1, 8];
    let lines = verdicts("r", &counts, &[NbCount::Edges(2), NbCount::Vertices(4)]);
    assert!(lines[0].starts_with("OK :  is GOOD because number of VERTEX"));
    assert_eq!(
        lines[1],
        "Error :  is WRONG because number of EDGE entities in shape \"r\" is 3"
    );
}
