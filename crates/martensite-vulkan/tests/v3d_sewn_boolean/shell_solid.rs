//! SWDRAW ssolid orients the shell by the actual infinite-point classification.
use super::*;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Shell, Solid},
};
use martensite_tkg3d::top_abs::State;
use martensite_tktopalgo::{brep_builder_api, brep_class3d::BRepClass3dSolidClassifier};

fn with_shell(shell: &Shell) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    let mut shape = solid.into_shape();
    builder.add(&mut shape, shell.as_ref())?;
    Ok(shape)
}

pub(super) fn solid_from_shell(mut shell: Shell) -> Result<Shape, Failure> {
    if !shell.free()? {
        println!("Shape non Free -> Freeing");
        shell.set_free(true)?;
    }
    let solid = with_shell(&shell)?;
    let mut classifier = BRepClass3dSolidClassifier::with_shape(&solid)?;
    classifier.perform_infinite_point(brep_builder_api::precision())?;
    if classifier.state() != State::In {
        return Ok(solid);
    }
    println!("NB : Shell to be reversed");
    shell.reverse();
    with_shell(&shell)
}
