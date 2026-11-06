//! The shared shape of the STEP entity classes.
//!
//! The entity packages hold over a thousand classes that repeat one shape: fields, an `init`, and
//! getters and setters. `step_entity!` declares a class (embedded parent, fields in one
//! `SharedState`, a runtime type) and `step_methods!` its regular accessors, one line each, so the
//! shape exists once. Classes are written by embedding: the parent
//! is a field and a `deref`, so `Handle<StepGeomCartesianPoint>` casts to `Handle<StepGeomPoint>`
//! and inherited members resolve through `deref`.
//!
//! A null array is the null dereference, a null object failure; an index out of its bounds
//! is an out of range failure.
use martensite_tkernel::ncollection::array2_failure;
pub(crate) use martensite_tkernel::ncollection::{
    Array1, DynamicArray, HArray1, HArray2, HSequence, ObjectHandle,
};
pub(crate) use martensite_tkernel::standard::StandardTransient;
use martensite_tkernel::standard::exceptions;
pub(crate) use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, SharedState};
pub(crate) use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
pub(crate) use martensite_tkmath::gp::Xyz;

/// The object of a handle, or the failure of the null dereference `where_called` makes.
pub(crate) fn step_live<'a, T: ?Sized + RuntimeTyped>(
    handle: &'a Handle<T>,
    where_called: &str,
) -> Result<&'a T, Failure> {
    handle
        .get()
        .ok_or_else(|| exceptions::null_object(&format!("{where_called}: null handle")))
}

/// `Value(index)` of a shared array.
pub(crate) fn step_array1_item<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray1<T>>,
    index: i32,
    where_called: &str,
) -> Result<T, Failure> {
    let items = step_live(array, where_called)?.array1();
    items
        .value(index)
        .cloned()
        .map_err(|error| error.into_failure(where_called))
}

/// `set_value(index, value)` of a shared array.
pub(crate) fn step_array1_set<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray1<T>>,
    index: i32,
    value: T,
    where_called: &str,
) -> Result<(), Failure> {
    let mut items = step_live(array, where_called)?.change_array1();
    items
        .set_value(index, value)
        .map_err(|error| error.into_failure(where_called))
}

/// `Value(row, column)` of a shared two-dimensional array.
pub(crate) fn step_array2_item<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray2<T>>,
    row: i32,
    column: i32,
    where_called: &str,
) -> Result<T, Failure> {
    let items = step_live(array, where_called)?.array2();
    items.value(row, column).cloned().map_err(array2_failure)
}

/// `set_value(row, column, value)` of a shared two-dimensional array.
pub(crate) fn step_array2_set<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray2<T>>,
    row: i32,
    column: i32,
    value: T,
    where_called: &str,
) -> Result<(), Failure> {
    let mut items = step_live(array, where_called)?.change_array2();
    items.set_value(row, column, value).map_err(array2_failure)
}

/// `new HArray1<T>(lower, upper)`.
pub(crate) fn step_new_array1<T: Default + Send + Sync + 'static>(
    lower: i32,
    upper: i32,
) -> Result<Handle<HArray1<T>>, Failure> {
    HArray1::with_bounds(lower, upper)
        .map(Handle::new)
        .map_err(|error| error.into_failure("HArray1"))
}

/// `new HArray2<T>(lower row, upper row, lower column, upper column)`.
pub(crate) fn step_new_array2<T: Default + Send + Sync + 'static>(
    lower_row: i32,
    upper_row: i32,
    lower_col: i32,
    upper_col: i32,
) -> Result<Handle<HArray2<T>>, Failure> {
    HArray2::with_bounds(lower_row, upper_row, lower_col, upper_col)
        .map(Handle::new)
        .map_err(array2_failure)
}

/// `Length()` of a shared array, 0 for a null array.
pub(crate) fn step_array1_len<T: Send + Sync + 'static>(array: &Handle<HArray1<T>>) -> i32 {
    array.get().map_or(0, |items| items.array1().length())
}

