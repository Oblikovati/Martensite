//! Explicit setter selection; historical replay remains the CLI default.
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkhlr::contap::{ContapArcFunction, ContapSamplingContext, ContapSurfFunction};

pub(super) enum SurfaceSetup<'a> {
    Historical,
    Local(&'a mut ContapSamplingContext),
}
impl SurfaceSetup<'_> {
    pub fn sample_count(&mut self, surface: &Adaptor3dSurfaceHandle) -> Result<i32, Failure> {
        match self {
            Self::Historical => {
                martensite_tkhlr::contap::ContapHContTool::nb_sample_points(surface)
            }
            Self::Local(context) => context.nb_sample_points(surface),
        }
    }
    pub fn sample_point(
        &self,
        surface: &Adaptor3dSurfaceHandle,
        index: i32,
    ) -> Result<(f64, f64), Failure> {
        match self {
            Self::Historical => {
                martensite_tkhlr::contap::ContapHContTool::sample_point(surface, index)
            }
            Self::Local(context) => context.sample_point(surface, index),
        }
    }

    pub fn set_function_surface(
        &mut self,
        function: &mut ContapSurfFunction,
        surface: &Adaptor3dSurfaceHandle,
    ) -> Result<(), Failure> {
        match self {
            Self::Historical => function.set_surface(surface),
            Self::Local(context) => function.set_surface_with_context(surface, context),
        }
    }
    pub fn set_arc_surface(
        &mut self,
        function: &mut ContapArcFunction,
        surface: &Adaptor3dSurfaceHandle,
    ) -> Result<(), Failure> {
        match self {
            Self::Historical => function.set_surface(surface),
            Self::Local(context) => function.set_surface_with_context(surface, context),
        }
    }
}
