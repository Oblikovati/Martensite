#version 450
#extension GL_GOOGLE_include_directive : require
// the reference implementation resources/Shaders/PBREnvBaking.vs (Graphic3dShaderManager::getPBREnvBakingProgram).
// The quad of GlPBREnvironment::initVAO is drawn as a four-vertex triangle strip. A
// positive Vulkan viewport maps NDC y = -1 to image row 0, as GL maps it to texel row 0 of the
// attached face, so the rendered faces keep the native texel layout.
#include "environment_cube.glsl"
#include "pbr_env_baking.glsl"
layout(location=0) out vec3 ViewDirection; //!< direction of fetching from environment cubemap
const vec4 THE_BAKING_QUAD[4] = vec4[] (vec4(-1.0, -1.0, 0.0, 0.0), vec4(1.0, -1.0, 0.0, 0.0),
                                        vec4(-1.0,  1.0, 0.0, 0.0), vec4(1.0,  1.0, 0.0, 0.0));

void main()
{
  vec4 occVertex = THE_BAKING_QUAD[gl_VertexIndex];
  vec3 aDir;
  vec2 aCoord;
  if (bake.uCurrentSide == 0)
  {
    aCoord = mat2( 0,-1,-1, 0) * occVertex.xy;
    aDir.x = 1.0;
    aDir.y = aCoord.x;
    aDir.z = aCoord.y;
  }
  else if (bake.uCurrentSide == 1)
  {
    aCoord = mat2( 0, 1,-1, 0) * occVertex.xy;
    aDir.x = -1.0;
    aDir.y = aCoord.x;
    aDir.z = aCoord.y;
  }
  else if (bake.uCurrentSide == 2)
  {
    aCoord = mat2( 0, 1, 1, 0) * occVertex.xy;
    aDir.x = aCoord.y;
    aDir.y = 1.0;
    aDir.z = aCoord.x;
  }
  else if (bake.uCurrentSide == 3)
  {
    aCoord = mat2( 0, 1,-1, 0) * occVertex.xy;
    aDir.x = aCoord.y;
    aDir.y = -1.0;
    aDir.z = aCoord.x;
  }
  else if (bake.uCurrentSide == 4)
  {
    aCoord = mat2( 1, 0, 0,-1) * occVertex.xy;
    aDir.x = aCoord.x;
    aDir.y = aCoord.y;
    aDir.z = 1.0;
  }
  else //if (uCurrentSide == 5)
  {
    aCoord = mat2(-1, 0, 0,-1) * occVertex.xy;
    aDir.x = aCoord.x;
    aDir.y = aCoord.y;
    aDir.z = -1.0;
  }
  ViewDirection = cubemapVectorTransform (aDir, bake.uYCoeff, bake.uZCoeff);
  gl_Position = vec4 (occVertex.xy, 0.0, 1.0);
}
