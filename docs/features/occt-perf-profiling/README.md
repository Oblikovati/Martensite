# OCCT vs Martensite Toolkit Performance Profiling (p95)

Systematic toolkit-by-toolkit p95 profiling and optimization against OCCT 8.0.0 Release (`/git/oblikovati-workspace/occt-install-vis`), proceeding in topological dependency order from foundation toolkits upward.

## Dependency Order & Progress

| Tier | Toolkit | OCCT Build | Status | Report |
| ---: | --- | --- | --- | --- |
| 1 | `martensite-tkernel` | prebuilt | **Done** (28 APIs; 20/28 parity or up to 38x faster) | [tkernel.md](tkernel.md) |
| 2 | `martensite-tkmath` | prebuilt | **Done** (24 APIs; up to 18.7x speedup) | [tkmath.md](tkmath.md) |
| 2 | `martensite-tkexpress` | prebuilt | Pending | |
| 3 | `martensite-tkg2d` | prebuilt | **Done** (12 APIs; up to 2.54x speedup) | [tkg2d.md](tkg2d.md) |
| 3 | `martensite-tkservice` | prebuilt | Pending | |
| 4 | `martensite-tkg3d` | prebuilt | **Done** (12 APIs; up to 1.44x speedup on B-spline surfaces) | [tkg3d.md](tkg3d.md) |
| 5 | `martensite-tkgeombase` | prebuilt | **Done** (10 APIs; up to 1.40x speedup across BndLib, GCPnts, GeomConvert, ExtremaPC) | [tkgeombase.md](tkgeombase.md) |
| 6 | `martensite-tkbrep` | prebuilt | **Done** (10 APIs; up to 2.85x speedup on TopExp_Explorer, BRepTools, BRep_Tool) | [tkbrep.md](tkbrep.md) |
| 7 | `martensite-tkgeomalgo` | prebuilt | **Done** (9 APIs; up to 1.19x speedup via zero-alloc BSplSLib cache & stack basis matrices) | [tkgeomalgo.md](tkgeomalgo.md) |
| 8 | `martensite-tktopalgo` | prebuilt | Pending | |
| 9 | `martensite-tkhelix`, `martensite-tkhlr`, `martensite-tkprim`, `martensite-tkshhealing`, `martensite-tkmesh` | prebuilt | Pending | |
| 10+ | `martensite-tkbo`, `martensite-tkbool`, `martensite-tkfeat`, `martensite-tkfillet`, `martensite-tkoffset`, `martensite-tkv3d`, `martensite-tkxmesh`, `martensite-tkmeshvs` | prebuilt | Pending | |
| AF/DE | `tkcdf`, `tklcaf`, `tktobj`, `tkcaf`, `tkvcaf`, `tkxcaf`, `tkde`, `tkxsbase`, `tkrwmesh`, `tkdestl`, `tkdeiges`, `tkdestep`, `tkdeobj`, `tkdeply` | build from `../OCCT` | Pending | |
