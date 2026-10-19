//! Frozen complete ordinary requests and retained full-owner observations.
use super::*;
pub const PREFIX: usize = 64;
pub const SUFFIX: usize = 16;
#[derive(Clone, Debug)]
pub struct CaseSpec {
    pub name: String,
    pub kind: i32,
    pub iso: i32,
    pub order: i32,
    pub cross: i32,
    pub roots: usize,
    pub degree: i32,
    pub limit: usize,
    pub maximum: usize,
    pub domain: [f64; 4],
    pub constant: f64,
    pub precision: f64,
}
pub struct RetainedObservation {
    pub name: String,
    pub status: i32,
    pub made: i32,
    pub calls: usize,
    pub counts: Vec<i32>,
    pub tables: [Vec<f64>; 8],
    pub roots: Vec<f64>,
}
pub(super) struct Tokens<'a>(std::str::SplitWhitespace<'a>);
impl<'a> Tokens<'a> {
    pub(super) fn new(line: &'a str) -> Self {
        Self(line.split_whitespace())
    }
    pub(super) fn text(&mut self) -> &'a str {
        self.0.next().unwrap()
    }
    pub(super) fn integer(&mut self) -> i32 {
        self.text().parse().unwrap()
    }
    pub(super) fn size(&mut self) -> usize {
        self.text().parse().unwrap()
    }
    pub(super) fn real(&mut self) -> f64 {
        f64::from_bits(self.text().parse().unwrap())
    }
}
fn parse_specification(line: &str) -> CaseSpec {
    let mut r = Tokens::new(line);
    CaseSpec {
        name: r.text().into(),
        kind: r.integer(),
        iso: r.integer(),
        order: r.integer(),
        cross: r.integer(),
        roots: r.size(),
        degree: r.integer(),
        limit: r.size(),
        maximum: r.size(),
        domain: std::array::from_fn(|_| r.real()),
        constant: r.real(),
        precision: r.real(),
    }
}
fn real_owner(line: &str, name: &str) -> Vec<f64> {
    let mut r = Tokens::new(line);
    assert_eq!(r.text(), name);
    let values = (0..r.size()).map(|_| r.real()).collect();
    assert!(r.0.next().is_none());
    values
}
fn count_owner(line: &str) -> Vec<i32> {
    let mut r = Tokens::new(line);
    assert_eq!(r.text(), "counts");
    let values = (0..r.size()).map(|_| r.integer()).collect();
    assert!(r.0.next().is_none());
    values
}
fn parse_observation(group: &str) -> RetainedObservation {
    let mut lines = group.lines();
    let mut header = Tokens::new(lines.next().unwrap());
    let (name, status, made, calls) = (
        header.text().into(),
        header.integer(),
        header.integer(),
        header.size(),
    );
    assert_eq!(header.integer(), 1);
    let counts = count_owner(lines.next().unwrap());
    let tables = std::array::from_fn(|index| real_owner(lines.next().unwrap(), TABLE_NAMES[index]));
    let roots = real_owner(lines.next().unwrap(), "roots");
    assert!(lines.next().is_none());
    RetainedObservation {
        name,
        status,
        made,
        calls,
        counts,
        tables,
        roots,
    }
}
pub fn cases() -> Vec<(CaseSpec, RetainedObservation)> {
    let requests = include_str!("../../fixtures/ordinary_curve/requests.txt");
    let baseline = include_str!("../../fixtures/ordinary_curve/baseline.txt");
    let inputs: Vec<CaseSpec> = requests.lines().map(parse_specification).collect();
    let outputs: Vec<RetainedObservation> = baseline
        .split("case ")
        .skip(1)
        .map(parse_observation)
        .collect();
    assert_eq!(inputs.len(), 60);
    assert_eq!(inputs.len(), outputs.len());
    inputs
        .into_iter()
        .zip(outputs)
        .map(|(input, output)| {
            assert_eq!(input.name, output.name);
            (input, output)
        })
        .collect()
}
