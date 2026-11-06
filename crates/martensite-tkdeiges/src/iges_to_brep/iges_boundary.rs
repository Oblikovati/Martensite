//! `IgesToBRepIgesBoundary`: the wires of boundary entities (types 141, 142 and 508), with their 3D
//! and 2D representations. A tool container gives the kind of boundary a transfer uses.
use super::{IgesToBRep, IgesToBRepCurveAndSurface, IgesToBRepTopoCurve};
use crate::iges_data::{IgesDataIgesEntity, iges_array1_item, iges_live};
use martensite_tkbrep::topods::{Face, Wire};
use martensite_tkernel::impl_transient;
use martensite_tkernel::impl_transient_interface;
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::standard::{
    Failure, Handle, SharedState, StandardTransient, Transient, exceptions,
};
use martensite_tkmath::gp::Transform2;
use martensite_tkshhealing::shape_analysis::ShapeAnalysisWire;
use martensite_tkshhealing::shape_extend::ShapeExtendWireData;
use martensite_tkshhealing::shape_fix::{FixRoot, ShapeFixWire};

use super::iges_boundary_reverse::{reverse_curves_2d, reverse_curves_3d};

const WHERE: &str = "IgesToBRepIgesBoundary";

/// The parameter curves of a boundary.
pub type IgesToBRepCurves2d = Handle<HArray1<Handle<IgesDataIgesEntity>>>;

/// Which representations of a boundary are good so far; all are until a transfer says otherwise.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundaryFlags {
    /// The wire is connected.
    pub ok_curve: bool,
    /// The 3D representation is connected.
    pub ok_curve_3d: bool,
    /// The 2D representation is connected.
    pub ok_curve_2d: bool,
}

impl Default for BoundaryFlags {
    fn default() -> Self {
        Self {
            ok_curve: true,
            ok_curve_3d: true,
            ok_curve_2d: true,
        }
    }
}

/// The curves of one transfer: a 3D entity or the edges made of one, with the parameter curves.
pub struct BoundaryCurves<'a> {
    /// The 3D curve entity, when `use_wire_data` is false.
    pub entity_3d: &'a Handle<IgesDataIgesEntity>,
    /// The edges of the 3D curve, when `use_wire_data` is true.
    pub wire_data_3d: &'a Handle<ShapeExtendWireData>,
    /// Which of those to use.
    pub use_wire_data: bool,
    /// The sense of the 3D curve is against the boundary.
    pub reverse_3d: bool,
    /// The parameter curves.
    pub curves_2d: &'a IgesToBRepCurves2d,
    /// The sense of the parameter curves is against the boundary.
    pub reverse_2d: bool,
    /// The rank of the curve in the boundary, from 1.
    pub number: i32,
}

/// The kinds of boundary: the translation of the curves and the check of their representations.
pub trait IgesToBRepBoundaryOps: Transient {
    /// The state of the boundary.
    fn base(&self) -> &IgesToBRepIgesBoundary;
    /// Translates the curves of one rank; gives whether it went well and their local wire.
    ///
    /// # Errors
    /// A failure of an entity or of the construction of the wire.
    fn transfer_curves(
        &self,
        flags: &mut BoundaryFlags,
        curves: &BoundaryCurves<'_>,
    ) -> Result<(bool, Handle<ShapeExtendWireData>), Failure>;
    /// Checks that the two representations agree, and keeps one if they do not.
    /// `check_closure` is false for a curve on surface without a parent trimmed surface.
    ///
    /// # Errors
    /// A failure of the wire analysis.
    fn check(
        &self,
        result: bool,
        check_closure: bool,
        ok_curve_3d: bool,
        ok_curve_2d: bool,
    ) -> Result<(), Failure>;
}
impl_transient_interface!(
    dyn IgesToBRepBoundaryOps,
    "IgesToBRepBoundaryOps",
    StandardTransient
);

