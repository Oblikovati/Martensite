//! The patterns of `TODO` and `REQUIRED` lines: the small part of regular expressions the
//! scripts use. Anything else is refused by name, not matched as a literal by accident.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Atom {
    Literal(char),
    AnyCharacter,
    WordBoundary,
}

/// A compiled pattern: literals, `.`, `\b` or `\y`, and backslash-escaped characters.
/// Example: `TodoPattern::compile("Faulty shapes").matches("Faulty shapes in variables")`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoPattern {
    atoms: Vec<Atom>,
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

impl TodoPattern {
    /// Compiles `text`; panics, naming it, when it needs a regular expression feature this
    /// matcher lacks, so that a new pattern extends the matcher instead of silently missing.
    /// Example: `TodoPattern::compile(r"curve\(s\) expected")`.
    pub fn compile(text: &str) -> Self {
        let mut atoms = Vec::new();
        let mut characters = text.chars();
        while let Some(character) = characters.next() {
            atoms.push(match character {
                '\\' => Self::escaped(characters.next(), text),
                '.' => Atom::AnyCharacter,
                '*' | '+' | '?' | '[' | ']' | '(' | ')' | '{' | '}' | '|' | '^' | '$' => {
                    panic!("TODO pattern {text:?}: {character:?} needs regular expression support")
                }
                literal => Atom::Literal(literal),
            });
        }
        Self { atoms }
    }

    fn escaped(character: Option<char>, text: &str) -> Atom {
        match character {
            Some('b' | 'y') => Atom::WordBoundary,
            Some(literal) if !literal.is_alphanumeric() => Atom::Literal(literal),
            other => panic!("TODO pattern {text:?}: escape {other:?} needs regular expressions"),
        }
    }

    /// Whether the pattern occurs in `line`; `TodoPattern::compile("a.c").matches("xabc")`.
    pub fn matches(&self, line: &str) -> bool {
        let characters: Vec<char> = line.chars().collect();
        (0..=characters.len()).any(|start| self.matches_at(&characters, start))
    }

    fn matches_at(&self, line: &[char], start: usize) -> bool {
        let mut at = start;
        for atom in &self.atoms {
            match atom {
                Atom::WordBoundary => {
                    if !at_word_boundary(line, at) {
                        return false;
                    }
                }
                Atom::AnyCharacter if at < line.len() => at += 1,
                Atom::Literal(literal) if line.get(at) == Some(literal) => at += 1,
                _ => return false,
            }
        }
        true
    }
}

fn at_word_boundary(line: &[char], at: usize) -> bool {
    let before = at > 0 && is_word_character(line[at - 1]);
    let after = line.get(at).is_some_and(|c| is_word_character(*c));
    before != after
}

#[test]
fn literals_dots_and_escapes_match_as_in_the_scripts() {
    let area = TodoPattern::compile("Error : The command is not valid. The area is");
    assert!(area.matches("Error : The command is not valid. The area is 0."));
    assert!(!area.matches("Error : The area of result shape is 3"));
    let escaped = TodoPattern::compile(r"Error: 1 intersection curve\(s\) expected but");
    assert!(escaped.matches("Error: 1 intersection curve(s) expected but 3"));
}

#[test]
fn word_boundaries_need_a_word_edge() {
    let word = TodoPattern::compile(r"\bFaulty\b");
    assert!(word.matches("a Faulty shape"));
    assert!(!word.matches("NotFaulty shape"));
}

#[test]
#[should_panic(expected = "needs regular expression support")]
fn a_quantifier_is_refused_by_name() {
    TodoPattern::compile("Faulty.*");
}
