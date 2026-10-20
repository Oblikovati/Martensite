//! The GLSL tables the standard programs append (`Shaders_*` literals), byte for byte.

/// `Shaders_LightShadow_glsl`: the shadow map lookup.
pub(super) const LIGHT_SHADOW: &str = include_str!("glsl/LightShadow_glsl.glsl");
/// `Shaders_PBRDistribution_glsl`: the micro facet normal distribution.
pub(super) const PBR_DISTRIBUTION: &str = include_str!("glsl/PBRDistribution_glsl.glsl");
/// `Shaders_PBRDirectionalLight_glsl`: the PBR directional light.
pub(super) const PBR_DIRECTIONAL_LIGHT: &str = include_str!("glsl/PBRDirectionalLight_glsl.glsl");
/// `Shaders_PBRGeometry_glsl`: the PBR geometry term.
pub(super) const PBR_GEOMETRY: &str = include_str!("glsl/PBRGeometry_glsl.glsl");
/// `Shaders_PBRFresnel_glsl`: the PBR Fresnel factors.
pub(super) const PBR_FRESNEL: &str = include_str!("glsl/PBRFresnel_glsl.glsl");
/// `Shaders_PBRCookTorrance_glsl`: the Cook-Torrance BRDF.
pub(super) const PBR_COOK_TORRANCE: &str = include_str!("glsl/PBRCookTorrance_glsl.glsl");
/// `Shaders_PBRIllumination_glsl`: the PBR direct illumination.
pub(super) const PBR_ILLUMINATION: &str = include_str!("glsl/PBRIllumination_glsl.glsl");
/// `Shaders_PBRPointLight_glsl`: the PBR point light.
pub(super) const PBR_POINT_LIGHT: &str = include_str!("glsl/PBRPointLight_glsl.glsl");
/// `Shaders_PBRSpotLight_glsl`: the PBR spot light.
pub(super) const PBR_SPOT_LIGHT: &str = include_str!("glsl/PBRSpotLight_glsl.glsl");
/// `Shaders_PBREnvBaking_fs`: the environment baking fragment stage.
pub(super) const PBR_ENV_BAKING_FS: &str = include_str!("glsl/PBREnvBaking_fs.glsl");
/// `Shaders_PBREnvBaking_vs`: the environment baking vertex stage.
pub(super) const PBR_ENV_BAKING_VS: &str = include_str!("glsl/PBREnvBaking_vs.glsl");
/// `Shaders_PhongDirectionalLight_glsl`: the Phong directional light.
pub(super) const PHONG_DIRECTIONAL_LIGHT: &str =
    include_str!("glsl/PhongDirectionalLight_glsl.glsl");
/// `Shaders_PhongPointLight_glsl`: the Phong point light.
pub(super) const PHONG_POINT_LIGHT: &str = include_str!("glsl/PhongPointLight_glsl.glsl");
/// `Shaders_PhongSpotLight_glsl`: the Phong spot light.
pub(super) const PHONG_SPOT_LIGHT: &str = include_str!("glsl/PhongSpotLight_glsl.glsl");
/// `Shaders_PointLightAttenuation_glsl`: the point light attenuation.
pub(super) const POINT_LIGHT_ATTENUATION: &str =
    include_str!("glsl/PointLightAttenuation_glsl.glsl");
/// `Shaders_SkydomBackground_fs`: the skydome background fragment stage.
pub(super) const SKYDOM_BACKGROUND_FS: &str = include_str!("glsl/SkydomBackground_fs.glsl");
/// `Shaders_TangentSpaceNormal_glsl`: the tangent-space normal map.
pub(super) const TANGENT_SPACE_NORMAL: &str = include_str!("glsl/TangentSpaceNormal_glsl.glsl");
