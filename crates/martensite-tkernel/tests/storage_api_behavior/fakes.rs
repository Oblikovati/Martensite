//! Fixed inputs for schema clock and type migration tests.
#[path = "scripted_driver.rs"]
mod scripted_driver;
use martensite_tkernel::storage::{CreationDateSource, MigrationFile, MigrationSource};
pub use scripted_driver::ScriptedDriver;

/// A clock stuck on 2 March 2001.
pub struct FixedClock;

impl CreationDateSource for FixedClock {
    fn local_date(&self) -> (u32, u32, i32) {
        (3, 2, 2001)
    }
}

/// A migration source serving fixed table bytes.
pub struct TableFile(pub &'static [u8]);

impl MigrationSource for TableFile {
    fn migration_file(&self) -> MigrationFile {
        MigrationFile::Contents(self.0.to_vec())
    }
}

/// A migration source whose table path exists but cannot be read.
pub struct UnreadableTable;

impl MigrationSource for UnreadableTable {
    fn migration_file(&self) -> MigrationFile {
        MigrationFile::Unreadable
    }
}
