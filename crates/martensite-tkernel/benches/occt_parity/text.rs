//! String cases; twins in `occt/text.cpp`.
use super::{Case, Sample};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString, HAsciiString, HExtendedString};
use std::hint::black_box;
use std::time::Instant;

pub const CASES: &[Case] = &[
    Case {
        name: "ascii_cat_search",
        run: ascii_cat_search,
    },
    Case {
        name: "ascii_borrowed_cat_search",
        run: ascii_borrowed_cat_search,
    },
    Case {
        name: "ascii_binary_search",
        run: ascii_binary_search,
    },
    Case {
        name: "ascii_format_integer",
        run: ascii_format_integer,
    },
    Case {
        name: "ascii_format_real",
        run: ascii_format_real,
    },
    Case {
        name: "extended_cat_search",
        run: extended_cat_search,
    },
    Case {
        name: "hascii_cat_search",
        run: hascii_cat_search,
    },
    Case {
        name: "hextended_from_ascii",
        run: hextended_from_ascii,
    },
];

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

/// Same native operation as the generic conversion above, using bounded input.
fn ascii_borrowed_cat_search(n: usize) -> Sample {
    let appends = n / 10;
    let start = Instant::now();
    let mut text = AsciiString::default();
    for _ in 0..appends {
        text.assign_cat_c_bytes(Some(black_box(b"ab")));
    }
    text.assign_cat_c_bytes(Some(b"needle"));
    let found = text.search("needle");
    Sample::since(start, appends, i64::from(found))
}

fn ascii_binary_search(n: usize) -> Sample {
    let bytes = b"\xFF\0a";
    let mut text = AsciiString::filled(1024, b'x');
    text.insert(513, bytes).unwrap();
    let start = Instant::now();
    let mut sum = 0_i64;
    for _ in 0..n / 10 {
        sum += i64::from(black_box(&text).search(black_box(bytes)));
        sum += i64::from(black_box(&text).search_from_end(black_box(bytes)));
    }
    Sample::since(start, n / 5, sum)
}

fn ascii_format_integer(n: usize) -> Sample {
    let start = Instant::now();
    let mut sum = 0_i64;
    for value in 0..n {
        let text = AsciiString::from_integer(i32::MIN + black_box(value as i32));
        sum = sum.wrapping_add(ascii_text_checksum(text.as_bytes()));
    }
    Sample::since(start, n, sum)
}

fn ascii_format_real(n: usize) -> Sample {
    let start = Instant::now();
    let mut sum = 0_i64;
    for value in 0..n {
        let text = AsciiString::from_real(black_box(value as f64) * -0.000_000_100_001);
        sum = sum.wrapping_add(ascii_text_checksum(text.as_bytes()));
    }
    Sample::since(start, n, sum)
}

fn ascii_text_checksum(bytes: &[u8]) -> i64 {
    bytes.iter().fold(0_i64, |sum, &byte| {
        sum.wrapping_mul(31).wrapping_add(i64::from(byte))
    })
}

fn extended_cat_search(n: usize) -> Sample {
    let appends = n / 10;
    let start = Instant::now();
    let mut text = ExtendedString::default();
    for _ in 0..appends {
        text.assign_cat_c_units(Some(black_box(&[97, 98, 0])));
    }
    let needle = [78, 0x20ac];
    text.assign_cat_c_units(Some(&needle));
    Sample::since(start, appends, i64::from(text.search(needle)))
}

fn hascii_cat_search(n: usize) -> Sample {
    let appends = n / 10;
    let start = Instant::now();
    let text = HAsciiString::default();
    for _ in 0..appends {
        text.assign_cat(black_box("ab"));
    }
    text.assign_cat("needle");
    Sample::since(start, appends, i64::from(text.search("needle")))
}

fn hextended_from_ascii(n: usize) -> Sample {
    let ascii = HAsciiString::new("WIDE \u{20ac}");
    let start = Instant::now();
    let mut sum = 0_i64;
    for _ in 0..n {
        let text = HExtendedString::from_h_ascii_string(black_box(&ascii));
        let text = text.string();
        sum += i64::from(text.length()) + i64::from(text.value(text.length()).unwrap());
    }
    Sample::since(start, n, sum)
}
