//! The select members of `StepElement` and the selects holding them.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_data::StepDataSelectNamed;
use crate::step_members::{step_case_name_member, step_immediate_select, step_member_accessors};

step_case_name_member!(
    /// Curve element freedom member.
    StepElementCurveElementFreedomMember,
    "StepElementCurveElementFreedomMember",
    StepDataSelectNamed,
    ["ENUMERATED_CURVE_ELEMENT_FREEDOM", "APPLICATION_DEFINED_DEGREE_OF_FREEDOM"]
);

step_case_name_member!(
    /// Curve element purpose member.
    StepElementCurveElementPurposeMember,
    "StepElementCurveElementPurposeMember",
    StepDataSelectNamed,
    ["ENUMERATED_CURVE_ELEMENT_PURPOSE", "APPLICATION_DEFINED_ELEMENT_PURPOSE"]
);

step_case_name_member!(
    /// Surface element purpose member.
    StepElementSurfaceElementPurposeMember,
    "StepElementSurfaceElementPurposeMember",
    StepDataSelectNamed,
    ["ENUMERATED_SURFACE_ELEMENT_PURPOSE", "APPLICATION_DEFINED_ELEMENT_PURPOSE"]
);

step_case_name_member!(
    /// Volume element purpose member.
    StepElementVolumeElementPurposeMember,
    "StepElementVolumeElementPurposeMember",
    StepDataSelectNamed,
    ["ENUMERATED_VOLUME_ELEMENT_PURPOSE", "APPLICATION_DEFINED_ELEMENT_PURPOSE"]
);

step_case_name_member!(
    /// Measure or unspecified value member.
    StepElementMeasureOrUnspecifiedValueMember,
    "StepElementMeasureOrUnspecifiedValueMember",
    StepDataSelectNamed,
    ["CONTEXT_DEPENDENT_MEASURE", "UNSPECIFIED_VALUE"]
);

step_case_name_member!(
    /// Element aspect member.
    StepElementElementAspectMember,
    "StepElementElementAspectMember",
    StepDataSelectNamed,
    [
        "ELEMENT_VOLUME",
        "VOLUME_3D_FACE",
        "VOLUME_2D_FACE",
        "VOLUME_3D_EDGE",
        "VOLUME_2D_EDGE",
        "SURFACE_3D_FACE",
        "SURFACE_2D_FACE",
        "SURFACE_3D_EDGE",
        "SURFACE_2D_EDGE",
        "CURVE_EDGE",
    ]
);

step_immediate_select!(
    /// Curve element freedom.
    StepElementCurveElementFreedom,
    StepElementCurveElementFreedomMember,
    ["ENUMERATED_CURVE_ELEMENT_FREEDOM", "APPLICATION_DEFINED_DEGREE_OF_FREEDOM"]
);

impl StepElementCurveElementFreedom {
    step_member_accessors! {
        StepElementCurveElementFreedomMember;
        enum set_enumerated_curve_element_freedom, enumerated_curve_element_freedom: StepElementEnumeratedCurveElementFreedom =
            "ENUMERATED_CURVE_ELEMENT_FREEDOM",
            [XTranslation, YTranslation, ZTranslation, XRotation, YRotation, ZRotation, Warp, None],
            default None;
        string set_application_defined_degree_of_freedom, application_defined_degree_of_freedom =
            "APPLICATION_DEFINED_DEGREE_OF_FREEDOM";
    }
}

step_immediate_select!(
    /// Measure or unspecified value.
    StepElementMeasureOrUnspecifiedValue,
    StepElementMeasureOrUnspecifiedValueMember,
    ["CONTEXT_DEPENDENT_MEASURE", "UNSPECIFIED_VALUE"]
);

impl StepElementMeasureOrUnspecifiedValue {
    step_member_accessors! {
        StepElementMeasureOrUnspecifiedValueMember;
        real set_context_dependent_measure, context_dependent_measure = "CONTEXT_DEPENDENT_MEASURE";
        enum set_unspecified_value, unspecified_value: StepElementUnspecifiedValue =
            "UNSPECIFIED_VALUE", [Unspecified], default Unspecified;
    }
}

step_immediate_select!(
    /// Element aspect.
    StepElementElementAspect,
    StepElementElementAspectMember,
    [
        "ELEMENT_VOLUME",
        "VOLUME_3D_FACE",
        "VOLUME_2D_FACE",
        "VOLUME_3D_EDGE",
        "VOLUME_2D_EDGE",
        "SURFACE_3D_FACE",
        "SURFACE_2D_FACE",
        "SURFACE_3D_EDGE",
        "SURFACE_2D_EDGE",
        "CURVE_EDGE",
    ]
);

