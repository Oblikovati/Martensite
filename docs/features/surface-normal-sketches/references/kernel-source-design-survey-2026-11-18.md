# Local kernel source survey for wrapped normal profiles

## Recommended direction

Keep the shared `construct_mapped` operation and its paired rational depth rows.
Feed text through the existing glyph-to-planar-region path. Carry seam
occurrences and integer period branches explicitly. Reuse captured geometry by
owned source identity; certificate or receipt reuse also requires the exact
captured revision, options, tolerance and precision policy. These are the useful
design transfers from the inspected kernels; none calls for another emboss engine.

If measurement identifies candidate-pair search as a bottleneck, investigate
operation-local acceleration over conservative analytic bounds, followed by the
existing checked intersection leaves. Reuse Martensite's BVH machinery before
adding a spatial-search subsystem. A triangle BVH can suggest this separation
of responsibilities, but its bounds cannot prune analytic geometry without an
enclosure proof. The hot exact UV membership leaf needs its own measurements;
mesh-search source code gives no evidence that a BVH would accelerate it.

The concrete Martensite seams are the [shared incidence builder](../../../../crates/martensite-tksketch/src/wrapped_profile/native.rs#L35),
[glyph topology](../../../../crates/martensite-tkv3d/src/brep_font/planar_region.rs#L1),
[BVH tree](../../../../crates/martensite-tkmath/src/bvh/tree.rs#L1),
[patch traversal](../../../../crates/martensite-tkgeomalgo/src/int_patch/bvh_traversal.rs#L1)
and [observed self-interference checker](../../../../crates/martensite-tkbo/src/bop_algo/checker_si.rs#L105).
Preserve source-keyed incidence, cancellation, deterministic candidate order,
complete pair coverage and actual work accounting. Tessellation remains outside
modelling acceptance.

## Inspection scope and revisions

This consolidates three read-only source surveys made on 2026-11-18. No reference
kernel was built, run or benchmarked. The inspected routes did not supply a
completed certified cone-profile normal-extrusion operation with Martensite's
reference-body, storage, topology and measure contract. This is a result of the
bounded source search, not proof that no other branch, add-on or application has
such an operation. OCCT is a design reference, not an oracle or compatibility
requirement.

All external checkout roots below are under
`/home/vmiguel/git/oblikovati-workspace/`. File/line citations identify the local
sources inspected; pinned web links identify the same revisions where available.
The Martensite references identify HEAD
`7678044cbd694477b26be17d48e7f6aa7af06586` plus unfinished cone working changes.
Their links are relative to this repository, so they remain usable after the
temporary worktree is removed.

| Checkout | HEAD inspected | License evidence |
| --- | --- | --- |
| `OCCT` | `3d097a0328e71b826377d4814ab05ec3c3d23871` | Inspected headers: LGPL-2.1 with `OCCT_LGPL_EXCEPTION.txt`, or commercial terms; [normal projection header:6](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKOffset/BRepOffsetAPI/BRepOffsetAPI_NormalProjection.hxx:6). |
| `FreeCAD` | `406063b1116a6cb872bbd92ec27c6f425a8d1f98` | Inspected projection and ShapeString files: LGPL-2.1-or-later; [projection:1](/home/vmiguel/git/oblikovati-workspace/FreeCAD/src/Mod/Part/Gui/DlgProjectionOnSurface.cpp:1), [ShapeString:1](/home/vmiguel/git/oblikovati-workspace/FreeCAD/src/Mod/Draft/draftobjects/shapestring.py:1). |
| `shaper` | `6abe3ee131b3166ac8f8cb3f997f2e336362c3a7` | Inspected GeomAlgoAPI files: LGPL-2.1-or-later; [offset header:3](/home/vmiguel/git/oblikovati-workspace/shaper/src/GeomAlgoAPI/GeomAlgoAPI_Offset.cpp:3). |
| `solvespace` | `ef966335e7fbc617ba8e759149a567158b37aa7b` | Checkout carries GPL version 3; [COPYING:2](/home/vmiguel/git/oblikovati-workspace/solvespace/COPYING.txt:2). |
| `blender` | `b2b5bf0686bf9c1312d4a393b57a8f42bdce75bf` plus existing local modifications | All 13 inspected implementation files declare GPL-2.0-or-later; exact working-file hashes appear below. [COPYING:1](/home/vmiguel/git/oblikovati-workspace/blender/COPYING:1) points to the full GPL text. |
| `truck` | `88ed005249e5e3a6b07f62425399435905cd3ab6` | Apache-2.0; [geometry manifest:9](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-geometry/Cargo.toml#L9), root `LICENSE:1`. |
| `RustyCADKernel` | `398ac057daa4b6509cdd87898259979f66591a5e` | Workspace declares GPL-2.0-only; [manifest:28](https://github.com/Oblikovati/RustyCADKernel/blob/398ac057daa4b6509cdd87898259979f66591a5e/Cargo.toml#L28), root `LICENSE:1`. A separate `licenses/LICENSE_LGPL_21.txt` also exists; the workspace declaration does not classify every bundled file. |
| `solidcore` | `452bea425753cecfb8877df2d1ac18d005d7d81d` | Apache-2.0, NVIDIA copyright; [LICENSE:1](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/LICENSE#L1). |
| `usd-brep` | `3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5` | Apache-2.0, NVIDIA copyright; [LICENSE:1](https://github.com/NVIDIA-Omniverse/usd-brep/blob/3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5/LICENSE#L1). |

Truck and RustyCADKernel had clean status during inspection. Their main source
trees supplied the evidence; stale `.slim/worktrees` copies were excluded.
Cited working files were also compared with their HEAD blobs: all 16 cited
OCCT/FreeCAD/Shaper/SolveSpace files, eight Solidcore files and six usd-brep files
matched byte for byte and had no scoped working changes.
Blender's existing user changes were preserved. The other local checkouts were
read without modification. No code was copied and no dependency was added;
future source reuse needs a file-specific license and attribution review.

## Projection, offset and construction in OCCT-derived kernels

OCCT normal projection exposes separate 3D/2D tolerances, continuity, maximum
degree/segments, maximum distance, optional 3D reconstruction and generated-edge,
ancestor and support-face history. It uses a composite projected-curve adaptor
and `Approx_CurveOnSurface` outside an elementary reuse branch. The inspected
approximation branch rejects only when the reported 3D error exceeds 1000 times
the requested tolerance. Its approximation acceptance is unsuitable for
Martensite's requested-bound proof. Sources: [API:63](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKOffset/BRepOffsetAPI/BRepOffsetAPI_NormalProjection.hxx:63),
[projector:238](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKBool/BRepAlgo/BRepAlgo_NormalProjection.cxx:238),
[error rule:335](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKBool/BRepAlgo/BRepAlgo_NormalProjection.cxx:335),
[history:113](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKOffset/BRepOffsetAPI/BRepOffsetAPI_NormalProjection.hxx:113).

OCCT shell offsets have explicit limitations. `PerformBySimple` operates without
intersections; the joined route distinguishes adjacent-face intersections and
joining geometry. Its header calls the all-parallel intersection method
incomplete and self-intersection elimination unimplemented, and warns about
inside-out faces, nonadjacent intersections, C1 requirements and C0 B-spline
refusal. `IsDone()` therefore cannot replace an offset-range or solid-validity
certificate. Sources: [simple route:47](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKOffset/BRepOffsetAPI/BRepOffsetAPI_MakeOffsetShape.hxx:47),
[limitations:64](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKOffset/BRepOffsetAPI/BRepOffsetAPI_MakeOffsetShape.hxx:64).

FreeCAD's inspected projection GUI uses `BRepProj_Projection`, then extrudes
the projected face with `BRepPrimAPI_MakePrism` along a reversed, scaled
projection direction. Its Python normal-projection entry delegates to
`BRepAlgo_NormalProjection`. Shaper likewise translates a base, constructs a
constant-vector prism and records generation history; its projection helper
projects onto a plane. Its simple/joined offsets delegate to OCCT, with
intersection and self-intersection switches false in the joined path. These
are workflow/history examples for directional construction, not algorithms for
extrusion along the varying cone normal. Sources: [FreeCAD wire projection:532](/home/vmiguel/git/oblikovati-workspace/FreeCAD/src/Mod/Part/Gui/DlgProjectionOnSurface.cpp:532),
[extrusion:991](/home/vmiguel/git/oblikovati-workspace/FreeCAD/src/Mod/Part/Gui/DlgProjectionOnSurface.cpp:991),
[Python wrapper:1761](/home/vmiguel/git/oblikovati-workspace/FreeCAD/src/Mod/Part/App/TopoShapePyImp.cpp:1761),
[Shaper prism/history:279](/home/vmiguel/git/oblikovati-workspace/shaper/src/GeomAlgoAPI/GeomAlgoAPI_Prism.cpp:279),
[plane projection:41](/home/vmiguel/git/oblikovati-workspace/shaper/src/GeomAlgoAPI/GeomAlgoAPI_Projection.cpp:41),
[offset:79](/home/vmiguel/git/oblikovati-workspace/shaper/src/GeomAlgoAPI/GeomAlgoAPI_Offset.cpp:79).

SolveSpace's exact extrusion representation copies the input Bézier degree,
weights and translated control rows into a surface of degree one in the
extrusion direction. Shell construction handles outer/hole loops, orientation
against the sketch normal and shared trims, retaining exact translated boundary
curves alongside PWL points. Loop grouping uses a chord tolerance; its optional
self-intersection check uses polygonal loops. The two-row construction is useful
representation evidence, while the polygon checks have different acceptance
semantics. Sources: [surface rows:11](/home/vmiguel/git/oblikovati-workspace/solvespace/src/srf/surface.cpp:11),
[orientation/trims:17](/home/vmiguel/git/oblikovati-workspace/solvespace/src/srf/shell.cpp:17),
[exact curve and PWL:65](/home/vmiguel/git/oblikovati-workspace/solvespace/src/srf/shell.cpp:65),
[grouping/check:48](/home/vmiguel/git/oblikovati-workspace/solvespace/src/groupmesh.cpp:48).

## Rust geometry, ownership and reference formulas

Truck's `PCurve` evaluates its UV curve then the surface, using the chain rule
and shared derivative bundles. `RevolutedCurve` supplies explicit partials and
a periodic angular parameter of `2π`. Translation/rotation sweeps exist, and
the cone builder handles generator endpoints on the axis, including splitting
a single-edge generator. Its normal offset surface adds a normal field and
calculates derivatives from shared base-surface derivatives. Sources:
[pcurve:65](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-geometry/src/decorators/pcurve.rs#L65),
[revolution:61](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-geometry/src/decorators/revolved_curve.rs#L61),
[sweeps:484](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-modeling/src/builder.rs#L484),
[cone:696](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-modeling/src/builder.rs#L696),
[normal field:65](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-geometry/src/decorators/offset/surface.rs#L65).

Truck's `HomotopySurface` uses `C0(u)+(C1(u)-C0(u))*v`; conversion synchronizes
degree and knots once, then stores the two curves in two control rows.
`sub_connect_wires` keeps connector edges in an operation-local map keyed by
`VertexID`, so neighboring faces reuse them. Martensite already has the stronger
authored/dyadic `CornerSource` identity and source-keyed bindings. Reuse those
owners, rather than reconstructing identity from coordinates or proximity.
Sources: [ruled rows:170](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-geometry/src/decorators/homotopy.rs#L170),
[connector ownership:54](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-modeling/src/topo_impls.rs#L54).

Truck's inspected Boolean path obtains polygon-mesh interference, builds
polylines and refines against parametric surfaces with Newton solving and
numerical derivatives. It retains both surfaces' UV polylines beside the spatial
leader. Unknown-component classification also uses polygon signed crossings.
This preserves useful correspondence data, but does not supply Martensite's
full-interval pair/self certification. Sources: [intersection construction:151](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-shapeops/src/transversal/intersection_curve/mod.rs#L151),
[UV parameters:5](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-shapeops/src/transversal/intersection_curve/mod.rs#L5),
[Newton refinement:17](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-geometry/src/decorators/intersection_curve.rs#L17),
[classification:96](https://github.com/ricosjp/truck/blob/88ed005249e5e3a6b07f62425399435905cd3ab6/truck-shapeops/src/transversal/integrate/mod.rs#L96).

RustyCADKernel is an application/integration reference here: its current
workspace imports Martensite toolkits at
`d051ff699982faba4ec2df3ee9a47e14f38dedbd`, so its wrappers are not independent
confirmation of those algorithms. `text2brep` uses Martensite's font renderer;
`projpcurve` composes an edge-on-face pcurve with its surface; pick-ray handling
delegates to `IntCurvesFaceIntersector`, reads UV coordinates and explicitly
refuses an unfinished result. An integration control counts a cylinder seam
once. Sources: [dependencies:31](https://github.com/Oblikovati/RustyCADKernel/blob/398ac057daa4b6509cdd87898259979f66591a5e/Cargo.toml#L31),
[text:170](https://github.com/Oblikovati/RustyCADKernel/blob/398ac057daa4b6509cdd87898259979f66591a5e/crates/tkviewertest/src/viewer_test/object_commands/text2brep.rs#L170),
[projection:253](https://github.com/Oblikovati/RustyCADKernel/blob/398ac057daa4b6509cdd87898259979f66591a5e/crates/tktoptest/src/swdraw/shape_analysis/projection.rs#L253),
[picking:29](https://github.com/Oblikovati/RustyCADKernel/blob/398ac057daa4b6509cdd87898259979f66591a5e/crates/kernel/src/pick/face_hits.rs#L29),
[seam control:67](https://github.com/Oblikovati/RustyCADKernel/blob/398ac057daa4b6509cdd87898259979f66591a5e/crates/kernel/src/topology_tests.rs#L67).

Solidcore's `CrvOnSurf` owns a UV curve and B-spline surface, evaluates
`C(t)=S(uv(t))`, and retains knot-crossing and optional 3D-approximation caches.
Its maximum-distance helper samples mapped curves. `CurveCache` owns a private
curve copy, Bézier decomposition and binary box tree; it is dropped when the
curve changes, and remembers both successful and failed analytic-conversion
attempts. These are ownership/invalidation examples, not interval-certified
residuals. Sources: [curve on surface:283](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/crates/solidcore/src/curve/crv_on_surf.rs#L283),
[sampling:1424](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/crates/solidcore/src/curve/crv_on_surf.rs#L1424),
[cache:439](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/crates/solidcore/src/curve/curve_cache.rs#L439).

The inspected Solidcore Rust surface dispatch contains B-spline, curve-bounded
and plane kinds. Cone/cylinder units remain `todo` in its port plan. Selected
surface-intersection paths return `SolidError::Unported`, and the tracer records
unavailable cache-tree dependencies. `UnbendVolume` implements a cylindrical
shell-to-plane map, with clamped singularity tolerance, a substituted neutral
distance of `1.0` when too small, and recorded unported domain checks and inherited
boundary defects. These gaps and repairs prevent treating this checkout as a
complete independent cone/intersection oracle. Sources: [surface dispatch:235](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/crates/solidcore/src/surface/surface/p2.rs#L235),
[port plan:1644](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/plan/modules.toml#L1644),
[unported intersection:1730](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/crates/solidcore/src/intersect/surface_intersector/p1.rs#L1730),
[tracer:60](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/crates/solidcore/src/intersect/surface_tracer.rs#L60),
[unbend:204](https://github.com/bherbruck/solidcore/blob/452bea425753cecfb8877df2d1ac18d005d7d81d/crates/solidcore/src/surface/unbend_volume.rs#L204).

SMLib's `SmCone::EvaluateSTEP` computes shared radius, sine and cosine values
for analytic evaluation and derivative bundles. Its STEP U is degrees; V may be
axis height or generator parameter. Martensite's radian U/slant V convention
therefore needs explicit conversion. `CreateOffsetSurface` adjusts sign for
swapped UV/inside-out orientation, then uses radial shift `cos(alpha)*offset`
and axial shift `-sin(alpha)*offset`. It also trims/rebuilds near-apex cases and
can succeed with no output when both radii fall below tolerance. The meridian
formula is useful independent evidence; those topology rules do not define
Martensite's refusal contract. Sources: [evaluation:2093](https://github.com/NVIDIA-Omniverse/usd-brep/blob/3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5/source/SMLib/src/SmCone.cpp#L2093),
[offset:560](https://github.com/NVIDIA-Omniverse/usd-brep/blob/3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5/source/SMLib/src/SmCone.cpp#L560).

SMLib's UV-drop/3D-lift wrappers retain outputs locally until success, but use
heuristic tolerances: projection derives one from the curve box with a floor of
`1.0`; drop/lift use `10` and `0.001`. Seam correction tests actual point movement
and rejects an unsuitable candidate span; that helper explicitly does not split
seam crossings. Linear extrusion is vector-directed. Surface-intersection
span-accuracy tracing adjusts its target with tangent-plane angle, limits it to
`tol/16..tol`, and tests five points. Sources: [drop/lift:1108](https://github.com/NVIDIA-Omniverse/usd-brep/blob/3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5/source/SM_API/src/SmApiCurves.cpp#L1108),
[seam correction:1935](https://github.com/NVIDIA-Omniverse/usd-brep/blob/3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5/source/SMLib/src/SmSurfaceDropLib.cpp#L1935),
[linear sweep:267](https://github.com/NVIDIA-Omniverse/usd-brep/blob/3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5/source/SM_API/src/SmApiSurfaces.cpp#L267),
[intersection tracing:3548](https://github.com/NVIDIA-Omniverse/usd-brep/blob/3fd5b9548d19aa8c12c29bc9637a96d42a06f8a5/source/SMLib/src/SmSurfaceIntersector.cpp#L3548).

## Text, seams and mesh deformation

FreeCAD ShapeString converts text through `Part.makeWireString`, handles font
and file errors, and optionally creates faces. SolveSpace uses FreeType outline
callbacks for line, quadratic and cubic segments. Both support treating glyphs
as ordinary curve/loop input, which Martensite already provides through
`BRepFontPlanarRegion`. Sources: [ShapeString:162](/home/vmiguel/git/oblikovati-workspace/FreeCAD/src/Mod/Draft/draftobjects/shapestring.py:162),
[outline callbacks:338](/home/vmiguel/git/oblikovati-workspace/solvespace/src/ttf.cpp:338).

OCCT `FixSeam` obtains both pcurves and updates their order on the face.
`FixShifted` repairs whole-period pcurve shifts after 3D reconstruction.
Blender's cylindrical UV projection flood-fills faces, uses `atan2` and a linear
axial coordinate, shifts U by an integer selected with `ceil`, and stops branch
propagation at marked seam edges. Its result is a UV corner attribute. These
mechanisms motivate explicit occurrence and branch state; mutable repair or a
floating-point UV winding rule does not establish BRep chart coverage. Sources:
[OCCT seam:1615](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKShHealing/ShapeFix/ShapeFix_Wire.cxx:1615),
[period shifts:1643](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKShHealing/ShapeFix/ShapeFix_Wire.cxx:1643),
[Blender UV flood fill:4045](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/editors/uvedit/uvedit_unwrap_ops.cc:4045),
[UV branch:4101](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/editors/uvedit/uvedit_unwrap_ops.cc:4101),
[projection primitive:5146](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenlib/intern/math_geom.cc:5146).
Martensite already has [seam repair](../../../../crates/martensite-tkshhealing/src/shape_fix/wire_seam.rs#L1);
healing remains explicit on a copy with identity remapping.

Blender curve deformation derives a path parameter from an axis/bounds or path
length, reads position/direction/quaternion/radius from a cached path, rotates a
transverse offset and optionally scales it by radius. Path evaluation searches
accumulated lengths and interpolates neighboring samples. Text-on-curve layout
uses each character's layout offset to choose path time, clamps it, and evaluates
location and orientation near the character center. This is useful placement
and preview machinery; character placement alone does not wrap every point of
the outline. Sources: [curve deformation:64](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/curve_deform.cc:64),
[path:280](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/anim_path.cc:280),
[text layout:1490](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/vfont_curve.cc:1490).

Blender Simple Deform Bend evaluates `theta = coordinate * factor` with
sine/cosine coordinates around radius `1/factor`, axis permutations and
corrections beyond limits; its source notes floating-point operation-order
sensitivity. Taper scales transverse coordinates by `1 + z*factor`. These
formulas do not establish a developed-sector-to-cone isometry. Mesh Deform
instead builds harmonic cage coordinates on a finite grid: cage-triangle BVH
queries tag cells, per-cage-vertex systems supply weights, and evaluation applies
weighted cage displacements or interpolated grid influences. Sources:
[bend:140](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/modifiers/intern/MOD_simpledeform.cc:140),
[taper:89](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/modifiers/intern/MOD_simpledeform.cc:89),
[harmonic binding:835](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/editors/armature/meshlaplacian.cc:835),
[grid setup:1573](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/editors/armature/meshlaplacian.cc:1573),
[systems:1445](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/editors/armature/meshlaplacian.cc:1445),
[evaluation:239](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/modifiers/intern/MOD_meshdeform.cc:239),
[dynamic influences:144](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/modifiers/intern/MOD_meshdeform.cc:144).

Shrinkwrap nearest-surface mode transforms vertices into target space, searches
the target mesh, applies snap distance, transforms back and blends by vertex
weight. A previous hit can seed the next search bound; target-normal mode
disables that heuristic. Projection raycasts through the BVH. Target-normal
triangle projection solves barycentric weights and signed distance along an
interpolated normal with a magnitude-scaled `1e-6` epsilon and a 20-step Newton
solve; failure can fall back to ordinary nearest-point search. These are
per-vertex numerical mesh algorithms, with no intervening edge/face enclosure.
Sources: [nearest vertex:292](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/shrinkwrap.cc:292),
[nearest surface:1030](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/shrinkwrap.cc:1030),
[ray projection:349](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/shrinkwrap.cc:349),
[target-normal solve:759](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/shrinkwrap.cc:759),
[fallback:989](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/shrinkwrap.cc:989).

Solidify derives two offsets from thickness/offset factor, duplicates topology,
reverses copied winding and moves vertices along normals, optionally clamping
using mesh lengths/angles. BMesh solidify reverses originals, extrudes a region
and pushes new vertices using computed normals; region extrusion constructs
boundary faces with explicit incidence orientation. These are useful winding
examples, but mesh clamping does not supply the decoded analytic row/use checks
required by #540. Sources: [offsets:187](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/modifiers/intern/MOD_solidify_extrude.cc:187),
[copied winding:355](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/modifiers/intern/MOD_solidify_extrude.cc:355),
[BMesh solidify:839](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/bmesh/operators/bmo_extrude.cc:839),
[boundary faces:471](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/bmesh/operators/bmo_extrude.cc:471).

## Spatial search and the existing validation path

Blender builds and caches balanced triangle BVHs. BMesh intersection aliases
the two trees for self-intersection, enumerates overlapping bounds, then runs
triangle-pair intersection. Shared-edge/vertex handling is explicit. This is
the concrete candidate-generation/checked-leaf pattern behind the conditional
acceleration recommendation above. Sources: [triangle bounds:508](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/bvhutils.cc:508),
[cached access:773](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/blenkernel/intern/bvhutils.cc:773),
[overlap traversal:1048](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/bmesh/tools/bmesh_intersect.cc:1048),
[triangle pairs:1100](/home/vmiguel/git/oblikovati-workspace/blender/source/blender/bmesh/tools/bmesh_intersect.cc:1100).

OCCT's self-interference checker runs PaveFiller, face self-checks and solid
checks. Its surface policy excludes planes, cylinders, cones and spheres from
self-face intersection; candidate surfaces call `IntTools_FaceFace` against
themselves. Martensite already has this checker structure and reports its
observed completion and native-search limitations. Importing the reference
implementation would duplicate it. Sources: [OCCT program:151](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKBO/BOPAlgo/BOPAlgo_CheckerSI.cxx:151),
[self-face call:76](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKBO/BOPAlgo/BOPAlgo_CheckerSI.cxx:76),
[surface policy:441](/home/vmiguel/git/oblikovati-workspace/OCCT/src/ModelingAlgorithms/TKBO/BOPAlgo/BOPAlgo_CheckerSI.cxx:441),
[Martensite coverage adapter](../../../../crates/martensite-tkbo/src/body_check/native.rs#L27).

Other existing reuse candidates are [normal projection](../../../../crates/martensite-tkoffset/src/brep_offset_api/normal_projection.rs#L1),
its [approximation owner](../../../../crates/martensite-tkbool/src/brep_algo/normal_projection/edge.rs#L1),
the [prism builder](../../../../crates/martensite-tkprim/src/brep_prim_api/make_prism.rs#L1),
[offset builder](../../../../crates/martensite-tkoffset/src/brep_offset_api/make_offset_shape.rs#L1)
and [point-on-surface projection](../../../../crates/martensite-tkgeomalgo/src/geom_api/project_point_on_surf.rs#L1).
Keep proposals separate from [decoded coefficient capture](../../../../crates/martensite-tksketch/src/wrapped_profile/operation/cone/storage/capture.rs#L147)
and [actual planar-pair execution](../../../../crates/martensite-tkbo/src/sketch_region_certification/bernstein_execution.rs#L39).
Reuse requires the operation's source snapshots and policies, not merely similar
geometry. Cache ownership and shared derivative bundles are design leads, not
measured speedups.

No remote PR was opened for #542. Its local `cargo xtask pr` gate found a real
legacy complete-owned callback-trace regression (8238 versus 8268)
and was interrupted after that failure was confirmed. The #542 default shortcut
will not ship. Remediation preserves the original eager legacy sign1 path and
restricts the shortcut to explicit #544 sign2. No measured end-to-end speedup or
five-second acceptance result is established here. These source inspections
provide no runtime or memory comparison and change no acceptance requirement.

## Exact Blender working-file provenance

HEAD alone does not identify Blender's inspected dirty working tree. The paths
below are relative to `/home/vmiguel/git/oblikovati-workspace/blender`; all 13
files declare `GPL-2.0-or-later`. These SHA256 values identify the bytes used for
the source claims above. Existing user modifications were left untouched.

| File | SHA256 inspected |
| --- | --- |
| `source/blender/blenkernel/intern/shrinkwrap.cc` | `e76467b656dcdee749ec2403c7e26e2ad88ab76e0b919e918f1be09ebb281797` |
| `source/blender/blenkernel/intern/curve_deform.cc` | `8e6f2ff70dc66d5ec32670b0911be0456b36536c5f703bf8bd48943555edb15e` |
| `source/blender/blenkernel/intern/anim_path.cc` | `9b478085a994cbde3174c409ca10790f472e3d2c26eae2836056df8f3c189bc9` |
| `source/blender/blenkernel/intern/vfont_curve.cc` | `0cfedb02a348f320b249e8d6cf80d3a304df175038cf612ae5c866ec31880e7c` |
| `source/blender/modifiers/intern/MOD_simpledeform.cc` | `74bf98dbbd0d307581c30fb067d80440a8000229bf939387cc81bcd9e2296dcf` |
| `source/blender/modifiers/intern/MOD_meshdeform.cc` | `e9c0683f3f58c7c5648a12cf825b86f77d404068845e3d1bfb6766dc530175d3` |
| `source/blender/editors/armature/meshlaplacian.cc` | `0a8d9f46006ae9c7c25fd34da11355789894a66caf03c4313bf4db4c3ef4d6d7` |
| `source/blender/modifiers/intern/MOD_solidify_extrude.cc` | `d2eb4b57c8b43656108b1d699da4bda23102359688aac3c8010ccc7ca3281e61` |
| `source/blender/bmesh/operators/bmo_extrude.cc` | `3fee6b21c48f29e5f37316b7f8d43180c86a30170f55e888c842453aa3bea665` |
| `source/blender/editors/uvedit/uvedit_unwrap_ops.cc` | `cf1af3518ac14b1a89a1de4937c2a7a5f5ab6cea53bf39e8ca837329b12cfbf6` |
| `source/blender/blenlib/intern/math_geom.cc` | `68114c7b9742363686ab606c54791b2d745df3cb8c339aaac057397fd3c0c419` |
| `source/blender/blenkernel/intern/bvhutils.cc` | `29763a770994271297e005fce937b54b77b60bc5b33e0e811b474fe89ca12a8a` |
| `source/blender/bmesh/tools/bmesh_intersect.cc` | `f6f3b51cd685e23d3922f47b4f3778b3791392874fe3c36148427763f623d4db` |
