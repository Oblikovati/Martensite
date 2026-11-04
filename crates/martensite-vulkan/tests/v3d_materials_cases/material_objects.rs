//! Each material is applied to the actual translated sphere and box before optional color.
use super::*;

pub(super) fn placements(column: usize, material: usize) -> ([f64; 3], [f64; 3]) {
    let x = 30. + column as f64 * 35.;
    let y = 15. + material as f64 * 10.;
    ([x, y, 0.], [x + 10., y, 0.])
}
pub(super) fn draw_column(session: &mut ViewerSession, column: usize) -> Result<(), Failure> {
    session.without_updates(|session| {
        for (index, material) in MATERIALS.iter().enumerate() {
            draw_pair(session, column, index, material)?;
        }
        Ok(())
    })
}
fn draw_pair(
    session: &mut ViewerSession,
    column: usize,
    index: usize,
    material: &str,
) -> Result<(), Failure> {
    let (sphere, cube) = pair_geometry(column, index)?;
    let sphere_name = format!("s{column}_{index}");
    let box_name = format!("b{column}_{index}");
    let request = DisplayRequest::default().with_display_mode(1);
    session.display(&sphere_name, &sphere, &request)?;
    session.display(&box_name, &cube, &request)?;
    apply_pair_material(session, [&sphere_name, &box_name], material, column)
}
fn apply_pair_material(
    session: &ViewerSession,
    names: [&str; 2],
    material: &str,
    column: usize,
) -> Result<(), Failure> {
    let material = resolve_material(material)?;
    for name in names {
        session.set_material(name, material)?;
    }
    if let Some(color) = column_color(column)? {
        for name in names {
            session.set_color(name, color)?;
        }
    }
    Ok(())
}
pub(super) fn resolve_material(
    material: &str,
) -> Result<martensite_tkservice::graphic3d::Graphic3dNameOfMaterial, Failure> {
    Graphic3dMaterialAspect::material_from_name(material).ok_or_else(|| {
        martensite_tkernel::standard::exceptions::program_error(&format!(
            "material {material:?}; expected standard material"
        ))
    })
}
pub(super) fn column_color(column: usize) -> Result<Option<Color>, Failure> {
    if column == 0 {
        return Ok(None);
    }
    Ok(Some(named_color(if column == 3 {
        "blue1"
    } else {
        COLUMN_COLORS[column]
    })?))
}
fn pair_geometry(column: usize, index: usize) -> Result<(Shape, Shape), Failure> {
    let (sphere_position, box_position) = placements(column, index);
    let sphere = crate::primitive_shapes::translated(
        &crate::primitive_shapes::sphere(4.)?,
        sphere_position,
    )?;
    let cube = crate::primitive_shapes::translated(
        &crate::primitive_shapes::box_at([-2.; 3], [4.; 3])?,
        box_position,
    )?;
    Ok((sphere, cube))
}
