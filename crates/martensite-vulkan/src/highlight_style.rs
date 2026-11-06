//! OpenGL color highlighting overrides vertex/bound color without replacing line width/pattern.
use crate::{PrimitiveDraw, RenderError, error::rejected};
use martensite_tkernel::standard::Handle;
use martensite_tkservice::{aspect::AspectTypeOfHighlightMethod, graphic3d::*};
pub(crate) fn validate_structure(structure: &Graphic3dCStructureFields) -> Result<(), RenderError> {
    let style = &structure.highlight_style;
    // AspectTypeOfHighlightMethod has only COLOR and BOUNDBOX; a future value stays explicit.
    if structure.highlight
        && !style.is_null()
        && !matches!(
            style.method(),
            AspectTypeOfHighlightMethod::Color | AspectTypeOfHighlightMethod::Boundbox
        )
    {
        return Err(rejected(
            style.method(),
            "color or bounding-box highlighting",
        ));
    }
    Ok(())
}
pub(crate) fn apply_highlight(
    draw: &mut PrimitiveDraw,
    structure: &Graphic3dCStructureFields,
) -> Result<(), RenderError> {
    // GlStructure::Render keeps the groups' own colours under a bounding-box highlight.
    if !structure.highlight
        || structure.highlight_style.is_null()
        || crate::bound_box_highlight::is_bound_box_highlight(structure)
    {
        return Ok(());
    }
    let color = structure.highlight_style.color_rgba();
    let rgba = [
        color.rgb().red() as f32,
        color.rgb().green() as f32,
        color.rgb().blue() as f32,
        color.alpha(),
    ];
    let vertices = draw
        .vertices
        .iter()
        .map(|v| crate::RenderVertex { color: rgba, ..*v })
        .collect::<Vec<_>>();
    crate::scene::validate_components(&vertices, &draw.clip_transform)?;
    if draw.annotation || draw.topology() == crate::PrimitiveTopology::Points {
        crate::scene::validate_annotation_opacity(&vertices)?;
    }
    draw.vertices = vertices;
    draw.unlit_back_color = draw.unlit_back_color.map(|_| rgba);
    let alpha = match highlight_fill_material(&structure.highlight_style) {
        Some((material, alpha)) => draw.lighting.as_mut().map_or(rgba[3], |lighting| {
            lighting.material = material;
            lighting.vertex_colors = false;
            alpha
        }),
        None => {
            if let Some(lighting) = &mut draw.lighting {
                lighting
                    .material
                    .set_color(rgba[..3].try_into().expect("RGB components"));
                lighting.vertex_colors = false;
            }
            rgba[3]
        }
    };
    crate::retained_user_program::highlight_user_program(draw, rgba);
    blend_translucent_fill(draw, alpha);
    Ok(())
}

/// Shading with a highlight fill aspect (a highlight style that carries a material):
/// the shaded faces take that aspect's materials and interior colours instead of the
/// highlight colour, and their opacity is the aspect's front material alpha
/// (GlContext::CheckIsTransparent). Example: `highlight_fill_material(&style)`.
fn highlight_fill_material(
    style: &Handle<dyn Graphic3dPresentationAttributesSource>,
) -> Option<(crate::RenderMaterial, f32)> {
    let fill = style.get()?.basic_fill_area_aspect();
    let aspect = fill.get()?;
    let material = crate::retained_lighting::aspect_material(aspect);
    Some((material, aspect.front_material().alpha()))
}

