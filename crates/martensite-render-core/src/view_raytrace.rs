//! The ray-traced pass of a view frame (GlView::render with Graphic3dRM_RAYTRACING): the
//! frame environment it samples, the device's texture support and the view's accumulation.
use crate::RenderError;
use martensite_tkservice::graphic3d::{
    Graphic3dCView, Graphic3dCViewMembers as _, Graphic3dRenderingMode,
};

/// The view environment of the frame (retained by the driver's view) and the ray-traced pass,
/// advanced in the view's accumulation: a cubemap background is the ray-traced environment.
/// Example: `let (raytrace, environment) = view_raytrace(view, extent, &extras)?;`.
pub(crate) fn view_raytrace(
    view: &dyn Graphic3dCView,
    extent: [u32; 2],
    extras: &crate::view_scene::ViewFrameExtras<'_>,
) -> Result<
    (
        Option<crate::raytrace::RaytraceFrame>,
        Option<crate::environment_frame::FrameEnvironment>,
    ),
    RenderError,
> {
    let mut environment = match extras.frame_steps {
        Some(render) => render.retained.frame_environment(view)?,
        None => None,
    };
    let raytraced = view.rendering_params().method == Graphic3dRenderingMode::Raytracing;
    let cube = match extras.frame_steps {
        Some(render) if raytraced => render.retained.raytrace_cube(view, &mut environment)?,
        _ => None,
    };
    let inputs = crate::raytrace::RaytraceFrameInputs {
        textured: extras
            .limits
            .is_some_and(|limits| limits.has_ray_tracing_textures()),
        cube,
    };
    let extent = crate::view_scene::scene_extent(view, extent)?;
    let mut raytrace = match extras.frame_steps {
        Some(render) => {
            let mut bvh = render.raytrace_bvh.write();
            crate::raytrace::prepare_raytrace(view, extent, &mut bvh, inputs)?
        }
        None => {
            let mut bvh = crate::raytrace::RaytraceBvhCache::default();
            crate::raytrace::prepare_raytrace(view, extent, &mut bvh, inputs)?
        }
    };
    if let (Some(frame), Some(render)) = (raytrace.as_mut(), extras.frame_steps) {
        render.raytrace_accumulation.write().advance(frame)?;
    }
    Ok((raytrace, environment))
}
