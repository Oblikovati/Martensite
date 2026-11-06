#version 450
// Graphic3dShaderManager::getBgCubeMapProgram fragment stage; the background
// cube map (occSampler0) is set 0, binding 0.
layout(location=0) in vec3 ViewDirection;
layout(location=0) out vec4 occFragColor;
layout(set=0, binding=0) uniform samplerCube occEnvCubemap;

void main()
{
  occFragColor = vec4(texture (occEnvCubemap, ViewDirection).rgb, 1.0);
}
