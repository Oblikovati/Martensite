//! The signatures of entities: their label, type, status and level. The one of the color is with
//! the color entity, in `sign_color`.
use crate::iges_data::{IgesDataIgesEntity, IgesDataLevelListEntity, IgesDataUndefinedEntity};
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Handle, RuntimeTyped, SharedState, StandardTransient},
    tcollection::AsciiString,
};
use martensite_tkxsbase::{
    if_select::{IFSelectSignature, IFSelectSignatureBase},
    interface::{InterfaceInterfaceModel, InterfaceSignType, interface_sign_type_text},
    moni_tool::MoniToolSignText,
};

/// Declares a signature: its fields, its runtime type, and the name and text it shares.
macro_rules! iges_signature {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, { $($field:ident: $ty:ty),* $(,)? }) => {
        $(#[$meta])*
        pub struct $class {
            transient: StandardTransient,
            base: IFSelectSignatureBase,
            $($field: $ty,)*
        }
        impl_transient!(
            $class,
            $recorded,
            dyn IFSelectSignature,
            transient,
            interfaces: [dyn MoniToolSignText, dyn InterfaceSignType, dyn IFSelectSignature]
        );
        impl MoniToolSignText for $class {
            fn name(&self) -> String {
                martensite_tkxsbase::if_select::signature_name(self)
            }
            fn text(
                &self,
                ent: &Handle<StandardTransient>,
                context: &Handle<StandardTransient>,
            ) -> AsciiString {
                interface_sign_type_text(self, ent, context)
            }
        }
    };
}
pub(super) use iges_signature;

iges_signature! {
    /// The short label of an entity.
    ///
    /// ```
    /// use martensite_tkdeiges::{iges_data::IgesDataIgesEntity, iges_select::IgesSelectIGESName};
    /// use martensite_tkernel::standard::Handle;
    /// use martensite_tkxsbase::interface::InterfaceSignType;
    /// let entity = Handle::new(IgesDataIgesEntity::default());
    /// assert_eq!(IgesSelectIGESName::new().value(&entity.cast(), &Handle::default()), "");
    /// ```
    IgesSelectIGESName, "IgesSelectIGESName", {}
}

impl Default for IgesSelectIGESName {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectIGESName {
    /// The signature.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSignatureBase::new("IGES Name (Short Label)"),
        }
    }
}

impl InterfaceSignType for IgesSelectIGESName {
    fn value(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return "?".to_owned();
        };
        live.short_label().get().map_or_else(String::new, |text| {
            String::from_utf8_lossy(text.string().as_bytes()).into_owned()
        })
    }
}

impl IFSelectSignature for IgesSelectIGESName {
    fn signature_base(&self) -> &IFSelectSignatureBase {
        &self.base
    }
}

/// The packages of the entity classes: a class name starts with one of these.
const ENTITY_PACKAGES: [&str; 9] = [
    "IgesAppli",
    "IgesBasic",
    "IgesData",
    "IgesDefs",
    "IgesDimen",
    "IgesDraw",
    "IgesGeom",
    "IgesGraph",
    "IgesSolid",
];

/// A class name without its package; `class_without_package("IgesGeomLine") == "Line"`.
fn class_without_package(name: &str) -> &str {
    ENTITY_PACKAGES
        .iter()
        .find_map(|package| name.strip_prefix(package))
        .unwrap_or(name)
}

iges_signature! {
    /// The class of an entity, without its package: what the session shows in its check lists.
    IgesSelectClassType, "IgesSelectClassType", {}
}

impl Default for IgesSelectClassType {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectClassType {
    /// The signature.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSignatureBase::new("Class Type"),
        }
    }
}

impl InterfaceSignType for IgesSelectClassType {
    fn value(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        ent.dynamic_type().map_or_else(String::new, |kind| {
            class_without_package(kind.name()).to_owned()
        })
    }
}