/// A highlight colour with alpha below one makes a fill transparent (GlContext::
/// CheckIsTransparent takes both alphas from it): the face is blended with that opacity in the
/// transparent pass. Lines keep the colour unblended, as the native opaque pass draws them.
fn blend_translucent_fill(draw: &mut PrimitiveDraw, alpha: f32) {
    if alpha >= 1. || draw.topology() != crate::PrimitiveTopology::Triangles {
        return;
    }
    if let Some(lighting) = &mut draw.lighting {
        lighting.material.set_alpha(alpha);
    }
    draw.surface_blend = true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::{quantity::ColorRgba, standard::Handle};
    #[test]
    fn color_highlight_preserves_homogeneous_position_weight() {
        let manager = Handle::new(
            Graphic3dStructureManager::new(
                &martensite_tkservice::null_driver::NullGraphicDriver::new_handle(),
            )
            .unwrap(),
        );
        let mut fields = Graphic3dCStructureBase::new(&manager)
            .unwrap()
            .fields
            .read()
            .clone();
        fields.highlight = true;
        fields.highlight_style = Handle::new(Graphic3dPresentationAttributes::default()).cast();
        fields
            .highlight_style
            .set_color(*ColorRgba::new(0., 1., 0., 1.).unwrap().rgb());
        let vertices = vec![crate::RenderVertex::homogeneous([2., 3., 4., 2.], [1.; 4]); 3];
        let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip()).unwrap();
        draw.unlit_back_color = Some([1., 0., 0., 0.25]);
        apply_highlight(&mut draw, &fields).unwrap();
        assert_eq!(draw.unlit_back_color, Some([0., 1., 0., 1.]));
        assert_eq!(draw.vertices[0].weight, 2.);
        assert_eq!(draw.vertices[0].color, [0., 1., 0., 1.]);
        fields.highlight_style.set_transparency(0.5);
        apply_highlight(&mut draw, &fields).unwrap();
        assert_eq!(draw.vertices[0].color, [0., 1., 0., 0.5]);
        assert!(
            draw.surface_blend,
            "a translucent highlight blends the fill"
        );
    }

    /// A dynamic highlight with 0.5 transparency and PLASTIC material shades the highlighted
    /// faces with the fill aspect's plastic material in the cyan interior colour (ambient 0.25,
    /// diffuse 0.24 of it) at half opacity, not with the flat highlight colour.
    #[test]
    fn a_highlight_fill_aspect_supplies_the_shaded_material() {
        let manager = Handle::new(
            Graphic3dStructureManager::new(
                &martensite_tkservice::null_driver::NullGraphicDriver::new_handle(),
            )
            .unwrap(),
        );
        let mut fields = Graphic3dCStructureBase::new(&manager)
            .unwrap()
            .fields
            .read()
            .clone();
        let cyan = *ColorRgba::new(0., 1., 1., 1.).unwrap().rgb();
        let style = Handle::new(Graphic3dPresentationAttributes::default());
        style.set_color(cyan);
        let fill = Handle::new(Graphic3dAspectFillArea3d::default());
        let mut plastic = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
        plastic.set_transparency(0.5).unwrap();
        fill.get().unwrap().set_front_material(plastic);
        fill.get().unwrap().set_interior_color(cyan);
        style.set_basic_fill_area_aspect(fill);
        fields.highlight = true;
        fields.highlight_style = style.cast();
        let lighting = crate::RenderLighting::new(
            vec![[0., 0., 1.]; 3],
            crate::RenderMaterial::default(),
            crate::identity_clip(),
            crate::identity_clip(),
            false,
            crate::RenderShading::Phong,
        )
        .unwrap();
        let vertices = vec![crate::RenderVertex::new([0.; 3], [1.; 4]); 3];
        let mut draw = PrimitiveDraw::new(vertices, vec![0, 1, 2], crate::identity_clip())
            .unwrap()
            .with_lighting(lighting)
            .unwrap();
        apply_highlight(&mut draw, &fields).unwrap();
        let front = draw.lighting.as_ref().unwrap().material.common[0];
        assert_eq!(front.diffuse[0], 0.);
        assert!(
            (front.diffuse[1] - 0.24).abs() < 1e-6,
            "{:?}",
            front.diffuse
        );
        assert!(
            (front.ambient[2] - 0.25).abs() < 1e-6,
            "{:?}",
            front.ambient
        );
        assert_eq!(front.diffuse[3], 0.5);
        assert!(
            draw.surface_blend,
            "the half-transparent plastic fill blends"
        );
    }

    #[test]
    fn translucent_highlight_stays_rejected_for_points() {
        let manager = Handle::new(
            Graphic3dStructureManager::new(
                &martensite_tkservice::null_driver::NullGraphicDriver::new_handle(),
            )
            .unwrap(),
        );
        let mut fields = Graphic3dCStructureBase::new(&manager)
            .unwrap()
            .fields
            .read()
            .clone();
        fields.highlight = true;
        fields.highlight_style = Handle::new(Graphic3dPresentationAttributes::default()).cast();
        fields.highlight_style.set_transparency(0.5);
        let vertices = vec![crate::RenderVertex::homogeneous([0., 0., 0., 1.], [1.; 4]); 1];
        let style = crate::PointStyle::new(1., [1, 1], vec![[1.; 4]]).unwrap();
        let mut draw =
            PrimitiveDraw::points(vertices, vec![0], crate::identity_clip(), style).unwrap();
        assert!(apply_highlight(&mut draw, &fields).is_err());
    }
}
