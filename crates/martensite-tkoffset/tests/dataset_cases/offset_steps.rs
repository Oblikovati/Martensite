//! The offset commands, on the library's builders with the options the commands pass.
use crate::script::{OffsetScript, command_stopped};
use crate::steps::{Offset, OffsetShape, PlanarOffset};
use martensite_tkbrep::{
    brep_tools::{History, HistoryAlgorithm},
    topexp::Explorer,
    topods::{self, Shape},
};
use martensite_tkernel::{
    message::ProgressRange,
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::{
    brep_offset::{
        BRepOffsetError, BRepOffsetMakeOffset, BRepOffsetMakeSimpleOffset, BRepOffsetMode,
        MakeOffsetOptions,
    },
    brep_offset_api::BRepOffsetAPIMakeOffset,
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Runs one offset command; `apply(script, &Offset::perform("result"))`.
pub fn apply(script: &mut OffsetScript<'_>, step: &Offset) -> Result<(), Failure> {
    match step {
        Offset::Parameter(tolerance, complete, join, remove) => {
            set_parameters(script, *tolerance, *complete, *join, *remove);
            Ok(())
        }
        Offset::Load(shape, value, faces) => load(script, shape, *value, faces),
        Offset::OnFace(pairs) => on_face(script, pairs),
        Offset::Perform(result) => perform(script, result),
        Offset::Shape(command) => offset_shape(script, command),
        Offset::Thick(result, shape, value, join, tolerance) => {
            thick(script, (result, shape), *value, *join, *tolerance)
        }
        Offset::Planar(command) => planar(script, command),
        Offset::Simple(result, shape, value, tolerance) => {
            simple(script, (result, shape), *value, *tolerance)
        }
    }
}

fn join_of(word: char) -> Option<JoinType> {
    match word {
        'a' => Some(JoinType::Arc),
        'i' => Some(JoinType::Intersection),
        't' => Some(JoinType::Tangent),
        _ => None,
    }
}

fn skin(complete: bool, join: JoinType, thickening: bool, remove: bool) -> MakeOffsetOptions {
    MakeOffsetOptions {
        mode: BRepOffsetMode::Skin,
        intersection: complete,
        self_inter: false,
        join,
        thickening,
        remove_int_edges: remove,
    }
}

fn set_parameters(
    script: &mut OffsetScript<'_>,
    tolerance: f64,
    complete: bool,
    join: char,
    remove: bool,
) {
    let state = &mut script.commands;
    state.tolerance = tolerance;
    state.complete = complete;
    if let Some(join) = join_of(join) {
        state.join = join;
    }
    state.remove_internal_edges = remove;
}

/// `offsetload shape value faces...`; a word that is no face adds no cap but still counts.
fn load(
    script: &mut OffsetScript<'_>,
    shape: &str,
    value: f64,
    faces: &[&str],
) -> Result<(), Failure> {
    let source = script.required(shape)?;
    let chosen = named_faces(script, faces);
    let state = &mut script.commands;
    let options = skin(
        state.complete,
        state.join,
        false,
        state.remove_internal_edges,
    );
    state
        .offset
        .initialize(&source, value, state.tolerance, options);
    for face in &chosen {
        state.offset.add_face(&topods::face(face)?);
    }
    state.has_caps = !faces.is_empty();
    Ok(())
}

/// The named variables that hold a face.
fn named_faces(script: &OffsetScript<'_>, names: &[&str]) -> Vec<Shape> {
    names
        .iter()
        .map(|name| script.shape(name))
        .filter(|shape| !shape.is_null() && shape.shape_type().ok() == Some(ShapeEnum::Face))
        .collect()
}

fn on_face(script: &mut OffsetScript<'_>, pairs: &[(&str, f64)]) -> Result<(), Failure> {
    for (name, value) in pairs {
        let face = script.shape(name);
        if !face.is_null() && face.shape_type().ok() == Some(ShapeEnum::Face) {
            let face = topods::face(&face)?;
            script.commands.offset.set_offset_on_face(&face, *value);
        }
    }
    Ok(())
}

fn perform(script: &mut OffsetScript<'_>, result: &'static str) -> Result<(), Failure> {
    let caps = script.commands.has_caps;
    let offset = &mut script.commands.offset;
    if caps {
        offset.make_thick_solid(&ProgressRange::default())?;
    } else {
        offset.make_offset_shape(&ProgressRange::default())?;
    }
    if offset.is_done() {
        let shape = offset.shape().clone();
        script.set_shape(result, shape);
    } else {
        println!("{}", offset_state_text(offset.error()));
    }
    let offset = &mut script.commands.offset;
    let init = offset.init_shape().clone();
    let history = History::from_algorithm(&[init], &mut OffsetHistory(offset))?;
    script.last_history = Some(Handle::new(history));
    Ok(())
}

/// The history of a `BRepOffsetMakeOffset`.
struct OffsetHistory<'a>(&'a mut BRepOffsetMakeOffset);

impl HistoryAlgorithm for OffsetHistory<'_> {
    fn is_deleted(&mut self, shape: &Shape) -> Result<bool, Failure> {
        self.0.is_deleted(shape)
    }

    fn modified(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.modified(shape)?.iter().cloned().collect())
    }

    fn generated(&mut self, shape: &Shape) -> Result<Vec<Shape>, Failure> {
        Ok(self.0.generated(shape)?.iter().cloned().collect())
    }
}

