
uniform mat4  uModelTrsf;
uniform mat3  uNormTrsf;
uniform vec4  uColor;
uniform vec3  uLightDir;
uniform vec2  uScale;
THE_SHADER_OUT vec4  vColor;
THE_SHADER_OUT vec3  vNormal;
void main() {
  vec4 aPos = uModelTrsf * occVertex;
  aPos.xy  *= uScale;
  vNormal   = normalize(uNormTrsf * occNormal.xyz);
  vColor    = uColor;
  gl_Position = occProjectionMatrix * occWorldViewMatrix * aPos;
}