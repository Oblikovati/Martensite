// The interface a user GLSL program sees when the recording tool compiles it for
// Vulkan with glslang's relaxed rules (-V -R): every loose `uniform` below and in the user
// source lands in one std140 default uniform block that the renderer fills by member name
// (src/user_program/uniform_values.rs). The names, packing and helper functions follow the
// native program interface: vertex attributes occVertex, occNormal, occTexCoord and occVertColor
// at locations 0-3, the occ* matrix, light, material, clipping and colour uniforms, and the
// occSetFragColor output. The tool prepends the stage define (VERTEX_SHADER, ...) and the
// THE_MAX_LIGHTS, THE_MAX_CLIP_PLANES and THE_NB_FRAG_OUTPUTS constants.
#define THE_SHADER_IN in
#define THE_SHADER_OUT out
#define THE_ATTRIBUTE in
#define THE_PREC_ENUM
#define occTexture1D texture
#define occTexture2D texture
#define occTexture3D texture
#define occTextureCube texture
#define occTextureCubeLod textureLod
// GLSL 1.10 spellings of sources written for a `#version 110` header.
#define texture2D texture
#define textureCube texture

#if defined(VERTEX_SHADER)
#define varying out
layout(location = 0) in vec4 occVertex;
layout(location = 1) in vec3 occNormal;
layout(location = 2) in vec4 occTexCoord;
layout(location = 3) in vec4 occVertColor;
#elif defined(FRAGMENT_SHADER)
#define varying in
layout(location = 0) out vec4 occFragColor;
#define occFragColor0 occFragColor
#define gl_FragColor occFragColor
#if defined(OCC_WRITE_WEIGHT_OIT_COVERAGE)
// The second output of a weighted-OIT program: the accumulated coverage (native occFragColor1).
layout(location = 1) out vec4 occFragCoverage;
#define occFragColor1 occFragCoverage
#endif
#endif

#define PI       3.141592654
#define PI_2     6.283185307
#define PI_DIV_2 1.570796327
#define PI_DIV_3 1.047197551
#define PI_DIV_4 0.785398163
#define INV_PI   0.318309886
#define INV_PI_2 0.159154943

uniform mat4 occWorldViewMatrix;
uniform mat4 occProjectionMatrix;
uniform mat4 occModelWorldMatrix;
uniform mat4 occWorldViewMatrixInverse;
uniform mat4 occProjectionMatrixInverse;
uniform mat4 occModelWorldMatrixInverse;
uniform mat4 occWorldViewMatrixTranspose;
uniform mat4 occProjectionMatrixTranspose;
uniform mat4 occModelWorldMatrixTranspose;
uniform mat4 occWorldViewMatrixInverseTranspose;
uniform mat4 occProjectionMatrixInverseTranspose;
uniform mat4 occModelWorldMatrixInverseTranspose;

const int OccLightType_Direct = 1;
const int OccLightType_Point  = 2;
const int OccLightType_Spot   = 3;

uniform vec4 occLightAmbient;
#if defined(THE_MAX_LIGHTS) && (THE_MAX_LIGHTS > 0)
uniform int  occLightSourcesCount;
uniform vec4 occLightSources[THE_MAX_LIGHTS * 4];
uniform int  occLightSourcesTypes[THE_MAX_LIGHTS];
#define occLight_Index(theId)             theId
#define occLight_Type(theId)              occLightSourcesTypes[occLight_Index(theId)]
#define occLight_Specular(theId)          occLightSources[occLight_Index(theId) * 4 + 0].rgb
#define occLight_Intensity(theId)         occLightSources[occLight_Index(theId) * 4 + 0].a
#define occLight_IsHeadlight(theId)       false
#define occLight_Position(theId)          occLightSources[occLight_Index(theId) * 4 + 1].xyz
#define occLight_SpotDirection(theId)     occLightSources[occLight_Index(theId) * 4 + 2].xyz
#define occLight_Range(theId)             occLightSources[occLight_Index(theId) * 4 + 2].w
#define occLight_SpotCutOff(theId)        occLightSources[occLight_Index(theId) * 4 + 3].z
#define occLight_SpotExponent(theId)      occLightSources[occLight_Index(theId) * 4 + 3].w
#define occLight_Diffuse(theId)           occLightSources[occLight_Index(theId) * 4 + 0].rgb
#define occLight_ConstAttenuation(theId)  occLightSources[occLight_Index(theId) * 4 + 3].x
#define occLight_LinearAttenuation(theId) occLightSources[occLight_Index(theId) * 4 + 3].y
#endif

// Common (non-PBR) front and back material: diffuse, emission, specular + shininess, ambient.
uniform vec4 occCommonMaterial[4 * 2];
vec4  occMaterial_Diffuse(in bool theIsFront)   { return theIsFront ? occCommonMaterial[0]     : occCommonMaterial[4]; }
vec3  occMaterial_Emission(in bool theIsFront)  { return theIsFront ? occCommonMaterial[1].rgb : occCommonMaterial[5].rgb; }
vec3  occMaterial_Specular(in bool theIsFront)  { return theIsFront ? occCommonMaterial[2].rgb : occCommonMaterial[6].rgb; }
float occMaterial_Shininess(in bool theIsFront) { return theIsFront ? occCommonMaterial[2].a   : occCommonMaterial[6].a; }
vec3  occMaterial_Ambient(in bool theIsFront)   { return theIsFront ? occCommonMaterial[3].rgb : occCommonMaterial[7].rgb; }
#define occMaterial_Color occMaterial_Diffuse

#if defined(THE_HAS_DEFAULT_SAMPLER)
uniform sampler2D occSampler0;
#define occActiveSampler    occSampler0
#define occSamplerBaseColor occSampler0
#endif

uniform vec4  occColor;
uniform int   occDistinguishingMode;
uniform int   occTextureEnable;
uniform vec4  occTexTrsf2d[2];
uniform float occPointSize;
uniform int   occOitOutput;
uniform float occOitDepthFactor;
uniform float occAlphaCutoff;
vec2  occTextureTrsf_Translation(void) { return occTexTrsf2d[0].xy; }
vec2  occTextureTrsf_Scale(void)       { return occTexTrsf2d[0].zw; }
float occTextureTrsf_RotationSin(void) { return occTexTrsf2d[1].x; }
float occTextureTrsf_RotationCos(void) { return occTexTrsf2d[1].y; }

#if defined(THE_MAX_CLIP_PLANES) && (THE_MAX_CLIP_PLANES > 0)
uniform vec4 occClipPlaneEquations[THE_MAX_CLIP_PLANES];
uniform int  occClipPlaneChains[THE_MAX_CLIP_PLANES];
uniform int  occClipPlaneCount;
#endif

#if defined(FRAGMENT_SHADER)
void occSetFragColor(in vec4 theColor)
{
#if defined(OCC_WRITE_WEIGHT_OIT_COVERAGE)
  // DeclarationsImpl.glsl: the weighted colour and coverage of the native OIT accumulation.
  float aWeight   = theColor.a * clamp (1e+2 * pow (1.0 - gl_FragCoord.z * occOitDepthFactor, 3.0), 1e-2, 1e+2);
  occFragCoverage = vec4 (theColor.a * aWeight, 0.0, 0.0, 0.0);
  occFragColor    = vec4 (theColor.rgb * theColor.a * aWeight, theColor.a);
#else
  occFragColor = theColor;
#endif
}
#endif
