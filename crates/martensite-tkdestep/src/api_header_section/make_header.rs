//! `APIHeaderSectionMakeHeader`: builds or edits the three header entities of a model.
//! What the file says of its author is Martensite: the default model names, description,
//! author, organization, authorisation, originating system and preprocessor version.
use crate::header_section::{
    HeaderSectionFileDescription, HeaderSectionFileName, HeaderSectionFileSchema,
    HeaderSectionTextList,
};
use crate::step_data::{StepDataProtocolOps, StepDataStepModel};
use martensite_tkernel::{
    ncollection::HArray1,
    standard::{Failure, Handle, RuntimeTyped},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceEntityIterator, InterfaceInterfaceModel, InterfaceModelEntities, InterfaceMsg,
    InterfaceProtocol,
};

/// `XSTEP_SYSTEM_VERSION`: the identity of the writing system.
fn system_version() -> String {
    format!("Martensite {}", env!("CARGO_PKG_VERSION"))
}

/// `XSTEP_PROCESSOR_VERSION`, for STEP.
fn processor_version() -> String {
    format!("Martensite STEP processor {}", env!("CARGO_PKG_VERSION"))
}

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

fn array_failure(error: martensite_tkernel::ncollection::ArrayError) -> Failure {
    error.into_failure("APIHeaderSectionMakeHeader")
}

/// A list of one text.
fn one_text(value: &str) -> Result<HeaderSectionTextList, Failure> {
    let list = HArray1::<Handle<HAsciiString>>::with_bounds(1, 1).map_err(array_failure)?;
    list.change_array1()
        .set_value(1, text(value))
        .map_err(array_failure)?;
    Ok(Handle::new(list))
}

/// Sets a value of a list, when in its bounds.
fn set_list_value(
    list: &HeaderSectionTextList,
    num: i32,
    value: &Handle<HAsciiString>,
) -> Result<(), Failure> {
    let Some(live) = list.get() else {
        return Ok(());
    };
    let mut array = live.change_array1();
    if num >= array.lower() && num <= array.upper() {
        array.set_value(num, value.clone()).map_err(array_failure)?;
    }
    Ok(())
}

/// The header accessors of a text of the file name: nothing when it has none.
macro_rules! file_name_text {
    ($setter:ident, $getter:ident, $recorded:literal) => {
        #[doc = concat!("`Set", $recorded, "`: nothing without a file name.")]
        pub fn $setter(&self, value: &Handle<HAsciiString>) {
            if let Some(live) = self.fn_value.get() {
                live.$setter(value);
            }
        }
        #[doc = concat!("`", $recorded, "`: null without a file name.")]
        pub fn $getter(&self) -> Handle<HAsciiString> {
            self.fn_value
                .get()
                .map_or_else(Handle::default, |live| live.$getter())
        }
    };
}

/// The header accessors of a list of an entity.
macro_rules! entity_list {
    ($entity:ident, $setter:ident, $set_value:ident, $getter:ident, $value:ident, $count:ident, $recorded:literal) => {
        #[doc = concat!("`Set", $recorded, "`.")]
        pub fn $setter(&self, value: &HeaderSectionTextList) {
            if let Some(live) = self.$entity.get() {
                live.$setter(value);
            }
        }
        #[doc = concat!("`Set", $recorded, "Value(num, value)`: nothing out of the bounds.")]
        pub fn $set_value(&self, num: i32, value: &Handle<HAsciiString>) -> Result<(), Failure> {
            set_list_value(&self.$getter(), num, value)
        }
        #[doc = concat!("`", $recorded, "`.")]
        pub fn $getter(&self) -> HeaderSectionTextList {
            self.$entity
                .get()
                .map_or_else(Handle::default, |live| live.$getter())
        }
        #[doc = concat!("`", $recorded, "Value(num)`.")]
        pub fn $value(&self, num: i32) -> Result<Handle<HAsciiString>, Failure> {
            self.$entity
                .get()
                .map_or_else(|| Ok(Handle::default()), |live| live.$value(num))
        }
        #[doc = concat!("`Nb", $recorded, "`.")]
        pub fn $count(&self) -> i32 {
            self.$entity.get().map_or(0, |live| live.$count())
        }
    };
}

/// ```
/// use martensite_tkdestep::api_header_section::APIHeaderSectionMakeHeader;
/// let header = APIHeaderSectionMakeHeader::new(0).unwrap();
/// assert!(header.is_done());
/// let name = header.name();
/// assert_eq!(name.get().unwrap().string().as_bytes(), b"Martensite Shape Model");
/// ```
#[derive(Default)]
pub struct APIHeaderSectionMakeHeader {
    done: bool,
    fn_value: Handle<HeaderSectionFileName>,
    fs_value: Handle<HeaderSectionFileSchema>,
    fd_value: Handle<HeaderSectionFileDescription>,
}

