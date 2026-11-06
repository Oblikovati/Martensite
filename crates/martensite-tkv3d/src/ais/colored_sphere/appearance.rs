use crate::ais::AisTriangulation;
use crate::prs3d::Prs3dShadingAspect;
use martensite_tkernel::{
    ncollection::{HArray1, Vec3},
    quantity::{Color, NamedColor},
    standard::{Failure, Handle},
};
use martensite_tkservice::{
    aspect::{AspectInteriorStyle, AspectTypeOfLine},
    graphic3d::{
        Graphic3dAspectFillArea3d, Graphic3dAspectsEdgeMembers as _, Graphic3dMaterialAspect,
        Graphic3dNameOfMaterial,
    },
};
/// The plastified shading aspect of the sphere, edges drawn when asked.
pub(super) fn sphere_shading(show_edges: bool) -> Result<Handle<Prs3dShadingAspect>, Failure> {
    let material = sphere_material()?;
    let aspect = Graphic3dAspectFillArea3d::new(
        AspectInteriorStyle::Solid,
        Color::named(NamedColor::WHITE),
        Color::named(NamedColor::YELLOW),
        AspectTypeOfLine::Solid,
        1.0,
        material.clone(),
        material,
    )?;
    aspect.set_draw_edges(show_edges);
    let shading = Prs3dShadingAspect::new()?;
    shading.set_aspect(Handle::new(aspect));
    Ok(Handle::new(shading))
}
/// One green colour per node of the triangulation of `shape`.
pub(super) fn set_green_node_colors(shape: &AisTriangulation) -> Result<(), Failure> {
    let tri = shape.get_triangulation();
    let points = tri
        .get()
        .ok_or_else(|| Failure::with_message("Standard_NullObject", "Poly_Triangulation"))?
        .nb_nodes();
    let colors = HArray1::<i32>::with_bounds(1, points)
        .map_err(|e| e.into_failure("NCollection_HArray1"))?;
    colors
        .change_array1()
        .init(i32::from_le_bytes([0, 255, 0, 0]));
    shape.set_colors(&Handle::new(colors));
    Ok(())
}

fn sphere_material() -> Result<Graphic3dMaterialAspect, Failure> {
    let mut material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    material.set_ambient_color(Color::from_vec3(Vec3::new(0.04, 0.04, 0.04))?);
    material.set_specular_color(Color::from_vec3(Vec3::new(0.5, 0.5, 0.5))?);
    Ok(material)
}
