// PBRDistribution.glsl (GGX normal distribution), shared by the PBR lighting
// and the PBR environment baking stages (Graphic3dShaderManager::getPBREnvBakingProgram).
#define INV_PI 0.318309886
float occPBRDistribution(in float theCosH, in float theRoughness) {
    float aDistribution = theRoughness * theRoughness;
    aDistribution = aDistribution / (theCosH * theCosH * (aDistribution * aDistribution - 1.0) + 1.0);
    aDistribution = INV_PI * aDistribution * aDistribution;
    return aDistribution;
}
