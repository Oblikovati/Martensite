//! The shared shape of the IGES entity classes.
//!
//! `iges_entity!` declares a class: its fields in one `SharedState`, an embedded parent, a runtime
//! type, `new` and `deref`, so inherited members resolve through `deref` and
//! `Handle<IgesGeomLine>` casts to `Handle<IgesDataIgesEntity>`.
//! `iges_methods!` writes the regular accessors, one line each.
use martensite_tkernel::ncollection::{HArray1, HArray2, array2_failure};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, exceptions};

/// The object of a handle, or the null object failure `where_called` makes.
///
/// # Errors
/// A null object failure for a null handle.
pub fn iges_live<'a, T: ?Sized + RuntimeTyped>(
    handle: &'a Handle<T>,
    where_called: &str,
) -> Result<&'a T, Failure> {
    handle
        .get()
        .ok_or_else(|| exceptions::null_object(&format!("{where_called}: null handle")))
}

/// A new shared array of bounds `lower..=upper`.
///
/// # Errors
/// A failure for bounds that do not make an array.
pub fn iges_new_array1<T: Default + Send + Sync + 'static>(
    lower: i32,
    upper: i32,
) -> Result<Handle<HArray1<T>>, Failure> {
    HArray1::with_bounds(lower, upper)
        .map(Handle::new)
        .map_err(|error| error.into_failure("HArray1"))
}

/// A new shared two-dimensional array of the given bounds.
///
/// # Errors
/// A failure for bounds that do not make an array.
pub fn iges_new_array2<T: Default + Send + Sync + 'static>(
    lower_row: i32,
    upper_row: i32,
    lower_col: i32,
    upper_col: i32,
) -> Result<Handle<HArray2<T>>, Failure> {
    HArray2::with_bounds(lower_row, upper_row, lower_col, upper_col)
        .map(Handle::new)
        .map_err(array2_failure)
}

/// The item at `index` of a shared array.
///
/// # Errors
/// A null object failure for a null array, an out of range failure for a bad index.
pub fn iges_array1_item<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray1<T>>,
    index: i32,
    where_called: &str,
) -> Result<T, Failure> {
    let items = iges_live(array, where_called)?.array1();
    items
        .value(index)
        .cloned()
        .map_err(|error| error.into_failure(where_called))
}

/// Sets the item at `index` of a shared array.
///
/// # Errors
/// A null object failure for a null array, an out of range failure for a bad index.
pub fn iges_array1_set<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray1<T>>,
    index: i32,
    value: T,
    where_called: &str,
) -> Result<(), Failure> {
    let mut items = iges_live(array, where_called)?.change_array1();
    items
        .set_value(index, value)
        .map_err(|error| error.into_failure(where_called))
}

/// The item at `row`, `column` of a shared two-dimensional array.
///
/// # Errors
/// A null object failure for a null array, an out of range failure for a bad index.
pub fn iges_array2_item<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray2<T>>,
    row: i32,
    column: i32,
    where_called: &str,
) -> Result<T, Failure> {
    let items = iges_live(array, where_called)?.array2();
    items.value(row, column).cloned().map_err(array2_failure)
}

/// Sets the item at `row`, `column` of a shared two-dimensional array.
///
/// # Errors
/// A null object failure for a null array, an out of range failure for a bad index.
pub fn iges_array2_set<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray2<T>>,
    row: i32,
    column: i32,
    value: T,
    where_called: &str,
) -> Result<(), Failure> {
    let mut items = iges_live(array, where_called)?.change_array2();
    items.set_value(row, column, value).map_err(array2_failure)
}

/// The length of a shared array, 0 for a null array.
pub fn iges_array1_len<T: Send + Sync + 'static>(array: &Handle<HArray1<T>>) -> i32 {
    array.get().map_or(0, |items| items.array1().length())
}

