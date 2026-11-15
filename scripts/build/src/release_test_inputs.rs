//! QA-only paths do not create new publication identities (#466).
use crate::release_macro_inputs::{
    LiteralMacroInputs, filename_extension, literal_string as decode_string,
    source_macro_identifier,
};
use crate::{git::ChangedFile, package_manifest::PackageManifest};
use proc_macro2::{TokenStream, TokenTree};
use std::{
    borrow::Cow,
    collections::BTreeSet,
    fs,
    path::{Component, Path, PathBuf},
};

struct ProductionReferences {
    paths: BTreeSet<PathBuf>,
    macros: LiteralMacroInputs,
    collecting: bool,
    changed_extensions: BTreeSet<String>,
}

#[derive(Default)]
struct SourceScope {
    owner: Option<String>,
    inline_module: bool,
}
impl SourceScope {
    fn within(&self, tokens: &[TokenTree], index: usize) -> Self {
        Self {
            owner: macro_owner(tokens, index).or_else(|| self.owner.clone()),
            inline_module: self.inline_module || inline_module(tokens, index),
        }
    }
}
/// Retains uncertain production inputs while separating proven QA-only edits.
/// Example: `publication_changes(root, manifest, directory, changed)`.
pub(crate) fn publication_changes<'a>(
    root: &Path,
    manifest: &PackageManifest,
    directory: &str,
    changed: &'a [ChangedFile],
) -> Cow<'a, [ChangedFile]> {
    let prefix = format!("{directory}/tests/");
    if !changed.iter().any(|file| file.path.starts_with(&prefix))
        || known_source_edit(directory, changed)
    {
        return Cow::Borrowed(changed);
    }
    let package = root.join(directory);
    let Some(references) = production_references(&package, manifest, changed) else {
        return Cow::Borrowed(changed);
    };
    Cow::Owned(filtered_changes(root, changed, &prefix, &references))
}
fn known_source_edit(directory: &str, changed: &[ChangedFile]) -> bool {
    let prefix = format!("{directory}/");
    let tests = format!("{directory}/tests/");
    let manifest = format!("{directory}/Cargo.toml");
    changed.iter().any(|file| {
        file.path.starts_with(&prefix) && !file.path.starts_with(&tests) && file.path != manifest
    })
}
fn filtered_changes(
    root: &Path,
    changed: &[ChangedFile],
    prefix: &str,
    refs: &ProductionReferences,
) -> Vec<ChangedFile> {
    changed
        .iter()
        .filter(|file| {
            !file.path.starts_with(prefix)
                || refs
                    .paths
                    .iter()
                    .any(|input| normalized(&root.join(&file.path)).starts_with(input))
        })
        .cloned()
        .collect()
}

