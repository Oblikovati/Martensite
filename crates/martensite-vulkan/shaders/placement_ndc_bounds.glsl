// Bounds are expanded before viewport conversion. Invalid or near-plane-crossing boxes remain visible.
bool ndcBounds(Candidate candidate, out vec3 lower, out vec3 upper) {
    if (candidate.minimum.w == 0.0) {
        return false;
    }
    lower = vec3(3.402823e38);
    upper = vec3(-3.402823e38);
    mat4 clip = placements[candidate.placement].clip;
    for (uint corner = 0u; corner < 8u; corner++) {
        bvec3 top = notEqual(uvec3(corner) & uvec3(1u, 2u, 4u), uvec3(0u));
        vec4 local = vec4(mix(candidate.minimum.xyz, candidate.maximum.xyz, top), 1.0);
        vec4 point = clip * local;
        vec4 magnitude = abs(clip[0] * local.x) + abs(clip[1] * local.y)
                       + abs(clip[2] * local.z) + abs(clip[3]);
        vec4 margin = magnitude * 0.000002 + vec4(1e-35);
        vec4 lo = point - margin;
        vec4 hi = point + margin;
        if (any(isnan(lo)) || any(isinf(lo)) || any(isnan(hi)) || any(isinf(hi))
            || lo.w <= 0.0 || lo.z <= -lo.w) {
            return false;
        }
        vec3 positive = min(min(lo.xyz / lo.w, lo.xyz / hi.w), min(hi.xyz / lo.w, hi.xyz / hi.w));
        vec3 negative = max(max(lo.xyz / lo.w, lo.xyz / hi.w), max(hi.xyz / lo.w, hi.xyz / hi.w));
        if (any(isinf(positive)) || any(isinf(negative))
            || any(isnan(positive)) || any(isnan(negative))) {
            return false;
        }
        vec3 rounding = max(abs(positive), abs(negative)) * 0.000002 + vec3(0.000002);
        lower = min(lower, positive - rounding);
        upper = max(upper, negative + rounding);
    }
    return true;
}

