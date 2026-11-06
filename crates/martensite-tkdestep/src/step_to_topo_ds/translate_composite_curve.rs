//! `StepToTopoDSTranslateCompositeCurve`: a composite_curve to a wire, with pcurves when it lies
//! on a surface.
use super::{StepToTopoDSRoot, StepToTopoDSTranslateEdge};
use crate::de_step::DeStepReadModeSurfaceCurve;
use crate::step_data::{StepDataFactors, StepDataStepModel};
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomCompositeCurve, StepGeomCompositeCurveSegment, StepGeomCurve, StepGeomPcurve,
    StepGeomSurface, StepGeomSurfaceCurve, StepGeomTransitionCode,
};
use crate::step_to_geom::StepToGeom;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Edge, ShapeIterator, Wire};
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::toploc::Location;
use martensite_tkshhealing::shape_extend::{ShapeExtendStatus, ShapeExtendWireData};
use martensite_tkshhealing::shape_fix::{FixRoot, ShapeFixWire};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use martensite_tkxsbase::transfer::TransferTransientProcess;

/// Translate composite curve.
#[derive(Default)]
pub struct StepToTopoDSTranslateCompositeCurve {
    /// Root.
    pub root: StepToTopoDSRoot,
    my_wire: Wire,
    my_infinite_segment: bool,
}

/// True when a parameter range reaches infinity.
fn is_infinite_range(first: f64, last: f64) -> bool {
    precision::is_negative_infinite(first) || precision::is_positive_infinite(last)
}

/// The pcurve of a surface curve on the surface: the first in the sense of the segment, else the
/// last (a reversed segment keeps looking).
fn surface_curve_pcurve(
    curve: &StepGeomSurfaceCurve,
    surface: &Handle<StepGeomSurface>,
    same_sense: bool,
) -> Result<Handle<StepGeomPcurve>, Failure> {
    let mut found = Handle::default();
    for rank in 1..=curve.nb_associated_geometry()? {
        let pcurve = curve.associated_geometry_value(rank)?.pcurve();
        if !pcurve
            .get()
            .is_some_and(|live| live.basis_surface() == *surface)
        {
            continue;
        }
        found = pcurve;
        if same_sense {
            break;
        }
    }
    Ok(found)
}

