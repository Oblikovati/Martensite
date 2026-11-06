// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0062 {
($ent:ident, $i:ident, $sw:ident) => {
rw_block_b_0161!($ent, $sw);
$sw.open_sub()?;
for $i in 1..=$ent.nb_modifiers() {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMAnyCrossSection {
$sw.send_enum(b".ANY_CROSS_SECTION.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMCommonZone {
$sw.send_enum(b".COMMON_ZONE.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMEachRadialElement {
$sw.send_enum(b".EACH_RADIAL_ELEMENT.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMFreeState {
$sw.send_enum(b".FREE_self.state.read().")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMLeastMaterialRequirement {
$sw.send_enum(b".LEAST_MATERIAL_REQUIREMENT.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMLineElement {
$sw.send_enum(b".LINE_ELEMENT.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMMajorDiameter {
$sw.send_enum(b".MAJOR_DIAMETER.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMMaximumMaterialRequirement {
$sw.send_enum(b".MAXIMUM_MATERIAL_REQUIREMENT.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMMinorDiameter {
$sw.send_enum(b".MINOR_DIAMETER.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMNotConvex {
$sw.send_enum(b".NOT_CONVEX.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMPitchDiameter {
$sw.send_enum(b".PITCH_DIAMETER.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMReciprocityRequirement {
$sw.send_enum(b".RECIPROCITY_REQUIREMENT.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMSeparateRequirement {
$sw.send_enum(b".SEPARATE_REQUIREMENT.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMStatisticalTolerance {
$sw.send_enum(b".STATISTICAL_TOLERANCE.")?;
} else {
if $ent.modifier_value($i)? == StepDimTolGeometricToleranceModifier::GTMTangentPlane {
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
};
}
pub(crate) use rw_block_b_0062;

macro_rules! rw_block_b_0063 {
    ($a_base_type:ident, $a_modifier:ident, $a_type:ident, $an_array:ident, $ent:ident, $i:ident, $nb:ident, $sw:ident) => {
        rw_block_b_0080!($a_base_type, $an_array, $ent, $i, $nb, $sw);
        if $ent.has_modifiers()? {
            let $i: i32 = Default::default();
            let $nb: i32 = $ent.nb_modifiers();
            $sw.open_sub()?;
            for $i in 1..=$nb {
                let $a_modifier: StepDimTolDatumReferenceModifier = $ent.modifiers_value($i)?;
                let $a_type: i32 = $a_modifier.case_num(&$a_modifier.value());
                if $a_type == 1 {
                    $sw.send_entity(&$a_modifier.datum_reference_modifier_with_value())?;
                } else {
                    if $a_type == 2 {
                        $sw.send_enum(
                            step_live(
                                &$a_modifier.simple_datum_reference_modifier_member(),
                                "WriteStep",
                            )?
                            .enum_text()
                            .as_bytes(),
                        )?;
                    }
                }
            }
            $sw.close_sub()?;
        } else {
            $sw.send_undef()?;
        }
    };
}
pub(crate) use rw_block_b_0063;

macro_rules! rw_block_b_0064 {
    ($a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $an_it0:ident, $i0:ident, $nb0:ident, $num2:ident, $sub2:ident, $the_check:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation.name",
            $the_check,
            &mut $a_representation_name,
        )?;
        let mut $a_representation_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> =
            Default::default();
        let mut $sub2: i32 = 0;
        if $the_data.read_sub_list(
            $the_num,
            2,
            "representation.items",
            $the_check,
            &mut $sub2,
            false,
            0,
            0,
        )? {
            let $nb0: i32 = $the_data.nb_params($sub2)?;
            $a_representation_items =
                step_new_array1::<Handle<StepReprRepresentationItem>>(1, $nb0)?;
            let $num2: i32 = $sub2;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: Handle<StepReprRepresentationItem> = Default::default();
                $the_data.read_entity(
                    $num2,
                    $i0,
                    "representation_item",
                    $the_check,
                    StepReprRepresentationItem::static_type(),
                    &mut $an_it0,
                )?;
                step_array1_set(&$a_representation_items, $i0, $an_it0.clone(), "ReadStep")?;
            }
        }
        let mut $a_representation_context_of_items: Handle<StepReprRepresentationContext> =
            Default::default();
        $the_data.read_entity(
            $the_num,
            3,
            "representation.context_of_items",
            $the_check,
            StepReprRepresentationContext::static_type(),
            &mut $a_representation_context_of_items,
        )?;
    };
}
pub(crate) use rw_block_b_0064;

macro_rules! rw_block_b_0065 {
    ($a_representation_context_of_items:ident, $a_representation_items:ident, $a_representation_name:ident, $an_it0:ident, $i0:ident, $nb0:ident, $num2:ident, $sub2:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_representation_name: Handle<HAsciiString> = Default::default();
        $the_data.read_string(
            $the_num,
            1,
            "representation.name",
            $the_arch,
            &mut $a_representation_name,
        )?;
        let mut $a_representation_items: Handle<HArray1<Handle<StepReprRepresentationItem>>> =
            Default::default();
        let mut $sub2: i32 = 0;
        if $the_data.read_sub_list(
            $the_num,
            2,
            "representation.items",
            $the_arch,
            &mut $sub2,
            false,
            0,
            0,
        )? {
            let $nb0: i32 = $the_data.nb_params($sub2)?;
            $a_representation_items =
                step_new_array1::<Handle<StepReprRepresentationItem>>(1, $nb0)?;
            let $num2: i32 = $sub2;
            for $i0 in 1..=$nb0 {
                let mut $an_it0: Handle<StepReprRepresentationItem> = Default::default();
                $the_data.read_entity(
                    $num2,
                    $i0,
                    "representation_item",
                    $the_arch,
                    StepReprRepresentationItem::static_type(),
                    &mut $an_it0,
                )?;
                step_array1_set(&$a_representation_items, $i0, $an_it0.clone(), "ReadStep")?;
            }
        }
        let mut $a_representation_context_of_items: Handle<StepReprRepresentationContext> =
            Default::default();
        $the_data.read_entity(
            $the_num,
            3,
            "representation.context_of_items",
            $the_arch,
            StepReprRepresentationContext::static_type(),
            &mut $a_representation_context_of_items,
        )?;
    };
}
pub(crate) use rw_block_b_0065;
