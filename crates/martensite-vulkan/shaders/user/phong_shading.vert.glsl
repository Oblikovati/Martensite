// Martensite per-pixel Blinn-Phong sample program, vertex stage: view-space position and
// normal for the fragment stage (the `vshaderprog phong` program of the viewer commands).

THE_SHADER_OUT vec3 View;          // direction towards the eye (orthographic approximation)
THE_SHADER_OUT vec3 Normal;        // view-space normal
THE_SHADER_OUT vec4 Position;      // view-space position
THE_SHADER_OUT vec4 PositionWorld; // world-space position

vec3 viewSpaceNormal (in vec3 theObjectNormal)
{
  vec4 aTurned = occWorldViewMatrixInverseTranspose
               * occModelWorldMatrixInverseTranspose
               * vec4 (theObjectNormal, 0.0);
  return normalize (aTurned.xyz);
}

void main()
{
  PositionWorld = occModelWorldMatrix * occVertex;
  Position      = occWorldViewMatrix * PositionWorld;
  Normal        = viewSpaceNormal (occNormal);
  View          = vec3 (0.0, 0.0, 1.0);
  gl_Position   = occProjectionMatrix * occWorldViewMatrix * occModelWorldMatrix * occVertex;
}
