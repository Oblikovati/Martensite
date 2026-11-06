
uniform vec3 uLightDir;
THE_SHADER_IN vec4 vColor;
THE_SHADER_IN vec3 vNormal;
void main() {
  float aNdotL = max(dot(normalize(vNormal), normalize(uLightDir)), 0.2);
  occSetFragColor(vec4(vColor.rgb * aNdotL, vColor.a));
}