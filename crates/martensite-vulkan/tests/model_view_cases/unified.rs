//! Same-domain unification of restored faces and wires: sub-shape counts of the result and of
//! the faces it replaces, each shown shaded and dumped (OCC26489).
use super::prelude::*;
use crate::boolean_typed_explode::all_distinct;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::Compound;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeUnifySameDomain;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

/// The input of one unification: a restored shape, its first wire, or the face its wire
/// bounds (`mkplane`).
#[derive(Clone, Copy, Debug)]
pub(crate) enum UnifyInput {
    Restored(&'static str),
    FirstWire(&'static str),
    PlaneOfWire(&'static str),
}

const BANNER: [&str; 4] = ["============", "OCC26489", "============", ""];

fn input_shape(input: UnifyInput) -> Result<Shape, Box<dyn Error>> {
    Ok(match input {
        UnifyInput::Restored(name) => read_dataset_shape(name)?,
        UnifyInput::FirstWire(name) => {
            all_distinct(&read_dataset_shape(name)?, ShapeEnum::Wire)[0].clone()
        }
        UnifyInput::PlaneOfWire(name) => {
            let wire = martensite_tkbrep::topods::wire(&read_dataset_shape(name)?)?;
            BRepBuilderAPIMakeFace::from_wire(&wire, false)?
                .face()?
                .into_shape()
        }
    })
}

/// `unifysamedom` with its defaults: edges and faces unified, no B-spline concatenation.
fn unified(shape: &Shape) -> Result<ShapeUpgradeUnifySameDomain, Box<dyn Error>> {
    let unifier = ShapeUpgradeUnifySameDomain::new(shape, true, true, false);
    unifier.build()?;
    Ok(unifier)
}

/// `bug26489_1/2/3/5/6`: the unified input has the expected sub-shape counts and is shown.
pub(crate) fn unified_view(
    input: UnifyInput,
    counts: [usize; 9],
    image: &'static str,
) -> Result<(), Box<dyn Error>> {
    banner(&BANNER);
    let result = unified(&input_shape(input)?)?.shape();
    check_sub_shape_counts(&result, &counts)?;
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    run.session().clear()?;
    show(&mut run, "result", &result)?;
    run.session().fit()?;
    run.dump(image)?;
    run.finish()
}

/// `modified`: the shapes the history derives from `initial`, one shape alone or a compound.
fn modified_shape(
    unifier: &ShapeUpgradeUnifySameDomain,
    initial: &Shape,
) -> Result<Shape, Box<dyn Error>> {
    let modified = unifier.history().modified(initial)?;
    if let [single] = modified.as_slice() {
        return Ok(single.clone());
    }
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let mut result = compound.into_shape();
    for shape in &modified {
        builder.add(&mut result, shape)?;
    }
    Ok(result)
}

/// `bug26489_4`: each of the first three faces is modified into one face with four edges.
pub(crate) fn unified_face_history() -> Result<(), Box<dyn Error>> {
    banner(&BANNER);
    let b1 = read_dataset_shape("bug26489_r002.brep")?;
    let unifier = unified(&b1)?;
    let faces = all_distinct(&b1, ShapeEnum::Face);
    let mut run = SessionRun::new()?;
    run.session().set_display_mode(1)?;
    let mut modified = Vec::new();
    for (face, image) in
        faces
            .iter()
            .zip(["bug26489_4_1.png", "bug26489_4_2.png", "bug26489_4_3.png"])
    {
        let shape = modified_shape(&unifier, face)?;
        run.session().clear()?;
        show(&mut run, "x", &shape)?;
        run.session().fit()?;
        run.dump(image)?;
        modified.push(shape);
    }
    for shape in &modified {
        check_sub_shape_counts(shape, &[4, 4, 1, 1, 0, 0, 0, 0, 10])?;
    }
    run.finish()
}
