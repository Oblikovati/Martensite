use super::lifecycle::ChildLifecycle;
use super::{CaseSpec, failure::io_failure, harness::mismatch, spec::HARNESS_PROFILE};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug)]
pub(crate) struct CaseContext {
    pub(crate) directory: PathBuf,
    pub(crate) executable: PathBuf,
    pub(crate) fingerprint: String,
    pub(crate) request: Vec<u8>,
    pub(crate) lifecycle: ChildLifecycle,
}

impl CaseContext {
    #[cfg(test)]
    pub(crate) fn create(root: &Path, spec: CaseSpec<'_>) -> Result<Self, super::CaseFailure> {
        Self::create_for(root, spec, ChildLifecycle::Direct)
    }

    pub(crate) fn create_for(
        root: &Path,
        spec: CaseSpec<'_>,
        lifecycle: ChildLifecycle,
    ) -> Result<Self, super::CaseFailure> {
        fs::create_dir_all(root).map_err(|e| io_failure("create caller artifact root", e))?;
        let directory = exclusive_directory(root)?;
        let context = Self::from_directory(directory, spec, lifecycle)?;
        fs::write(context.directory.join("request.txt"), &context.request)
            .map_err(|e| io_failure("write child request", e))?;
        Ok(context)
    }

    #[cfg(test)]
    pub(crate) fn load(directory: PathBuf, spec: CaseSpec<'_>) -> Result<Self, super::CaseFailure> {
        Self::load_for(directory, spec, ChildLifecycle::Direct)
    }

    pub(crate) fn load_for(
        directory: PathBuf,
        spec: CaseSpec<'_>,
        lifecycle: ChildLifecycle,
    ) -> Result<Self, super::CaseFailure> {
        let context = Self::from_directory(directory, spec, lifecycle)?;
        context.require_record("request.txt")?;
        Ok(context)
    }

    fn from_directory(
        directory: PathBuf,
        spec: CaseSpec<'_>,
        lifecycle: ChildLifecycle,
    ) -> Result<Self, super::CaseFailure> {
        let directory = directory
            .canonicalize()
            .map_err(|e| io_failure("resolve child artifact directory", e))?;
        let executable =
            std::env::current_exe().map_err(|e| io_failure("find actual current executable", e))?;
        let fingerprint = fingerprint(&executable)?;
        let request = request_bytes(spec, &directory, &fingerprint, lifecycle);
        Ok(Self {
            directory,
            executable,
            fingerprint,
            request,
            lifecycle,
        })
    }

    pub(crate) fn claim_entry(&self) -> Result<(), super::CaseFailure> {
        let path = self.directory.join("entry.txt");
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| {
                io_failure(
                    "claim one child entry (existing entry means recursion/reentry)",
                    e,
                )
            })?;
        file.write_all(&self.request)
            .map_err(|e| io_failure("write bound child entry", e))
    }

    pub(crate) fn complete(&self) -> Result<(), super::CaseFailure> {
        self.write_exclusive_record(
            "complete.txt",
            "claim one successful child completion",
            "write bound child completion",
        )
    }

    pub(crate) fn record_prepared(&self) -> Result<(), super::CaseFailure> {
        if self.lifecycle == ChildLifecycle::Direct {
            return Ok(());
        }
        self.write_exclusive_record(
            "prepared.txt",
            "claim one successful child preparation",
            "write bound child preparation",
        )
    }

    fn write_exclusive_record(
        &self,
        name: &str,
        claim: &'static str,
        write: &'static str,
    ) -> Result<(), super::CaseFailure> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(self.directory.join(name))
            .map_err(|e| io_failure(claim, e))?;
        file.write_all(&self.request)
            .map_err(|e| io_failure(write, e))
    }

    pub(crate) fn require_record(&self, name: &str) -> Result<(), super::CaseFailure> {
        let bytes = fs::read(self.directory.join(name))
            .map_err(|e| io_failure("read bound child record", e))?;
        if bytes != self.request {
            return Err(mismatch(
                format!("{name}: {}", hex(&bytes)),
                String::from_utf8_lossy(&self.request),
            ));
        }
        Ok(())
    }

    pub(crate) fn require_same_executable(&self) -> Result<(), super::CaseFailure> {
        let observed = fingerprint(&self.executable)?;
        if observed != self.fingerprint {
            return Err(mismatch(observed, &self.fingerprint));
        }
        Ok(())
    }
}

fn exclusive_directory(root: &Path) -> Result<PathBuf, super::CaseFailure> {
    let stamp = invocation_stamp()?;
    for collision in 0..64 {
        let path = root.join(format!(
            "case-{}-{stamp:x}-{collision:x}",
            std::process::id()
        ));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(io_failure("create exclusive case directory", e)),
        }
    }
    Err(mismatch(
        "64 artifact name collisions",
        "an exclusive invocation directory",
    ))
}

fn invocation_stamp() -> Result<u128, super::CaseFailure> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| mismatch(e.to_string(), "system time at/after Unix epoch"))?
        .as_nanos())
}

fn request_bytes(
    spec: CaseSpec<'_>,
    directory: &Path,
    executable: &str,
    lifecycle: ChildLifecycle,
) -> Vec<u8> {
    let factor = match spec.factor {
        super::FactorProfile::RequireUnset => "unset",
    };
    format!("MARTENSITE_EXACT_CASE_V1\nidentity={}\nseconds={}\nprofile={HARNESS_PROFILE}\nfactor={factor}\nexecutable={executable}\ninvocation={}\n{}",
        hex(spec.identity.as_bytes()), spec.seconds, hex(directory.as_os_str().as_encoded_bytes()), lifecycle.request_suffix()).into_bytes()
}

pub(crate) fn fingerprint(path: &Path) -> Result<String, super::CaseFailure> {
    let bytes = fs::read(path).map_err(|e| io_failure("read executable identity", e))?;
    Ok(hex(&Sha256::digest(bytes)))
}

pub(crate) fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
