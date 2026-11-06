// The RW member bodies several classes repeat.

macro_rules! rw_block_0078 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.parent.parent.parent.id())?;
        $sw.send_entity(&$ent.parent.parent.parent.name())?;
        if $ent.parent.parent.parent.has_description() {
            $sw.send_entity(&$ent.parent.parent.parent.description())?;
        } else {
            $sw.send_undef()?;
        }
        $sw.send_entity(
            &$ent
                .parent
                .parent
                .parent
                .relating_product_definition_ap242()
                .value(),
        )?;
        $sw.send_entity(
            &$ent
                .parent
                .parent
                .parent
                .related_product_definition_ap242()
                .value(),
        )?;
        if $ent.parent.has_reference_designator() {
            $sw.send_entity(&$ent.parent.reference_designator())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0078;

macro_rules! rw_block_0079 {
    ($a_parent_curve:ident, $a_same_sense:ident, $a_transition:ident, $ach:ident, $data:ident, $num:ident, $text:ident) => {
        let mut $a_transition: StepGeomTransitionCode = StepGeomTransitionCode::TcDiscontinuous;
        if $data.param_type($num, 1)? == InterfaceParamType::ParamEnum {
            let $text: Vec<u8> = $data.param_c_value($num, 1)?;
            if !RwStepGeomRwTransitionCode::convert_to_enum(&$text, &mut $a_transition) {
                $ach.add_fail_text("Enumeration transition_code has not an allowed value", "");
            }
        } else {
            $ach.add_fail_text("Parameter #1 (transition) is not an enumeration", "");
        }
        let mut $a_same_sense: bool = Default::default();
        $data.read_boolean($num, 2, "same_sense", $ach, &mut $a_same_sense)?;
        let mut $a_parent_curve: Handle<StepGeomCurve> = Default::default();
        $data.read_entity(
            $num,
            3,
            "parent_curve",
            $ach,
            StepGeomCurve::static_type(),
            &mut $a_parent_curve,
        )?;
    };
}
pub(crate) use rw_block_0079;

macro_rules! rw_block_0080 {
    ($a_closed_curve:ident, $a_curve_form:ident, $a_self_intersect:ident, $ach:ident, $data:ident, $num:ident, $text:ident) => {
        let mut $a_curve_form: StepGeomBSplineCurveForm =
            StepGeomBSplineCurveForm::BscfPolylineForm;
        if $data.param_type($num, 4)? == InterfaceParamType::ParamEnum {
            let $text: Vec<u8> = $data.param_c_value($num, 4)?;
            if !RwStepGeomRwBSplineCurveForm::convert_to_enum(&$text, &mut $a_curve_form) {
                $ach.add_fail_text(
                    "Enumeration b_spline_curve_form has not an allowed value",
                    "",
                );
            }
        } else {
            $ach.add_fail_text("Parameter #4 (curve_form) is not an enumeration", "");
        }
        let mut $a_closed_curve: StepDataLogical = Default::default();
        $data.read_logical($num, 5, "closed_curve", $ach, &mut $a_closed_curve)?;
        let mut $a_self_intersect: StepDataLogical = Default::default();
        $data.read_logical($num, 6, "self_intersect", $ach, &mut $a_self_intersect)?;
    };
}
pub(crate) use rw_block_0080;

macro_rules! rw_block_0081 {
    ($a_basis_surface:ident, $a_name:ident, $a_reference_to_curve:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0133!($a_basis_surface, $a_name, $ach, $data, $num);
        let mut $a_reference_to_curve: Handle<StepReprDefinitionalRepresentation> =
            Default::default();
        $data.read_entity(
            $num,
            3,
            "reference_to_curve",
            $ach,
            StepReprDefinitionalRepresentation::static_type(),
            &mut $a_reference_to_curve,
        )?;
    };
}
pub(crate) use rw_block_0081;

macro_rules! rw_block_0082 {
    ($a_description:ident, $a_name:ident, $ach:ident, $data:ident, $ent:ident, $has_description:ident, $num:ident) => {
        rw_block_0102!($a_description, $a_name, $ach, $data, $has_description, $num);
        $ent.init(&$a_name, $has_description, &$a_description);
    };
}
pub(crate) use rw_block_0082;

macro_rules! rw_block_0083 {
    ($a_conversion_factor:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0100!($a_conversion_factor, $a_name, $ach, $data, $num);
        $num = $data.next_for_complex($num);
    };
}
pub(crate) use rw_block_0083;

macro_rules! rw_block_0084 {
    ($a_associated_geometry:ident, $a_associated_geometry_item:ident, $a_curve3d:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_curve3d: Handle<StepGeomCurve> = Default::default();
        $data.read_entity(
            $num,
            2,
            "curve_3d",
            $ach,
            StepGeomCurve::static_type(),
            &mut $a_curve3d,
        )?;
        let mut $a_associated_geometry: Handle<HArray1<StepGeomPcurveOrSurface>> =
            Default::default();
        let mut $a_associated_geometry_item: StepGeomPcurveOrSurface = Default::default();
    };
}
pub(crate) use rw_block_0084;

macro_rules! rw_block_0085 {
    ($a_name:ident, $a_position:ident, $a_radius:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0140!($a_name, $a_position, $ach, $data, $num);
        let mut $a_radius: f64 = Default::default();
        $data.read_real($num, 3, "radius", $ach, &mut $a_radius)?;
    };
}
pub(crate) use rw_block_0085;

macro_rules! rw_block_0086 {
    ($a_agreement_flag:ident, $a_base_surface:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_base_surface: Handle<StepGeomSurface> = Default::default();
        $data.read_entity(
            $num,
            2,
            "base_surface",
            $ach,
            StepGeomSurface::static_type(),
            &mut $a_base_surface,
        )?;
        let mut $a_agreement_flag: bool = Default::default();
        $data.read_boolean($num, 3, "agreement_flag", $ach, &mut $a_agreement_flag)?;
    };
}
pub(crate) use rw_block_0086;

macro_rules! rw_block_0087 {
    ($a_name:ident, $a_repr_item:ident, $ach:ident, $data:ident, $num:ident, $num0:ident) => {
        rw_block_0131!($a_name, $ach, $data, $num, $num0);
        let $a_repr_item: Handle<StepReprRepresentationItem> =
            Handle::new(StepReprRepresentationItem::new());
        $a_repr_item.init(&$a_name);
    };
}
pub(crate) use rw_block_0087;

macro_rules! rw_block_0088 {
    ($a_context_identifier:ident, $a_context_type:ident, $ach:ident, $data:ident, $num:ident) => {
        $num = $data.next_for_complex($num);
        rw_block_0104!($a_context_identifier, $a_context_type, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0088;

macro_rules! rw_block_0089 {
    ($a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0114!($a_name, $ach, $data, $num);
        if !$data.check_nb_params($num, 0, $ach, "surface")? {
            return Ok(());
        }
    };
}
pub(crate) use rw_block_0089;

macro_rules! rw_block_0090 {
    ($ent:ident, $has_adescription:ident, $i3:ident, $sw:ident) => {
        rw_block_0191!($ent, $has_adescription, $sw);
        $sw.open_sub()?;
        for $i3 in 1..=$ent.nb_products() {
            $sw.send_entity(&$ent.products_value($i3)?)?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_0090;

macro_rules! rw_block_0091 {
    ($ent:ident, $has_aprefix:ident, $sw:ident) => {
        step_start_entity($sw, "NAMED_UNIT")?;
        $sw.send_entity(&$ent.dimensions())?;
        rw_block_0145!($ent, $has_aprefix, $sw);
    };
}
pub(crate) use rw_block_0091;
