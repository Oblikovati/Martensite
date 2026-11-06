//! `tests/heal/unify_same_domain/end`, the history check every script of that grid inherits.
use super::{compound_exploded, compound_of, isdeleted, modified, savehistory};
use crate::support::{
    checkprops::{CHECKPROPS_EPSILON, Expected, mass_problems},
    commands::{bound_shape, explode},
    nbshapes::checknbshapes_like,
    one_step::bsection,
    properties::{Prop, printed_mass},
    session::Session,
};
use martensite_geometry_raster::property_report;
use martensite_tkbrep::{
    brep_tools::History,
    topods::{self, Shape},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn removed_parts(s: &dyn Session, history: &Handle<History>, names: &[String]) -> Vec<Shape> {
    let mut removed = Vec::new();
    for name in names {
        if isdeleted(s, history, name).contains("Deleted.") {
            removed.push(bound_shape(s, name));
        }
    }
    removed
}

/// `compound whole`, and `add` for each part: `explode` names the parts of `input`.
fn removed_compound(s: &mut dyn Session, history: &Handle<History>, input: &str, whole: &str) {
    let names = explode(s, input, None);
    let removed = removed_parts(s, history, &names);
    s.bind(whole, &compound_of(&removed));
}

/// `bsection name first second` and the script's `llength [explode name kind]`.
fn section_has_parts(
    s: &mut dyn Session,
    name: &str,
    operands: (&str, &str),
    kind: ShapeEnum,
) -> bool {
    bsection(s, name, operands.0, operands.1);
    !explode(s, name, Some(kind)).is_empty()
}

fn no_face_deleted(s: &mut dyn Session, history: &Handle<History>) {
    for name in explode(s, "a", Some(ShapeEnum::Face)) {
        if isdeleted(s, history, &name).contains("Deleted.") {
            println!("Error: the faces should not be removed during unification.");
            break;
        }
    }
}

/// One part of the script's loop: unless removed, what it became joins `all_hist_shapes`.
fn history_shapes_of(
    s: &mut dyn Session,
    history: &Handle<History>,
    name: &str,
    collected: &mut Vec<Shape>,
) {
    if isdeleted(s, history, name).contains("Deleted.") {
        return;
    }
    if modified(s, "s_mod", history, name).contains("has not been modified") {
        collected.push(bound_shape(s, name));
        return;
    }
    let became = bound_shape(s, "s_mod");
    if became.shape_type().unwrap() == ShapeEnum::Compound {
        explode(s, "s_mod", None);
        let children: Vec<Shape> = topods::ShapeIterator::new(&became, true, true).collect();
        collected.extend(children);
    } else {
        collected.push(became);
    }
}

fn collected_history_shapes(s: &mut dyn Session, history: &Handle<History>) {
    compound_exploded(s, "a", ShapeEnum::Face, "input_faces");
    let mut collected = Vec::new();
    for comp in ["input_verts", "input_edges", "input_faces"] {
        for name in explode(s, comp, None) {
            history_shapes_of(s, history, &name, &mut collected);
        }
    }
    s.bind("all_hist_shapes", &compound_of(&collected));
}

fn collected_result_shapes(s: &mut dyn Session) {
    compound_exploded(s, "result", ShapeEnum::Face, "res_faces");
    let mut collected = Vec::new();
    for comp in ["res_verts", "res_edges", "res_faces"] {
        explode(s, comp, None);
        let whole = bound_shape(s, comp);
        collected.extend(topods::ShapeIterator::new(&whole, true, true));
    }
    s.bind("all_res_shapes", &compound_of(&collected));
}

/// `lprops` or `sprops` with `-skip`: writes the report, returns the printed mass.
fn props_skipping(kind: Prop, shape: &Shape, epsilon: Option<f64>) -> f64 {
    let mut properties = GProps::default();
    let reached = match (kind, epsilon) {
        (Prop::Length, _) => {
            BRepGProp::linear_properties(shape, &mut properties, true, false).map(|()| None)
        }
        (_, Some(eps)) => {
            BRepGProp::surface_properties_eps(shape, &mut properties, eps, true).map(Some)
        }
        // Without an epsilon the command uses the non-adaptive rule, see `properties.rs`.
        (_, None) => {
            BRepGProp::surface_properties_eps(shape, &mut properties, 1.0, true).map(|_| None)
        }
    }
    .unwrap_or_else(|failure| panic!("{} -skip: {failure:?}", kind.word()));
    let principal = properties.principal_properties().unwrap();
    print!(
        "{}",
        property_report(&properties, &principal, reached, false).unwrap()
    );
    printed_mass(properties.mass())
}

/// `checkprops all -s [sprops result -skip] -l [lprops result -skip] -skip`.
fn check_properties_against_result(s: &dyn Session) {
    let result = bound_shape(s, "result");
    let area = props_skipping(Prop::Area, &result, None);
    let length = props_skipping(Prop::Length, &result, None);
    println!("checkprops all_hist_shapes -s {area} -l {length} -skip");
    let all = bound_shape(s, "all_hist_shapes");
    let epsilon = Some(CHECKPROPS_EPSILON);
    for (kind, wanted) in [(Prop::Length, length), (Prop::Area, area)] {
        let mass = props_skipping(kind, &all, epsilon);
        for problem in mass_problems(kind, mass, Expected::Mass(wanted), 0.01) {
            println!("{problem}");
        }
    }
}

/// `tests/heal/unify_same_domain/end`: what the history says was removed or became of each part
/// must be what the result holds. Example: `unify_same_domain_end(session)`.
pub fn unify_same_domain_end(s: &mut dyn Session) {
    if s.shape("result").is_none() {
        return;
    }
    compound_exploded(s, "result", ShapeEnum::Vertex, "res_verts");
    compound_exploded(s, "result", ShapeEnum::Edge, "res_edges");
    compound_exploded(s, "a", ShapeEnum::Vertex, "input_verts");
    compound_exploded(s, "a", ShapeEnum::Edge, "input_edges");
    let history = savehistory();
    removed_compound(s, &history, "input_verts", "rem_verts");
    removed_compound(s, &history, "input_edges", "rem_edges");
    let operands = ("res_verts", "rem_verts");
    if section_has_parts(s, "sec_v", operands, ShapeEnum::Vertex) {
        println!("Error: the history of the removed vertices is incorrect.");
    }
    let operands = ("res_edges", "rem_edges");
    if section_has_parts(s, "sec_e", operands, ShapeEnum::Edge) {
        println!("Error: the history of the removed edges is incorrect.");
    }
    no_face_deleted(s, &history);
    collected_history_shapes(s, &history);
    collected_result_shapes(s);
    checknbshapes_like(s, "all_hist_shapes", "all_res_shapes");
    check_properties_against_result(s);
}
