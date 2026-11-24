// Draw tables (#594, instancing.md step 3): a run of plain lit surface draws records as one
// indirect draw, and each draw reads the matrices it would push or bind from the frame's table
// at its first instance. The values are the bytes of the push constants and the lighting block,
// so the products and the captures are the same as for a single draw.
#if defined(MARTENSITE_FIXED_LIGHTING) || defined(MARTENSITE_FACTORIZED_TRIANGLES_V2) || defined(MARTENSITE_MARKER_LIGHTING) || defined(MARTENSITE_LINE_LIGHTING)
#error a draw table serves only plain precomposed lit surfaces
#endif
struct DrawTableEntry {
    mat4 clipFromObject;
    mat4 worldFromObject;
    mat4 normalFromObject;
};
layout(set=7,binding=0,std430) readonly buffer DrawTable { DrawTableEntry entries[]; } drawTable;
#define DRAW_CLIP_FROM_OBJECT drawTable.entries[gl_InstanceIndex].clipFromObject
#define DRAW_WORLD_FROM_OBJECT drawTable.entries[gl_InstanceIndex].worldFromObject
#define DRAW_NORMAL_FROM_OBJECT drawTable.entries[gl_InstanceIndex].normalFromObject
