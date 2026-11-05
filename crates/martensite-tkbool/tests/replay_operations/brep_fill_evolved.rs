//! The Rust twin of the recorded capture.
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::evolved_digest::evolved_digest as shape;
use super::shapes::tp_shapes;
use martensite_tkbool::brep_fill::BRepFillEvolved;
use martensite_tkbrep::{
    topexp,
    topods::{self, Shape, Wire},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::{geom_abs::JoinType, gp::Axis3};
use martensite_tktopalgo::brep_lib::BRepLibMakeFace;

fn evolved_result(output: &BufferedTokens, evolved: &BRepFillEvolved) {
    output.flag(evolved.is_done());
    output.attempt(|| shape(output, evolved.shape()));
    output.attempt(|| shape(output, evolved.top()));
    output.attempt(|| shape(output, evolved.bottom()));
}

fn evolved_history(
    output: &BufferedTokens,
    evolved: &BRepFillEvolved,
    spine: &Shape,
    profile: &Shape,
) -> Result<(), Failure> {
    let (mut spines, mut profiles) = (IndexedMapOfShape::default(), IndexedMapOfShape::default());
    topexp::map_shapes(spine, &mut spines, true, true);
    topexp::map_shapes(profile, &mut profiles, true, true);
    for s in spines.iter() {
        for p in profiles.iter() {
            let generated = evolved.generated_shapes(s, p);
            output.int(generated.len() as i32);
            for result in generated.iter() {
                output.int(result.shape_type()? as i32);
                output.int(result.orientation() as i32);
            }
        }
    }
    Ok(())
}

fn perform_evolved(
    spine: &Wire,
    profile: &Wire,
    face: bool,
    solid: bool,
    constructor: bool,
    join: JoinType,
    axes: &Axis3,
) -> Result<BRepFillEvolved, Failure> {
    let mut evolved = BRepFillEvolved::new();
    if face {
        let spine = BRepLibMakeFace::from_wire(spine, true)?.face()?;
        if constructor {
            return BRepFillEvolved::from_face(&spine, profile, axes, join, solid);
        }
        evolved.perform_face(&spine, profile, axes, join, solid)?;
    } else {
        if constructor {
            return BRepFillEvolved::from_wire(spine, profile, axes, join, solid);
        }
        evolved.perform_wire(spine, profile, axes, join, solid)?;
    }
    Ok(evolved)
}

/// Compare defined default-state queries; `evolved_empty_case(&[])`.
pub fn evolved_empty_case(output: &BufferedTokens, _: &[f64]) {
    let evolved = BRepFillEvolved::new();
    evolved_result(output, &evolved);
    output.int(
        evolved
            .generated_shapes(&Shape::default(), &Shape::default())
            .len() as i32,
    );
    output.end();
}

fn evolved_record(output: &BufferedTokens, r: &mut Reader<'_>) -> Result<(), Failure> {
    let table = tp_shapes(r)?;
    let (spine, profile) = (
        topods::wire(&at(&table, r.integer())?)?,
        topods::wire(&at(&table, r.integer())?)?,
    );
    let (face, solid, constructor) = (r.flag(), r.flag(), r.flag());
    let join = match r.integer() {
        0 => JoinType::Arc,
        1 => JoinType::Tangent,
        _ => JoinType::Intersection,
    };
    let axes = Axis3::new(r.pnt(), r.dir(), r.dir())?;
    let evolved = perform_evolved(&spine, &profile, face, solid, constructor, join, &axes)?;
    evolved_result(output, &evolved);
    output.int(evolved.join_type() as i32);
    evolved_history(output, &evolved, spine.as_ref(), profile.as_ref())
}

/// Compare one evolved sweep; `evolved_case(&request_numbers)`.
pub fn evolved_case(output: &BufferedTokens, values: &[f64]) {
    if let Err(error) = evolved_record(output, &mut Reader::new(values)) {
        return failed(output, error);
    }
    output.end();
}