impl StepElementElementAspect {
    step_member_accessors! {
        StepElementElementAspectMember;
        enum set_element_volume, element_volume: StepElementElementVolume = "ELEMENT_VOLUME", [Volume], default Volume;
        int set_volume3d_face, volume3d_face = "VOLUME_3D_FACE";
        int set_volume2d_face, volume2d_face = "VOLUME_2D_FACE";
        int set_volume3d_edge, volume3d_edge = "VOLUME_3D_EDGE";
        int set_volume2d_edge, volume2d_edge = "VOLUME_2D_EDGE";
        int set_surface3d_face, surface3d_face = "SURFACE_3D_FACE";
        int set_surface2d_face, surface2d_face = "SURFACE_2D_FACE";
        int set_surface3d_edge, surface3d_edge = "SURFACE_3D_EDGE";
        int set_surface2d_edge, surface2d_edge = "SURFACE_2D_EDGE";
        enum set_curve_edge, curve_edge: StepElementCurveEdge = "CURVE_EDGE", [ElementEdge], default ElementEdge;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::step_data::{StepDataSelectMember, StepDataSelectType};
    use crate::step_entity::*;

    fn aspect_named(name: &str) -> StepElementElementAspect {
        let mut select = StepElementElementAspect::new();
        let member = Handle::new(StepElementElementAspectMember::new());
        assert!(member.set_name(name));
        select.set_value(&member.cast()).unwrap();
        select
    }

    #[test]
    fn aspect_integer_is_read_only_under_its_own_name() {
        let select = aspect_named("VOLUME_3D_FACE");
        select.set_surface2d_edge(5);
        assert_eq!(select.surface2d_edge(), 5);
        assert_eq!(select.volume3d_face(), 0);
        select.set_volume3d_face(7);
        assert_eq!(select.volume3d_face(), 7);
        assert_eq!(select.surface2d_edge(), 0);
    }

    #[test]
    fn setters_do_nothing_without_a_stored_member() {
        let select = StepElementElementAspect::new();
        select.set_volume2d_face(3);
        assert_eq!(select.volume2d_face(), 0);
        assert_eq!(select.curve_edge(), StepElementCurveEdge::ElementEdge);
    }

    #[test]
    fn enumerated_freedom_reads_numbers_from_one() {
        let mut select = StepElementCurveElementFreedom::new();
        let member = Handle::new(StepElementCurveElementFreedomMember::new());
        member.set_name("ENUMERATED_CURVE_ELEMENT_FREEDOM");
        select.set_value(&member.cast()).unwrap();
        select.set_enumerated_curve_element_freedom(
            StepElementEnumeratedCurveElementFreedom::YTranslation,
        );
        assert_eq!(member.enum_value(), 1);
        assert_eq!(
            select.enumerated_curve_element_freedom(),
            StepElementEnumeratedCurveElementFreedom::XTranslation
        );
        select.set_enumerated_curve_element_freedom(
            StepElementEnumeratedCurveElementFreedom::XTranslation,
        );
        assert_eq!(
            select.enumerated_curve_element_freedom(),
            StepElementEnumeratedCurveElementFreedom::None
        );
    }

    /// `compare_names` of the surface and volume members gives case 1 to both names, so the
    /// second is written back as the first; here each name keeps its case, as the curve member's does.
    #[test]
    fn each_purpose_member_keeps_the_name_it_was_given() {
        const APPLICATION: &str = "APPLICATION_DEFINED_ELEMENT_PURPOSE";
        let surface = StepElementSurfaceElementPurposeMember::new();
        assert!(surface.set_name(APPLICATION));
        assert_eq!(surface.name(), APPLICATION);
        assert!(!surface.matches("ENUMERATED_SURFACE_ELEMENT_PURPOSE"));
        assert!(surface.set_name("ENUMERATED_SURFACE_ELEMENT_PURPOSE"));
        assert_eq!(surface.name(), "ENUMERATED_SURFACE_ELEMENT_PURPOSE");
        let volume = StepElementVolumeElementPurposeMember::new();
        assert!(volume.set_name(APPLICATION));
        assert_eq!(volume.name(), APPLICATION);
        assert!(volume.matches(APPLICATION));
        assert!(!volume.matches("ENUMERATED_VOLUME_ELEMENT_PURPOSE"));
        let curve = StepElementCurveElementPurposeMember::new();
        assert!(curve.set_name(APPLICATION));
        assert_eq!(curve.name(), APPLICATION);
    }

    #[test]
    fn measure_keeps_a_real_under_its_name() {
        let mut select = StepElementMeasureOrUnspecifiedValue::new();
        let member = Handle::new(StepElementMeasureOrUnspecifiedValueMember::new());
        member.set_name("CONTEXT_DEPENDENT_MEASURE");
        select.set_value(&member.cast()).unwrap();
        select.set_context_dependent_measure(1.25);
        assert_eq!(select.context_dependent_measure(), 1.25);
        assert_eq!(select.case_member(), 1);
    }
}
