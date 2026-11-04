
layout(triangles) in;
layout(triangle_strip, max_vertices=3) out;
in vec4 VertColor[3];
out vec4 Color;
void main() {
  mat4 aMat = occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix;
  vec3 aCenter = vec3 (0.0, 0.0, 0.0);
  for (int aTriVertIter = 0; aTriVertIter < 3; ++aTriVertIter) {
    aCenter += gl_in[aTriVertIter].gl_Position.xyz;
  }
  aCenter /= 3.0;
  for (int aTriVertIter = 0; aTriVertIter < 3; ++aTriVertIter) {
    vec3 aVec = gl_in[aTriVertIter].gl_Position.xyz - aCenter;
    vec3 aVertRes = aCenter + normalize (aVec) * length (aVec) * 0.75;
    gl_Position = aMat * vec4 (aVertRes, 1.0);
    Color = VertColor[aTriVertIter] * 2.0;
    EmitVertex();
  }
  EndPrimitive();
}