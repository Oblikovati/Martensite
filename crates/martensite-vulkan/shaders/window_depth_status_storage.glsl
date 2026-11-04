#ifndef MARTENSITE_WINDOW_DEPTH_STATUS_STORAGE
#define MARTENSITE_WINDOW_DEPTH_STATUS_STORAGE
layout(set=7,binding=0,std430) buffer WindowDepthStatus {
    uint triangleCount; uint schema; uint invocations; uint fault; uint words[];
} windowDepthStatus;
const uint WINDOW_DEPTH_SCHEMA=0x57445633u;
const uint WINDOW_DEPTH_DONE=1u;
#endif
