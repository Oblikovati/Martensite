// Shared by the stock and directional illumination paths of Quantized16OctV2.
vec4 decodeMeshPosition()
{
    uvec3 code = uvec3(packedPosition.x & 65535u, packedPosition.x >> 16, packedPosition.y);
    // The part bounds were qualified against a binary32 multiply followed by an add.
    precise vec3 displacement = vec3(code) * transform.positionStep.xyz;
    precise vec3 decoded = displacement + transform.positionOrigin.xyz;
    return vec4(decoded, 1.0);
}

vec3 decodeMeshNormal()
{
    vec2 folded = unpackSnorm2x16(packedNormal);
    vec3 octahedral = vec3(folded, 1.0 - abs(folded.x) - abs(folded.y));
    float fold = max(-octahedral.z, 0.0);
    octahedral.xy += mix(vec2(fold), vec2(-fold), greaterThanEqual(folded, vec2(0.0)));
    // The vertex stage normalizes after applying the normal matrix.
    return octahedral;
}
