//! The plain enumerations of the IGES data package.

/// The way a directory field is defined by an entity: void, a value, a reference, or an error.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataDefType;
/// assert_eq!(IgesDataDefType::default(), IgesDataDefType::DefVoid);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IgesDataDefType {
    /// The field is not defined.
    #[default]
    DefVoid,
    /// The field holds a value.
    DefValue,
    /// The field refers to an entity.
    DefReference,
    /// Any kind of definition is accepted (a checking criterion).
    DefAny,
    /// The field holds a wrong value.
    ErrorVal,
    /// The field holds a wrong reference.
    ErrorRef,
}

/// The way a directory list field (level, view) is defined.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataDefList;
/// assert_eq!(IgesDataDefList::default(), IgesDataDefList::DefNone);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IgesDataDefList {
    /// The field is not defined.
    #[default]
    DefNone,
    /// The field holds one item.
    DefOne,
    /// The field holds a list of items.
    DefSeveral,
    /// The field holds one wrong item.
    ErrorOne,
    /// The field holds a wrong list.
    ErrorSeveral,
}

/// The stage reached while reading the parameters of one entity.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataReadStage;
/// assert_eq!(IgesDataReadStage::ReadOwn.next(), IgesDataReadStage::ReadAssocs);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IgesDataReadStage {
    /// Reading the directory entry.
    #[default]
    ReadDir,
    /// Reading the entity's own parameters.
    ReadOwn,
    /// Reading the associativities.
    ReadAssocs,
    /// Reading the properties.
    ReadProps,
    /// Reading is over.
    ReadEnd,
}

impl IgesDataReadStage {
    /// The stage after this one; the end stage stays where it is.
    pub fn next(self) -> Self {
        match self {
            Self::ReadDir => Self::ReadOwn,
            Self::ReadOwn => Self::ReadAssocs,
            Self::ReadAssocs => Self::ReadProps,
            Self::ReadProps | Self::ReadEnd => Self::ReadEnd,
        }
    }
}

/// The outcome of reading one entity reference.
///
/// ```
/// use martensite_tkdeiges::iges_data::IgesDataStatus;
/// assert_eq!(IgesDataStatus::default(), IgesDataStatus::EntityOk);
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum IgesDataStatus {
    /// The entity was read.
    #[default]
    EntityOk,
    /// The parameter does not give an entity.
    EntityError,
    /// The reference is null where one is required.
    ReferenceError,
    /// The entity read is not of the required type.
    TypeError,
}
