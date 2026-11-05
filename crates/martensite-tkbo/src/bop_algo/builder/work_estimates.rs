//! Native work estimates count active DS entries, never geometrical measurements or elapsed time.
use super::ranged_driver::BuilderProgressProfile;
use crate::bop_algo::{
    BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoMakerVolume, BOPAlgoSection, PISteps,
    progress::analyze_progress_with,
};
use crate::bop_ds::DataStructure;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(crate) const HISTORY_STEP: i32 = 8;
pub(crate) const POST_TREATMENT_STEP: i32 = 9;
pub(crate) const VOLUME_SOLIDS_STEP: i32 = 10;
pub(crate) const BOOLEAN_RESULT_STEP: i32 = 10;
pub(crate) const SECTION_RESULT_STEP: i32 = 2;
pub(crate) const SECTION_HISTORY_STEP: i32 = 3;
pub(crate) const SECTION_POST_TREATMENT_STEP: i32 = 4;

impl BOPAlgoBuilder {
    /// Reserves history and post-treatment weights without clearing variable slots.
    /// Example: `builder.fill_pi_constants(2000.0, &mut steps);` reserves 100 and 60 units.
    pub fn fill_pi_constants(&self, whole: f64, steps: &mut PISteps) {
        reserve_builder_constants(
            whole,
            steps,
            self.has_history(),
            BuilderProgressProfile::GeneralFuse,
        );
    }
}

impl BOPAlgoBOP {
    /// Adds the selected Boolean result reservation to the base constant weights.
    /// Example: `boolean.fill_pi_constants(2000.0, &mut steps);` reserves 200 result units for Fuse.
    pub fn fill_pi_constants(&self, whole: f64, steps: &mut PISteps) {
        let profile = BuilderProgressProfile::Boolean {
            operation: self.operation(),
        };
        reserve_builder_constants(whole, steps, self.has_history(), profile);
    }
}

impl BOPAlgoSection {
    /// Reserves Section's own history and post-treatment slots, retaining caller bounds.
    /// Example: `section.fill_pi_constants(2000.0, &mut steps);` reserves 200 and 100 units.
    pub fn fill_pi_constants(&self, whole: f64, steps: &mut PISteps) {
        reserve_builder_constants(
            whole,
            steps,
            self.has_history(),
            BuilderProgressProfile::Section,
        );
    }
}

impl BOPAlgoBuilder {
    /// Fills the raw General Fuse image weights, preserving every other slot and neither
    /// normalizing nor reserving (`fillPISteps`); the data structure of the run is explicit.
    /// Example: `builder.fill_pi_steps(filler.data_structure().unwrap(), &mut steps)?;`.
    pub fn fill_pi_steps(
        &self,
        structure: &DataStructure,
        steps: &mut PISteps,
    ) -> Result<(), Failure> {
        let profile = BuilderProgressProfile::GeneralFuse;
        populate_work_steps(steps, &source_work_counts(structure)?, profile);
        Ok(())
    }
}

impl BOPAlgoSection {
    /// Fills Section's own raw vertex, edge and result weights, preserving every other slot
    /// (`fillPISteps`); the data structure of the run is explicit.
    /// Example: `section.fill_pi_steps(filler.data_structure().unwrap(), &mut steps)?;`.
    pub fn fill_pi_steps(
        &self,
        structure: &DataStructure,
        steps: &mut PISteps,
    ) -> Result<(), Failure> {
        let profile = BuilderProgressProfile::Section;
        populate_work_steps(steps, &source_work_counts(structure)?, profile);
        Ok(())
    }
}

impl BOPAlgoMakerVolume {
    /// Fills the raw solid weight and, when this receiver intersects, its image weights,
    /// preserving every other slot (`fillPISteps`); the data structure of the run is explicit.
    /// Example: `volume.fill_pi_steps(filler.data_structure().unwrap(), &mut steps)?;`.
    pub fn fill_pi_steps(
        &self,
        structure: &DataStructure,
        steps: &mut PISteps,
    ) -> Result<(), Failure> {
        let profile = BuilderProgressProfile::MakerVolume {
            intersect: self.is_intersect(),
        };
        populate_work_steps(steps, &source_work_counts(structure)?, profile);
        Ok(())
    }
}

/// Counts only the actual work entries selected by native getNbShapes.
/// Example: `let counts = source_work_counts(structure)?;` does not initialize any pool.
pub(crate) fn source_work_counts(structure: &DataStructure) -> Result<[usize; 8], Failure> {
    let mut counts = [0; 8];
    counts[0] = structure.same_domain_binding_count();
    for index in 0..structure.nb_source_shapes() {
        if let Some(slot) = source_work_slot(structure, index)? {
            counts[slot] += 1;
        }
    }
    Ok(counts)
}

