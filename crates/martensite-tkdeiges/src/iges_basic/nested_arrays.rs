//! Arrays of shared arrays: a rank of a loop or a boundary holds a list of its own, which may be null.
use crate::iges_data::IgesDataIgesEntity;
use martensite_tkernel::impl_transient;
use martensite_tkernel::ncollection::{Array1, HArray1};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{Xy, Xyz};
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

/// The lists of one array; a poisoned lock still holds consistent handles.
fn read_lists<'a, T: Send + Sync + 'static>(
    lists: &'a RwLock<Array1<'static, Handle<HArray1<T>>>>,
) -> RwLockReadGuard<'a, Array1<'static, Handle<HArray1<T>>>> {
    lists
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

macro_rules! iges_nested_array {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $element:ty) => {
        $(#[$meta])*
        pub struct $class {
            transient: StandardTransient,
            lists: RwLock<Array1<'static, Handle<HArray1<$element>>>>,
        }
        impl_transient!($class, $recorded, StandardTransient, transient);
        impl $class {
            /// An array of bounds `low..=up`, with every list null.
            ///
            /// # Errors
            /// A failure for bounds that do not make an array.
            pub fn with_bounds(low: i32, up: i32) -> Result<Self, Failure> {
                let lists = Array1::with_bounds(low, up)
                    .map_err(|error| error.into_failure(stringify!($class)))?;
                Ok(Self { transient: StandardTransient::default(), lists: RwLock::new(lists) })
            }
            /// The lower bound.
            pub fn lower(&self) -> i32 {
                read_lists(&self.lists).lower()
            }
            /// The upper bound.
            pub fn upper(&self) -> i32 {
                read_lists(&self.lists).upper()
            }
            /// The number of lists.
            pub fn length(&self) -> i32 {
                read_lists(&self.lists).length()
            }
            /// Sets the list at a rank.
            ///
            /// # Errors
            /// An out of range failure for a bad rank.
            pub fn set_value(&self, num: i32, val: &Handle<HArray1<$element>>) -> Result<(), Failure> {
                let mut lists = self.lists.write().unwrap_or_else(std::sync::PoisonError::into_inner);
                lists
                    .set_value(num, val.clone())
                    .map_err(|error| error.into_failure(concat!(stringify!($class), "::set_value")))
            }
            /// The list at a rank, null when none was set.
            ///
            /// # Errors
            /// An out of range failure for a bad rank.
            pub fn value(&self, num: i32) -> Result<Handle<HArray1<$element>>, Failure> {
                read_lists(&self.lists)
                    .value(num)
                    .cloned()
                    .map_err(|error| error.into_failure(concat!(stringify!($class), "::value")))
            }
        }
    };
}

iges_nested_array! {
    /// An array of lists of entities.
    IgesBasicHArray1OfHArray1OfIgesEntity, "IgesBasicHArray1OfHArray1OfIgesEntity", Handle<IgesDataIgesEntity>
}
iges_nested_array! {
    /// An array of lists of integers.
    IgesBasicHArray1OfHArray1OfInteger, "IgesBasicHArray1OfHArray1OfInteger", i32
}
iges_nested_array! {
    /// An array of lists of reals.
    IgesBasicHArray1OfHArray1OfReal, "IgesBasicHArray1OfHArray1OfReal", f64
}
iges_nested_array! {
    /// An array of lists of plane coordinates.
    IgesBasicHArray1OfHArray1OfXy, "IgesBasicHArray1OfHArray1OfXy", Xy
}
iges_nested_array! {
    /// An array of lists of space coordinates.
    IgesBasicHArray1OfHArray1OfXyz, "IgesBasicHArray1OfHArray1OfXyz", Xyz
}

/// An array of two lists of two lines, for the samples of the classes that hold such lists.
#[cfg(test)]
pub(crate) fn nested_entity_lists() -> Result<Handle<IgesBasicHArray1OfHArray1OfIgesEntity>, Failure>
{
    use crate::iges_geom::samples_01::sample_iges_geom_line;
    use crate::iges_test_samples::sample_list;
    let nested = IgesBasicHArray1OfHArray1OfIgesEntity::with_bounds(1, 2)?;
    for rank in 1..=2 {
        let line = || -> Result<Handle<IgesDataIgesEntity>, Failure> {
            Ok(sample_iges_geom_line()?.cast())
        };
        nested.set_value(rank, &sample_list([line()?, line()?])?)?;
    }
    Ok(Handle::new(nested))
}

#[cfg(test)]
mod tests {
    use super::{IgesBasicHArray1OfHArray1OfInteger, IgesBasicHArray1OfHArray1OfXyz};
    use martensite_tkernel::ncollection::HArray1;
    use martensite_tkernel::standard::Handle;

    #[test]
    fn a_new_array_holds_null_lists_within_its_bounds() {
        let lists = IgesBasicHArray1OfHArray1OfInteger::with_bounds(1, 3).expect("bounds");
        assert_eq!((lists.lower(), lists.upper(), lists.length()), (1, 3, 3));
        assert!(lists.value(2).expect("rank").is_null());
        assert!(lists.value(4).is_err());
    }

    #[test]
    fn a_set_list_is_read_back_at_its_rank() {
        let lists = IgesBasicHArray1OfHArray1OfXyz::with_bounds(1, 2).expect("bounds");
        let list = Handle::new(HArray1::with_bounds(1, 4).expect("bounds"));
        lists.set_value(2, &list).expect("rank");
        assert_eq!(lists.value(2).expect("rank").array1().length(), 4);
        assert!(lists.set_value(3, &list).is_err());
    }
}
