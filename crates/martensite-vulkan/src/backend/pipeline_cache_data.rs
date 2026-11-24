//! Durable, bounded envelopes protect driver-produced cache data from partial writes.
use ash::vk;
use std::{
    env, fs,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const FILE_MAGIC: [u8; 8] = *b"MARTVK01";
const ENVELOPE_BYTES: usize = 24;
const HEADER_BYTES: usize = 32;
const MAX_CACHE_BYTES: usize = 16 << 20;

#[derive(Clone, Copy)]
pub(in crate::backend) struct CacheIdentity {
    vendor: u32,
    device: u32,
    uuid: [u8; vk::UUID_SIZE],
}

impl CacheIdentity {
    /// Select the compatibility fields from the actual Vulkan physical device.
    pub(in crate::backend) fn from_properties(properties: &vk::PhysicalDeviceProperties) -> Self {
        Self {
            vendor: properties.vendor_id,
            device: properties.device_id,
            uuid: properties.pipeline_cache_uuid,
        }
    }

    fn header(&self) -> [u8; HEADER_BYTES] {
        let fields = [
            HEADER_BYTES as u32,
            vk::PipelineCacheHeaderVersion::ONE.as_raw() as u32,
            self.vendor,
            self.device,
        ];
        let mut header = [0; HEADER_BYTES];
        header[..16].copy_from_slice(fields.map(u32::to_le_bytes).as_flattened());
        header[16..].copy_from_slice(&self.uuid);
        header
    }

    fn file_name(&self) -> String {
        format!("pipeline-{:08x}-{:08x}.bin", self.vendor, self.device)
    }
}

pub(in crate::backend) struct CacheStorage {
    path: PathBuf,
    identity: CacheIdentity,
}

pub(super) struct LoadedCache {
    pub(super) bytes: Vec<u8>,
    pub(super) checksum: u64,
}

impl CacheStorage {
    /// Use the per-user OS cache folder for this physical device when available.
    pub(in crate::backend) fn platform(identity: CacheIdentity) -> Option<Self> {
        if cfg!(test) {
            // Device tests explicitly inject a temporary path for persistence checks.
            return None;
        }
        let root = os_cache_folder()?.join("martensite").join("vulkan");
        Some(Self::at(root.join(identity.file_name()), identity))
    }

    pub(super) fn at(path: PathBuf, identity: CacheIdentity) -> Self {
        Self { path, identity }
    }

    pub(super) fn compatible(&self, bytes: &[u8]) -> bool {
        valid_size(bytes.len()) && bytes.starts_with(&self.identity.header())
    }

    pub(super) fn load(&self) -> Option<LoadedCache> {
        if !fs::symlink_metadata(&self.path).ok()?.is_file() || !private_parent(&self.path) {
            return None;
        }
        let mut file = File::open(&self.path).ok()?;
        let (length, expected) = read_envelope(&mut file)?;
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(length).ok()?;
        bytes.resize(length, 0);
        file.read_exact(&mut bytes).ok()?;
        if !self.compatible(&bytes) || checksum(&bytes) != expected {
            return None;
        }
        (file.read(&mut [0]).ok()? == 0).then_some(LoadedCache {
            bytes,
            checksum: expected,
        })
    }

    pub(super) fn save(&self, bytes: &[u8], checksum: u64) -> io::Result<()> {
        let parent = self.path.parent().ok_or(io::ErrorKind::InvalidInput)?;
        create_cache_folder(parent)?;
        if !private_parent(&self.path) {
            return Err(io::ErrorKind::PermissionDenied.into());
        }
        let staging = OwnedCacheStage::create(parent)?;
        let mut file = create_cache_file(&staging.data_path())?;
        write_envelope(&mut file, bytes, checksum)?;
        file.sync_all()?;
        drop(file);
        fs::rename(staging.data_path(), &self.path)
    }
}

fn create_cache_folder(folder: &Path) -> io::Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    std::os::unix::fs::DirBuilderExt::mode(&mut builder, 0o700);
    builder.create(folder)
}

fn create_cache_file(path: &Path) -> io::Result<File> {
    let mut options = fs::OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    options.open(path)
}

fn private_parent(path: &Path) -> bool {
    let Some(metadata) = path
        .parent()
        .and_then(|parent| fs::symlink_metadata(parent).ok())
    else {
        return false;
    };
    metadata.is_dir() && owner_access_only(&metadata)
}

fn owner_access_only(metadata: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        metadata.mode() & 0o077 == 0
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        true
    }
}

fn os_cache_folder() -> Option<PathBuf> {
    if cfg!(windows) {
        return absolute_environment_path("LOCALAPPDATA");
    }
    if cfg!(target_os = "macos") {
        return home_folder().map(|home| home.join("Library").join("Caches"));
    }
    absolute_environment_path("XDG_CACHE_HOME")
        .or_else(|| home_folder().map(|home| home.join(".cache")))
}

fn absolute_environment_path(name: &str) -> Option<PathBuf> {
    env::var_os(name)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
}

fn home_folder() -> Option<PathBuf> {
    env::home_dir().filter(|path| path.is_absolute())
}

pub(super) fn valid_size(length: usize) -> bool {
    (HEADER_BYTES..=MAX_CACHE_BYTES).contains(&length)
}

fn read_envelope(file: &mut File) -> Option<(usize, u64)> {
    let metadata = file.metadata().ok()?;
    if !metadata.is_file()
        || !owner_access_only(&metadata)
        || metadata.len() > (MAX_CACHE_BYTES + ENVELOPE_BYTES) as u64
    {
        return None;
    }
    let mut envelope = [[0; 8]; 3];
    file.read_exact(envelope.as_flattened_mut()).ok()?;
    let [magic, size, checksum] = envelope;
    let length = usize::try_from(u64::from_le_bytes(size)).ok()?;
    if magic != FILE_MAGIC
        || !valid_size(length)
        || metadata.len() != (ENVELOPE_BYTES + length) as u64
    {
        return None;
    }
    Some((length, u64::from_le_bytes(checksum)))
}

fn write_envelope(file: &mut File, bytes: &[u8], checksum: u64) -> io::Result<()> {
    let envelope = [
        FILE_MAGIC,
        (bytes.len() as u64).to_le_bytes(),
        checksum.to_le_bytes(),
    ];
    file.write_all(envelope.as_flattened())?;
    file.write_all(bytes)
}

/// The stable FNV-1a checksum detects accidental payload corruption.
pub(super) fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

struct OwnedCacheStage(PathBuf);

impl OwnedCacheStage {
    fn create(parent: &Path) -> io::Result<Self> {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?;
        let prefix = format!(
            ".pipeline-stage-{}-{}",
            std::process::id(),
            stamp.as_nanos()
        );
        for number in 0..8 {
            let path = parent.join(format!("{prefix}-{number}"));
            match martensite_tkernel::osd::create_private_document_directory(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        Err(io::ErrorKind::AlreadyExists.into())
    }

    fn data_path(&self) -> PathBuf {
        self.0.join("data")
    }
}

impl Drop for OwnedCacheStage {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.data_path());
        let _ = fs::remove_dir(&self.0);
    }
}

#[cfg(test)]
#[path = "pipeline_cache_data_tests.rs"]
mod tests;
