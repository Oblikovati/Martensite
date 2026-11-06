//! `StepToTopoDSTranslateCurveBoundedSurface`: a curve_bounded_surface to a face.
use super::{StepToTopoDSRoot, StepToTopoDSTranslateCompositeCurve};
use crate::step_data::StepDataFactors;
use crate::step_entity::{step_array1_item, step_live};
use crate::step_geom::{
    StepGeomBSplineSurface, StepGeomCompositeCurve, StepGeomCurveBoundedSurface, StepGeomSurface,
};
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::Face;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg3d::geom::{GeomBoundedSurface, GeomSurface};
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// Translate curve bounded surface.
#[derive(Default)]
pub struct StepToTopoDSTranslateCurveBoundedSurface {
    /// Root.
    pub root: StepToTopoDSRoot,
    my_face: Face,
}

impl StepToTopoDSTranslateCurveBoundedSurface {
    /// `StepToTopoDSTranslateCurveBoundedSurface()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTranslateCurveBoundedSurface(CBS, TP, factors)`.
    pub fn with_surface(
        surface: &Handle<StepGeomCurveBoundedSurface>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(surface, tp, factors)?;
        Ok(made)
    }
    /// The surface, forced periodic: abv 30.06.00: trj4_k1_geo-tu.stp #108: do as in TranslateFace.
    fn periodic_surface(
        step_surface: &Handle<StepGeomSurface>,
        surface: Handle<dyn GeomSurface>,
        process: &TransferTransientProcess,
    ) -> Result<Handle<dyn GeomSurface>, Failure> {
        if step_surface.cast::<StepGeomBSplineSurface>().is_null() {
            return Ok(surface);
        }
        let algorithms = ShapeAlgo::algo_container();
        let periodic =
            step_live(&algorithms, "ShapeAlgo::AlgoContainer")?.convert_to_periodic(&surface)?;
        if periodic.is_null() {
            return Ok(surface);
        }
        process.add_warning_text(&step_surface.cast(), "Surface forced to be periodic", "");
        Ok(periodic)
    }
    /// `init(CBS, TP, factors)`: false for a null entity or a surface that cannot be made.
    pub fn init(
        &mut self,
        bounded: &Handle<StepGeomCurveBoundedSurface>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        self.my_face = Face::default();
        let Some(live) = bounded.get() else {
            return Ok(false);
        };
        let process = step_live(tp, "StepToTopoDSTranslateCurveBoundedSurface")?;
        let entity: Handle<StandardTransient> = bounded.cast();
        let step_surface = live.basis_surface();
        let surface = StepToGeom::make_surface(&step_surface, factors)?;
        if surface.is_null() {
            process.add_fail_text(&entity, "Basis surface not translated", "");
            return Ok(false);
        }
        let surface = Self::periodic_surface(&step_surface, surface, process)?;
        let builder = BRepBuilder::default();
        builder.make_face_on_surface(&mut self.my_face, &surface, precision::confusion())?;
        // add natural bound if implicit
        if live.implicit_outer() {
            if surface.cast::<dyn GeomBoundedSurface>().is_some() {
                let maker = BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())?;
                self.my_face = maker.face()?;
            } else {
                process.add_warning_text(
                    &entity,
                    "Cannot make natural bounds on infinite surface",
                    "",
                );
            }
        }
        // translate boundaries
        let boundaries = live.boundaries();
        let count = step_live(&boundaries, "StepToTopoDSTranslateCurveBoundedSurface")?
            .array1()
            .length();
        for rank in 1..=count {
            let boundary = step_array1_item(
                &boundaries,
                rank,
                "StepToTopoDSTranslateCurveBoundedSurface",
            )?;
            let curve: Handle<StepGeomCompositeCurve> = boundary.boundary_curve().cast();
            if curve.is_null() {
                continue;
            }
            let translated = StepToTopoDSTranslateCompositeCurve::with_curve_on_surface(
                &curve,
                tp,
                &step_surface,
                &surface,
                factors,
            )?;
            if !translated.is_done() {
                process.add_warning_text(&entity, "Boundary not translated", "");
                continue;
            }
            builder.add(&mut self.my_face, translated.value())?;
        }
        self.root.done = !self.my_face.is_null();
        Ok(self.root.done)
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    pub fn value(&self) -> &Face {
        &self.my_face
    }
}
