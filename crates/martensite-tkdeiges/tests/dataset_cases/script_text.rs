//! What the end scripts ask of script: reading a number before a marker, a list's words, `format %g`.

/// A character of the scripts' number class `[-0-9.+eE]`.
fn is_number_char(byte: u8) -> bool {
    byte.is_ascii_digit() || matches!(byte, b'-' | b'.' | b'+' | b'e' | b'E')
}

fn is_blank(byte: u8) -> bool {
    byte == b' ' || byte == b'\t'
}

/// The sum `[\t ]+([-0-9.+eE]+)[\t ]+<marker>` finds in `text`, as `data c`'s and `tpstat c`'s parts of
/// `de/end` count warnings and faulties. Example: `sum_before_marker("  3\tW: a\n 2\tW: b", "W:") == 5`.
pub fn sum_before_marker(text: &str, marker: &str) -> i64 {
    let bytes = text.as_bytes();
    let mut total = 0;
    for (at, _) in text.match_indices(marker) {
        if let Some(count) = number_before(bytes, at) {
            total += count;
        }
    }
    total
}

fn number_before(bytes: &[u8], marker_at: usize) -> Option<i64> {
    let mut end = marker_at;
    while end > 0 && is_blank(bytes[end - 1]) {
        end -= 1;
    }
    if end == marker_at {
        return None;
    }
    let mut start = end;
    while start > 0 && is_number_char(bytes[start - 1]) {
        start -= 1;
    }
    if start == end || start == 0 || !is_blank(bytes[start - 1]) {
        return None;
    }
    let word = std::str::from_utf8(&bytes[start..end]).ok()?;
    Some(
        word.parse()
            .unwrap_or_else(|_| panic!("expr: {word:?}; expected an integer count")),
    )
}

/// The list a script's `lsort` reads: words split on blanks, with double-quoted and braced ones whole.
/// Example: `list_words("\"a b\" c ")` is `["a b", "c"]`.
pub fn list_words(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut words = Vec::new();
    let mut at = 0;
    while at < chars.len() {
        if chars[at].is_whitespace() {
            at += 1;
            continue;
        }
        let (word, next) = list_word(&chars, at);
        words.push(word);
        at = next;
    }
    words
}

fn list_word(chars: &[char], start: usize) -> (String, usize) {
    let close = match chars[start] {
        '"' => '"',
        '{' => '}',
        _ => {
            let end = (start..chars.len())
                .find(|&i| chars[i].is_whitespace())
                .unwrap_or(chars.len());
            return (chars[start..end].iter().collect(), end);
        }
    };
    let end = (start + 1..chars.len())
        .find(|&i| chars[i] == close)
        .unwrap_or_else(|| {
            panic!(
                "list: unmatched {close} in {:?}",
                chars.iter().collect::<String>()
            )
        });
    (chars[start + 1..end].iter().collect(), end + 1)
}

/// `[lsort text]`, as `format %s` then prints it. Example: `lsort_text("b a ") == "a b"`.
pub fn lsort_text(text: &str) -> String {
    let mut words = list_words(text);
    words.sort();
    let quoted: Vec<String> = words.iter().map(|word| list_element(word)).collect();
    quoted.join(" ")
}

/// A word as a script list prints it: braced when it holds a blank or is empty.
fn list_element(word: &str) -> String {
    let special =
        word.is_empty() || word.contains(|c: char| c.is_whitespace() || "[]$;\"\\{}".contains(c));
    if special {
        format!("{{{word}}}")
    } else {
        word.to_owned()
    }
}

/// C's `%14.10g`, which the `TOLERANCE` report line uses. Example: `general_14_10(1e-7)` is
/// `"         1e-07"`.
pub fn general_14_10(value: f64) -> String {
    format!("{:>14}", general_10(value))
}

/// C's `%.10g`, without its width. Example: `general_10(0.5) == "0.5"`.
pub fn general_10(value: f64) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    let scientific = format!("{value:.9e}");
    let (mantissa, exponent) = scientific.split_once('e').expect("a formatted exponent");
    let exponent: i32 = exponent.parse().expect("an integer exponent");
    if (-4..10).contains(&exponent) {
        let decimals = usize::try_from(9 - exponent).expect("exponent below 10");
        return trim_zeros(&format!("{value:.decimals$}")).to_owned();
    }
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{}e{sign}{:02}", trim_zeros(mantissa), exponent.abs())
}

fn trim_zeros(text: &str) -> &str {
    if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.')
    } else {
        text
    }
}

/// The list of `split $line " "` then the first `count` words `string is integer|double -strict`
/// accepts, skipping the line's first word, as the reference comparison reads a report line.
/// Example: `numbers_of_line("A = 1  ( 2 )", 2, false)` is `["1", "2"]`.
pub fn numbers_of_line(line: &str, count: usize, double: bool) -> Vec<String> {
    let accepts = |word: &str| {
        if double {
            word.parse::<f64>().is_ok() && !word.is_empty()
        } else {
            is_tcl_integer(word)
        }
    };
    line.split(' ')
        .skip(1)
        .filter(|word| accepts(word))
        .take(count)
        .map(str::to_owned)
        .collect()
}

fn is_tcl_integer(word: &str) -> bool {
    let digits = word.strip_prefix(['-', '+']).unwrap_or(word);
    !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
}

#[test]
fn counts_only_numbers_between_blanks_before_the_marker() {
    let text = " Count\tCheck\n    32\tW:(A): one\n     3\tW:(B): two\n x4\tW:(C)\n  Nb Total:35";
    assert_eq!(sum_before_marker(text, "W:"), 35);
    assert_eq!(sum_before_marker(text, "F:"), 0);
}

#[test]
fn general_format_follows_c() {
    assert_eq!(general_10(0.0002663644062), "0.0002663644062");
    assert_eq!(general_10(1e-7), "1e-07");
    assert_eq!(general_14_10(1.0001e-7), "    1.0001e-07");
    assert_eq!(general_14_10(0.0), "             0");
    assert_eq!(general_10(0.004247173073949), "0.004247173074");
    assert_eq!(general_10(1.0), "1");
}

#[test]
fn lsort_keeps_a_named_layer_whole() {
    assert_eq!(lsort_text("YELLOW RED BLUE "), "BLUE RED YELLOW");
    // As a script sorts them: a blank sorts before a digit.
    assert_eq!(lsort_text("\"Layer 2\" \"Layer1\" "), "{Layer 2} Layer1");
    assert_eq!(lsort_text(""), "");
}
