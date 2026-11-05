//! Fixed authored graph metadata and independently captured public results.
#[derive(Clone, Copy)]
pub(super) struct FaceWitness {
    pub uv: [f64; 2],
    pub bounds: [f64; 4],
    pub range: [f64; 2],
    pub kind: i32,
}

pub(super) struct FixtureSpec {
    pub name: &'static str,
    pub brep: &'static [u8],
    pub expected: &'static str,
    pub first: FaceWitness,
    pub second: FaceWitness,
    pub occurrences: [usize; 2],
    pub same_face: bool,
}

pub(super) struct CaseRow {
    pub flags: [bool; 5],
    pub correct: bool,
    pub tolerance: f64,
    pub result: i32,
}

fn flag(text: &str) -> bool {
    match text {
        "0" => false,
        "1" => true,
        other => panic!("flag={other:?}: expected literal0or1"),
    }
}

impl CaseRow {
    /// Parse the fixed independent native row; `CaseRow::parse("0 0 0 0 0 0 1e-5 2")`.
    pub(super) fn parse(line: &str) -> Self {
        let values: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(values.len(), 8, "row={line:?}: expected8fields");
        let tolerance: f64 = values[6].parse().unwrap();
        assert!(tolerance.is_finite(), "row={line:?}: expected finite limit");
        Self {
            flags: std::array::from_fn(|i| flag(values[i])),
            correct: flag(values[5]),
            tolerance,
            result: values[7].parse().unwrap(),
        }
    }
}
