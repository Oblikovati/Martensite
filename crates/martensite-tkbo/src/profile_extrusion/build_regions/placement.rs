// SPDX-License-Identifier: GPL-2.0-only
//! Scoped graph proof in intrinsic UV, followed by a shared native location.
use super::{
    DECLINE, PlanarRegion, PlanarRegionSet, ProfileTopologyMode, Resolution, SketchProfile, Stage,
    cancelled_failure, program,
};
use crate::profile_extrusion::{
    DeclineCause, OperationDecline, PlanarProofVersion, PlanarProofWorkLimit,
    PlanarRegionWorkFailure, PlanarRegionWorkResult, ProfilePlane, ToleranceCap,
    admission::{admit_frame, admit_resolution},
    admitted::WorldFrame,
    topology::native_plane,
    validation::{SelfIntersectionEvidence, check_publication},
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::{
    gp::{Axis3, Direction3, Point3, Transform3},
    predicates::compare_scalar_triple_product_3d,
    toploc::Location,
};

pub(super) fn build_located(
    profile: &SketchProfile,
    resolution: Resolution,
    requested: PlanarProofWorkLimit,
    mode: ProfileTopologyMode,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> PlanarRegionWorkResult {
    let admission =
        |cause| PlanarRegionWorkFailure::new(requested, DECLINE.at(Stage::Admission, cause), None);
    admit_resolution(&resolution).map_err(admission)?;
    let frame = admit_frame(&profile.plane, resolution.angular).map_err(admission)?;
    let canonical = SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: profile.vertices.clone(),
        curves: profile.curves.clone(),
    };
    let regions = program::build_selected(
        &canonical,
        resolution,
        requested,
        PlanarProofVersion::CarrierV2,
        None,
        mode,
        cancelled,
    )?;
    place(regions, frame, requested, cancelled)
}

fn place(
    mut regions: PlanarRegionSet,
    frame: WorldFrame,
    requested: PlanarProofWorkLimit,
    cancelled: &mut impl FnMut() -> bool,
) -> PlanarRegionWorkResult {
    let work = Some(regions.proof_work());
    let failed = |decline| PlanarRegionWorkFailure::new(requested, decline, work);
    let mut poll = || {
        if cancelled() {
            return Err(cancelled_failure(requested, Stage::Construction, work));
        }
        Ok(())
    };
    poll()?;
    let (frame, location) = placement(frame).map_err(failed)?;
    if !location.is_identity() {
        for region in &mut regions.regions {
            poll()?;
            place_region(region, frame, &location, regions.resolution.cap).map_err(failed)?;
        }
        for member in &mut regions.curve_edges {
            poll()?;
            member
                .edge
                .move_by(&location, true)
                .map_err(native_placement)
                .map_err(failed)?;
        }
    }
    poll()?;
    Ok(regions)
}

fn placement(mut frame: WorldFrame) -> Result<(WorldFrame, Location), OperationDecline> {
    let to = native_plane(&frame).map_err(native_placement)?.position();
    if to.location() == Point3::default()
        && to.x_direction().coordinates() == Direction3::X.coordinates()
        && to.direction().coordinates() == Direction3::Z.coordinates()
    {
        return Ok((frame, Location::default()));
    }
    frame.normal = *to.direction().coordinates();
    frame.x = *to.x_direction().coordinates();
    frame.y = *to.y_direction().coordinates();
    if compare_scalar_triple_product_3d(frame.x, frame.y, frame.normal)
        != Ok(std::cmp::Ordering::Greater)
    {
        return Err(DECLINE.at(
            Stage::Construction,
            DeclineCause::InvalidPlane {
                reason: "placed basis must be finite and right-handed",
            },
        ));
    }
    Ok((
        frame,
        Location::new(Transform3::displacement(Axis3::default(), to)),
    ))
}

fn place_region(
    region: &mut PlanarRegion,
    frame: WorldFrame,
    location: &Location,
    cap: ToleranceCap,
) -> Result<(), OperationDecline> {
    region.frame = frame;
    region
        .face
        .move_by(location, true)
        .map_err(native_placement)?;
    for (_, edge) in &mut region.edges {
        edge.move_by(location, true).map_err(native_placement)?;
    }
    for (id, vertex) in &mut region.vertices {
        vertex.move_by(location, true).map_err(native_placement)?;
        let point = BRepTool::pnt(vertex).map_err(native_placement)?;
        if point
            .coordinates()
            .iter()
            .any(|component| !component.is_finite())
        {
            return Err(DECLINE.at(
                Stage::Construction,
                DeclineCause::NonFiniteVertex { vertex: *id },
            ));
        }
    }
    let evidence = SelfIntersectionEvidence::RegionContactCertificate;
    let checked = check_publication(&region.face, evidence, cap, DECLINE)?;
    region.validation = checked.validation;
    region.achieved = checked.achieved;
    Ok(())
}

fn native_placement(source: Failure) -> OperationDecline {
    DECLINE.native(Stage::Construction, "placed planar region", source)
}
