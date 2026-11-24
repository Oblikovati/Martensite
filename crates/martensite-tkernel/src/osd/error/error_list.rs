//! The portable `ERR_*` codes returned by `OsdError::error`.
//!
//! The values are fixed; `error_list::ERR_NOENT == 9`.
/// `ERR_SURPRISE`.
pub const ERR_SURPRISE: i32 = -2;
/// `ERR_UNKNOWN`.
pub const ERR_UNKNOWN: i32 = -1;
/// `ERR_ACCESS`.
pub const ERR_ACCESS: i32 = 1;
/// `ERR_EXIST`.
pub(crate) const ERR_EXIST: i32 = 2;
/// `ERR_FAULT`.
pub(crate) const ERR_FAULT: i32 = 3;
/// `ERR_INTR`.
#[cfg(unix)]
pub(crate) const ERR_INTR: i32 = 4;
/// `ERR_INVAL`.
pub const ERR_INVAL: i32 = 5;
/// `ERR_IO`.
pub(crate) const ERR_IO: i32 = 6;
/// `ERR_ISDIR`.
#[cfg(unix)]
pub(crate) const ERR_ISDIR: i32 = 7;
/// `ERR_NAMETOOLONG`.
pub(crate) const ERR_NAMETOOLONG: i32 = 8;
/// `ERR_NOENT`.
pub const ERR_NOENT: i32 = 9;
/// `ERR_NOMEM`.
pub const ERR_NOMEM: i32 = 10;
/// `ERR_NOTDIR`.
#[cfg(unix)]
pub(crate) const ERR_NOTDIR: i32 = 11;
/// `ERR_PERM`.
#[cfg(unix)]
pub(crate) const ERR_PERM: i32 = 12;
/// `ERR_QUOT`.
pub(crate) const ERR_QUOT: i32 = 13;
/// `ERR_ROFS`.
#[cfg(unix)]
pub(crate) const ERR_ROFS: i32 = 15;
/// `ERR_TOOBIG`.
pub(crate) const ERR_TOOBIG: i32 = 16;
/// `ERR_DMLINK`.
#[cfg(unix)]
pub(crate) const ERR_DMLINK: i32 = 17;
/// `ERR_DNOENT`.
#[cfg(unix)]
pub(crate) const ERR_DNOENT: i32 = 18;
/// `ERR_FAGAIN`.
pub(crate) const ERR_FAGAIN: i32 = 19;
/// `ERR_FBADF`.
pub const ERR_FBADF: i32 = 20;
/// `ERR_FEXIST`.
#[cfg(unix)]
pub(crate) const ERR_FEXIST: i32 = 23;
/// `ERR_FFBIG`.
#[cfg(unix)]
pub(crate) const ERR_FFBIG: i32 = 24;
/// `ERR_FINVAL`.
pub(crate) const ERR_FINVAL: i32 = 25;
/// `ERR_FIO`.
pub(crate) const ERR_FIO: i32 = 26;
/// `ERR_FLOCKED`.
pub(crate) const ERR_FLOCKED: i32 = 27;
/// `ERR_FMFILE`.
pub(crate) const ERR_FMFILE: i32 = 28;
/// `ERR_FNOLCK`.
#[cfg(unix)]
pub(crate) const ERR_FNOLCK: i32 = 29;
/// `ERR_FPERM`.
#[cfg(unix)]
pub(crate) const ERR_FPERM: i32 = 30;
/// `ERR_FRANGE`.
#[cfg(unix)]
pub(crate) const ERR_FRANGE: i32 = 31;
/// `ERR_FWFD`.
#[cfg(unix)]
pub(crate) const ERR_FWFD: i32 = 32;
/// `ERR_FNBUSY`.
#[cfg(unix)]
pub(crate) const ERR_FNBUSY: i32 = 33;
/// `ERR_FNFILE`.
#[cfg(unix)]
pub(crate) const ERR_FNFILE: i32 = 34;
/// `ERR_FNINVAL`.
pub(crate) const ERR_FNINVAL: i32 = 35;
/// `ERR_FNOSPC`.
pub(crate) const ERR_FNOSPC: i32 = 36;
/// `ERR_FNNOTEMPTY`.
pub const ERR_FNNOTEMPTY: i32 = 37;
/// `ERR_FNXDEV`.
pub(crate) const ERR_FNXDEV: i32 = 38;
/// `ERR_PPERM`.
#[cfg(unix)]
pub(crate) const ERR_PPERM: i32 = 50;
