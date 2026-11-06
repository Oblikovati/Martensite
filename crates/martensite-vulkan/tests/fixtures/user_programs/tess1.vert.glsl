
out vec4 VertColor;
void main() {
  VertColor = occColor;
  gl_Position = occVertex;
}