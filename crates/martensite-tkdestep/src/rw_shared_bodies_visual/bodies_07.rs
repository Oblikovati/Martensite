// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0053 {
    ($a_description:ident, $a_magnitude:ident, $a_name:ident, $a_toleranced_shape_aspect:ident, $ach:ident, $an_unit_size:ident, $data:ident, $num:ident) => {
        rw_block_b_0061!(
            $a_description,
            $a_magnitude,
            $a_name,
            $a_toleranced_shape_aspect,
            $ach,
            $data,
            $num
        );
        let mut $an_unit_size: Handle<StepBasicLengthMeasureWithUnit> = Default::default();
    };
}
pub(crate) use rw_block_b_0053;

macro_rules! rw_block_b_0054 {
    ($a_shape_aspect_description:ident, $a_shape_aspect_name:ident, $a_shape_aspect_of_shape:ident, $a_shape_aspect_product_definitional:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        rw_block_b_0192!(
            $a_shape_aspect_description,
            $a_shape_aspect_name,
            $ach,
            $data,
            $num
        );
        $data.read_string(
            $num,
            2,
            "shape_aspect.description",
            $ach,
            &mut $a_shape_aspect_description,
        )?;
        rw_block_b_0141!(
            $a_shape_aspect_of_shape,
            $a_shape_aspect_product_definitional,
            $ach,
            $data,
            $num
        );
        $ent.init(
            &$a_shape_aspect_name,
            &$a_shape_aspect_description,
            &$a_shape_aspect_of_shape,
            $a_shape_aspect_product_definitional,
        );
    };
}
pub(crate) use rw_block_b_0054;

macro_rules! rw_block_b_0055 {
    ($a_item:ident, $a_name:ident, $a_styles:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        rw_block_b_0182!($a_name, $a_styles, $ach, $data, $num);
        let mut $anent2: Handle<StepVisualPresentationStyleAssignment> = Default::default();
        let mut $nsub2: i32 = Default::default();
        if $data.read_sub_list($num, 2, "styles", $ach, &mut $nsub2, false, 0, 0)? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_styles = step_new_array1::<Handle<StepVisualPresentationStyleAssignment>>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity(
                    $nsub2,
                    $i2,
                    "presentation_style_assignment",
                    $ach,
                    StepVisualPresentationStyleAssignment::static_type(),
                    &mut $anent2,
                )? {
                    step_array1_set(&$a_styles, $i2, $anent2.clone(), "ReadStep")?;
                }
            }
        }
        let mut $a_item: Handle<StandardTransient> = Default::default();
        $data.read_entity(
            $num,
            3,
            "item",
            $ach,
            StandardTransient::static_type(),
            &mut $a_item,
        )?;
    };
}
pub(crate) use rw_block_b_0055;

macro_rules! rw_block_b_0056 {
    ($i3:ident, $j3:ident, $the_ent:ident, $the_sw:ident, $var0:ident) => {
        rw_block_b_0166!($the_ent, $the_sw);
        for $i3 in 1..=step_live(&$the_ent.normals(), "WriteStep")?
            .array2()
            .col_length()
        {
            $the_sw.new_line(false)?;
            $the_sw.open_sub()?;
            for $j3 in 1..=step_live(&$the_ent.normals(), "WriteStep")?
                .array2()
                .row_length()
            {
                let $var0: f64 = step_array2_item(&$the_ent.normals(), $i3, $j3, "WriteStep")?;
                $the_sw.send_real($var0)?;
            }
            $the_sw.close_sub()?;
        }
        $the_sw.close_sub()?;
        if $the_ent.has_geometric_link() {
            $the_sw.send_entity(&$the_ent.geometric_link().value())?;
        } else {
            $the_sw.send_undef()?;
        }
        $the_sw.open_sub()?;
    };
}
pub(crate) use rw_block_b_0056;

macro_rules! rw_block_b_0057 {
    ($the_ent:ident, $the_sw:ident) => {
        rw_block_b_0084!($the_ent, $the_sw);
        $the_sw.send_entity(&$the_ent.pair_curve())?;
        $the_sw.send_boolean($the_ent.orientation())?;
    };
}
pub(crate) use rw_block_b_0057;