fn source_work_slot(structure: &DataStructure, index: i32) -> Result<Option<usize>, Failure> {
    let kind = structure.shape_info(index)?.shape_type();
    Ok(match kind {
        ShapeEnum::Edge => structure.has_pave_blocks(index)?.then_some(1),
        ShapeEnum::Wire => Some(2),
        ShapeEnum::Face => structure.has_face_info(index)?.then_some(3),
        ShapeEnum::Shell => Some(4),
        ShapeEnum::Solid => Some(5),
        ShapeEnum::CompSolid => Some(6),
        ShapeEnum::Compound => Some(7),
        _ => None,
    })
}

/// Shares canonical normalization and the actual receiver's source profile.
/// Example: `let steps = builder_work_estimates(structure, profile, fill_history)?;`.
pub(crate) fn builder_work_estimates(
    structure: &DataStructure,
    profile: BuilderProgressProfile,
    fill_history: bool,
) -> Result<PISteps, Failure> {
    let counts = source_work_counts(structure)?;
    let mut steps = allocated_work_steps(profile)?;
    analyze_progress_with(
        100.0,
        &mut steps,
        |whole, target| reserve_builder_constants(whole, target, fill_history, profile),
        |target| populate_work_steps(target, &counts, profile),
    );
    Ok(steps)
}

fn allocated_work_steps(profile: BuilderProgressProfile) -> Result<PISteps, Failure> {
    let count = match profile {
        BuilderProgressProfile::GeneralFuse | BuilderProgressProfile::Splitter => {
            VOLUME_SOLIDS_STEP
        }
        BuilderProgressProfile::Section => SECTION_POST_TREATMENT_STEP + 1,
        BuilderProgressProfile::MakerVolume { .. } | BuilderProgressProfile::Boolean { .. } => {
            VOLUME_SOLIDS_STEP + 1
        }
    };
    PISteps::new(count).map_err(|cause| {
        Failure::out_of_range(&format!(
            "progress_stage_count={count}; expected GeneralFuse/Splitter10, Section5 or MakerVolume/Boolean11: {cause}"
        ))
    })
}

fn reserve_builder_constants(
    whole: f64,
    steps: &mut PISteps,
    fill_history: bool,
    profile: BuilderProgressProfile,
) {
    if profile == BuilderProgressProfile::Section {
        reserve_section_constants(whole, steps, fill_history);
        return;
    }
    if fill_history {
        steps.set_step(HISTORY_STEP, 0.05 * whole);
    }
    steps.set_step(POST_TREATMENT_STEP, 0.03 * whole);
    if let BuilderProgressProfile::Boolean { operation } = profile {
        reserve_boolean_result(whole, steps, operation);
    }
}

fn reserve_section_constants(whole: f64, steps: &mut PISteps, fill_history: bool) {
    // The reference's evaluation order matters for arbitrary budgets, beyond the driver’s fixed 100.
    if fill_history {
        steps.set_step(SECTION_HISTORY_STEP, 10.0 * whole / 100.0);
    }
    steps.set_step(SECTION_POST_TREATMENT_STEP, 5.0 * whole / 100.0);
}
fn reserve_boolean_result(whole: f64, steps: &mut PISteps, operation: crate::bop_algo::Operation) {
    let percent = if operation == crate::bop_algo::Operation::Fuse {
        10.0
    } else {
        5.0
    };
    steps.set_step(BOOLEAN_RESULT_STEP, percent * whole / 100.0);
}

fn populate_work_steps(steps: &mut PISteps, counts: &[usize; 8], profile: BuilderProgressProfile) {
    match profile {
        BuilderProgressProfile::Section => {
            steps.set_step(0, counts[0] as f64);
            steps.set_step(1, counts[1] as f64);
            steps.set_step(SECTION_RESULT_STEP, (counts[1] + counts[3]) as f64);
        }
        BuilderProgressProfile::GeneralFuse
        | BuilderProgressProfile::Splitter
        | BuilderProgressProfile::Boolean { .. } => populate_general_fuse_work(steps, counts),
        BuilderProgressProfile::MakerVolume { intersect } => {
            populate_volume_work(steps, counts, intersect);
        }
    }
}

fn populate_general_fuse_work(steps: &mut PISteps, counts: &[usize; 8]) {
    for (index, count) in counts.iter().enumerate() {
        let factor = match index {
            3 => 20.0,
            5 => 50.0,
            _ => 1.0,
        };
        steps.set_step(index as i32, factor * *count as f64);
    }
}

fn populate_volume_work(steps: &mut PISteps, counts: &[usize; 8], intersect: bool) {
    steps.set_step(VOLUME_SOLIDS_STEP, 50.0 * counts[3] as f64);
    if !intersect {
        return;
    }
    for (index, count) in counts.iter().take(4).enumerate() {
        let factor = if index == 3 { 50.0 } else { 1.0 };
        steps.set_step(index as i32, factor * *count as f64);
    }
}

#[cfg(test)]
#[path = "work_estimates_contract.rs"]
mod contract;
