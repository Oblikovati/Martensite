//! The select members of `StepVisual`, the marker select, and the two value classes whose names
//! say select although they hold plain numbers.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::impl_step_select_storage;
use crate::step_data::{StepDataSelectMember, StepDataSelectType, StepDataSelectValue};
use crate::step_entity::*;
use crate::step_geom::StepGeomPlane;
use crate::step_members::{step_enum_member, step_select_case_mem};
use crate::step_repr::StepReprRepresentationItem;
use martensite_tkxsbase::interface::InterfaceParamType;

step_enum_member!(
    /// `StepVisualMarkerMember`: the marker type, named `MARKER_TYPE`.
    StepVisualMarkerMember,
    "StepVisualMarkerMember",
    StepVisualMarkerType,
    "MARKER_TYPE",
    kind: None,
    [
        MtDot = ".DOT.",
        MtX = ".X.",
        MtPlus = ".PLUS.",
        MtAsterisk = ".ASTERISK.",
        MtRing = ".RING.",
        MtSquare = ".SQUARE.",
        MtTriangle = ".TRIANGLE.",
    ]
);

step_enum_member!(
    /// `StepVisualNullStyleMember`: the null style, named `NULL_STYLE`, of kind enumeration.
    StepVisualNullStyleMember,
    "StepVisualNullStyleMember",
    StepVisualNullStyle,
    "NULL_STYLE",
    kind: Some(4),
    [Null = ".NULL."]
);

/// `StepVisualMarkerSelect`: only the immediate marker type.
#[derive(Clone, Default)]
pub struct StepVisualMarkerSelect {
    value: StepDataSelectValue,
}

impl StepDataSelectType for StepVisualMarkerSelect {
    impl_step_select_storage!(value);
    fn case_num(&self, _ent: &Handle<StandardTransient>) -> i32 {
        0
    }
    fn new_member(&self) -> Handle<dyn StepDataSelectMember> {
        Handle::new(StepVisualMarkerMember::new()).cast()
    }
    /// An undefined value is accepted too, besides an enumeration.
    fn case_mem(&self, ent: &Handle<dyn StepDataSelectMember>) -> i32 {
        let Some(member) = ent.get() else {
            return 0;
        };
        let kind = member.param_type();
        if kind != InterfaceParamType::ParamVoid && kind != InterfaceParamType::ParamEnum {
            return 0;
        }
        step_select_case_mem(ent, &["MARKER_TYPE"])
    }
}

impl StepVisualMarkerSelect {
    /// `StepVisualMarkerSelect()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `marker_member`: null when no marker member is held.
    pub fn marker_member(&self) -> Handle<StepVisualMarkerMember> {
        self.value().cast::<StepVisualMarkerMember>()
    }
}

impl StepVisualStyledItem {
    /// `set_item(RepresentationItem)`: the same item under both fields.
    pub fn set_item(&self, a_item: &Handle<StepReprRepresentationItem>) {
        let mut state = self.state.write();
        state.my_item = a_item.cast();
        state.my_repr_item = a_item.clone();
    }
    /// `set_item(StyledItemTarget)`: the second field is null when the target is no representation
    /// item.
    pub fn set_item_styled_item_target(&self, the_item: &StepVisualStyledItemTarget) {
        let target = the_item.value();
        let mut state = self.state.write();
        state.my_repr_item = target.cast::<StepReprRepresentationItem>();
        state.my_item = target;
    }
    /// `item_ap242`: the item as a target, holding nothing when it is not a kind of target.
    ///
    /// # Errors
    /// Never in practice: the value set was just recognized by the same target.
    pub fn item_ap242(&self) -> Result<StepVisualStyledItemTarget, Failure> {
        let mut target = StepVisualStyledItemTarget::new();
        let item = self.state.read().my_item.clone();
        if target.case_num(&item) > 0 {
            target.set_value(&item)?;
        }
        Ok(target)
    }
}

/// A clipping select: a plane of any kind, or exactly the other clipping class. The second
/// case tests `is_instance`, so a subclass of it is refused.
macro_rules! step_clipping_select {
    ($(#[$meta:meta])* $class:ident, $clipping:ty, $reader:ident) => {
        $(#[$meta])*
        #[derive(Clone, Default)]
        pub struct $class {
            value: StepDataSelectValue,
        }
        impl StepDataSelectType for $class {
            impl_step_select_storage!(value);
            fn case_num(&self, ent: &Handle<StandardTransient>) -> i32 {
                if ent.is_kind(<StepGeomPlane as RuntimeTyped>::static_type()) {
                    return 1;
                }
                i32::from(ent.is_instance(<$clipping as RuntimeTyped>::static_type())) * 2
            }
        }
        impl $class {
            /// A select holding nothing.
            pub fn new() -> Self {
                Self::default()
            }
            /// `Plane`: null when another type is held.
            pub fn plane(&self) -> Handle<StepGeomPlane> {
                self.value().cast::<StepGeomPlane>()
            }
            /// The stored clipping, null when another type is held.
            pub fn $reader(&self) -> Handle<$clipping> {
                self.value().cast::<$clipping>()
            }
        }
    };
}

step_clipping_select!(
    /// Camera model D 3 multi clipping interection select.
    StepVisualCameraModelD3MultiClippingInterectionSelect,
    StepVisualCameraModelD3MultiClippingUnion,
    camera_model_d3_multi_clipping_union
);

step_clipping_select!(
    /// Camera model D 3 multi clipping union select.
    StepVisualCameraModelD3MultiClippingUnionSelect,
    StepVisualCameraModelD3MultiClippingIntersection,
    camera_model_d3_multi_clipping_intersection
);

/// `StepVisualBoxCharacteristicSelect`: a real and a free number for the kind of content.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StepVisualBoxCharacteristicSelect {
    the_real_value: f64,
    the_type_of_content: i32,
}

impl StepVisualBoxCharacteristicSelect {
    /// `StepVisualBoxCharacteristicSelect()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `type_of_content`.
    pub fn type_of_content(&self) -> i32 {
        self.the_type_of_content
    }
    /// `set_type_of_content`.
    pub fn set_type_of_content(&mut self, a_type: i32) {
        self.the_type_of_content = a_type;
    }
    /// `real_value`.
    pub fn real_value(&self) -> f64 {
        self.the_real_value
    }
    /// `set_real_value`.
    pub fn set_real_value(&mut self, a_value: f64) {
        self.the_real_value = a_value;
    }
}

/// `StepVisualDirectionCountSelect`: the U or the V direction count, recorded by content number.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StepVisualDirectionCountSelect {
    the_u_direction_count: i32,
    the_v_direction_count: i32,
    the_type_of_content: i32,
}

