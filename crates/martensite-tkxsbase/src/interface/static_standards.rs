//! `InterfaceStatic::Standards`: the standard static values of data exchange (read and
//! write precision, surface curve modes) and the XSTEP messages.
use super::InterfaceStatic;
use crate::xs_message::XSTEP_US;
use martensite_tkernel::{message::MsgFile, standard::Failure, tcollection::AsciiString};
use std::sync::{Mutex, PoisonError};

/// `THE_Interface_Static_deja`: `Standards` runs once per process. The flag is held
/// locked while the statics are defined, so that a concurrent caller waits for them
/// instead of finding the flag set and the statics still missing.
static STANDARDS_DONE: Mutex<bool> = Mutex::new(false);

/// One standard static: name, type letter, init, the '&' edits, and the integer value
/// set after them (`None` for a real).
type StandardStatic = (
    &'static str,
    u8,
    &'static str,
    &'static [&'static str],
    Option<i32>,
);

/// The standard statics, in the native order.
const STANDARD_STATICS: &[StandardStatic] = &[
    (
        "read.precision.mode",
        b'e',
        "",
        &["ematch 0", "eval File", "eval User"],
        Some(0),
    ),
    ("read.precision.val", b'r', "1.e-03", &[], None),
    (
        "read.maxprecision.mode",
        b'e',
        "",
        &["ematch 0", "eval Preferred", "eval Forced"],
        Some(0),
    ),
    ("read.maxprecision.val", b'r', "1.", &[], None),
    ("read.encoderegularity.angle", b'r', "0.01", &[], None),
    (
        "read.surfacecurve.mode",
        b'e',
        "",
        &[
            "ematch -3",
            "eval 3DUse_Forced",
            "eval 2DUse_Forced",
            "eval ?",
            "eval Default",
            "eval ?",
            "eval 2DUse_Preferred",
            "eval 3DUse_Preferred",
        ],
        Some(0),
    ),
    (
        "write.precision.mode",
        b'e',
        "",
        &[
            "ematch -1",
            "eval Min",
            "eval Average",
            "eval Max",
            "eval User",
        ],
        Some(0),
    ),
    ("write.precision.val", b'r', "1.e-03", &[], None),
    (
        "write.surfacecurve.mode",
        b'e',
        "",
        &["ematch 0", "eval Off", "eval On"],
        Some(1),
    ),
];

impl InterfaceStatic {
    /// `Standards`: once per process, the standard statics of the "XSTEP" family, then
    /// the XSTEP messages (from the `MARTENSITE_XS_MESSAGE` directory, else the compiled
    /// `XSTEP.us`); `StandardProgramError` when they are still missing.
    pub fn standards() -> Result<(), Failure> {
        let mut done = STANDARDS_DONE
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if std::mem::replace(&mut *done, true) {
            return Ok(());
        }
        for (name, kind, init, edits, value) in STANDARD_STATICS {
            Self::init_char("XSTEP", name, *kind, init)?;
            for edit in *edits {
                Self::init_char("XSTEP", name, b'&', edit)?;
            }
            if let Some(ival) = value {
                Self::set_i_val(name, *ival)?;
            }
        }
        load_xstep_messages(MsgFile::global())
    }
}

/// The message part of `Standards` on `catalog`.
fn load_xstep_messages(catalog: &MsgFile) -> Result<(), Failure> {
    let key = AsciiString::new("XSTEP_1");
    if catalog.has_msg(&key) {
        return Ok(());
    }
    if !catalog.load_from_env(Some("MARTENSITE_XS_MESSAGE"), "XSTEP", Some(""))? {
        // sizeof(XSMessageXSTEP_us) - 1: the literal without its NUL.
        catalog.load_from_string(XSTEP_US, XSTEP_US.len() as i32)?;
    }
    if catalog.has_msg(&key) {
        return Ok(());
    }
    Err(Failure::with_message(
        "Standard_ProgramError",
        "Critical Error - message resources for InterfaceStatic are invalid or undefined! (no \"XSTEP_1\" message)",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standards_define_the_xstep_statics_and_messages() {
        InterfaceStatic::standards().unwrap();
        InterfaceStatic::standards().unwrap();
        assert_eq!(InterfaceStatic::i_val("write.surfacecurve.mode"), 1);
        assert_eq!(
            InterfaceStatic::c_val("read.surfacecurve.mode").as_bytes(),
            b"Default"
        );
        assert_eq!(InterfaceStatic::r_val("read.precision.val"), 1.0e-3);
        assert_eq!(
            InterfaceStatic::i_def("read.surfacecurve.mode", "estart"),
            -3
        );
        assert_eq!(
            InterfaceStatic::i_def("read.surfacecurve.mode", "ecount"),
            7
        );
        assert_eq!(
            InterfaceStatic::c_def("write.precision.mode", "enum 1"),
            "Max"
        );
        assert!(MsgFile::global().has_msg(&AsciiString::new("XSTEP_1")));
        let catalog = MsgFile::new(std::sync::Arc::new(
            martensite_tkernel::message::NativeMsgFileSource,
        ));
        load_xstep_messages(&catalog).unwrap();
        assert!(catalog.has_msg(&AsciiString::new("XSTEP_278")));
    }
}
