//! `BOPAlgoCheckerSI::CheckFaceSelfIntersection` and its `BOPAlgoFaceSelfIntersect` task.
use super::BOPAlgoCheckerSI;
use super::completion::{
    SelfCheckCompletion, SelfCheckPhase, enter_face, enter_phase, record_policy,
    record_termination, schedule_face_phase,
};
use crate::bop_ds::{DataStructure, Pair};
use crate::int_tools::FaceFace;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topods::{self, Face};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::SurfaceType;

/// The first level of check that includes face/face interferences.
const FACE_FACE_LEVEL: i32 = 5;

/// The source faces whose surface type may intersect itself, with their indices.
fn self_intersection_candidates(
    structure: &DataStructure,
    mut trace: Option<&mut SelfCheckCompletion>,
) -> Result<Vec<(i32, Face)>, Failure> {
    let mut candidates = Vec::new();
    for index in 0..structure.nb_source_shapes() {
        let info = structure.shape_info(index)?;
        if info.shape_type() != ShapeEnum::Face {
            continue;
        }
        enter_face(trace.as_deref_mut(), index);
        let face = topods::face(info.shape())?;
        if may_self_intersect(&face, index, trace.as_deref_mut())? {
            candidates.push((index, face));
        }
    }
    Ok(candidates)
}

/// False for the analytic surfaces that cannot self-intersect, a torus included when its
/// major radius exceeds the minor one by more than the confusion tolerance.
fn may_self_intersect(
    face: &Face,
    index: i32,
    trace: Option<&mut SelfCheckCompletion>,
) -> Result<bool, Failure> {
    let surface = BRepAdaptorSurface::with_face(face, false)?;
    let kind = surface.get_type()?;
    let candidate = match kind {
        SurfaceType::Plane | SurfaceType::Cylinder | SurfaceType::Cone | SurfaceType::Sphere => {
            false
        }
        SurfaceType::Torus => {
            let torus = surface.torus()?;
            torus.major_radius() <= torus.minor_radius() + precision::confusion()
        }
        _ => true,
    };
    record_policy(trace, index, kind, candidate);
    Ok(candidate)
}

/// Whether intersecting a face with itself gives a curve or a point
/// (`BOPAlgoFaceSelfIntersect::Perform`), independently of execution-progress policy.
fn self_face_termination(
    face: &Face,
    run_parallel: bool,
) -> Result<super::FaceTermination, Failure> {
    let mut intersector = FaceFace::default();
    intersector.perform(face, face, run_parallel)?;
    if !intersector.is_done() {
        return Ok(super::FaceTermination::NotDone);
    }
    let has_contacts = !intersector.lines()?.is_empty() || !intersector.points().is_empty();
    Ok(super::FaceTermination::Terminated { has_contacts })
}

fn add_self_pair(structure: &mut DataStructure, index: i32) {
    let pair = Pair::new(index, index);
    structure.interference_table_mut().add(pair);
}

#[path = "face_self/progress.rs"]
mod progress;
