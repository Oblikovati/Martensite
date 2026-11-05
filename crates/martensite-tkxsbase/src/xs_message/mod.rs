//! The message texts of the data exchange toolkits, carried byte for byte and embedded here.
//! `InterfaceStaticStandards` loads the English text into the message catalog.

/// `XSMessageXSTEP_us`: the English message texts, the bytes of the literal without its
/// terminating NUL (`sizeof(XSMessageXSTEP_us) - 1` natively).
///
/// ```
/// use martensite_tkxsbase::xs_message::XSTEP_US;
/// let text = std::str::from_utf8(XSTEP_US).unwrap();
/// assert!(text.starts_with("!LOADING PHASE\n.XSTEP_1\n"));
/// ```
pub const XSTEP_US: &[u8] = include_bytes!("XSTEP.us");

/// `XSMessageXSTEP_fr`: the French message texts. The file ships with the package but is
/// not compiled natively; no member reads it.
///
/// ```
/// use martensite_tkxsbase::xs_message::XSTEP_FR;
/// assert!(XSTEP_FR.len() > 1000);
/// ```
pub const XSTEP_FR: &[u8] = include_bytes!("XSTEP.fr");
