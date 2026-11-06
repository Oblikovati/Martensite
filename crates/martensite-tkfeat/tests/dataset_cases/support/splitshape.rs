//! `splitshape result shape [splitedges] [face wire/edge/compound ...] [@ edgeonshape edgeonwire ...]`.
use super::{
    feat_script::{FeatScript, checked, command_failed},
    session::Session,
};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{self, Shape, Shell},
};
use martensite_tkernel::{message::Message, ncollection::Sequence};
use martensite_tkfeat::brep_feat::BRepFeatSplitShape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::fmt::Write as _;

fn failed_with_message(text: &str) -> ! {
    let _ = write!(Message::send_fail(), "{text}");
    command_failed("");
}

fn append_edges(shape: &Shape, into: &mut Sequence<Shape>) {
    for edge in checked(Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Face)) {
        into.append(edge);
    }
}

fn bound_or_refused(s: &FeatScript<'_>, name: &str) -> Shape {
    let shape = s.shape_or_null(name);
    if shape.is_null() {
        failed_with_message(&format!("Invalid input shape {name}"));
    }
    shape
}

/// Adds the wires, edges and compounds of `args` to `face`, as the command walks them.
fn add_to_face(
    split: &mut BRepFeatSplitShape,
    s: &FeatScript<'_>,
    args: &[&str],
    face: Shape,
) -> Shape {
    let mut current = face;
    for name in args {
        let tool = s.shape_or_null(name);
        if tool.is_null() {
            command_failed("");
        }
        let kind = checked(tool.shape_type());
        if !matches!(
            kind,
            ShapeEnum::Wire | ShapeEnum::Edge | ShapeEnum::Compound
        ) {
            current = tool;
            continue;
        }
        let on = checked(topods::face(&current));
        match kind {
            ShapeEnum::Wire => checked(split.add_wire(&checked(topods::wire(&tool)), &on)),
            ShapeEnum::Edge => checked(split.add_edge(&checked(topods::edge(&tool)), &on)),
            _ => checked(split.add_compound(&checked(topods::compound(&tool)), &on)),
        }
    }
    current
}

/// `splitshape result shape first ... [@ edgeonshape edgeonwire ...]`: `first` is a face followed
/// by what splits it, or the splitting edges themselves.
/// Example: `splitshape(s, "result", "f", &["f", "e"], &[])`.
pub fn splitshape(
    s: &mut FeatScript<'_>,
    result: &str,
    shape: &str,
    args: &[&str],
    on_edges: &[(&str, &str)],
) {
    let base = bound_or_refused(s, shape);
    let mut split = BRepFeatSplitShape::with_shape(&base);
    let Some((first, rest)) = args.split_first() else {
        command_failed("");
    };
    let leading = bound_or_refused(s, first);
    let kind = checked(leading.shape_type());
    if kind == ShapeEnum::Face {
        add_to_face(&mut split, s, rest, leading);
    } else {
        let mut edges = Sequence::<Shape>::default();
        append_edges(&leading, &mut edges);
        for name in rest {
            append_edges(&bound_or_refused(s, name), &mut edges);
        }
        checked(split.add_edges(&edges));
    }
    for (on_shape, on_wire) in on_edges {
        let on_shape = checked(topods::edge(&s.shape_or_null(on_shape)));
        let on_wire = checked(topods::edge(&s.shape_or_null(on_wire)));
        checked(split.add_edge_on_edge(&on_wire, &on_shape));
    }
    let shape = checked(split.shape()).clone();
    s.bind(result, &shape);
}

/// `BUC60854 result shape face tools... @ edgeonshape edgeonwire ... # L|R`: the split's left or
/// right faces, as a shell. Example: `buc60854(s, "result", "b", "f", &["e"], &[("x", "y")], true)`.
pub fn buc60854(
    s: &mut FeatScript<'_>,
    result: &str,
    shape: &str,
    face: &str,
    tools: &[&str],
    on_edges: &[(&str, &str)],
    left: bool,
) {
    let mut split = BRepFeatSplitShape::with_shape(&s.shape_or_null(shape));
    let on = checked(topods::face(&s.shape_or_null(face)));
    for name in tools {
        let tool = s.shape_or_null(name);
        match checked(tool.shape_type()) {
            ShapeEnum::Edge => checked(split.add_edge(&checked(topods::edge(&tool)), &on)),
            ShapeEnum::Wire => checked(split.add_wire(&checked(topods::wire(&tool)), &on)),
            _ => command_failed(""),
        }
    }
    for (on_shape, on_wire) in on_edges {
        let on_shape = checked(topods::edge(&s.shape_or_null(on_shape)));
        let on_wire = checked(topods::edge(&s.shape_or_null(on_wire)));
        checked(split.add_edge_on_edge(&on_wire, &on_shape));
    }
    let _ = checked(split.shape());
    let parts = if left {
        checked(split.left()).clone()
    } else {
        checked(split.right()).clone()
    };
    let mut shell = Shell::default();
    let builder = BRepBuilder::default();
    builder.make_shell(&mut shell);
    for part in parts.iter() {
        checked(builder.add(&mut shell, part));
    }
    s.bind(result, &shell.into_shape());
}
