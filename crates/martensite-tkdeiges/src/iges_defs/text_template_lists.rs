//! An array of lists of text templates: a rank of an attribute holds a list of its own, which may be null.
use crate::iges_graph::IgesGraphTextDisplayTemplate;
use martensite_tkernel::impl_transient;
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};

/// The lists of text templates of the attributes of a definition, by rank.
///
/// ```
/// use martensite_tkdeiges::iges_defs::IgesDefsHArray1OfHArray1OfTextDisplayTemplate;
/// let lists = IgesDefsHArray1OfHArray1OfTextDisplayTemplate::with_bounds(1, 3).unwrap();
/// assert_eq!(lists.length(), 3);
/// assert!(lists.value(2).unwrap().is_null());
/// ```
pub struct IgesDefsHArray1OfHArray1OfTextDisplayTemplate {
    base: StandardTransient,
    lists: SharedState<Array1<'static, Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>>>>,
}
impl_transient!(
    IgesDefsHArray1OfHArray1OfTextDisplayTemplate,
    "IgesDefsHArray1OfHArray1OfTextDisplayTemplate",
    StandardTransient,
    base
);

impl IgesDefsHArray1OfHArray1OfTextDisplayTemplate {
    /// An array of bounds `low..=up`, with every list null.
    ///
    /// # Errors
    /// A failure for bounds that do not make an array.
    pub fn with_bounds(low: i32, up: i32) -> Result<Self, Failure> {
        let lists = Array1::with_bounds(low, up)
            .map_err(|error| error.into_failure("IgesDefsHArray1OfHArray1OfTextDisplayTemplate"))?;
        Ok(Self {
            base: StandardTransient::default(),
            lists: SharedState::new(lists),
        })
    }
    /// The lower bound.
    pub fn lower(&self) -> i32 {
        self.lists.read().lower()
    }
    /// The upper bound.
    pub fn upper(&self) -> i32 {
        self.lists.read().upper()
    }
    /// The number of lists.
    pub fn length(&self) -> i32 {
        self.lists.read().length()
    }
    /// Sets the list at a rank.
    ///
    /// # Errors
    /// An out of range failure for a bad rank.
    pub fn set_value(
        &self,
        num: i32,
        val: &Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>>,
    ) -> Result<(), Failure> {
        self.lists
            .write()
            .set_value(num, val.clone())
            .map_err(|error| error.into_failure("set_value"))
    }
    /// The list at a rank, null when none was set.
    ///
    /// # Errors
    /// An out of range failure for a bad rank.
    pub fn value(
        &self,
        num: i32,
    ) -> Result<Handle<HArray1<Handle<IgesGraphTextDisplayTemplate>>>, Failure> {
        self.lists
            .read()
            .value(num)
            .cloned()
            .map_err(|error| error.into_failure("value"))
    }
}
