// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0034 {
($a_first:ident, $a_gtwm:ident, $a_last:ident, $a_modifiers:ident, $a_type:ident, $ach:ident, $an_it0:ident, $data:ident, $i0:ident, $nb0:ident, $num:ident, $num0:ident, $num2:ident, $sub:ident, $text:ident) => {
$data.named_for_complex("GEOMETRIC_TOLERANCE_WITH_MODIFIERS", $num0, &mut $num, $ach);
let mut $a_modifiers: Handle<HArray1<StepDimTolGeometricToleranceModifier>> = Default::default();
let mut $sub: i32 = 0;
if $data.read_sub_list($num, 1, "modifiers", $ach, &mut $sub, false, 0, 0)? {
let $nb0: i32 = $data.nb_params($sub)?;
$a_modifiers = step_new_array1::<StepDimTolGeometricToleranceModifier>(1, $nb0)?;
let $num2: i32 = $sub;
for $i0 in 1..=$nb0 {
let mut $an_it0: StepDimTolGeometricToleranceModifier = StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement;
if $data.param_type($num2, $i0)? == InterfaceParamType::ParamEnum {
let $text: Vec<u8> = $data.param_c_value($num2, $i0)?;
if $text == b".ANY_CROSS_SECTION." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMAnyCrossSection;
} else {
if $text == b".COMMON_ZONE." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMCommonZone;
} else {
if $text == b".EACH_RADIAL_ELEMENT." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMEachRadialElement;
} else {
if $text == b".FREE_self.state.read()." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMFreeState;
} else {
if $text == b".LEAST_MATERIAL_REQUIREMENT." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMLeastMaterialRequirement;
} else {
if $text == b".LINE_ELEMENT." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMLineElement;
} else {
if $text == b".MAJOR_DIAMETER." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMMajorDiameter;
} else {
if $text == b".MAXIMUM_MATERIAL_REQUIREMENT." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement;
} else {
if $text == b".MINOR_DIAMETER." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMMinorDiameter;
} else {
if $text == b".NOT_CONVEX." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMNotConvex;
} else {
if $text == b".PITCH_DIAMETER." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMPitchDiameter;
} else {
if $text == b".RECIPROCITY_REQUIREMENT." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMReciprocityRequirement;
} else {
if $text == b".SEPARATE_REQUIREMENT." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMSeparateRequirement;
} else {
if $text == b".STATISTICAL_TOLERANCE." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMStatisticalTolerance;
} else {
if $text == b".TANGENT_PLANE." {
$an_it0 = StepDimTolGeometricToleranceModifier::GTMTangentPlane;
} else {
$ach.add_fail_text("Parameter #5 (modifiers) has not allowed value", "");
}
}
}
}
}
}
}
}
}
}
}
}
}
}
}
} else {
$ach.add_fail_text("Parameter #5 (modifier) is not set of enumerations", "");
}
step_array1_set(&$a_modifiers, $i0, $an_it0, "ReadStep")?;
}
}
let $a_gtwm: Handle<StepDimTolGeometricToleranceWithModifiers> = Handle::new(StepDimTolGeometricToleranceWithModifiers::new());
$a_gtwm.set_modifiers(&$a_modifiers);
rw_block_b_0122!($a_first, $a_last, $a_type, $ach, $data, $num0);
};
}
pub(crate) use rw_block_b_0034;

