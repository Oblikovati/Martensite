//! `HeaderSection`: the three entities of the header of a STEP file and their protocol.
use crate::step_data::{StepDataProtocolOps, StepDataProtocolTables, StepDataUndefinedEntity};
use martensite_tkernel::{
    impl_transient,
    ncollection::HArray1,
    standard::{
        Failure, Handle, RuntimeType, RuntimeTyped, SharedState, StandardTransient, exceptions,
    },
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceProtocol};
use std::sync::OnceLock;

/// `Handle<HArray1<Handle<HAsciiString>>>`.
pub type HeaderSectionTextList = Handle<HArray1<Handle<HAsciiString>>>;

/// The count of a list, 0 when null.
fn list_length(list: &HeaderSectionTextList) -> i32 {
    list.get().map_or(0, |live| live.array1().len() as i32)
}

/// A value of a list; an exception on a null list or out of its bounds.
fn list_value(list: &HeaderSectionTextList, num: i32) -> Result<Handle<HAsciiString>, Failure> {
    let live = list.get().ok_or_else(|| {
        exceptions::null_object(&format!(
            "HeaderSection: value {num} of a null list; expected the list to be set"
        ))
    })?;
    let array = live.array1();
    let value = array
        .value(num)
        .map_err(|error| error.into_failure("HeaderSection"))?;
    Ok(value.clone())
}

/// The members of a text field; `text_field!(thename, set_name, name, "Name")`.
macro_rules! text_field {
    ($field:ident, $setter:ident, $getter:ident, $recorded:literal) => {
        #[doc = concat!("`Set", $recorded, "`.")]
        pub fn $setter(&self, value: &Handle<HAsciiString>) {
            self.state.write().$field = value.clone();
        }
        #[doc = concat!("`", $recorded, "`.")]
        pub fn $getter(&self) -> Handle<HAsciiString> {
            self.state.read().$field.clone()
        }
    };
}

/// The members of a list of texts, with its value and its count.
macro_rules! text_list_field {
    ($field:ident, $setter:ident, $getter:ident, $value:ident, $count:ident, $recorded:literal) => {
        #[doc = concat!("`Set", $recorded, "`.")]
        pub fn $setter(&self, value: &HeaderSectionTextList) {
            self.state.write().$field = value.clone();
        }
        #[doc = concat!("`", $recorded, "`.")]
        pub fn $getter(&self) -> HeaderSectionTextList {
            self.state.read().$field.clone()
        }
        #[doc = concat!("`", $recorded, "Value(num)`.")]
        pub fn $value(&self, num: i32) -> Result<Handle<HAsciiString>, Failure> {
            list_value(&self.state.read().$field, num)
        }
        #[doc = concat!("`Nb", $recorded, "`: 0 when the list is null.")]
        pub fn $count(&self) -> i32 {
            list_length(&self.state.read().$field)
        }
    };
}

#[derive(Default)]
struct FileNameState {
    name: Handle<HAsciiString>,
    time_stamp: Handle<HAsciiString>,
    author: HeaderSectionTextList,
    organization: HeaderSectionTextList,
    preprocessor_version: Handle<HAsciiString>,
    originating_system: Handle<HAsciiString>,
    authorisation: Handle<HAsciiString>,
}

/// ```
/// use martensite_tkdestep::header_section::HeaderSectionFileName;
/// let name = HeaderSectionFileName::default();
/// assert_eq!(name.nb_author(), 0);
/// assert!(name.name().is_null());
/// ```
#[derive(Default)]
pub struct HeaderSectionFileName {
    transient: StandardTransient,
    state: SharedState<FileNameState>,
}
impl_transient!(
    HeaderSectionFileName,
    "HeaderSectionFileName",
    StandardTransient,
    transient
);

impl HeaderSectionFileName {
    /// `Init(aName, aTimeStamp, aAuthor, aOrganization, aPreprocessorVersion,
    /// aOriginatingSystem, aAuthorisation)`.
    #[allow(clippy::too_many_arguments)]
    pub fn init(
        &self,
        name: &Handle<HAsciiString>,
        time_stamp: &Handle<HAsciiString>,
        author: &HeaderSectionTextList,
        organization: &HeaderSectionTextList,
        preprocessor_version: &Handle<HAsciiString>,
        originating_system: &Handle<HAsciiString>,
        authorisation: &Handle<HAsciiString>,
    ) {
        *self.state.write() = FileNameState {
            name: name.clone(),
            time_stamp: time_stamp.clone(),
            author: author.clone(),
            organization: organization.clone(),
            preprocessor_version: preprocessor_version.clone(),
            originating_system: originating_system.clone(),
            authorisation: authorisation.clone(),
        };
    }
    text_field!(name, set_name, name, "Name");
    text_field!(time_stamp, set_time_stamp, time_stamp, "TimeStamp");
    text_list_field!(
        author,
        set_author,
        author,
        author_value,
        nb_author,
        "Author"
    );
    text_list_field!(
        organization,
        set_organization,
        organization,
        organization_value,
        nb_organization,
        "Organization"
    );
    text_field!(
        preprocessor_version,
        set_preprocessor_version,
        preprocessor_version,
        "PreprocessorVersion"
    );
    text_field!(
        originating_system,
        set_originating_system,
        originating_system,
        "OriginatingSystem"
    );
    text_field!(
        authorisation,
        set_authorisation,
        authorisation,
        "Authorisation"
    );
}

