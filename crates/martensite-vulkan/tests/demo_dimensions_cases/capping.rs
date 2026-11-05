//! Both object-local planes retain the actual file texture and signed texture scale.
use super::*;
use martensite_tkernel::{
    ncollection::{Vec2, Vec4},
    tcollection::AsciiString,
};
use martensite_tkservice::graphic3d::{
    Graphic3dClipPlane, Graphic3dTexture2D, Graphic3dTextureMap,
};
use martensite_tkservice::graphic3d::{Graphic3dTextureRoot, Graphic3dTextureRootMembers};
use std::path::Path;

pub(super) fn attach_planes(session: &ViewerSession, hatch: &Path) -> Outcome {
    for (name, color) in [("a", NamedColor::GRAY90), ("b", NamedColor::GOLD)] {
        let plane = capping_plane(hatch, Color::named(color))?;
        let object = session
            .object(name)
            .ok_or_else(|| format!("object {name:?}; expected source a or b"))?;
        object.add_clip_plane(&plane);
        session.view().redraw();
    }
    Ok(())
}
pub(super) fn capping_plane(
    path: &Path,
    color: Color,
) -> Result<Handle<Graphic3dClipPlane>, Failure> {
    let plane = Handle::new(Graphic3dClipPlane::from_equation(Vec4::new(
        0., 0., 1., 0.,
    ))?);
    plane.set_capping(true);
    plane.set_capping_color(color)?;
    let texture = hatch_texture(path)?;
    plane.set_capping_texture(texture.cast::<dyn Graphic3dTextureMap>())?;
    Ok(plane)
}
fn hatch_texture(path: &Path) -> Result<Handle<Graphic3dTexture2D>, Failure> {
    let texture = Handle::new(Graphic3dTexture2D::from_file(&AsciiString::from(
        path.to_string_lossy().as_ref(),
    )));
    if !texture.is_done() {
        return Err(Failure::with_message(
            "Standard_ProgramError",
            &format!("hatch path {path:?}; expected readable source texture"),
        ));
    }
    texture.get_params().set_scale(Vec2::new(0.05, -0.05));
    Ok(texture)
}
