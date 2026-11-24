bool occludedByPyramid(Candidate candidate) {
    vec3 lower;
    vec3 upper;
    if (!ndcBounds(candidate, lower, upper)) {
        return false;
    }
    vec2 start = vec2(uintBitsToFloat(pass.viewportX), uintBitsToFloat(pass.viewportY));
    vec2 size = vec2(uintBitsToFloat(pass.viewportWidth), uintBitsToFloat(pass.viewportHeight));
    vec2 firstEdge = start + ((lower.xy + 1.0) * 0.5) * size;
    vec2 secondEdge = start + ((upper.xy + 1.0) * 0.5) * size;
    vec2 limit = vec2(pass.pyramidWidth - 1u, pass.pyramidHeight - 1u);
    uvec2 first = uvec2(clamp(floor(min(firstEdge, secondEdge)) - 1.0, vec2(0.0), limit));
    uvec2 last = uvec2(clamp(ceil(max(firstEdge, secondEdge)) + 1.0, vec2(0.0), limit));
    uvec2 span = last - first;
    // A level whose cells are at least as wide as the query can intersect at most four cells.
    uint shift = uint(findMSB(max(max(span.x, span.y), 2u) - 1u)) + 1u;
    uint offset = floatBitsToUint(depthValues[shift - 1u]);
    uint rowWords = ((pass.pyramidWidth - 1u) >> shift) + 1u;
    first >>= shift;
    last >>= shift;
    uint top = offset + first.y * rowWords;
    uint bottom = offset + last.y * rowWords;
    float farthest = max(max(depthValues[top + first.x], depthValues[top + last.x]),
                         max(depthValues[bottom + first.x], depthValues[bottom + last.x]));
    // A clear pixel is 1. Strictly farther only, including depth-format quantization.
    float nearest = lower.z * 0.5 + 0.5;
    return farthest < 1.0 && nearest > farthest + 0.000002;
}
