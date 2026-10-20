//! Source lexical reference paths; filesystem canonicalization would change their meaning.
use martensite_tkernel::tcollection::AsciiString;

// TODO: corroborate the defined Windows syntax interface on a Windows native build.
pub(super) fn resolved_path(directory: &AsciiString, file: &AsciiString) -> AsciiString {
    absolute_reference_path(directory.as_bytes(), file.as_bytes(), cfg!(windows))
        .map_or_else(|| file.clone(), |bytes| AsciiString::from_bytes(&bytes))
}

fn absolute_reference_path(directory: &[u8], file: &[u8], windows: bool) -> Option<Vec<u8>> {
    if reference_has_absolute_prefix(file, windows) {
        return Some(file.to_vec());
    }
    if !reference_has_absolute_prefix(directory, windows) {
        return None;
    }
    let mut directory = collapse_reference_separators(directory, windows);
    let mut file = collapse_reference_separators(file, windows);
    reduce_leading_parent_segments(&mut directory, &mut file)?;
    directory.push(b'/');
    directory.extend(file);
    Some(directory)
}

fn reduce_leading_parent_segments(directory: &mut Vec<u8>, file: &mut Vec<u8>) -> Option<()> {
    while file.starts_with(b"../") {
        if file.len() == 3 {
            return None;
        }
        file.drain(..3);
        if directory.is_empty() {
            return None;
        }
        let separator = directory.iter().rposition(|byte| *byte == b'/')?;
        directory.truncate(separator);
    }
    Some(())
}

fn reference_has_absolute_prefix(path: &[u8], windows: bool) -> bool {
    if !windows {
        return path.starts_with(b"/");
    }
    path.iter().position(|byte| *byte == b':') == Some(1) || path.starts_with(b"\\\\")
}

fn collapse_reference_separators(path: &[u8], windows: bool) -> Vec<u8> {
    let path = reference_separator_bytes(path, windows);
    let skip = usize::from(windows).min(path.len());
    let mut result = path[..skip].to_vec();
    let mut slash = false;
    for byte in &path[skip..] {
        if *byte == b'/' && slash {
            continue;
        }
        result.push(*byte);
        slash = *byte == b'/';
    }
    if result.last() == Some(&b'/') {
        result.pop();
    }
    result
}

fn reference_separator_bytes(path: &[u8], windows: bool) -> Vec<u8> {
    path.iter()
        .map(|byte| {
            if windows && *byte == b'\\' {
                b'/'
            } else {
                *byte
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::absolute_reference_path;

    #[test]
    fn unix_paths_preserve_only_the_source_lexical_reductions() {
        let cases = [
            ("/a//b/", "child.std", Some("/a/b/child.std")),
            ("/a/b/", "../child.std", Some("/a/child.std")),
            ("/a/b/", "../../child.std", Some("/child.std")),
            ("/a/b/", "../../../child.std", None),
            ("relative/", "child.std", None),
            ("/a/b/", "sub//part.std", Some("/a/b/sub/part.std")),
            ("/a/b/", "./dot.std", Some("/a/b/./dot.std")),
            ("/a/b/", "inner/../part.std", Some("/a/b/inner/../part.std")),
            ("/a/b/", "../", Some("/a/b/..")),
            ("/a/b/", "/absolute//child", Some("/absolute//child")),
            ("/", "child", Some("/child")),
        ];
        for (directory, file, expected) in cases {
            assert_eq!(
                absolute_reference_path(directory.as_bytes(), file.as_bytes(), false).as_deref(),
                expected.map(str::as_bytes),
                "{directory}:{file}"
            );
        }
    }

    #[test]
    fn windows_interface_retains_drive_unc_and_separator_rules() {
        let cases = [
            ("C:\\a\\b\\", "..\\child.std", Some("C:/a/child.std")),
            (
                "\\\\host\\share\\dir\\",
                "child",
                Some("//host/share/dir/child"),
            ),
            ("C:\\a\\", "C:child", Some("C:child")),
            ("C:\\a\\", "\\child", Some("C:/a//child")),
            (
                "\\\\host\\share\\",
                "\\\\other\\child",
                Some("\\\\other\\child"),
            ),
            ("C:a", "child", Some("C:a/child")),
            ("rel\\", "child", None),
            ("//host/share", "child", None),
            ("::", "child", None),
        ];
        for (directory, file, expected) in cases {
            assert_eq!(
                absolute_reference_path(directory.as_bytes(), file.as_bytes(), true).as_deref(),
                expected.map(str::as_bytes),
                "{directory}:{file}"
            );
        }
    }
}
