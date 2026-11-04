// the reference implementation Declarations.glsl texture-set samplers of the textured Phong/PBR programs
// (Graphic3dShaderManager::getStdProgramPhong sets BaseColor|Emissive|Occlusion and, for PBR,
// MetallicRoughness). Bindings follow Graphic3dTextureUnit; GlContext::BindTextures binds a
// white mock for a missing unit (black for Normal), so a missing map multiplies by one.
layout(set=5,binding=1) uniform sampler2D surfaceEmissiveMap;
layout(set=5,binding=2) uniform sampler2D surfaceOcclusionMap;
layout(set=5,binding=3) uniform sampler2D surfaceNormalMap;
layout(set=5,binding=4) uniform sampler2D surfaceMetallicRoughnessMap;
vec2 materialMapCoord() { return surfaceUV.st / surfaceUV.w; }
// occMaterialEmission: the material emission times the RGB of the emissive map.
vec3 emissiveMapTexel() { return texture(surfaceEmissiveMap, materialMapCoord()).rgb; }
// occMaterialOcclusion: the colour times the R channel of the occlusion map.
float occlusionMapTexel() { return texture(surfaceOcclusionMap, materialMapCoord()).r; }
// occMaterialRoughness/occMaterialMetallic: G scales roughness, B scales metallic.
vec2 metallicRoughnessMapTexel() { return texture(surfaceMetallicRoughnessMap, materialMapCoord()).bg; }
