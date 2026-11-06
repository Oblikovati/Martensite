//! `STEPCAFControlExternFile`: the data resulting from the translation of an external file.
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, SharedState, StandardTransient},
    tcollection::HAsciiString,
};
use martensite_tklcaf::tdf::TdfLabel as Label;
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

#[derive(Clone)]
struct ExternFileState {
    ws: Handle<XSControlWorkSession>,
    load_status: IFSelectReturnStatus,
    transfer_status: bool,
    write_status: IFSelectReturnStatus,
    name: Handle<HAsciiString>,
    label: Label,
}

/// ```
/// use martensite_tkdestep::stepcaf_control::STEPCAFControlExternFile;
/// use martensite_tkxsbase::if_select::IFSelectReturnStatus;
/// let file = STEPCAFControlExternFile::new();
/// assert!(file.get_load_status() == IFSelectReturnStatus::RetVoid);
/// assert!(!file.get_transfer_status() && file.get_ws().is_null());
/// file.set_transfer_status(true);
/// assert!(file.get_transfer_status());
/// ```
pub struct STEPCAFControlExternFile {
    transient: StandardTransient,
    state: SharedState<ExternFileState>,
}
impl_transient!(
    STEPCAFControlExternFile,
    "STEPCAFControlExternFile",
    StandardTransient,
    transient
);

impl Default for STEPCAFControlExternFile {
    fn default() -> Self {
        Self {
            transient: StandardTransient::default(),
            state: SharedState::new(ExternFileState {
                ws: Handle::default(),
                load_status: IFSelectReturnStatus::RetVoid,
                transfer_status: false,
                write_status: IFSelectReturnStatus::RetVoid,
                name: Handle::default(),
                label: Label::default(),
            }),
        }
    }
}

impl STEPCAFControlExternFile {
    /// `STEPCAFControlExternFile()`: an empty structure.
    pub fn new() -> Self {
        Self::default()
    }
    /// `set_ws`.
    pub fn set_ws(&self, ws: &Handle<XSControlWorkSession>) {
        self.state.write().ws = ws.clone();
    }
    /// `get_ws`.
    pub fn get_ws(&self) -> Handle<XSControlWorkSession> {
        self.state.read().ws.clone()
    }
    /// `set_load_status`.
    pub fn set_load_status(&self, stat: IFSelectReturnStatus) {
        self.state.write().load_status = stat;
    }
    /// `get_load_status`.
    pub fn get_load_status(&self) -> IFSelectReturnStatus {
        self.state.read().load_status
    }
    /// `set_transfer_status`.
    pub fn set_transfer_status(&self, isok: bool) {
        self.state.write().transfer_status = isok;
    }
    /// `get_transfer_status`.
    pub fn get_transfer_status(&self) -> bool {
        self.state.read().transfer_status
    }
    /// `set_write_status`.
    pub fn set_write_status(&self, stat: IFSelectReturnStatus) {
        self.state.write().write_status = stat;
    }
    /// `get_write_status`.
    pub fn get_write_status(&self) -> IFSelectReturnStatus {
        self.state.read().write_status
    }
    /// `set_name`.
    pub fn set_name(&self, name: &Handle<HAsciiString>) {
        self.state.write().name = name.clone();
    }
    /// `get_name`.
    pub fn get_name(&self) -> Handle<HAsciiString> {
        self.state.read().name.clone()
    }
    /// `set_label`.
    pub fn set_label(&self, label: &Label) {
        self.state.write().label = label.clone();
    }
    /// `get_label`.
    pub fn get_label(&self) -> Label {
        self.state.read().label.clone()
    }
}
