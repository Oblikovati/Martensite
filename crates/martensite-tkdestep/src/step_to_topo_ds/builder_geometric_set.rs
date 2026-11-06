//! `StepToTopoDSBuilder`: the geometric_set overload of `init`, which falls back on the reading
//! actor for what it does not know.
use super::{StepToTopoDSBuilder, StepToTopoDSBuilderError};
use super::{StepToTopoDSTranslateCompositeCurve, StepToTopoDSTranslateCurveBoundedSurface};
use crate::step_control::StepControlActorRead;
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomCartesianPoint, StepGeomCompositeCurve, StepGeomCurve, StepGeomCurveBoundedSurface,
    StepGeomGeometricRepresentationItem, StepGeomRectangularCompositeSurface, StepGeomSurface,
};
use crate::step_shape::StepShapeGeometricSet;
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Compound, Face, Shape};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::geom::{
    GeomBoundedSurface, GeomCurve, GeomPoint, GeomRectangularTrimmedSurface,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex,
};
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::transfer_brep::TransferBRep;

/// `translate_bounded_surf`: :i6: protection against what is not a bounded surface.
fn translate_bounded_surf(
    surface: &Handle<StepGeomSurface>,
    tol_degen: f64,
    factors: &StepDataFactors,
) -> Result<Face, Failure> {
    let made = StepToGeom::make_surface(surface, factors)?;
    if made.is_null() || made.cast::<dyn GeomBoundedSurface>().is_null() {
        return Ok(Face::default());
    }
    let trimmed: Handle<GeomRectangularTrimmedSurface> = made.cast();
    let maker = match trimmed.get() {
        Some(live) => BRepBuilderAPIMakeFace::from_surface_bounds(
            &live.basis_surface(),
            made.bounds(),
            tol_degen,
        )?,
        None => BRepBuilderAPIMakeFace::from_surface(&made, tol_degen)?,
    };
    maker.face()
}

/// The compound of the edges of a composite curve with an infinite segment, its wire otherwise.
fn composite_curve_shape(
    curve: &Handle<StepGeomCompositeCurve>,
    tp: &Handle<TransferTransientProcess>,
    tolerances: (f64, f64),
    factors: &StepDataFactors,
) -> Result<Shape, Failure> {
    let mut translator = StepToTopoDSTranslateCompositeCurve::new();
    translator.root.set_precision(tolerances.0);
    translator.root.set_max_tol(tolerances.1);
    translator.init(curve, tp, factors)?;
    if !translator.is_done() {
        return Ok(Shape::default());
    }
    if !translator.is_infinite_segment() {
        return Ok(translator.value().clone().into_shape());
    }
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut edges = Explorer::new(translator.value(), ShapeEnum::Edge, ShapeEnum::Shape)?;
    while edges.more() {
        builder.add(&mut compound, edges.current())?;
        edges.next()?;
    }
    Ok(compound.into_shape())
}

/// A curve other than a composite one: the edge on its geometry. An exception from the geometry
/// leaves no curve (the trace is informative only).
fn free_curve_shape(
    curve: &Handle<StepGeomCurve>,
    factors: &StepDataFactors,
) -> Result<Shape, Failure> {
    let made: Handle<dyn GeomCurve> = StepToGeom::make_curve(curve, factors).unwrap_or_default();
    let Some(live) = made.get() else {
        return Ok(Shape::default());
    };
    let maker = BRepBuilderAPIMakeEdge::from_curve_params(
        &made,
        live.first_parameter(),
        live.last_parameter(),
    )?;
    if !maker.is_done() {
        return Ok(Shape::default());
    }
    Ok(maker.edge()?.into_shape())
}

/// The faces of the patches of a rectangular composite surface.
fn rectangular_composite_shape(
    surface: &Handle<StepGeomRectangularCompositeSurface>,
    precision: f64,
    factors: &StepDataFactors,
) -> Result<Shape, Failure> {
    let live = step_live(surface, "StepToTopoDSBuilder")?;
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    for i in 1..=live.nb_segments_i() {
        for j in 1..=live.nb_segments_j() {
            let patch = live.segments_value(i, j)?;
            let parent: Handle<StepGeomSurface> = step_live(&patch, "StepToTopoDSBuilder")?
                .parent_surface()
                .cast();
            let face = translate_bounded_surf(&parent, precision, factors)?;
            if !face.is_null() {
                builder.add(&mut compound, &face)?;
            }
        }
    }
    Ok(compound.into_shape())
}

