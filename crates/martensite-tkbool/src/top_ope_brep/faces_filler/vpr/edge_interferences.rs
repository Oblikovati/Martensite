//! Stores edge incidences with the original face or edge support kind.
use super::{GeometrySearch, TopOpeBRepFacesFiller, context::Context, selection};
use crate::top_ope_brep::TopOpeBRepVPointInter;
use crate::top_ope_brep_ds::{
    TopOpeBRepDSKind as Kind, TopOpeBRepDSTransition as Transition,
    export::{fds_has_same_domain3d, fds_lo_iinfsup},
    process_interferences_tool::{make_epv_interference, make_epv_interference_sk},
};
use crate::top_ope_brep_tool::{
    TopOpeBRepToolMakeTransition, fun_tool_etg_f, fun_tool_nggeom_f, fun_tool_onapex,
    fun_tool_paron_ef, fun_tool_tggeom_e_vec,
};
use martensite_tkbrep::{brep::BRepTool, topexp, topods::Edge};
use martensite_tkernel::{precision, standard::Failure};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};
use martensite_tkmath::gp::{Direction3, Point2};

/// One interference to store on an edge: on the edge's support, or on the face's when
/// `face_support` holds.
pub(super) struct EdgeInterferenceRequest<'a> {
    pub edge: &'a Edge,
    pub transition: &'a Transition,
    pub face_index: i32,
    pub support: i32,
    pub parameter: f64,
    pub search: &'a GeometrySearch,
    pub vertex: bool,
    pub face_support: bool,
}

pub(super) fn store_edge(
    filler: &TopOpeBRepFacesFiller<'_>,
    request: &EdgeInterferenceRequest<'_>,
) -> Result<(), Failure> {
    let EdgeInterferenceRequest {
        edge,
        transition,
        face_index,
        support,
        parameter,
        search,
        vertex,
        face_support,
    } = *request;
    let mut transition = *transition;
    transition.set_index(face_index);
    let interference = if face_support {
        make_epv_interference_sk(
            &transition,
            support,
            search.index,
            parameter,
            search.kind,
            Kind::Face,
            vertex,
        )?
    } else {
        make_epv_interference(
            &transition,
            support,
            search.index,
            parameter,
            search.kind,
            vertex,
        )
    };
    filler
        .hds
        .get()
        .ok_or_else(super::super::missing_storage)?
        .store_interference(&interference, edge)
}

pub(super) fn on_endpoint(
    point: &crate::top_ope_brep_ds::TopOpeBRepDSPoint,
    edge: &Edge,
) -> Result<bool, Failure> {
    let edge_tolerance = BRepTool::edge_tolerance(edge)?;
    let tolerance = if point.tolerance() < edge_tolerance {
        edge_tolerance
    } else {
        point.tolerance()
    };
    let (first, last) = topexp::vertices(edge, false);
    Ok(point.point().is_equal(&BRepTool::pnt(&first)?, tolerance)
        || point.point().is_equal(&BRepTool::pnt(&last)?, tolerance))
}

fn inl_transition(
    point: &TopOpeBRepVPointInter<'_>,
    context: &Context,
    on_endpoint: bool,
) -> Result<Transition, Failure> {
    let tangent = fun_tool_tggeom_e_vec(context.current.parameter, &context.current.edge)?;
    let mut uv = Point2::default();
    let ok = if on_endpoint {
        uv = point.surface_parameters(context.other_side)?;
        true
    } else if let Some(other) = &context.other {
        fun_tool_paron_ef(&other.edge, other.parameter, &context.other_face, &mut uv)?
    } else {
        false
    };
    let mut normal = if ok {
        fun_tool_nggeom_f(uv, &context.other_face)?
    } else {
        Default::default()
    };
    if context.other_face.orientation() == Orientation::Reversed {
        normal.reverse();
    }
    let dot = if ok && tangent.magnitude() > 1e-7 && normal.magnitude() > 1e-7 {
        Some(Direction3::from_xyz(*tangent.xyz())?.dot(Direction3::from_xyz(*normal.xyz())?))
    } else {
        None
    };
    let apex = fun_tool_onapex(uv, &BRepTool::surface(&context.other_face)?)?;
    Ok(match dot {
        Some(dot) if !apex && dot.abs() > precision::angular() * 1e5 => {
            Transition::from_orientation(if dot < 0. {
                Orientation::Forward
            } else {
                Orientation::Reversed
            })
        }
        _ => Transition::default(),
    })
}

pub(super) fn process_inl(
    filler: &TopOpeBRepFacesFiller<'_>,
    point: &TopOpeBRepVPointInter<'_>,
    context: &mut Context,
    search: &GeometrySearch,
    on_endpoint: bool,
    transition: &mut Transition,
) -> Result<bool, Failure> {
    let computed = inl_transition(point, context, on_endpoint)?;
    if !on_endpoint || context.current.degenerate {
        if !computed.is_unknown() {
            transition.set_state_before(computed.before());
            transition.set_state_after(computed.after());
        }
        return Ok(false);
    }
    store_edge(
        filler,
        &EdgeInterferenceRequest {
            edge: &context.current.edge,
            transition: &computed,
            face_index: context.other_face_index,
            support: context.other_face_index,
            parameter: context.current.parameter,
            search,
            vertex: context.current.vertex,
            face_support: true,
        },
    )?;
    if let Some(other) = &mut context.other {
        other.ensure_index(filler, context.other_side)?;
        store_edge(
            filler,
            &EdgeInterferenceRequest {
                edge: &context.current.edge,
                transition: &computed,
                face_index: context.other_face_index,
                support: other.edge_index,
                parameter: context.current.parameter,
                search,
                vertex: context.current.vertex,
                face_support: false,
            },
        )?;
    }
    Ok(true)
}

