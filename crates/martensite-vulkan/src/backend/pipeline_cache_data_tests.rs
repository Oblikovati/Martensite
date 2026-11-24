use super::super::{PipelineCache, save_cache};
use super::{CacheIdentity, CacheStorage, MAX_CACHE_BYTES, OwnedCacheStage, checksum};
use crate::backend::resource_lifecycle_test_support::create_resource_renderer;
use std::{
    env, fs,
    io::{Seek, SeekFrom, Write},
};

fn identity(uuid: [u8; 16]) -> CacheIdentity {
    CacheIdentity {
        vendor: 0x1234,
        device: 0x5678,
        uuid,
    }
}

fn driver_bytes(tail: &[u8]) -> Vec<u8> {
    let mut bytes = vec![32, 0, 0, 0, 1, 0, 0, 0, 0x34, 0x12, 0, 0, 0x78, 0x56, 0, 0];
    bytes.extend([7; 16]);
    bytes.extend_from_slice(tail);
    bytes
}

fn fixture() -> (OwnedCacheStage, CacheStorage) {
    let root = OwnedCacheStage::create(&env::temp_dir()).unwrap();
    let storage = CacheStorage::at(root.data_path(), identity([7; 16]));
    (root, storage)
}

#[test]
fn valid_envelope_replaces_contents_without_leaving_staging_files() {
    let (root, storage) = fixture();
    let first = driver_bytes(&[5; 41]);
    storage.save(&first, checksum(&first)).unwrap();
    assert_eq!(storage.load().unwrap().bytes, first);
    let replacement = driver_bytes(&[9; 71]);
    storage.save(&replacement, checksum(&replacement)).unwrap();
    assert_eq!(storage.load().unwrap().bytes, replacement);
    assert_eq!(fs::read_dir(&root.0).unwrap().count(), 1);
}

#[test]
fn native_header_and_exact_blob_identity_guard_reload() {
    let (_root, storage) = fixture();
    let compatible = driver_bytes(&[5; 41]);
    assert!(storage.compatible(&compatible));
    for byte in [0, 4, 8, 12, 16] {
        let mut bad = compatible.clone();
        bad[byte] ^= 1;
        storage.save(&bad, checksum(&bad)).unwrap();
        assert!(storage.load().is_none(), "header byte {byte} was ignored");
    }
}

#[test]
fn truncated_corrupt_and_oversize_files_are_misses() {
    let (_root, storage) = fixture();
    let bytes = driver_bytes(&[5; 41]);
    storage.save(&bytes, checksum(&bytes)).unwrap();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(&storage.path)
        .unwrap();
    file.seek(SeekFrom::End(-1)).unwrap();
    file.write_all(&[99]).unwrap();
    assert!(storage.load().is_none());
    file.set_len(12).unwrap();
    assert!(storage.load().is_none());
    file.set_len(MAX_CACHE_BYTES as u64 + 100).unwrap();
    assert!(storage.load().is_none());
}

#[cfg(unix)]
#[test]
fn cache_file_symlinks_are_ignored() {
    let (root, storage) = fixture();
    let target = root.0.join("written-by-someone-else");
    fs::write(&target, b"external content").unwrap();
    std::os::unix::fs::symlink(&target, &storage.path).unwrap();
    assert!(storage.load().is_none());
    let replacement = driver_bytes(&[34]);
    storage.save(&replacement, checksum(&replacement)).unwrap();
    assert_eq!(fs::read(&target).unwrap(), b"external content");
    fs::remove_file(target).unwrap();
}

#[test]
fn saved_driver_cache_reloads_on_its_compatible_device() {
    let renderer = create_resource_renderer();
    let runtime = &renderer.runtime;
    let device = runtime.device();
    let (_root, initial) = fixture();
    // SAFETY: the renderer owns this instance and physical device.
    let properties = unsafe {
        runtime
            .instance
            .get_physical_device_properties(runtime.physical)
    };
    let storage = CacheStorage::at(initial.path, CacheIdentity::from_properties(&properties));
    let saved = runtime
        .pipeline_cache
        .with_handle(|handle| {
            save_cache(&storage, device, handle, None);
            storage
                .load()
                .ok_or("expected cache data from the actual device")
        })
        .unwrap();
    let mut restored = PipelineCache::new(device, Some(storage));
    assert_eq!(restored.loaded_checksum, Some(saved.checksum));
    restored.close(device);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[cfg(unix)]
#[test]
fn group_accessible_cache_data_is_refused() {
    use std::os::unix::fs::PermissionsExt;
    let (_root, storage) = fixture();
    let bytes = driver_bytes(&[3; 4]);
    storage.save(&bytes, checksum(&bytes)).unwrap();
    fs::set_permissions(&storage.path, fs::Permissions::from_mode(0o640)).unwrap();
    assert!(storage.load().is_none());
}
