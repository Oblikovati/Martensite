//! `TDataXtdPosition`: a point position attribute.
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Guid, Handle, SharedState, StandardTransient},
};
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::tdata_std::support::{cast_attribute, find_or_add};
use martensite_tklcaf::tdf::{
    TdfAttribute, TdfAttributeBase, TdfAttributeState, TdfLabel, TdfRelocationTable,
};
use martensite_tkmath::gp::Point3;

/// `TDataXtdPosition::GetID()`.
const POSITION_ID: Guid = guid_from_text("55553252-ce0c-11d1-b5d8-00a0c9064368");

/// `TDataXtdPosition`.
///
/// ```
/// use martensite_tkcaf::tdata_xtd::TDataXtdPosition;
/// use martensite_tkmath::gp::Point3;
/// use martensite_tklcaf::tdf::TdfData;
/// let data = TdfData::new();
/// TDataXtdPosition::set_position_of(&data.root(), Point3::new(1.0, 2.0, 3.0)).unwrap();
/// let mut position = Point3::default();
/// assert!(TDataXtdPosition::get(&data.root(), &mut position).unwrap());
/// assert_eq!(position, Point3::new(1.0, 2.0, 3.0));
/// ```
pub struct TDataXtdPosition {
    transient: StandardTransient,
    base: TdfAttributeBase,
    position: SharedState<Point3>,
}
impl_transient!(TDataXtdPosition, "TDataXtdPosition", dyn TdfAttribute, transient, interfaces: [dyn TdfAttribute]);

impl Default for TDataXtdPosition {
    fn default() -> Self {
        Self::new()
    }
}

impl TDataXtdPosition {
    /// `Set(aLabel, aPos)`: the position of the label (found or added) set to `aPos`.
    pub fn set_position_of(label: &TdfLabel, position: Point3) -> Result<(), Failure> {
        let attribute = Self::set(label)?;
        martensite_tklcaf::live(&attribute, "TDataXtdPosition::Set")?.set_position(position)
    }
    /// `Set(aLabel)`: finds or adds the position.
    pub fn set(label: &TdfLabel) -> Result<Handle<TDataXtdPosition>, Failure> {
        find_or_add(label, &Self::get_id(), false, Self::new)
    }
    /// `Get(aLabel, aPos)`: the position of the label, false without one.
    pub fn get(label: &TdfLabel, position: &mut Point3) -> Result<bool, Failure> {
        let mut attribute: Handle<TDataXtdPosition> = Handle::default();
        if !label.find_attribute_as(&Self::get_id(), &mut attribute)? {
            return Ok(false);
        }
        *position = martensite_tklcaf::live(&attribute, "TDataXtdPosition::Get")?.get_position();
        Ok(true)
    }
    /// `TDataXtdPosition()`: at the origin.
    pub fn new() -> Self {
        Self {
            transient: StandardTransient::default(),
            base: TdfAttributeBase::default(),
            position: SharedState::new(Point3::new(0.0, 0.0, 0.0)),
        }
    }
    /// `GetID`.
    pub fn get_id() -> Guid {
        POSITION_ID
    }
    /// `GetPosition`.
    pub fn get_position(&self) -> Point3 {
        *self.position.read()
    }
    /// `SetPosition(aPos)`: backed up when it changes (OCC2932).
    pub fn set_position(&self, position: Point3) -> Result<(), Failure> {
        let old = self.get_position();
        if old.x() == position.x() && old.y() == position.y() && old.z() == position.z() {
            return Ok(());
        }
        self.backup()?;
        *self.position.write() = position;
        Ok(())
    }
}

impl TdfAttribute for TDataXtdPosition {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base
    }
    fn id(&self) -> Guid {
        POSITION_ID
    }
    fn restore(&self, attribute: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other = cast_attribute::<TDataXtdPosition>(attribute, "TDataXtdPosition::Restore")?;
        let position = martensite_tklcaf::live(&other, "TDataXtdPosition::Restore")?.get_position();
        *self.position.write() = position;
        Ok(())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).cast()
    }
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        _relocation_table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        let target = cast_attribute::<TDataXtdPosition>(into, "TDataXtdPosition::Paste")?;
        martensite_tklcaf::live(&target, "TDataXtdPosition::Paste")?
            .set_position(self.get_position())
    }
}
