use super::*;

#[test]
fn capture_directory_and_basename_preserve_the_existing_join_contract() {
    let root = std::env::var_os("MARTENSITE_OPENGL_CASE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("martensite-opengl-original-captures"));
    let expected = root.join(std::process::id().to_string());
    for basename in ["owned.png", "nested/image.png", ""] {
        assert_eq!(capture_path(basename).unwrap(), expected.join(basename));
        assert!(expected.is_dir());
    }
}