impl dyn IgesToBRepBoundaryOps {
    /// Sets what is common to all kinds of boundary: the settings, the entity, the face with its
    /// parameter transformation; `file_preference` is the preferred representation in the file, 2 or 3.
    /// Example: `boundary.init(&tool, &entity, &face, trans, 1.0, 0)`.
    pub fn init(
        &self,
        settings: &IgesToBRepCurveAndSurface,
        entity: &Handle<IgesDataIgesEntity>,
        face: &Face,
        trans: Transform2,
        u_fact: f64,
        file_preference: i32,
    ) {
        let base = self.base();
        let mut state = base.state.write();
        state.settings = settings.clone();
        state.entity = entity.clone();
        state.face = face.clone();
        state.trans = trans;
        state.u_fact = u_fact;
        state.file_preference = file_preference;
    }
    /// Translates entities of types 141 and 142; gives whether it went well.
    /// Example: `boundary.transfer_entity(&mut flags, &curve, false, &curves, 1)?`.
    ///
    /// # Errors
    /// A failure of an entity or of the construction of the wire.
    pub fn transfer_entity(
        &self,
        flags: &mut BoundaryFlags,
        curve_3d: &Handle<IgesDataIgesEntity>,
        reverse_3d: bool,
        curves_2d: &IgesToBRepCurves2d,
        number: i32,
    ) -> Result<bool, Failure> {
        let curves = BoundaryCurves {
            entity_3d: curve_3d,
            wire_data_3d: &Handle::default(),
            use_wire_data: false,
            reverse_3d,
            curves_2d,
            reverse_2d: false,
            number,
        };
        Ok(self.transfer_curves(flags, &curves)?.0)
    }
    /// Translates an edge of type 508 from its 3D edges; gives whether it went well and its wire.
    /// Example: `boundary.transfer_wire_data(&mut flags, &edges, &curves, false, 1)?`.
    ///
    /// # Errors
    /// A failure of an entity or of the construction of the wire.
    pub fn transfer_wire_data(
        &self,
        flags: &mut BoundaryFlags,
        curve_3d: &Handle<ShapeExtendWireData>,
        curves_2d: &IgesToBRepCurves2d,
        reverse_2d: bool,
        number: i32,
    ) -> Result<(bool, Handle<ShapeExtendWireData>), Failure> {
        let curves = BoundaryCurves {
            entity_3d: &Handle::default(),
            wire_data_3d: curve_3d,
            use_wire_data: true,
            reverse_3d: false,
            curves_2d,
            reverse_2d,
            number,
        };
        self.transfer_curves(flags, &curves)
    }
}

/// What a boundary holds between its transfers.
#[derive(Default)]
pub struct BoundaryState {
    pub(crate) settings: IgesToBRepCurveAndSurface,
    pub(crate) entity: Handle<IgesDataIgesEntity>,
    pub(crate) wire_data: Handle<ShapeExtendWireData>,
    pub(crate) wire_data_3d: Handle<ShapeExtendWireData>,
    pub(crate) wire_data_2d: Handle<ShapeExtendWireData>,
    pub(crate) face: Face,
    pub(crate) trans: Transform2,
    pub(crate) u_fact: f64,
    pub(crate) file_preference: i32,
}

/// The boundary with the plain rules, and the state of the others.
///
/// ```
/// use martensite_tkdeiges::iges_to_brep::IgesToBRepIgesBoundary;
/// let boundary = IgesToBRepIgesBoundary::new();
/// assert!(boundary.wire_data().is_null());
/// ```
#[derive(Default)]
pub struct IgesToBRepIgesBoundary {
    transient: StandardTransient,
    pub(crate) state: SharedState<BoundaryState>,
}
impl_transient!(
    IgesToBRepIgesBoundary,
    "IgesToBRepIgesBoundary",
    dyn IgesToBRepBoundaryOps,
    transient,
    interfaces: [dyn IgesToBRepBoundaryOps]
);