impl StepToTopoDSBuilder {
    /// The shape of a surface element of the set.
    fn surface_shape(
        &self,
        surface: &Handle<StepGeomSurface>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<Shape, Failure> {
        let (precision, max_tol) = (self.root.precision(), self.root.max_tol());
        let bounded: Handle<StepGeomCurveBoundedSurface> = surface.cast();
        if bounded.is_some() {
            let mut translator = StepToTopoDSTranslateCurveBoundedSurface::new();
            translator.root.set_precision(precision);
            translator.root.set_max_tol(max_tol);
            translator.init(&bounded, tp, factors)?;
            if translator.is_done() {
                return Ok(translator.value().clone().into_shape());
            }
            return Ok(Shape::default());
        }
        let composite: Handle<StepGeomRectangularCompositeSurface> = surface.cast();
        if composite.is_some() {
            return rectangular_composite_shape(&composite, precision, factors);
        }
        Ok(translate_bounded_surf(surface, precision, factors)?.into_shape())
    }
    /// The shape of one element of the set, null when none could be made.
    fn geometric_set_element_shape(
        &self,
        ent: &Handle<StandardTransient>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        actor: Option<&StepControlActorRead>,
        is_manifold: bool,
        range: &ProgressRange<'_>,
    ) -> Result<Shape, Failure> {
        let curve: Handle<StepGeomCurve> = ent.cast();
        if curve.is_some() {
            let composite: Handle<StepGeomCompositeCurve> = curve.cast();
            if composite.is_some() {
                let tolerances = (self.root.precision(), self.root.max_tol());
                return composite_curve_shape(&composite, tp, tolerances, factors);
            }
            return free_curve_shape(&curve, factors);
        }
        let point: Handle<StepGeomCartesianPoint> = ent.cast();
        if point.is_some() {
            let made = StepToGeom::make_cartesian_point(&point, factors)?;
            let Some(live) = made.get() else {
                return Ok(Shape::default());
            };
            let maker = BRepBuilderAPIMakeVertex::new(live.pnt())?;
            if maker.is_done() {
                return Ok(maker.vertex()?.into_shape());
            }
            return Ok(Shape::default());
        }
        let surface: Handle<StepGeomSurface> = ent.cast();
        if surface.is_some() {
            return self.surface_shape(&surface, tp, factors);
        }
        if ent.cast::<StepGeomGeometricRepresentationItem>().is_some() {
            let Some(actor) = actor else {
                return Ok(Shape::default());
            };
            let binder = actor.transfer_shape(ent, tp, factors, is_manifold, false, range)?;
            if binder.is_some() {
                return Ok(TransferBRep::shape_result(&binder));
            }
            return Ok(Shape::default());
        }
        let text = " Entity is not a Curve, Point, Surface or GeometricRepresentationItem";
        step_live(tp, "StepToTopoDSBuilder")?.add_warning_text(ent, text, "");
        Ok(Shape::default())
    }
    /// `init(GCS, TP, local_factors, RA, isManifold, progress)`.
    pub fn init_geometric_set(
        &mut self,
        set: &Handle<StepShapeGeometricSet>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
        actor: Option<&StepControlActorRead>,
        is_manifold: bool,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let live = step_live(set, "StepToTopoDSBuilder")?;
        let live_tp = step_live(tp, "StepToTopoDSBuilder")?;
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let count = live.nb_elements();
        let scope = ProgressScope::unnamed(progress, f64::from(count), false);
        let mut rank = 1;
        while rank <= count && scope.more() {
            let range = scope.next(1.0);
            let ent = live.elements_value(rank)?.value();
            let index = rank;
            rank += 1;
            let known = TransferBRep::shape_result_of(tp, &ent)?;
            if !known.is_null() {
                // already translated
                builder.add(&mut compound, &known)?;
                continue;
            }
            if ent.is_null() {
                let text = format!("Entity {index} is a Null entity");
                live_tp.add_warning_text(&set.cast(), &text, "");
                continue;
            }
            let made =
                self.geometric_set_element_shape(&ent, tp, factors, actor, is_manifold, &range)?;
            if made.is_null() {
                live_tp.add_warning_text(&ent, " Entity not mapped to TopoDS", "");
            } else {
                builder.add(&mut compound, &made)?;
                TransferBRep::set_shape_result(tp, &ent, &made)?;
            }
        }
        self.my_result = compound.into_shape();
        self.my_error = StepToTopoDSBuilderError::Done;
        self.root.done = true;
        Ok(())
    }
}
