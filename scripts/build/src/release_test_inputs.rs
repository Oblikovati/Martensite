//! QA-only paths do not create new publication identities (#466).
use crate::release_manifest_inputs::{is_include_keyword, is_include_macro};
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
    manifest_proof: bool,
    manifest_inputs_known: bool,
    changed_extensions: BTreeSet<String>,
}

impl ProductionReferences {
    fn new(changed: &[ChangedFile], manifest_proof: bool) -> Self {
        Self {
            paths: BTreeSet::new(),
            macros: LiteralMacroInputs::default(),
            collecting: true,
            manifest_proof,
            manifest_inputs_known: true,
            changed_extensions: changed_extensions(changed),
        }
    }
}

struct SourceScope<'a> {
    parent: &'a Path,
    package: &'a Path,
    owner: Option<String>,
    inline_module: bool,
}
impl<'a> SourceScope<'a> {
    fn new(parent: &'a Path, package: &'a Path) -> Self {
        Self {
            parent,
            package,
            owner: None,
            inline_module: false,
        }
    }
    fn within(&self, tokens: &[TokenTree], index: usize) -> Self {
        Self {
            parent: self.parent,
            package: self.package,
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
    let Some(references) = production_references(&package, manifest, changed, false) else {
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
    manifest_proof: bool,
) -> Option<ProductionReferences> {
    if !crate::release_input_layout::known_production_layout(root, manifest) {
        return None;
    }
    let mut references = ProductionReferences::new(changed, manifest_proof);
    scan_directory(root, root, &mut references)?;
    references.paths.clear();
    references.collecting = false;
    scan_directory(root, root, &mut references)?;
    Some(references)
}
/// A test-target manifest edit is QA-only only when no production input retains its bytes.
pub(crate) fn manifest_is_unreferenced(
    root: &Path,
    manifest: &PackageManifest,
    changed: &[ChangedFile],
) -> bool {
    let Some(references) =
        production_references(root, manifest, &with_manifest_evidence(changed), true)
    else {
        return false;
    };
    references.manifest_inputs_known
        && !references
            .paths
            .contains(&normalized(&root.join("Cargo.toml")))
}
fn with_manifest_evidence(changed: &[ChangedFile]) -> Vec<ChangedFile> {
    let mut inputs = changed.to_vec();
    inputs.push(ChangedFile {
        path: "Cargo.toml".into(),
        deleted: false,
        untracked: false,
    });
    inputs
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
fn scan_directory(directory: &Path, root: &Path, refs: &mut ProductionReferences) -> Option<()> {
    for entry in fs::read_dir(directory).ok()? {
        let entry = entry.ok()?;
        let kind = entry.file_type().ok()?;
        if kind.is_symlink() {
            return None;
        }
        if kind.is_dir() {
            scan_child(&entry.path(), root, refs)?;
        } else if entry.path().extension().is_some_and(|ext| ext == "rs")
            && entry.path() != root.join("build.rs")
        {
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
    scan_tokens(
        tokens,
        path.parent()?,
        root,
        refs,
        &SourceScope::new(path.parent()?, root),
    )
}
fn scan_tokens(
    tokens: TokenStream,
    parent: &Path,
    root: &Path,
    refs: &mut ProductionReferences,
    scope: &SourceScope<'_>,
) -> Option<()> {
    let tokens: Vec<_> = tokens.into_iter().collect();
    for (index, token) in tokens.iter().enumerate() {
        if !refs.collecting {
            check_include(&tokens[index..], refs, scope)?;
        }
        check_attribute(&tokens, index, scope)?;
        scan_token(token, parent, root, refs, &scope.within(&tokens, index))?;
    }
    Some(())
}
fn scan_token(
    token: &TokenTree,
    parent: &Path,
    root: &Path,
    refs: &mut ProductionReferences,
    scope: &SourceScope<'_>,
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
fn check_attribute(tokens: &[TokenTree], index: usize, scope: &SourceScope<'_>) -> Option<()> {
    if !attribute_prefix(tokens, index) {
        return Some(());
    }
    if let TokenTree::Group(group) = &tokens[index] {
        return check_path_attribute(group, scope);
    }
    Some(())
}
fn attribute_prefix(tokens: &[TokenTree], index: usize) -> bool {
    let punctuation = |offset, expected| {
        index
            .checked_sub(offset)
            .and_then(|i| tokens.get(i))
            .is_some_and(
                |token| matches!(token, TokenTree::Punct(mark) if mark.as_char() == expected),
            )
    };
    punctuation(1, '#') || punctuation(1, '!') && punctuation(2, '#')
}
fn check_include(
    tokens: &[TokenTree],
    refs: &mut ProductionReferences,
    scope: &SourceScope<'_>,
) -> Option<()> {
    record_manifest_include(tokens, refs, scope);
    if !is_include_macro(tokens) {
        return Some(());
    }
    if matches!(tokens.first(), Some(TokenTree::Ident(name)) if source_macro_identifier(name) == "include")
    {
        return rust_include_argument(tokens.get(2), scope);
    }
    include_argument(tokens.get(2), refs, scope.owner.as_deref())
}
fn record_manifest_include(
    tokens: &[TokenTree],
    refs: &mut ProductionReferences,
    scope: &SourceScope<'_>,
) {
    if !refs.manifest_proof || !refs.manifest_inputs_known || !is_include_keyword(tokens) {
        return;
    }
    refs.manifest_inputs_known = is_include_macro(tokens)
        && crate::release_manifest_inputs::literal_is_disjoint(
            tokens.get(2),
            scope.parent,
            scope.package,
        );
}
fn rust_include_argument(token: Option<&TokenTree>, scope: &SourceScope<'_>) -> Option<()> {
    let Some(TokenTree::Group(arguments)) = token else {
        return None;
    };
    let arguments: Vec<_> = arguments.stream().into_iter().collect();
    let [TokenTree::Literal(value)] = arguments.as_slice() else {
        return None;
    };
    owned_rust_source(&decode_string(&value.to_string())?, scope)
}
fn owned_rust_source(value: &str, scope: &SourceScope<'_>) -> Option<()> {
    if value.contains('\\')
        || Path::new(value)
            .extension()
            .is_none_or(|extension| extension != "rs")
    {
        return None;
    }
    let path = scope.parent.join(value);
    if path.is_symlink() {
        return None;
    }
    let path = path.canonicalize().ok()?;
    (path.starts_with(scope.package.canonicalize().ok()?) && path.is_file()).then_some(())
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
    if path == normalized(&root.join("Cargo.toml")) {
        refs.paths.insert(path);
        return Some(());
    }
    if path.starts_with(root.join("tests"))
        && refs.paths.insert(path.clone())
        && path.extension().is_some_and(|extension| extension == "rs")
    {
        if !path
            .canonicalize()
            .ok()?
            .starts_with(root.canonicalize().ok()?)
        {
            return None;
        }
        scan_source(&path, root, refs)?;
    }
    Some(())
}

fn check_path_attribute(group: &proc_macro2::Group, scope: &SourceScope<'_>) -> Option<()> {
    if group.delimiter() != proc_macro2::Delimiter::Bracket || !mentions_path(group.stream()) {
        return Some(());
    }
    if scope.inline_module {
        return None;
    }
    owned_rust_source(&direct_path_literal(group.stream())?, scope)
}
fn mentions_path(stream: TokenStream) -> bool {
    stream.into_iter().any(|token| match token {
        TokenTree::Ident(name) => source_macro_identifier(&name) == "path",
        TokenTree::Group(group) => mentions_path(group.stream()),
        _ => false,
    })
}
fn direct_path_literal(stream: TokenStream) -> Option<String> {
    let tokens: Vec<_> = stream.into_iter().collect();
    let [
        TokenTree::Ident(name),
        TokenTree::Punct(equal),
        TokenTree::Literal(value),
    ] = tokens.as_slice()
    else {
        return None;
    };
    if source_macro_identifier(name) != "path" || equal.as_char() != '=' {
        return None;
    }
    decode_string(&value.to_string())
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
