//! The subset of script regular expressions the test-log rules use: literal characters, `.`,
//! bracket classes such as `[Ee]`, `[0-9]` and `[*]`, `\y` (script's word boundary, which
//! the harness substitutes for Perl's `\b`) and the `*` quantifier. Every `parse.rules`
//! line and every `TODO` pattern of the cases is written in this subset.

/// One element of a compiled pattern.
#[derive(Clone, Debug, PartialEq)]
enum Atom {
    Char(char),
    Any,
    Class(Vec<(char, char)>),
    WordBoundary,
}

/// An atom and whether it repeats (`*`).
#[derive(Clone, Debug, PartialEq)]
struct Piece {
    atom: Atom,
    repeated: bool,
}

/// A compiled pattern; `Pattern::new(r"\yFaulty\y").is_match("Faulty case")`.
#[derive(Clone, Debug)]
pub struct Pattern {
    pieces: Vec<Piece>,
}

impl Pattern {
    /// Compiles `source`, with `\b` read as `\y` like the harness does.
    pub fn new(source: &str) -> Self {
        let mut pieces: Vec<Piece> = Vec::new();
        let mut chars = source.chars().peekable();
        while let Some(c) = chars.next() {
            let atom = match c {
                '.' => Atom::Any,
                '[' => Atom::Class(class(&mut chars, source)),
                '\\' => escaped(chars.next(), source),
                '*' => {
                    pieces.last_mut().expect("`*` after an atom").repeated = true;
                    continue;
                }
                other => Atom::Char(other),
            };
            pieces.push(Piece {
                atom,
                repeated: false,
            });
        }
        Self { pieces }
    }

    /// Whether the pattern matches anywhere in `line` (script `regexp`).
    pub fn is_match(&self, line: &str) -> bool {
        let text: Vec<char> = line.chars().collect();
        (0..=text.len()).any(|start| matches_at(&self.pieces, &text, start))
    }
}

fn class(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, source: &str) -> Vec<(char, char)> {
    let mut ranges = Vec::new();
    loop {
        let first = chars
            .next()
            .unwrap_or_else(|| panic!("pattern {source:?}: unterminated bracket class"));
        if first == ']' {
            return ranges;
        }
        if chars.peek() == Some(&'-') {
            chars.next();
            let last = chars.next().expect("class range end");
            ranges.push((first, last));
        } else {
            ranges.push((first, first));
        }
    }
}

fn escaped(next: Option<char>, source: &str) -> Atom {
    match next {
        Some('y' | 'b') => Atom::WordBoundary,
        Some(other) if !other.is_alphanumeric() => Atom::Char(other),
        other => panic!("pattern {source:?}: unsupported escape {other:?}"),
    }
}

fn is_word(c: Option<&char>) -> bool {
    c.is_some_and(|c| c.is_alphanumeric() || *c == '_')
}

fn atom_matches(atom: &Atom, c: char) -> bool {
    match atom {
        Atom::Char(expected) => *expected == c,
        Atom::Any => true,
        Atom::Class(ranges) => ranges.iter().any(|(a, b)| (*a..=*b).contains(&c)),
        Atom::WordBoundary => false,
    }
}

fn matches_at(pieces: &[Piece], text: &[char], at: usize) -> bool {
    let Some((piece, rest)) = pieces.split_first() else {
        return true;
    };
    if piece.atom == Atom::WordBoundary {
        let before = at.checked_sub(1).and_then(|i| text.get(i));
        return is_word(before) != is_word(text.get(at)) && matches_at(rest, text, at);
    }
    if piece.repeated {
        let mut end = at;
        while end < text.len() && atom_matches(&piece.atom, text[end]) {
            end += 1;
        }
        return (at..=end).rev().any(|stop| matches_at(rest, text, stop));
    }
    at < text.len() && atom_matches(&piece.atom, text[at]) && matches_at(rest, text, at + 1)
}

#[test]
fn parse_rule_patterns_follow_script_regexp() {
    let error = Pattern::new(r"\b[Ee][Rr][Rr][Oo][Rr]\b");
    assert!(error.is_match("Error: wrong"));
    assert!(error.is_match("x ERROR y"));
    assert!(!error.is_match("Errors"));
    assert!(!error.is_match("my_error"));
    let exception = Pattern::new(r"\b[Ee]xception\b");
    assert!(exception.is_match("An exception was caught"));
    assert!(!exception.is_match("NO EXCEPTION."));
    let missing = Pattern::new(r"ScriptStep Exception: .*[fF]ile .* could not be found");
    assert!(missing.is_match("ScriptStep Exception: File a.brep could not be found"));
    assert!(
        Pattern::new(r"ScriptStep Exception: [*][*] Exception [*][*]")
            .is_match("ScriptStep Exception: ** Exception **")
    );
    assert!(Pattern::new(r"\b[Ff][Aa][Ii][Ll][0-9]\b").is_match("FAIL1"));
    assert!(Pattern::new("Error in ii: T=").is_match("Error in ii: T=0.5\tD=1"));
    assert!(Pattern::new("result.").is_match("result_1"));
}
