//! Real STEP files of the test data, read with the header protocol only. By default the
//! smallest files; `MARTENSITE_STEP_CORPUS_ALL=1` reads them all. With
//! `STEP_CORE_STREAM_DIR` each read's stream is written there.
use super::support::read_file;
use std::path::{Path, PathBuf};

const SAMPLE_SIZE: usize = 12;

fn step_files() -> Vec<PathBuf> {
    let root = std::env::var_os("STEP_CORE_CORPUS_DIR").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../../external/martensite-test-data"),
        PathBuf::from,
    );
    let mut found = Vec::new();
    let mut pending = vec![root];
    while let Some(folder) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&folder) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            let extension = path
                .extension()
                .map_or_else(String::new, |e| e.to_string_lossy().to_lowercase());
            if extension == "stp" || extension == "step" {
                found.push(path);
            }
        }
    }
    found.sort_by_key(|path| std::fs::metadata(path).map_or(0, |m| m.len()));
    found
}

/// An independent count: the entity instances of a file are its `#n=` at the start of a
/// statement, outside strings and comments.
fn count_instances(text: &[u8]) -> usize {
    let mut count = 0;
    let mut index = 0;
    let mut at_start = true;
    while index < text.len() {
        match text[index] {
            b'\'' => {
                index += 1;
                while index < text.len() && text[index] != b'\'' {
                    index += 1;
                }
                at_start = false;
            }
            b'/' if text.get(index + 1) == Some(&b'*') => {
                index += 2;
                while index + 1 < text.len() && &text[index..index + 2] != b"*/" {
                    index += 1;
                }
                index += 1;
            }
            b';' => at_start = true,
            b'#' if at_start => {
                count += 1;
                at_start = false;
            }
            byte if !byte.is_ascii_whitespace() => at_start = false,
            _ => {}
        }
        index += 1;
    }
    count
}

fn unspaced(text: &[u8]) -> String {
    let kept: Vec<u8> = text
        .iter()
        .copied()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    String::from_utf8_lossy(&kept).into_owned()
}

#[test]
fn real_files_read_and_write_back() {
    let everything = std::env::var_os("MARTENSITE_STEP_CORPUS_ALL").is_some();
    let stream_dir = std::env::var_os("STEP_CORE_STREAM_DIR").map(PathBuf::from);
    let files = step_files();
    assert!(!files.is_empty(), "the test data holds STEP files");
    let take = if everything {
        files.len()
    } else {
        SAMPLE_SIZE.min(files.len())
    };
    for path in files.iter().take(take) {
        let name = path.to_string_lossy().into_owned();
        let read = read_file(&name);
        if let Some(folder) = &stream_dir {
            let target = folder.join(format!(
                "{}.txt",
                path.file_name().expect("a name").to_string_lossy()
            ));
            std::fs::write(target, read.stream()).expect("the stream is written");
            continue;
        }
        assert_eq!(read.status, 0, "{name}: {:?}", read.fails());
        let text = std::fs::read(path).expect("the file");
        let written = read.written();
        // Reading what was written gives the same text, but for the margins of wrapped
        // strings, also: the writer loses nothing.
        let again = super::support::read_text(&written);
        assert_eq!(again.status, 0, "{name} again: {:?}", again.fails());
        // A read with fails writes them as a comment, which the next read skips.
        if read.fails().is_empty() {
            assert_eq!(unspaced(&again.written()), unspaced(&written), "{name}");
        }
        if read.fails().is_empty() {
            assert_eq!(count_instances(&written), count_instances(&text), "{name}");
        }
    }
}
