//! CPU transform bits from retained B1 submitted geometry, source primitives1495/1595/1496.
// Source .witness SHA and raw input binding: the recorded witness.
pub(super) const EMPTY_CLIP_TRIANGLES: [[[u32; 3]; 3]; 3] = [
    [
        [3204550651, 1055134992, 0],
        [3204546180, 1055120595, 0],
        [3204540855, 1055107392, 0],
    ],
    [
        [3196698888, 1024655240, 0],
        [3196685472, 1024482472, 0],
        [3196669498, 1024324032, 0],
    ],
    [
        [3171094720, 3205390967, 0],
        [3170951632, 3205405363, 0],
        [3170668672, 3205418567, 0],
    ],
];

// Retained submitted clip positions from the V3 triangle-753 domain refusal;
// its nonzero exact area collapses in the legacy f32 window-area operation.
pub(super) const V3_DOMAIN_EMPTY_TRIANGLE: [[u32; 3]; 3] = [
    [1046639979, 1059859441, 1031895835],
    [1046639976, 1059859413, 1031895943],
    [1046639978, 1059859495, 1031895888],
];
