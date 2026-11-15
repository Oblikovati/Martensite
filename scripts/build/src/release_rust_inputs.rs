//! Filesystem traversal for the conservative production-input proof.
use super::{ProductionReferences, inline_module, macro_owner, scan_tokens};
use proc_macro2::{TokenStream, TokenTree};
use std::{fs, path::Path};

pub(super) struct SourceScope<'a> {
    pub(super) parent: &'a Path,
    pub(super) package: &'a Path,
    pub(super) owner: Option<String>,
    pub(super) inline_module: bool,
}
impl<'a> SourceScope<'a> {
    pub(super) fn new(parent: &'a Path, package: &'a Path) -> Self {
        Self {
            parent,
            package,
            owner: None,
            inline_module: false,
        }
    }
    pub(super) fn within(&self, tokens: &[TokenTree], index: usize) -> Self {
        Self {
            parent: self.parent,
            package: self.package,
            owner: macro_owner(tokens, index).or_else(|| self.owner.clone()),
            inline_module: self.inline_module || inline_module(tokens, index),
        }
    }
}
pub(super) fn scan_directory(directory: &Path, root: &Path, refs: &mut ProductionReferences) -> Option<()> {
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
