// Environment (sphere) texture coordinates of Graphic3dShaderManager::getStdProgramUnlit with
// Graphic3dShaderFlags_TextureEnv, evaluated as Mesa's GLSL front end evaluates
// the native expression: matrix products column by column in source order (clip_position.glsl;
// SPIR-V's OpMatrixTimesMatrix is lowered from the last column with ffma instead), normalize()
// as x * inversesqrt(dot) (glsl_normalize.glsl) and reflect() as I - 2.0 * (dot(N, I) * N).
#include "glsl_normalize.glsl"
layout(location=3) in vec3 normal;
// occModelWorldMatrix, occWorldViewMatrix and their inverse transposes, as the reference implementation uploads them.
layout(set=2,binding=0,std430) readonly buffer SphereMapMatrices {
    mat4 modelWorld;
    mat4 modelWorldInverseTranspose;
    mat4 worldView;
    mat4 worldViewInverseTranspose;
} sphereMap;
mat4 glslMatrixProduct(mat4 left, mat4 right) {
    return mat4(clipPosition(left,right[0]),clipPosition(left,right[1]),
                clipPosition(left,right[2]),clipPosition(left,right[3]));
}
// THE_FUNC_transformNormal_view.
vec3 sphereMapNormal(vec3 objectNormal) {
    mat4 inverseTranspose=glslMatrixProduct(sphereMap.worldViewInverseTranspose,sphereMap.modelWorldInverseTranspose);
    vec4 aResult=clipPosition(inverseTranspose,vec4(objectNormal,0.0));
    return glslNormalize(aResult.xyz);
}
vec4 sphereMapTexCoord(vec4 occVertex, vec3 occNormal) {
    vec4 aPosition=clipPosition(glslMatrixProduct(sphereMap.worldView,sphereMap.modelWorld),occVertex);
    vec3 aNormal=sphereMapNormal(occNormal);
    vec3 anIncident=glslNormalize(aPosition.xyz);
    vec3 aReflect=anIncident-2.0*(dot(aNormal,anIncident)*aNormal);
    aReflect.z+=1.0;
    return vec4(aReflect.xy*inversesqrt(dot(aReflect,aReflect))*0.5+vec2(0.5),0.0,1.0);
}
