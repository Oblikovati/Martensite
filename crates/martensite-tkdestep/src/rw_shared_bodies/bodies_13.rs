// The RW member bodies several classes repeat.

macro_rules! rw_block_0121 {
    ($ent:ident, $sw:ident) => {
        step_start_entity($sw, "CONVERSION_BASED_UNIT")?;
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.conversion_factor())?;
        step_start_entity($sw, "NAMED_UNIT")?;
        $sw.send_entity(&$ent.dimensions())?;
    };
}
pub(crate) use rw_block_0121;

macro_rules! rw_block_0122 {
    ($ent:ident, $sw:ident) => {
        rw_block_0208!($ent, $sw);
        $sw.send_entity(&$ent.parent.parent.relating_shape_aspect())?;
        $sw.send_entity(&$ent.parent.parent.related_shape_aspect())?;
    };
}
pub(crate) use rw_block_0122;

macro_rules! rw_block_0123 {
    ($a_dimensions:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        $data.named_for_complex_short("NAMED_UNIT", "NMDUNT", $num0, &mut $num, $ach);
        rw_block_0172!($a_dimensions, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0123;

macro_rules! rw_block_0124 {
    ($a_formation:ident, $a_frame_of_reference:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_formation: Handle<StepBasicProductDefinitionFormation> = Default::default();
        $data.read_entity(
            $num,
            3,
            "formation",
            $ach,
            StepBasicProductDefinitionFormation::static_type(),
            &mut $a_formation,
        )?;
        let mut $a_frame_of_reference: Handle<StepBasicProductDefinitionContext> =
            Default::default();
        $data.read_entity(
            $num,
            4,
            "frame_of_reference",
            $ach,
            StepBasicProductDefinitionContext::static_type(),
            &mut $a_frame_of_reference,
        )?;
    };
}
pub(crate) use rw_block_0124;

macro_rules! rw_block_0125 {
    ($a_rep1:ident, $a_rep2:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_rep1: Handle<StepReprRepresentation> = Default::default();
        $data.read_entity(
            $num,
            3,
            "rep_1",
            $ach,
            StepReprRepresentation::static_type(),
            &mut $a_rep1,
        )?;
        let mut $a_rep2: Handle<StepReprRepresentation> = Default::default();
        $data.read_entity(
            $num,
            4,
            "rep_2",
            $ach,
            StepReprRepresentation::static_type(),
            &mut $a_rep2,
        )?;
    };
}
pub(crate) use rw_block_0125;

macro_rules! rw_block_0126 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0199!($a_description, $a_name, $ach, $data, $num);
        if $data.is_param_defined($num, 2)? {
            $data.read_string($num, 2, "description", $ach, &mut $a_description)?;
        }
    };
}
pub(crate) use rw_block_0126;

macro_rules! rw_block_0127 {
    ($a_description:ident, $a_id:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0200!($a_description, $a_id, $ach, $data, $num);
        if $data.is_param_defined($num, 2)? {
            $data.read_string($num, 2, "description", $ach, &mut $a_description)?;
        }
    };
}
pub(crate) use rw_block_0127;

macro_rules! rw_block_0128 {
    ($ent:ident, $sw:ident) => {
        rw_block_0210!($ent, $sw);
        $sw.send_entity(&$ent.parent.relating_shape_aspect())?;
        $sw.send_entity(&$ent.parent.related_shape_aspect())?;
    };
}
pub(crate) use rw_block_0128;

macro_rules! rw_block_0129 {
    ($a_frame_of_reference:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_frame_of_reference: Handle<StepBasicApplicationContext> = Default::default();
        $data.read_entity(
            $num,
            2,
            "frame_of_reference",
            $ach,
            StepBasicApplicationContext::static_type(),
            &mut $a_frame_of_reference,
        )?;
    };
}
pub(crate) use rw_block_0129;

macro_rules! rw_block_0130 {
    ($a_location:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_location: Handle<StepGeomCartesianPoint> = Default::default();
        $data.read_entity(
            $num,
            2,
            "location",
            $ach,
            StepGeomCartesianPoint::static_type(),
            &mut $a_location,
        )?;
    };
}
pub(crate) use rw_block_0130;

macro_rules! rw_block_0131 {
    ($a_name:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        $data.named_for_complex_short("REPRESENTATION_ITEM", "RPRITM", $num0, &mut $num, $ach);
        rw_block_0176!($a_name, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0131;

macro_rules! rw_block_0132 {
    ($a_basis_curve:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_basis_curve: Handle<StepGeomCurve> = Default::default();
        $data.read_entity(
            $num,
            2,
            "basis_curve",
            $ach,
            StepGeomCurve::static_type(),
            &mut $a_basis_curve,
        )?;
    };
}
pub(crate) use rw_block_0132;

macro_rules! rw_block_0133 {
    ($a_basis_surface:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_basis_surface: Handle<StepGeomSurface> = Default::default();
        $data.read_entity(
            $num,
            2,
            "basis_surface",
            $ach,
            StepGeomSurface::static_type(),
            &mut $a_basis_surface,
        )?;
    };
}
pub(crate) use rw_block_0133;

macro_rules! rw_block_0134 {
    ($a_name:ident, $a_swept_curve:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_swept_curve: Handle<StepGeomCurve> = Default::default();
        $data.read_entity(
            $num,
            2,
            "swept_curve",
            $ach,
            StepGeomCurve::static_type(),
            &mut $a_swept_curve,
        )?;
    };
}
pub(crate) use rw_block_0134;

macro_rules! rw_block_0135 {
    ($a_dimensional_size_applies_to:ident, $a_dimensional_size_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_dimensional_size_applies_to: Handle<StepReprShapeAspect> = Default::default();
        $data.read_entity(
            $num,
            1,
            "dimensional_size.applies_to",
            $ach,
            StepReprShapeAspect::static_type(),
            &mut $a_dimensional_size_applies_to,
        )?;
        let mut $a_dimensional_size_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            2,
            "dimensional_size.name",
            $ach,
            &mut $a_dimensional_size_name,
        )?;
    };
}
pub(crate) use rw_block_0135;

macro_rules! rw_block_0136 {
    ($a_name:ident, $a_outer:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_outer: Handle<StepShapeClosedShell> = Default::default();
        $data.read_entity(
            $num,
            2,
            "outer",
            $ach,
            StepShapeClosedShell::static_type(),
            &mut $a_outer,
        )?;
    };
}
pub(crate) use rw_block_0136;

macro_rules! rw_block_0137 {
    ($a_name:ident, $a_swept_area:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_swept_area: Handle<StepGeomCurveBoundedSurface> = Default::default();
        $data.read_entity(
            $num,
            2,
            "swept_area",
            $ach,
            StepGeomCurveBoundedSurface::static_type(),
            &mut $a_swept_area,
        )?;
    };
}
pub(crate) use rw_block_0137;
