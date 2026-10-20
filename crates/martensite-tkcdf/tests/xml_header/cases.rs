//! Owned XML inputs and explicitly specified format results.
pub const VALID_HEADERS: &[(&str, &str, &str)] = &[
    (
        "root",
        "<document format='Owned'><body></document>",
        "Owned",
    ),
    ("self-closing", "<document format='Owned'/>", "Owned"),
    (
        "declaration",
        "<?xml version='1.0'?><document format='Owned'>",
        "Owned",
    ),
    (
        "nested",
        "<root><document format='Nested'><broken",
        "Nested",
    ),
    ("spaces", "<document format=' a b '/>", " a b "),
    ("numeric", "<document format='123'/>", "123"),
    ("hex-string", "<document format='##feff00410042'/>", "AB"),
];

pub const EMPTY_HEADERS: &[(&str, &str)] = &[
    ("empty", ""),
    ("no-format", "<document/>"),
    ("empty-format", "<document format=''/>"),
    ("other-root", "<other format='Wrong'/>"),
    ("case-sensitive", "<Document format='Wrong'/>"),
    ("bad-prefix", "not XML <document format='Wrong'/>"),
    ("unfinished-attributes", "<document format='Wrong' broken"),
    ("unfinished-value", "<document format='Wrong"),
];
