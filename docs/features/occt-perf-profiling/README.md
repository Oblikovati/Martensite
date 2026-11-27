# OCCT vs Martensite Toolkit Performance Profiling (p95)

Systematic toolkit-by-toolkit p95 profiling and optimization against OCCT 8.0.0 Release (`/git/oblikovati-workspace/occt-install-vis`), proceeding in topological dependency order from foundation toolkits upward.

## Dependency Order & Progress

| Tier | Toolkit | OCCT Build | Status | Report |
| ---: | --- | --- | --- | --- |
| 1 | `martensite-tkernel` | prebuilt | **Done** (28 APIs; 20/28 parity or up to 38x faster) | [tkernel.md](tkernel.md) |
| 2 | `martensite-tkmath` | prebuilt | **Done** (24 APIs; up to 18.7x speedup) | [tkmath.md](tkmath.md) |
| 2 | `martensite-tkexpress` | prebuilt | Pending | |
| 3 | `martensite-tkg2d` | prebuilt | **Done** (12 APIs; up to 2.54x speedup) | [tkg2d.md](tkg2d.md) |
| 3 | `martensite-tkservice` | prebuilt/built | **Done** (**3.23x speedup** on `Graphic3dCamera::project` → 2.60x M/O, 1.27x M/O on camera matrices) | [tkaf_tkservice.md](tkaf_tkservice.md) |
| 4 | `martensite-tkg3d` | prebuilt | **Done** (12 APIs; up to 1.44x speedup on B-spline surfaces) | [tkg3d.md](tkg3d.md) |
| 5 | `martensite-tkgeombase` | prebuilt | **Done** (10 APIs; up to 1.40x speedup across BndLib, GCPnts, GeomConvert, ExtremaPC) | [tkgeombase.md](tkgeombase.md) |
| 6 | `martensite-tkbrep` | prebuilt | **Done** (10 APIs; up to 2.85x speedup on TopExp_Explorer, BRepTools, BRep_Tool) | [tkbrep.md](tkbrep.md) |
| 7 | `martensite-tkgeomalgo` | prebuilt | **Done** (9 APIs; up to 1.19x speedup via zero-alloc BSplSLib cache & stack basis matrices) | [tkgeomalgo.md](tkgeomalgo.md) |
| 8 | `martensite-tktopalgo` | prebuilt | **Done** (8 APIs; 1.28x speedup on Gauss-Kronrod VolumeProperties, 1.10x on BRepBndLib) | [tktopalgo.md](tktopalgo.md) |
| 9 | `martensite-tkprim`, `martensite-tkmesh` | prebuilt | **Done** (11 APIs; up to 1.26x speedup via lock-free MeshLinkKey, batched node reads, zero-copy CellFilter) | [tkprim_tkmesh.md](tkprim_tkmesh.md) |
| 9 | `martensite-tkshhealing`, `martensite-tkhlr` | prebuilt | **Done** (7 APIs; up to 1.76x speedup via lazy ReShape::empty_copied & single-lock TEdge copy) | [tkshhealing_tkhlr.md](tkshhealing_tkhlr.md) |
| 9 | `martensite-tkhelix` | prebuilt | **Done** (paired with `tkbool`, `tkfeat`; `sincos` optimization) | [tkbool_tkfeat_tkhelix.md](tkbool_tkfeat_tkhelix.md) |
| 10 | `martensite-tkbo` | prebuilt | **Done** (8 APIs; up to 1.34x speedup on Fuse/Cut/Common/Section via stack Bernstein/LeastSquare, inlined PLib/VectorBase, zero-alloc empty FailureText) | [tkbo.md](tkbo.md) |
| 11 | `martensite-tkfillet`, `martensite-tkoffset` | prebuilt | **Done** (6 APIs; **5.49x speedup** on ThruSections loft via zero-clone lazy-projection `curve_on_surface_by_index`, 1.39x on cylinder fillet) | [tkfillet_tkoffset.md](tkfillet_tkoffset.md) |
| 12 | `martensite-tkbool`, `martensite-tkfeat` | prebuilt | **Done** (5 APIs; 2.56–3.11x M/O across Pipe, NormalProjection, MakePrism boss, Helix) | [tkbool_tkfeat_tkhelix.md](tkbool_tkfeat_tkhelix.md) |
| 12+ | `martensite-tkexpress`, `martensite-tkservice`, `martensite-tkv3d`, `martensite-tkxmesh`, `martensite-tkmeshvs` | prebuilt | Pending | |
| DE | `martensite-tkde`, `martensite-tkxsbase`, `martensite-tkdestl`, `martensite-tkdestep`, `martensite-tkdeiges` | built from `../OCCT` | **Done** (8 APIs; **5.32x speedup** on ASCII STL write → **0.66x M/O FASTER than OCCT**, 1.45x on binary STL read) | [tkde.md](tkde.md) |
| AF/Vis | `martensite-tkcdf`, `martensite-tklcaf`, `martensite-tkservice` | built from `../OCCT` | **Done** (4 APIs; **3.23x speedup** on `Graphic3dCamera::project`, 1.17x on `TDF::lookup_500_labels`) | [tkaf_tkservice.md](tkaf_tkservice.md) |
