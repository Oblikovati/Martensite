// The RW member bodies several classes repeat.

macro_rules! rw_block_b_0217 {
($a_type:ident, $sw:ident) => {
if $a_type == StepDimTolGeometricToleranceType::GTTLineProfileTolerance {
step_start_entity($sw, "LINE_PROFILE_TOLERANCE")?;
} else {
if $a_type == StepDimTolGeometricToleranceType::GTTParallelismTolerance {
step_start_entity($sw, "PARALLELISM_TOLERANCE")?;
} else {
if $a_type == StepDimTolGeometricToleranceType::GTTPerpendicularityTolerance {
step_start_entity($sw, "PERPENDICULARITY_TOLERANCE")?;
} else {
if $a_type == StepDimTolGeometricToleranceType::GTTPositionTolerance {
step_start_entity($sw, "POSITION_TOLERANCE")?;
} else {
if $a_type == StepDimTolGeometricToleranceType::GTTRoundnessTolerance {
step_start_entity($sw, "ROUNDNESS_TOLERANCE")?;
} else {
if $a_type == StepDimTolGeometricToleranceType::GTTStraightnessTolerance {
step_start_entity($sw, "STRAIGHTNESS_TOLERANCE")?;
} else {
if $a_type == StepDimTolGeometricToleranceType::GTTSurfaceProfileTolerance {
step_start_entity($sw, "SURFACE_PROFILE_TOLERANCE")?;
} else {
if $a_type == StepDimTolGeometricToleranceType::GTTSymmetryTolerance {
step_start_entity($sw, "SYMMETRY_TOLERANCE")?;
} else {
if $a_type == StepDimTolGeometricToleranceType::GTTTotalRunoutTolerance {
step_start_entity($sw, "TOTAL_RUNOUT_TOLERANCE")?;
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
pub(crate) use rw_block_b_0217;

macro_rules! rw_block_b_0218 {
    ($i:ident, $the_ent:ident, $the_sw:ident) => {
        if !$the_ent.input_orientation().ypr_rotation().is_null() {
            $the_sw.open_sub()?;
            for $i in 1..=step_live(&$the_ent.input_orientation().ypr_rotation(), "WriteStep")?
                .array1()
                .length()
            {
                $the_sw.send_real(step_array1_item(
                    &$the_ent.input_orientation().ypr_rotation(),
                    $i,
                    "WriteStep",
                )?)?;
            }
            $the_sw.close_sub()?;
        } else {
            $the_sw.send_entity(&$the_ent.input_orientation().value())?;
        }
    };
}
pub(crate) use rw_block_b_0218;

macro_rules! rw_block_b_0219 {
    ($i1:ident, $the_ent:ident, $the_iter:ident, $var0:ident) => {
        for $i1 in 1..=step_live(&$the_ent.items(), "Share")?.array1().length() {
            let $var0: Handle<StepVisualTessellatedStructuredItem> =
                step_array1_item(&$the_ent.items(), $i1, "Share")?;
            $the_iter.add_item(&$var0.cast());
        }
    };
}
pub(crate) use rw_block_b_0219;
