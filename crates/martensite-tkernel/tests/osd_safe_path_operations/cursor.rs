//! Native directory ownership and naming contracts before replacement (#89).
use super::{Errno, NativePosixCalls, PathScratch, PosixFileCalls};
use martensite_tkernel::osd::DirectoryEntries;

fn drain_cursor(mut cursor: Box<dyn DirectoryEntries>) -> Vec<Vec<u8>> {
    let names = std::iter::from_fn(|| cursor.next_name()).collect();
    assert_eq!(cursor.next_name(), None);
    names
}

#[test]
fn native_cursor_preserves_dot_entries_and_bulk_order() {
    let scratch = PathScratch::new("cursor-order");
    std::fs::write(scratch.0.join("é.step"), b"payload").unwrap();
    let path = scratch.bytes(b"");
    let bulk = NativePosixCalls.read_directory(&path).unwrap();
    let mut terminated = path.clone();
    terminated.extend_from_slice(b"\0ignored");
    let actual = drain_cursor(NativePosixCalls.open_directory(&terminated).unwrap());
    assert_eq!(actual, bulk);
    let mut names = actual;
    names.sort();
    assert_eq!(
        names,
        vec![b".".to_vec(), b"..".to_vec(), "é.step".as_bytes().to_vec()]
    );
}

#[test]
fn native_cursor_survives_directory_rename_and_thread_transfer() {
    let scratch = PathScratch::new("cursor-owned");
    let source = scratch.0.join("source");
    std::fs::create_dir(&source).unwrap();
    std::fs::write(source.join("child"), b"payload").unwrap();
    let cursor = NativePosixCalls
        .open_directory(&scratch.bytes(b"source"))
        .unwrap();
    std::fs::rename(&source, scratch.0.join("moved")).unwrap();
    std::fs::create_dir(&source).unwrap();
    std::fs::write(source.join("replacement"), b"other").unwrap();
    let mut names = std::thread::spawn(move || drain_cursor(cursor))
        .join()
        .unwrap();
    names.sort();
    assert_eq!(
        names,
        vec![b".".to_vec(), b"..".to_vec(), b"child".to_vec()]
    );
}

#[test]
fn native_cursor_open_reports_path_errors_immediately() {
    let scratch = PathScratch::new("cursor-errors");
    std::fs::write(scratch.0.join("file"), b"payload").unwrap();
    for (path, code) in [
        (scratch.bytes(b"missing"), libc::ENOENT),
        (scratch.bytes(b"file"), libc::ENOTDIR),
        (b"\0ignored".to_vec(), libc::ENOENT),
    ] {
        assert_eq!(
            NativePosixCalls.open_directory(&path).err(),
            Some(Errno(code))
        );
    }
}
