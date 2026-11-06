//! The shapes that repeat among the select members and the SELECT types holding immediate values.
//!
//! Twelve member classes and seven immediate selects differ only in their name tables, so each
//! shape is declared once here: `step_case_name_member!`, `step_enum_member!`,
//! `step_immediate_select!`, `step_member_accessors!` and `step_derived_select!`.
use crate::step_data::{StepDataSelectMember, StepDataStepReaderData};
use crate::step_entity::*;
use martensite_tkernel::standard::{Transient, exceptions};

/// The first and last type names of a complex record, which the geometric tolerance readers test.
pub(crate) trait StepReaderComplexNames {
    /// `complex_type(num, types); types.First()`.
    fn first_complex_type(&self, num: i32) -> Result<Vec<u8>, Failure>;
    /// `complex_type(num, types); types.Last()`.
    fn last_complex_type(&self, num: i32) -> Result<Vec<u8>, Failure>;
    /// `complex_type(num, types); types.Value(3)`.
    fn third_complex_type(&self, num: i32) -> Result<Vec<u8>, Failure>;
}

impl StepReaderComplexNames for StepDataStepReaderData {
    fn first_complex_type(&self, num: i32) -> Result<Vec<u8>, Failure> {
        complex_type_name(self, num, |names| names.first())
    }
    fn last_complex_type(&self, num: i32) -> Result<Vec<u8>, Failure> {
        complex_type_name(self, num, |names| names.last())
    }
    fn third_complex_type(&self, num: i32) -> Result<Vec<u8>, Failure> {
        complex_type_name(self, num, |names| names.get(2))
    }
}

/// One name of a complex record; an empty sequence is the a no such object failure.
fn complex_type_name(
    data: &StepDataStepReaderData,
    num: i32,
    pick: impl FnOnce(&[AsciiString]) -> Option<&AsciiString>,
) -> Result<Vec<u8>, Failure> {
    let mut names = Vec::new();
    data.complex_type(num, &mut names);
    pick(&names)
        .map(|name| name.as_bytes().to_vec())
        .ok_or_else(|| exceptions::no_such_object(&format!("record {num} has no complex type")))
}

/// `compare_names`: the case (from 1) of a name in a member's table, 0 for empty or unknown.
/// `step_member_case(&["A"], "A") == 1`.
pub(crate) fn step_member_case(names: &[&str], name: &str) -> usize {
    if name.is_empty() {
        return 0;
    }
    names
        .iter()
        .position(|known| *known == name)
        .map_or(0, |rank| rank + 1)
}

/// `case_mem`: the rank (from 1) of the first name the member matches, 0 for a null member or none.
pub(crate) fn step_select_case_mem(ent: &Handle<dyn StepDataSelectMember>, names: &[&str]) -> i32 {
    let Some(member) = ent.get() else {
        return 0;
    };
    names
        .iter()
        .position(|name| member.matches(name))
        .map_or(0, |rank| rank as i32 + 1)
}

/// The immediate setter: names the stored member and writes it; nothing when the stored
/// value is not an `M`.
pub(crate) fn step_member_store<M>(
    value: &Handle<StandardTransient>,
    name: &str,
    write: impl FnOnce(&M),
) where
    M: StepDataSelectMember + RuntimeTyped + Transient,
{
    let member = value.cast::<M>();
    if let Some(live) = member.get() {
        live.set_name(name);
        write(live);
    }
}

/// The immediate getter: reads the stored member when it is an `M` named `name`.
pub(crate) fn step_member_load<M, R>(
    value: &Handle<StandardTransient>,
    name: &str,
    read: impl FnOnce(&M) -> R,
) -> Option<R>
where
    M: StepDataSelectMember + RuntimeTyped + Transient,
{
    let member = value.cast::<M>();
    member.get().filter(|live| live.name() == name).map(read)
}

/// The string setter: a null `val` is the null dereference, reached only when a
/// member is stored.
pub(crate) fn step_member_store_string<M>(
    value: &Handle<StandardTransient>,
    name: &str,
    val: &Handle<HAsciiString>,
    where_called: &str,
) -> Result<(), Failure>
where
    M: StepDataSelectMember + RuntimeTyped + Transient,
{
    let member = value.cast::<M>();
    let Some(live) = member.get() else {
        return Ok(());
    };
    live.set_name(name);
    let text = step_live(val, where_called)?;
    live.set_string(text.string().as_bytes());
    Ok(())
}

