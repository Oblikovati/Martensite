//! Follow owned literal module inputs without scanning unrelated QA siblings (#485).
use super::{ProductionReferences, inline_module, macro_owner, scan_tokens};
use crate::release_input_layout::covered_rust_input;
use proc_macro2::{TokenStream, TokenTree};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(super) struct SourceScope<'a> {
    pub(super) parent: &'a Path,
    pub(super) package: &'a Path,
    pub(super) owner: Option<String>,
    pub(super) inline_module: bool,
    module_directory: Option<PathBuf>,
}
impl<'a> SourceScope<'a> {
    pub(super) fn new(parent: &'a Path, package: &'a Path) -> Self {
        Self {
            parent,
            package,
            owner: None,
            inline_module: false,
            module_directory: None,
        }
    }
    pub(super) fn within(&self, tokens: &[TokenTree], index: usize) -> Self {
        let inline = inline_module(tokens, index);
        Self {
            parent: self.parent,
            package: self.package,
            owner: macro_owner(tokens, index).or_else(|| self.owner.clone()),
            inline_module: self.inline_module || inline,
            module_directory: nested_directory(
                self.module_directory.as_deref(),
                tokens,
                index,
                inline,
            ),
        }
    }
}
fn nested_directory(
    base: Option<&Path>,
    tokens: &[TokenTree],
    index: usize,
    inline: bool,
) -> Option<PathBuf> {
    let base = base?;
    if !inline {
        return Some(base.to_owned());
    }
    let TokenTree::Ident(name) = tokens.get(index.checked_sub(1)?)? else {
        return None;
    };
    Some(base.join(name.to_string().trim_start_matches("r#")))
}
pub(super) fn scan_directory(
    directory: &Path,
    root: &Path,
    refs: &mut ProductionReferences,
) -> Option<()> {
    if !refs
        .scanned_directories
        .insert(directory.canonicalize().ok()?)
    {
        return Some(());
    }
    for entry in sorted_entries(directory)? {
        scan_entry(entry, root, refs)?;
    }
    Some(())
}
fn sorted_entries(directory: &Path) -> Option<Vec<fs::DirEntry>> {
    let mut entries = fs::read_dir(directory)
        .ok()?
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    entries.sort_by_key(fs::DirEntry::path);
    Some(entries)
}
fn scan_entry(entry: fs::DirEntry, root: &Path, refs: &mut ProductionReferences) -> Option<()> {
    let kind = entry.file_type().ok()?;
    if kind.is_symlink() {
        return None;
    }
    if kind.is_dir() {
        return scan_child(&entry.path(), root, refs);
    }
    if entry.path().extension().is_some_and(|ext| ext == "rs")
        && entry.path() != root.join("build.rs")
    {
        scan_source(&entry.path(), root, refs, None)?;
    }
    Some(())
}
fn scan_child(path: &Path, root: &Path, refs: &mut ProductionReferences) -> Option<()> {
    if ["tests", "target", ".git"]
        .iter()
        .any(|name| path == root.join(name))
    {
        return Some(());
    }
    if path
        .file_name()
        .is_some_and(|name| matches!(name.to_str(), Some("target" | ".git")))
    {
        return None;
    }
    scan_directory(path, root, refs)
}
fn scan_source(
    path: &Path,
    root: &Path,
    refs: &mut ProductionReferences,
    module_directory: Option<PathBuf>,
) -> Option<()> {
    let path = path.canonicalize().ok()?;
    let key = (path.clone(), module_directory.clone());
    if !refs.scanned_sources.insert(key) {
        return Some(());
    }
    let tokens = source_tokens(&path)?;
    if refs.collecting {
        refs.macros.record(tokens.clone());
    }
    let mut scope = SourceScope::new(path.parent()?, root);
    scope.module_directory = module_directory;
    scan_tokens(tokens, path.parent()?, root, refs, &scope)
}
fn source_tokens(path: &Path) -> Option<TokenStream> {
    let source = fs::read_to_string(path).ok()?;
    source.parse().ok()
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RustSourceKind {
    Module,
    TextualInclude,
}
pub(super) fn record_source(
    value: &str,
    scope: &SourceScope<'_>,
    refs: &mut ProductionReferences,
    kind: RustSourceKind,
) -> Option<()> {
    if scope.owner.is_some() {
        return None;
    }
    let path = owned_source(value, scope)?;
    if covered_rust_input(scope.package, &path) {
        return Some(());
    }
    let directory = qa_module_directory(&path, scope.package, kind)?;
    refs.paths.insert(directory.clone());
    scan_source(&path, scope.package, refs, Some(directory))
}
fn qa_module_directory(path: &Path, package: &Path, kind: RustSourceKind) -> Option<PathBuf> {
    if kind != RustSourceKind::Module
        || !path.starts_with(package.canonicalize().ok()?.join("tests"))
    {
        return None;
    }
    Some(path.parent()?.to_path_buf())
}
fn owned_source(value: &str, scope: &SourceScope<'_>) -> Option<PathBuf> {
    if !super::literal_input_supported(value, scope)
        || Path::new(value)
            .extension()
            .is_none_or(|extension| extension != "rs")
    {
        return None;
    }
    owned_path(&scope.parent.join(value), scope.package)
}
fn owned_path(path: &Path, package: &Path) -> Option<PathBuf> {
    if path.ancestors().any(Path::is_symlink) {
        return None;
    }
    let path = path.canonicalize().ok()?;
    let package = package.canonicalize().ok()?;
    let relative = path.strip_prefix(&package).ok()?;
    // Native separators are not authored backslashes (#487).
    if !crate::release_input_layout::input_components_supported(relative) {
        return None;
    }
    path.is_file().then_some(path)
}
pub(super) fn check_module(
    tokens: &[TokenTree],
    index: usize,
    scope: &SourceScope<'_>,
    refs: &mut ProductionReferences,
) -> Option<()> {
    let Some(directory) = scope.module_directory.as_deref() else {
        return Some(());
    };
    let Some(name) = implicit_name(tokens, index)? else {
        return Some(());
    };
    if scope.owner.is_some() {
        return None;
    }
    let directory = directory.join(name);
    let path = implicit_source(&directory, scope.package)?;
    refs.paths.insert(path.parent()?.to_path_buf());
    scan_source(&path, scope.package, refs, Some(directory))
}
fn implicit_name(tokens: &[TokenTree], index: usize) -> Option<Option<String>> {
    if !matches!(&tokens[index], TokenTree::Ident(name) if name == "mod") {
        return Some(None);
    }
    if matches!(tokens.get(index + 2), Some(TokenTree::Group(_))) || explicit_path(tokens, index) {
        return Some(None);
    }
    let [
        TokenTree::Ident(_),
        TokenTree::Ident(name),
        TokenTree::Punct(mark),
        ..,
    ] = &tokens[index..]
    else {
        return None;
    };
    (mark.as_char() == ';').then(|| Some(name.to_string().trim_start_matches("r#").to_owned()))
}
fn implicit_source(directory: &Path, package: &Path) -> Option<PathBuf> {
    let candidates = [directory.with_extension("rs"), directory.join("mod.rs")];
    let present: Vec<_> = candidates.iter().filter(|path| path.exists()).collect();
    let [path] = present.as_slice() else {
        return None;
    };
    owned_path(path, package)
}
fn visibility_start(tokens: &[TokenTree], mut end: usize) -> usize {
    if end > 0
        && matches!(&tokens[end - 1], TokenTree::Group(group) if group.delimiter() == proc_macro2::Delimiter::Parenthesis)
    {
        end -= 1;
    }
    if end > 0 && matches!(&tokens[end - 1], TokenTree::Ident(name) if name == "pub") {
        end -= 1;
    }
    end
}
fn explicit_path(tokens: &[TokenTree], end: usize) -> bool {
    let mut end = visibility_start(tokens, end);
    while end >= 2 {
        let (TokenTree::Punct(mark), TokenTree::Group(group)) =
            (&tokens[end - 2], &tokens[end - 1])
        else {
            break;
        };
        if mark.as_char() != '#' || group.delimiter() != proc_macro2::Delimiter::Bracket {
            break;
        }
        if super::mentions_path(group.stream()) {
            return true;
        }
        end -= 2;
    }
    false
}

#[cfg(test)]
#[path = "release_rust_inputs_tests.rs"]
mod tests;
