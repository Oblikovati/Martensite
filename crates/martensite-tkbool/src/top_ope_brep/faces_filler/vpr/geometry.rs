//! VPoint geometry lookup retains the source search ordering and parameter fallback.
use super::{GeometrySearch, TopOpeBRepFacesFiller};
use crate::top_ope_brep::{TopOpeBRepLineInter, TopOpeBRepTypeLineCurve, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::export::fds_parameter;
use crate::top_ope_brep_ds::{ListOfInterference, TopOpeBRepDSInterference, TopOpeBRepDSKind};
use crate::top_ope_brep_tool::fun_tool_maxtol;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::State;

pub(super) fn on_parameter(
    list: &ListOfInterference,
    position: &mut usize,
    parameter: f64,
    tolerance: f64,
) -> Result<Option<(i32, TopOpeBRepDSKind)>, Failure> {
    for (index, interference) in list.iter().enumerate().skip(*position) {
        *position = index;
        let Ok(value) = fds_parameter(interference) else {
            continue;
        };
        if (parameter - value).abs() < tolerance {
            let entry = interference
                .get()
                .ok_or_else(super::super::missing_interference)?;
            return Ok(Some((entry.geometry(), entry.geometry_type())));
        }
    }
    *position = list.len();
    Ok(None)
}

struct Restriction {
    rank: i32,
    edge: Shape,
    parameter: f64,
}

impl Restriction {
    fn of(line: &TopOpeBRepLineInter, point: &TopOpeBRepVPointInter<'_>) -> Result<Self, Failure> {
        if line.type_line_curve() != TopOpeBRepTypeLineCurve::Restriction {
            return Ok(Self {
                rank: 0,
                edge: Shape::default(),
                parameter: 0.0,
            });
        }
        let rank = if line.arc_is_edge(1)? {
            1
        } else if line.arc_is_edge(2)? {
            2
        } else {
            0
        };
        Ok(Self {
            rank,
            edge: line.arc()?,
            parameter: point.parameter_on_line()?,
        })
    }

    fn incidence(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        side: i32,
        on_two: bool,
    ) -> Result<(Shape, f64), Failure> {
        if self.rank == side {
            return Ok((self.edge.clone(), self.parameter));
        }
        if on_two {
            return Ok((point.edge(side)?, point.edge_parameter(side)?));
        }
        Ok((point.edge_on(side)?.clone(), point.edge_on_parameter(side)?))
    }
}

/// What a search for existing geometry found: its index and kind, whether it is on an edge, and
/// the interference it came from.
type GeometrySearchHit = (
    Option<(i32, TopOpeBRepDSKind)>,
    bool,
    Handle<TopOpeBRepDSInterference>,
);

fn lookup(
    filler: &TopOpeBRepFacesFiller<'_>,
    list: &ListOfInterference,
    point: &TopOpeBRepVPointInter<'_>,
    parameter: f64,
    tolerance: f64,
) -> Result<GeometrySearchHit, Failure> {
    let mut position = 0;
    let geometric = filler.get_geometry(list, &mut position, point)?;
    let mut result = geometric;
    if result.is_none() {
        position = 0;
        result = on_parameter(list, &mut position, parameter, tolerance)?;
    }
    let handle = list.iter().nth(position).cloned().unwrap_or_default();
    Ok((result, geometric.is_some(), handle))
}

fn geometry_found(
    filler: &TopOpeBRepFacesFiller<'_>,
    line: &TopOpeBRepLineInter,
    point: &TopOpeBRepVPointInter<'_>,
    side: i32,
    search: &mut GeometrySearch,
) -> Result<bool, Failure> {
    let restriction = Restriction::of(line, point)?;
    let other = if side == 1 { 2 } else { 1 };
    let opposite_mask = if point.shape_index() == 1 { 2 } else { 1 };
    let on_two = point.shape_index() == 3 || restriction.rank == opposite_mask;
    let (edge, parameter) = restriction.incidence(point, side, true)?;
    let tolerance = precision::parametric(fun_tool_maxtol(&edge)?);
    search.index = 0;
    search.edge_found = false;
    search.curve_found = false;
    let storage = filler.hds.get().ok_or_else(super::super::missing_storage)?;
    let edge_list = storage.ds().shape_interferences(&edge, true).clone();
    if storage.ds().has_shape(&edge, true) {
        let (geometry, _, interference) = lookup(filler, &edge_list, point, parameter, tolerance)?;
        if let Some((index, kind)) = geometry {
            (
                search.index,
                search.kind,
                search.edge_found,
                search.edge_interference,
            ) = (index, kind, true, interference);
        }
    }
    let mut position = 0;
    if let Some((index, kind)) =
        filler.get_geometry(&filler.curve_interferences, &mut position, point)?
    {
        (search.index, search.kind, search.curve_found) = (index, kind, true);
        search.curve_interference = filler
            .curve_interferences
            .iter()
            .nth(position)
            .cloned()
            .unwrap_or_default();
    }
    if !on_two && point.state(other)? != State::On {
        return Ok(false);
    }
    let (other_edge, other_parameter) = restriction.incidence(point, other, on_two)?;
    let tolerance = precision::parametric(fun_tool_maxtol(&other_edge)?);
    let list = storage.ds().shape_interferences(&other_edge, true).clone();
    if !storage.ds().has_shape(&other_edge, true) {
        return Ok(false);
    }
    let (geometry, geometric_match, _) = lookup(filler, &list, point, other_parameter, tolerance)?;
    if let Some((index, kind)) = geometry {
        (search.index, search.kind) = (index, kind);
    }
    // The source fails to assign the parameter-only match to OOEPIfound.
    Ok(geometric_match)
}

pub(in crate::top_ope_brep::faces_filler) fn vp_index(
    filler: &mut TopOpeBRepFacesFiller<'_>,
    line: &TopOpeBRepLineInter,
    point: &TopOpeBRepVPointInter<'_>,
    side: i32,
    mode: i32,
    search: &mut GeometrySearch,
) -> Result<(), Failure> {
    search.index = 0;
    let other = if side == 1 { 2 } else { 1 };
    let vertex = point.is_vertex(side)?;
    let other_vertex = point.is_vertex(other)?;
    let other_found = if mode == 0 || mode == 2 {
        geometry_found(filler, line, point, side, search)?
    } else {
        false
    };
    let found = search.edge_found || search.curve_found || other_found;
    if mode == 0 {
        if !found {
            search.index = 0;
        }
        return Ok(());
    }
    if search.index == 0 {
        search.kind = if vertex || other_vertex {
            TopOpeBRepDSKind::Vertex
        } else {
            TopOpeBRepDSKind::Point
        };
    }
    if found {
        return Ok(());
    }
    let has_other = point.shape_index() == 3 || point.state(other)? == State::On;
    if !has_other && let Some((kind, index)) = filler.get_ff_geometry_vpoint(point)? {
        (search.kind, search.index) = (kind, index);
        return Ok(());
    }
    let preferred_side = if !vertex && other_vertex { other } else { side };
    (search.kind, search.index) = filler.make_geometry(point, preferred_side)?;
    Ok(())
}
