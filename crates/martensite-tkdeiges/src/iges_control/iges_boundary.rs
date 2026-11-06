//! `IgesControlIgesBoundary`: the boundary that checks its representations against each other, and
//! keeps the one that holds together when they disagree.
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_geom::IgesGeomBSplineCurve;
use crate::iges_to_brep::{
    BoundaryCurves, BoundaryFlags, BoundaryPlace, BoundarySequences, IgesToBRep,
    IgesToBRepBoundaryOps, IgesToBRepCurveAndSurface, IgesToBRepIgesBoundary, IgesToBRepTopoCurve,
    local_2d_curve, local_3d_curve, reverse_curves_2d, reverse_curves_3d, transfer_2d_only,
    transfer_3d_only, wire_fix_on_face,
};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::Wire;
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::Msg;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_algo::ShapeAlgo;
use martensite_tkshhealing::shape_analysis::{ShapeAnalysisShapeTolerance, ShapeAnalysisWire};
use martensite_tkshhealing::shape_build::ShapeBuildEdge;
use martensite_tkshhealing::shape_extend::{ShapeExtendStatus, ShapeExtendWireData};
use martensite_tkshhealing::shape_fix::{ShapeFixEdge, ShapeFixShapeTolerance, ShapeFixWire};
use martensite_tkxsbase::interface::InterfaceStatic;
use std::ops::Deref;

const WHERE: &str = "IgesControlIgesBoundary";

/// The boundary with the checks of its representations.
///
/// ```
/// use martensite_tkdeiges::iges_control::IgesControlIgesBoundary;
/// let boundary = IgesControlIgesBoundary::new();
/// assert!(boundary.wire_data().is_null());
/// ```
#[derive(Default)]
pub struct IgesControlIgesBoundary {
    base: IgesToBRepIgesBoundary,
}
impl_transient!(
    IgesControlIgesBoundary,
    "IgesControlIgesBoundary",
    IgesToBRepIgesBoundary,
    base,
    interfaces: [dyn IgesToBRepBoundaryOps]
);

