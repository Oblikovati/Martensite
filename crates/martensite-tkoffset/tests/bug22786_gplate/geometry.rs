//! Literal five-boundary gplate calls; no n-sided filling substitute or geometry retry.
use martensite_tkbool::brep_fill::BRepFillCurveConstraint;
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorCurve,
    topods::{self, Edge, Face, Shape, ShapeIterator},
};
use martensite_tkernel::{
    message::ProgressRange,
    ncollection::Sequence,
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, geom::GeomSurface};
use martensite_tkgeomalgo::geom_plate::{
    GeomPlateBuildPlateSurface, GeomPlateCurveConstraint, GeomPlateMakeApprox,
    GeomPlatePlateG0Criterion, GeomPlateSurface,
};
use martensite_tkgeombase::adv_app2var::{AdvApp2VarCriterionRepartition, AdvApp2VarCriterionType};
use martensite_tkmath::{
    geom_abs::Shape as Continuity,
    gp::{Xy, Xyz},
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

/// Preserve default immediate-child aliases, including accumulated orientation/location.
/// Example: `let edges = boundary_edges(&restored)?`.
pub(super) fn boundary_edges(shape: &Shape) -> Result<Vec<Edge>, Failure> {
    ShapeIterator::new(shape, true, true)
        .map(|child| topods::edge(&child))
        .collect()
}

/// Run the exact five-curve command through its existing algorithms; `gplate_from_five(&edges)?`.
pub(super) fn gplate_from_five(edges: &[Edge]) -> Result<Face, Failure> {
    let mut plate = prepared_plate(edges)?;
    println!("{{\"geometry_stage\":\"gplate_perform\",\"degree\":3,\"curves\":5}}");
    plate.perform(&ProgressRange::default())?;
    let surface = plate.surface();
    let criterion = contour_criterion(&mut plate)?;
    let approximated = approximation(&surface, &criterion)?;
    let bounds = plate
        .surface()
        .get()
        .ok_or_else(missing_plate_surface)?
        .bounds();
    BRepBuilderAPIMakeFace::from_surface_bounds(&approximated, bounds, precision::confusion())?
        .face()
}

fn prepared_plate(edges: &[Edge]) -> Result<GeomPlateBuildPlateSurface, Failure> {
    let required = edges.get(..5).ok_or_else(|| {
        Failure::with_message(
            "Standard_ConstructionError",
            &format!(
                "gplate boundary count={}; expected at least five aliases",
                edges.len()
            ),
        )
    })?;
    let mut plate = GeomPlateBuildPlateSurface::new(3, 15, 2, 1.0e-5, 1.0e-4, 0.01, 0.1, false)?;
    for edge in required {
        plate.add_curve(&c0_constraint(edge)?);
    }
    Ok(plate)
}

fn c0_constraint(edge: &Edge) -> Result<Handle<GeomPlateCurveConstraint>, Failure> {
    let curve: Handle<dyn Adaptor3dCurve> = Handle::new(BRepAdaptorCurve::with_edge(edge)?).cast();
    let constraint = BRepFillCurveConstraint::with_curve(&curve, 0, 10, 1.0e-4)?;
    Ok(constraint)
}

fn contour_criterion(
    plate: &mut GeomPlateBuildPlateSurface,
) -> Result<GeomPlatePlateG0Criterion, Failure> {
    let mut parameters: Sequence<Xy> = Sequence::default();
    let mut positions: Sequence<Xyz> = Sequence::default();
    plate.disc2d_contour(4, &mut parameters)?;
    plate.disc3d_contour(4, 0, &mut positions)?;
    let maximum = (10.0 * plate.g0_error()).max(0.0001);
    Ok(GeomPlatePlateG0Criterion::new(
        &parameters,
        &positions,
        maximum,
        AdvApp2VarCriterionType::Absolute,
        AdvApp2VarCriterionRepartition::Regular,
    ))
}

fn approximation(
    surface: &Handle<GeomPlateSurface>,
    criterion: &GeomPlatePlateG0Criterion,
) -> Result<Handle<dyn GeomSurface>, Failure> {
    println!(
        "{{\"geometry_stage\":\"gplate_approximation\",\"maximum_patches\":9,\"maximum_degree\":8}}"
    );
    let approximation =
        GeomPlateMakeApprox::with_criterion(surface, criterion, 0.0001, 9, 8, Continuity::C1, 1.1)?;
    Ok(approximation.surface().cast())
}

fn missing_plate_surface() -> Failure {
    let operation =
        "gplate Surface returned null; expected completed plate before bounded-face construction";
    Failure::with_message("Standard_NullObject", operation)
}
