//! Prepare planar p-curves without coupling preparation to split-edge construction.
use super::progress_observation::native_user_break;
use super::{BOPAlgoPaveFiller, missing_structure};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp,
    topods::{Edge, Face, Shape},
    toptools::ShapeMapHasher,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::IndexedMap,
    standard::Failure,
};
use martensite_tkg3d::{
    geom::{GeomPlane, GeomRectangularTrimmedSurface},
    top_abs::ShapeEnum,
};

#[cfg(test)]
#[path = "preparation/contract.rs"]
mod contract;
#[cfg(test)]
#[path = "preparation/fixture.rs"]
mod fixture;
#[cfg(test)]
#[path = "preparation/reporter.rs"]
mod reporter;
#[path = "preparation/task.rs"]
mod task;
use task::{PlanarPcurveInput, PlanarPcurveWork, execute_planar_inputs};
type PlanarFaceMap = IndexedMap<Shape, ShapeMapHasher>;

impl BOPAlgoPaveFiller {
    /// Stores planar input p-curves unless the operation is non-destructive.
    /// Example: `filler.prepare()?` retains the unbound default entry.
    pub fn prepare(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        let observed = ProgressCancellation::default();
        self.prepare_with_progress(&range, &observed).map(|_| ())
    }

    /// Forward actual BPC ranges and preserve native post-worker/update polls.
    /// Example: `filler.prepare_with_progress(range, observed)?` before intersections.
    pub(in crate::bop_algo::pave_filler) fn prepare_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        if self.non_destructive {
            return Ok(Vec::new());
        }
        let outer = ProgressScope::unnamed(range, 1.0, false);
        let faces = self.planar_preparation_faces()?;
        if faces.extent() == 0 {
            return Ok(Vec::new());
        }
        let inputs = planar_preparation_inputs(&faces)?;
        self.complete_planar_preparation(inputs, &outer, observed)
    }

    fn planar_preparation_faces(&mut self) -> Result<PlanarFaceMap, Failure> {
        let mut faces = PlanarFaceMap::with_hasher(ShapeMapHasher, 1);
        for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
            self.initialize_progress_pairs(kind, ShapeEnum::Face)?;
            while self.progress_pair_cursor()?.more() {
                let (_, face) = self.progress_pair_cursor()?.value()?;
                self.add_planar_preparation_face(face, &mut faces)?;
                self.progress_pair_cursor()?.next();
            }
        }
        Ok(faces)
    }

    fn add_planar_preparation_face(
        &self,
        face: i32,
        faces: &mut PlanarFaceMap,
    ) -> Result<(), Failure> {
        let shape = self
            .data_structure
            .as_ref()
            .ok_or_else(missing_structure)?
            .shape(face)?
            .clone();
        if is_based_on_plane(&Face::unchecked(shape.clone()))? {
            faces.add(shape);
        }
        Ok(())
    }

    fn complete_planar_preparation(
        &mut self,
        inputs: Vec<PlanarPcurveInput>,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let tasks = planar_preparation_scope(outer, inputs.len());
        let mut work =
            execute_planar_inputs(inputs, &tasks, self.options.run_parallel(), observed)?;
        let completions = planar_preparation_completions(&mut work)?;
        if native_user_break(&self.options, &tasks, observed) {
            return Ok(completions);
        }
        self.store_planar_preparation(work, outer, observed)?;
        Ok(completions)
    }

    fn store_planar_preparation(
        &mut self,
        work: Vec<PlanarPcurveWork<'_>>,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        for item in work {
            if native_user_break(&self.options, outer, observed) {
                return Ok(());
            }
            item.store_curve()?;
        }
        Ok(())
    }
}

fn planar_preparation_inputs(faces: &PlanarFaceMap) -> Result<Vec<PlanarPcurveInput>, Failure> {
    let mut inputs = Vec::new();
    for face in faces.iter() {
        let face = Face::unchecked(face.clone());
        for edge in topexp::Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)? {
            inputs.push(PlanarPcurveInput {
                edge: Edge::unchecked(edge),
                face: face.clone(),
            });
        }
    }
    Ok(inputs)
}

fn planar_preparation_scope<'a>(outer: &'a ProgressScope<'a>, count: usize) -> ProgressScope<'a> {
    let range = outer.next(1.0);
    ProgressScope::with_literal(
        &range,
        "Building 2d curves on planar faces",
        count as f64,
        false,
    )
}

fn planar_preparation_completions(
    work: &mut [PlanarPcurveWork<'_>],
) -> Result<Vec<ProgressCompletion>, Failure> {
    let mut completions = Vec::new();
    for item in work {
        let completion = item.take_completion()?;
        if !matches!(completion, ProgressCompletion::Completed) {
            completions.push(completion);
        }
    }
    Ok(completions)
}

/// Native static `IsBasedOnPlane`: a plane or a rectangular trimmed plane.
fn is_based_on_plane(face: &Face) -> Result<bool, Failure> {
    let (surface, _) = BRepTool::surface_with_location(face)?;
    let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
    if let Some(trimmed) = trimmed.get() {
        return Ok(trimmed.basis_surface().cast::<GeomPlane>().is_some());
    }
    Ok(surface.cast::<GeomPlane>().is_some())
}