impl IgesToBRepIgesBoundary {
    /// An empty boundary; `IgesToBRepIgesBoundary::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// An empty boundary with the settings of `settings`; `IgesToBRepIgesBoundary::with_base(&tool)`.
    pub fn with_base(settings: &IgesToBRepCurveAndSurface) -> Self {
        let boundary = Self::default();
        boundary.state.write().settings = settings.clone();
        boundary
    }
    /// The resulting wire; `boundary.wire_data()`.
    pub fn wire_data(&self) -> Handle<ShapeExtendWireData> {
        self.state.read().wire_data.clone()
    }
    /// The wire from 3D curves: its edges hold them and may hold pcurves; `boundary.wire_data_3d()`.
    pub fn wire_data_3d(&self) -> Handle<ShapeExtendWireData> {
        self.state.read().wire_data_3d.clone()
    }
    /// The wire from 2D curves: its edges hold only pcurves; `boundary.wire_data_2d()`.
    pub fn wire_data_2d(&self) -> Handle<ShapeExtendWireData> {
        self.state.read().wire_data_2d.clone()
    }
}

/// The sequences of curves and their preference, as both kinds of boundary find them.
pub(crate) struct BoundarySequences {
    pub(crate) seq_3d: Vec<Handle<StandardTransient>>,
    pub(crate) seq_2d: Vec<Handle<StandardTransient>>,
    pub(crate) len_3d: i32,
    pub(crate) len_2d: i32,
}

impl BoundarySequences {
    pub(crate) fn of(curves: &BoundaryCurves<'_>) -> Result<Self, Failure> {
        let mut seq_3d = Vec::new();
        let len_3d = if curves.use_wire_data {
            iges_live(curves.wire_data_3d, WHERE)?.nb_edges()
        } else {
            IgesToBRep::iges_curve_to_sequence_of_iges_curve(curves.entity_3d, &mut seq_3d)
        };
        let mut seq_2d = Vec::new();
        let mut len_2d = 0;
        if let Some(list) = curves.curves_2d.get() {
            let array = list.array1();
            for rank in array.lower()..=array.upper() {
                let item = iges_array1_item(curves.curves_2d, rank, WHERE)?;
                len_2d = IgesToBRep::iges_curve_to_sequence_of_iges_curve(&item, &mut seq_2d);
            }
        }
        Ok(Self {
            seq_3d,
            seq_2d,
            len_3d,
            len_2d,
        })
    }
    /// The 3D curve of a rank from 1.
    pub(crate) fn curve_3d(&self, rank: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        rank_of(&self.seq_3d, rank)
    }
    /// The 2D curve of a rank from 1.
    pub(crate) fn curve_2d(&self, rank: i32) -> Result<Handle<IgesDataIgesEntity>, Failure> {
        rank_of(&self.seq_2d, rank)
    }
}

fn rank_of(
    list: &[Handle<StandardTransient>],
    rank: i32,
) -> Result<Handle<IgesDataIgesEntity>, Failure> {
    usize::try_from(rank - 1)
        .ok()
        .and_then(|index| list.get(index))
        .map(Handle::cast)
        .ok_or_else(|| {
            exceptions::out_of_range(&format!(
                "{WHERE}: curve rank {rank}, expected 1..={}",
                list.len()
            ))
        })
}

/// The 3D wire alone: a given wire, or the translation of the 3D entity.
pub(crate) fn transfer_3d_only(
    tool: &mut IgesToBRepTopoCurve,
    curves: &BoundaryCurves<'_>,
    local: &ShapeExtendWireData,
    local_3d: &Handle<ShapeExtendWireData>,
) -> Result<(), Failure> {
    if curves.use_wire_data {
        return local.add_wire(&iges_live(curves.wire_data_3d, WHERE)?.wire()?, 0);
    }
    let shape = tool.transfer_topo_curve(curves.entity_3d)?;
    if shape.is_null() {
        return Ok(());
    }
    iges_live(local_3d, WHERE)?.add_shape(&shape, 0)?;
    if curves.reverse_3d {
        reverse_curves_3d(local_3d)?;
        return local.add_wire(&iges_live(local_3d, WHERE)?.wire()?, 0);
    }
    // Sharing the local 3D list is not possible (UK1.igs entity 7): the shape is added.
    local.add_shape(&shape, 0)
}

