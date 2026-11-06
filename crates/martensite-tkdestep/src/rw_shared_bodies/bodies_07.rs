// The RW member bodies several classes repeat.

macro_rules! rw_block_0047 {
    ($a_measure_with_unit:ident, $a_unit_component:ident, $ach:ident, $data:ident, $mvc:ident, $num:ident, $num0:ident) => {
        let mut $num: i32 = 0;
        $data.named_for_complex_short("MEASURE_WITH_UNIT", "MSWTUN", $num0, &mut $num, $ach);
        rw_block_0103!($a_unit_component, $ach, $data, $mvc, $num);
        let $a_measure_with_unit: Handle<StepBasicMeasureWithUnit> =
            Handle::new(StepBasicMeasureWithUnit::new());
        $a_measure_with_unit.init(&$mvc, &$a_unit_component);
    };
}
pub(crate) use rw_block_0047;

macro_rules! rw_block_0048 {
    ($a_qri:ident, $ent:ident, $i:ident, $nbq:ident, $sw:ident) => {
        step_start_entity($sw, "QUALIFIED_REPRESENTATION_ITEM")?;
        let $a_qri: Handle<StepShapeQualifiedRepresentationItem> =
            $ent.get_qualified_representation_item();
        let $i: i32 = Default::default();
        let $nbq: i32 = step_live(&$a_qri, "WriteStep")?.nb_qualifiers()?;
        $sw.open_sub()?;
        for $i in 1..=$nbq {
            $sw.send_entity(
                &step_live(&$a_qri, "WriteStep")?
                    .qualifiers_value($i)?
                    .value(),
            )?;
        }
        $sw.close_sub()?;
        step_start_entity($sw, "REPRESENTATION_ITEM")?;
        $sw.send_entity(&$ent.name())?;
    };
}
pub(crate) use rw_block_0048;

macro_rules! rw_block_0049 {
    ($ent:ident, $i3:ident, $sw:ident) => {
        rw_block_0061!($ent, $i3, $sw);
        $sw.open_sub()?;
    };
}
pub(crate) use rw_block_0049;

macro_rules! rw_block_0050 {
    ($ent:ident, $i2:ident, $sw:ident) => {
        step_start_entity($sw, "BOUNDED_CURVE")?;
        step_start_entity($sw, "B_SPLINE_CURVE")?;
        $sw.send_integer($ent.degree())?;
        $sw.open_sub()?;
        for $i2 in 1..=$ent.nb_control_points_list() {
            $sw.send_entity(&$ent.control_points_list_value($i2)?)?;
        }
        rw_block_0169!($ent, $sw);
    };
}
pub(crate) use rw_block_0050;

macro_rules! rw_block_0051 {
    ($a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $ach:ident, $an_it0:ident, $data:ident, $ent:ident, $i0:ident, $nb0:ident, $num:ident, $num2:ident, $sub2:ident) => {
        rw_block_0155!(
            $a_representation_items,
            $a_representation_name,
            $ach,
            $data,
            $num,
            $sub2
        );
        if $data.read_sub_list(
            $num,
            2,
            "representation.items",
            $ach,
            &mut $sub2,
            false,
            0,
            0,
        )? {
            let $num2: i32 = $sub2;
            let $nb0: i32 = $data.nb_params($num2)?;
            $a_representation_items =
                step_new_array1::<Handle<StepReprRepresentationItem>>(1, $nb0)?;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: Handle<StepReprRepresentationItem> = Default::default();
                $data.read_entity(
                    $num2,
                    $i0,
                    "representation.items",
                    $ach,
                    StepReprRepresentationItem::static_type(),
                    &mut $an_it0,
                )?;
                step_array1_set(&$a_representation_items, $i0, $an_it0.clone(), "ReadStep")?;
            }
        }
        rw_block_0186!(
            $a_representation_context_of_items,
            $a_representation_items,
            $a_representation_name,
            $ach,
            $data,
            $ent,
            $num
        );
    };
}
pub(crate) use rw_block_0051;

