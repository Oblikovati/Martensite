//! The tessellated overloads: `StepToTopoDSTranslateFace`, `StepToTopoDSTranslateShell` and
//! `StepToTopoDSTranslateSolid`.
use super::tessellated_mesh::create_poly_triangulation;
use super::{
    StepToTopoDSNMTool, StepToTopoDSRoot, StepToTopoDSTool, StepToTopoDSTranslateFace,
    StepToTopoDSTranslateShell, StepToTopoDSTranslateSolidError,
};
use crate::step_data::StepDataFactors;
use crate::step_entity::step_live;
use crate::step_visual::{
    StepVisualComplexTriangulatedFace, StepVisualComplexTriangulatedSurfaceSet,
    StepVisualTessellatedFace, StepVisualTessellatedShell, StepVisualTessellatedSolid,
    StepVisualTessellatedStructuredItem, StepVisualTessellatedSurfaceSet,
    StepVisualTriangulatedFace, StepVisualTriangulatedSurfaceSet,
};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{self, Face, Shape, Shell, Solid};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::transfer_brep::TransferBRepShapeBinder;

/// `theTP->Find(link)` as a shape binder.
fn shape_binder_of(
    process: &TransferTransientProcess,
    link: &Handle<StandardTransient>,
) -> Handle<TransferBRepShapeBinder> {
    process.find(link).cast()
}

impl StepToTopoDSTranslateFace {
    /// Puts the mesh on the face and records the result.
    fn finish_mesh(
        &mut self,
        face: &Face,
        mesh: &Handle<martensite_tkmath::poly::Triangulation>,
        entity: &Handle<StandardTransient>,
        nm_tool: &mut StepToTopoDSNMTool,
    ) -> Result<(), Failure> {
        BRepBuilder::default().update_face_triangulation(face, mesh, true)?;
        if nm_tool.is_active() {
            nm_tool.bind(&entity.cast(), face);
        }
        self.finish_tessellated(face.clone().into_shape());
        Ok(())
    }
    /// `StepToTopoDSTranslateFace(theTF, theTool, theNMTool, theReadTessellatedWhenNoBRepOnly,
    /// theHasGeom, factors)`.
    pub fn with_tessellated_face(
        tessellated: &Handle<StepVisualTessellatedFace>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        only_no_brep: bool,
        has_geom: &mut bool,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut translator = Self::new();
        translator.init_tessellated_face(
            tessellated,
            tool,
            nm_tool,
            only_no_brep,
            has_geom,
            factors,
        )?;
        Ok(translator)
    }
    /// `StepToTopoDSTranslateFace(theTSS, theTool, theNMTool, factors)`.
    pub fn with_tessellated_surface_set(
        set: &Handle<StepVisualTessellatedSurfaceSet>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut translator = Self::new();
        translator.init_tessellated_surface_set(set, tool, nm_tool, factors)?;
        Ok(translator)
    }
    /// `init(tf, tool, nm_tool, read_tessellated_when_no_b_rep_only, has_geom, factors)`.
    pub fn init_tessellated_face(
        &mut self,
        tessellated: &Handle<StepVisualTessellatedFace>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        only_no_brep: bool,
        has_geom: &mut bool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let Some(live) = tessellated.get() else {
            return Ok(());
        };
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateFace")?;
        let entity: Handle<StandardTransient> = tessellated.cast();
        let mut face = Face::default();
        if live.has_geometric_link() {
            let link = live.geometric_link().face();
            let binder = shape_binder_of(process, &link.cast());
            if let Some(live_binder) = binder.get() {
                face = live_binder.face()?;
                *has_geom = true;
            }
            if !face.is_null() && BRepTool::surface(&face)?.is_some() && only_no_brep {
                return Ok(());
            }
        }
        if face.is_null() {
            BRepBuilder::default().make_face(&mut face);
            *has_geom = false;
        }
        if tessellated.cast::<StepVisualTriangulatedFace>().is_null()
            && tessellated
                .cast::<StepVisualComplexTriangulatedFace>()
                .is_null()
        {
            let text = " Triangulated or ComplexTriangulated entity is supported only.";
            process.add_warning_text(&entity, text, "");
            return Ok(());
        }
        let mesh = create_poly_triangulation(&tessellated.cast(), factors)?;
        if mesh.is_null() {
            process.add_warning_text(
                &entity,
                " Poly triangulation is not set to TopoDS face.",
                "",
            );
            return Ok(());
        }
        self.finish_mesh(&face, &mesh, &entity, nm_tool)?;
        let binder = Handle::new(TransferBRepShapeBinder::with_result(&face));
        process.bind(&entity, &binder.cast())
    }
    /// `init(tss, tool, nm_tool, factors)`.
    pub fn init_tessellated_surface_set(
        &mut self,
        set: &Handle<StepVisualTessellatedSurfaceSet>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        if set.is_null() {
            return Ok(());
        }
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateFace")?;
        let entity: Handle<StandardTransient> = set.cast();
        let mut face = Face::default();
        BRepBuilder::default().make_face(&mut face);
        if set.cast::<StepVisualTriangulatedSurfaceSet>().is_null()
            && set
                .cast::<StepVisualComplexTriangulatedSurfaceSet>()
                .is_null()
        {
            let text = " Triangulated or ComplexTriangulated entity is supported only.";
            process.add_warning_text(&entity, text, "");
            return Ok(());
        }
        let mesh = create_poly_triangulation(&set.cast(), factors)?;
        if mesh.is_null() {
            process.add_warning_text(
                &entity,
                " Poly triangulation is not set to TopoDS face.",
                "",
            );
            return Ok(());
        }
        self.finish_mesh(&face, &mesh, &entity, nm_tool)
    }
}

