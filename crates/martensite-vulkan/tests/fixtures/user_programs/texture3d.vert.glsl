
THE_SHADER_OUT vec3 TexCoord;
uniform float uSlice;
void main() {
  float aNbSlices = 6.0;
  TexCoord = occVertex.xyz;
  if (uSlice >= 0.0) { TexCoord = vec3(occTexCoord.xy, uSlice * 1.0 / aNbSlices + 0.5 / aNbSlices); }
  if (occTextureTrsf_Scale().y < 0.0) { TexCoord.y = 1.0 - TexCoord.y; }
  gl_Position = occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix * occVertex;
}