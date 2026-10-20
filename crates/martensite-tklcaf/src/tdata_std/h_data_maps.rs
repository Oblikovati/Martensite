//! The handle-shared maps of named values that `TDataStd_NamedData` keeps. Natively each
//! class wraps an `NCollection_DataMap` with `Map()`/`ChangeMap()`; here the map sits
//! behind a lock, and `map`/`change_map` lend its read and write guards.
use martensite_tkernel::{
    impl_transient,
    ncollection::{DataMap, HArray1},
    standard::{Handle, StandardTransient},
    tcollection::ExtendedString,
};
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// The behaviour `TDataStd_NamedData` needs from each map class.
pub trait TDataStdNamedMap:
    martensite_tkernel::standard::Transient
    + martensite_tkernel::standard::RuntimeTyped
    + Send
    + Sync
    + 'static
{
    /// The value type.
    type Value: Clone + PartialEq + Send + Sync + 'static;
    /// The copying constructor `(theOther)`.
    fn from_map(map: &DataMap<ExtendedString, Self::Value>) -> Self;
    /// `Map()`: read access.
    fn map(&self) -> RwLockReadGuard<'_, DataMap<ExtendedString, Self::Value>>;
    /// `ChangeMap()`: write access seen by every handle holder.
    fn change_map(&self) -> RwLockWriteGuard<'_, DataMap<ExtendedString, Self::Value>>;
}

/// Declares one handle-shared map class.
macro_rules! h_data_map {
    ($(#[$doc:meta])* $name:ident, $type_name:literal, $value:ty) => {
        $(#[$doc])*
        pub struct $name {
            transient: StandardTransient,
            map: RwLock<DataMap<ExtendedString, $value>>,
        }
        impl_transient!($name, $type_name, StandardTransient, transient);

        impl Default for $name {
            /// The constructor with the default `NbBuckets = 1`.
            fn default() -> Self {
                Self::new(1)
            }
        }

        impl $name {
            /// The constructor `(NbBuckets)`: an empty map resized to `nb_buckets`.
            pub fn new(nb_buckets: i32) -> Self {
                let mut map = DataMap::default();
                map.resize(usize::try_from(nb_buckets).unwrap_or(0));
                Self { transient: StandardTransient::default(), map: RwLock::new(map) }
            }
        }

        impl TDataStdNamedMap for $name {
            type Value = $value;
            /// The constructor `(theOther)`: a copy of `theOther`'s bindings.
            fn from_map(other: &DataMap<ExtendedString, $value>) -> Self {
                let mut map = DataMap::default();
                map.assign(other);
                Self { transient: StandardTransient::default(), map: RwLock::new(map) }
            }
            fn map(&self) -> RwLockReadGuard<'_, DataMap<ExtendedString, $value>> {
                self.map.read().unwrap_or_else(PoisonError::into_inner)
            }
            fn change_map(&self) -> RwLockWriteGuard<'_, DataMap<ExtendedString, $value>> {
                self.map.write().unwrap_or_else(PoisonError::into_inner)
            }
        }
    };
}

h_data_map!(
    /// `TDataStd_HDataMapOfStringInteger`; `Handle::new(TDataStdHDataMapOfStringInteger::new(1))`.
    ///
    /// ```
    /// use martensite_tklcaf::tdata_std::{TDataStdHDataMapOfStringInteger, TDataStdNamedMap};
    /// use martensite_tkernel::tcollection::ExtendedString;
    /// let map = TDataStdHDataMapOfStringInteger::new(1);
    /// map.change_map().bind(ExtendedString::new("a"), 1);
    /// assert_eq!(map.map().find(&ExtendedString::new("a")), Ok(&1));
    /// ```
    TDataStdHDataMapOfStringInteger, "TDataStd_HDataMapOfStringInteger", i32
);
h_data_map!(
    /// `TDataStd_HDataMapOfStringReal`.
    TDataStdHDataMapOfStringReal, "TDataStd_HDataMapOfStringReal", f64
);
h_data_map!(
    /// `TDataStd_HDataMapOfStringString`.
    TDataStdHDataMapOfStringString, "TDataStd_HDataMapOfStringString", ExtendedString
);
h_data_map!(
    /// `TDataStd_HDataMapOfStringByte`.
    TDataStdHDataMapOfStringByte, "TDataStd_HDataMapOfStringByte", u8
);
h_data_map!(
    /// `TDataStd_HDataMapOfStringHArray1OfInteger`: the values are shared arrays.
    TDataStdHDataMapOfStringHArray1OfInteger, "TDataStd_HDataMapOfStringHArray1OfInteger",
    Handle<HArray1<i32>>
);
h_data_map!(
    /// `TDataStd_HDataMapOfStringHArray1OfReal`: the values are shared arrays.
    TDataStdHDataMapOfStringHArray1OfReal, "TDataStd_HDataMapOfStringHArray1OfReal",
    Handle<HArray1<f64>>
);