/// The faces of tessellated items: added to `shell` when given, `theHasGeom &= ...`.
#[allow(
    clippy::too_many_arguments,
    reason = "the two native loops read each of these"
)]
fn translate_items(
    items: &mut dyn FnMut(i32) -> Result<Handle<StepVisualTessellatedStructuredItem>, Failure>,
    count: i32,
    translator: &mut StepToTopoDSTranslateFace,
    tool: &mut StepToTopoDSTool,
    nm_tool: &mut StepToTopoDSNMTool,
    only_no_brep: bool,
    has_geom: &mut bool,
    factors: &StepDataFactors,
    mut shell: Option<&mut Shell>,
    scope: &ProgressScope<'_>,
    texts: (&str, &str),
) -> Result<(), Failure> {
    let process_handle = tool.transient_process();
    let process = step_live(&process_handle, "StepToTopoDS tessellated translator")?;
    let builder = BRepBuilder::default();
    let mut rank = 1;
    while rank <= count && scope.more() {
        let item = items(rank)?;
        rank += 1;
        let _ = scope.next(1.0);
        let entity: Handle<StandardTransient> = item.cast();
        let face: Handle<StepVisualTessellatedFace> = item.cast();
        if face.is_null() {
            process.add_warning_text(&entity, texts.1, "");
            continue;
        }
        let mut face_geom = false;
        translator.init_tessellated_face(
            &face,
            tool,
            nm_tool,
            only_no_brep,
            &mut face_geom,
            factors,
        )?;
        if !translator.is_done() {
            process.add_warning_text(&entity, texts.0, "");
            continue;
        }
        if let Some(target) = shell.as_deref_mut() {
            builder.add(target, translator.value()?)?;
        }
        *has_geom &= face_geom;
    }
    Ok(())
}