/// Declares an entity class: its state, the embedded parent, the runtime type, `new` and `deref`.
///
/// `custom_default` leaves `Default` and `new` to the class, for a constructor that does
/// more than zero its fields (the combined classes create their sub-entities in it).
macro_rules! step_entity {
    ($(#[$meta:meta])* $class:ident, $state:ident, $recorded:literal, $parent:ty, { $($field:ident: $ty:ty),* $(,)? }) => {
        $crate::step_entity::step_entity!(@shape $(#[$meta])* $class, $state, $recorded, $parent, { $($field: $ty),* }, [Default]);
        impl $class {
            #[doc = concat!("`", $recorded, "()`, with every field at its neutral value.")]
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
    ($(#[$meta:meta])* custom_default $class:ident, $state:ident, $recorded:literal, $parent:ty, { $($field:ident: $ty:ty),* $(,)? }) => {
        $crate::step_entity::step_entity!(@shape $(#[$meta])* $class, $state, $recorded, $parent, { $($field: $ty),* }, []);
    };
    (@shape $(#[$meta:meta])* $class:ident, $state:ident, $recorded:literal, $parent:ty, { $($field:ident: $ty:ty),* }, [$($derive:ident)?]) => {
        #[doc = concat!("The fields of `", $recorded, "`.")]
        #[derive(Default)]
        #[allow(dead_code, reason = "a field can be write-only until a later package reads it")]
        pub(crate) struct $state {
            $(pub(crate) $field: $ty,)*
        }
        $(#[$meta])*
        $(#[derive($derive)])?
        pub struct $class {
            pub(crate) parent: $parent,
            #[allow(dead_code, reason = "a class without own fields never reads its empty state")]
            pub(crate) state: $crate::step_entity::SharedState<$state>,
        }
        martensite_tkernel::impl_transient!($class, $recorded, $parent, parent);
        impl std::ops::Deref for $class {
            type Target = $parent;
            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
    };
}
pub(crate) use step_entity;

/// The regular accessors of an entity class, one grammar line each, inside its `impl` block.
///
/// `get`: `return field;`. `set`, `set_copy`: `field = value;` by reference or by value, with the
/// `flag` of an optional field raised. `unset`: lowers the flag and resets the field. `clear`:
/// lowers the flag only. `len`: the array length, 0 for a null array. `len_checked`: the array
/// length where the code dereferences a null array. `item`: `array->Value(num)`.
macro_rules! step_methods {
    () => {};
    (get $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("The `", stringify!($field), "` field.")]
        pub fn $name(&self) -> $ty {
            Clone::clone(&self.state.read().$field)
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (set $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("Sets the `", stringify!($field), "` field.")]
        pub fn $name(&self, value: &$ty) {
            self.state.write().$field = value.clone();
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (set $name:ident = $field:ident flag $flag:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("Sets the optional `", stringify!($field), "` field.")]
        pub fn $name(&self, value: &$ty) {
            let mut state = self.state.write();
            state.$field = value.clone();
            state.$flag = true;
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (set_copy $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("Sets the `", stringify!($field), "` field.")]
        pub fn $name(&self, value: $ty) {
            self.state.write().$field = value;
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (set_copy $name:ident = $field:ident flag $flag:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("Sets the optional `", stringify!($field), "` field.")]
        pub fn $name(&self, value: $ty) {
            let mut state = self.state.write();
            state.$field = value;
            state.$flag = true;
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (unset $name:ident = $field:ident flag $flag:ident; $($rest:tt)*) => {
        #[doc = concat!("Resets the optional `", stringify!($field), "` field.")]
        pub fn $name(&self) {
            let mut state = self.state.write();
            state.$flag = false;
            state.$field = Default::default();
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (clear $name:ident = $flag:ident; $($rest:tt)*) => {
        #[doc = concat!("Lowers the `", stringify!($flag), "` flag; the field keeps its value.")]
        pub fn $name(&self) {
            self.state.write().$flag = false;
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (len $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The length of the `", stringify!($field), "` list, 0 when it is null.")]
        pub fn $name(&self) -> i32 {
            let items = self.state.read().$field.clone();
            $crate::step_entity::step_array1_len(&items)
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (len_checked $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The length of the `", stringify!($field), "` list.\n\n# Errors\n`Standard_NullObject` when the list is null.")]
        pub fn $name(&self) -> Result<i32, $crate::step_entity::Failure> {
            let items = self.state.read().$field.clone();
            Ok($crate::step_entity::step_live(&items, stringify!($name))?.array1().length())
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
    (item $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("The item at a 1-based rank of the `", stringify!($field), "` list.\n\n# Errors\n`Standard_NullObject` for a null list, An out of range failure for a bad rank.")]
        pub fn $name(&self, num: i32) -> Result<$ty, $crate::step_entity::Failure> {
            let items = self.state.read().$field.clone();
            $crate::step_entity::step_array1_item(&items, num, stringify!($name))
        }
        $crate::step_entity::step_methods!($($rest)*);
    };
}
pub(crate) use step_methods;

/// Declares a SELECT type of entity types: the value class, its `case_num` in the order, and
/// one casting read per member type. `Value` is the stored entity, null when another type is held.
macro_rules! step_select {
    ($(#[$meta:meta])* $class:ident, [$($case:ty),+ $(,)?], { $($reader:ident: $target:ty),* $(,)? }) => {
        $crate::step_entity::step_select!(@shape $(#[$meta])* $class, { $($reader: $target),* });
        impl $crate::step_data::StepDataSelectType for $class {
            $crate::impl_step_select_storage!(value);
            fn case_num(&self, ent: &$crate::step_entity::Handle<$crate::step_entity::StandardTransient>) -> i32 {
                $crate::step_select_case!(ent; $($case),+)
            }
        }
    };
    // A `case_num` whose numbers are not simply 1 to n, tested in the order.
    ($(#[$meta:meta])* $class:ident, numbered [$($case:ty = $number:literal),+ $(,)?], { $($reader:ident: $target:ty),* $(,)? }) => {
        $crate::step_entity::step_select!(@shape $(#[$meta])* $class, { $($reader: $target),* });
        impl $crate::step_data::StepDataSelectType for $class {
            $crate::impl_step_select_storage!(value);
            fn case_num(&self, ent: &$crate::step_entity::Handle<$crate::step_entity::StandardTransient>) -> i32 {
                if ent.is_null() {
                    return 0;
                }
                $(if ent.is_kind(<$case as $crate::step_entity::RuntimeTyped>::static_type()) {
                    return $number;
                })+
                0
            }
        }
    };
    (@shape $(#[$meta:meta])* $class:ident, { $($reader:ident: $target:ty),* }) => {
        $(#[$meta])*
        #[derive(Clone, Default)]
        pub struct $class {
            value: $crate::step_data::StepDataSelectValue,
        }
        impl $class {
            /// A select holding nothing.
            pub fn new() -> Self {
                Self::default()
            }
            $(
                #[doc = concat!("The stored value as `", stringify!($target), "`, null when another type is held.")]
                pub fn $reader(&self) -> $crate::step_entity::Handle<$target> {
                    $crate::step_data::StepDataSelectType::value(self).cast::<$target>()
                }
            )*
        }
    };
}
pub(crate) use step_select;

/// Forwards the real value of a select member to its embedded `StepDataSelectReal`.
macro_rules! step_real_member_values {
    ($field:ident) => {
        fn kind(&self) -> i32 {
            $crate::step_data::StepDataSelectMember::kind(&self.$field)
        }
        fn real(&self) -> f64 {
            $crate::step_data::StepDataSelectMember::real(&self.$field)
        }
        fn set_real(&self, val: f64) {
            $crate::step_data::StepDataSelectMember::set_real(&self.$field, val);
        }
    };
}
pub(crate) use step_real_member_values;

/// Declares a select member over a real that has one fixed name, whatever name it is given.
macro_rules! step_fixed_name_member {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $name:literal) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $class {
            base: $crate::step_data::StepDataSelectReal,
        }
        martensite_tkernel::impl_transient!(
            $class,
            $recorded,
            $crate::step_data::StepDataSelectReal,
            base,
            interfaces: [dyn $crate::step_data::StepDataSelectMember]
        );
        impl $crate::step_data::StepDataSelectMember for $class {
            fn has_name(&self) -> bool {
                true
            }
            fn name(&self) -> String {
                $name.to_string()
            }
            fn set_name(&self, _name: &str) -> bool {
                true
            }
            $crate::step_entity::step_real_member_values!(base);
        }
        impl $class {
            #[doc = concat!("`", $recorded, "()`.")]
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
}
pub(crate) use step_fixed_name_member;