macro_rules! rw_block_0052 {
    ($a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $ach:ident, $an_it0:ident, $data:ident, $ent:ident, $i0:ident, $nb0:ident, $num:ident, $num2:ident, $sub2:ident) => {
        rw_block_0155!(
            $a_representation_items,
            $a_representation_name,
            $ach,
            $data,
            $num,
            $sub2
        );
        if $data.read_sub_list(
            $num,
            2,
            "representation.items",
            $ach,
            &mut $sub2,
            false,
            0,
            0,
        )? {
            let $nb0: i32 = $data.nb_params($sub2)?;
            $a_representation_items =
                step_new_array1::<Handle<StepReprRepresentationItem>>(1, $nb0)?;
            let $num2: i32 = $sub2;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: Handle<StepReprRepresentationItem> = Default::default();
                $data.read_entity(
                    $num2,
                    $i0,
                    "representation_item",
                    $ach,
                    StepReprRepresentationItem::static_type(),
                    &mut $an_it0,
                )?;
                step_array1_set(&$a_representation_items, $i0, $an_it0.clone(), "ReadStep")?;
            }
        }
        rw_block_0186!(
            $a_representation_context_of_items,
            $a_representation_items,
            $a_representation_name,
            $ach,
            $data,
            $ent,
            $num
        );
    };
}
pub(crate) use rw_block_0052;

macro_rules! rw_block_0053 {
    ($a_context_of_items:ident, $a_items:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        rw_block_0108!($a_items, $a_name, $ach, $anent2, $data, $nsub2, $num);
        if $data.read_sub_list($num, 2, "items", $ach, &mut $nsub2, false, 0, 0)? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity(
                    $nsub2,
                    $i2,
                    "representation_item",
                    $ach,
                    StepReprRepresentationItem::static_type(),
                    &mut $anent2,
                )? {
                    step_array1_set(&$a_items, $i2, $anent2.clone(), "ReadStep")?;
                }
            }
        }
        let mut $a_context_of_items: Handle<StepReprRepresentationContext> = Default::default();
        $data.read_entity(
            $num,
            3,
            "context_of_items",
            $ach,
            StepReprRepresentationContext::static_type(),
            &mut $a_context_of_items,
        )?;
    };
}
pub(crate) use rw_block_0053;

macro_rules! rw_block_0054 {
    ($a_self_intersect:ident, $a_surface_form:ident, $a_u_closed:ident, $a_v_closed:ident, $ach:ident, $data:ident, $num:ident, $text:ident) => {
        let mut $a_surface_form: StepGeomBSplineSurfaceForm =
            StepGeomBSplineSurfaceForm::BssfPlaneSurf;
        if $data.param_type($num, 4)? == InterfaceParamType::ParamEnum {
            let $text: Vec<u8> = $data.param_c_value($num, 4)?;
            if !RwStepGeomRwBSplineSurfaceForm::convert_to_enum(&$text, &mut $a_surface_form) {
                $ach.add_fail_text(
                    "Enumeration b_spline_surface_form has not an allowed value",
                    "",
                );
            }
        } else {
            $ach.add_fail_text("Parameter #4 (surface_form) is not an enumeration", "");
        }
        let mut $a_u_closed: StepDataLogical = Default::default();
        $data.read_logical($num, 5, "u_closed", $ach, &mut $a_u_closed)?;
        let mut $a_v_closed: StepDataLogical = Default::default();
        $data.read_logical($num, 6, "v_closed", $ach, &mut $a_v_closed)?;
        let mut $a_self_intersect: StepDataLogical = Default::default();
        $data.read_logical($num, 7, "self_intersect", $ach, &mut $a_self_intersect)?;
    };
}
pub(crate) use rw_block_0054;

macro_rules! rw_block_0055 {
    ($a_description:ident, $a_name:ident, $a_rep1:ident, $a_rep2:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0126!($a_description, $a_name, $ach, $data, $num);
        rw_block_0125!($a_rep1, $a_rep2, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0055;

macro_rules! rw_block_0056 {
    ($a_shape_aspect_description:ident, $a_shape_aspect_name:ident, $a_shape_aspect_of_shape:ident, $a_shape_aspect_product_definitional:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_shape_aspect_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "shape_aspect.name",
            $ach,
            &mut $a_shape_aspect_name,
        )?;
        let mut $a_shape_aspect_description: Handle<HAsciiString> = Default::default();
        if $data.is_param_defined($num, 2)? {
            $data.read_string(
                $num,
                2,
                "shape_aspect.description",
                $ach,
                &mut $a_shape_aspect_description,
            )?;
        }
        let mut $a_shape_aspect_of_shape: Handle<StepReprProductDefinitionShape> =
            Default::default();
        $data.read_entity(
            $num,
            3,
            "shape_aspect.of_shape",
            $ach,
            StepReprProductDefinitionShape::static_type(),
            &mut $a_shape_aspect_of_shape,
        )?;
        let mut $a_shape_aspect_product_definitional: StepDataLogical = Default::default();
        $data.read_logical(
            $num,
            4,
            "shape_aspect.product_definitional",
            $ach,
            &mut $a_shape_aspect_product_definitional,
        )?;
    };
}
pub(crate) use rw_block_0056;