/// The 2D wire alone, from the translation of each parameter curve.
pub(crate) fn transfer_2d_only(
    tool: &mut IgesToBRepTopoCurve,
    curves: &BoundaryCurves<'_>,
    state: &BoundaryPlace,
    local: &ShapeExtendWireData,
    local_2d: &Handle<ShapeExtendWireData>,
) -> Result<(), Failure> {
    let list = iges_live(curves.curves_2d, WHERE)?;
    let (lower, upper) = (list.array1().lower(), list.array1().upper());
    for rank in lower..=upper {
        let item = iges_array1_item(curves.curves_2d, rank, WHERE)?;
        let shape = tool.transfer_2d_topo_curve(&item, &state.face, state.trans, state.u_fact)?;
        if !shape.is_null() {
            iges_live(local_2d, WHERE)?.add_shape(&shape, 0)?;
        }
    }
    if curves.reverse_2d {
        reverse_curves_2d(local_2d, &state.face)?;
    }
    local.add_wire(&iges_live(local_2d, WHERE)?.wire()?, 0)
}

/// The face and the parameter transformation of a boundary.
#[derive(Clone)]
pub(crate) struct BoundaryPlace {
    pub(crate) face: Face,
    pub(crate) trans: Transform2,
    pub(crate) u_fact: f64,
}

/// One 3D curve of a rank as a local wire; `false` when it gave no shape.
pub(crate) fn local_3d_curve(
    tool: &mut IgesToBRepTopoCurve,
    curves: &BoundaryCurves<'_>,
    sequences: &BoundarySequences,
    rank: i32,
    local: &Handle<ShapeExtendWireData>,
) -> Result<bool, Failure> {
    let list = iges_live(local, WHERE)?;
    if curves.use_wire_data {
        list.add_edge(&iges_live(curves.wire_data_3d, WHERE)?.edge(rank)?, 0)?;
        return Ok(true);
    }
    let shape = tool.transfer_topo_curve(&sequences.curve_3d(rank)?)?;
    if shape.is_null() {
        return Ok(false);
    }
    list.add_shape(&shape, 0)?;
    if curves.reverse_3d {
        reverse_curves_3d(local)?;
    }
    Ok(true)
}

/// One 2D curve of a rank as a local wire; `false` when it gave no shape.
pub(crate) fn local_2d_curve(
    tool: &mut IgesToBRepTopoCurve,
    curves: &BoundaryCurves<'_>,
    sequences: &BoundarySequences,
    place: &BoundaryPlace,
    (rank, local): (i32, &Handle<ShapeExtendWireData>),
) -> Result<bool, Failure> {
    let item = sequences.curve_2d(rank)?;
    let shape = tool.transfer_2d_topo_curve(&item, &place.face, place.trans, place.u_fact)?;
    if shape.is_null() {
        return Ok(false);
    }
    iges_live(local, WHERE)?.add_shape(&shape, 0)?;
    if curves.reverse_2d {
        reverse_curves_2d(local, &place.face)?;
    }
    Ok(true)
}

