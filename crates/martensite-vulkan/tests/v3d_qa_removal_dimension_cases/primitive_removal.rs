//! OCC136 removes the same four anonymous identities it displayed and selected.
use super::Outcome;
use crate::session_run::{SessionRun, banner};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis,
    BRepPrimAPIMakeSphere,
};
use martensite_tkv3d::ais::{AisInteractiveContext, AisInteractiveObject, AisShape};

type Objects = Vec<Handle<dyn AisInteractiveObject>>;

pub(super) fn primitive_objects() -> Result<Objects, Failure> {
    let origin = Point3::new(0., 0., 0.);
    let far = Point3::new(100., 100., 100.);
    let box_shape = BRepPrimAPIMakeBox::from_points(origin, far)?
        .solid()?
        .into_shape();
    let sphere = BRepPrimAPIMakeSphere::new(50.)?.solid()?.into_shape();
    let mut axis = Axis2::new(far, Direction3::new(1., 1., 1.)?)?;
    let cone = BRepPrimAPIMakeCone::with_axes(axis, 70., 30., 100.)?
        .solid()?
        .into_shape();
    axis.set_location(Point3::new(100., 0., 0.));
    axis.set_direction(Direction3::new(-1., -1., 1.)?)?;
    let cylinder = BRepPrimAPIMakeCylinder::with_axes(axis, 50., 100.)
        .solid()?
        .into_shape();
    Ok(anonymous_objects([box_shape, sphere, cone, cylinder]))
}

fn anonymous_objects(shapes: [Shape; 4]) -> Objects {
    shapes
        .iter()
        .map(|shape| AisShape::new(shape).cast())
        .collect()
}

pub(super) fn display_and_select(
    context: &AisInteractiveContext,
    objects: &Objects,
) -> Result<(), Failure> {
    context.erase_all(false)?;
    for object in objects {
        context.display(object, false)?;
    }
    context.init_selected();
    for object in objects {
        context.add_or_remove_selected_object(object, false)?;
    }
    Ok(())
}

pub(super) fn remove_primitives(
    context: &AisInteractiveContext,
    objects: &Objects,
) -> Result<(), Failure> {
    for object in objects {
        context.remove(object, false)?;
    }
    context.update_current_viewer();
    Ok(())
}

/// Full original, including its literal expected color failure; `remove_four_primitives()?`.
pub(crate) fn remove_four_primitives() -> Outcome {
    banner(&[
        "TODO OCC12345 ALL: Error : Colors are not equal in default coordinate and in the near coordinates too",
        "================",
        "OCC136",
        "================",
    ]);
    let mut run = SessionRun::new()?;
    let objects = primitive_objects()?;
    display_and_select(run.session().context(), &objects)?;
    remove_primitives(run.session().context(), &objects)?;
    run.session().fit()?;
    run.session().check_color(211, 215, [0.8; 3])?;
    run.dump("bug136.png")?;
    run.finish()
}