impl APIHeaderSectionMakeHeader {
    /// `APIHeaderSectionMakeHeader(shapetype = 0)`: a new header named after the type.
    pub fn new(shapetype: i32) -> Result<Self, Failure> {
        let mut header = Self::default();
        header.init(match shapetype {
            1 => "Martensite Facetted BRep Model",
            2 => "Martensite Face Based Surface Model",
            3 => "Martensite Shell Based Surface Model",
            4 => "Martensite Manifold Solid Brep Model",
            _ => "Martensite Shape Model",
        })?;
        Ok(header)
    }
    /// `APIHeaderSectionMakeHeader(model)`: from the header of a model; `is_done` tells
    /// whether all three entities were there.
    pub fn from_model(model: &StepDataStepModel) -> Result<Self, Failure> {
        let mut header = Self {
            done: true,
            ..Self::default()
        };
        if model.has_header_entity(HeaderSectionFileName::static_type()) {
            header.fn_value = model
                .header_entity(HeaderSectionFileName::static_type())?
                .cast();
        } else {
            header.done = false;
        }
        if model.has_header_entity(HeaderSectionFileSchema::static_type()) {
            header.fs_value = model
                .header_entity(HeaderSectionFileSchema::static_type())?
                .cast();
        } else {
            header.done = false;
        }
        if model.has_header_entity(HeaderSectionFileDescription::static_type()) {
            header.fd_value = model
                .header_entity(HeaderSectionFileDescription::static_type())?
                .cast();
        } else {
            header.done = false;
        }
        Ok(header)
    }
    /// `init(nameval)`: cancels the former definition and gives a file name; the time
    /// stamp is the current date.
    pub fn init(&mut self, nameval: &str) -> Result<(), Failure> {
        self.done = true;
        if self.fn_value.is_null() {
            self.fn_value = Handle::new(HeaderSectionFileName::default());
        }
        let mut timestamp = Vec::new();
        InterfaceMsg::t_date(
            &mut timestamp,
            0,
            0,
            0,
            0,
            0,
            1,
            "C:%4.4d-%2.2d-%2.2dT%2.2d:%2.2d:%2.2d",
        )?;
        if let Some(live) = self.fn_value.get() {
            live.set_name(&text(nameval));
            live.set_time_stamp(&Handle::new(HAsciiString::new(&timestamp)));
            live.set_author(&one_text("Martensite")?);
            live.set_organization(&one_text("Martensite")?);
            live.set_preprocessor_version(&text(&processor_version()));
            live.set_originating_system(&text(&system_version()));
            live.set_authorisation(&text("Martensite"));
        }
        if self.fd_value.is_null() {
            self.fd_value = Handle::new(HeaderSectionFileDescription::default());
        }
        if let Some(live) = self.fd_value.get() {
            live.set_description(&one_text("Martensite Model")?);
            live.set_implementation_level(&text("2;1"));
        }
        if self.fs_value.is_null() {
            self.fs_value = Handle::new(HeaderSectionFileSchema::default());
        }
        if let Some(live) = self.fs_value.get() {
            live.set_schema_identifiers(&one_text("")?);
        }
        Ok(())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.done
    }
    /// `apply(model)`: gives the model the entities it has not, the schema name from its
    /// protocol when not defined.
    pub fn apply(&self, model: &StepDataStepModel) -> Result<(), Failure> {
        let mut header: InterfaceEntityIterator = model.header();
        if self.has_fd() && !model.has_header_entity(HeaderSectionFileDescription::static_type()) {
            header.add_item(&self.fd_value.cast());
        }
        if self.has_fn() && !model.has_header_entity(HeaderSectionFileName::static_type()) {
            header.add_item(&self.fn_value.cast());
        }
        if self.has_fs() && !model.has_header_entity(HeaderSectionFileSchema::static_type()) {
            self.complete_schema(model)?;
            header.add_item(&self.fs_value.cast());
        }
        model.clear_header();
        for entity in header.content() {
            model.add_header_entity(&entity)?;
        }
        Ok(())
    }
    /// The schema name is defined, or taken from the protocol of the model.
    fn complete_schema(&self, model: &StepDataStepModel) -> Result<(), Failure> {
        let mut schid = self.schema_identifiers();
        let mut sch = Handle::<HAsciiString>::default();
        if let Some(live) = schid.get() {
            sch = live.array1().value(1).map_err(array_failure)?.clone();
        } else {
            schid = Handle::new(
                HArray1::<Handle<HAsciiString>>::with_bounds(1, 1).map_err(array_failure)?,
            );
            self.set_schema_identifiers(&schid);
        }
        if sch.get().is_some_and(|live| live.length() >= 2) {
            return Ok(());
        }
        let stepro = model.protocol().cast::<dyn StepDataProtocolOps>();
        if let Some(live) = stepro.get() {
            let this: Handle<dyn InterfaceInterfaceModel> = model.model_base().this();
            sch = text(&live.schema_name(&this));
        } else {
            sch = Handle::default();
        }
        if sch.is_some() {
            schid
                .get()
                .map(|list| list.change_array1().set_value(1, sch.clone()))
                .transpose()
                .map_err(array_failure)?;
        }
        Ok(())
    }
    /// `new_model(protocol)`: a new model with this header.
    pub fn new_model(
        &self,
        protocol: &Handle<dyn InterfaceProtocol>,
    ) -> Result<Handle<StepDataStepModel>, Failure> {
        let model = StepDataStepModel::new();
        if let Some(live) = model.get() {
            live.set_protocol(protocol);
            self.apply(live)?;
        }
        Ok(model)
    }
    /// `has_fn`.
    pub fn has_fn(&self) -> bool {
        self.fn_value.is_some()
    }
    /// `fn_value`.
    pub fn fn_value(&self) -> Handle<HeaderSectionFileName> {
        self.fn_value.clone()
    }
    file_name_text!(set_name, name, "Name");
    file_name_text!(set_time_stamp, time_stamp, "TimeStamp");
    entity_list!(
        fn_value,
        set_author,
        set_author_value,
        author,
        author_value,
        nb_author,
        "Author"
    );
    entity_list!(
        fn_value,
        set_organization,
        set_organization_value,
        organization,
        organization_value,
        nb_organization,
        "Organization"
    );
    file_name_text!(
        set_preprocessor_version,
        preprocessor_version,
        "PreprocessorVersion"
    );
    file_name_text!(
        set_originating_system,
        originating_system,
        "OriginatingSystem"
    );
    file_name_text!(set_authorisation, authorisation, "Authorisation");
    /// `has_fs`.
    pub fn has_fs(&self) -> bool {
        self.fs_value.is_some()
    }
    /// `fs_value`.
    pub fn fs_value(&self) -> Handle<HeaderSectionFileSchema> {
        self.fs_value.clone()
    }
    entity_list!(
        fs_value,
        set_schema_identifiers,
        set_schema_identifiers_value,
        schema_identifiers,
        schema_identifiers_value,
        nb_schema_identifiers,
        "SchemaIdentifiers"
    );
    /// `add_schema_identifier(schema_identifier)`: when not yet in the list.
    pub fn add_schema_identifier(&mut self, schema: &Handle<HAsciiString>) -> Result<(), Failure> {
        if self.fs_value.is_null() {
            self.fs_value = Handle::new(HeaderSectionFileSchema::default());
        }
        let known = self.schema_identifiers();
        let mut values = Vec::new();
        if let Some(list) = known.get() {
            values = list.array1().as_slice().to_vec();
        }
        let wanted = schema.get().map(|live| live.string().as_bytes().to_vec());
        let is_same = |value: &Handle<HAsciiString>| {
            value.get().map(|live| live.string().as_bytes().to_vec()) == wanted
        };
        if values.iter().any(is_same) {
            return Ok(());
        }
        values.push(schema.clone());
        let list = HArray1::<Handle<HAsciiString>>::with_bounds(1, values.len() as i32)
            .map_err(array_failure)?;
        for (rank, value) in values.into_iter().enumerate() {
            list.change_array1()
                .set_value(rank as i32 + 1, value)
                .map_err(array_failure)?;
        }
        self.set_schema_identifiers(&Handle::new(list));
        Ok(())
    }
    /// `has_fd`.
    pub fn has_fd(&self) -> bool {
        self.fd_value.is_some()
    }
    /// `fd_value`.
    pub fn fd_value(&self) -> Handle<HeaderSectionFileDescription> {
        self.fd_value.clone()
    }
    /// `set_description(description)`: tested on the file schema, kept.
    pub fn set_description(&self, description: &HeaderSectionTextList) {
        if self.fs_value.is_some()
            && let Some(live) = self.fd_value.get()
        {
            live.set_description(description);
        }
    }
    /// `set_description_value(num, value)`: nothing out of the bounds.
    pub fn set_description_value(
        &self,
        num: i32,
        value: &Handle<HAsciiString>,
    ) -> Result<(), Failure> {
        set_list_value(&self.description(), num, value)
    }
    /// `description`.
    pub fn description(&self) -> HeaderSectionTextList {
        self.fd_value
            .get()
            .map_or_else(Handle::default, HeaderSectionFileDescription::description)
    }
    /// `description_value(num)`.
    pub fn description_value(&self, num: i32) -> Result<Handle<HAsciiString>, Failure> {
        self.fd_value
            .get()
            .map_or_else(|| Ok(Handle::default()), |live| live.description_value(num))
    }
    /// `nb_description`.
    pub fn nb_description(&self) -> i32 {
        self.fd_value
            .get()
            .map_or(0, HeaderSectionFileDescription::nb_description)
    }
    /// `set_implementation_level(level)`.
    pub fn set_implementation_level(&self, level: &Handle<HAsciiString>) {
        if let Some(live) = self.fd_value.get() {
            live.set_implementation_level(level);
        }
    }
    /// `implementation_level`.
    pub fn implementation_level(&self) -> Handle<HAsciiString> {
        self.fd_value.get().map_or_else(
            Handle::default,
            HeaderSectionFileDescription::implementation_level,
        )
    }
}
