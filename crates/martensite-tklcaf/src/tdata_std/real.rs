//! `TDataStd_Real`: a real value attribute with a user ID and a (deprecated) dimension.
use super::TDataStd;
use super::support::{cast_attribute, find_or_add, reset_user_id, set_backed_up, set_user_id};
use crate::guid_from_text;
use crate::json::dump_attribute_json_base;
use crate::tdf::{
    TdfAttribute, TdfAttributeBase, TdfAttributeState, TdfLabel, TdfReference, TdfRelocationTable,
};
use martensite_tkcdf::json_dump::{class_begin, field_integer, field_real};
use martensite_tkernel::{
    impl_transient, impl_transient_interface_target,
    standard::{
        Failure, Guid, Handle, HandleUpcast, RuntimeType, RuntimeTyped, SharedState,
        dump::DumpStream,
    },
    tcollection::AsciiString,
};
use std::{io, sync::Arc};

/// `TDataStd_Real::GetID()`.
const REAL_ID: Guid = guid_from_text("2a96b60f-ec8b-11d0-bee7-080009dc3333");

/// `TDataStd_RealEnum`: the dimension of a real, in native order.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TDataStdRealEnum {
    /// `TDataStd_SCALAR`.
    #[default]
    Scalar,
    /// `TDataStd_LENGTH`.
    Length,
    /// `TDataStd_ANGULAR`.
    Angular,
}

/// `TDataStd_Real`; `TDataStdReal::set(&label, 1.5)`.
///
/// ```
/// use martensite_tklcaf::{tdata_std::TDataStdReal, tdf::TdfData};
/// let data = TdfData::new();
/// let attribute = TDataStdReal::set(&data.root(), 1.5).unwrap();
/// assert_eq!(TDataStdReal::get(&attribute), 1.5);
/// ```
#[derive(Default)]
pub struct TDataStdReal {
    base: TDataStdRealBase,
}
impl_transient!(TDataStdReal, "TDataStd_Real", dyn TdfAttribute, interfaces: [dyn TdfAttribute, dyn TDataStdRealSource]);

/// Storage shared by OCAF real attributes. Example: `TDataStdRealBase::default()`.
pub struct TDataStdRealBase {
    attribute: TdfAttributeBase,
    value: SharedState<f64>,
    dimension: SharedState<TDataStdRealEnum>,
    id: SharedState<Guid>,
}

impl Default for TDataStdRealBase {
    /// The native initial value is the most negative double.
    fn default() -> Self {
        Self {
            attribute: TdfAttributeBase::default(),
            value: SharedState::new(f64::MIN),
            dimension: SharedState::new(TDataStdRealEnum::Scalar),
            id: SharedState::new(REAL_ID),
        }
    }
}

impl TDataStdRealBase {
    /// The composed label and transaction fields. Example: `real.real_base().attribute()`.
    pub fn attribute(&self) -> &TdfAttributeBase {
        &self.attribute
    }
}

/// Inherited real operations on the actual attribute. Example: `real.upcast::<dyn TDataStdRealSource>()`.
pub trait TDataStdRealSource: TdfAttribute {
    /// The real fields. Example: `real.real_base().attribute()`.
    fn real_base(&self) -> &TDataStdRealBase;

    /// Changes the value with a backup of this attribute. Example: `real.set_value(2.5)?`.
    fn set_value(&self, value: f64) -> Result<(), Failure> {
        set_backed_up(self, &self.real_base().value, &value)
    }

    /// Reads the value. Example: `TDataStdRealSource::get(real)`.
    fn get(&self) -> f64 {
        *self.real_base().value.read()
    }

    /// Changes the legacy dimension with a backup. Example: `real.set_dimension(TDataStdRealEnum::Length)?`.
    fn set_dimension(&self, dimension: TDataStdRealEnum) -> Result<(), Failure> {
        set_backed_up(self, &self.real_base().dimension, &dimension)
    }

    /// Reads the legacy dimension. Example: `real.get_dimension()`.
    fn get_dimension(&self) -> TDataStdRealEnum {
        *self.real_base().dimension.read()
    }

    /// Checks whether the label references an origin with this real ID. Example: `real.is_captured()?`.
    fn is_captured(&self) -> Result<bool, Failure> {
        let mut reference: Handle<TdfReference> = Handle::default();
        if !self
            .label()
            .find_attribute_as(&TdfReference::get_id(), &mut reference)?
        {
            return Ok(false);
        }
        let origin = crate::live(&reference, "TDataStd_Real::IsCaptured")?.get();
        let id = *self.real_base().id.read();
        origin.is_attribute(&id)
    }

