
uniform sampler2D occSampler1;
uniform sampler2D occSampler2;
uniform sampler2D occSampler3;
THE_SHADER_IN vec2 TexCoord;
void main() {
  if      (TexCoord.s <  0.5 && TexCoord.t <  0.5) { occFragColor = occTexture2D(occSampler0, TexCoord.st); }
  else if (TexCoord.s <  0.5 && TexCoord.t >= 0.5) { occFragColor = occTexture2D(occSampler1, TexCoord.st); }
  else if (TexCoord.s >= 0.5 && TexCoord.t <  0.5) { occFragColor = occTexture2D(occSampler2, TexCoord.st); }
  else                                             { occFragColor = occTexture2D(occSampler3, TexCoord.st); }
}