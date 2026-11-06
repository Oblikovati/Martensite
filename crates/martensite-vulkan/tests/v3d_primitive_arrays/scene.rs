use super::{arrays, specification};
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomPoint};
use martensite_tkv3d::{
    ais::{AisPoint, PrimitiveArrayPresentation},
    v3d::V3dTypeOfOrientation,
};
pub(super) fn stage(
    session: &mut ViewerSession,
    enabled: bool,
    non_interleaved: bool,
) -> Result<(), Failure> {
    session.clear()?;
    session.activate_view(0)?;
    session.set_standard_projection(V3dTypeOfOrientation::Zpos)?;
    session.view().viewer().driver().enable_vbo(enabled);
    println!("TEST WITH VBO is {}", if enabled { "ON" } else { "OFF" });
    markers(session)?;
    primitive_objects(session, non_interleaved)?;
    session.fit()
}
fn markers(session: &mut ViewerSession) -> Result<(), Failure> {
    for (name, x) in [("point1", 65.), ("point2", -145.)] {
        let point: Handle<dyn GeomPoint> = Handle::new(GeomCartesianPoint::new(x, 0., 0.)).cast();
        session.display_object(name, AisPoint::new(&point)?.cast())?;
    }
    Ok(())
}
fn primitive_objects(session: &mut ViewerSession, non_interleaved: bool) -> Result<(), Failure> {
    let mut previous = None;
    for spec in specification::specifications() {
        if previous != Some(spec.kind) {
            println!(
                "Graphic3dArrayOf{}: TEST",
                specification::category(spec.kind)
            );
            previous = Some(spec.kind);
        }
        let array = arrays::build(spec, non_interleaved)?;
        let object = PrimitiveArrayPresentation::from_array(array);
        object.setup_vertex_color_shading()?;
        point_marker(&object, spec.kind)?;
        session.display_object(spec.name, object.cast())?;
    }
    Ok(())
}

fn point_marker(
    object: &PrimitiveArrayPresentation,
    kind: martensite_tkservice::graphic3d::Graphic3dTypeOfPrimitiveArray,
) -> Result<(), Failure> {
    use martensite_tkernel::quantity::{Color, NamedColor};
    use martensite_tkservice::{
        aspect::AspectTypeOfMarker,
        graphic3d::{Graphic3dAspectMarker3d, Graphic3dTypeOfPrimitiveArray},
    };
    if kind != Graphic3dTypeOfPrimitiveArray::Points {
        return Ok(());
    }
    let marker = Graphic3dAspectMarker3d::new(
        AspectTypeOfMarker::Point,
        Color::named(NamedColor::YELLOW),
        1.,
    )?;
    object.set_marker_aspect(&Handle::new(marker));
    Ok(())
}
