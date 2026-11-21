//! Case lines, medians and the parity verdict of `cargo xtask occt-parity` (#583).
use std::collections::{BTreeMap, BTreeSet};

use crate::error::{BuildError, BuildResult};

/// One printed case line: `name<TAB>nanoseconds per operation<TAB>checksum`.
#[derive(Debug, Clone, PartialEq)]
pub struct CaseLine {
    /// The case name, shared by both sides.
    pub name: String,
    /// The median time of one operation in that run.
    pub nanos: f64,
    /// The work checksum; both sides must agree.
    pub checksum: i64,
}

/// Parses a cases program's stdout; `parse_lines("OCCT", "a\t1.5\t7\n")?`.
pub fn parse_lines(origin: &str, stdout: &str) -> BuildResult<Vec<CaseLine>> {
    stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(origin, line))
        .collect()
}

fn parse_line(origin: &str, line: &str) -> BuildResult<CaseLine> {
    let unexpected = || BuildError::UnexpectedOutput {
        origin: origin.to_owned(),
        detail: format!("{line:?}; expected `name<TAB>nanoseconds<TAB>checksum`"),
    };
    let fields: Vec<&str> = line.split('\t').collect();
    let [name, nanos, checksum] = fields.as_slice() else {
        return Err(unexpected());
    };
    Ok(CaseLine {
        name: (*name).to_owned(),
        nanos: nanos.parse().map_err(|_| unexpected())?,
        checksum: checksum.parse().map_err(|_| unexpected())?,
    })
}

/// Every round of one side, merged per case.
#[derive(Debug, Default)]
pub struct SideRuns {
    cases: BTreeMap<String, CaseRuns>,
}

#[derive(Debug)]
struct CaseRuns {
    nanos: Vec<f64>,
    checksum: i64,
}

impl SideRuns {
    /// Adds one run of every case; `runs.add_round("Rust", lines)?`.
    pub fn add_round(&mut self, origin: &str, lines: Vec<CaseLine>) -> BuildResult<()> {
        lines
            .into_iter()
            .try_for_each(|line| self.add_line(origin, line))
    }

    fn add_line(&mut self, origin: &str, line: CaseLine) -> BuildResult<()> {
        let runs = self.cases.entry(line.name.clone()).or_insert(CaseRuns {
            nanos: Vec::new(),
            checksum: line.checksum,
        });
        if runs.checksum != line.checksum {
            return Err(BuildError::ParityFailed(format!(
                "{origin} case {} reported checksum {} after {}; expected one checksum every round",
                line.name, line.checksum, runs.checksum
            )));
        }
        runs.nanos.push(line.nanos);
        Ok(())
    }
}

/// How a Rust case compares with its OCCT twin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Standing {
    /// Faster than OCCT by more than the tolerance.
    Faster,
    /// Within the tolerance of OCCT.
    Parity,
    /// Slower than OCCT by more than the tolerance.
    Slower,
}

/// The medians of one case on both sides.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CaseComparison {
    /// The case name.
    pub case: String,
    /// OCCT median, nanoseconds per operation.
    pub occt_ns: f64,
    /// Rust median, nanoseconds per operation.
    pub rust_ns: f64,
    /// `rust_ns / occt_ns`.
    pub ratio: f64,
    /// The checksum both sides reported.
    pub checksum: i64,
    /// The verdict at the requested tolerance.
    pub standing: Standing,
}

/// Compares the medians of both sides; `compare(&occt, &rust, 0.05)?`.
pub fn compare(
    occt: &SideRuns,
    rust: &SideRuns,
    tolerance: f64,
) -> BuildResult<Vec<CaseComparison>> {
    same_cases(occt, rust)?;
    occt.cases
        .iter()
        .zip(rust.cases.values())
        .map(|((name, occt), rust)| compare_case(name, occt, rust, tolerance))
        .collect()
}

fn same_cases(occt: &SideRuns, rust: &SideRuns) -> BuildResult<()> {
    let occt_names: BTreeSet<&String> = occt.cases.keys().collect();
    let rust_names: BTreeSet<&String> = rust.cases.keys().collect();
    if occt_names == rust_names {
        return Ok(());
    }
    Err(BuildError::ParityFailed(format!(
        "cases only in OCCT: {:?}; cases only in Rust: {:?}; expected the same case names",
        occt_names.difference(&rust_names).collect::<Vec<_>>(),
        rust_names.difference(&occt_names).collect::<Vec<_>>()
    )))
}

fn compare_case(
    name: &str,
    occt: &CaseRuns,
    rust: &CaseRuns,
    tolerance: f64,
) -> BuildResult<CaseComparison> {
    if occt.checksum != rust.checksum {
        return Err(BuildError::ParityFailed(format!(
            "case {name}: Rust checksum {} differs from OCCT checksum {}; expected the same work",
            rust.checksum, occt.checksum
        )));
    }
    let (occt_ns, rust_ns) = (median(&occt.nanos), median(&rust.nanos));
    let ratio = rust_ns / occt_ns;
    Ok(CaseComparison {
        case: name.to_owned(),
        occt_ns,
        rust_ns,
        ratio,
        checksum: occt.checksum,
        standing: standing(ratio, tolerance),
    })
}

fn standing(ratio: f64, tolerance: f64) -> Standing {
    if ratio > 1.0 + tolerance {
        Standing::Slower
    } else if ratio < 1.0 - tolerance {
        Standing::Faster
    } else {
        Standing::Parity
    }
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let middle = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    } else {
        sorted[middle]
    }
}

/// Fails when a case is slower than OCCT, naming each one with its ratio.
pub fn verdict(comparisons: &[CaseComparison]) -> BuildResult<()> {
    let slower: Vec<String> = comparisons
        .iter()
        .filter(|item| item.standing == Standing::Slower)
        .map(|item| format!("{} ({:.2}x)", item.case, item.ratio))
        .collect();
    if slower.is_empty() {
        return Ok(());
    }
    Err(BuildError::ParityFailed(format!(
        "{} of {} cases are slower than OCCT: {}",
        slower.len(),
        comparisons.len(),
        slower.join(", ")
    )))
}

/// A Markdown table of the comparisons, one row per case.
pub fn render_table(comparisons: &[CaseComparison]) -> String {
    let mut table = String::from(
        "| Case | OCCT ns/op | Rust ns/op | Ratio | Standing |\n| --- | ---: | ---: | ---: | --- |\n",
    );
    for item in comparisons {
        table.push_str(&format!(
            "| {} | {:.3} | {:.3} | {:.2} | {:?} |\n",
            item.case, item.occt_ns, item.rust_ns, item.ratio, item.standing
        ));
    }
    table
}

#[cfg(test)]
#[path = "compare_tests.rs"]
mod tests;
