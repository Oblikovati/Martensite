//! The content of an IGES file once split: start lines, global section, directory entries and
//! parameter records.
use super::{IgesDataDirPart, IgesDataDirValues, IgesDataGlobalSection, IgesDataIgesType};
use martensite_tkernel::{
    impl_transient,
    ncollection::HSequence,
    standard::{Failure, Handle, SharedState, StandardTransient, exceptions},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceFileReaderData, InterfaceFileReaderDataBase, InterfaceParamSet,
    InterfaceParamType,
};

/// The start lines or any list of texts of a file section.
pub type IgesDataTextLines = HSequence<Handle<HAsciiString>>;

struct ReaderDataFields {
    thestar: Handle<IgesDataTextLines>,
    theparh: Handle<InterfaceParamSet>,
    thehead: IgesDataGlobalSection,
    thedirs: Vec<IgesDataDirPart>,
    thedefw: f64,
    thechk: Handle<InterfaceCheck>,
}

/// The records of an IGES file being read: one per directory entry.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataIgesReaderData;
/// use martensite_tkxsbase::interface::InterfaceFileReaderData;
/// let data = IgesDataIgesReaderData::new(2, 10);
/// assert_eq!(data.nb_entities(), 2);
/// assert_eq!(data.find_next_record(2), 0);
/// ```
pub struct IgesDataIgesReaderData {
    transient: StandardTransient,
    base: InterfaceFileReaderDataBase,
    state: SharedState<ReaderDataFields>,
}
impl_transient!(
    IgesDataIgesReaderData,
    "IgesDataIgesReaderData",
    dyn InterfaceFileReaderData,
    transient,
    interfaces: [dyn InterfaceFileReaderData]
);

impl IgesDataIgesReaderData {
    /// Room for `nbe` directory entries and `nbp` parameters.
    pub fn new(nbe: i32, nbp: i32) -> Self {
        let places = usize::try_from(nbe + 1).unwrap_or(1);
        Self {
            transient: StandardTransient::default(),
            base: InterfaceFileReaderDataBase::new(nbe, nbp),
            state: SharedState::new(ReaderDataFields {
                thestar: Handle::new(IgesDataTextLines::default()),
                theparh: Handle::new(InterfaceParamSet::new(30, 1)),
                thehead: IgesDataGlobalSection::new(),
                thedirs: vec![IgesDataDirPart::default(); places],
                thedefw: 0.0,
                thechk: Handle::new(InterfaceCheck::new()),
            }),
        }
    }
    /// Adds a start line.
    pub fn add_start_line(&self, aval: impl AsRef<[u8]>) {
        if let Some(lines) = self.state.read().thestar.get() {
            lines.append(Handle::new(HAsciiString::new(aval)));
        }
    }
    /// The start lines.
    pub fn start_section(&self) -> Handle<IgesDataTextLines> {
        self.state.read().thestar.clone()
    }
    /// Adds a parameter of the global section.
    ///
    /// # Errors
    /// A failure of the parameter set.
    pub fn add_global(&self, atype: InterfaceParamType, aval: &[u8]) -> Result<(), Failure> {
        let params = self.state.read().theparh.clone();
        let set = super::iges_live(&params, "add_global")?;
        set.append(aval, aval.len() as i32, atype, 0).map(|_| ())
    }
    /// Sets the global section up from the parameters added.
    ///
    /// # Errors
    /// A failure of the parameter set.
    pub fn set_global_section(&self) -> Result<(), Failure> {
        let (params, check) = {
            let state = self.state.read();
            (state.theparh.clone(), state.thechk.clone())
        };
        let mut head = IgesDataGlobalSection::new();
        head.init(&params, &check)?;
        self.state.write().thehead = head;
        Ok(())
    }
    /// The global section.
    pub fn global_section(&self) -> IgesDataGlobalSection {
        self.state.read().thehead.clone()
    }
    /// Sets directory entry `num`.
    ///
    /// # Errors
    /// An out of range failure for a bad number.
    pub fn set_dir_part(&self, num: i32, values: &IgesDataDirValues) -> Result<(), Failure> {
        let mut state = self.state.write();
        let slot = dir_slot(num, state.thedirs.len())?;
        state.thedirs[slot].init(values);
        Ok(())
    }
    /// Directory entry `num`; entry 0 and any number out of range are empty.
    pub fn dir_part(&self, num: i32) -> IgesDataDirPart {
        let state = self.state.read();
        dir_slot(num, state.thedirs.len())
            .map_or_else(|_| IgesDataDirPart::default(), |slot| state.thedirs[slot])
    }
    /// The fields of directory entry `num`.
    pub fn dir_values(&self, num: i32) -> IgesDataDirValues {
        self.dir_part(num).values()
    }
    /// The type and form of directory entry `num`.
    pub fn dir_type(&self, num: i32) -> IgesDataIgesType {
        self.dir_part(num).dir_type()
    }
    /// Nothing to do: the numbers of the entities are computed when the parameters are stored.
    pub fn set_entity_numbers(&self) {}
    /// The check of the global section.
    pub fn global_check(&self) -> Handle<InterfaceCheck> {
        self.state.read().thechk.clone()
    }
    /// Sets the line weight taken where an entity gives none.
    pub fn set_default_line_weight(&self, defw: f64) {
        self.state.write().thedefw = defw;
    }
    /// The line weight taken where an entity gives none.
    pub fn default_line_weight(&self) -> f64 {
        self.state.read().thedefw
    }
}

fn dir_slot(num: i32, count: usize) -> Result<usize, Failure> {
    usize::try_from(num)
        .ok()
        .filter(|slot| *slot < count)
        .ok_or_else(|| {
            exceptions::out_of_range(&format!(
                "IGES reader data: directory entry {num}; expected 0 to {}",
                count.saturating_sub(1)
            ))
        })
}

impl InterfaceFileReaderData for IgesDataIgesReaderData {
    fn reader_data_base(&self) -> &InterfaceFileReaderDataBase {
        &self.base
    }
    fn nb_entities(&self) -> i32 {
        self.state.read().thedirs.len() as i32 - 1
    }
    fn find_next_record(&self, num: i32) -> i32 {
        if num >= self.state.read().thedirs.len() as i32 - 1 {
            0
        } else {
            num + 1
        }
    }
}
