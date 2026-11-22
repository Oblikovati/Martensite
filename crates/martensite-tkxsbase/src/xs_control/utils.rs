//! `XSControlUtils`: conversions between the texts, lists and arrays the exchange
//! commands handle as transients. The shape conversions are in `utils_shapes`.
//!
//! The native static buffers of the returned `const char*` and `const char16_t*` are
//! owned values here; the null `char16_t*` is an empty vector.
use crate::interface::InterfaceMsg;
use martensite_tkernel::{
    message::Message,
    ncollection::{HArray1, HSequence},
    standard::{Failure, Handle, RuntimeType, Transient, exceptions},
    tcollection::{AsciiString, ExtendedString, HAsciiString, HExtendedString},
};
use std::fmt::Write as _;

/// A list of texts (`HSequence<Handle<HAsciiString>>`).
pub type XSControlTextSeq = HSequence<Handle<HAsciiString>>;
/// A list of extended texts.
pub type XSControlExtTextSeq = HSequence<Handle<HExtendedString>>;
/// A list of transients.
pub type XSControlTransientSeq = HSequence<Handle<dyn Transient>>;

/// `XSControlUtils`.
///
/// ```
/// use martensite_tkxsbase::xs_control::XSControlUtils;
/// let xu = XSControlUtils::new();
/// assert_eq!(xu.extended_to_ascii(&[0x41, 0xC1]).as_bytes(), b"AA");
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct XSControlUtils;

/// The text of an `AsciiString`.
fn text(s: &AsciiString) -> String {
    String::from_utf8_lossy(s.as_bytes()).into_owned()
}

/// The item `num` (1-based) of a list, `None` out of range.
fn nth<T: Clone + Send + Sync + 'static>(list: &HSequence<T>, num: i32) -> Option<T> {
    let index = usize::try_from(num).ok().filter(|n| *n >= 1)?;
    list.sequence(|s| s.value(index).ok().cloned())
}

/// The length of a list.
fn len<T: Send + Sync + 'static>(list: &HSequence<T>) -> i32 {
    list.sequence(|s| s.length())
}

/// The lines of a list of texts (the null texts skipped), `None` when `lines` is not a
/// list of texts.
fn line_texts(lines: &Handle<dyn Transient>) -> Option<Vec<String>> {
    if let Some(l) = lines.downcast_ref::<XSControlTextSeq>() {
        return Some(l.sequence(|s| {
            s.iter()
                .filter_map(|h| h.get().map(|t| text(&t.string())))
                .collect()
        }));
    }
    if let Some(l) = lines.downcast_ref::<HSequence<AsciiString>>() {
        return Some(l.sequence(|s| s.iter().map(text).collect()));
    }
    if let Some(l) = lines.downcast_ref::<XSControlExtTextSeq>() {
        return Some(l.sequence(|s| {
            s.iter()
                .filter_map(|h| h.get().map(|t| t.string().to_utf8()))
                .collect()
        }));
    }
    lines
        .downcast_ref::<HSequence<ExtendedString>>()
        .map(|l| l.sequence(|s| s.iter().map(ExtendedString::to_utf8).collect()))
}

impl XSControlUtils {
    /// `XSControlUtils()`.
    pub fn new() -> Self {
        Self
    }
    /// `TraceLine(line)`: a line on the information messenger.
    pub fn trace_line(&self, line: &str) {
        let _ = writeln!(Message::send_info(), "{line}");
    }
    /// `TraceLines(lines)`: the lines of a list of texts (one per line), or a text alone
    /// (without end of line).
    pub fn trace_lines(&self, lines: &Handle<dyn Transient>) {
        let mut sout = Message::send_info();
        if let Some(texts) = line_texts(lines) {
            for line in texts {
                let _ = writeln!(sout, "{line}");
            }
            return;
        }
        if let Some(t) = lines.downcast_ref::<HAsciiString>() {
            let _ = write!(sout, "{}", text(&t.string()));
        }
        if let Some(t) = lines.downcast_ref::<HExtendedString>() {
            let _ = write!(sout, "{}", t.string().to_utf8());
        }
    }
    /// `IsKind(item, what)`: false for a null item or type.
    pub fn is_kind(&self, item: &Handle<dyn Transient>, what: Option<&RuntimeType>) -> bool {
        what.is_some_and(|w| item.is_kind(w))
    }
    /// Returns the dynamic type name of an entity; `utils.type_name(&entity, true)`.
    pub fn type_name(&self, item: &Handle<dyn Transient>, nopk: bool) -> String {
        item.dynamic_type()
            .map(|typ| self.type_name_from_descriptor(typ, nopk))
            .unwrap_or_default()
    }

