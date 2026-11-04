use super::*;
use std::collections::BTreeMap;

struct NamedB6InputFiles {
    located: BTreeMap<String, PathBuf>,
    regular: Vec<PathBuf>,
}

impl NamedB6InputFiles {
    fn complete(root: &Path) -> Self {
        let located: BTreeMap<String, PathBuf> = FONT_NAMES
            .iter()
            .map(|name| ((*name).to_owned(), root.join(name)))
            .collect();
        let regular = located.values().cloned().collect();
        Self { located, regular }
    }
}

impl B6InputFiles for NamedB6InputFiles {
    fn locate(&self, name: &str) -> Result<PathBuf, Failure> {
        self.located.get(name).cloned().ok_or_else(|| {
            program_error(&format!("fake missing {name:?}; expected named B6 input"))
        })
    }

    fn is_regular_file(&self, path: &Path) -> bool {
        self.regular.iter().any(|candidate| candidate == path)
    }
}

#[test]
fn three_typed_paths_retain_exact_names_and_order() {
    let root = Path::new("/owned-fonts");
    let files = NamedB6InputFiles::complete(root);
    let paths = resolve_b6_inputs(None, &files).unwrap();
    assert_eq!(paths.olf, root.join("OLFTestFont-Regular.ttf"));
    assert_eq!(paths.machine_tool, root.join("machtgth.ttf"));
    assert_eq!(paths.sans, root.join("DejaVuSans.ttf"));
}

#[test]
fn explicit_private_directory_does_not_consult_canonical_paths() {
    let root = Path::new("/private-fonts");
    let mut files = NamedB6InputFiles::complete(root);
    files.located.clear();
    let paths = resolve_b6_inputs(Some(root), &files).unwrap();
    assert_eq!(paths.sans, root.join("DejaVuSans.ttf"));
}

#[test]
fn missing_private_input_never_uses_an_available_canonical_font() {
    let files = NamedB6InputFiles::complete(Path::new("/canonical-fonts"));
    let error = resolve_b6_inputs(Some(Path::new("/missing-private")), &files);
    assert!(error.is_err());
}

#[test]
fn wrong_basename_is_rejected_even_when_it_is_a_regular_font_file() {
    let root = Path::new("/owned-fonts");
    let mut files = NamedB6InputFiles::complete(root);
    let replacement = root.join("replacement.ttf");
    files
        .located
        .insert(FONT_NAMES[0].to_owned(), replacement.clone());
    files.regular.push(replacement);
    assert!(resolve_b6_inputs(None, &files).is_err());
}

#[test]
fn missing_required_member_fails_before_any_font_is_opened() {
    let root = Path::new("/owned-fonts");
    let mut files = NamedB6InputFiles::complete(root);
    files
        .regular
        .retain(|path| path != &root.join("machtgth.ttf"));
    assert!(resolve_b6_inputs(None, &files).is_err());
}

#[test]
fn typed_paths_are_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<B6FontPaths>();
}
