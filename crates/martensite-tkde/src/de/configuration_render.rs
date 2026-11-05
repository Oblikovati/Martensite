//! Exact resource-template expansion shared by typed format configuration slices.
use martensite_tkernel::{
    standard::{Failure, exceptions},
    tcollection::AsciiString,
};

/// Expand each `{{Name}}` exactly once without interpreting substituted bytes.
///
/// ```
/// use martensite_tkde::de::render_configuration;
/// let result = render_configuration("value : {{Value}}\n", &[("Value", 7.into())]).unwrap();
/// assert_eq!(result.as_bytes(), b"value : 7\n");
/// ```
pub fn render_configuration(
    template: &str,
    substitutions: &[(&str, AsciiString)],
) -> Result<AsciiString, Failure> {
    validate_bindings(template, substitutions)?;
    let mut remaining = template;
    let mut rendered = Vec::new();
    while let Some(start) = remaining.find("{{") {
        rendered.extend_from_slice(&remaining.as_bytes()[..start]);
        let (name, tail) = placeholder_tail(&remaining[start + 2..])?;
        let value = substitution_value(name, substitutions)?;
        rendered.extend_from_slice(value.as_bytes());
        remaining = tail;
    }
    rendered.extend_from_slice(remaining.as_bytes());
    Ok(AsciiString::from_bytes(rendered))
}

fn validate_bindings(template: &str, substitutions: &[(&str, AsciiString)]) -> Result<(), Failure> {
    for (index, (name, _)) in substitutions.iter().enumerate() {
        let marker = format!("{{{{{name}}}}}");
        let occurrences = template.matches(&marker).count();
        let duplicate = substitutions[..index]
            .iter()
            .any(|(prior, _)| prior == name);
        if name.is_empty() || occurrences != 1 || duplicate {
            return Err(exceptions::program_error(&format!(
                "configuration placeholder {name:?}: occurrences={occurrences}, duplicate={duplicate}; expected one unique binding and one template occurrence"
            )));
        }
    }
    Ok(())
}

fn placeholder_tail(text: &str) -> Result<(&str, &str), Failure> {
    text.split_once("}}").ok_or_else(|| {
        exceptions::program_error(&format!(
            "configuration placeholder {text:?}: expected closing double brace"
        ))
    })
}

fn substitution_value<'a>(
    name: &str,
    substitutions: &'a [(&str, AsciiString)],
) -> Result<&'a AsciiString, Failure> {
    substitutions
        .iter()
        .find_map(|(key, value)| (*key == name).then_some(value))
        .ok_or_else(|| {
            exceptions::program_error(&format!(
                "configuration placeholder {name:?}: expected one supplied binding"
            ))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_are_literal_bytes_and_comment_braces_remain_literal() {
        let input = AsciiString::from_bytes(b"{{Second}}\xff");
        let rendered = render_configuration(
            "!{sequence<string>}\na:{{First}}\nb:{{Second}}\n",
            &[("First", input), ("Second", AsciiString::from_integer(3))],
        )
        .unwrap();
        assert_eq!(
            rendered.as_bytes(),
            b"!{sequence<string>}\na:{{Second}}\xff\nb:3\n"
        );
    }

    #[test]
    fn malformed_missing_duplicate_and_unused_bindings_are_rejected() {
        for (template, bindings) in [
            ("{{Value", vec![]),
            ("{{Value}}", vec![]),
            ("{{Value}}{{Value}}", vec![("Value", 1.into())]),
            ("{{Value}}", vec![("Value", 1.into()), ("Value", 2.into())]),
            ("literal", vec![("Value", 1.into())]),
            ("{{}}", vec![("", 1.into())]),
        ] {
            let error = render_configuration(template, &bindings).unwrap_err();
            assert_eq!(error.exception_type(), "Standard_ProgramError");
            assert!(error.message().contains("expected"));
        }
    }
}