/// Declares an entity class.
///
/// ```text
/// iges_entity! {
///     /// A line segment.
///     IgesGeomLine, IgesGeomLineState, "IgesGeomLine", parent IgesDataIgesEntity, {
///         start: Xyz,
///         end: Xyz,
///     }
/// }
/// iges_entity! {
///     /// A transformation matrix.
///     IgesGeomTransformationMatrix, IgesGeomTransformationMatrixState, "IgesGeomTransformationMatrix",
///     kind dyn IgesDataTransfEntity, { data: Handle<HArray2<f64>> }
/// }
/// ```
///
/// `parent P` embeds the concrete class `P`. `kind dyn K` embeds `IgesDataIgesEntity` and registers
/// the abstract kind `K`; the class then implements `K` by hand. `parent P, kinds [dyn K]`
/// embeds `P` and registers kinds `P` carries. `custom_default` before the class name leaves
/// `Default` and `new` to the class.
#[macro_export]
macro_rules! iges_entity {
    ($(#[$meta:meta])* $class:ident, $state:ident, $recorded:literal, parent $parent:ty, { $($field:ident: $ty:ty),* $(,)? }) => {
        $crate::iges_entity!(@shape $(#[$meta])* $class, $state, $recorded, $parent, $parent, [], { $($field: $ty),* }, [Default]);
        $crate::iges_entity!(@new $class, $recorded);
    };
    ($(#[$meta:meta])* $class:ident, $state:ident, $recorded:literal, parent $parent:ty, kinds [$($kind:ty),+ $(,)?], { $($field:ident: $ty:ty),* $(,)? }) => {
        $crate::iges_entity!(@shape $(#[$meta])* $class, $state, $recorded, $parent, $parent, [$($kind),+], { $($field: $ty),* }, [Default]);
        $crate::iges_entity!(@new $class, $recorded);
    };
    ($(#[$meta:meta])* $class:ident, $state:ident, $recorded:literal, kind $kind:ty, { $($field:ident: $ty:ty),* $(,)? }) => {
        $crate::iges_entity!(@shape $(#[$meta])* $class, $state, $recorded, $kind, $crate::iges_data::IgesDataIgesEntity, [$kind], { $($field: $ty),* }, [Default]);
        $crate::iges_entity!(@new $class, $recorded);
    };
    ($(#[$meta:meta])* custom_default $class:ident, $state:ident, $recorded:literal, parent $parent:ty, { $($field:ident: $ty:ty),* $(,)? }) => {
        $crate::iges_entity!(@shape $(#[$meta])* $class, $state, $recorded, $parent, $parent, [], { $($field: $ty),* }, []);
    };
    ($(#[$meta:meta])* custom_default $class:ident, $state:ident, $recorded:literal, kind $kind:ty, { $($field:ident: $ty:ty),* $(,)? }) => {
        $crate::iges_entity!(@shape $(#[$meta])* $class, $state, $recorded, $kind, $crate::iges_data::IgesDataIgesEntity, [$kind], { $($field: $ty),* }, []);
    };
    (@new $class:ident, $recorded:literal) => {
        impl $class {
            #[doc = concat!("A `", $recorded, "` with every field at its neutral value.")]
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
    (@shape $(#[$meta:meta])* $class:ident, $state:ident, $recorded:literal, $descriptor:ty, $embedded:ty, [$($kind:ty),*], { $($field:ident: $ty:ty),* }, [$($derive:ident)?]) => {
        #[doc = concat!("The fields of `", $recorded, "`.")]
        #[derive(Default)]
        #[allow(dead_code, reason = "a field can be write-only until a later package reads it")]
        pub(crate) struct $state {
            $(pub(crate) $field: $ty,)*
        }
        $(#[$meta])*
        $(#[derive($derive)])?
        pub struct $class {
            pub(crate) parent: $embedded,
            #[allow(dead_code, reason = "a class without own fields never reads its empty state")]
            pub(crate) state: martensite_tkernel::standard::SharedState<$state>,
        }
        martensite_tkernel::impl_transient!($class, $recorded, $descriptor, parent, interfaces: [$($kind),*]);
        impl std::ops::Deref for $class {
            type Target = $embedded;
            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
        impl $crate::iges_data::IgesDataEntityKind for $class {
            fn iges_entity(&self) -> &$crate::iges_data::IgesDataIgesEntity {
                $crate::iges_data::IgesDataEntityKind::iges_entity(&self.parent)
            }
        }
    };
}

/// The regular accessors of an entity class, one grammar line each, inside its `impl` block.
///
/// `get`: the field, cloned. `set`, `set_copy`: replaces the field, by reference or by value.
/// `len`: the array length, 0 for a null array. `len_checked`: the array length, a failure for a
/// null array. `item`: the array item at a rank.
#[macro_export]
macro_rules! iges_methods {
    () => {};
    (get $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("The `", stringify!($field), "` field.")]
        pub fn $name(&self) -> $ty {
            Clone::clone(&self.state.read().$field)
        }
        $crate::iges_methods!($($rest)*);
    };
    (set $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("Sets the `", stringify!($field), "` field.")]
        pub fn $name(&self, value: &$ty) {
            self.state.write().$field = value.clone();
        }
        $crate::iges_methods!($($rest)*);
    };
    (set_copy $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("Sets the `", stringify!($field), "` field.")]
        pub fn $name(&self, value: $ty) {
            self.state.write().$field = value;
        }
        $crate::iges_methods!($($rest)*);
    };
    (len $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The length of the `", stringify!($field), "` list, 0 when it is null.")]
        pub fn $name(&self) -> i32 {
            let items = self.state.read().$field.clone();
            $crate::iges_data::iges_array1_len(&items)
        }
        $crate::iges_methods!($($rest)*);
    };
    (len_checked $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The length of the `", stringify!($field), "` list.\n\n# Errors\nA null object failure when the list is null.")]
        pub fn $name(&self) -> Result<i32, martensite_tkernel::standard::Failure> {
            let items = self.state.read().$field.clone();
            Ok($crate::iges_data::iges_live(&items, stringify!($name))?.array1().length())
        }
        $crate::iges_methods!($($rest)*);
    };
    (item $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("The item at a rank from 1 of the `", stringify!($field), "` list.\n\n# Errors\nA null object failure for a null list, an out of range failure for a bad rank.")]
        pub fn $name(&self, num: i32) -> Result<$ty, martensite_tkernel::standard::Failure> {
            let items = self.state.read().$field.clone();
            $crate::iges_data::iges_array1_item(&items, num, stringify!($name))
        }
        $crate::iges_methods!($($rest)*);
    };
}
