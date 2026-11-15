//! Finite literal macro arguments bound otherwise dynamic file extensions (#466).
use proc_macro2::{TokenStream, TokenTree};
use std::collections::BTreeMap;

#[derive(Default)]
pub(crate) struct LiteralMacroInputs {
    definitions: BTreeMap<String, Option<Vec<String>>>,
    calls: BTreeMap<String, Vec<Vec<Option<String>>>>,
    aliases: std::collections::BTreeSet<String>,
    exported: bool,
}
impl LiteralMacroInputs {
    pub(crate) fn record(&mut self, stream: TokenStream) {
        let tokens: Vec<_> = stream.into_iter().collect();
        let mut index = 0;
        while index < tokens.len() {
            if self.definition(&tokens[index..]) {
                index += 4;
                continue;
            }
            self.name_uncertainty(&tokens[index..], tokens.get(index.wrapping_sub(1)));
            self.invocation(&tokens[index..]);
            if let TokenTree::Group(group) = &tokens[index] {
                self.record(group.stream());
            }
            index += 1;
        }
    }
    fn definition(&mut self, tokens: &[TokenTree]) -> bool {
        let Some((name, body)) = macro_definition(tokens) else {
            return false;
        };
        self.exported |= !source_macro_identifier(name).is_ascii();
        let parsed = matcher_parameters(body.stream());
        self.record(body.stream());
        self.definitions
            .entry(source_macro_identifier(name))
            .and_modify(|old| {
                if *old != parsed {
                    *old = None;
                }
            })
            .or_insert(parsed);
        true
    }
    fn invocation(&mut self, tokens: &[TokenTree]) {
        if let Some((name, arguments)) = literal_call(tokens) {
            self.exported |= !name.is_ascii();
            self.calls.entry(name).or_default().push(arguments);
        }
    }
    fn name_uncertainty(&mut self, tokens: &[TokenTree], previous: Option<&TokenTree>) {
        self.exported |= matches!(tokens.first(), Some(TokenTree::Ident(name))
            if source_macro_identifier(name) == "macro_export");
        self.exported |= matches!(previous, Some(TokenTree::Punct(mark)) if mark.as_char() == '$')
            && matches!(tokens.get(1), Some(TokenTree::Punct(mark)) if mark.as_char() == '!');
        if let Some(name) = aliased_macro(tokens) {
            self.aliases.insert(name);
        }
    }
    pub(crate) fn extension(&self, owner: &str, parameter: &str) -> Option<String> {
        if self.exported || self.aliases.contains(owner) {
            return None;
        }
        let parameters = self.definitions.get(owner)?.as_ref()?;
        let index = parameters.iter().position(|name| name == parameter)?;
        let calls = self.calls.get(owner)?;
        let mut extension = None;
        for call in calls {
            if call.len() != parameters.len() {
                return None;
            }
            let current = filename_extension(call.get(index)?.as_ref()?)?;
            if extension.as_ref().is_some_and(|old| old != &current) {
                return None;
            }
            extension = Some(current);
        }
        extension
    }
}
fn aliased_macro(tokens: &[TokenTree]) -> Option<String> {
    let [
        TokenTree::Ident(name),
        TokenTree::Ident(rename),
        TokenTree::Ident(_),
        ..,
    ] = tokens
    else {
        return None;
    };
    (rename == "as").then(|| source_macro_identifier(name))
}
fn macro_definition(tokens: &[TokenTree]) -> Option<(&proc_macro2::Ident, &proc_macro2::Group)> {
    let [
        TokenTree::Ident(keyword),
        TokenTree::Punct(mark),
        TokenTree::Ident(name),
        TokenTree::Group(body),
        ..,
    ] = tokens
    else {
        return None;
    };
    if keyword != "macro_rules" || mark.as_char() != '!' {
        return None;
    }
    Some((name, body))
}
fn literal_call(tokens: &[TokenTree]) -> Option<(String, Vec<Option<String>>)> {
    let [
        TokenTree::Ident(name),
        TokenTree::Punct(mark),
        TokenTree::Group(args),
        ..,
    ] = tokens
    else {
        return None;
    };
    if mark.as_char() != '!' {
        return None;
    }
    let values = literal_values(args.stream());
    Some((source_macro_identifier(name), values))
}
fn literal_values(stream: TokenStream) -> Vec<Option<String>> {
    split_arguments(stream)
        .into_iter()
        .map(|argument| match argument.as_slice() {
            [TokenTree::Literal(value)] => literal_string(&value.to_string()),
            _ => None,
        })
        .collect()
}
fn matcher_parameters(stream: TokenStream) -> Option<Vec<String>> {
    let matcher = literal_matcher(stream)?;
    split_arguments(matcher)
        .into_iter()
        .map(|argument| literal_parameter(&argument))
        .collect()
}
fn literal_matcher(stream: TokenStream) -> Option<TokenStream> {
    let tokens: Vec<_> = stream.into_iter().collect();
    let [
        TokenTree::Group(matcher),
        TokenTree::Punct(equal),
        TokenTree::Punct(arrow),
        TokenTree::Group(_),
        tail @ ..,
    ] = tokens.as_slice()
    else {
        return None;
    };
    if equal.as_char() != '=' || arrow.as_char() != '>' || tail.len() > 1 {
        return None;
    }
    Some(matcher.stream())
}
fn literal_parameter(tokens: &[TokenTree]) -> Option<String> {
    let [
        TokenTree::Punct(dollar),
        TokenTree::Ident(name),
        TokenTree::Punct(colon),
        TokenTree::Ident(kind),
    ] = tokens
    else {
        return None;
    };
    if dollar.as_char() != '$' || colon.as_char() != ':' || kind != "literal" {
        return None;
    }
    Some(source_macro_identifier(name))
}
fn split_arguments(stream: TokenStream) -> Vec<Vec<TokenTree>> {
    let mut arguments = vec![Vec::new()];
    for token in stream {
        if matches!(&token, TokenTree::Punct(mark) if mark.as_char() == ',') {
            arguments.push(Vec::new());
        } else if let Some(last) = arguments.last_mut() {
            last.push(token);
        }
    }
    if arguments.last().is_some_and(Vec::is_empty) {
        arguments.pop();
    }
    arguments
}
pub(crate) fn literal_string(literal: &str) -> Option<String> {
    if literal.starts_with('"') {
        return serde_json::from_str(literal).ok();
    }
    if literal.starts_with('r') {
        let first = literal.find('"')?;
        let last = literal.rfind('"')?;
        return (last > first).then(|| literal[first + 1..last].to_owned());
    }
    None
}
pub(crate) fn filename_extension(suffix: &str) -> Option<String> {
    let extension = suffix.rsplit_once('.')?.1;
    if extension.is_empty() || !extension.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        return None;
    }
    Some(extension.to_ascii_lowercase())
}

pub(crate) fn source_macro_identifier(identifier: &proc_macro2::Ident) -> String {
    let spelling = identifier.to_string();
    spelling.strip_prefix("r#").unwrap_or(&spelling).to_owned()
}