impl StepVisualDirectionCountSelect {
    /// `StepVisualDirectionCountSelect()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `set_type_of_content`.
    pub fn set_type_of_content(&mut self, a_type_of_content: i32) {
        self.the_type_of_content = a_type_of_content;
    }
    /// `type_of_content`.
    pub fn type_of_content(&self) -> i32 {
        self.the_type_of_content
    }
    /// `UDirectionCount`.
    pub fn u_direction_count(&self) -> i32 {
        self.the_u_direction_count
    }
    /// `set_u_direction_count`: also records content 1.
    pub fn set_u_direction_count(&mut self, a_u_direction_count: i32) {
        self.the_u_direction_count = a_u_direction_count;
        self.the_type_of_content = 1;
    }
    /// `VDirectionCount`. It returns the U count, so the V count is never read.
    pub fn v_direction_count(&self) -> i32 {
        self.the_u_direction_count
    }
    /// `set_v_direction_count`: also records content 2.
    pub fn set_v_direction_count(&mut self, a_v_direction_count: i32) {
        self.the_v_direction_count = a_v_direction_count;
        self.the_type_of_content = 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_member_round_trips_its_enumerator_and_text() {
        let member = StepVisualMarkerMember::new();
        member.set_value(StepVisualMarkerType::MtRing);
        assert_eq!(member.value(), StepVisualMarkerType::MtRing);
        assert_eq!(member.enum_text().as_bytes(), b".RING.");
        member.set_enum_text(0, b".PLUS.");
        assert_eq!(member.value(), StepVisualMarkerType::MtPlus);
        member.set_enum_text(0, b".UNKNOWN.");
        assert_eq!(member.value(), StepVisualMarkerType::MtPlus);
        assert_eq!(member.name(), "MARKER_TYPE");
        assert!(member.has_name() && member.set_name("ANY"));
    }

    #[test]
    fn null_style_member_has_the_enumeration_kind() {
        let member = StepVisualNullStyleMember::new();
        assert_eq!(member.kind(), 4);
        assert_eq!(member.enum_text().as_bytes(), b".NULL.");
    }

    #[test]
    fn marker_select_accepts_only_a_marker_named_member() {
        let select = StepVisualMarkerSelect::new();
        assert_eq!(select.case_num(&Handle::default()), 0);
        assert_eq!(select.case_mem(&Handle::default()), 0);
        let member = select.new_member();
        assert_eq!(select.case_mem(&member), 1);
    }

    #[test]
    fn direction_count_records_the_content_and_answers_v_with_u() {
        let mut select = StepVisualDirectionCountSelect::new();
        select.set_u_direction_count(4);
        assert_eq!(select.type_of_content(), 1);
        select.set_v_direction_count(9);
        assert_eq!(select.type_of_content(), 2);
        assert_eq!(select.v_direction_count(), 4);
    }

    #[test]
    fn styled_item_keeps_a_representation_item_under_both_fields() {
        let styled = StepVisualStyledItem::new();
        let item: Handle<StepReprRepresentationItem> = Handle::new(StepGeomPlane::new()).cast();
        styled.set_item(&item);
        assert_eq!(styled.item().object_address(), item.object_address());
        assert_eq!(
            styled.item_ap242().unwrap().value().object_address(),
            item.object_address()
        );
    }

    #[test]
    fn styled_item_without_an_item_gives_an_empty_target() {
        assert!(StepVisualStyledItem::new().item_ap242().unwrap().is_null());
    }

    #[test]
    fn clipping_select_recognizes_a_plane_and_exactly_its_clipping_class() {
        let select = StepVisualCameraModelD3MultiClippingUnionSelect::new();
        assert_eq!(select.case_num(&Handle::default()), 0);
        let plane: Handle<StandardTransient> = Handle::new(StepGeomPlane::new()).cast();
        assert_eq!(select.case_num(&plane), 1);
        let clipping: Handle<StandardTransient> =
            Handle::new(StepVisualCameraModelD3MultiClippingIntersection::new()).cast();
        assert_eq!(select.case_num(&clipping), 2);
        let other: Handle<StandardTransient> =
            Handle::new(StepVisualCameraModelD3MultiClippingUnion::new()).cast();
        assert_eq!(select.case_num(&other), 0);
    }

    #[test]
    fn box_characteristic_keeps_its_two_numbers() {
        let mut select = StepVisualBoxCharacteristicSelect::new();
        select.set_real_value(1.5);
        select.set_type_of_content(3);
        assert_eq!((select.real_value(), select.type_of_content()), (1.5, 3));
    }
}
