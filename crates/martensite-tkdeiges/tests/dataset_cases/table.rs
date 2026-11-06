//! The table types of the dataset cases: data only, no API call.

/// What the reference run ended in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecordedVerdict {
    Ok,
    /// `BAD (known problem)`: every problem is declared by a `TODO`.
    Known,
    Failed,
}

/// One command of a script. Words are already substituted; a path word is `@data/name` for
/// `[locate_data_file name]` and `@image/name` for `${imagedir}/name`.
#[derive(Debug)]
pub enum Step {
    Run(&'static str, &'static [&'static str]),
    /// `catch {command}`: a failure of the command does not stop the script.
    Catch(&'static str, &'static [&'static str]),
}

/// A script with its own commands; the group's `begin` and `end` come from `cpu_seconds`.
pub struct ScriptCase {
    pub source: &'static str,
    /// The limit the chain sets last; `de_wrapper` sets none.
    pub cpu_seconds: Option<i64>,
    pub recorded: RecordedVerdict,
    pub steps: &'static [Step],
}

/// One script of the offset family: a surface read, made one shell, offset, then checked.
pub struct OffsetCase {
    pub source: &'static str,
    pub recorded: RecordedVerdict,
    /// The lines the script says first, `TODO`s and heading.
    pub heading: &'static [&'static str],
    pub file: &'static str,
    /// The variable the file is read into and the one the shell is made in.
    pub names: (&'static str, &'static str),
    /// `shape_divide` rather than `sewing`.
    pub divided: bool,
    /// What the script says of the join and of the shape.
    pub said: (&'static str, &'static str),
    pub value: f64,
    /// The word of the value, as the command echoes it.
    pub value_word: &'static str,
    /// Join by intersection rather than by arc.
    pub intersection: bool,
    /// The expected area, as the script writes it.
    pub area: &'static str,
    pub counts: &'static [&'static str],
    /// A `cpulimit` of its own.
    pub cpu_seconds: i64,
}
