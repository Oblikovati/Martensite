//! PaveFiller's fourteen native slots retain input-dependent stage proportions.
use super::{BOPAlgoPaveFiller, Glue, missing_structure};
use crate::bop_algo::{PISteps, progress::analyze_progress_with};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

pub(super) const SLOT_COUNT: i32 = 14;

impl BOPAlgoPaveFiller {
    /// Reserves preparation's constant contribution, preserving every other slot.
    /// Example: `filler.fill_pi_constants(2000.0, &mut steps);` reserves 20 units.
    pub fn fill_pi_constants(&self, whole: f64, steps: &mut PISteps) {
        reserve_preparation(self.non_destructive, whole, steps);
    }

    /// Fill raw variable weights on this concrete receiver, preserving all other slots.
    /// Example: `filler.initialize()?; filler.fill_pi_steps(&mut steps)?;`.
    /// Requires a prepared iterator; an absent iterator returns `StandardNullObject`.
    /// The iterator is reset to its first face/face pair even if no target slot is in bounds.
    /// This concrete callback neither normalizes weights nor supplies virtual customization.
    pub fn fill_pi_steps(&mut self, steps: &mut PISteps) -> Result<(), Failure> {
        let counts = interference_counts(self)?;
        self.iterator
            .as_mut()
            .ok_or_else(missing_structure)?
            .initialize(ShapeEnum::Face, ShapeEnum::Face);
        populate_intersections(counts, self.glue, self.avoid_build_pcurve, steps);
        Ok(())
    }
}

/// Computes weights after actual initialization without disturbing its iterator cursor.
/// Example: `let weights = intersection_weights(filler)?;` before any stage runs.
pub(super) fn intersection_weights(filler: &BOPAlgoPaveFiller) -> Result<PISteps, Failure> {
    let counts = interference_counts(filler)?;
    let mut steps = PISteps::new(SLOT_COUNT).map_err(|error| Failure::new(&error.to_string()))?;
    analyze_progress_with(
        95.0,
        &mut steps,
        |whole, steps| filler.fill_pi_constants(whole, steps),
        |steps| populate_intersections(counts, filler.glue, filler.avoid_build_pcurve, steps),
    );
    Ok(steps)
}

fn reserve_preparation(non_destructive: bool, whole: f64, steps: &mut PISteps) {
    if non_destructive {
        return;
    }
    steps.set_step(0, whole / 100.0);
}

fn interference_counts(filler: &BOPAlgoPaveFiller) -> Result<[i32; 6], Failure> {
    use ShapeEnum::{Edge, Face, Vertex};
    let iterator = filler.iterator.as_ref().ok_or_else(missing_structure)?;
    let pairs = [
        (Vertex, Vertex),
        (Vertex, Edge),
        (Edge, Edge),
        (Vertex, Face),
        (Edge, Face),
        (Face, Face),
    ];
    let mut counts = pairs.map(|(first, second)| iterator.candidate_count(first, second) as i32);
    if filler.glue == Glue::Full {
        counts[4] = 0;
    }
    Ok(counts)
}

fn populate_intersections(counts: [i32; 6], glue: Glue, avoid: bool, steps: &mut PISteps) {
    let weights = variable_weights(counts, glue == Glue::Full, avoid);
    for (index, weight) in weights.into_iter().enumerate() {
        steps.set_step(index as i32 + 1, weight);
    }
}

fn variable_weights(counts: [i32; 6], full_glue: bool, avoid: bool) -> [f64; 13] {
    let [vv, ve, ee, vf, ef, ff] = counts;
    [
        f64::from(vv),
        f64::from(2 * ve),
        f64::from(5 * ee),
        f64::from(5 * vf),
        f64::from(10 * ef),
        0.2 * f64::from(vv + ve + vf),
        f64::from(2 * ee),
        f64::from(2 * ef),
        f64::from(if full_glue { ff } else { 30 * ff }),
        f64::from(ee),
        f64::from(if full_glue { 0 } else { 5 * ff }),
        if avoid { 0.0 } else { 0.2 * f64::from(ee + ef) },
        0.1 * f64::from(ee),
    ]
}

#[cfg(test)]
#[path = "weights_contract.rs"]
mod contract;
