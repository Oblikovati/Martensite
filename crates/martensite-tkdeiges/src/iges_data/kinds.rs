//! The abstract kinds of entities a directory entry refers to.
//!
//! Each kind is a trait over [`IgesDataEntityKind`], so a handle of a kind reaches the embedded
//! directory part through `iges_entity()`. An entity class declares its kind through the `kind`
//! form of `iges_entity!`.
use super::{IgesDataEntityKind, IgesDataIgesEntity};
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle},
    tcollection::HAsciiString,
};
use martensite_tkmath::gp::GeneralTransform3;

/// An entity that defines a color.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataColorEntity, IgesDataEntityKind};
/// fn type_of(color: &dyn IgesDataColorEntity) -> i32 { color.iges_entity().type_number() }
/// ```
pub trait IgesDataColorEntity: IgesDataEntityKind {}
impl_transient_interface!(
    dyn IgesDataColorEntity,
    "IgesDataColorEntity",
    IgesDataIgesEntity
);

/// An entity that defines how a label is displayed.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataLabelDisplayEntity, IgesDataEntityKind};
/// fn type_of(label: &dyn IgesDataLabelDisplayEntity) -> i32 { label.iges_entity().type_number() }
/// ```
pub trait IgesDataLabelDisplayEntity: IgesDataEntityKind {}
impl_transient_interface!(
    dyn IgesDataLabelDisplayEntity,
    "IgesDataLabelDisplayEntity",
    IgesDataIgesEntity
);

/// An entity that defines a line font.
///
/// ```
/// use martensite_tkdeiges::iges_data::{IgesDataLineFontEntity, IgesDataEntityKind};
/// fn type_of(font: &dyn IgesDataLineFontEntity) -> i32 { font.iges_entity().type_number() }
/// ```
pub trait IgesDataLineFontEntity: IgesDataEntityKind {}
impl_transient_interface!(
    dyn IgesDataLineFontEntity,
    "IgesDataLineFontEntity",
    IgesDataIgesEntity
);

/// An entity that lists levels.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataLevelListEntity;
/// fn empty(list: &dyn IgesDataLevelListEntity) -> bool { list.nb_level_numbers().unwrap_or(0) == 0 }
/// ```
pub trait IgesDataLevelListEntity: IgesDataEntityKind {
    /// The count of levels.
    ///
    /// # Errors
    /// A failure when the list is not set up.
    fn nb_level_numbers(&self) -> Result<i32, Failure>;
    /// The level at a rank from 1.
    ///
    /// # Errors
    /// An out of range failure for a bad rank.
    fn level_number(&self, num: i32) -> Result<i32, Failure>;
    /// Whether `level` is in the list.
    ///
    /// # Errors
    /// As [`Self::level_number`].
    fn has_level_number(&self, level: i32) -> Result<bool, Failure> {
        for num in 1..=self.nb_level_numbers()? {
            if self.level_number(num)? == level {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
impl_transient_interface!(
    dyn IgesDataLevelListEntity,
    "IgesDataLevelListEntity",
    IgesDataIgesEntity
);

/// An entity that gives a name to the entity it is a property of.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataNameEntity;
/// fn is_named(name: &dyn IgesDataNameEntity) -> bool { name.value().is_some() }
/// ```
pub trait IgesDataNameEntity: IgesDataEntityKind {
    /// The name.
    fn value(&self) -> Handle<HAsciiString>;
}
impl_transient_interface!(
    dyn IgesDataNameEntity,
    "IgesDataNameEntity",
    IgesDataIgesEntity
);

/// An entity that binds one parent to its children.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataSingleParentEntity;
/// fn has_parent(link: &dyn IgesDataSingleParentEntity) -> bool { link.single_parent().is_some() }
/// ```
pub trait IgesDataSingleParentEntity: IgesDataEntityKind {
    /// The parent.
    fn single_parent(&self) -> Handle<IgesDataIgesEntity>;
    /// The count of children.
    ///
    /// # Errors
    /// A failure when the list is not set up.
    fn nb_children(&self) -> Result<i32, Failure>;
    /// The child at a rank from 1.
    ///
    /// # Errors
    /// An out of range failure for a bad rank.
    fn child(&self, num: i32) -> Result<Handle<IgesDataIgesEntity>, Failure>;
}
impl_transient_interface!(
    dyn IgesDataSingleParentEntity,
    "IgesDataSingleParentEntity",
    IgesDataIgesEntity
);

/// An entity that defines a transformation.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataTransfEntity;
/// fn is_set(transf: &dyn IgesDataTransfEntity) -> bool { transf.value().is_ok() }
/// ```
pub trait IgesDataTransfEntity: IgesDataEntityKind {
    /// The transformation, compounded with the one this entity refers to.
    ///
    /// # Errors
    /// A failure when the transformation is not set up.
    fn value(&self) -> Result<GeneralTransform3, Failure>;
}
impl_transient_interface!(
    dyn IgesDataTransfEntity,
    "IgesDataTransfEntity",
    IgesDataIgesEntity
);

/// An entity that defines a view, or a list of views.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataViewKindEntity;
/// fn is_list(view: &dyn IgesDataViewKindEntity) -> bool { !view.is_single() }
/// ```
pub trait IgesDataViewKindEntity: IgesDataEntityKind {
    /// Whether this is one view rather than a list.
    fn is_single(&self) -> bool;
    /// The count of views.
    ///
    /// # Errors
    /// A failure when the list is not set up.
    fn nb_views(&self) -> Result<i32, Failure>;
    /// The view at a rank from 1.
    ///
    /// # Errors
    /// An out of range failure for a bad rank.
    fn view_item(&self, num: i32) -> Result<Handle<dyn IgesDataViewKindEntity>, Failure>;
}
impl_transient_interface!(
    dyn IgesDataViewKindEntity,
    "IgesDataViewKindEntity",
    IgesDataIgesEntity
);
