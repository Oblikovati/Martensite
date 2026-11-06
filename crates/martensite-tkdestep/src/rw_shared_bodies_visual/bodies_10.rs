// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0078 {
($a_gtwm:ident, $a_type:ident, $ent:ident, $i:ident, $sw:ident) => {
step_start_entity($sw, "GEOMETRIC_TOLERANCE_WITH_MODIFIERS")?;
$sw.open_sub()?;
let $a_gtwm: Handle<StepDimTolGeometricToleranceWithModifiers> = $ent.get_geometric_tolerance_with_modifiers();
for $i in 1..=step_live(&$a_gtwm, "WriteStep")?.nb_modifiers() {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMAnyCrossSection {
$sw.send_enum(b".ANY_CROSS_SECTION.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMCommonZone {
$sw.send_enum(b".COMMON_ZONE.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMEachRadialElement {
$sw.send_enum(b".EACH_RADIAL_ELEMENT.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMFreeState {
$sw.send_enum(b".FREE_self.state.read().")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMLeastMaterialRequirement {
$sw.send_enum(b".LEAST_MATERIAL_REQUIREMENT.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMLineElement {
$sw.send_enum(b".LINE_ELEMENT.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMMajorDiameter {
$sw.send_enum(b".MAJOR_DIAMETER.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement {
$sw.send_enum(b".MAXIMUM_MATERIAL_REQUIREMENT.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMMinorDiameter {
$sw.send_enum(b".MINOR_DIAMETER.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMNotConvex {
$sw.send_enum(b".NOT_CONVEX.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMPitchDiameter {
$sw.send_enum(b".PITCH_DIAMETER.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMReciprocityRequirement {
$sw.send_enum(b".RECIPROCITY_REQUIREMENT.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMSeparateRequirement {
$sw.send_enum(b".SEPARATE_REQUIREMENT.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMStatisticalTolerance {
$sw.send_enum(b".STATISTICAL_TOLERANCE.")?;
} else {
if step_live(&$a_gtwm, "WriteStep")?.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMTangentPlane {
$sw.send_enum(b".TANGENT_PLANE.")?;
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
}
$sw.close_sub()?;
rw_block_b_0217!($a_type, $sw);
};
}
pub(crate) use rw_block_b_0078;

macro_rules! rw_block_b_0079 {
    ($a_definition:ident, $a_description:ident, $a_name:ident, $a_representation:ident, $a_type:ident, $ach:ident, $an_ent:ident, $an_items:ident, $data:ident, $ent:ident, $i:ident, $nb_elements:ident, $nb_sub:ident, $num:ident) => {
        let mut $an_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> = Default::default();
        let mut $an_ent: Handle<StepReprRepresentationItem> = Default::default();
        let mut $nb_sub: i32 = Default::default();
        let $a_type: InterfaceParamType = $data.param_type($num, 5)?;
        if $a_type == InterfaceParamType::ParamIdent {
            $data.read_entity(
                $num,
                5,
                "item_identified_representation_usage.identified_item",
                $ach,
                StepReprRepresentationItem::static_type(),
                &mut $an_ent,
            )?;
            $an_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, 1)?;
            step_array1_set(&$an_items, 1, $an_ent.clone(), "ReadStep")?;
        } else {
            if $data.read_sub_list(
                $num,
                5,
                "item_identified_representation_usage.identified_item",
                $ach,
                &mut $nb_sub,
                false,
                0,
                0,
            )? {
                let $nb_elements: i32 = $data.nb_params($nb_sub)?;
                $an_items = step_new_array1::<Handle<StepReprRepresentationItem>>(1, $nb_elements)?;
                for $i in 1..=$nb_elements {
                    if $data.read_entity(
                        $nb_sub,
                        $i,
                        "representation_item",
                        $ach,
                        StepReprRepresentationItem::static_type(),
                        &mut $an_ent,
                    )? {
                        step_array1_set(&$an_items, $i, $an_ent.clone(), "ReadStep")?;
                    }
                }
            }
        }
        $ent.init(
            &$a_name,
            &$a_description,
            &$a_definition,
            &$a_representation,
            &$an_items,
        );
    };
}
pub(crate) use rw_block_b_0079;

macro_rules! rw_block_b_0080 {
    ($a_base_type:ident, $an_array:ident, $ent:ident, $i:ident, $nb:ident, $sw:ident) => {
        rw_block_b_0165!($ent, $sw);
        let $a_base_type: i32 = $ent.base().case_num(&$ent.base().value());
        if $a_base_type == 1 {
            $sw.send_entity(&$ent.base().datum())?;
        } else {
            if $a_base_type == 2 {
                let $an_array: Handle<HArray1<Handle<StepDimTolDatumReferenceElement>>> =
                    $ent.base().common_datum_list();
                let $i: i32 = Default::default();
                let $nb: i32 = if $an_array.is_null() {
                    0
                } else {
                    $an_array.array1().length()
                };
                $sw.open_typed_sub(b"COMMON_DATUM_LIST")?;
                for $i in 1..=$nb {
                    $sw.send_entity(&step_array1_item(&$an_array, $i, "WriteStep")?)?;
                }
                $sw.close_sub()?;
            }
        }
    };
}
pub(crate) use rw_block_b_0080;

macro_rules! rw_block_b_0081 {
    ($a_tolerance_zone:ident, $ach:ident, $an_ent:ident, $an_items:ident, $data:ident, $i:ident, $nb_elements:ident, $nb_sub:ident, $num:ident) => {
        let mut $a_tolerance_zone: Handle<StepDimTolToleranceZone> = Default::default();
        $data.read_entity(
            $num,
            1,
            "tolerance_zone_definition.zone",
            $ach,
            StepDimTolToleranceZone::static_type(),
            &mut $a_tolerance_zone,
        )?;
        rw_block_b_0190!($an_ent, $an_items, $nb_sub);
        if $data.read_sub_list(
            $num,
            2,
            "tolerance_zone_definition.boundaries",
            $ach,
            &mut $nb_sub,
            false,
            0,
            0,
        )? {
            let $nb_elements: i32 = $data.nb_params($nb_sub)?;
            $an_items = step_new_array1::<Handle<StepReprShapeAspect>>(1, $nb_elements)?;
            for $i in 1..=$nb_elements {
                if $data.read_entity(
                    $nb_sub,
                    $i,
                    "shape_aspect",
                    $ach,
                    StepReprShapeAspect::static_type(),
                    &mut $an_ent,
                )? {
                    step_array1_set(&$an_items, $i, $an_ent.clone(), "ReadStep")?;
                }
            }
        }
    };
}
pub(crate) use rw_block_b_0081;