impl IFSelectSignature for IgesSelectClassType {
    fn signature_base(&self) -> &IFSelectSignatureBase {
        &self.base
    }
}

iges_signature! {
    /// The type number of an entity, with its form when asked; `(?)` after an undefined entity.
    ///
    /// ```
    /// use martensite_tkdeiges::{iges_data::IgesDataIgesEntity, iges_select::IgesSelectIGESTypeForm};
    /// use martensite_tkernel::standard::Handle;
    /// use martensite_tkxsbase::{interface::InterfaceSignType, moni_tool::MoniToolSignText};
    /// let entity = Handle::new(IgesDataIgesEntity::default());
    /// entity.get().unwrap().init_type_and_form(110, 1);
    /// let sign = IgesSelectIGESTypeForm::new(true);
    /// assert_eq!(sign.value(&entity.cast(), &Handle::default()), "110 1");
    /// assert_eq!(sign.name(), "IGES Type & Form Numbers");
    /// ```
    IgesSelectIGESTypeForm, "IgesSelectIGESTypeForm", { theform: SharedState<bool> }
}

impl IgesSelectIGESTypeForm {
    /// The signature, with the form when `withform`. The form can be changed; the name given at
    /// the construction stays only until then.
    pub fn new(withform: bool) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSignatureBase::new(Self::name_of(withform)),
            theform: SharedState::new(withform),
        }
    }
    fn name_of(withform: bool) -> &'static str {
        if withform {
            "IGES Type & Form Numbers"
        } else {
            "IGES Type Number"
        }
    }
    /// Sets whether the form is in the value.
    pub fn set_form(&mut self, withform: bool) {
        *self.theform.write() = withform;
        self.base.thename = AsciiString::new(Self::name_of(withform));
    }
}

impl InterfaceSignType for IgesSelectIGESTypeForm {
    fn value(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return "?".to_owned();
        };
        let unknown = if ent.is_kind(IgesDataUndefinedEntity::static_type()) {
            " (?)"
        } else {
            ""
        };
        if *self.theform.read() {
            format!("{} {}{unknown}", live.type_number(), live.form_number())
        } else {
            format!("{}{unknown}", live.type_number())
        }
    }
}

impl IFSelectSignature for IgesSelectIGESTypeForm {
    fn signature_base(&self) -> &IFSelectSignatureBase {
        &self.base
    }
}

iges_signature! {
    /// The level of an entity; for counting, or the whole level list.
    ///
    /// ```
    /// use martensite_tkdeiges::{iges_data::IgesDataIgesEntity, iges_select::IgesSelectSignLevelNumber};
    /// use martensite_tkernel::standard::Handle;
    /// use martensite_tkxsbase::interface::InterfaceSignType;
    /// let entity = Handle::new(IgesDataIgesEntity::default());
    /// let sign = IgesSelectSignLevelNumber::new(false);
    /// assert_eq!(sign.value(&entity.cast(), &Handle::default()), "/0/");
    /// ```
    IgesSelectSignLevelNumber, "IgesSelectSignLevelNumber", { thecountmode: bool }
}

impl IgesSelectSignLevelNumber {
    /// The signature; `countmode` gives a value fit to count.
    pub fn new(countmode: bool) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSignatureBase::new("Level Number"),
            thecountmode: countmode,
        }
    }
    fn list_value(&self, list: &dyn IgesDataLevelListEntity) -> Result<String, Failure> {
        if self.thecountmode {
            return Ok("LEVEL LIST".to_owned());
        }
        let mut value = "LIST:/".to_owned();
        for rank in 1..=list.nb_level_numbers()? {
            value.push_str(&format!("{}/", list.level_number(rank)?));
        }
        Ok(value)
    }
}