pub(super) fn correct_on(
    filler: &TopOpeBRepFacesFiller<'_>,
    point: &TopOpeBRepVPointInter<'_>,
    context: &mut Context,
    search: &GeometrySearch,
) -> Result<bool, Failure> {
    let Some(other) = &mut context.other else {
        return Ok(false);
    };
    other.ensure_index(filler, context.other_side)?;
    let storage = filler.hds.get().ok_or_else(super::super::missing_storage)?;
    let (first, last, _) = {
        let ds = storage.ds();
        fds_lo_iinfsup(
            &ds,
            &other.edge,
            other.parameter,
            search.kind,
            search.index,
            ds.shape_interferences(&other.edge, true),
        )?
    };
    let uv = point.surface_parameters(context.side)?;
    // Source traversal never clears this flag when no edge on the face matches.
    let same_domain = storage.has_same_domain(&other.edge, true);
    let tangent = same_domain
        || fun_tool_etg_f(
            other.parameter,
            &other.edge,
            uv,
            &context.current.face,
            precision::angular() * 1e2,
        )?;
    let restriction =
        super::new_transition::contains_shape(&filler.restriction_edges, &context.current.edge);
    let interference_2d = tangent && (other.section || restriction);
    let mut maker = TopOpeBRepToolMakeTransition::new();
    if !maker.initialize(
        &other.edge,
        (first, last, other.parameter),
        &context.current.face,
        uv,
        0.01,
    )? || (interference_2d && !maker.is_t2d())
    {
        return Ok(false);
    }
    maker.set_rest(&context.current.edge, context.current.parameter)?;
    let (done, before, after) = maker.mk_ton_e()?;
    if !done {
        return Ok(false);
    }
    let transition = Transition::new(before, after, ShapeEnum::Face, ShapeEnum::Face);
    store_edge(
        filler,
        &EdgeInterferenceRequest {
            edge: &other.edge,
            transition: &transition,
            face_index: context.current.face_index,
            support: context.current.edge_index,
            parameter: other.parameter,
            search,
            vertex: other.vertex,
            face_support: false,
        },
    )?;
    if !filler.line_is_on_edge && !fds_has_same_domain3d(&storage.ds(), &other.edge, None) {
        store_edge(
            filler,
            &EdgeInterferenceRequest {
                edge: &other.edge,
                transition: &transition,
                face_index: context.current.face_index,
                support: context.current.face_index,
                parameter: other.parameter,
                search,
                vertex: other.vertex,
                face_support: true,
            },
        )?;
    }
    Ok(true)
}

pub(super) fn needs_on_correction(
    filler: &TopOpeBRepFacesFiller<'_>,
    context: &Context,
    point: &crate::top_ope_brep_ds::TopOpeBRepDSPoint,
) -> Result<bool, Failure> {
    if context.on_two || !context.on_other || context.other_degenerate() {
        return Ok(false);
    }
    let Some(other) = &context.other else {
        return Ok(false);
    };
    if other.edge_index == 0 {
        return Ok(true);
    }
    let storage = filler.hds.get().ok_or_else(super::super::missing_storage)?;
    let list = storage.ds().shape_interferences(&other.edge, true).clone();
    let candidates = selection::at_point(storage, &list, point)?;
    Ok(selection::on_transition_shape(&candidates, context.current.face_index)?.is_empty())
}

pub(super) fn should_add_face(
    filler: &TopOpeBRepFacesFiller<'_>,
    context: &Context,
    search: &GeometrySearch,
    transition: &Transition,
) -> Result<bool, Failure> {
    if !search.edge_found && !search.curve_found {
        return Ok(true);
    }
    let list = filler
        .hds
        .get()
        .ok_or_else(super::super::missing_storage)?
        .ds()
        .shape_interferences(&context.current.edge, true)
        .clone();
    let selected = selection::same_geometry_support(&list, search.index, context.other_face_index)?;
    if selected.is_empty() {
        return Ok(true);
    }
    let orientation = transition.orientation(State::In, ShapeEnum::Face);
    let mut same = false;
    let mut opposite = false;
    for entry in selected.iter() {
        let current = entry
            .get()
            .ok_or_else(super::super::missing_interference)?
            .transition()
            .orientation(State::In, ShapeEnum::Face);
        same |= current == orientation;
        opposite |= current == martensite_tkg3d::top_abs::complement(orientation);
        // Native selection stops at the first opposite, even if a same follows.
        if opposite && !same {
            return Ok(true);
        }
    }
    Ok(false)
}
