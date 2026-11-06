//! `TopoDSToStepMakeStepFace`: a face to an advanced_face, with its bounds and its pcurves.
use super::pcurves::translate_face_pcurves;
use super::torus_as_revolution::degenerate_torus_as_revolution;
use super::{
    TopoDSToStepMakeFaceError, TopoDSToStepMakeStepWire, TopoDSToStepRoot, TopoDSToStepTool,
    add_warning, step_model_of,
};
use crate::geom_to_step::{GeomToStepMakeSurface, empty_step_name, shared_array1};
use crate::step_data::StepDataFactors;
use crate::step_geom::StepGeomSurface;
use crate::step_shape::{
    StepShapeAdvancedFace, StepShapeFaceBound, StepShapeLoop,
    StepShapeTopologicalRepresentationItem,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_tools;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Face};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::geom::{GeomRectangularTrimmedSurface, GeomSurface};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkxsbase::transfer::TransferFinderProcess;
use martensite_tkxsbase::transfer_brep::{TransferBRep, TransferBRepShapeMapper};

/// Make step face.
#[derive(Clone, Default)]
pub struct TopoDSToStepMakeStepFace {
    /// Root.
    pub root: TopoDSToStepRoot,
    my_result: Handle<StepShapeTopologicalRepresentationItem>,
    my_error: TopoDSToStepMakeFaceError,
}

