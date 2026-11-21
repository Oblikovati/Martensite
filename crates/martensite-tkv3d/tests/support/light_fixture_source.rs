//! Shared native fixture light types preserve their native zero-direction constructor state.
use martensite_tkernel::standard::Handle;
use martensite_tkservice::graphic3d::*;
pub fn fixture_light(kind: usize) -> Handle<dyn Graphic3dCLightSource> {
    let kinds = [
        Graphic3dTypeOfLightSource::Ambient,
        Graphic3dTypeOfLightSource::Directional,
        Graphic3dTypeOfLightSource::Positional,
        Graphic3dTypeOfLightSource::Spot,
    ];
    Handle::new(Graphic3dCLight::new(kinds[kind])).into_upcast::<dyn Graphic3dCLightSource>()
}
