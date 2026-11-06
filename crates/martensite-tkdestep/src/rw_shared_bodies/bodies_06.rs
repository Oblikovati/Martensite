// The RW member bodies several classes repeat.

macro_rules! rw_block_0037 {
    ($ent:ident, $has_aaxis1:ident, $has_aaxis2:ident, $has_ascale:ident, $sw:ident) => {
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.name())?;
        $sw.send_entity(&$ent.name())?;
        let $has_aaxis1: bool = $ent.has_axis1();
        if $has_aaxis1 {
            $sw.send_entity(&$ent.axis1())?;
        } else {
            $sw.send_undef()?;
        }
        let $has_aaxis2: bool = $ent.has_axis2();
        if $has_aaxis2 {
            $sw.send_entity(&$ent.axis2())?;
        } else {
            $sw.send_undef()?;
        }
        $sw.send_entity(&$ent.local_origin())?;
        let $has_ascale: bool = $ent.has_scale();
        if $has_ascale {
            $sw.send_real($ent.scale())?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_0037;

macro_rules! rw_block_0038 {
    ($a_context_of_items:ident, $a_items:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        if !$data.check_nb_params($num, 3, $ach, "representation")? {
            return Ok(());
        }
        rw_block_0053!(
            $a_context_of_items,
            $a_items,
            $a_name,
            $ach,
            $anent2,
            $data,
            $i2,
            $nb2,
            $nsub2,
            $num
        );
    };
}
pub(crate) use rw_block_0038;

macro_rules! rw_block_0039 {
    ($a_context_of_items:ident, $a_items:ident, $a_name:ident, $ach:ident, $anent2:ident, $data:ident, $ent:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        rw_block_0053!(
            $a_context_of_items,
            $a_items,
            $a_name,
            $ach,
            $anent2,
            $data,
            $i2,
            $nb2,
            $nsub2,
            $num
        );
        $ent.init(&$a_name, &$a_items, &$a_context_of_items);
    };
}
pub(crate) use rw_block_0039;

macro_rules! rw_block_0040 {
    ($a_name:ident, $a_segments:ident, $a_self_intersect:ident, $ach:ident, $anent2:ident, $data:ident, $ent:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        let mut $a_name: Handle<HAsciiString> = Default::default();
        $data.read_string($num, 1, "name", $ach, &mut $a_name)?;
        let mut $a_segments: Handle<HArray1<Handle<StepGeomCompositeCurveSegment>>> =
            Default::default();
        let mut $anent2: Handle<StepGeomCompositeCurveSegment> = Default::default();
        let mut $nsub2: i32 = Default::default();
        if $data.read_sub_list($num, 2, "segments", $ach, &mut $nsub2, false, 0, 0)? {
            let $nb2: i32 = $data.nb_params($nsub2)?;
            $a_segments = step_new_array1::<Handle<StepGeomCompositeCurveSegment>>(1, $nb2)?;
            for $i2 in 1..=$nb2 {
                if $data.read_entity(
                    $nsub2,
                    $i2,
                    "composite_curve_segment",
                    $ach,
                    StepGeomCompositeCurveSegment::static_type(),
                    &mut $anent2,
                )? {
                    step_array1_set(&$a_segments, $i2, $anent2.clone(), "ReadStep")?;
                }
            }
        }
        let mut $a_self_intersect: StepDataLogical = Default::default();
        $data.read_logical($num, 3, "self_intersect", $ach, &mut $a_self_intersect)?;
        $ent.init(&$a_name, &$a_segments, $a_self_intersect);
    };
}
pub(crate) use rw_block_0040;

macro_rules! rw_block_0041 {
    ($a_name:ident, $a_weights_data:ident, $a_weights_data_item:ident, $ach:ident, $data:ident, $i6:ident, $nb6:ident, $nsub6:ident, $num:ident) => {
        rw_block_0180!($a_weights_data, $a_weights_data_item, $ach, $data, $num);
        let mut $nsub6: i32 = Default::default();
        if $data.read_sub_list($num, 1, "weights_data", $ach, &mut $nsub6, false, 0, 0)? {
            let $nb6: i32 = $data.nb_params($nsub6)?;
            $a_weights_data = step_new_array1::<f64>(1, $nb6)?;
            for $i6 in 1..=$nb6 {
                if $data.read_real($nsub6, $i6, "weights_data", $ach, &mut $a_weights_data_item)? {
                    step_array1_set(&$a_weights_data, $i6, $a_weights_data_item, "ReadStep")?;
                }
            }
        }
        rw_block_0152!($a_name, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0041;

macro_rules! rw_block_0042 {
    ($a_description:ident, $a_name:ident, $a_rep1:ident, $a_rep2:ident, $ach:ident, $data:ident, $num:ident) => {
        if !$data.check_nb_params($num, 4, $ach, "representation_relationship")? {
            return Ok(());
        }
        rw_block_0055!($a_description, $a_name, $a_rep1, $a_rep2, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0042;

macro_rules! rw_block_0043 {
    ($a_shape_aspect_relationship_description:ident, $a_shape_aspect_relationship_name:ident, $a_shape_aspect_relationship_related_shape_aspect:ident, $a_shape_aspect_relationship_relating_shape_aspect:ident, $ach:ident, $data:ident, $has_shape_aspect_relationship_description:ident, $num:ident) => {
        let mut $a_shape_aspect_relationship_name: Handle<HAsciiString> = Default::default();
        $data.read_string(
            $num,
            1,
            "shape_aspect_relationship.name",
            $ach,
            &mut $a_shape_aspect_relationship_name,
        )?;
        let mut $a_shape_aspect_relationship_description: Handle<HAsciiString> = Default::default();
        let mut $has_shape_aspect_relationship_description: bool = true;
        if $data.is_param_defined($num, 2)? {
            $data.read_string(
                $num,
                2,
                "shape_aspect_relationship.description",
                $ach,
                &mut $a_shape_aspect_relationship_description,
            )?;
        } else {
            $has_shape_aspect_relationship_description = false;
        }
        let mut $a_shape_aspect_relationship_relating_shape_aspect: Handle<StepReprShapeAspect> =
            Default::default();
        $data.read_entity(
            $num,
            3,
            "shape_aspect_relationship.relating_shape_aspect",
            $ach,
            StepReprShapeAspect::static_type(),
            &mut $a_shape_aspect_relationship_relating_shape_aspect,
        )?;
        let mut $a_shape_aspect_relationship_related_shape_aspect: Handle<StepReprShapeAspect> =
            Default::default();
        $data.read_entity(
            $num,
            4,
            "shape_aspect_relationship.related_shape_aspect",
            $ach,
            StepReprShapeAspect::static_type(),
            &mut $a_shape_aspect_relationship_related_shape_aspect,
        )?;
    };
}
pub(crate) use rw_block_0043;

macro_rules! rw_block_0044 {
    ($a_conversion_factor:ident, $a_dimensions:ident, $a_name:ident, $ach:ident, $data:ident, $num:ident) => {
        rw_block_0083!($a_conversion_factor, $a_name, $ach, $data, $num);
        rw_block_0172!($a_dimensions, $ach, $data, $num);
    };
}
pub(crate) use rw_block_0044;

macro_rules! rw_block_0045 {
    ($a_shape_aspect_description:ident, $a_shape_aspect_name:ident, $a_shape_aspect_of_shape:ident, $a_shape_aspect_product_definitional:ident, $ach:ident, $data:ident, $ent:ident, $num:ident) => {
        rw_block_0056!(
            $a_shape_aspect_description,
            $a_shape_aspect_name,
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
pub(crate) use rw_block_0045;

macro_rules! rw_block_0046 {
($a_product_definition_relationship_description:ident, $a_product_definition_relationship_id:ident, $a_product_definition_relationship_name:ident, $a_product_definition_relationship_relating_product_definition:ident, $ach:ident, $data:ident, $has_product_definition_relationship_description:ident, $num:ident) => {
let mut $a_product_definition_relationship_id: Handle<HAsciiString> = Default::default();
$data.read_string($num, 1, "product_definition_relationship.id", $ach, &mut $a_product_definition_relationship_id)?;
let mut $a_product_definition_relationship_name: Handle<HAsciiString> = Default::default();
$data.read_string($num, 2, "product_definition_relationship.name", $ach, &mut $a_product_definition_relationship_name)?;
let mut $a_product_definition_relationship_description: Handle<HAsciiString> = Default::default();
let mut $has_product_definition_relationship_description: bool = true;
if $data.is_param_defined($num, 3)? {
$data.read_string($num, 3, "product_definition_relationship.description", $ach, &mut $a_product_definition_relationship_description)?;
} else {
$has_product_definition_relationship_description = false;
}
let mut $a_product_definition_relationship_relating_product_definition: StepBasicProductDefinitionOrReference = Default::default();
$data.read_entity_select($num, 4, "product_definition_relationship.relating_product_definition", $ach, &mut $a_product_definition_relationship_relating_product_definition)?;
};
}
pub(crate) use rw_block_0046;