macro_rules! rw_block_b_0035 {
    ($a_parent:ident, $a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $an_it0:ident, $i0:ident, $nb0:ident, $num2:ident, $sub2:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        rw_block_b_0065!(
            $a_representation_context_of_items,
            $a_representation_items,
            $a_representation_name,
            $an_it0,
            $i0,
            $nb0,
            $num2,
            $sub2,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_parent: Handle<StepKinematicsKinematicTopologyStructure> = Default::default();
        $the_data.read_entity(
            $the_num,
            4,
            "parent",
            $the_arch,
            StepKinematicsKinematicTopologyStructure::static_type(),
            &mut $a_parent,
        )?;
        $the_ent.init(
            &$a_representation_name,
            &$a_representation_items,
            &$a_representation_context_of_items,
            &$a_parent,
        );
    };
}
pub(crate) use rw_block_b_0035;

macro_rules! rw_block_b_0036 {
    ($a_kinematic_link_representation_represented_link:ident, $a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $an_it0:ident, $i0:ident, $nb0:ident, $num2:ident, $sub2:ident, $the_arch:ident, $the_data:ident, $the_ent:ident, $the_num:ident) => {
        rw_block_b_0065!(
            $a_representation_context_of_items,
            $a_representation_items,
            $a_representation_name,
            $an_it0,
            $i0,
            $nb0,
            $num2,
            $sub2,
            $the_arch,
            $the_data,
            $the_num
        );
        let mut $a_kinematic_link_representation_represented_link: Handle<
            StepKinematicsKinematicLink,
        > = Default::default();
        $the_data.read_entity(
            $the_num,
            4,
            "kinematic_link_representation.represented_link",
            $the_arch,
            StepKinematicsKinematicLink::static_type(),
            &mut $a_kinematic_link_representation_represented_link,
        )?;
        $the_ent.init(
            &$a_representation_name,
            &$a_representation_items,
            &$a_representation_context_of_items,
            &$a_kinematic_link_representation_represented_link,
        );
    };
}
pub(crate) use rw_block_b_0036;

macro_rules! rw_block_b_0037 {
    ($a_node_representation_model_ref:ident, $a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $ach:ident, $an_it0:ident, $data:ident, $ent:ident, $i0:ident, $nb0:ident, $num:ident, $num2:ident, $sub2:ident) => {
        rw_block_b_0066!(
            $a_representation_context_of_items,
            $a_representation_items,
            $a_representation_name,
            $ach,
            $an_it0,
            $data,
            $i0,
            $nb0,
            $num,
            $num2,
            $sub2
        );
        let mut $a_node_representation_model_ref: Handle<StepFEAFeaModel> = Default::default();
        $data.read_entity(
            $num,
            4,
            "node_representation.model_ref",
            $ach,
            StepFEAFeaModel::static_type(),
            &mut $a_node_representation_model_ref,
        )?;
        $ent.init(
            &$a_representation_name,
            &$a_representation_items,
            &$a_representation_context_of_items,
            &$a_node_representation_model_ref,
        );
    };
}
pub(crate) use rw_block_b_0037;

macro_rules! rw_block_b_0038 {
    ($a_representation_item_name:ident, $a_tessellated_edge_coordinates:ident, $a_tessellated_edge_geometric_link:ident, $a_tessellated_edge_line_strip:ident, $an_it0:ident, $has_tessellated_edge_geometric_link:ident, $i0:ident, $nb0:ident, $num2:ident, $sub4:ident, $the_check:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_item_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation_item.name",
            $the_check,
            &mut $a_representation_item_name,
        )?;
        let mut $a_tessellated_edge_coordinates: Handle<StepVisualCoordinatesList> =
            Default::default();
        $the_data.read_entity(
            $the_num,
            2,
            "tessellated_edge.coordinates",
            $the_check,
            StepVisualCoordinatesList::static_type(),
            &mut $a_tessellated_edge_coordinates,
        )?;
        let mut $a_tessellated_edge_geometric_link: StepVisualEdgeOrCurve = Default::default();
        let mut $has_tessellated_edge_geometric_link: bool = true;
        if $the_data.is_param_defined($the_num, 3)? {
            $the_data.read_entity_select(
                $the_num,
                3,
                "tessellated_edge.geometric_link",
                $the_check,
                &mut $a_tessellated_edge_geometric_link,
            )?;
        } else {
            $has_tessellated_edge_geometric_link = false;
            $a_tessellated_edge_geometric_link = StepVisualEdgeOrCurve::default();
        }
        let mut $a_tessellated_edge_line_strip: Handle<HArray1<i32>> = Default::default();
        let mut $sub4: i32 = 0;
        if $the_data.read_sub_list(
            $the_num,
            4,
            "tessellated_edge.line_strip",
            $the_check,
            &mut $sub4,
            false,
            0,
            0,
        )? {
            let $nb0: i32 = $the_data.nb_params($sub4)?;
            $a_tessellated_edge_line_strip = step_new_array1::<i32>(1, $nb0)?;
            let $num2: i32 = $sub4;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: i32 = Default::default();
                $the_data.read_integer($num2, $i0, "integer", $the_check, &mut $an_it0)?;
                step_array1_set(&$a_tessellated_edge_line_strip, $i0, $an_it0, "ReadStep")?;
            }
        }
    };
}
pub(crate) use rw_block_b_0038;

macro_rules! rw_block_b_0039 {
    ($a_item:ident, $a_name:ident, $a_over_ridden_style:ident, $a_styles:ident, $ach:ident, $anent2:ident, $data:ident, $i2:ident, $nb2:ident, $nsub2:ident, $num:ident) => {
        rw_block_b_0055!(
            $a_item, $a_name, $a_styles, $ach, $anent2, $data, $i2, $nb2, $nsub2, $num
        );
        let mut $a_over_ridden_style: Handle<StepVisualStyledItem> = Default::default();
        $data.read_entity(
            $num,
            4,
            "over_ridden_style",
            $ach,
            StepVisualStyledItem::static_type(),
            &mut $a_over_ridden_style,
        )?;
    };
}
pub(crate) use rw_block_b_0039;
