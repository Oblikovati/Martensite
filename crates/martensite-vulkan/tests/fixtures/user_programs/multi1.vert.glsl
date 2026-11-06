
THE_SHADER_OUT vec2 TexCoord;
void main() {
  TexCoord    = occTexCoord.st;
  gl_Position = occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix * occVertex;
}