impl Deref for IgesControlIgesBoundary {
    type Target = IgesToBRepIgesBoundary;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

/// What a transfer works with.
struct BoundaryJob {
    tool: IgesToBRepTopoCurve,
    place: BoundaryPlace,
    precision: f64,
    max_tol: f64,
    preferred_3d: bool,
    preferred_2d: bool,
    local: Handle<ShapeExtendWireData>,
    local_3d: Handle<ShapeExtendWireData>,
    local_2d: Handle<ShapeExtendWireData>,
    analysis: Handle<ShapeAnalysisWire>,
    analysis_3d: Handle<ShapeAnalysisWire>,
    analysis_2d: Handle<ShapeAnalysisWire>,
}

fn analysed_list(
    list: &Handle<ShapeExtendWireData>,
    precision: f64,
) -> Result<Handle<ShapeAnalysisWire>, Failure> {
    let analysis = Handle::new(ShapeAnalysisWire::new());
    let live = iges_live(&analysis, WHERE)?;
    live.load_wire_data(list);
    live.set_precision(precision);
    Ok(analysis)
}

/// Connects the next wire to the wire an analysis has loaded.
fn connect_next(
    analysis: &Handle<ShapeAnalysisWire>,
    next: &Handle<ShapeExtendWireData>,
    max_tol: f64,
) -> Result<bool, Failure> {
    let container = ShapeAlgo::algo_container();
    Ok(iges_live(&container, WHERE)?
        .connect_next_wire(analysis, next, max_tol)?
        .connected)
}

/// Loads a wire in the analysis, with the next wire connected inside when asked, and connects them.
fn connect_lists(
    analysis: &Handle<ShapeAnalysisWire>,
    list: &Handle<ShapeExtendWireData>,
    next: &Handle<ShapeExtendWireData>,
    connect_next_inside: bool,
    max_tol: f64,
) -> Result<bool, Failure> {
    iges_live(analysis, WHERE)?.load_wire_data(list);
    if connect_next_inside {
        let fix = ShapeFixWire::new();
        fix.load_wire_data(next)?;
        // The internal connections are enough.
        fix.set_closed_mode(false);
        fix.fix_connected_all(-1.0)?;
    }
    connect_next(analysis, next, max_tol)
}

impl BoundaryJob {
    /// Copies the pcurves onto the edges with 3D curves. With a deviation over the maximum, the
    /// representations are inconsistent: gives the 2D list when the 3D one is not preferred.
    fn copied_pcurves(
        &self,
        piece_3d: &Handle<ShapeExtendWireData>,
        piece_2d: &Handle<ShapeExtendWireData>,
    ) -> Result<Handle<ShapeExtendWireData>, Failure> {
        let (list_3d, list_2d) = (iges_live(piece_3d, WHERE)?, iges_live(piece_2d, WHERE)?);
        let face = &self.place.face;
        let fix = ShapeFixEdge::new();
        for rank in 1..=list_3d.nb_edges() {
            let edge_3d = list_3d.edge(rank)?;
            if !IgesToBRep::transfer_pcurve(&list_2d.edge(rank)?, &edge_3d, face)? {
                continue;
            }
            let _ = fix.fix_reversed_2d(&edge_3d, face)?;
            // The use of same parameter is a static parameter.
            if InterfaceStatic::i_val("read.stdsameparameter.mode") != 0 {
                let (first, last) = BRepTool::range(&edge_3d)?;
                // A tolerance as the same parameter approximation chooses it.
                let tolerance = self.precision.min((last - first).abs() / 1000.0);
                BRepBuilder::default().same_parameter(&edge_3d, false)?;
                let _ = fix.fix_same_parameter(&edge_3d, tolerance)?;
            } else {
                let _ = fix.fix_same_parameter_default(&edge_3d)?;
            }
            // Computed again only over the maximum tolerance.
            if BRepTool::edge_tolerance(&edge_3d)? > self.max_tol {
                ShapeFixShapeTolerance.set_tolerance(
                    &edge_3d,
                    precision::confusion(),
                    ShapeEnum::Shape,
                )?;
                for done in 1..=rank {
                    ShapeBuildEdge.remove_pcurve_on_face(&list_3d.edge(done)?, face)?;
                }
                if !self.preferred_3d {
                    return Ok(piece_2d.clone());
                }
                break;
            }
        }
        Ok(piece_3d.clone())
    }
    /// The curves of the rank that did not connect: both whole wires are made and fixed, and the one
    /// with the lower tolerance is kept.
    fn rebuilt_after_gap(
        &mut self,
        curves: &BoundaryCurves<'_>,
        sequences: &BoundarySequences,
    ) -> Result<(), Failure> {
        let rebuilt_3d = Handle::new(ShapeExtendWireData::new());
        for rank in 1..=sequences.len_3d {
            if curves.use_wire_data {
                let edge = iges_live(curves.wire_data_3d, WHERE)?.edge(rank)?;
                iges_live(&rebuilt_3d, WHERE)?.add_edge(&edge, 0)?;
            } else {
                let shape = self.tool.transfer_topo_curve(&sequences.curve_3d(rank)?)?;
                if !shape.is_null() {
                    iges_live(&rebuilt_3d, WHERE)?.add_shape(&shape, 0)?;
                }
            }
        }
        if curves.reverse_3d {
            reverse_curves_3d(&rebuilt_3d)?;
        }
        let rebuilt_2d = Handle::new(ShapeExtendWireData::new());
        for rank in 1..=sequences.len_2d {
            let item = sequences.curve_2d(rank)?;
            let (face, trans, u_fact) =
                (self.place.face.clone(), self.place.trans, self.place.u_fact);
            let shape = self
                .tool
                .transfer_2d_topo_curve(&item, &face, trans, u_fact)?;
            if !shape.is_null() {
                iges_live(&rebuilt_2d, WHERE)?.add_shape(&shape, 0)?;
            }
        }
        if curves.reverse_2d {
            reverse_curves_2d(&rebuilt_2d, &self.place.face)?;
        }
        let wire_3d = self.fixed_on_face(&rebuilt_3d)?;
        let wire_2d = self.fixed_on_face(&rebuilt_2d)?;
        let mut measure = ShapeAnalysisShapeTolerance::default();
        let tol_3d = measure.tolerance(&wire_3d, 1, ShapeEnum::Shape)?;
        let tol_2d = measure.tolerance(&wire_2d, 1, ShapeEnum::Shape)?;
        self.local_3d = Handle::new(ShapeExtendWireData::new());
        self.local_2d = Handle::new(ShapeExtendWireData::new());
        iges_live(&self.local_3d, WHERE)?.add_wire(&wire_3d, 0)?;
        iges_live(&self.local_2d, WHERE)?.add_wire(&wire_3d, 0)?;
        let kept = if tol_3d < tol_2d { &wire_3d } else { &wire_2d };
        iges_live(&self.local, WHERE)?.add_wire(kept, 0)
    }
    fn fixed_on_face(&self, list: &Handle<ShapeExtendWireData>) -> Result<Wire, Failure> {
        let fix = wire_fix_on_face(
            &iges_live(list, WHERE)?.wire()?,
            &self.place.face,
            self.precision,
        )?;
        let _ = fix.perform()?;
        fix.wire()
    }
    /// One rank with both a 3D and a 2D curve.
    fn transfer_both(
        &mut self,
        flags: &mut BoundaryFlags,
        curves: &BoundaryCurves<'_>,
        sequences: &BoundarySequences,
        rank: i32,
    ) -> Result<(), Failure> {
        let piece_3d = Handle::new(ShapeExtendWireData::new());
        self.tool.set_bad_case(false);
        let mut has_3d = local_3d_curve(&mut self.tool, curves, sequences, rank, &piece_3d)?;
        let bad_3d = self.tool.bad_case();
        flags.ok_curve_3d =
            flags.ok_curve_3d && connect_next(&self.analysis_3d, &piece_3d, self.max_tol)?;
        let piece_2d = Handle::new(ShapeExtendWireData::new());
        self.tool.set_bad_case(false);
        let place = self.place.clone();
        let mut has_2d =
            local_2d_curve(&mut self.tool, curves, sequences, &place, (rank, &piece_2d))?;
        let bad_2d = self.tool.bad_case();
        if has_2d {
            flags.ok_curve_2d =
                flags.ok_curve_2d && connect_next(&self.analysis_2d, &piece_2d, self.max_tol)?;
        }
        let both_bspline = !curves.use_wire_data && is_bspline_pair(sequences, rank)?;
        let (nb_3d, nb_2d) = (
            iges_live(&piece_3d, WHERE)?.nb_edges(),
            iges_live(&piece_2d, WHERE)?.nb_edges(),
        );
        let inconsistent = (both_bspline && nb_3d != nb_2d)
            || (!both_bspline && (nb_3d != 1 || nb_2d != 1))
            || bad_3d
            || bad_2d;
        if has_3d && has_2d && inconsistent {
            has_3d = self.preferred_3d;
            has_2d = self.preferred_2d;
        }
        let chosen = if has_3d && !has_2d {
            piece_3d
        } else if !has_3d && has_2d {
            piece_2d
        } else {
            self.copied_pcurves(&piece_3d, &piece_2d)?
        };
        flags.ok_curve = flags.ok_curve && connect_next(&self.analysis, &chosen, self.max_tol)?;
        if !flags.ok_curve {
            self.rebuilt_after_gap(curves, sequences)?;
            *flags = BoundaryFlags::default();
        }
        Ok(())
    }
}

fn is_bspline_pair(sequences: &BoundarySequences, rank: i32) -> Result<bool, Failure> {
    let (curve_3d, curve_2d): (Handle<IgesDataIgesEntity>, Handle<IgesDataIgesEntity>) =
        (sequences.curve_3d(rank)?, sequences.curve_2d(rank)?);
    Ok(curve_3d.cast::<IgesGeomBSplineCurve>().is_some()
        && curve_2d.cast::<IgesGeomBSplineCurve>().is_some())
}

/// The preference on an inconsistency: the priority setting, then the file, then the 2D curves.
fn preferences(surface_curve: i32, file_preference: i32) -> (bool, bool) {
    match (surface_curve, file_preference) {
        (2, _) => (false, true),
        (3, _) => (true, false),
        (_, 3) => (true, false),
        _ => (false, true),
    }
}

impl IgesControlIgesBoundary {
    /// An empty boundary; `IgesControlIgesBoundary::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// An empty boundary with the settings of `settings`; `IgesControlIgesBoundary::with_base(&tool)`.
    pub fn with_base(settings: &IgesToBRepCurveAndSurface) -> Self {
        Self {
            base: IgesToBRepIgesBoundary::with_base(settings),
        }
    }
    fn new_job(
        &self,
        settings: &IgesToBRepCurveAndSurface,
        place: BoundaryPlace,
        preferred: (bool, bool),
    ) -> Result<BoundaryJob, Failure> {
        let precision = settings.get_eps_geom() * settings.get_unit_factor();
        let local = Handle::new(ShapeExtendWireData::new());
        let local_3d = Handle::new(ShapeExtendWireData::new());
        let local_2d = Handle::new(ShapeExtendWireData::new());
        Ok(BoundaryJob {
            tool: IgesToBRepTopoCurve::with_base(settings),
            place,
            precision,
            max_tol: settings.get_max_tol(),
            preferred_3d: preferred.0,
            preferred_2d: preferred.1,
            analysis: analysed_list(&local, precision)?,
            analysis_3d: analysed_list(&local_3d, precision)?,
            analysis_2d: analysed_list(&local_2d, precision)?,
            local,
            local_3d,
            local_2d,
        })
    }
    /// The representations of a rank in the lists of the boundary.
    fn store_rank(
        &self,
        flags: &mut BoundaryFlags,
        job: &BoundaryJob,
        curves: &BoundaryCurves<'_>,
        sequences: &BoundarySequences,
    ) -> Result<(), Failure> {
        let mut state = self.base.state.write();
        if curves.number <= 1 {
            state.wire_data = job.local.clone();
            state.wire_data_3d = job.local_3d.clone();
            state.wire_data_2d = job.local_2d.clone();
            return Ok(());
        }
        let (long_3d, long_2d) = (sequences.len_3d > 1, sequences.len_2d > 1);
        flags.ok_curve = flags.ok_curve
            && connect_lists(
                &job.analysis,
                &state.wire_data,
                &job.local,
                long_3d || long_2d,
                job.max_tol,
            )?;
        flags.ok_curve_3d = flags.ok_curve_3d
            && connect_lists(
                &job.analysis_3d,
                &state.wire_data_3d,
                &job.local_3d,
                long_3d,
                job.max_tol,
            )?;
        flags.ok_curve_2d = flags.ok_curve_2d
            && connect_lists(
                &job.analysis_2d,
                &state.wire_data_2d,
                &job.local_2d,
                long_2d,
                job.max_tol,
            )?;
        Ok(())
    }
}

impl IgesToBRepBoundaryOps for IgesControlIgesBoundary {
    fn base(&self) -> &IgesToBRepIgesBoundary {
        &self.base
    }
    fn check(
        &self,
        result: bool,
        check_closure: bool,
        ok_curve_3d: bool,
        ok_curve_2d: bool,
    ) -> Result<(), Failure> {
        let mut state = self.base.state.write();
        let max_tol = state.settings.get_max_tol();
        let (mut result, mut ok_3d, mut ok_2d) = (result, ok_curve_3d, ok_curve_2d);
        if result && check_closure {
            // USA60022 7277: the check of the closure.
            let analysis = ShapeAnalysisWire::new();
            analysis.load_wire_data(&state.wire_data);
            analysis.set_precision(max_tol);
            let _ = analysis.check_connected_at(1, 0.0)?;
            if analysis.last_check_status(ShapeExtendStatus::Fail) {
                analysis.load_wire_data(&state.wire_data_3d);
                let _ = analysis.check_connected_at(1, 0.0)?;
                if analysis.last_check_status(ShapeExtendStatus::Fail) {
                    ok_3d = false;
                } else {
                    ok_2d = false;
                }
                result = false;
            }
        }
        if result {
            return Ok(());
        }
        iges_live(&state.wire_data, WHERE)?.clear();
        let dimension = if ok_3d && iges_live(&state.wire_data_3d, WHERE)?.nb_edges() > 0 {
            state.wire_data = state.wire_data_3d.clone();
            3
        } else if ok_2d && iges_live(&state.wire_data_2d, WHERE)?.nb_edges() > 0 {
            state.wire_data = state.wire_data_2d.clone();
            2
        } else {
            return Ok(());
        };
        let mut message = Msg::from_key(&"IGES_1070".into());
        message.arg_integer(dimension)?;
        state.settings.send_warning(&state.entity, &message);
        Ok(())
    }
    fn transfer_curves(
        &self,
        flags: &mut BoundaryFlags,
        curves: &BoundaryCurves<'_>,
    ) -> Result<(bool, Handle<ShapeExtendWireData>), Failure> {
        let sequences = BoundarySequences::of(curves)?;
        let (settings, place, file_preference) = {
            let state = self.base.state.read();
            (
                state.settings.clone(),
                BoundaryPlace {
                    face: state.face.clone(),
                    trans: state.trans,
                    u_fact: state.u_fact,
                },
                state.file_preference,
            )
        };
        let surface_curve = settings.get_surface_curve();
        let (len_3d, len_2d) = (sequences.len_3d, sequences.len_2d);
        let mut translate_3d = !((surface_curve == -2 && len_2d > 0) || len_3d == 0);
        let mut translate_2d =
            !translate_3d || !((surface_curve == -3 && len_3d > 0) || len_2d == 0);
        let mut preferred = (true, true);
        if translate_3d && translate_2d {
            preferred = preferences(surface_curve, file_preference);
        }
        if translate_3d && translate_2d && len_3d != len_2d {
            (translate_3d, translate_2d) = preferred;
        }
        let mut job = self.new_job(&settings, place, preferred)?;
        if translate_3d && !translate_2d {
            let list = iges_live(&job.local, WHERE)?;
            transfer_3d_only(&mut job.tool, curves, list, &job.local_3d)?;
        } else if !translate_3d && translate_2d {
            let list = iges_live(&job.local, WHERE)?;
            transfer_2d_only(&mut job.tool, curves, &job.place, list, &job.local_2d)?;
        } else if translate_3d && translate_2d {
            for rank in 1..=len_3d {
                job.transfer_both(flags, curves, &sequences, rank)?;
            }
        }
        self.store_rank(flags, &job, curves, &sequences)?;
        Ok((flags.ok_curve, job.local))
    }
}
