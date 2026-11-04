// Uniforms resources/Shaders/PBREnvBaking.{vs,fs} as one push-constant block
// (GlPBREnvironment::processSpecIBLMap/processDiffIBLMap set them per draw).
layout(push_constant) uniform PbrEnvBakingConstants {
  int uCurrentSide;          //!< current side of cubemap
  int uYCoeff;               //!< coefficient of Y controlling horizontal flip of cubemap
  int uZCoeff;               //!< coefficient of Z controlling vertical flip of cubemap
  int uSamplesNum;           //!< number of samples in Monte-Carlo integration
  int uCurrentLevel;         //!< current level of specular IBL map
  int occNbSpecIBLLevels;    //!< number of levels of the specular IBL map
  float uEnvSolidAngleSource; //!< solid angle of one texel of the source map's zero level
} bake;
