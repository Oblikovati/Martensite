//! Named filesystem fixture models only the released runner's finalized receipt boundary.
use martensite_process_policy::case_runner::CaseReceipt;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

/// Own the fake finalized files; `FakeFinalizedChildReceipt::new("stdout", "stderr")`.
pub(super) struct FakeFinalizedChildReceipt {
    directory: PathBuf,
}

impl FakeFinalizedChildReceipt {
    /// Construct retained streams without changing parent environment; `Self::new("", "")`.
    pub(super) fn new(stdout: &str, stderr: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "tkbool-report-fixture-{}-{nonce}",
            std::process::id()
        ));
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("receipt.json"), "{}").unwrap();
        std::fs::write(directory.join("stdout.log"), stdout).unwrap();
        std::fs::write(directory.join("stderr.log"), stderr).unwrap();
        Self { directory }
    }

    /// Supply the concrete released runner receipt shape; `fixture.receipt()`.
    pub(super) fn receipt(&self) -> CaseReceipt {
        CaseReceipt {
            directory: self.directory.clone(),
        }
    }

    /// Remove finalization to model child state; `fixture.remove_finalization()`.
    pub(super) fn remove_finalization(&self) {
        let marker = self.directory.join("receipt.json");
        std::fs::remove_file(marker).unwrap();
    }
}

impl Drop for FakeFinalizedChildReceipt {
    fn drop(&mut self) {
        // Each fixture owns its unique directory and every retained fake stream.
        std::fs::remove_dir_all(&self.directory).unwrap();
    }
}
