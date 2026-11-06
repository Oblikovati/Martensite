// Visual phase uses logical window coordinates; attachment fetches must keep physical XY.
#ifdef MARTENSITE_TOP_LEFT_LOGICAL_COORDINATES
#ifndef MARTENSITE_LOGICAL_VISUAL_COORDINATES_GLSL
#define MARTENSITE_LOGICAL_VISUAL_COORDINATES_GLSL
float logicalVisualWindowY(float physicalY, float frameHeight) {
    // Reflect pixel centers without introducing a one-pixel phase shift. A frame of odd height
    // starts one attachment row down (derivative quads pair rows as OpenGL), so its attachment
    // is one row taller than the logical frame.
    return frameHeight + mod(frameHeight, 2.0) - physicalY;
}
#endif
#endif
