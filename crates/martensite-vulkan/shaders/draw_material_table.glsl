layout(set=7,binding=1,std430) readonly buffer DrawMaterials {
    uint words[];
} drawMaterials;

#ifdef MARTENSITE_FRAGMENT
layout(location=3) flat in uint drawMaterialOffset;
#else
layout(location=3) flat out uint drawMaterialOffset;
#endif

vec4 materialCoefficient(uint slot) {
    uint word = drawMaterialOffset + slot * 4u;
    return uintBitsToFloat(uvec4(drawMaterials.words[word], drawMaterials.words[word + 1u],
        drawMaterials.words[word + 2u], drawMaterials.words[word + 3u]));
}
