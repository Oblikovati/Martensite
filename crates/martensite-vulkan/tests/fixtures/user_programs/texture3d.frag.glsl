
#ifdef GL_ES
  uniform mediump sampler3D occSampler0;
#else
  uniform sampler3D occSampler0;
#endif
THE_SHADER_IN vec3 TexCoord;
void main() {
  occFragColor = occTexture3D(occSampler0, TexCoord);
}