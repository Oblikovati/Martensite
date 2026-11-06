//! `StepDataStepReaderData`: the literal content of a STEP file (for each record its type,
//! ident and parameters). The header records come first. The reading of parameters is in
//! `reader_read*`, the resolution of references in `reader_numbers`, the strings in
//! `reader_text`. The `set_record` idents: `#n` entity (0 for header or complex
//! member), `$n` sub-list (stored `-2-n`), `SCOPE` -1, `ENDSCOPE` -2.
use martensite_tkernel::{
    impl_transient,
    message::Message,
    ncollection::{DataMap, IndexedMap},
    resource::FormatType,
    standard::{Failure, Handle, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileReaderData, InterfaceFileReaderDataBase,
    InterfaceFileReaderDataParams, InterfaceParamType,
};
use std::fmt::Write as _;

pub(super) struct ReaderState {
    pub(super) theidents: Vec<i32>,
    thetypes: Vec<usize>,
    thenametypes: IndexedMap<AsciiString>,
    themults: DataMap<i32, i32>,
    pub(super) thenbents: i32,
    pub(super) thelastn: i32,
    pub(super) thenbhead: i32,
    pub(super) thenbscop: i32,
}

/// `atoi`: the optional sign and digits at the start of `text`, after blanks, wrapping as
/// the C library does on the supported platforms.
pub(crate) fn c_atoi(text: &[u8]) -> i32 {
    let trimmed = text.trim_ascii_start();
    let (negative, digits) = match trimmed.first() {
        Some(b'-') => (true, &trimmed[1..]),
        Some(b'+') => (false, &trimmed[1..]),
        _ => (false, trimmed),
    };
    let mut value = 0i32;
    for byte in digits.iter().take_while(|b| b.is_ascii_digit()) {
        value = value.wrapping_mul(10).wrapping_add(i32::from(byte - b'0'));
    }
    if negative {
        value.wrapping_neg()
    } else {
        value
    }
}

/// `StepDataStepReaderData`. Built before its [`Handle`], then shared.
///
/// ```
/// use martensite_tkdestep::step_data::StepDataStepReaderData;
/// use martensite_tkernel::resource::FormatType;
/// use martensite_tkxsbase::interface::{InterfaceFileReaderData, InterfaceFileReaderDataParams, InterfaceParamType};
/// let data = StepDataStepReaderData::new(0, 1, 1, FormatType::Utf8);
/// data.set_record(1, b"#7", b"POINT", 1);
/// data.add_step_param(1, b"1.5", InterfaceParamType::ParamReal, 0).unwrap();
/// data.init_params(1).unwrap();
/// assert_eq!(data.record_ident(1), 7);
/// assert_eq!(data.nb_params(1).unwrap(), 1);
/// assert_eq!(data.find_next_record(0), 1);
/// ```
pub struct StepDataStepReaderData {
    transient: StandardTransient,
    base: InterfaceFileReaderDataBase,
    pub(super) state: SharedState<ReaderState>,
    pub(super) thecheck: Handle<InterfaceCheck>,
    pub(super) my_source_code_page: FormatType,
}
impl_transient!(
    StepDataStepReaderData,
    "StepDataStepReaderData",
    dyn InterfaceFileReaderData,
    transient,
    interfaces: [dyn InterfaceFileReaderData]
);

impl InterfaceFileReaderData for StepDataStepReaderData {
    fn reader_data_base(&self) -> &InterfaceFileReaderDataBase {
        &self.base
    }
    fn nb_entities(&self) -> i32 {
        self.state.read().thenbents
    }
    fn find_next_record(&self, num: i32) -> i32 {
        if num < 0 {
            return 0;
        }
        let state = self.state.read();
        let first = if num == 0 {
            state.thenbhead + 1
        } else {
            num + 1
        };
        (first..state.theidents.len() as i32)
            .find(|rank| state.theidents[*rank as usize] > 0)
            .unwrap_or(0)
    }
}

impl StepDataStepReaderData {
    /// `StepDataStepReaderData(nbheader, nbtotal, nbpar, theSourceCodePage = UTF8)`.
    pub fn new(nbheader: i32, nbtotal: i32, nbpar: i32, source_code_page: FormatType) -> Self {
        let places = nbtotal.max(0) as usize + 1;
        Self {
            transient: StandardTransient::default(),
            base: InterfaceFileReaderDataBase::new(nbtotal, nbpar),
            state: SharedState::new(ReaderState {
                theidents: vec![0; places],
                thetypes: vec![0; places],
                thenametypes: IndexedMap::default(),
                themults: DataMap::default(),
                thenbents: 0,
                thelastn: 0,
                thenbhead: nbheader,
                thenbscop: 0,
            }),
            thecheck: Handle::new(InterfaceCheck::new()),
            my_source_code_page: source_code_page,
        }
    }
    /// `set_record(num, ident, type, nbpar)`. Complex members (ident `#0` after the header)
    /// chain to the previous component, which is expected in alphabetical order: a warning
    /// is recorded otherwise.
    pub fn set_record(&self, num: i32, ident: &[u8], record_type: &[u8], _nbpar: i32) {
        let slot = num as usize;
        let mut state = self.state.write();
        if record_type.first() != Some(&b'(') {
            state.thenbents += 1;
        }
        let type_rank = state.thenametypes.add(AsciiString::from_bytes(record_type));
        state.thetypes[slot] = type_rank;
        match ident.first() {
            Some(b'$') => {
                let numlst = c_atoi(&ident[1..]);
                if state.thelastn < numlst {
                    state.thelastn = numlst;
                }
                state.theidents[slot] = -2 - numlst;
            }
            Some(b'#') => {
                let numlst = c_atoi(&ident[1..]);
                state.theidents[slot] = numlst;
                if numlst == 0 && num > state.thenbhead {
                    drop(state);
                    self.chain_complex(num);
                }
            }
            _ if ident == b"SCOPE" => {
                state.theidents[slot] = -1;
                state.thenbscop += 1;
            }
            _ if ident == b"ENDSCOPE" => state.theidents[slot] = -2,
            _ => {}
        }
    }
    /// Links complex member `num` to its previous component and checks their order.
    fn chain_complex(&self, num: i32) {
        let mut state = self.state.write();
        let head = state.thenbhead;
        let Some(prev) = (head + 1..num)
            .rev()
            .find(|rank| state.theidents[*rank as usize] >= 0)
        else {
            return;
        };
        state.themults.bind(prev, num);
        let names = [state.thetypes[prev as usize], state.thetypes[num as usize]];
        let (Ok(before), Ok(after)) = (
            state.thenametypes.find_key(names[0]),
            state.thenametypes.find_key(names[1]),
        ) else {
            return;
        };
        if after.as_bytes() >= before.as_bytes() {
            return;
        }
        let error = format!(
            "Complex Type incorrect : {} / {} ... ",
            String::from_utf8_lossy(before.as_bytes()),
            String::from_utf8_lossy(after.as_bytes())
        );
        let ident = (1..=prev)
            .rev()
            .map(|rank| state.theidents[rank as usize])
            .find(|id| *id > 0);
        let total = state.theidents.len() as i32 - 1;
        drop(state);
        let mut sout = Message::send_trace();
        let _ = write!(
            sout,
            "  ***  Incorrect record {num} (on {total} -> {} % in File)  ***",
            i64::from(num) * 100 / i64::from(total.max(1))
        );
        if let Some(ident) = ident {
            let _ = write!(sout, "  Ident #{ident}");
        }
        let _ = writeln!(sout, "\n{error}");
        self.thecheck
            .add_warning_text(&error, "Complex Type incorrect : ");
    }
    /// `add_step_param(num, aval, atype, nument = 0)`: resolves the number of a sub-list
    /// (`$n`) or an ident (`#n`) from its text.
    pub fn add_step_param(
        &self,
        num: i32,
        aval: &[u8],
        atype: InterfaceParamType,
        nument: i32,
    ) -> Result<(), Failure> {
        let number = match atype {
            InterfaceParamType::ParamSub | InterfaceParamType::ParamIdent => {
                c_atoi(aval.get(1..).unwrap_or_default())
            }
            _ => nument,
        };
        self.add_param(num, aval, atype, number)
    }
    /// `record_type(num)`.
    pub fn record_type(&self, num: i32) -> AsciiString {
        let state = self.state.read();
        usize::try_from(num)
            .ok()
            .and_then(|rank| state.thetypes.get(rank))
            .and_then(|rank| state.thenametypes.find_key(*rank).ok().cloned())
            .unwrap_or_default()
    }
    /// `CType(num)`.
    pub fn c_type(&self, num: i32) -> String {
        String::from_utf8_lossy(self.record_type(num).as_bytes()).into_owned()
    }
    /// `record_ident(num)`.
    pub fn record_ident(&self, num: i32) -> i32 {
        let state = self.state.read();
        usize::try_from(num)
            .ok()
            .and_then(|rank| state.theidents.get(rank).copied())
            .unwrap_or(0)
    }
    /// `sub_list_number(num, nump, aslast)`: the sub-list a parameter designates, 0 when it
    /// is not one (or not the last parameter, with `aslast`).
    pub fn sub_list_number(&self, num: i32, nump: i32, aslast: bool) -> Result<i32, Failure> {
        let nbp = self.nb_params(num)?;
        if nump == 0 || nump > nbp {
            return Ok(0);
        }
        let param = self.param(num, nump)?;
        if param.param_type() != InterfaceParamType::ParamSub || (aslast && nump != nbp) {
            return Ok(0);
        }
        Ok(param.entity_number())
    }
    /// `is_complex(num)`.
    pub fn is_complex(&self, num: i32) -> bool {
        self.state.read().themults.is_bound(&num)
    }
    /// `complex_type(num, types)`: the types of its components; nothing for a sub-list or a
    /// scope mark.
    pub fn complex_type(&self, num: i32, types: &mut Vec<AsciiString>) {
        if self.record_ident(num) < 0 {
            return;
        }
        let mut rank = num;
        while rank > 0 {
            types.push(self.record_type(rank));
            rank = self.next_for_complex(rank);
        }
    }
    /// `next_for_complex(num)`: 0 at the end.
    pub fn next_for_complex(&self, num: i32) -> i32 {
        self.state.read().themults.find(&num).copied().unwrap_or(0)
    }
    /// `named_for_complex(name, num0, num, ach)`: the next component named `name`. Returns
    /// whether it was in alphabetical order; a component out of order warns and restarts
    /// from `num0`, a missing one fails and gives `num` = 0.
    pub fn named_for_complex(
        &self,
        name: &str,
        num0: i32,
        num: &mut i32,
        ach: &Handle<InterfaceCheck>,
    ) -> bool {
        let mut n = if *num <= 0 {
            num0
        } else {
            self.next_for_complex(*num)
        };
        let matches_name = |rank: i32| self.record_type(rank).as_bytes() == name.as_bytes();
        if n != 0 && matches_name(n) {
            *num = n;
            return true;
        }
        if n == 0 {
            // Recursion "rewinding": n stays 0 on the found path and its messages
            // go to the same check, kept for the same warnings.
            self.named_for_complex(name, num0, &mut n, ach);
        }
        self.scan_complex(num0, num, ach, name, &matches_name)
    }
    /// `named_for_complex(name, short_name, num0, num, ach)`.
    pub fn named_for_complex_short(
        &self,
        name: &str,
        short_name: &str,
        num0: i32,
        num: &mut i32,
        ach: &Handle<InterfaceCheck>,
    ) -> bool {
        let n = if *num <= 0 {
            num0
        } else {
            self.next_for_complex(*num)
        };
        let matches_name = |rank: i32| {
            let found = self.record_type(rank);
            found.as_bytes() == name.as_bytes() || found.as_bytes() == short_name.as_bytes()
        };
        if n != 0 && matches_name(n) {
            *num = n;
            return true;
        }
        self.scan_complex(num0, num, ach, name, &matches_name)
    }
    fn scan_complex(
        &self,
        num0: i32,
        num: &mut i32,
        ach: &Handle<InterfaceCheck>,
        name: &str,
        matches_name: &dyn Fn(i32) -> bool,
    ) -> bool {
        let mut rank = num0;
        while rank > 0 {
            if matches_name(rank) {
                *num = rank;
                ach.add_warning_text(
                    &format!(
                        "Complex Record n0.{num0}, member type {name} not in alphabetic order"
                    ),
                    "Complex Record n0.%d, member type %s not in alphabetic order",
                );
                return false;
            }
            rank = self.next_for_complex(rank);
        }
        *num = 0;
        ach.add_fail_text(
            &format!("Complex Record n0.{num0}, member type {name} not found"),
            "Complex Record n0.%d, member type %s not found",
        );
        false
    }
    /// `check_nb_params(num, nbreq, ach, mess = "")`.
    pub fn check_nb_params(
        &self,
        num: i32,
        nbreq: i32,
        ach: &Handle<InterfaceCheck>,
        mess: &str,
    ) -> Result<bool, Failure> {
        if self.nb_params(num)? == nbreq {
            return Ok(true);
        }
        if mess.is_empty() {
            ach.add_fail_text(
                &format!("Count of Parameters is not {nbreq}"),
                "Count of Parameters is not %d",
            );
        } else {
            ach.add_fail_text(
                &format!("Count of Parameters is not {nbreq} for {mess}"),
                "Count of Parameters is not %d for %s",
            );
        }
        Ok(false)
    }
    /// `find_next_header_record(num)`.
    pub fn find_next_header_record(&self, num: i32) -> i32 {
        if num < 0 {
            return 0;
        }
        let state = self.state.read();
        (num + 1..=state.thenbhead)
            .find(|rank| state.theidents[*rank as usize] >= 0)
            .unwrap_or(0)
    }
    /// `prepare_header`: nothing, `set_entity_numbers` already resolved the header sub-lists.
    pub fn prepare_header(&self) {}
    /// `global_check`.
    pub fn global_check(&self) -> Handle<InterfaceCheck> {
        self.thecheck.clone()
    }
}
