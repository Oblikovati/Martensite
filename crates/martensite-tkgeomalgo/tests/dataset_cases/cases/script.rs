//! What every case script does around its commands: the banner it prints and the script
//! error for a variable a `regexp` did not set.
use crate::support::script::{Script, ScriptError, ScriptStep};

/// `puts "<rule>"; puts "<title>"; puts "<rule>"; puts ""`.
pub fn banner(d: &mut Script, rule: &str, title: &str) {
    d.puts(rule);
    d.puts(title);
    d.puts(rule);
    d.puts("");
}

/// A `regexp ... full var` capture the script reads afterwards: when the pattern did not
/// match, the variable is unset and reading it is a script error.
pub fn captured<'a>(found: Option<&'a str>, variable: &str) -> ScriptStep<&'a str> {
    found.ok_or_else(|| ScriptError(format!("can't read \"{variable}\": no such variable")))
}
