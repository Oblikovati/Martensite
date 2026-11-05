//! Endpoint pairing uses operation-local state, including closed-curve seam decisions.
use super::{
    TopOpeBRepLineInter as Line, TopOpeBRepPointGeomTool as PointTool,
    TopOpeBRepTypeLineCurve as LineKind, TopOpeBRepVPointInter as VPoint,
};
use crate::top_ope_brep_ds::{
    ListOfInterference as Interferences, TopOpeBRepDSDataStructure as DS,
    TopOpeBRepDSInterference as Interference, TopOpeBRepDSInterferenceTool as InterferenceTool,
    TopOpeBRepDSKind as Kind, TopOpeBRepDSTransition as Transition,
};
use crate::top_ope_brep_tool::TopOpeBRepToolShapeTool;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
mod geometric;
mod parameter;
mod walking;

/// Retains the previously accepted endpoint within one operation; `TopOpeBRepKPartContext::default()`.
#[derive(Default, Debug)]
pub struct TopOpeBRepKPartContext {
    pub(crate) last_vpoint_index: i32,
}

struct Pairing<'a> {
    found: &'a Handle<Interference>,
    list: &'a Interferences,
    point: &'a VPoint<'a>,
    edge: &'a Shape,
    same_parameter: bool,
    seam: bool,
    first: i32,
    last: i32,
}
impl Pairing<'_> {
    fn previous(&self) -> Result<Transition, Failure> {
        self.list
            .iter()
            .last()
            .map(|i| {
                i.get()
                    .map(|v| v.transition())
                    .ok_or_else(null_interference)
            })
            .transpose()
            .map(|t| t.unwrap_or_default())
    }
    fn oriented_edge(&self) -> bool {
        matches!(
            self.edge.orientation(),
            Orientation::Forward | Orientation::Reversed
        )
    }
}
fn null_interference() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "null endpoint interference; expected populated interference",
    )
}
fn leaving(transition: &Transition) -> bool {
    transition.orientation(State::Out, ShapeEnum::Face) == Orientation::Forward
}
fn entering() -> Transition {
    Transition::new(State::Out, State::In, ShapeEnum::Face, ShapeEnum::Face)
}
fn exiting() -> Transition {
    Transition::new(State::In, State::Out, ShapeEnum::Face, ShapeEnum::Face)
}

/// Selects and parametrizes a curve endpoint; `funbrep_topokpart(&mut context,&found,&list,&mut line,&point,&ds,&edge,&face,1e-8,&mut parameter,&mut transition)?`.
#[allow(clippy::too_many_arguments)]
pub fn funbrep_topokpart(
    context: &mut TopOpeBRepKPartContext,
    found: &Handle<Interference>,
    list: &Interferences,
    line: &mut Line,
    point: &VPoint<'_>,
    ds: &DS,
    edge: &Shape,
    face: &Shape,
    tolerance_uv: f64,
    parameter: &mut f64,
    transition: &mut Transition,
) -> Result<bool, Failure> {
    let seam = TopOpeBRepToolShapeTool::closed(edge, face)?;
    let same_parameter = parameter::same_parameter(line, found, *parameter, seam)?;
    let (first, last, _) = line.vp_bounds();
    let pairing = Pairing {
        found,
        list,
        point,
        edge,
        same_parameter,
        seam,
        first,
        last,
    };
    let keep = if line.type_line_curve() == LineKind::Walking {
        walking::keep(&pairing, line, transition)?
    } else {
        geometric::keep(
            context,
            &pairing,
            line,
            ds,
            tolerance_uv,
            parameter,
            transition,
        )?
    };
    if keep {
        context.last_vpoint_index = point.index();
    }
    Ok(keep)
}
#[cfg(test)]
mod tests;