fn production_references(
    root: &Path,
    manifest: &PackageManifest,
    changed: &[ChangedFile],
) -> Option<ProductionReferences> {
    if root.join("build.rs").exists() || custom_build(manifest) || custom_target(manifest) {
        return None;
    }
    let mut references = ProductionReferences {
        paths: BTreeSet::new(),
        macros: LiteralMacroInputs::default(),
        collecting: true,
        changed_extensions: changed_extensions(changed),
    };
    scan_directory(root, root, &mut references)?;
    references.paths.clear();
    references.collecting = false;
    scan_directory(root, root, &mut references)?;
    Some(references)
}
fn changed_extensions(changed: &[ChangedFile]) -> BTreeSet<String> {
    changed
        .iter()
        .map(|file| {
            Path::new(&file.path)
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or("")
                .to_ascii_lowercase()
        })
        .collect()
}
fn custom_build(manifest: &PackageManifest) -> bool {
    manifest
        .source
        .get("package")
        .and_then(|p| p.get("build"))
        .is_some_and(|build| build.as_str().is_some())
}
fn custom_target(manifest: &PackageManifest) -> bool {
    let library = manifest.source.get("lib").into_iter();
    let binaries = manifest
        .source
        .get("bin")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten();
    library.chain(binaries).any(|target| {
        target
            .get("path")
            .and_then(toml::Value::as_str)
            .is_some_and(|path| !path.starts_with("src/"))
    })
}
fn scan_directory(directory: &Path, root: &Path, refs: &mut ProductionReferences) -> Option<()> {
    for entry in fs::read_dir(directory).ok()? {
        let entry = entry.ok()?;
        let kind = entry.file_type().ok()?;
        if kind.is_symlink() {
            return None;
        }
        if kind.is_dir() {
            scan_child(&entry.path(), root, refs)?;
        } else if entry.path().extension().is_some_and(|ext| ext == "rs") {
            scan_source(&entry.path(), root, refs)?;
        }
    }
    Some(())
}
fn scan_child(path: &Path, root: &Path, refs: &mut ProductionReferences) -> Option<()> {
    if path == root.join("tests")
        || path
            .file_name()
            .is_some_and(|name| matches!(name.to_str(), Some("target" | ".git")))
    {
        return Some(());
    }
    scan_directory(path, root, refs)
}
fn scan_source(path: &Path, root: &Path, refs: &mut ProductionReferences) -> Option<()> {
    let source = fs::read_to_string(path).ok()?;
    let tokens = source.parse::<TokenStream>().ok()?;
    if refs.collecting {
        refs.macros.record(tokens.clone());
    }
    scan_tokens(tokens, path.parent()?, root, refs, &SourceScope::default())
}
fn scan_tokens(
    tokens: TokenStream,
    parent: &Path,
    root: &Path,
    refs: &mut ProductionReferences,
    scope: &SourceScope,
) -> Option<()> {
    let tokens: Vec<_> = tokens.into_iter().collect();
    for (index, token) in tokens.iter().enumerate() {
        if !refs.collecting {
            check_include(&tokens[index..], refs, scope.owner.as_deref())?;
        }
        check_attribute(&tokens, index, scope.inline_module)?;
        scan_token(token, parent, root, refs, &scope.within(&tokens, index))?;
    }
    Some(())
}
fn scan_token(
    token: &TokenTree,
    parent: &Path,
    root: &Path,
    refs: &mut ProductionReferences,
    scope: &SourceScope,
) -> Option<()> {
    match token {
        TokenTree::Group(group) => scan_tokens(group.stream(), parent, root, refs, scope),
        TokenTree::Literal(literal) => record_literal(&literal.to_string(), parent, root, refs),
        _ => Some(()),
    }
}
fn inline_module(tokens: &[TokenTree], index: usize) -> bool {
    index >= 2
        && matches!(&tokens[index - 2], TokenTree::Ident(keyword) if keyword == "mod")
        && matches!(&tokens[index - 1], TokenTree::Ident(_))
        && matches!(&tokens[index], TokenTree::Group(group) if group.delimiter() == proc_macro2::Delimiter::Brace)
}
fn macro_owner(tokens: &[TokenTree], index: usize) -> Option<String> {
    if index < 3 {
        return None;
    }
    let TokenTree::Ident(keyword) = &tokens[index - 3] else {
        return None;
    };
    if keyword != "macro_rules" {
        return None;
    }
    let TokenTree::Ident(name) = &tokens[index - 1] else {
        return None;
    };
    Some(source_macro_identifier(name))
}
fn concat_parameter(tokens: &[TokenTree]) -> Option<String> {
    let tokens = concat_arguments(tokens)?;
    let end = tokens
        .iter()
        .rposition(|token| !matches!(token, TokenTree::Punct(p) if p.as_char() == ','))?;
    let TokenTree::Ident(parameter) = tokens.get(end)? else {
        return None;
    };
    let TokenTree::Punct(dollar) = tokens.get(end.checked_sub(1)?)? else {
        return None;
    };
    (dollar.as_char() == '$').then(|| source_macro_identifier(parameter))
}
fn check_attribute(tokens: &[TokenTree], index: usize, inline: bool) -> Option<()> {
    if index == 0 || !matches!(&tokens[index - 1], TokenTree::Punct(mark) if mark.as_char() == '#')
    {
        return Some(());
    }
    if let TokenTree::Group(group) = &tokens[index] {
        return check_path_attribute(group, inline);
    }
    Some(())
}
fn check_include(
    tokens: &[TokenTree],
    refs: &ProductionReferences,
    owner: Option<&str>,
) -> Option<()> {
    if is_include_macro(tokens) {
        return include_argument(tokens.get(2), refs, owner);
    }
    Some(())
}
fn is_include_macro(tokens: &[TokenTree]) -> bool {
    let Some(TokenTree::Ident(name)) = tokens.first() else {
        return false;
    };
    matches!(
        source_macro_identifier(name).as_str(),
        "include" | "include_bytes" | "include_str"
    ) && matches!(tokens.get(1), Some(TokenTree::Punct(mark)) if mark.as_char() == '!')
}
fn include_argument(
    token: Option<&TokenTree>,
    refs: &ProductionReferences,
    owner: Option<&str>,
) -> Option<()> {
    let Some(TokenTree::Group(arguments)) = token else {
        return None;
    };
    let arguments: Vec<_> = arguments.stream().into_iter().collect();
    if let [TokenTree::Literal(literal)] = arguments.as_slice() {
        return decode_string(&literal.to_string()).map(|_| ());
    }
    let extension = fixed_include_extension(&arguments).or_else(|| {
        refs.macros
            .extension(owner?, &concat_parameter(&arguments)?)
    })?;
    (!refs.changed_extensions.contains(&extension)).then_some(())
}
fn fixed_include_extension(tokens: &[TokenTree]) -> Option<String> {
    let suffix = concat_suffix(tokens)?;
    filename_extension(&suffix)
}
fn concat_suffix(tokens: &[TokenTree]) -> Option<String> {
    let tokens = concat_arguments(tokens)?;
    let suffix = tokens
        .iter()
        .rev()
        .find(|token| !matches!(token, TokenTree::Punct(p) if p.as_char() == ','))?;
    let TokenTree::Literal(suffix) = suffix else {
        return None;
    };
    decode_string(&suffix.to_string())
}
fn concat_arguments(tokens: &[TokenTree]) -> Option<Vec<TokenTree>> {
    let [
        TokenTree::Ident(name),
        TokenTree::Punct(mark),
        TokenTree::Group(arguments),
    ] = tokens
    else {
        return None;
    };
    if source_macro_identifier(name) != "concat" || mark.as_char() != '!' {
        return None;
    }
    Some(arguments.stream().into_iter().collect())
}
fn record_literal(
    literal: &str,
    parent: &Path,
    root: &Path,
    refs: &mut ProductionReferences,
) -> Option<()> {
    let Some(value) = decode_string(literal) else {
        return Some(());
    };
    let value = value.replace('\\', "/");
    for path in [parent.join(&value), root.join(value)] {
        record_path(normalized(&path), root, refs)?;
    }
    Some(())
}
fn record_path(path: PathBuf, root: &Path, refs: &mut ProductionReferences) -> Option<()> {
    if path.starts_with(root.join("tests"))
        && refs.paths.insert(path.clone())
        && path.extension().is_some_and(|extension| extension == "rs")
    {
        scan_source(&path, root, refs)?;
    }
    Some(())
}

fn check_path_attribute(group: &proc_macro2::Group, inline: bool) -> Option<()> {
    if group.delimiter() != proc_macro2::Delimiter::Bracket {
        return Some(());
    }
    let tokens: Vec<_> = group.stream().into_iter().collect();
    let Some(TokenTree::Ident(name)) = tokens.first() else {
        return Some(());
    };
    if source_macro_identifier(name) != "path" {
        return Some(());
    }
    if inline {
        return None;
    }
    let Some(TokenTree::Literal(value)) = tokens.get(2) else {
        return None;
    };
    decode_string(&value.to_string()).map(|_| ())
}
fn normalized(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                result.pop();
            }
            Component::CurDir => {}
            other => result.push(other.as_os_str()),
        }
    }
    result
}

#[cfg(test)]
#[path = "release_test_inputs_tests.rs"]
mod tests;