/// The string getter: a new string, null unless an `M` named `name` is stored.
pub(crate) fn step_member_load_string<M>(
    value: &Handle<StandardTransient>,
    name: &str,
) -> Handle<HAsciiString>
where
    M: StepDataSelectMember + RuntimeTyped + Transient,
{
    step_member_load::<M, _>(value, name, |live| {
        Handle::new(HAsciiString::from_string(&live.string()))
    })
    .unwrap_or_default()
}

/// Forwards a member's stored values to its embedded storage.
macro_rules! step_member_values {
    ($field:ident) => {
        fn set_kind(&self, kind: i32) {
            $crate::step_data::StepDataSelectMember::set_kind(&self.$field, kind);
        }
        fn int(&self) -> i32 {
            $crate::step_data::StepDataSelectMember::int(&self.$field)
        }
        fn set_int(&self, val: i32) {
            $crate::step_data::StepDataSelectMember::set_int(&self.$field, val);
        }
        fn real(&self) -> f64 {
            $crate::step_data::StepDataSelectMember::real(&self.$field)
        }
        fn set_real(&self, val: f64) {
            $crate::step_data::StepDataSelectMember::set_real(&self.$field, val);
        }
        fn string(&self) -> martensite_tkernel::tcollection::AsciiString {
            $crate::step_data::StepDataSelectMember::string(&self.$field)
        }
        fn set_string(&self, val: &[u8]) {
            $crate::step_data::StepDataSelectMember::set_string(&self.$field, val);
        }
    };
}
pub(crate) use step_member_values;

/// A member whose name is a case in a table, recorded as a number and not stored in its base.
/// Quirk kept: `matches` of an unknown name is true while no case is set.
macro_rules! step_case_name_member {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $base:ty, [$($name:literal),+ $(,)?]) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $class {
            base: $base,
            mycase: $crate::step_entity::SharedState<usize>,
        }
        martensite_tkernel::impl_transient!(
            $class,
            $recorded,
            $base,
            base,
            interfaces: [dyn $crate::step_data::StepDataSelectMember]
        );
        impl std::ops::Deref for $class {
            type Target = $base;
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }
        impl $class {
            const NAMES: &'static [&'static str] = &[$($name),+];
            #[doc = concat!("`", $recorded, "()`: no case yet.")]
            pub fn new() -> Self {
                Self::default()
            }
            /// The `compare_names`.
            fn compare_names(name: &str) -> usize {
                $crate::step_members::step_member_case(Self::NAMES, name)
            }
        }
        impl $crate::step_data::StepDataSelectMember for $class {
            fn has_name(&self) -> bool {
                *self.mycase.read() > 0
            }
            fn name(&self) -> String {
                let case = *self.mycase.read();
                case.checked_sub(1)
                    .and_then(|rank| Self::NAMES.get(rank))
                    .map_or_else(String::new, ToString::to_string)
            }
            fn set_name(&self, name: &str) -> bool {
                let case = Self::compare_names(name);
                *self.mycase.write() = case;
                case > 0
            }
            fn matches(&self, name: &str) -> bool {
                *self.mycase.read() == Self::compare_names(name)
            }
            fn kind(&self) -> i32 {
                $crate::step_data::StepDataSelectMember::kind(&self.base)
            }
            $crate::step_members::step_member_values!(base);
        }
    };
}
pub(crate) use step_case_name_member;

