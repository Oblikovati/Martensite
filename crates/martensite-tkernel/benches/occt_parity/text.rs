//! String cases; twins in `occt/text.cpp`.
use super::{Case, Sample};
use martensite_tkernel::tcollection::AsciiString;
use std::hint::black_box;
use std::time::Instant;

pub const CASES: &[Case] = &[Case {
    name: "ascii_cat_search",
    run: ascii_cat_search,
}];

/// `n / 10` appends of two characters, then one search for a suffix.
fn ascii_cat_search(n: usize) -> Sample {
    let appends = n / 10;
    let start = Instant::now();
    let mut text = AsciiString::new("");
    for _ in 0..appends {
        text.assign_cat(black_box("ab"));
    }
    text.assign_cat("needle");
    let found = text.search("needle");
    Sample::since(start, appends, i64::from(found))
}
