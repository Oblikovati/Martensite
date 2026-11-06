// The RW member bodies several classes repeat.

macro_rules! rw_block_0169 {
    ($ent:ident, $sw:ident) => {
        $sw.close_sub()?;
        $sw.send_enum(RwStepGeomRwBSplineCurveForm::convert_to_string(
            $ent.curve_form(),
        ))?;
        $sw.send_logical($ent.closed_curve())?;
        $sw.send_logical($ent.self_intersect())?;
    };
}
pub(crate) use rw_block_0169;

macro_rules! rw_block_0170 {
    ($a_associated_geometry:ident, $a_curve3d:ident, $a_master_representation:ident, $a_name:ident, $ach:ident, $data:ident, $ent:ident, $num:ident, $text:ident) => {
        let mut $a_master_representation: StepGeomPreferredSurfaceCurveRepresentation =
            StepGeomPreferredSurfaceCurveRepresentation::PscrCurve3d;
        if $data.param_type($num, 4)? == InterfaceParamType::ParamEnum {
            let $text: Vec<u8> = $data.param_c_value($num, 4)?;
            if !RwStepGeomRwPreferredSurfaceCurveRepresentation::convert_to_enum(
                &$text,
                &mut $a_master_representation,
            ) {
                $ach.add_fail_text(
                    "Enumeration preferred_surface_curve_representation has not an allowed value",
                    "",
                );
            }
        } else {
            $ach.add_fail_text(
                "Parameter #4 (master_representation) is not an enumeration",
                "",
            );
        }
        $ent.init(
            &$a_name,
            &$a_curve3d,
            &$a_associated_geometry,
            $a_master_representation,
        );
    };
}
pub(crate) use rw_block_0170;

macro_rules! rw_block_0171 {
    ($ent:ident, $sw:ident) => {
        rw_block_0210!($ent, $sw);
        $sw.send_entity(&$ent.parent.definition().value())?;
    };
}
pub(crate) use rw_block_0171;

macro_rules! rw_block_0172 {
    ($a_dimensions:ident, $ach:ident, $data:ident, $num:ident) => {
        if !$data.check_nb_params($num, 1, $ach, "named_unit")? {
            return Ok(());
        }
        let mut $a_dimensions: Handle<StepBasicDimensionalExponents> = Default::default();
        $data.read_entity(
            $num,
            1,
            "dimensions",
            $ach,
            StepBasicDimensionalExponents::static_type(),
            &mut $a_dimensions,
        )?;
    };
}
pub(crate) use rw_block_0172;

macro_rules! rw_block_0173 {
    ($ent:ident, $iter:ident) => {
        if $ent.has_axis1() {
            $iter.get_one_item(&$ent.axis1().cast());
        }
        if $ent.has_axis2() {
            $iter.get_one_item(&$ent.axis2().cast());
        }
        $iter.get_one_item(&$ent.local_origin().cast());
    };
}
pub(crate) use rw_block_0173;

macro_rules! rw_block_0174 {
    ($ent:ident, $has_aprefix:ident, $sw:ident) => {
        let $has_aprefix: bool = $ent.has_prefix();
        if $has_aprefix {
            $sw.send_enum(RwStepBasicRwSiPrefix::convert_to_string($ent.prefix()))?;
        } else {
            $sw.send_undef()?;
        }
        $sw.send_enum(RwStepBasicRwSiUnitName::convert_to_string($ent.name()))?;
    };
}
pub(crate) use rw_block_0174;

macro_rules! rw_block_0175 {
    ($ach:ident, $data:ident, $num:ident, $num0:ident, $quals:ident) => {
        $data.named_for_complex_short(
            "QUALIFIED_REPRESENTATION_ITEM",
            "QLRPIT",
            $num0,
            &mut $num,
            $ach,
        );
        if !$data.check_nb_params($num, 1, $ach, "qualified_representation_item")? {
            return Ok(());
        }
        let mut $quals: Handle<HArray1<StepShapeValueQualifier>> = Default::default();
    };
}
pub(crate) use rw_block_0175;

macro_rules! rw_block_0176 {
    ($a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        if !$data.check_nb_params($num, 1, $ach, "representation_item")? {
            return Ok(());
        }
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
    };
}
pub(crate) use rw_block_0176;

macro_rules! rw_block_0177 {
    ($ent:ident, $is1:ident, $iter:ident, $js1:ident, $nbi_elem1:ident, $nbj_elem1:ident) => {
        let $nbi_elem1: i32 = $ent.nb_control_points_list_i();
        let $nbj_elem1: i32 = $ent.nb_control_points_list_j();
        for $is1 in 1..=$nbi_elem1 {
            for $js1 in 1..=$nbj_elem1 {
                $iter.get_one_item(&$ent.control_points_list_value($is1, $js1)?.cast());
            }
        }
    };
}
pub(crate) use rw_block_0177;

macro_rules! rw_block_0178 {
    ($ent:ident, $sw:ident) => {
        $sw.send_entity(&$ent.id())?;
        $sw.send_entity(&$ent.name())?;
        if $ent.has_description() {
            $sw.send_entity(&$ent.description())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0178;

macro_rules! rw_block_0179 {
    ($a_items:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> = Default::default();
    };
}
pub(crate) use rw_block_0179;

macro_rules! rw_block_0180 {
    ($a_weights_data:ident, $a_weights_data_item:ident, $ach:ident, $data:ident, $num:ident) => {
        if !$data.check_nb_params($num, 1, $ach, "rational_b_spline_curve")? {
            return Ok(());
        }
        let mut $a_weights_data: Handle<HArray1<f64>> = Default::default();
        let mut $a_weights_data_item: f64 = Default::default();
    };
}
pub(crate) use rw_block_0180;

macro_rules! rw_block_0181 {
    ($a_weights_data:ident, $a_weights_data_item:ident, $ach:ident, $data:ident, $num:ident) => {
        if !$data.check_nb_params($num, 1, $ach, "rational_b_spline_surface")? {
            return Ok(());
        }
        let mut $a_weights_data: Handle<HArray2<f64>> = Default::default();
        let mut $a_weights_data_item: f64 = Default::default();
    };
}
pub(crate) use rw_block_0181;

macro_rules! rw_block_0182 {
    ($a_edge_list:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_edge_list: Handle<HArray1<Handle<StepShapeOrientedEdge>>> = Default::default();
    };
}
pub(crate) use rw_block_0182;
