//! A manifest exemption requires bounded literal inputs, without platform alias assumptions (#481).
use crate::release_macro_inputs::{literal_string, source_macro_identifier};
use proc_macro2::TokenTree;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn literal_is_disjoint(
    token: Option<&TokenTree>,
    parent: &Path,
    package: &Path,
) -> bool {
    let Some(value) = literal_argument(token) else {
        return false;
    };
    let Some(target) = owned_input(&value, parent, package) else {
        return false;
    };
    distinct_bytes(&target, &package.join("Cargo.toml")).unwrap_or(false)
}

fn literal_argument(token: Option<&TokenTree>) -> Option<String> {
    let TokenTree::Group(arguments) = token? else {
        return None;
    };
    let tokens: Vec<_> = arguments.stream().into_iter().collect();
    let [TokenTree::Literal(value)] = tokens.as_slice() else {
        return None;
    };
    literal_string(&value.to_string())
}

fn owned_input(value: &str, parent: &Path, package: &Path) -> Option<PathBuf> {
    if !portable_input_name(value) {
        return None;
    }
    let target = parent.join(value);
    if target
        .ancestors()
        .take_while(|path| path.starts_with(package))
        .any(Path::is_symlink)
    {
        return None;
    }
    let target = target.canonicalize().ok()?;
    (target.starts_with(package.canonicalize().ok()?) && target.is_file()).then_some(target)
}

fn portable_input_name(value: &str) -> bool {
    let path = Path::new(value);
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    value.is_ascii()
        && !path.is_absolute()
        && value.chars().all(|ch| !matches!(ch, '\\' | ':' | '~'))
        && !name
            .trim_end_matches([' ', '.'])
            .eq_ignore_ascii_case("Cargo.toml")
}

fn distinct_bytes(target: &Path, manifest: &Path) -> Option<bool> {
    if manifest.is_symlink() {
        return None;
    }
    let included = fs::metadata(target).ok()?;
    let source = fs::metadata(manifest).ok()?;
    if !included.is_file() || !source.is_file() {
        return None;
    }
    if included.len() != source.len() {
        return Some(true);
    }
    Some(fs::read(target).ok()? != fs::read(manifest).ok()?)
}

pub(crate) fn is_include_keyword(tokens: &[TokenTree]) -> bool {
    matches!(tokens.first(), Some(TokenTree::Ident(name))
        if matches!(source_macro_identifier(name).as_str(), "include" | "include_bytes" | "include_str"))
}
pub(crate) fn is_include_macro(tokens: &[TokenTree]) -> bool {
    is_include_keyword(tokens)
        && matches!(tokens.get(1), Some(TokenTree::Punct(mark)) if mark.as_char() == '!')
}