/// A member over an integer with one fixed name, whose value is an enumeration written through a
/// text table. `kind` is `Some` when the class fixes it, `None` when it is stored.
macro_rules! step_enum_member {
    ($(#[$meta:meta])* $class:ident, $recorded:literal, $enum:ty, $name:literal, kind: $kind:expr, [$($variant:ident = $text:literal),+ $(,)?]) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $class {
            base: $crate::step_data::StepDataSelectInt,
        }
        martensite_tkernel::impl_transient!(
            $class,
            $recorded,
            $crate::step_data::StepDataSelectInt,
            base,
            interfaces: [dyn $crate::step_data::StepDataSelectMember]
        );
        impl $class {
            const VARIANTS: &'static [$enum] = &[$(<$enum>::$variant),+];
            fn enum_tool() -> &'static $crate::step_data::StepDataEnumTool {
                static TOOL: std::sync::LazyLock<$crate::step_data::StepDataEnumTool> =
                    std::sync::LazyLock::new(|| $crate::step_data::StepDataEnumTool::new(&[$($text),+]));
                &TOOL
            }
            #[doc = concat!("`", $recorded, "()`.")]
            pub fn new() -> Self {
                Self::default()
            }
            /// `set_value`: stores the enumerator's rank.
            pub fn set_value(&self, val: $enum) {
                $crate::step_data::StepDataSelectMember::set_int(self, val as i32);
            }
            /// `Value`: the enumerator of the stored rank; the first one out of range, where the
            /// cast has no meaning.
            pub fn value(&self) -> $enum {
                let rank = $crate::step_data::StepDataSelectMember::int(self);
                usize::try_from(rank)
                    .ok()
                    .and_then(|rank| Self::VARIANTS.get(rank).copied())
                    .unwrap_or_default()
            }
        }
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
            fn kind(&self) -> i32 {
                let fixed: Option<i32> = $kind;
                fixed.unwrap_or_else(|| $crate::step_data::StepDataSelectMember::kind(&self.base))
            }
            fn enum_text(&self) -> martensite_tkernel::tcollection::AsciiString {
                Self::enum_tool().text($crate::step_data::StepDataSelectMember::int(self))
            }
            fn set_enum_text(&self, _val: i32, text: &[u8]) {
                let rank = Self::enum_tool().value(text);
                if rank >= 0 {
                    $crate::step_data::StepDataSelectMember::set_int(self, rank);
                }
            }
            $crate::step_members::step_member_values!(base);
        }
    };
}
pub(crate) use step_enum_member;

/// A SELECT with no entity type and immediate values through one member class.
macro_rules! step_immediate_select {
    ($(#[$meta:meta])* $class:ident, $member:ty, [$($name:literal),+ $(,)?]) => {
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
        }
        impl $crate::step_data::StepDataSelectType for $class {
            $crate::impl_step_select_storage!(value);
            fn case_num(&self, _ent: &$crate::step_entity::Handle<$crate::step_entity::StandardTransient>) -> i32 {
                0
            }
            fn new_member(&self) -> $crate::step_entity::Handle<dyn $crate::step_data::StepDataSelectMember> {
                $crate::step_entity::Handle::new(<$member>::new()).cast()
            }
            fn case_mem(&self, ent: &$crate::step_entity::Handle<dyn $crate::step_data::StepDataSelectMember>) -> i32 {
                $crate::step_members::step_select_case_mem(ent, &[$($name),+])
            }
        }
    };
}
pub(crate) use step_immediate_select;

