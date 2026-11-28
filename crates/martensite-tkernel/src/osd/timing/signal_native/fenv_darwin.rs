//! Darwin floating-point trap control through `fegetenv`/`fesetenv` (libSystem).
//!
//! Darwin has no `feenableexcept`, so the trap bits are edited in the `fenv_t` image:
//! on x86_64 a set x87 control-word or MXCSR mask bit *masks* the exception, so enabling
//! clears it; on arm64 the FPCR trap-enable bits sit eight places above the `FE_*` flags.
//! Apple Silicon cores implement those FPCR bits as read-as-zero, so there a request to
//! enable is accepted and [`to_catch_floating_signals`] truthfully reads back `false`.
//! Pending exception flags are cleared first, as glibc's `feclearexcept` path does, so
//! enabling a trap never fires on a flag raised before the call.
#![allow(unsafe_code)]

/// The `<fenv.h>` image of the calling thread's floating-point environment (x86_64).
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct DarwinFenv {
    /// `__control`: the x87 control word; a set low bit masks that exception.
    pub control: u16,
    /// `__status`: the x87 status word; its low bits are the raised flags.
    pub status: u16,
    /// `__mxcsr`: SSE flags in bits 0-5, their mask bits in bits 7-12.
    pub mxcsr: u32,
    reserved: [u8; 8],
}

/// The `<fenv.h>` image of the calling thread's floating-point environment (arm64).
#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct DarwinFenv {
    /// `__fpsr`: the raised exception flags in its low bits.
    pub fpsr: u64,
    /// `__fpcr`: trap-enable bits `FE_* << 8`.
    pub fpcr: u64,
}

/// `_OSD_FPX`: FE_INVALID | FE_DIVBYZERO | FE_OVERFLOW in Darwin's x86 numbering.
#[cfg(target_arch = "x86_64")]
const OSD_FPX: u32 = 0x01 | 0x04 | 0x08;
/// `_OSD_FPX`: FE_INVALID | FE_DIVBYZERO | FE_OVERFLOW in Darwin's arm64 numbering.
#[cfg(target_arch = "aarch64")]
const OSD_FPX: u64 = 0x01 | 0x02 | 0x04;
/// `FE_ALL_EXCEPT` on Darwin x86 (includes the x87 denormal-operand flag).
#[cfg(target_arch = "x86_64")]
const FE_ALL_EXCEPT: u32 = 0x3f;
/// `FE_ALL_EXCEPT` on Darwin arm64 (includes `FE_FLUSHTOZERO`'s flag bit 7).
#[cfg(target_arch = "aarch64")]
const FE_ALL_EXCEPT: u64 = 0x9f;
/// The MXCSR mask bits sit seven places above their flags.
#[cfg(target_arch = "x86_64")]
const MXCSR_MASK_SHIFT: u32 = 7;
/// The FPCR trap-enable bits sit eight places above their flags.
#[cfg(target_arch = "aarch64")]
const FPCR_TRAP_SHIFT: u32 = 8;

unsafe extern "C" {
    fn fegetenv(env: *mut DarwinFenv) -> i32;
    fn fesetenv(env: *const DarwinFenv) -> i32;
}

/// The environment with pending flags cleared and the OSD traps enabled or masked;
/// `with_traps(env, true)`.
#[cfg(target_arch = "x86_64")]
pub(super) fn with_traps(mut env: DarwinFenv, enable: bool) -> DarwinFenv {
    env.status &= !(FE_ALL_EXCEPT as u16);
    env.mxcsr &= !FE_ALL_EXCEPT;
    let x87_masks = OSD_FPX as u16;
    let sse_masks = OSD_FPX << MXCSR_MASK_SHIFT;
    if enable {
        env.control &= !x87_masks;
        env.mxcsr &= !sse_masks;
    } else {
        env.control |= x87_masks;
        env.mxcsr |= sse_masks;
    }
    env
}

/// The environment with pending flags cleared and the OSD traps enabled or disabled;
/// `with_traps(env, true)`.
#[cfg(target_arch = "aarch64")]
pub(super) fn with_traps(mut env: DarwinFenv, enable: bool) -> DarwinFenv {
    env.fpsr &= !FE_ALL_EXCEPT;
    let enables = OSD_FPX << FPCR_TRAP_SHIFT;
    if enable {
        env.fpcr |= enables;
    } else {
        env.fpcr &= !enables;
    }
    env
}

/// Whether any OSD trap is unmasked in the SSE unit, which carries Rust's `f64`
/// arithmetic on x86_64; `traps_enabled(env)`.
#[cfg(target_arch = "x86_64")]
pub(super) fn traps_enabled(env: DarwinFenv) -> bool {
    (!env.mxcsr >> MXCSR_MASK_SHIFT) & OSD_FPX != 0
}

/// Whether any OSD trap-enable bit reads back set in FPCR; `traps_enabled(env)`.
#[cfg(target_arch = "aarch64")]
pub(super) fn traps_enabled(env: DarwinFenv) -> bool {
    (env.fpcr >> FPCR_TRAP_SHIFT) & OSD_FPX != 0
}

/// The calling thread's environment, `None` when `fegetenv` refuses; `current()`.
fn current() -> Option<DarwinFenv> {
    let mut env = DarwinFenv::default();
    // SAFETY: `env` is a writable value with the SDK's `fenv_t` layout for this architecture.
    let status = unsafe { fegetenv(&mut env) };
    (status == 0).then_some(env)
}

/// Enables or masks the OSD traps for the calling thread; `set_floating_signal(false)`.
pub(crate) fn set_floating_signal(enable: bool) {
    let Some(env) = current() else { return };
    let updated = with_traps(env, enable);
    // SAFETY: `updated` is a readable `fenv_t` image derived from the thread's own environment.
    // A refusal leaves the previous environment, which `to_catch_floating_signals` reports.
    let _ = unsafe { fesetenv(&updated) };
}

/// Whether the calling thread traps the OSD exceptions; `to_catch_floating_signals()`.
pub(crate) fn to_catch_floating_signals() -> bool {
    current().is_some_and(traps_enabled)
}

#[cfg(test)]
#[path = "fenv_darwin_tests.rs"]
mod tests;
