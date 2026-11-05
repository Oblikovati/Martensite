//! No surrogate geometry replaces the original face with its genuinely null surface.
mod commands;
#[cfg(test)]
mod cpu;
use crate::{
    dataset_shape::read_dataset_shape,
    property_queries::{Kind, properties},
    session_run::{SessionRun, banner},
};
use martensite_tkg3d::top_abs::ShapeEnum;
type Outcome = Result<(), Box<dyn std::error::Error>>;

/// Query and display the original invalid-surface input without a crash; `null_surface_display()?`.
pub(crate) fn null_surface_display() -> Outcome {
    banner(&[
        "========",
        "AisShape - displaying a topological face with NULL surface (test case checks there is no crash)",
        "========",
    ]);
    let shape = read_dataset_shape("bug27821_nullsurf.brep")?;
    let faces = crate::boolean_typed_explode::all_distinct(&shape, ShapeEnum::Face);
    let _surface = properties(&shape, Kind::Area, false)?;
    let _volume = properties(&shape, Kind::Volume, false)?;
    let mut run = SessionRun::new()?;
    run.session().set_axonometric()?;
    commands::presentation(run.session(), &shape, &faces)?;
    run.session().fit()?;
    run.session().select_point(250, 250)?;
    run.dump("bug27821.png")?;
    run.finish()
}