fn offset_state_text(error: BRepOffsetError) -> &'static str {
    match error {
        BRepOffsetError::NoError => "OK. Offset performed successfully.",
        BRepOffsetError::BadNormalsOnGeometry => "ERROR. Degenerated normal on input data.",
        BRepOffsetError::C0Geometry => "ERROR. C0 continuity of input data.",
        BRepOffsetError::NullOffset => "ERROR. Null offset of all faces.",
        BRepOffsetError::NotConnectedShell => {
            "ERROR. Incorrect set of faces to remove, the remaining shell is not connected."
        }
        BRepOffsetError::CannotTrimEdges => "ERROR. Can not trim edges.",
        BRepOffsetError::CannotFuseVertices => "ERROR. Can not fuse vertices.",
        BRepOffsetError::CannotExtentEdge => "ERROR. Can not extent edge.",
        BRepOffsetError::MixedConnectivity => "ERROR. Mixed connectivity of faces.",
        _ => "ERROR. offsetperform operation not done.",
    }
}

fn offset_shape(script: &mut OffsetScript<'_>, command: &OffsetShape) -> Result<(), Failure> {
    let source = script.required(command.shape)?;
    let tolerance = command.tolerance.unwrap_or_else(precision::confusion);
    let chosen = named_faces(script, command.faces);
    let mut builder = BRepOffsetMakeOffset::new();
    let options = skin(command.complete, JoinType::Arc, false, false);
    builder.initialize(&source, command.value, tolerance, options);
    for face in &chosen {
        builder.add_face(&topods::face(face)?);
    }
    if chosen.is_empty() {
        builder.make_offset_shape(&ProgressRange::default())?;
    } else {
        builder.make_thick_solid(&ProgressRange::default())?;
    }
    println!("{}", offset_state_text(builder.error()));
    if !builder.shape().is_null() {
        script.set_shape(command.result, builder.shape().clone());
    }
    Ok(())
}

fn thick(
    script: &mut OffsetScript<'_>,
    (result, shape): (&'static str, &str),
    value: f64,
    join: char,
    tolerance: f64,
) -> Result<(), Failure> {
    let source = script.required(shape)?;
    let join = join_of(join).unwrap_or(JoinType::Arc);
    let mut builder = BRepOffsetMakeOffset::new();
    builder.initialize(&source, value, tolerance, skin(false, join, true, false));
    builder.make_offset_shape(&ProgressRange::default())?;
    println!("{}", offset_state_text(builder.error()));
    if !builder.shape().is_null() {
        script.set_shape(result, builder.shape().clone());
    }
    Ok(())
}

fn simple(
    script: &mut OffsetScript<'_>,
    (result, shape): (&'static str, &str),
    value: f64,
    tolerance: f64,
) -> Result<(), Failure> {
    let source = script.required(shape)?;
    let mut builder = BRepOffsetMakeSimpleOffset::with_shape(&source, value);
    builder.set_tolerance(tolerance);
    builder.set_build_solid_flag(false);
    builder.perform()?;
    if !builder.is_done() {
        println!("ERROR:{}", builder.error_message());
        return Ok(());
    }
    script.set_shape(result, builder.result_shape().clone());
    Ok(())
}

/// `mkoffset` and `openoffset`.
fn planar(script: &mut OffsetScript<'_>, command: &PlanarOffset) -> Result<(), Failure> {
    let mut offset = BRepOffsetAPIMakeOffset::new();
    let join = if command.join == 'i' {
        JoinType::Intersection
    } else {
        JoinType::Arc
    };
    let base = script.shape(command.base);
    if base.is_null() {
        return Err(command_stopped(&format!(
            "mkoffset: {} is not a shape",
            command.base
        )));
    }
    if base.shape_type()? == ShapeEnum::Face {
        let face = topods::face(&base.oriented(Orientation::Forward))?;
        offset.init_face(&face, join, command.open)?;
    } else {
        offset.init(join, command.open);
        add_wires(&mut offset, &base)?;
    }
    offset.set_approx(command.approx);
    for index in 1..=command.count {
        offset.perform(f64::from(index) * command.step, command.altitude)?;
        if !offset.is_done() {
            return Err(command_stopped(" Error: Offset is not done."));
        }
        let shape = offset.shape()?.clone();
        script.set_shape_owned(&format!("{}_{index}", command.result), shape);
    }
    Ok(())
}

fn add_wires(offset: &mut BRepOffsetAPIMakeOffset, base: &Shape) -> Result<(), Failure> {
    let mut explorer = Explorer::new(base, ShapeEnum::Wire, ShapeEnum::Shape)?;
    while explorer.more() {
        offset.add_wire(&topods::wire(explorer.current())?);
        explorer.next()?;
    }
    Ok(())
}
