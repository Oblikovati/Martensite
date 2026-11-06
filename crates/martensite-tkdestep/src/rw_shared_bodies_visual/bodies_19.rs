// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0199 {
    ($ent:ident, $i1:ident, $sw:ident) => {
        $sw.open_sub()?;
        for $i1 in 1..=$ent.nb_items() {
            $sw.send_entity(&$ent.items_value($i1)?.value())?;
        }
        $sw.close_sub()?;
    };
}
pub(crate) use rw_block_b_0199;

macro_rules! rw_block_b_0200 {
($a_first:ident, $a_last:ident, $a_type:ident, $ach:ident) => {
let mut $a_type: StepDimTolGeometricToleranceType = StepDimTolGeometricToleranceType::GTTPositionTolerance;
if $a_first == b"ANGULARITY_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTAngularityTolerance;
} else {
if $a_first == b"CIRCULAR_RUNOUT_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTCircularRunoutTolerance;
} else {
if $a_first == b"COAXIALITY_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTCoaxialityTolerance;
} else {
if $a_first == b"CONCENTRICITY_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTConcentricityTolerance;
} else {
if $a_first == b"CYLINDRICITY_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTCylindricityTolerance;
} else {
if $a_first == b"FLATNESS_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTFlatnessTolerance;
} else {
if $a_last == b"LINE_PROFILE_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTLineProfileTolerance;
} else {
if $a_last == b"PARALLELISM_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTParallelismTolerance;
} else {
if $a_last == b"PERPENDICULARITY_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTPerpendicularityTolerance;
} else {
if $a_last == b"POSITION_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTPositionTolerance;
} else {
if $a_last == b"ROUNDNESS_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTRoundnessTolerance;
} else {
if $a_last == b"STRAIGHTNESS_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTStraightnessTolerance;
} else {
if $a_last == b"SURFACE_PROFILE_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTSurfaceProfileTolerance;
} else {
if $a_last == b"SYMMETRY_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTSymmetryTolerance;
} else {
if $a_last == b"TOTAL_RUNOUT_TOLERANCE" {
$a_type = StepDimTolGeometricToleranceType::GTTTotalRunoutTolerance;
} else {
$ach.add_fail_text("The type of geometric tolerance is not supported", "");
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
};
}
pub(crate) use rw_block_b_0200;

macro_rules! rw_block_b_0201 {
    ($a_input_orientation:ident, $a_items:ident, $an_it0:ident, $i0:ident, $nb:ident, $nsub:ident, $num2:ident, $the_arch:ident, $the_data:ident, $the_num:ident) => {
        let mut $a_input_orientation: StepKinematicsSpatialRotation = Default::default();
        if $the_data.sub_list_number($the_num, 4, true)? != 0 {
            let mut $a_items: Handle<HArray1<f64>> = Default::default();
            let mut $nsub: i32 = 0;
            if $the_data.read_sub_list($the_num, 4, "items", $the_arch, &mut $nsub, false, 0, 0)? {
                let $nb: i32 = $the_data.nb_params($nsub)?;
                $a_items = step_new_array1::<f64>(1, $nb)?;
                let $num2: i32 = $nsub;
                for $i0 in 1..=$nb {
                    let mut $an_it0: f64 = Default::default();
                    $the_data.read_real($num2, $i0, "real", $the_arch, &mut $an_it0)?;
                    step_array1_set(&$a_items, $i0, $an_it0, "ReadStep")?;
                }
            }
            $a_input_orientation.set_value(&$a_items.cast())?;
        } else {
            $the_data.read_entity_select(
                $the_num,
                4,
                "input_orientation",
                $the_arch,
                &mut $a_input_orientation,
            )?;
        }
    };
}
pub(crate) use rw_block_b_0201;

macro_rules! rw_block_b_0202 {
    ($a_type:ident, $ent:ident, $sw:ident) => {
        let $a_type: StepDimTolGeometricToleranceType = $ent.get_tolerance_type();
        if $a_type == StepDimTolGeometricToleranceType::GTTAngularityTolerance {
            step_start_entity($sw, "ANGULARITY_TOLERANCE")?;
        } else {
            if $a_type == StepDimTolGeometricToleranceType::GTTCircularRunoutTolerance {
                step_start_entity($sw, "CIRCULAR_RUNOUT_TOLERANCE")?;
            } else {
                if $a_type == StepDimTolGeometricToleranceType::GTTCoaxialityTolerance {
                    step_start_entity($sw, "COAXIALITY_TOLERANCE")?;
                } else {
                    if $a_type == StepDimTolGeometricToleranceType::GTTConcentricityTolerance {
                        step_start_entity($sw, "CONCENTRICITY_TOLERANCE")?;
                    } else {
                        if $a_type == StepDimTolGeometricToleranceType::GTTCylindricityTolerance {
                            step_start_entity($sw, "CYLINDRICITY_TOLERANCE")?;
                        } else {
                            if $a_type == StepDimTolGeometricToleranceType::GTTFlatnessTolerance {
                                step_start_entity($sw, "FLATNESS_TOLERANCE")?;
                            }
                        }
                    }
                }
            }
        }
    };
}
pub(crate) use rw_block_b_0202;

macro_rules! rw_block_b_0203 {
    ($ent:ident, $sw:ident) => {
        if $ent.rendering_method() == StepVisualShadingSurfaceMethod::SsmConstantShading {
            $sw.send_enum(b".CONSTANT_SHADING.")?;
        } else {
            if $ent.rendering_method() == StepVisualShadingSurfaceMethod::SsmColourShading {
                $sw.send_enum(b".COLOUR_SHADING.")?;
            } else {
                if $ent.rendering_method() == StepVisualShadingSurfaceMethod::SsmDotShading {
                    $sw.send_enum(b".DOT_SHADING.")?;
                } else {
                    if $ent.rendering_method() == StepVisualShadingSurfaceMethod::SsmNormalShading {
                        $sw.send_enum(b".NORMAL_SHADING.")?;
                    }
                }
            }
        }
        $sw.send_entity(&$ent.surface_colour())?;
    };
}
pub(crate) use rw_block_b_0203;