/// The immediate accessors of a select, one grammar line each, inside its `impl` block. A
/// setter does nothing without a stored member of this class, and a getter gives its default when
/// another name is stored. `enum` tables are read from 1 while the setter stores the rank from 0, as
///. `array` setters only name the member and their getters give null: the bodies
/// have the array commented out.
macro_rules! step_member_accessors {
    ($member:ty;) => {};
    ($member:ty; int $set:ident, $get:ident = $name:literal; $($rest:tt)*) => {
        #[doc = concat!("Stores the integer named `", $name, "`.")]
        pub fn $set(&self, val: i32) {
            $crate::step_members::step_member_store::<$member>(&self.value, $name, |live| {
                $crate::step_data::StepDataSelectMember::set_integer(live, val)
            });
        }
        #[doc = concat!("The integer named `", $name, "`, 0 when another is stored.")]
        pub fn $get(&self) -> i32 {
            $crate::step_members::step_member_load::<$member, _>(&self.value, $name, |live| {
                $crate::step_data::StepDataSelectMember::integer(live)
            })
            .unwrap_or(0)
        }
        $crate::step_members::step_member_accessors!($member; $($rest)*);
    };
    ($member:ty; real $set:ident, $get:ident = $name:literal; $($rest:tt)*) => {
        #[doc = concat!("Stores the real named `", $name, "`.")]
        pub fn $set(&self, val: f64) {
            $crate::step_members::step_member_store::<$member>(&self.value, $name, |live| {
                $crate::step_data::StepDataSelectMember::set_real(live, val)
            });
        }
        #[doc = concat!("The real named `", $name, "`, 0 when another is stored.")]
        pub fn $get(&self) -> f64 {
            $crate::step_members::step_member_load::<$member, _>(&self.value, $name, |live| {
                $crate::step_data::StepDataSelectMember::real(live)
            })
            .unwrap_or(0.0)
        }
        $crate::step_members::step_member_accessors!($member; $($rest)*);
    };
    ($member:ty; string $set:ident, $get:ident = $name:literal; $($rest:tt)*) => {
        #[doc = concat!("Stores the string named `", $name, "`.")]
        ///
        /// # Errors
        /// a null object failure for a null string while a member is stored.
        pub fn $set(&self, val: &$crate::step_entity::Handle<$crate::step_entity::HAsciiString>) -> Result<(), $crate::step_entity::Failure> {
            $crate::step_members::step_member_store_string::<$member>(&self.value, $name, val, stringify!($set))
        }
        #[doc = concat!("A copy of the string named `", $name, "`, null when another is stored.")]
        pub fn $get(&self) -> $crate::step_entity::Handle<$crate::step_entity::HAsciiString> {
            $crate::step_members::step_member_load_string::<$member>(&self.value, $name)
        }
        $crate::step_members::step_member_accessors!($member; $($rest)*);
    };
    ($member:ty; enum $set:ident, $get:ident: $enum:ty = $name:literal, [$($variant:ident),+ $(,)?], default $default:ident; $($rest:tt)*) => {
        #[doc = concat!("Stores the enumerator named `", $name, "`.")]
        pub fn $set(&self, val: $enum) {
            $crate::step_members::step_member_store::<$member>(&self.value, $name, |live| {
                $crate::step_data::StepDataSelectMember::set_enum(live, val as i32, b"")
            });
        }
        #[doc = concat!("The enumerator named `", $name, "`, the default when another is stored.")]
        pub fn $get(&self) -> $enum {
            const TABLE: &[$enum] = &[$(<$enum>::$variant),+];
            $crate::step_members::step_member_load::<$member, _>(&self.value, $name, |live| {
                let number = $crate::step_data::StepDataSelectMember::enum_value(live);
                usize::try_from(number - 1).ok().and_then(|rank| TABLE.get(rank).copied())
            })
            .flatten()
            .unwrap_or(<$enum>::$default)
        }
        $crate::step_members::step_member_accessors!($member; $($rest)*);
    };
    ($member:ty; array $set:ident, $get:ident = $name:literal; $($rest:tt)*) => {
        #[doc = concat!("Names the stored member `", $name, "`; the array itself is dropped.")]
        pub fn $set(&self, _val: &$crate::step_entity::Handle<$crate::step_entity::HArray1<f64>>) {
            $crate::step_members::step_member_store::<$member>(&self.value, $name, |_| {});
        }
        #[doc = concat!("Null, whichever member is stored (`", $name, "`).")]
        pub fn $get(&self) -> $crate::step_entity::Handle<$crate::step_entity::HArray1<f64>> {
            $crate::step_entity::Handle::default()
        }
        $crate::step_members::step_member_accessors!($member; $($rest)*);
    };
}
pub(crate) use step_member_accessors;

/// A SELECT that derives from another SELECT: it answers its own `case_num`, keeps the
/// parent's readers through `deref` and adds or shadows readers.
macro_rules! step_derived_select {
    ($(#[$meta:meta])* $class:ident, $parent:ty, [$($case:ty),+ $(,)?], { $($reader:ident: $target:ty),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Default)]
        pub struct $class {
            parent: $parent,
        }
        impl std::ops::Deref for $class {
            type Target = $parent;
            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
        impl $crate::step_data::StepDataSelectType for $class {
            fn select_value(&self) -> &$crate::step_data::StepDataSelectValue {
                $crate::step_data::StepDataSelectType::select_value(&self.parent)
            }
            fn select_value_mut(&mut self) -> &mut $crate::step_data::StepDataSelectValue {
                $crate::step_data::StepDataSelectType::select_value_mut(&mut self.parent)
            }
            fn case_num(&self, ent: &$crate::step_entity::Handle<$crate::step_entity::StandardTransient>) -> i32 {
                $crate::step_select_case!(ent; $($case),+)
            }
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
pub(crate) use step_derived_select;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_case_is_from_one_and_zero_for_empty_or_unknown() {
        assert_eq!(step_member_case(&["A", "B"], "B"), 2);
        assert_eq!(step_member_case(&["A", "B"], ""), 0);
        assert_eq!(step_member_case(&["A", "B"], "C"), 0);
    }

    #[test]
    fn case_mem_of_a_null_member_is_zero() {
        assert_eq!(step_select_case_mem(&Handle::default(), &["A"]), 0);
    }
}
