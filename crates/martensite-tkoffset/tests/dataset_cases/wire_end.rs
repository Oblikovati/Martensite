//! The `mkoffset` branch of `offset/end`: it offsets the input by a fraction of its length, then
//! checks every wire of the result.
use crate::groups::{WireEnd, expect_length, view_when_massive};
use crate::script::OffsetScript;
use crate::steps::{Offset, PlanarOffset};
use crate::support::{
    checkshape::{checkshape, checkshape_text_quiet},
    properties::{Prop, printed_mass, props},
    section_check::checksection_count,
    tolerance_check::tolerance_report,
};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::storage::format_general;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

/// `explode name kind`: binds the parts and returns each with its name;
/// `exploded(script, "result", ShapeEnum::face)`.
pub fn exploded(
    script: &mut OffsetScript<'_>,
    name: &str,
    kind: ShapeEnum,
) -> Vec<(String, Shape)> {
    let shape = script.shape(name);
    let parts = crate::input_shapes::distinct_parts(&shape, kind)
        .unwrap_or_else(|failure| panic!("explode {name}: {failure:?}"));
    script.set_parts(name, parts.clone());
    let named = parts.into_iter().enumerate();
    named
        .map(|(index, part)| (format!("{name}_{}", index + 1), part))
        .collect()
}

/// A double as a script string shows it: the shortest text that reads back, in `%g`'s form.
fn tcl_double(value: f64) -> String {
    let scientific = format!("{value:e}");
    let (mantissa, exponent) = scientific.split_once('e').unwrap();
    let exponent: i32 = exponent.parse().unwrap();
    if (-4..17).contains(&exponent) {
        return format!("{value}");
    }
    format!("{mantissa}e{exponent:+03}")
}

fn parsed_tolerance(text: &str) -> f64 {
    text.parse()
        .unwrap_or_else(|_| panic!("tolerance {text:?}: expected a number"))
}

/// Runs `mkoffset result s 1 value`; true when the command neither failed nor stopped.
fn offset_input(script: &mut OffsetScript<'_>, value: f64) -> bool {
    let command = Offset::Planar(PlanarOffset {
        result: "result",
        base: "s",
        count: 1,
        step: value,
        approx: false,
        join: 'a',
        altitude: 0.,
        open: false,
    });
    match crate::offset_steps::apply(script, &command) {
        Ok(()) => true,
        Err(failure) => {
            println!("{}", String::from_utf8_lossy(failure.what()));
            false
        }
    }
}

/// `offset/end` for `set command mkoffset`; `wire_end(script, &end)`.
pub fn wire_end(script: &mut OffsetScript<'_>, end: &WireEnd) {
    let source = script.shape("s");
    let length = printed_mass(props(Prop::Length, &source, None));
    let source_tolerance = parsed_tolerance(&tolerance_report(script.session(), "s"));
    println!(
        "Length of shape a is equal to {}",
        format_general(length, 15)
    );
    let value = length * end.off_param;
    println!("Considered Offset value is {} ", tcl_double(value));
    if offset_input(script, value) {
        let result = script.shape("result_1");
        script.set_shape("result", result.clone());
        let report = checkshape(&result);
        println!("{}", report.text);
    }
    expect_length(script, end.length);
    if !script.is_bound("result") {
        println!("Error : The offset cannot be built.");
        return;
    }
    let result = script.shape("result");
    let mass = printed_mass(props(Prop::Length, &result, None));
    wire_checks(script, end, value, source_tolerance);
    view_when_massive(script, mass);
}

fn wire_checks(script: &mut OffsetScript<'_>, end: &WireEnd, value: f64, source_tolerance: f64) {
    let vertices = exploded(script, "result", ShapeEnum::Vertex).len();
    let edges = exploded(script, "result", ShapeEnum::Edge).len();
    let mut wires = exploded(script, "result", ShapeEnum::Wire);
    if wires.is_empty() {
        wires.push(("result".to_owned(), script.shape("result")));
    }
    if vertices == 0 {
        empty_result(script, value);
    } else {
        println!("The resulting shape contains {} wires", wires.len());
        for (name, wire) in &wires {
            one_wire(script, name, wire, source_tolerance);
        }
    }
    count_line("vertexes", vertices, end.vertices);
    count_line("edges", edges, end.edges);
    count_line("wires", wires.len(), end.wires);
}

fn empty_result(script: &mut OffsetScript<'_>, value: f64) {
    let word = if value < 0. { "Warning" } else { "Error" };
    println!("{word} : The resulting shape is an empty COMPOUND");
    println!("{}", whatis_line(&script.shape("result")));
    println!();
    // `puts [checksection result]`: the command's report, then the same text printed again.
    let alone = checksection_count(script.session(), "result");
    println!(" nb alone Vertices : {alone}");
}

/// `whatis result`: its kind, its orientation and the flags it has set.
fn whatis_line(shape: &Shape) -> String {
    let kind = format!("{:?}", shape.shape_type().unwrap()).to_uppercase();
    let orientation = format!("{:?}", shape.orientation()).to_uppercase();
    let mut line = format!("result is a shape {kind} {orientation}");
    let flags = shape.tshape().get().map_or(Vec::new(), |tshape| {
        [
            (tshape.free(), " Free"),
            (tshape.modified(), " Modified"),
            (tshape.checked(), " Checked"),
            (tshape.orientable(), " Orientable"),
            (tshape.closed(), " Closed"),
            (tshape.infinite(), " Infinite"),
            (tshape.convex(), " Convex"),
            (tshape.locked(), " Locked"),
        ]
        .into_iter()
        .filter(|(set, _)| *set)
        .map(|(_, word)| word)
        .collect()
    });
    for flag in flags {
        line.push_str(flag);
    }
    line
}

fn one_wire(script: &mut OffsetScript<'_>, name: &str, wire: &Shape, source_tolerance: f64) {
    println!("Info for {name}:");
    let tolerance = parsed_tolerance(&tolerance_report(script.session(), name));
    if tolerance > source_tolerance {
        println!("Error : big tolerance of shape {name}");
    }
    if checksection_count(script.session(), name) != 0 {
        println!("Error : {name} is NOT a closed wire");
    }
    if plane_is_faulty(wire) {
        println!("Shape {name} has self-intersection");
    }
}

/// `mkplane res_plane wire`, then `checkshape res_plane`, whose report the script reads.
fn plane_is_faulty(wire: &Shape) -> bool {
    let Ok(typed) = topods::wire(wire) else {
        return false;
    };
    let Ok(maker) = BRepBuilderAPIMakeFace::from_wire(&typed, false) else {
        return false;
    };
    let Ok(face) = maker.face() else {
        return false;
    };
    let report = checkshape_text_quiet(face.as_ref());
    println!("{}", report.text);
    report.nb_faulty > 0
}

fn count_line(word: &str, found: usize, expected: Option<usize>) {
    let Some(expected) = expected else {
        return;
    };
    if found != expected || (expected == 0 && found != 0) {
        println!(
            "Error : The resulting shape is WRONG because it must contain {expected} {word} instead of {found}"
        );
    } else {
        println!("The resulting shape contains {found} {word}");
    }
}
