//! `TopOpeBRepDSEdgeInterferenceTool`: the transition of an edge at a point or
//! vertex crossed by several edges, accumulated with a curve transition.
use super::export::{fds_parameter, iref};
use super::{TopOpeBRepDSInterference, TopOpeBRepDSPoint};
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Shape, ShapeIterator};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
use martensite_tkgeomalgo::toptrans::CurveTransition;
use martensite_tkmath::gp::Direction3;

/// The edge interference tool; natively the edge orientation and index start
/// uninitialized (FORWARD and 0 here);
/// `let mut t = TopOpeBRepDSEdgeInterferenceTool::new();`.
#[derive(Clone)]
pub struct TopOpeBRepDSEdgeInterferenceTool {
    my_edge_orientation: Orientation,
    my_edge_oriented: i32,
    my_tool: CurveTransition,
}

impl Default for TopOpeBRepDSEdgeInterferenceTool {
    fn default() -> Self {
        Self::new()
    }
}

/// `::Parameter(I)` (static): the parameter of an edge/vertex or curve/point
/// interference; StandardProgramError otherwise.
fn parameter(i: &Handle<TopOpeBRepDSInterference>) -> Result<f64, Failure> {
    fds_parameter(i).map_err(|_| {
        Failure::with_type(
            "Standard_ProgramError",
            "TopOpeBRepDSEdgeInterferenceTool1",
            "TopOpeBRepDSEdgeInterferenceTool",
        )
    })
}

/// Whether an orientation is INTERNAL or EXTERNAL.
fn inex(o: Orientation) -> bool {
    o == Orientation::Internal || o == Orientation::External
}

impl TopOpeBRepDSEdgeInterferenceTool {
    /// `TopOpeBRepDSEdgeInterferenceTool()`; `TopOpeBRepDSEdgeInterferenceTool::new()`.
    pub fn new() -> Self {
        Self {
            my_edge_orientation: Orientation::Forward,
            my_edge_oriented: 0,
            my_tool: CurveTransition::new(),
        }
    }

    /// `Init(E, I)`: the reference edge (its orientation, the support of `i`) and, for
    /// a FORWARD/REVERSED edge, its geometry at the parameter of `i`;
    /// `t.init(&e, &i)?`.
    pub fn init(&mut self, e: &Shape, i: &Handle<TopOpeBRepDSInterference>) -> Result<(), Failure> {
        self.my_edge_orientation = e.orientation();
        self.my_edge_oriented = iref(i)?.support();
        if inex(self.my_edge_orientation) {
            return Ok(());
        }
        let par = parameter(i)?;
        let (mut t, mut n, mut c) = (Direction3::default(), Direction3::default(), 0.);
        TopOpeBRepToolShapeTool::edge_data_of_edge(e, par, &mut t, &mut n, &mut c)?;
        self.my_tool.reset(t, n, c);
        Ok(())
    }

    /// `Add(E, V, I)`: the edge `e` crossing at its vertex `v` compared (nothing when
    /// `v` is no vertex of `e`, or `e` INTERNAL/EXTERNAL; `Init` when the reference is);
    /// `t.add_vertex(&e, &v, &i)?`.
    pub fn add_vertex(
        &mut self,
        e: &Shape,
        v: &Shape,
        i: &Handle<TopOpeBRepDSInterference>,
    ) -> Result<(), Failure> {
        if inex(e.orientation()) {
            return Ok(());
        }
        if inex(self.my_edge_orientation) {
            return self.init(e, i);
        }
        let mut it = ShapeIterator::new(e, false, true);
        let mut oritan = None;
        while it.more() {
            if it.value()?.is_same(v) {
                oritan = Some(it.value()?.orientation());
                break;
            }
            it.next();
        }
        let Some(oritan) = oritan else {
            return Ok(());
        };
        let par = BRepTool::parameter(&topods::vertex(v)?, &topods::edge(e)?)?;
        let (mut t, mut n, mut c) = (Direction3::default(), Direction3::default(), 0.);
        let tol = TopOpeBRepToolShapeTool::edge_data_of_edge(e, par, &mut t, &mut n, &mut c)?;
        let oriloc = iref(i)?
            .transition()
            .orientation(State::In, ShapeEnum::Face);
        self.my_tool.compare(tol, t, n, c, oriloc, oritan);
        Ok(())
    }

    /// `Add(E, P, I)`: the edge `e` crossing at the point of `i` compared (as INTERNAL);
    /// `t.add_point(&e, &p, &i)?`.
    pub fn add_point(
        &mut self,
        e: &Shape,
        _p: &TopOpeBRepDSPoint,
        i: &Handle<TopOpeBRepDSInterference>,
    ) -> Result<(), Failure> {
        if inex(e.orientation()) {
            return Ok(());
        }
        if inex(self.my_edge_orientation) {
            return self.init(e, i);
        }
        let par = parameter(i)?;
        let (mut t, mut n, mut c) = (Direction3::default(), Direction3::default(), 0.);
        let tol = TopOpeBRepToolShapeTool::edge_data_of_edge(e, par, &mut t, &mut n, &mut c)?;
        let oriloc = iref(i)?
            .transition()
            .orientation(State::In, ShapeEnum::Face);
        self.my_tool
            .compare(tol, t, n, c, oriloc, Orientation::Internal);
        Ok(())
    }

    /// `Transition(I)`: the transition of `i` set to IN/IN (INTERNAL reference), OUT/OUT
    /// (EXTERNAL) or the accumulated states with the reference edge as support (on
    /// FACE shapes); `t.transition(&i)?`.
    pub fn transition(&self, i: &Handle<TopOpeBRepDSInterference>) -> Result<(), Failure> {
        let ii = iref(i)?;
        let (b, a) = match self.my_edge_orientation {
            Orientation::Internal => (State::In, State::In),
            Orientation::External => (State::Out, State::Out),
            _ => {
                ii.set_support(self.my_edge_oriented);
                (self.my_tool.state_before(), self.my_tool.state_after())
            }
        };
        ii.change_transition(|t| t.set(b, a, ShapeEnum::Face, ShapeEnum::Face));
        Ok(())
    }
}