impl TopoDSToStepMakeStepFace {
    /// `TopoDSToStepMakeStepFace()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `TopoDSToStepMakeStepFace(F, T, FP, theLocalFactors)`.
    pub fn with_face(
        face: &Face,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(face, tool, fp, factors)?;
        Ok(made)
    }
    fn finish(&mut self, result: Handle<StepShapeTopologicalRepresentationItem>) {
        self.my_error = TopoDSToStepMakeFaceError::FaceDone;
        self.my_result = result;
        self.root.done = true;
    }
    fn fail(
        &mut self,
        error: TopoDSToStepMakeFaceError,
        face: &Face,
        fp: &Handle<TransferFinderProcess>,
        message: &str,
    ) -> Result<(), Failure> {
        add_warning(fp, face, message)?;
        self.my_error = error;
        self.root.done = false;
        Ok(())
    }
    /// [BEGIN] Processing non-manifold topology (another approach) (ssv; 10.11.2010): the other
    /// side of a face a former shell wrote.
    fn linking_face(
        face: &Face,
        fp: &Handle<TransferFinderProcess>,
    ) -> Result<Option<Handle<StepShapeTopologicalRepresentationItem>>, Failure> {
        if !step_model_of(fp)?.internal_parameters().write_nonmanifold {
            return Ok(None);
        }
        let mapper = TransferBRep::shape_mapper(fp, face)?;
        let mut known = Handle::<StepShapeAdvancedFace>::default();
        let atype = StepShapeAdvancedFace::static_type();
        if !fp.find_typed_transient_as(&mapper.cast(), atype, &mut known) {
            return Ok(None);
        }
        // Non-manifold topology detected
        let linking = Handle::new(StepShapeAdvancedFace::new());
        linking.init(
            &known.name(),
            &known.bounds()?,
            &known.face_geometry(),
            !known.same_sense(),
        );
        Ok(Some(linking.cast()))
    }
    /// `init(face, tool, FP, local_factors)`: the face is given with its relative orientation
    /// (in the shell). A failure in the geometry is recorded on the process, not raised.
    pub fn init(
        &mut self,
        face: &Face,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let forward_face = topods::face(&face.oriented(Orientation::Forward))?;
        tool.set_current_face(&forward_face)?;
        if let Some(linking) = Self::linking_face(face, fp)? {
            self.finish(linking);
            return Ok(());
        }
        if tool.is_bound(face) {
            self.finish(tool.find(face)?);
            return Ok(());
        }
        if matches!(
            face.orientation(),
            Orientation::Internal | Orientation::External
        ) {
            let text = " Face from Non Manifold Topology";
            return self.fail(TopoDSToStepMakeFaceError::NonManifoldFace, face, fp, text);
        }
        if brep_tools::outer_wire(&forward_face)?.is_null() {
            let text = " Face without wire not mapped";
            return self.fail(TopoDSToStepMakeFaceError::InfiniteFace, face, fp, text);
        }
        if let Err(failure) = self.translate(face, &forward_face, tool, fp, factors) {
            let mapper = Handle::new(TransferBRepShapeMapper::new(face));
            fp.add_fail_text(&mapper.cast(), &String::from_utf8_lossy(failure.what()), "");
            self.my_error = TopoDSToStepMakeFaceError::FaceOther;
            self.root.done = false;
        }
        Ok(())
    }
    fn translate(
        &mut self,
        face: &Face,
        forward_face: &Face,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let stored = BRepTool::surface(forward_face)?;
        if stored.is_null() {
            let text = " Face without geometry not mapped";
            return self.fail(TopoDSToStepMakeFaceError::FaceOther, face, fp, text);
        }
        // CKY  23 SEP 1996: une FACE de Step n a pas droit a RECTANGULAR_TRIMMED...
        // Il faut donc d abord "demonter" la RectangularTrimmedSurface pour
        // passer la Surface de base
        let surface: Handle<dyn GeomSurface> =
            match stored.cast::<GeomRectangularTrimmedSurface>().get() {
                Some(trimmed) => trimmed.basis_surface(),
                None => stored,
            };
        // Surfaces with indirect Axes are already reversed
        tool.set_surface_reversed(false);
        let made = GeomToStepMakeSurface::from_geom(&surface, factors)?.value()?;
        let surface_entity = degenerate_torus_as_revolution(&surface, made, face, factors)?;
        let bounds = self.translate_wires(face, forward_face, tool, fp, factors)?;
        if !tool.faceted() && tool.pcurve_mode() {
            translate_face_pcurves(forward_face, &surface, &surface_entity, tool, factors)?;
        }
        self.translate_face_entity(face, bounds, &surface_entity, tool, fp)
    }
    /// The face_bounds of the wires of the forward face.
    fn translate_wires(
        &mut self,
        face: &Face,
        forward_face: &Face,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
        factors: &StepDataFactors,
    ) -> Result<Vec<Handle<StepShapeFaceBound>>, Failure> {
        let mut bounds = Vec::new();
        let mut maker = TopoDSToStepMakeStepWire::new();
        let mut wires = Explorer::new(forward_face, ShapeEnum::Wire, ShapeEnum::Shape)?;
        while wires.more() {
            let wire = topods::wire(wires.current())?;
            wires.next()?;
            maker.init(&wire, tool, fp, factors)?;
            if !maker.is_done() {
                add_warning(fp, face, " a Wire not mapped")?;
                continue;
            }
            let bound_loop: Handle<StepShapeLoop> = maker.value()?.cast();
            // CAS.CADE face orientation:
            // when a face is reversed in a shell, the orientation of the underlying
            // topology is implicitly reversed. This is not the case in Step.
            // If face orientation is Reversed => the underlying (Step mapped) wire
            // are explicitly reversed
            // Ajoute le 30 Juin pour TEST
            // Il convient de reprendre a la base ce probleme d'orientation
            // et notamment la politique d`exploration du Shape (on explore
            // toujours les sous-shapes d'un shape de maniere FORWARD !
            // la modif (on ajoute: si context faceted ... sinon) est a verifier
            // aupres des autres editeurs de CFAO de la Round Table.
            let expected = if !tool.faceted() && face.orientation() == Orientation::Reversed {
                Orientation::Reversed
            } else {
                Orientation::Forward
            };
            let bound = Handle::new(StepShapeFaceBound::new());
            bound.init(
                &empty_step_name(),
                &bound_loop,
                wire.orientation() == expected,
            );
            bounds.push(bound);
        }
        Ok(bounds)
    }
    fn translate_face_entity(
        &mut self,
        face: &Face,
        bounds: Vec<Handle<StepShapeFaceBound>>,
        surface_entity: &Handle<StepGeomSurface>,
        tool: &mut TopoDSToStepTool,
        fp: &Handle<TransferFinderProcess>,
    ) -> Result<(), Failure> {
        if bounds.is_empty() {
            let text = " No Wires of this Face were mapped";
            return self.fail(TopoDSToStepMakeFaceError::NoWireMapped, face, fp, text);
        }
        let entity = Handle::new(StepShapeAdvancedFace::new());
        // The underlying surface has always a direct axis (see above)
        let same_sense = face.orientation() == Orientation::Forward;
        entity.init(
            &empty_step_name(),
            &shared_array1(bounds),
            surface_entity,
            same_sense,
        );
        tool.bind(face, &entity.cast());
        self.finish(entity.cast());
        Ok(())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    ///
    /// # Errors
    /// a not done failure when the translation gave no result.
    pub fn value(&self) -> Result<Handle<StepShapeTopologicalRepresentationItem>, Failure> {
        self.root.check_done("TopoDSToStepMakeStepFace::Value()")?;
        Ok(self.my_result.clone())
    }
    /// `Error`.
    pub fn error(&self) -> TopoDSToStepMakeFaceError {
        self.my_error
    }
}