    /// Restores the stored real fields. Example: `real.restore_real(&backup)?`.
    fn restore_real(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other = cast_attribute::<dyn TDataStdRealSource>(with, "TDataStd_Real::Restore")?;
        let other = crate::live(&other, "TDataStd_Real::Restore")?;
        let (value, dimension, id) = (
            other.get(),
            other.get_dimension(),
            *other.real_base().id.read(),
        );
        let target = self.real_base();
        *target.value.write() = value;
        *target.dimension.write() = dimension;
        *target.id.write() = id;
        Ok(())
    }

    /// Copies the stored real fields through the destination's backup. Example: `real.paste_real(&destination)?`.
    fn paste_real(&self, into: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let target = cast_attribute::<dyn TDataStdRealSource>(into, "TDataStd_Real::Paste")?;
        let target = crate::live(&target, "TDataStd_Real::Paste")?;
        target.set_value(self.get())?;
        target.set_dimension(self.get_dimension())?;
        // Derived measures have a fixed attribute GUID separate from the stored real GUID.
        let id = *self.real_base().id.read();
        set_user_id(target, &target.real_base().id, &id)
    }
}

impl RuntimeTyped for dyn TDataStdRealSource {
    const TYPE_NAME: &'static str = TDataStdReal::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        TDataStdReal::static_type()
    }
}
impl_transient_interface_target!(dyn TDataStdRealSource);

impl HandleUpcast<dyn TdfAttribute> for dyn TDataStdRealSource {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn TdfAttribute> {
        owner
    }
}

impl TDataStdRealSource for TDataStdReal {
    fn real_base(&self) -> &TDataStdRealBase {
        &self.base
    }
}

impl TDataStdReal {
    /// `GetID`: the default ID.
    pub fn get_id() -> Guid {
        REAL_ID
    }
    /// `TDataStd_Real()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Set(label, value)`: finds or adds the attribute of the default ID and sets it.
    pub fn set(label: &TdfLabel, value: f64) -> Result<Handle<TDataStdReal>, Failure> {
        Self::set_with_id(label, &REAL_ID, value)
    }
    /// `Set(label, guid, value)`: the same with a user ID.
    pub fn set_with_id(
        label: &TdfLabel,
        guid: &Guid,
        value: f64,
    ) -> Result<Handle<TDataStdReal>, Failure> {
        let attribute = find_or_add(label, guid, true, Self::new)?;
        crate::live(&attribute, "TDataStd_Real::Set")?.set_value(value)?;
        Ok(attribute)
    }
    /// Changes the value with a backup. Example: `attribute.set_value(2.5)?`.
    pub fn set_value(&self, value: f64) -> Result<(), Failure> {
        TDataStdRealSource::set_value(self, value)
    }
    /// Reads the value. Example: `TDataStdReal::get(&attribute)`.
    pub fn get(&self) -> f64 {
        TDataStdRealSource::get(self)
    }
    /// Changes the legacy dimension with a backup. Example: `attribute.set_dimension(TDataStdRealEnum::Length)?`.
    pub fn set_dimension(&self, dimension: TDataStdRealEnum) -> Result<(), Failure> {
        TDataStdRealSource::set_dimension(self, dimension)
    }
    /// Reads the legacy dimension. Example: `attribute.get_dimension()`.
    pub fn get_dimension(&self) -> TDataStdRealEnum {
        TDataStdRealSource::get_dimension(self)
    }
    /// Checks for a referenced origin with this real ID. Example: `attribute.is_captured()?`.
    pub fn is_captured(&self) -> Result<bool, Failure> {
        TDataStdRealSource::is_captured(self)
    }
}

impl TdfAttribute for TDataStdReal {
    fn attribute_base(&self) -> &TdfAttributeBase {
        self.base.attribute()
    }
    fn id(&self) -> Guid {
        *self.base.id.read()
    }
    fn set_id(&self, guid: &Guid) -> Result<(), Failure> {
        set_user_id(self, &self.base.id, guid)
    }
    fn set_default_id(&self) -> Result<(), Failure> {
        reset_user_id(self, &self.base.id, REAL_ID)
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        self.restore_real(with)
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).into_upcast()
    }
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        _relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        self.paste_real(into)
    }
    /// `Dump`: `Real <dimension><value><guid>` with the default stream precision.
    fn dump(&self, stream: &mut dyn io::Write) -> io::Result<()> {
        write!(stream, "Real ")?;
        TDataStd::print(self.get_dimension(), stream)?;
        let value = AsciiString::from_real(self.get());
        stream.write_all(value.as_bytes())?;
        write!(stream, "{}", self.id())
    }
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "TDataStd_Real")?;
        dump_attribute_json_base(self, stream, depth)?;
        field_real(stream, "myValue", self.get())?;
        field_integer(stream, "myDimension", self.get_dimension() as i32)
    }
}