    /// Formats a static type name, optionally omitting its package; `utils.type_name_from_descriptor(descriptor, true)`.
    pub fn type_name_from_descriptor(&self, descriptor: &RuntimeType, nopk: bool) -> String {
        let name = descriptor.name();
        if nopk && let Some((_, local)) = name.split_once('_') {
            return local.to_owned();
        }
        name.to_owned()
    }
    /// `TraValue(seqval, num)`: the item `num` of a list of texts or transients; null
    /// otherwise.
    pub fn tra_value(&self, seqval: &Handle<dyn Transient>, num: i32) -> Handle<dyn Transient> {
        if num < 1 {
            return Handle::default();
        }
        if let Some(seqs) = seqval.downcast_ref::<XSControlTextSeq>() {
            return nth(seqs, num).map(|h| h.cast()).unwrap_or_default();
        }
        if let Some(seqt) = seqval.downcast_ref::<XSControlTransientSeq>() {
            return nth(seqt, num).unwrap_or_default();
        }
        Handle::default()
    }
    /// `NewSeqTra`: a new empty list of transients.
    pub fn new_seq_tra(&self) -> Handle<XSControlTransientSeq> {
        Handle::new(HSequence::default())
    }
    /// `AppendTra(seqval, traval)`: `StandardNullObject` for a null list.
    pub fn append_tra(
        &self,
        seqval: &Handle<XSControlTransientSeq>,
        traval: &Handle<dyn Transient>,
    ) -> Result<(), Failure> {
        let list = seqval
            .get()
            .ok_or_else(|| exceptions::null_object("XSControlUtils::AppendTra: null list"))?;
        list.append(traval.clone());
        Ok(())
    }
    /// `DateString(yy, mm, dd, hh, mn, ss)`: the date as `InterfaceMsg::TDate`
    /// writes it.
    pub fn date_string(
        &self,
        yy: i32,
        mm: i32,
        dd: i32,
        hh: i32,
        mn: i32,
        ss: i32,
    ) -> Result<AsciiString, Failure> {
        let mut ladate = Vec::new();
        InterfaceMsg::t_date(&mut ladate, yy, mm, dd, hh, mn, ss, "")?;
        Ok(AsciiString::new(&String::from_utf8_lossy(&ladate)))
    }
    /// `DateValues(text, yy, mm, dd, hh, mn, ss)`: the fields read by
    /// `InterfaceMsg::NDate`, in order.
    pub fn date_values(&self, text: &str) -> [i32; 6] {
        let mut fields = [0; 6];
        InterfaceMsg::n_date(text, &mut fields);
        fields
    }
    /// `ToCString(strval)`: the text of a handle text (empty for null).
    pub fn to_c_string(&self, strval: &Handle<HAsciiString>) -> AsciiString {
        strval.get().map(|s| s.string().clone()).unwrap_or_default()
    }
    /// `ToCString(strval)` of an `AsciiString`.
    pub fn to_c_string_ascii(&self, strval: &AsciiString) -> AsciiString {
        strval.clone()
    }
    /// `ToHString(strcon)`: a new handle text.
    pub fn to_h_string(&self, strcon: &str) -> Handle<HAsciiString> {
        Handle::new(HAsciiString::new(strcon))
    }
    /// `ToAString(strcon)`.
    pub fn to_a_string(&self, strcon: &str) -> AsciiString {
        AsciiString::new(strcon)
    }
    /// `ToEString(strval)`: the units of a handle extended text (none for null).
    pub fn to_e_string(&self, strval: &Handle<HExtendedString>) -> Vec<u16> {
        strval
            .get()
            .map(|s| s.string().as_utf16().to_vec())
            .unwrap_or_default()
    }
    /// `ToEString(strval)` of an `ExtendedString`.
    pub fn to_e_string_ext(&self, strval: &ExtendedString) -> Vec<u16> {
        strval.as_utf16().to_vec()
    }
    /// `ToHString(strcon)` of extended units.
    pub fn to_h_string_ext(&self, strcon: &[u16]) -> Handle<HExtendedString> {
        Handle::new(HExtendedString::from_utf16(strcon))
    }
    /// `ToXString(strcon)`.
    pub fn to_x_string(&self, strcon: &[u16]) -> ExtendedString {
        ExtendedString::from_utf16(strcon)
    }
    /// `AsciiToExtended(str)`: each byte as one unit.
    pub fn ascii_to_extended(&self, s: &str) -> Vec<u16> {
        ExtendedString::from_c_string(s.as_bytes(), false)
            .as_utf16()
            .to_vec()
    }
    /// `IsAscii(str)`.
    pub fn is_ascii(&self, s: &[u16]) -> bool {
        ExtendedString::from_utf16(s).is_ascii()
    }
    /// `ExtendedToAscii(str)`: each unit reduced to its 7 low bits.
    pub fn extended_to_ascii(&self, s: &[u16]) -> AsciiString {
        let bytes: Vec<u8> = s.iter().map(|u| (u & 127) as u8).collect();
        AsciiString::new(&String::from_utf8_lossy(&bytes))
    }
    /// `CStrValue(list, num)`: the text `num` of a list of texts (extended texts reduced
    /// to ASCII), or the text itself; empty otherwise or beyond the list.
    /// `StandardOutOfRange` for a number below 1 of a list (a native out of range).
    pub fn c_str_value(
        &self,
        list: &Handle<dyn Transient>,
        num: i32,
    ) -> Result<AsciiString, Failure> {
        let out_of_range = || {
            exceptions::out_of_range(&format!(
                "XSControlUtils::CStrValue: {num}; expected at least 1"
            ))
        };
        if let Some(l) = list.downcast_ref::<XSControlTextSeq>() {
            return if num > len(l) {
                Ok(AsciiString::default())
            } else {
                nth(l, num)
                    .map(|h| self.to_c_string(&h))
                    .ok_or_else(out_of_range)
            };
        }
        if let Some(l) = list.downcast_ref::<HSequence<AsciiString>>() {
            return if num > len(l) {
                Ok(AsciiString::default())
            } else {
                nth(l, num).ok_or_else(out_of_range)
            };
        }
        if let Some(l) = list.downcast_ref::<XSControlExtTextSeq>() {
            return if num > len(l) {
                Ok(AsciiString::default())
            } else {
                nth(l, num)
                    .map(|h| self.extended_to_ascii(&self.to_e_string(&h)))
                    .ok_or_else(out_of_range)
            };
        }
        if let Some(l) = list.downcast_ref::<HSequence<ExtendedString>>() {
            return if num > len(l) {
                Ok(AsciiString::default())
            } else {
                nth(l, num)
                    .map(|e| self.extended_to_ascii(e.as_utf16()))
                    .ok_or_else(out_of_range)
            };
        }
        if let Some(t) = list.downcast_ref::<HAsciiString>() {
            return Ok(t.string().clone());
        }
        if let Some(t) = list.downcast_ref::<HExtendedString>() {
            return Ok(self.extended_to_ascii(t.string().as_utf16()));
        }
        Ok(AsciiString::default())
    }
    /// `EStrValue(list, num)`: as `CStrValue`, in extended units. Native defect kept: a
    /// list of `AsciiString` gives no units (the value computed is not returned).
    pub fn e_str_value(&self, list: &Handle<dyn Transient>, num: i32) -> Result<Vec<u16>, Failure> {
        let out_of_range = || {
            exceptions::out_of_range(&format!(
                "XSControlUtils::EStrValue: {num}; expected at least 1"
            ))
        };
        if let Some(l) = list.downcast_ref::<XSControlTextSeq>() {
            return if num > len(l) {
                Ok(Vec::new())
            } else {
                nth(l, num)
                    .map(|h| self.ascii_to_extended(&text(&self.to_c_string(&h))))
                    .ok_or_else(out_of_range)
            };
        }
        if let Some(l) = list.downcast_ref::<XSControlExtTextSeq>() {
            return if num > len(l) {
                Ok(Vec::new())
            } else {
                nth(l, num)
                    .map(|h| self.to_e_string(&h))
                    .ok_or_else(out_of_range)
            };
        }
        if let Some(l) = list.downcast_ref::<HSequence<ExtendedString>>() {
            return if num > len(l) {
                Ok(Vec::new())
            } else {
                nth(l, num)
                    .map(|e| e.as_utf16().to_vec())
                    .ok_or_else(out_of_range)
            };
        }
        if let Some(t) = list.downcast_ref::<HAsciiString>() {
            return Ok(self.ascii_to_extended(&text(&t.string())));
        }
        if let Some(t) = list.downcast_ref::<HExtendedString>() {
            return Ok(t.string().as_utf16().to_vec());
        }
        Ok(Vec::new())
    }
    /// `NewSeqCStr`.
    pub fn new_seq_c_str(&self) -> Handle<XSControlTextSeq> {
        Handle::new(HSequence::default())
    }
    /// `AppendCStr(seqval, strval)`: `StandardNullObject` for a null list.
    pub fn append_c_str(
        &self,
        seqval: &Handle<XSControlTextSeq>,
        strval: &str,
    ) -> Result<(), Failure> {
        let list = seqval
            .get()
            .ok_or_else(|| exceptions::null_object("XSControlUtils::AppendCStr: null list"))?;
        list.append(self.to_h_string(strval));
        Ok(())
    }
    /// `NewSeqEStr`.
    pub fn new_seq_e_str(&self) -> Handle<XSControlExtTextSeq> {
        Handle::new(HSequence::default())
    }
    /// `AppendEStr(seqval, strval)`: `StandardNullObject` for a null list.
    pub fn append_e_str(
        &self,
        seqval: &Handle<XSControlExtTextSeq>,
        strval: &[u16],
    ) -> Result<(), Failure> {
        let list = seqval
            .get()
            .ok_or_else(|| exceptions::null_object("XSControlUtils::AppendEStr: null list"))?;
        list.append(self.to_h_string_ext(strval));
        Ok(())
    }
    /// `SeqLength(list)`: the length of a list of texts, transients, shapes or integers
    /// (0 otherwise).
    pub fn seq_length(&self, list: &Handle<dyn Transient>) -> i32 {
        if let Some(l) = list.downcast_ref::<XSControlTextSeq>() {
            return len(l);
        }
        if let Some(l) = list.downcast_ref::<HSequence<AsciiString>>() {
            return len(l);
        }
        if let Some(l) = list.downcast_ref::<XSControlExtTextSeq>() {
            return len(l);
        }
        if let Some(l) = list.downcast_ref::<XSControlTransientSeq>() {
            return len(l);
        }
        if let Some(l) = list.downcast_ref::<HSequence<martensite_tkbrep::topods::Shape>>() {
            return len(l);
        }
        list.downcast_ref::<HSequence<i32>>().map_or(0, len)
    }
    /// `SeqToArr(seq, first = 1)`: a list of texts or transients as an array of bounds
    /// `first..length - first + 1`, item `i` at `i - first + 1`. Native quirk kept: for
    /// `first > 1` the first items fall below the bounds (`StandardOutOfRange`);
    /// `StandardTypeMismatch` for another object; null for null.
    pub fn seq_to_arr(
        &self,
        seq: &Handle<dyn Transient>,
        first: i32,
    ) -> Result<Handle<dyn Transient>, Failure> {
        if seq.is_null() {
            return Ok(Handle::default());
        }
        if let Some(seqs) = seq.downcast_ref::<XSControlTextSeq>() {
            return Ok(Handle::new(to_array(seqs, first)?).into_upcast());
        }
        if let Some(seqt) = seq.downcast_ref::<XSControlTransientSeq>() {
            return Ok(Handle::new(to_array(seqt, first)?).into_upcast());
        }
        Err(exceptions::type_mismatch(
            "XSControlUtils::SeqToArr: expected a list of texts or transients",
        ))
    }
    /// `ArrToSeq(arr)`: an array of texts or transients as a list; `StandardTypeMismatch`
    /// for another object; null for null.
    pub fn arr_to_seq(
        &self,
        arr: &Handle<dyn Transient>,
    ) -> Result<Handle<dyn Transient>, Failure> {
        if arr.is_null() {
            return Ok(Handle::default());
        }
        if let Some(arrs) = arr.downcast_ref::<HArray1<Handle<HAsciiString>>>() {
            return Ok(Handle::new(to_sequence(arrs)).into_upcast());
        }
        if let Some(arrt) = arr.downcast_ref::<HArray1<Handle<dyn Transient>>>() {
            return Ok(Handle::new(to_sequence(arrt)).into_upcast());
        }
        Err(exceptions::type_mismatch(
            "XSControlUtils::ArrToSeq: expected an array of texts or transients",
        ))
    }
    /// `SeqIntValue(list, num)`: 0 for a null list; `StandardOutOfRange` out of it.
    pub fn seq_int_value(&self, list: &Handle<HSequence<i32>>, num: i32) -> Result<i32, Failure> {
        let Some(l) = list.get() else {
            return Ok(0);
        };
        let length = len(l);
        nth(l, num).ok_or_else(|| {
            exceptions::out_of_range(&format!(
                "XSControlUtils::SeqIntValue: {num}; expected [1, {length}]"
            ))
        })
    }
}

/// The array of `SeqToArr`.
fn to_array<T: Clone + Default + Send + Sync + 'static>(
    list: &HSequence<T>,
    first: i32,
) -> Result<HArray1<T>, Failure> {
    let lng = len(list);
    let arr = HArray1::with_bounds(first, lng - first + 1).map_err(|e| {
        exceptions::range_error(&format!(
            "XSControlUtils::SeqToArr: bounds {first}..{}: {e:?}",
            lng - first + 1
        ))
    })?;
    for i in 1..=lng {
        let value = nth(list, i).unwrap_or_default();
        arr.change_array1()
            .set_value(i - first + 1, value)
            .map_err(|e| {
                exceptions::out_of_range(&format!(
                    "XSControlUtils::SeqToArr: item {i} at {}: {e:?}",
                    i - first + 1
                ))
            })?;
    }
    Ok(arr)
}

/// The list of `ArrToSeq`.
fn to_sequence<T: Clone + Send + Sync + 'static>(arr: &HArray1<T>) -> HSequence<T> {
    let seq = HSequence::default();
    let array = arr.array1();
    for i in array.lower()..=array.upper() {
        if let Ok(v) = array.value(i) {
            seq.append(v.clone());
        }
    }
    seq
}