impl StepToTopoDSTranslateShell {
    /// `init(t_sh, tool, nm_tool, read_tessellated_when_no_b_rep_only, has_geom, factors, progress)`.
    #[allow(clippy::too_many_arguments, reason = "the member takes each of these")]
    pub fn init_tessellated(
        &mut self,
        shell_entity: &Handle<StepVisualTessellatedShell>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        only_no_brep: bool,
        has_geom: &mut bool,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let Some(live) = shell_entity.get() else {
            return Ok(());
        };
        let count = live.nb_items();
        let has_link = live.has_topological_link();
        let steps = f64::from(if has_link { count + 1 } else { count });
        let scope = ProgressScope::with_literal(progress, "Face", steps, false);
        let process_handle = tool.transient_process();
        let process = step_live(&process_handle, "StepToTopoDSTranslateShell")?;
        let mut shell = Shell::default();
        if has_link {
            let binder = shape_binder_of(process, &live.topological_link().cast());
            if let Some(live_binder) = binder.get() {
                shell = live_binder.shell()?;
                *has_geom = true;
            }
        }
        let is_new = shell.is_null();
        if is_new {
            BRepBuilder::default().make_shell(&mut shell);
            *has_geom = false;
        }
        let mut translator = StepToTopoDSTranslateFace::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        let texts = (
            " Triangulated face if not mapped to TopoDS",
            " Face is not of TriangulatedFace Type; not mapped to TopoDS",
        );
        let mut items = |rank| live.items_value(rank);
        translate_items(
            &mut items,
            count,
            &mut translator,
            tool,
            nm_tool,
            only_no_brep,
            has_geom,
            factors,
            is_new.then_some(&mut shell),
            &scope,
            texts,
        )?;
        shell.set_closed(BRepTool::is_closed(&shell)?)?;
        self.finish(shell.into_shape());
        Ok(())
    }
}

/// Translate solid.
#[derive(Default)]
pub struct StepToTopoDSTranslateSolid {
    /// Root.
    pub root: StepToTopoDSRoot,
    my_result: Shape,
    my_error: StepToTopoDSTranslateSolidError,
}

impl StepToTopoDSTranslateSolid {
    /// `StepToTopoDSTranslateSolid()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `init(t_so, tp, tool, nm_tool, read_tessellated_when_no_b_rep_only, has_geom, factors, progress)`.
    #[allow(clippy::too_many_arguments, reason = "the member takes each of these")]
    pub fn init(
        &mut self,
        solid_entity: &Handle<StepVisualTessellatedSolid>,
        tp: &Handle<TransferTransientProcess>,
        tool: &mut StepToTopoDSTool,
        nm_tool: &mut StepToTopoDSNMTool,
        only_no_brep: bool,
        has_geom: &mut bool,
        factors: &StepDataFactors,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let Some(live) = solid_entity.get() else {
            return Ok(());
        };
        let count = live.nb_items();
        let scope = ProgressScope::with_literal(progress, "Face", f64::from(count), false);
        let process = step_live(tp, "StepToTopoDSTranslateSolid")?;
        let builder = BRepBuilder::default();
        let mut solid = Solid::default();
        if live.has_geometric_link() && process.is_bound(&live.geometric_link().cast()) {
            let binder = shape_binder_of(process, &live.geometric_link().cast());
            if let Some(live_binder) = binder.get() {
                solid = live_binder.solid()?;
            }
        }
        let is_new = solid.is_null();
        let mut shell = Shell::default();
        if is_new {
            builder.make_shell(&mut shell);
            builder.make_solid(&mut solid);
            *has_geom = false;
        }
        let mut translator = StepToTopoDSTranslateFace::new();
        translator.root.set_precision(self.root.precision());
        translator.root.set_max_tol(self.root.max_tol());
        let texts = (
            " Tessellated face if not mapped to TopoDS",
            " Face is not of TessellatedFace Type; not mapped to TopoDS",
        );
        let mut items = |rank| live.items_value(rank);
        translate_items(
            &mut items,
            count,
            &mut translator,
            tool,
            nm_tool,
            only_no_brep,
            has_geom,
            factors,
            is_new.then_some(&mut shell),
            &scope,
            texts,
        )?;
        if is_new {
            builder.add(&mut solid, &shell)?;
        }
        self.my_result = solid.into_shape();
        self.my_error = StepToTopoDSTranslateSolidError::Done;
        self.root.done = true;
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
    pub fn value(&self) -> Result<&Shape, Failure> {
        self.root
            .check_done("StepToTopoDSTranslateSolid::Value()")?;
        Ok(&self.my_result)
    }
    /// `Error`.
    pub fn error(&self) -> StepToTopoDSTranslateSolidError {
        self.my_error
    }
}

/// Used by the builder for the casts of its results.
pub(super) fn checked_face(shape: &Shape) -> Result<Shape, Failure> {
    Ok(topods::face(shape)?.into_shape())
}