macro_rules! rw_block_b_0058 {
    ($a_shape_aspect_description:ident, $a_shape_aspect_name:ident, $a_shape_aspect_of_shape:ident, $a_shape_aspect_product_definitional:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_b_0192!(
            $a_shape_aspect_description,
            $a_shape_aspect_name,
            $ach,
            $data,
            $num
        );
        if $data.is_param_defined($num, 2)? {
            $data.read_string(
                $num,
                2,
                "shape_aspect.description",
                $ach,
                &mut $a_shape_aspect_description,
            )?;
        }
        rw_block_b_0141!(
            $a_shape_aspect_of_shape,
            $a_shape_aspect_product_definitional,
            $ach,
            $data,
            $num
        );
    };
}
pub(crate) use rw_block_b_0058;

macro_rules! rw_block_b_0059 {
    ($a_property_definition_definition:ident, $a_property_definition_description:ident, $a_property_definition_name:ident, $has_property_definition_description:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        let mut $a_property_definition_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "property_definition.name",
            $the_arch,
            &mut $a_property_definition_name,
        )?;
        let mut $a_property_definition_description: Handle<HAsciiString> = Default::default();
        let mut $has_property_definition_description: bool = true;
        if $the_data.is_param_defined($the_num, 2)? {
            $the_data.read_string(
                $the_num,
                2,
                "property_definition.description",
                $the_arch,
                &mut $a_property_definition_description,
            )?;
        } else {
            $has_property_definition_description = false;
            $a_property_definition_description = Default::default();
        }
        let mut $a_property_definition_definition: StepReprCharacterizedDefinition =
            Default::default();
        $the_data.read_entity_select(
            $the_num,
            3,
            "property_definition.definition",
            $the_arch,
            &mut $a_property_definition_definition,
        )?;
        $the_ent.init(
            &$a_property_definition_name,
            $has_property_definition_description,
            &$a_property_definition_description,
            &$a_property_definition_definition,
        );
    };
}
pub(crate) use rw_block_b_0059;

macro_rules! rw_block_b_0060 {
    ($a_geometric_tolerance_description:ident, $a_geometric_tolerance_magnitude:ident, $a_geometric_tolerance_name:ident, $a_geometric_tolerance_toleranced_shape_aspect:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_geometric_tolerance_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "geometric_tolerance.name",
            $ach,
            &mut $a_geometric_tolerance_name,
        )?;
        let mut $a_geometric_tolerance_description: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            2,
            "geometric_tolerance.description",
            $ach,
            &mut $a_geometric_tolerance_description,
        )?;
        let mut $a_geometric_tolerance_magnitude: Handle<StandardTransient> = Default::default();
        $data.read_entity(
            $num,
            3,
            "geometric_tolerance.magnitude",
            $ach,
            StandardTransient::static_type(),
            &mut $a_geometric_tolerance_magnitude,
        )?;
        let mut $a_geometric_tolerance_toleranced_shape_aspect: StepDimTolGeometricToleranceTarget =
            Default::default();
        $data.read_entity_select(
            $num,
            4,
            "geometric_tolerance.toleranced_shape_aspect",
            $ach,
            &mut $a_geometric_tolerance_toleranced_shape_aspect,
        )?;
    };
}
pub(crate) use rw_block_b_0060;

macro_rules! rw_block_b_0061 {
    ($a_description:ident, $a_magnitude:ident, $a_name:ident, $a_toleranced_shape_aspect:ident, $ach:ident, $data:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "geometric_tolerance.name", $ach, &mut $a_name)?;
        let mut $a_description: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            2,
            "geometric_tolerance.description",
            $ach,
            &mut $a_description,
        )?;
        let mut $a_magnitude: Handle<StandardTransient> = Default::default();
        $data.read_entity(
            $num,
            3,
            "geometric_tolerance.magnitude",
            $ach,
            StandardTransient::static_type(),
            &mut $a_magnitude,
        )?;
        let mut $a_toleranced_shape_aspect: StepDimTolGeometricToleranceTarget = Default::default();
        $data.read_entity_select(
            $num,
            4,
            "geometric_tolerance.toleranced_shape_aspect",
            $ach,
            &mut $a_toleranced_shape_aspect,
        )?;
    };
}
pub(crate) use rw_block_b_0061;
