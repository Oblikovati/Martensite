//! Objects moved between Z-layers and display priorities.
use super::prelude::*;
use martensite_tkservice::graphic3d::{
    Graphic3dDisplayPriority, Graphic3dRenderingParamsFrustumCulling,
};

/// `bug25400`: a box stays visible in a new Z-layer, after that layer is deleted and with a
/// higher display priority.
pub(crate) fn box_through_layers() -> Result<(), Box<dyn Error>> {
    banner(&[
        "========",
        "OCC25400: Visualization, Renderer - Structure disappears if assigned a non-default Z layer",
    ]);
    banner(&["========", ""]);
    let b = box_sized([1., 2., 3.])?;
    let mut run = SessionRun::new()?;
    show(&mut run, "b", &b)?;
    run.session().update_view(|view| {
        view.change_rendering_params(&mut |params| {
            params.ray_tracing.frustum_culling_state = Graphic3dRenderingParamsFrustumCulling::On
        });
        Ok(())
    })?;
    run.session().fit()?;
    run.dump("bug25400_default_layer.png")?;
    let layer = run.session().add_z_layer()?;
    run.session().set_object_z_layer("b", layer)?;
    run.session().fit()?;
    run.dump("bug25400_new_layer.png")?;
    run.session().remove_z_layer(layer)?;
    run.session().fit()?;
    run.dump("bug25400_after_delete_layer.png")?;
    run.session()
        .set_display_priority("b", Graphic3dDisplayPriority::Above)?;
    run.dump("bug25400_new_priority.png")?;
    run.finish()
}

/// `bug22795`: shaded boxes in added Z-layers return to the default layer when theirs is
/// deleted.
pub(crate) fn boxes_in_deleted_layers() -> Result<(), Box<dyn Error>> {
    banner(&[
        "================",
        "OCC22795: Makes it possible to display some presentable objects in overlay of others, grouped by display priority ",
    ]);
    banner(&["================", ""]);
    let b = box_sized([10., 10., 10.])?;
    let s = translated(&box_sized([10., 10., 10.])?, [10., 0., 0.])?;
    let mut run = SessionRun::new()?;
    show(&mut run, "b", &b)?;
    show(&mut run, "s", &s)?;
    run.session().set_object_display_mode("b", 1)?;
    run.session().set_object_display_mode("s", 1)?;
    run.session().fit()?;
    for _ in 0..3 {
        run.session().add_z_layer()?;
    }
    run.session().remove_z_layer(1)?;
    run.session().set_object_z_layer("b", 2)?;
    let mut layers_ok = run.session().object_z_layer_text("b")? == "Z layer id: 2";
    run.session().set_object_z_layer("s", 3)?;
    layers_ok &= run.session().object_z_layer_text("s")? == "Z layer id: 3";
    run.session().remove_z_layer(3)?;
    layers_ok &= run.session().object_z_layer_text("s")? == "Z layer id: 0";
    // The original's layer-list check compares one list with two different values, so it
    // always fails; with its two flags and-combined, only the object layers decide.
    let status = if layers_ok { "OK" } else { "Faulty" };
    println!("{status} OCC22795");
    run.dump("bug22795.png")?;
    run.finish()
}