#[derive(Default)]
struct FileSchemaState {
    schema_identifiers: HeaderSectionTextList,
}

/// ```
/// use martensite_tkdestep::header_section::HeaderSectionFileSchema;
/// assert_eq!(HeaderSectionFileSchema::default().nb_schema_identifiers(), 0);
/// ```
#[derive(Default)]
pub struct HeaderSectionFileSchema {
    transient: StandardTransient,
    state: SharedState<FileSchemaState>,
}
impl_transient!(
    HeaderSectionFileSchema,
    "HeaderSectionFileSchema",
    StandardTransient,
    transient
);

impl HeaderSectionFileSchema {
    /// `init(schema_identifiers)`.
    pub fn init(&self, schema_identifiers: &HeaderSectionTextList) {
        self.set_schema_identifiers(schema_identifiers);
    }
    text_list_field!(
        schema_identifiers,
        set_schema_identifiers,
        schema_identifiers,
        schema_identifiers_value,
        nb_schema_identifiers,
        "SchemaIdentifiers"
    );
}

#[derive(Default)]
struct FileDescriptionState {
    description: HeaderSectionTextList,
    implementation_level: Handle<HAsciiString>,
}

/// ```
/// use martensite_tkdestep::header_section::HeaderSectionFileDescription;
/// assert_eq!(HeaderSectionFileDescription::default().nb_description(), 0);
/// ```
#[derive(Default)]
pub struct HeaderSectionFileDescription {
    transient: StandardTransient,
    state: SharedState<FileDescriptionState>,
}
impl_transient!(
    HeaderSectionFileDescription,
    "HeaderSectionFileDescription",
    StandardTransient,
    transient
);

impl HeaderSectionFileDescription {
    /// `init(description, implementation_level)`.
    pub fn init(
        &self,
        description: &HeaderSectionTextList,
        implementation_level: &Handle<HAsciiString>,
    ) {
        self.set_description(description);
        self.set_implementation_level(implementation_level);
    }
    text_list_field!(
        description,
        set_description,
        description,
        description_value,
        nb_description,
        "Description"
    );
    text_field!(
        implementation_level,
        set_implementation_level,
        implementation_level,
        "ImplementationLevel"
    );
}

/// `HeaderSectionProtocol`: FileName 1, FileDescription 2, FileSchema 3, undefined 4.
///
/// ```
/// use martensite_tkdestep::header_section::{HeaderSectionFileSchema, HeaderSectionProtocol};
/// use martensite_tkernel::standard::RuntimeTyped;
/// use martensite_tkxsbase::interface::InterfaceProtocol;
/// assert_eq!(HeaderSectionProtocol::default().type_number(HeaderSectionFileSchema::static_type()), 3);
/// ```
#[derive(Default)]
pub struct HeaderSectionProtocol {
    transient: StandardTransient,
    tables: StepDataProtocolTables,
}
impl_transient!(
    HeaderSectionProtocol,
    "HeaderSectionProtocol",
    dyn StepDataProtocolOps,
    transient,
    interfaces: [dyn InterfaceProtocol, dyn StepDataProtocolOps]
);
crate::impl_step_protocol!(HeaderSectionProtocol);

impl StepDataProtocolOps for HeaderSectionProtocol {
    fn protocol_tables(&self) -> &StepDataProtocolTables {
        &self.tables
    }
    fn step_type_number(&self, atype: &RuntimeType) -> i32 {
        let kinds = [
            HeaderSectionFileName::static_type(),
            HeaderSectionFileDescription::static_type(),
            HeaderSectionFileSchema::static_type(),
            StepDataUndefinedEntity::static_type(),
        ];
        kinds
            .iter()
            .position(|kind| std::ptr::eq(*kind, atype))
            .map_or(0, |rank| rank as i32 + 1)
    }
    fn schema_name(&self, _model: &Handle<dyn InterfaceInterfaceModel>) -> String {
        "header_section".to_owned()
    }
}

/// `HeaderSection`.
pub struct HeaderSection;

impl HeaderSection {
    /// `Protocol`: the shared protocol of the header.
    pub fn protocol() -> Handle<HeaderSectionProtocol> {
        static PROTOCOL: OnceLock<Handle<HeaderSectionProtocol>> = OnceLock::new();
        PROTOCOL
            .get_or_init(|| Handle::new(HeaderSectionProtocol::default()))
            .clone()
    }
}
