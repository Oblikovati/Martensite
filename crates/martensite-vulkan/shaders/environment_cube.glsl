// THE_FUNC_cubemap_vector_transform of Graphic3dShaderManager: maps a
// Graphic3d direction to the cube map's lookup space; theYCoeff is +1 for top-down images and
// theZCoeff -1 for Z-inverted maps (Graphic3dCubeMap::IsTopDown, ZIsInverted).
vec3 cubemapVectorTransform (in vec3 theVector,
                             in int  theYCoeff,
                             in int  theZCoeff)
{
  theVector = theVector.yzx;
  theVector.y *= float(theYCoeff);
  theVector.z *= float(theZCoeff);
  return theVector;
}