impl InterfaceSignType for IgesSelectSignLevelNumber {
    fn value(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        let none = if self.thecountmode {
            " NO LEVEL"
        } else {
            "/0/"
        };
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return none.to_owned();
        };
        let levels = live.level_list();
        if let Some(list) = levels.get() {
            // A list not set up has no number to show.
            return self
                .list_value(list)
                .unwrap_or_else(|_| "LIST:/".to_owned());
        }
        let level = live.level();
        if level < 0 {
            none.to_owned()
        } else if self.thecountmode {
            format!("{level:7}")
        } else {
            format!("/{level}/")
        }
    }
}

impl IFSelectSignature for IgesSelectSignLevelNumber {
    fn signature_base(&self) -> &IFSelectSignatureBase {
        &self.base
    }
}

iges_signature! {
    /// The four status of the directory entry.
    ///
    /// ```
    /// use martensite_tkdeiges::{iges_data::IgesDataIgesEntity, iges_select::IgesSelectSignStatus};
    /// use martensite_tkernel::standard::Handle;
    /// use martensite_tkxsbase::interface::InterfaceSignType;
    /// let entity = Handle::new(IgesDataIgesEntity::default());
    /// entity.get().unwrap().init_status(1, 2, 3, 0);
    /// assert_eq!(IgesSelectSignStatus::new().value(&entity.cast(), &Handle::default()), "1,2,3,0");
    /// ```
    IgesSelectSignStatus, "IgesSelectSignStatus", {}
}

impl Default for IgesSelectSignStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesSelectSignStatus {
    /// The signature.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSignatureBase::new("D.E. Status"),
        }
    }
}

impl InterfaceSignType for IgesSelectSignStatus {
    fn value(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
    ) -> String {
        let entity = ent.cast::<IgesDataIgesEntity>();
        entity.get().map_or_else(String::new, |live| {
            format!(
                "{},{},{},{}",
                live.blank_status(),
                live.subordinate_status(),
                live.use_flag(),
                live.hierarchy_status()
            )
        })
    }
}

/// The value a character of a status text means, by the rank of its field: blank (V, B) or
/// subordinate (I, P, L, D) letters, else a digit.
fn status_value(car: u8, field: i32) -> i32 {
    match (car, field) {
        (b'V', 0) | (b'I', 1) => 0,
        (b'B', 0) | (b'P', 1) => 1,
        (b'L', 1) => 2,
        (b'D', 1) => 3,
        _ => i32::from(car) - 48,
    }
}

impl IFSelectSignature for IgesSelectSignStatus {
    fn signature_base(&self) -> &IFSelectSignatureBase {
        &self.base
    }
    /// Whether the status fit a text of fields separated by commas: all of them when `exact`,
    /// else one.
    fn matches(
        &self,
        ent: &Handle<StandardTransient>,
        _model: &Handle<dyn InterfaceInterfaceModel>,
        text: &AsciiString,
        exact: bool,
    ) -> Result<bool, Failure> {
        let entity = ent.cast::<IgesDataIgesEntity>();
        let Some(live) = entity.get() else {
            return Ok(false);
        };
        let fields = [
            live.blank_status(),
            live.subordinate_status(),
            live.use_flag(),
            live.hierarchy_status(),
        ];
        let mut vir = 0;
        for car in text.as_bytes().iter().take(9) {
            if *car == b',' {
                vir += 1;
                continue;
            }
            let Some(field) = fields.get(vir as usize) else {
                continue;
            };
            let same = *field == status_value(*car, vir);
            if same && !exact {
                return Ok(true);
            }
            if !same && exact {
                return Ok(false);
            }
        }
        Ok(exact)
    }
}

#[cfg(test)]
mod class_type_tests {
    use super::IgesSelectClassType;
    use crate::iges_geom::IgesGeomLine;
    use martensite_tkernel::standard::Handle;
    use martensite_tkxsbase::interface::InterfaceSignType;

    #[test]
    fn the_class_type_drops_the_package() {
        let entity = Handle::new(IgesGeomLine::default());
        let value = IgesSelectClassType::new().value(&entity.cast(), &Handle::default());
        assert_eq!(value, "Line");
    }
}