impl IgesToBRepBoundaryOps for IgesToBRepIgesBoundary {
    fn base(&self) -> &IgesToBRepIgesBoundary {
        self
    }
    /// Implemented by the checking kind of boundary; the plain one has nothing to check.
    fn check(&self, _: bool, _: bool, _: bool, _: bool) -> Result<(), Failure> {
        Ok(())
    }
    fn transfer_curves(
        &self,
        _flags: &mut BoundaryFlags,
        curves: &BoundaryCurves<'_>,
    ) -> Result<(bool, Handle<ShapeExtendWireData>), Failure> {
        let sequences = BoundarySequences::of(curves)?;
        let (settings, place, file_preference) = {
            let state = self.state.read();
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
        let translate_3d = sequences.len_3d != 0;
        let translate_2d = !translate_3d || sequences.len_2d != 0;
        let (mut preferred_3d, mut preferred_2d) = (true, true);
        if translate_3d && translate_2d {
            if file_preference == 3 {
                preferred_2d = false;
            } else {
                preferred_3d = false;
            }
        }
        let differ = translate_3d && translate_2d && sequences.len_3d != sequences.len_2d;
        let translate_3d = if differ { preferred_3d } else { translate_3d };
        let translate_2d = if differ { preferred_2d } else { translate_2d };
        let local = Handle::new(ShapeExtendWireData::new());
        let local_3d = Handle::new(ShapeExtendWireData::new());
        let local_2d = Handle::new(ShapeExtendWireData::new());
        let mut tool = IgesToBRepTopoCurve::with_base(&settings);
        let list = iges_live(&local, WHERE)?;
        if translate_3d && !translate_2d {
            transfer_3d_only(&mut tool, curves, list, &local_3d)?;
        } else if !translate_3d && translate_2d {
            transfer_2d_only(&mut tool, curves, &place, list, &local_2d)?;
        } else if translate_3d && translate_2d {
            for rank in 1..=sequences.len_3d {
                tool.set_bad_case(false);
                let piece_3d = Handle::new(ShapeExtendWireData::new());
                let mut has_3d = local_3d_curve(&mut tool, curves, &sequences, rank, &piece_3d)?;
                let live_3d = iges_live(&piece_3d, WHERE)?;
                iges_live(&local_3d, WHERE)?.add_wire(&live_3d.wire()?, 0)?;
                let piece_2d = Handle::new(ShapeExtendWireData::new());
                let mut has_2d =
                    local_2d_curve(&mut tool, curves, &sequences, &place, (rank, &piece_2d))?;
                let live_2d = iges_live(&piece_2d, WHERE)?;
                if has_2d {
                    iges_live(&local_2d, WHERE)?.add_wire(&live_2d.wire()?, 0)?;
                }
                if has_3d && has_2d && live_3d.nb_edges() != live_2d.nb_edges() {
                    has_3d = preferred_3d;
                    has_2d = preferred_2d;
                }
                let chosen = if !has_3d && has_2d { live_2d } else { live_3d };
                if !(has_3d != has_2d) {
                    for edge in 1..=live_3d.nb_edges() {
                        IgesToBRep::transfer_pcurve(
                            &live_2d.edge(edge)?,
                            &live_3d.edge(edge)?,
                            &place.face,
                        )?;
                    }
                }
                list.add_wire(&chosen.wire()?, 0)?;
            }
        }
        let mut state = self.state.write();
        if curves.number > 1 {
            iges_live(&state.wire_data, WHERE)?.add_wire(&list.wire()?, 0)?;
            iges_live(&state.wire_data_3d, WHERE)?
                .add_wire(&iges_live(&local_3d, WHERE)?.wire()?, 0)?;
            iges_live(&state.wire_data_2d, WHERE)?
                .add_wire(&iges_live(&local_2d, WHERE)?.wire()?, 0)?;
        } else {
            state.wire_data = local.clone();
            state.wire_data_3d = local_3d;
            state.wire_data_2d = local_2d;
        }
        Ok((true, local))
    }
}

/// A wire fix loaded with a wire on a face, at a precision.
pub(crate) fn wire_fix_on_face(
    wire: &Wire,
    face: &Face,
    precision: f64,
) -> Result<ShapeFixWire, Failure> {
    let analyzer = Handle::new(ShapeAnalysisWire::from_wire(wire, face, precision)?);
    let fix = ShapeFixWire::new();
    fix.init(&analyzer);
    fix.set_precision(precision);
    fix.set_max_tolerance(precision);
    Ok(fix)
}
