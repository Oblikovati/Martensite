
layout(triangles, equal_spacing, ccw) in;
void main() {
  vec3 aPnt0 = gl_TessCoord.x * gl_in[0].gl_Position.xyz;
  vec3 aPnt1 = gl_TessCoord.y * gl_in[1].gl_Position.xyz;
  vec3 aPnt2 = gl_TessCoord.z * gl_in[2].gl_Position.xyz;
  gl_Position = vec4 (aPnt0 + aPnt1 + aPnt2, 1.0);
}