impl StepToTopoDSTranslateCompositeCurve {
    /// `StepToTopoDSTranslateCompositeCurve()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepToTopoDSTranslateCompositeCurve(CC, TP, factors)`.
    pub fn with_curve(
        curve: &Handle<StepGeomCompositeCurve>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init(curve, tp, factors)?;
        Ok(made)
    }
    /// `StepToTopoDSTranslateCompositeCurve(CC, TP, S, Surf, factors)`.
    pub fn with_curve_on_surface(
        curve: &Handle<StepGeomCompositeCurve>,
        tp: &Handle<TransferTransientProcess>,
        step_surface: &Handle<StepGeomSurface>,
        surface: &Handle<dyn GeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<Self, Failure> {
        let mut made = Self::new();
        made.init_on_surface(curve, tp, step_surface, surface, factors)?;
        Ok(made)
    }
    /// `init(CC, TP, factors)`.
    pub fn init(
        &mut self,
        curve: &Handle<StepGeomCompositeCurve>,
        tp: &Handle<TransferTransientProcess>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        self.init_on_surface(curve, tp, &Handle::default(), &Handle::default(), factors)
    }
    /// The surface mode: a surface is given and the file does not forbid its pcurves.
    fn surface_mode(
        process: &TransferTransientProcess,
        step_surface: &Handle<StepGeomSurface>,
        surface: &Handle<dyn GeomSurface>,
    ) -> Result<bool, Failure> {
        if step_surface.is_null() || surface.is_null() {
            return Ok(false);
        }
        let model = process.model().cast::<StepDataStepModel>();
        let mode = step_live(&model, "StepToTopoDSTranslateCompositeCurve")?
            .internal_parameters()
            .read_surface_curve_mode;
        Ok(mode != DeStepReadModeSurfaceCurve::USE_3D_FORCED)
    }
    /// `init(CC, TP, S, Surf, factors)`: false for a null curve, a null segment or no edge.
    pub fn init_on_surface(
        &mut self,
        curve: &Handle<StepGeomCompositeCurve>,
        tp: &Handle<TransferTransientProcess>,
        step_surface: &Handle<StepGeomSurface>,
        surface: &Handle<dyn GeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<bool, Failure> {
        self.my_wire = Wire::default();
        self.my_infinite_segment = false;
        let Some(live) = curve.get() else {
            return Ok(false);
        };
        let process = step_live(tp, "StepToTopoDSTranslateCompositeCurve")?;
        let entity: Handle<StandardTransient> = curve.cast();
        let surf_mode = Self::surface_mode(process, step_surface, surface)?;
        let wire_data = Handle::new(ShapeExtendWireData::new());
        let edges = step_live(&wire_data, "StepToTopoDSTranslateCompositeCurve")?;
        let mut is_closed = false;
        for rank in 1..=live.nb_segments() {
            let segment = live.segments_value(rank)?;
            let Some(live_segment) = segment.get() else {
                process.add_fail_text(&entity, "Null segment", "");
                return Ok(false);
            };
            let parent = live_segment.parent_curve();
            if parent.is_null() {
                process.add_fail_text(&entity, "Segment has null parent curve", "");
                return Ok(false);
            }
            is_closed = live_segment.transition() != StepGeomTransitionCode::TcDiscontinuous;
            // if segment is itself a composite_curve, translate recursively
            let nested = parent.cast::<StepGeomCompositeCurve>();
            if nested.is_some() {
                self.add_nested_curve(
                    &nested,
                    curve,
                    &segment,
                    edges,
                    tp,
                    step_surface,
                    surface,
                    factors,
                )?;
                continue;
            }
            self.add_ordinary_segment(
                &parent,
                curve,
                live_segment,
                surf_mode,
                edges,
                process,
                step_surface,
                surface,
                factors,
            )?;
        }
        if edges.nb_edges() <= 0 {
            process.add_fail_text(&entity, "Translation gives no result", "");
            return Ok(false);
        }
        self.connect_wire(&wire_data, is_closed, &entity, process)?;
        self.my_wire = edges.wire()?;
        self.root.done = edges.nb_edges() > 0;
        Ok(true)
    }
    /// The edges of a nested composite curve, in the sense of the segment.
    #[allow(clippy::too_many_arguments, reason = "the locals of the segment loop")]
    fn add_nested_curve(
        &mut self,
        nested: &Handle<StepGeomCompositeCurve>,
        outer: &Handle<StepGeomCompositeCurve>,
        segment: &Handle<StepGeomCompositeCurveSegment>,
        edges: &ShapeExtendWireData,
        tp: &Handle<TransferTransientProcess>,
        step_surface: &Handle<StepGeomSurface>,
        surface: &Handle<dyn GeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        if nested == outer {
            // cyclic reference protection
            let process = step_live(tp, "StepToTopoDSTranslateCompositeCurve")?;
            process.add_fail_text(&segment.cast(), "Cyclic reference; segment dropped", "");
            return Ok(());
        }
        if !self.init_on_surface(nested, tp, step_surface, surface, factors)?
            || self.my_wire.is_null()
        {
            return Ok(());
        }
        let same_sense = step_live(segment, "StepToTopoDSTranslateCompositeCurve")?.same_sense();
        let first_new = edges.nb_edges() + 1;
        let mut iterator = ShapeIterator::new(&self.my_wire, true, true);
        while iterator.more() {
            let mut edge = topods::edge(iterator.value()?)?;
            if same_sense {
                edges.add_edge(&edge, 0)?;
            } else {
                edge.reverse();
                edges.add_edge(
                    &edge,
                    if first_new > edges.nb_edges() {
                        0
                    } else {
                        first_new
                    },
                )?;
            }
            iterator.next();
        }
        self.my_wire = Wire::default();
        Ok(())
    }
    /// The 3D edge of a curve; a failure leaves the edge null, as the `catch` does.
    fn translate_3d(
        &mut self,
        curve: &Handle<StepGeomCurve>,
        entity: &Handle<StandardTransient>,
        process: &TransferTransientProcess,
        factors: &StepDataFactors,
    ) -> Result<Edge, Failure> {
        let made = StepToGeom::make_curve(curve, factors)?;
        let Some(live) = made.get() else {
            return Ok(Edge::default());
        };
        let (first, last) = (live.first_parameter(), live.last_parameter());
        let maker = BRepBuilderAPIMakeEdge::from_curve_params(&made, first, last)?;
        if !maker.is_done() {
            return Ok(Edge::default());
        }
        if is_infinite_range(first, last) {
            self.my_infinite_segment = true;
            process.add_warning_text(entity, "Segment with infinite parameters", "");
        }
        maker.edge()
    }
    /// The pcurve of a segment on the edge, which it makes when the edge has no 3D curve.
    fn translate_pcurve(
        &mut self,
        pcurve: &Handle<StepGeomPcurve>,
        mut edge: Edge,
        surface: &Handle<dyn GeomSurface>,
        entity: &Handle<StandardTransient>,
        process: &TransferTransientProcess,
        factors: &StepDataFactors,
    ) -> Result<Edge, Failure> {
        let made: Handle<dyn Geom2dCurve> =
            StepToTopoDSTranslateEdge::new().make_pcurve(pcurve, surface, factors)?;
        let Some(live) = made.get() else {
            return Ok(edge);
        };
        let (first, last) = (live.first_parameter(), live.last_parameter());
        if edge.is_null() {
            let maker = BRepBuilderAPIMakeEdge::from_pcurve_params(&made, surface, first, last)?;
            if maker.is_done() {
                if is_infinite_range(first, last) {
                    self.my_infinite_segment = true;
                    process.add_warning_text(entity, "Segment with infinite parameters", "");
                }
                edge = maker.edge()?;
            }
            return Ok(edge);
        }
        let builder = BRepBuilder::default();
        let location = Location::default();
        builder.update_edge_pcurve(&edge, &made, surface, &location, 0.0, None)?;
        builder.range_on_surface(&edge, surface, &location, first, last)?;
        builder.same_range(&edge, false)?;
        builder.same_parameter(&edge, false)?;
        Ok(edge)
    }
    /// An ordinary segment: its 3D curve and its pcurve, whichever are there.
    #[allow(clippy::too_many_arguments, reason = "the locals of the segment loop")]
    fn add_ordinary_segment(
        &mut self,
        parent: &Handle<StepGeomCurve>,
        outer: &Handle<StepGeomCompositeCurve>,
        segment: &StepGeomCompositeCurveSegment,
        surf_mode: bool,
        edges: &ShapeExtendWireData,
        process: &TransferTransientProcess,
        step_surface: &Handle<StepGeomSurface>,
        surface: &Handle<dyn GeomSurface>,
        factors: &StepDataFactors,
    ) -> Result<(), Failure> {
        let entity: Handle<StandardTransient> = outer.cast();
        // detect pcurve and 3d curve
        let mut curve = parent.clone();
        let mut pcurve = parent.cast::<StepGeomPcurve>();
        if let Some(live) = pcurve.get() {
            if !surf_mode || live.basis_surface() != *step_surface {
                pcurve = Handle::default();
            }
            curve = Handle::default();
        } else if let Some(surface_curve) = parent.cast::<StepGeomSurfaceCurve>().get() {
            curve = surface_curve.curve3d();
            if surf_mode {
                pcurve = surface_curve_pcurve(surface_curve, step_surface, segment.same_sense())?;
            }
        }
        let mut edge = Edge::default();
        if curve.is_some() {
            // A failure leaves the edge null: the code catches and goes on.
            edge = self
                .translate_3d(&curve, &entity, process, factors)
                .unwrap_or_default();
        }
        if pcurve.is_some() {
            let kept = edge.clone();
            edge = self
                .translate_pcurve(&pcurve, edge, surface, &entity, process, factors)
                .unwrap_or(kept);
        }
        if edge.is_null() {
            process.add_fail_text(&curve.cast(), "Curve can not be translated", "");
            return Ok(());
        }
        if !segment.same_sense() {
            edge.reverse();
        }
        edges.add_edge(&edge, 0)
    }
    /// Connects the wire; all other fixes are left for the caller.
    fn connect_wire(
        &self,
        wire_data: &Handle<ShapeExtendWireData>,
        is_closed: bool,
        entity: &Handle<StandardTransient>,
        process: &TransferTransientProcess,
    ) -> Result<(), Failure> {
        let preci = self.root.precision();
        let fixer = ShapeFixWire::new();
        fixer.load_wire_data(wire_data)?;
        fixer.set_precision(preci);
        fixer.set_closed_mode(is_closed);
        fixer.fix_reorder(false)?; //:o3 abv 17 Feb 99: r0301_db.stp #57082
        if fixer.status_reorder(ShapeExtendStatus::Done) {
            process.add_warning_text(entity, "Segments were disordered; fixed", "");
        }
        fixer.fix_connected_all(preci)?;
        if fixer.status_connected(ShapeExtendStatus::Fail) {
            process.add_warning_text(entity, "Segments are not connected", "");
        }
        Ok(())
    }
    /// `is_done`.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
    /// `Value`.
    pub fn value(&self) -> &Wire {
        &self.my_wire
    }
    /// `is_infinite_segment`.
    pub fn is_infinite_segment(&self) -> bool {
        self.my_infinite_segment
    }
}
