# Changelog

All notable changes to this project are documented in this file. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.14.0] - 2026-11-24

### Added

- Added `Graphic3dCView::structure_update` and `Graphic3dStructureManager::update_structure`. A structure or group update now also gives each view the layer of the updated structure, or every layer when other structures connect it. The update invalidates the same bounding boxes as before (#594).
- Added `graphic3d_cview_invalidate_z_layer_bounding_box`, the base body of `Graphic3dCView::invalidate_z_layer_bounding_box` for a view that overrides it (#594).
- Added `Graphic3dCamera::un_projection`, which returns the inverse matrices of `un_project` for many points, and `SelectMgrFrustumBuilder::view_plane_projector`, which reads the builder and these matrices once per selecting frustum. A pick inverts the camera matrices once instead of ten times, and its points do not change by one bit (#594).

- Added `transform_utils::un_project_inverted`, which unprojects a point through an already inverted matrix with the same arithmetic as `un_project`. Vulkan text inverts its projection once per label instead of once per point (#594).

- Added `Graphic3dText::revision`, the number of edits made to a text. Vulkan keeps the draws of each text label and lowers a label again only after its text, aspects, font or resolution change; `frame_text_1k` takes 1.23 ms instead of 1.86 ms on the same host (#594).
- Added `Graphic3dLayer::structure_handles`, which lists a layer's structures in draw order without copying its maps. Frustum culling no longer copies the layer's BVH tree on each frame and locks each structure once to update its culled flag; `frame_transparent_1k` takes 0.59 ms instead of 0.62 ms on the same host (#594).

- Added `RevisedState` to tkservice: a shared cell whose writes advance a revision. A renderer reads the revision to know that the state did not change since it last read it (#594).

- Added `graphic3d_cview_min_max_values_with_size`, which computes `MinMaxValues` with a window size the view supplies. The Vulkan view passes the size it keeps between resizes, so auto Z-fit no longer asks the X server for the window size on every redraw (#594).

- Added `Graphic3dRenderDriver::finish_frames`, which waits for the frames that Vulkan redraws left on the GPU, as `glFinish` does, and returns the first GPU fault they reported (#594).

- Added `Graphic3dCamera::orientation_with_axial_scale_f` to read the float view matrix and its axial scale together. Vulkan uses it to avoid copying the camera for each placement (#594).

- Added Vulkan redraw counters for buffers, images, memory allocations, pipelines, descriptor sets, submissions, bytes uploaded and read back, and CPU time by phase. `Graphic3dRenderView::last_frame_work` and `Graphic3dRenderDriver::device_work` return `RenderDeviceWork`, which reports geometry buffers and geometry upload bytes separately from the device totals (#594).

- Added per-shape presentation mesh levels. `AisShape::set_presentation_mesh_lod` selects alternate shaded meshes under an approved screen-space error budget, including for connected placements (#594).

- Added opt-in mesh shaders for eligible opaque Phong triangles on devices with `VK_EXT_mesh_shader`. Meshlets reject groups outside the frustum or facing away from the camera, and unsupported draws keep their indexed path (#594).

- Added `RenderMeshVertexBehavior::Quantized16OctV2`, an opt-in vertex encoding using 16-bit positions relative to each mesh's bounds and octahedral normals. Binary32 vertices stay selectable for replay (#594).

- Added opt-in two-pass occlusion culling for retained placement batches. The depth pyramid comes from current-camera draws of previously visible placements, and the final pass keeps the original draw order (#594).

- Added opt-in progressive geometry uploads for mapped Vulkan views. Set `RenderAssemblyStreaming::VisibleBudgetedV2` to limit geometry upload per redraw and query the visible assembly's progress (#594).

- Added canonical single-chart conical normal extrusion for solved polynomial profiles with holes, using shared keyed construction and returning separate reference, storage and measure bounds, authored history and detached exports (#418).

- Added opt-in exact point-root and bracket filtering for generated Bézier pair membership signs and the `FilteredBernsteinV4` planar-proof selector while retaining earlier sign and planar-proof policies for replay (#544).

- Added opt-in exact Bernstein exclusion for planar Bézier pair and self queries while preserving the existing solver routes (#539).

- Added an explicit chart-angle normal-extrusion entry point with a separate immutable result, rotation-aware spatial evidence and detached native exports while preserving the earlier constructor for replay (#420).

- Added separate rotation-aware frame and spatial-budget certificates with bounded trigonometric evidence for explicit sketch seam angles (#420).

- Added revision-checked extraction of the extrusion chart from a saved cylindrical face, so edits could reuse the selected face's native location, axes and radius (#420).

- Added revision-checked conversion of native cylindrical face-hit parameters into developed coordinates with explicit seams, arithmetic bounds and separate trim-search coverage (#420).

- Added owned cylindrical face attachments with revision checks, checked persistence and explicit native-history reattachment that refused missing or ambiguous face images (#420).

- Added certified stored-frame deviation, positive stretch and determinant bounds against exact normalized reference frames (#425).

- Added certified point-domain placement budgets with translation and affine-evaluation rounding bounds, downward canonical allocations and explicit spatial-limit refusals (#425).

- Added certified oblique cylindrical profile placement with shared decoded locations, physical-frame validation, world-space reports and determinant-scaled retained boundary flux (#425).

- Added explicit finite pair-proof work requests and work-bearing planar failures with original root-isolation causes and authored curve IDs, and reported requested, effective and consumed work on successful region sets and wrapped normal-extrusion reports without changing existing defaults (#425).

- Added solved-profile cylindrical normal extrusion with typed spatial and measure reports, authored history, cooperative cancellation and detached native exports (#418).

- Added affected-package ThreadSanitizer execution of the wrapped-family concurrency stress campaign, retaining startup refusal and failing on campaign errors (#418).

- Added exact periodic material classification for private disconnected cylindrical selections, with guarded translated witnesses, original-source snapshots, complete interval coverage and aggregate winding budgets (#418).

- Added global planar selection and shared boundary-clearance preparation for disconnected wrapped-profile regions, preserving authored identities and rejecting overlapping filled selections (#418).

- Added source-matched outward comparisons between retained cylindrical boundary flux and ideal normal volume, preserving separate integration, profile-displacement and representative-error evidence (#418).

- Added source-bound closed-profile area and ideal cylindrical normal-volume enclosures, with exact requested depth arithmetic and explicit geometric-error reporting or refusal (#418).

- Added frozen incidence and curve-representation coverage checks for private rational wrapped profiles, with semantic-owner diagnostics and malformed-body controls (#418).

- Added certified conditional flux integration for retained rational ruled sides and cylindrical cap pcurves, with aggregate work limits and per-span receipts (#418).

- Added full-interval cylindrical cap-trim residual certificates over retained rational space curves and polynomial pcurves, checked against both retained edge and face tolerances (#418).

- Added decoded pcurve, parameter-domain and raw/effective topology-tolerance checks for rational wrapped profiles, with edge/face-scoped refusals (#418).

- Added complete cylindrical boundary-image clearance with certified true-period rounding, an infinite-tail bound and aggregate proof budgets, enabling private connected profiles spanning multiple turns (#418).

- Added deterministic cylindrical multi-chart Hermite planning from original polynomial bounds under one work allowance, enabling private wrapped profiles across the cylinder parameter seam (#418).

- Added outward true-circumference slab certificates for all nonzero cylindrical periodic images, bound to original profile sources with explicit incomplete results for wider selections (#418).

- Added certified developed-profile clearance with bounding-hierarchy pruning, cached dyadic refinement, exact-parameter witnesses and later binding to retained cylindrical profile certificates (#418).

- Added strict cylindrical profile tangent-cone certificates tied to retained position/derivative proofs, with exact positive margins and explicit work limits (#418).

- Added exact radius/depth rounding bounds composed with cylindrical storage certificates under one requested error allowance (#418).

- Added certified cylindrical profile position and derivative bounds against original authored polynomials, plus a priori dyadic Hermite fit planning with explicit resource limits (#418).

- Added `certify_cylindrical_storage` for conservative error bounds on stored cylindrical curves and their linear depth interpolation, with retained coefficient bits and explicit precision and work limits (#418).

- Added strict owned text capture with exact font-content bindings, caller occurrence IDs, raw glyph outlines and frozen text-builder plans. Added source-bound control-character diagnostics and an affected-package ThreadSanitizer stress control. (#425)

- Added conical-sector sketches with signed normal extrusion, seam-crossing regions, validated conical carriers and preserved source history. (#417)

- Added rectangular sketches on cylindrical surfaces with signed normal extrusion, seam-crossing regions, semantic history, ordered body validation and detached body exports. Existing planar extrusion interfaces were preserved. (#416)

- Added checked opt-in rigid location-transform decoding for coefficient-bound geometry while preserving legacy normalization defaults (#451).

- Added coherent owned Bézier pole/weight snapshots and used them for fresh sketch-source capture without changing evaluator bindings or validation budgets (#449).

### Changed

- Changed Vulkan `redraw_immediate` to reuse the image of the main layers while it holds, as OCCT's `RedrawImmediate` reuses its back buffer. The image holds until the view receives a main-layer notice, or until its camera, lights, rendering parameters, background colour, extent, layer selection or clip planes change. A notice is an update, display, erase, layer or priority change of a structure in a non-immediate layer, `invalidate`, or a change of the view's layers, background, lights or clip planes. While the image holds, an immediate redraw culls, builds and uploads only the immediate layers. An aspect or vertex buffer that is edited in place, without a structure or group update, now reaches the main layers at the next `redraw` instead of the next `redraw_immediate`. The `draw-record-verification` feature builds the whole frame beside each reused image and fails when its main draws differ (#594).
- Changed the tkv3d presentable-object accessors, such as `transform_persistence`, `clip_planes` and `view_affinity`, to read their field under the lock instead of copying every field and the presentation list (#594).
- Changed tkv3d picking to read the rectangular frustum once per pick instead of taking four locks for each box test. The tree walk reads the bounds of both children under the lock it already holds, and a box test reads the camera projection only after the axis test passes, as OCCT does. The objects of one pick share their scaled frustum and their walk stack. Pick results do not change, and `highlight_ais_spheres_400` goes from 1.24-1.35 to 1.18-1.22 times OCCT on the same host (#594).
- Changed immediate redraws with floating-point depth to restore the saved main colour and depth in one fetch draw. Retained targets for this path have sampled usage. Captures do not change, and `highlight_ais_spheres_400` saves about 10 µs per hover pair on RADV (#594).
- Changed Vulkan window redraws to hand the recorded frame to a presentation thread, one per renderer, which submits and presents it while the redraw returns. A submit or present error of such a frame now shows at the next redraw, `finish_frames`, capture or close instead of at the redraw itself. Acquire errors stay immediate, and frames with GUI layers, window depth status or ray-tracing feedback are still submitted and presented by the redraw. `highlight_ais_spheres_400` takes about 107 µs per hover pair instead of 130 µs on the same host, against 113 µs for OCCT (#594).
- Changed Vulkan group draw records to replay under any placement of the group: a record is keyed on whether the model mirrors and whether the stack is implicit, not on the model matrix, and a replay sets the clip transform, projection factors, world and normal matrices and the model stack of each placement. `RenderLighting` now keeps its world and normal matrices inline, outside the shared `LightingBody`. A connected structure replays the records of the structure it references without lowering. Captures do not change, and `frame_connected_30k` falls from about 154 ms to 40 ms per frame on RADV (#594).
- Changed Vulkan draw records and retained view batches to subscribe to the graphics inputs they read. Edits to watched groups, buffers, aspects, or per-view visibility trigger a fresh check without invalidating unrelated views (#594).
- Changed eligible raster uploads to use 16-bit indices when they fit and draw tables to refer to shared materials by index. Larger index streams and unsupported draw tables keep their previous layouts (#594).
- Changed eligible Vulkan layers to record secondary command buffers on workers. Large cold geometry uploads can run on a dedicated transfer queue when one is available (#594).
- Changed Vulkan camera-only redraws of eligible connected assemblies to reuse their draw templates and placements. Edits to geometry, material, placement, or visibility invalidate the retained frame. Cropped views retain the batch and use ordered GPU compaction when the device and frame qualify (#594).
- Changed the Vulkan renderer to record a run of at least eight consecutive plain lit surface draws as one indexed indirect draw when the device has `multiDrawIndirect` and `drawIndirectFirstInstance`. Each draw reads its clip, world and normal matrices from a per-frame draw table, so the draw order and the captures do not change, and draws of a run share one lighting block. `frame_connected_30k` records in 0.14 ms instead of 5 ms and uploads 6.2 MB instead of 11.2 MB per frame on RADV (#594).
- Changed the Vulkan renderer to read the device limits and OIT attachment formats once per device instead of in every frame, and to check twice its uploads for geometry retirement in a frame that restores the main image, instead of 512 entries (#594).

- Changed Vulkan `PrimitiveDraw` and `RenderLighting` to keep the values a frame sets on every draw inline and the rest in a shared `DrawBody` and `LightingBody`, which they dereference to. Replayed draws, the frames in flight and the instances of a connected presentation share one body instead of copying 1.5 KB per draw (#594).

- Changed the Vulkan renderer to assemble a layer of 256 structures or more on `rayon` worker threads when every group of a chunk of structures replays a draw record; lowering and user callbacks stay on the calling thread, and the merged program keeps the structure order. Runs of 256 scene text labels or more replay their label records on the workers in the same way. `frame_transparent_1k` takes 0.68 ms instead of 0.87 ms, and `frame_text_1k` 0.68 ms instead of 0.88 ms, on the same host (#594).

- Changed `Graphic3dAspectsBase::fields` from `SharedState` to `RevisedState`. `read`, `write`, `try_read`, `try_write` and `snapshot` work as before; each write now advances `fields.revision()`. Code that names the field's type must use `RevisedState<Graphic3dAspectsFields>` (#594).

- Made a Vulkan group that two passes lowered keep its lowered surfaces and replay them on later redraws while its commands, aspects, geometry, transformation and the view's rendering parameters stay the same. Only the values derived from the camera are computed again. `frame_transparent_1k` takes 1.5 ms instead of 2.7 ms on the same build host (#594).

- Changed a Vulkan view on a native window to keep the window size between `set_window` and `must_be_resized`, as OCCT's OpenGL view does, instead of asking the X server on every redraw. A native resize that the application does not report with `must_be_resized` now shows when the swapchain reports it out of date or suboptimal. GUI frames still check the surface on every redraw (#594).

- Changed Vulkan swapchains without vertical sync to hold one more image than with FIFO, so acquiring an image no longer waits for XWayland to release one. FIFO keeps its image count and adds no latency (#594).

- Made Vulkan colour dumps read only the colour attachment and reuse their host buffer. An RGB dump packs the rows on the GPU and copies them straight into the image; a 1080p RGB dump takes 0.46 ms instead of 12.8 ms. Captures return the same pixels as before (#594).

- Stopped blocking a Vulkan resize on the destruction of the replaced swapchain; the old swapchain is destroyed once its presentation fences signal. A resize still recreates the swapchain, which RADV requires on X11, so `resize_redraw` stays about twice as slow as OCCT (#594).

- Stopped waiting on an acquire fence and re-querying the surface in each Vulkan window frame (#594).

- Stopped waiting for the GPU and copying the frame to host memory in each Vulkan redraw. Up to two frames run on the GPU while the host prepares the next one. `Graphic3dRenderView::capture` reads the frame back the first time it is called and returns the same pixels as before. A GPU fault that a frame reports after its redraw returned now comes from the next `capture`, the next redraw that needs the frame's slot, or `Graphic3dRenderDriver::finish_frames`. The redraw counters report no read-back bytes (#594).

- Reused Vulkan vertex and index buffers between redraws, including for static textured meshes, and batched geometry transfers. Shared mapped storage pages and descriptor pools across the draws of a frame (#594).

- **BREAKING** Replaced transient object ownership with typed Rust handles and static class descriptors. Callers use compile-time upcasts, checked downcasts and explicit weak references (#592).

- Used Rust strong counts during TKV3D callbacks and released pixmaps displaced by reentrant texture setters (#592).

- Restricted toolkit internals with no production callers outside their crate. Use the public palette lookups to access named colors whose unused constants were removed (#591).

- Replaced `AsciiString`'s shared allocation with owned bytes and removed `buffer_identity`. Borrow `as_bytes` or `as_bytes_with_nul` while the string is live, and copy the bytes when they must survive an edit (#588).

### Fixed

- Preserved exact 24-bit depth during retained Vulkan immediate redraws by copying the stored attachments (#596).

- Shared one intersection context across the workers of parallel Boolean operations instead of building one per worker, and built missing cached services outside the cache lock; the body self-intersection check stays serial because its parallel path raised peak memory across long operations (#496, #580).

- Reused complete native endpoint-incidence discovery across contact-cardinality reports while preserving empty-report and self-contact diagnostics (#566).

- Indexed Census report reuse by source pair and query mode, preserving the first retained outcome and solver work (#567).

- Reused native-source validation within private owned-profile graph batches, preserving live alias checks for borrowed inputs and exact source-history matching (#555).

- Decoupled cylinder-only error reporting from cone-specific failure formatting while preserving typed causes and diagnostics (#547).

- Clarified the raw native volume error-status units and corrected cone numerical verification to compare occurrence flux at the same field origin without changing native estimates or certified bounds (#546).

- Refused negative grid sample counts in generic curve–surface extrema searches with a typed error instead of an indexing panic (#535).

- Corrected the placed cylindrical flux contract to use `(x-t)/3` and exposed the decoded field origin on normal and chart results. The existing enclosures and volume budgets remained unchanged (#528).


- Restored selection of the ignored wrapped-family ThreadSanitizer stress test for affected packages while retaining existing replay controls (#529).

- Preserved cylindrical operation failures when cancellation arrived later and propagated cancellation through global planar proof work (#526).

- Verified decoded export bindings by authored identity and rejected duplicate or mismatched bindings before publishing native exports (#519).

- Preserved checked native conic frames and curve directions during explicitly configured binary capture, including recursive and surface-embedded curves, while retaining legacy read behavior (#455).

- Required placed cap traces to remain within their retained edge and face tolerances, including affine-evaluation rounding and typed span/depth refusals (#454).

- Canonicalized wrapped planar proof inputs by authored IDs so reordered selections retained identical actual work receipts, while preserving original admission errors and unchanged limits (#453).

- Preserved authored entities and typed failure classes in solved-profile normal extrusion errors (#448).

- Reported admitted input ranges and retained offending values in cylindrical profile certificate diagnostics (#524).

- Rejected invalid raw body tolerances before native validation could mask or overwrite them, preserving refusal input state and explicit stage coverage (#490).

- Restored explicit walking-line allocator ownership, allocation and reset policies, and legacy signed point counts (#507).

- Kept release source proofs and changed-input comparisons in one canonical package-root coordinate system while preserving native Windows path components (#487).

- Followed literal production QA modules while excluding disjoint test edits from release package version checks (#485).

- Retained publication impact for traversing and unsupported Cargo production-target paths (#482).

- Excluded proven test-target manifest edits and the reviewed budget-marker script from package source-version checks while retaining affected tests and conservative production-input checks (#481).

## [0.13.1] - 2026-11-15

### Fixed

- Avoided broad sibling scans for proven single-label document transactions while retaining native traversal for ambiguous or reentrant changes (#489).

- Separated test-only edits from release package version checks while retaining affected QA suites and conservative production-fixture checks (#466).

- Retained all contributing solid parents through final Boolean Fuse assembly and preserved reverse ancestry bindings (#456).

- Stabilized planar-profile area cross-checks under large coordinate translations without changing certification limits (#446).

- Removed repeated pair-ledger validation during planar profile certification while preserving live geometry checks and shared work budgets (#449).

- Updated rustix to 1.1.6 and xattr to 1.7.0 to resolve RUSTSEC-2026-0486 while preserving extended-attribute values and name lists larger than 4 KiB (#445).

- Matched local PR test optimization to CI for script-budget packages while preserving other packages' profiles and test selection (#447).

- Removed quadratic backtracking from the binary32 proof-source parser on malformed whitespace records without changing accepted records or certificates (#441).

- Bound Linux renderer test gates to the recorded system-font configuration so user font folders no longer changed rendered test images (#433).

- Preserved stored analytic surface frame coefficients through an explicit checked BinTools read policy while keeping legacy readers unchanged (#432).

- Skipped expanded unsafe inventory rebuilds for unrelated driver and documentation changes, while removing stale reports on skipped PR checks. (#411)

- Reused verified artifacts from a successful release rehearsal on the exact tag commit, avoiding duplicate build and test jobs while retaining publication checks. (#275)

- Limited ThreadSanitizer controls to affected packages and execution inputs, accepting skipped jobs only when no controls were selected. (#410)

- Corrected the PR formatting failure hint to list checked files and explain direct formatting for sources included by test suites. (#122)

## [0.13.0] - 2026-11-07

### Added

- Added `martensite-tkdeiges` (IGES reading and writing: the data structures, the file parser and writer, the entity packages, shape and document transfers, and the format provider). The global section of a written file names Martensite as the sending product, system and preprocessor, its default author is `Martensite`, and its dates are in UTC. The configuration keys are `provider.IGES.Martensite.*`. Reads and transfers on separate threads take one lock of the format, so they run one at a time and give the same shapes as on one thread. (#405).
- Added `martensite-tkrwmesh` (the tools of the mesh file formats: axis and unit conversion, the filling of exchange documents from mesh readers, face, edge and vertex iterators, and material maps). (#405).
- Added `martensite-tkdeobj` (OBJ reading and writing: OBJ and MTL files, document transfers, and the format provider). A written OBJ or MTL file starts with `# Exported by Martensite`, and the configuration keys are `provider.OBJ.Martensite.*`. `read.memory.limit.mib` is a limit in mebibytes. Positions, normals and texture coordinates keep 9 significant digits, so a small value is not lost and a 32-bit float reads back exactly. A comment keeps a one-letter last line and repeated lines, and a line break in a group or material name is written as a space. (#405).
- Added `martensite-tkdeply` (PLY writing of documents and shapes, and the format provider). A written file carries `comment Exported by Martensite`, and the configuration keys are `provider.PLY.Martensite.*`. The provider's `WriteFile` applies the coordinate system and length unit settings, so a default Z-up document written to a Y-up file holds `(x, z, -y)`; `RwPlyCafWriter` with no conversion set writes coordinates as they are. Single-precision values keep 9 significant digits, so a 32-bit float reads back exactly, and double-precision positions keep 17. Closing a file with fewer records than its header states answers false, unless the export was aborted. (#405).
- Added `PackedMap::same_walk_copy`, a copy that walks exactly as its source does. A clone sizes its table for the blocks left, so it can walk the same values in another order. (#405).

### Fixed

- Meshing no longer changes the order of a face's triangles when triangles were removed from a large domain on the way. `BRepMeshDataStructureOfDelaun::elements_of_domain` copied the domain into a smaller table, whose walk differs from the domain's own. On one face of `as1_motor.step` with 369 triangles, the domain kept 1009 buckets and the copy had 101. The triangles themselves do not change.
- Healing a wire no longer loses the surface of a face on a surface periodic in both directions. All the ends of such a wire can lie at one point in space, a few digits of a file apart, so the gaps in space chose the opposite edge. A torus read from a file as faces, written and read again, came back with an area of about 0. `ShapeFixWire::perform` now also reads the p-curve gaps when every edge has a p-curve, the gaps in space would reorder the wire and the two orders chain the edges differently; other wires keep the order they had. `ShapeAnalysisWireOrder`, reading both gaps, now continues the chain at a joint closed in space and on the surface, even from a part of the chain that is itself closed in space.
- Unifying faces now walks its map of replacement p-curves in the order the edges were added, like every other map keyed by shapes. Its own hash policy did not ask for that order, so this one map still followed addresses.
- The same input now gives the same shapes in every run and in every repeat within a run. `Map` and `DataMap` keyed by shapes walked their entries in the order of their hashed addresses, which change between runs, so the offset of a plain box by arcs came out in a different order each time and the check of one offset moved between valid and faulty. Maps keyed by shapes, shape sets, pave blocks and common blocks now walk their entries in the order they were added; a removal moves the last entry into the freed place. All other maps keep their order. A policy or key type outside the kernel chooses this with `HashPolicy::walks_in_order_added` or `HashKey::WALKS_IN_ORDER_ADDED`.
- Reading a STEP file no longer moves the end poles of a B-spline edge curve when a vertex lies inside the curve's range. The vertex tolerance now covers the same evaluation that checks it, so the edge is accepted at once. In `as1_motor.step` the moved poles took the trimmed edge up to 68 units away from its vertices, which gave those vertices a tolerance of 68 and meshed the two faces beside the edge far too coarsely.

## [0.12.0] - 2026-11-06

### Added

- Added `martensite-tkdestep` (STEP reading and writing: the parser and writer, the entity schemas, shape and document transfers and the format provider). Each value reads as the file has it and writes back the same: shading methods, the actuated directions of kinematic pairs, the surface and volume element purpose names, and `EXTERNALLY_DEFINED_CLASS` and the other records whose fields are filled on read. Triangulated faces whose normals are not a square array write like any other (#403).
- Added `V3dView::default_view_axis`, which returns the default view point and axis that `set_axis` keeps, with every digit (#403).
- Added `certify_tolerance_resolved_extrusion_face` with `ToleranceResolvedFacePolicy::v4()`: it certifies a closed extrusion face whose stored boundary carrier stays within the original tolerances of the surface but is not exactly on it, and reports every boundary contact and error bound without changing geometry. Earlier face policies keep their results and refusals, and an enclosing solid still needs its own checks (#213).
- Added `certify_given_vertex_neighborhood`, `certify_given_vertex_contacts` and `certify_disjoint_vertex_neighborhoods`, which prove from exact coefficients how curves enter a given vertex ball, where a curve meets a line inside it, and that two vertex balls do not overlap (#213).
- Added `certify_open_quadratic_extrusion`, which proves that the extrusion of an open quadratic spline does not intersect itself without joining its endpoints, with `StoredSplineInput::into_open_extrusion_input` and `capture_placed_open_support_inputs` to build its inputs from stored curves and face snapshots. Face validation does not yet certify faces on open splines (#213).

### Changed

- `XCAFDocClippingPlaneTool::remove_clipping_plane` now removes a clipping plane a view refers to, and the view loses the reference. It used to refuse the removal (#403).
- Validated Vulkan startup now refuses a Khronos validation layer older than 1.4.313 before creating the instance, and `RenderValidationAdmissionFailure::UnverifiedLayer` reports both versions. Older layers could start and then report false errors or crash; Ubuntu 24.04's packaged 1.3.275 is one of them. Install 1.4.313 or newer, which `scripts/install/vulkan-validation.sh` does on Ubuntu 24.04 x86_64, or start with validation off (#388).
- `install-linux.sh` now supports apt on Ubuntu only, because other apt-based systems cannot be relied on for validation layer 1.4.313 or newer; they stop before any package changes. Ubuntu 24.04 on x86_64 keeps the pinned layer, Ubuntu 25.10 and later use apt's package, and the installer fails with both versions if the installed layer is too old. Ubuntu 24.04 on other architectures also stops, and should move to 26.04 (#388).

### Fixed

- Attributes are now added after the ones already on a label, so a label lists them in the order they were created. This covers the array, list, map, tree, expression, variable and function attributes, the document attributes, metadata copies and the viewer attribute (#403).
- Reading a file into a null model now reports a failure instead of panicking (#403).
- Validated Vulkan startup now names a missing validation layer or required instance extension before creating the instance. The check includes extensions from the selected layer and keeps validation enabled (#388).
- A volume no longer loses the sliver of a trimmed face narrower than `1e-9` in surface parameter. `volume_properties` and `volume_properties_gk` integrate every span of positive width, so a slightly skewed prism gets its exact volume, centre and inertia (refs #386).

## [0.11.0] - 2026-11-06

### Added

- Added `martensite-tkdestl` (STL reading and writing: text and binary files, shape transfers and the format provider), `martensite-tkmeshvs` (mesh visualization: mesh data sources, the drawer, presentation builders and selection on elements and nodes) and `martensite-tkxmesh` (a named meshing factory, an alternative entry to the incremental mesher).

### Fixed

- Shape healing now aligns seams correctly and no longer loses faces or produces invalid shells on real STEP files (#399).

## [0.10.0] - 2026-11-05

### Added

- Added `martensite-tkfeat` (local operations and form features: prisms, revolutions, pipes, holes, ribs, slots, glued shapes and splits) and `martensite-tkoffset` (offset modeling: offset shapes and thick solids, simple offsets, drafts, pipe shells, lofts and tangent blends) (#397).

### Changed

- Offset shells are now oriented by their offset faces when a split cap face closes the shell, so the same input gives the valid solid on every run (#397).

### Fixed

- Volume properties of symmetric curved solids no longer take minutes. On the default route, refinement now also stops once the absolute error is at most `1e-14 × L^k`, so an integral that cancels by symmetry no longer chases a relative error it cannot reach. The volume stays the same to its printed digits, and `volume_properties_gk` keeps the relative stop (#397).

## [0.9.0] - 2026-11-05

### Added

- Added `martensite-tkde` (the data exchange framework: configuration nodes, providers and the format registry), `martensite-tkxsbase` (the data exchange foundation: interface models, transfer processes, selections and work sessions), `martensite-tkvcaf` (the presentation attributes of a document and their drivers) and `martensite-tkxcaf` (the extended exchange document: shapes, colors, layers, materials, dimensions and tolerances, notes, views and their presentation) (#395).

### Changed

- Shape files now carry a Martensite version line (`Martensite Topology V3` in text, `Martensite Binary Topology V4` in binary). The older lines are still read (#395).

## [0.8.0] - 2026-11-05

### Added

- Added `martensite-tkbo` (Boolean operations: general fuse, builders, intersection tools, splitting and defeaturing), `martensite-tkbool` (topological operations, sweeps, evolved shapes and projections), `martensite-tkfillet` (fillets and chamfers) and `martensite-tkcaf` (topological naming, the extended document attributes and the standard application), with `martensite-boolean-cases` for the check contract and regression cases of the Boolean scripts (#393).

### Fixed

- Dividing a face across a degenerate edge, such as the one closing a sphere at its pole, keeps that edge's vertex instead of leaving a coincident new one for each cut. A cut near an end vertex now ends on it even when the two are apart on the surface.
- Curve-curve extrema no longer overflow when balancing search intervals between a curve of nearly no length and a much longer one. Builds with overflow checks panicked there.

## [0.7.0] - 2026-11-04

### Added

- Added `martensite-tkv3d` (selection, presentation, interactive objects and the 3D viewer) and `martensite-vulkan` (the Vulkan 1.3 renderer behind the graphic driver interfaces), with `martensite-raster-core`, `martensite-geometry-raster` and `martensite-topology-test-support` as development helpers for their regression tests; this adds the `ash` and `num-rational` dependencies (#365).

- Added a checksum-pinned Vulkan validation layer and Mesa lavapipe driver to the Linux installer and CI for the renderer tests (#365).

- Added bounded owned rational surface query embedding certificates, coherent tensor-surface input leases and an explicit expanded input-capture policy (#213).

- Added explicitly selected relative-frame carrier direction candidates with exact world-vector verification for finite extrusion face checks (#213).

- Added an owned finite closed-extrusion face certificate joining exact support injectivity, trim and seam incidences, and unchanged spatial error budgets (#213).

- Added finite coherent face-input capture and nonblocking geometry leases, with owned topology, representation and geometry fields and explicit incomplete outcomes (#213).

- Added an owned exact sufficient self-exclusion operation for closed quadratic spline extrusion supports, with complete span obligations and explicit incomplete outcomes (#213).

- Added exact coordinate-projection self-exclusion certificates for owned bilinear patches and supported single-span spline snapshots (#213).

- Added owned surface self-search work receipts, local numerical termination observations and cancellable search entry points, preserving unknown inner termination and sampled discovery scope (#213).

### Fixed

- Retained source-owner and typed relation context for face-query failures and corrected owner labels in spatial-consistency diagnostics (#378).

- Preserved quiet-NaN signs, payloads and direction priority in `next_after` across optimization levels (#370).

- Fixed skipped tag publication after successful release checks when the PR-only test job was intentionally skipped (#350).

- Allowed a later restoration bundle to replace a retired recovery anchor, keeping exact publication identity and bytes (#358).

## [0.6.0] - 2026-11-01

### Added

- Added an isolated-case mode that preserves inherited process policies, with bound version2 records and no runner-installed CPU or elapsed budget (#351).

### Fixed

- Fixed release staging of restored packages by deriving local checksum companions after their retained payload and manifest identities are verified (#362).
- Fixed rejection of bounded GNU long-name metadata in verified historical package archives while retaining strict resource and manifest checks (#360).
- Restored verification of exact historical package bytes through separately recorded current publication anchors when original assets are retired (#358).
- Fixed stale layer bounds after moving displayed structures so bounds and fit queries use current membership (#353).

- Fixed avoidable PNG buffer-save refusals by using default compression with adaptive filtering across file, stream and buffer output (#347).

## [0.5.0] - 2026-10-31

### Added

- Added explicit OSD signal capabilities, reporting Windows console cancellation and floating-point trap controls while keeping native hardware faults fatal (#334).

- Added opt-in Windows multi-axis raw HID input with owned reports, acquisition errors and bounded-queue overflow diagnostics (#334).

- Activated the C locale per thread on Windows and restored the previous CRT locale name and mode when the sentry left scope (#334).
- Released Windows directory-iterator cursors on explicit destruction and allowed later reinitialization (#334).

- Routed Windows display connections through the process module binding and retained them with window leases until native teardown (#334).

- Checked requested Windows file read, write and execute access against the native ACL using the effective token instead of inferring access from read-only attributes (#334).

- Created Windows temporary files and directories with random names, exclusive creation and private ACLs, retrying name collisions through `tempfile` (#334).

- Added native Windows file-node ACL queries and updates for system, owner, group and world rights without reducing them to a Unix mode (#334).
- Applied requested Windows ACLs atomically during file and directory creation, retaining existing-file policies when reopening (#334).

- Added live HWND wrapping, mapping, title updates, rectangle-change detection and repaint requests to `WntWindow` on Windows (#334).
- Added owned Win32 windows with a registered native class, dedicated message dispatch and destruction on the owner thread even when the wrapper moves between threads (#334).

- Added native Windows login-name and effective administrator-token queries to `Process` (#334).
- Added Windows computer-name, IPv4 and available pagefile-backed memory queries to `Host`, capping available bytes at the existing signed return type's maximum (#334).
- Added Windows process-memory counters from Psapi, virtual address-space queries and a locked process-heap walk, retaining unknown values when queries fail (#334).
- Added native Windows major, minor and build reporting to `Host::system_version`, preserving native query failures (#334).

- Added SGI (".rgb", ".rgba", ".bw", ".sgi") loading to `ImageAlienPixMap` with a decoder in the crate: verbatim and run-length encoded files with one to four one-byte channels; no new dependency (#332).

- Added PNG, JPEG, BMP and GIF loading to `ImageAlienPixMap`, storing rows bottom-up with four-byte padding and gray, blue-green-red or blue-green-red-alpha pixels; this adds the `image` crate as a dependency (#330).

- Added owned live buffer-kind arguments with explicit unsupported-backend errors for reference-aware capture (#323).

- Added shared-buffer stream construction and scoped input borrowing that rejoins state, read counts, and unread lookahead with owned cleanup failures (#319).

### Changed

- Changed `ImageAlienPixMap` saving to write the format its extension names (PNG, JPEG, BMP, GIF or PPM) into a file, stream or buffer; other extensions and empty images now return false instead of writing PPM (#330).
- Retired only the groups present when clear begins, allowed destructor reentry without the structure guard, and preserved callback-created groups and old flags when retirement panics (#324).
- BREAKING: Changed the public driver-structure group field to a stable sequence, requiring custom drivers to migrate vector-specific mutation and indexing calls (#324).

### Fixed

- Rebuilt the Windows PR build driver in a separate target directory so the running executable no longer blocked all-target build validation (#339).

- Kept harmless Git checkout-conversion warnings from failing the Windows PR whitespace gate while preserving actual whitespace failures and user configuration (#338).

- Matched MSVC C-locale wide-character formatting for byte-sized units above ASCII while preserving Unix's 7-bit restriction (#337).

- Kept borrowed Windows library-cache handles separate from owned loader references so closing a cache entry no longer unloads another owner's module (#170).
- Reopened Windows shared libraries with a new live loader reference and re-enabled symbol lookup after close (#336).

- Released the Windows file descriptor registry before blocking I/O so contended locks no longer prevent another descriptor from unlocking (#335).
- Converted Windows file timestamps through the native local timezone instead of treating UTC as local time (#334).

- Kept the selected group node across virtual clear callbacks that remove earlier groups, and released retired nodes outside the structure guard (#325).
- Released buffer-kind read guards before invoking reentrant formatting callbacks (#327).

- Provided an owner-aware borrowed runtime descriptor query for projected handles without increasing reference counts (#317).
- Retained stream devices after write or positioning panics and rejected incomplete scratch input after interrupted reads (#318).

## [0.4.0] - 2026-10-26

### Added

- Added explicit surface integration and representation policies with owned properties and estimate-or-unknown evidence (#277).
- Added a verified production-library source SDK bundle with locked dependencies and explicit individual-archive requirement checks (#279).

### Fixed

- Reduced B-spline span-evaluation allocations by borrowing existing control arrays during planar and spatial cache rebuilds (#309).

- Selected the retained gradient background when changing its colors or fill method, while preserving explicit background-kind overrides (#305).

- Preserved CPU-budget exit code 2 on macOS 12+ with closed or full stdout while leaving application signal actions and descriptor flags intact. Older macOS returns typed `Unsupported` before changing the cap; other kernel features retain their operating-system support (#289).

- Updated native macOS validation to `macos-latest` for ARM64 and `macos-26-intel` for Intel while preserving all four release targets (#298).

- Reduced walking-line point-read overhead by keeping fixed curve ownership outside mutable metadata (#294).

- Serialized native source SDK paths as portable archive members on Windows while retaining member validation (#290).

- Used Cargo's resolved target directory when collecting release source archives, including configured and external output directories (#285).

- Published unchanged package versions once when complete release history contains no earlier archive, while preserving verified existing assets and recording publication provenance (#283).

- Restored CPU-limit diagnostics and exit code 2 for isolated Unix processes, including limits applied after CPU was consumed (#278).

- Reused one walking-point snapshot for combined spatial and parametric coordinate reads during fitting (#220).

## [0.3.6] - 2026-10-24

### Fixed

- Pinned pull-request CI selection to the event base commit so later jobs retained the planned packages when the base branch advanced (#273).

- Optimized TKMath development and test builds to reduce repeated assembly-integration cost while retaining debug checks and existing test budgets (#271).

- Preserved hash-pinned healing-resource bytes on Windows and made the release workflow contract test accept Windows line endings (#269).

- Split release tests into bounded parallel shards on every supported platform and required their aggregate result before publication (#250).

- Preserved unchanged package versions and selected release tests by source and dependency impact instead of requiring workspace-wide version bumps (#256).
- Refused Windows document replacements with destination named streams before changing metadata or publishing bytes, preserving the original file and its stream contents (#248).
- Reduced exact-certification test costs by optimizing the existing big-integer dependency in development and test builds (#252).
- Split affected toolkit tests into parallel PR jobs while retaining dependency-aware selection and a required aggregate result (#250).

- Staged document writes so preparation failures and incomplete driver output leave existing files intact; verified access policies before publication and retained requested paths and symbolic links (#245).
- Refused custom restrictive writer policies before replacing existing documents, while retaining ordinary staging defaults and destination access policies (#245).

- Rejected incomplete geometric volume integration and preserved caller properties on failure while retaining completed empty selections (#246).

- Preserved planar wire geometry during face construction and reused finite curve bounds within classification operations without storing projection caches on input edges (#242).

- Removed the reciprocal hidden-line/primitive test dependency while preserving the two-solid projection check, so dependency policy and source packaging agree (#241).

- Preserved distinct boundary incidences at restriction-line endpoints during interval construction (#238).

- Bounded default trimmed-face surface integration error (#230).

- Reduced isolated dataset-test overhead by optimizing SHA-256 in development and test builds without removing executable identity checks (#228).

- Preserved natural-bound face construction for closed non-periodic surfaces (#225)

- Corrected volume properties of trimmed faces with bounded Gauss-Kronrod integration (#221).

### Added

- Added shape healing, mesh construction and document-object model crates (#215, #216, #217).

## [0.3.5] - 2026-10-20

### Added

- Added the `martensite-tkbrep` crate for the boundary representation: topological
  shapes and their exploration, the geometric shapes with their builder and queries,
  shape tools and adaptors, local continuity, the graph form of a shape and the binary
  shape stream (#208).
- Added the `martensite-tkgeomalgo` crate for geometric algorithms: curve and surface
  intersections, hatching, 2d tangent constraint solving, filling, plate surfaces,
  evolution laws, local continuity analysis and the sketch spline construction (#208).
- Added the `martensite-tktopalgo` crate for topological algorithms: point and solid
  classification, shape construction and sewing, validity checks, distance and
  proximity extrema, bounding boxes, global properties and the medial axis of faces
  (#208).
- Added the `martensite-tkhlr` crate for hidden line removal: interval sets, the
  polyhedral and exact hidden line algorithms, contour lines of surfaces seen from a
  direction or a point and the conversion of the results back to shapes (#208).
- Added the `martensite-tkprim` crate for primitive solids: boxes, wedges, cylinders,
  cones, spheres, tori, revolved bodies and half-spaces, and the linear and rotational
  sweeps of any shape with their history (#208).
- Added the `martensite-tkhelix` crate for helix curves: the analytic helix, its
  B-spline approximation coil by coil and composite helix wires of several parts
  (#208).

## [0.3.4] - 2026-10-20

### Added

- Added the `martensite-tkcdf` crate for application documents, references between
  documents, document metadata, format-driver lookup, container header reading and a
  light XML parser and writer (#205).
- Added the `martensite-tklcaf` crate for the application data framework: labels and
  attributes, transactions with undo and redo, standard attributes, the function
  mechanism, documents, external links and multi-document transactions (#205).
- Added the `martensite-tkservice` crate for visualization services: windows and input
  events, pixel maps, fonts and text layout, the presentation data model, shader
  program generation and the graphic driver interfaces. Linux is complete; the Windows,
  macOS and canvas windows are defined interfaces that return a typed
  `Standard_NotImplemented` error until their backends exist. FreeType, fontconfig and
  Xlib are loaded at run time (#205).
- Added the opt-in `host-libm` feature of `martensite-cmath`: every transcendental
  function returns the platform C library result (numerical behavior version 1
  replay), for verification against programs linked with that library. It is
  never enabled in releases or Martensite CI gates (#205).

### Fixed

- Recorded the Intel macOS compiler-inventory baseline from a clean native capture (#198).

- Corrected release coverage to Linux x64, Windows x64, and both macOS architectures, with native Intel macOS validation replacing Linux ARM64 (#198).

- Preserved frozen fixture bytes during Windows checkouts with automatic line-ending conversion (#197).

## [0.3.3] - 2026-10-19

### Added

- Added geometric construction, conversion, approximation, extrema, bounds and mass-property tools with retained numerical fixtures (#193).

- Added spatial curves, surfaces, adaptors, evaluation grids and retained numerical fixtures (#192).

- Added checked borrowed input with state masks, position capability and owned typed
  causes, sharing extraction with existing owned streams and binary container
  metadata decoding with checked offset-range rejection (#142, #181).

- Added explicit fallible printer and ordered messenger delivery with owned typed
  failures, preserving existing infallible entry points (#112).

- Added the `martensite-tkexpress` crate for symbolic expressions, differentiation,
  simplification, named functions, relations and text interpretation (#100).
- Added the `martensite-tkg2d` crate for planar curves, splines, offsets, adaptors,
  derivative and grid evaluation, and quantized geometry hashes (#105).

### Changed

- Computed every transcendental function (trigonometric, hyperbolic, exponential,
  logarithmic, power, cube root and `hypot`) with one portable, correctly rounded
  library, so finite transcendental results are bit-identical on Linux x64,
  Windows x64 and both macOS architectures. This is numerical behavior version 2
  (#201).

  Migration from behavior version 1 (0.3.2 and earlier): a result changes only
  where the platform C math library did not round correctly. On Linux, 971 rows
  in 73 retained fixtures and the expected values of 3 test literals and 20
  fixture seals changed. Each change was traced to individual function results
  that MPFR confirms the new library rounds correctly and glibc did not.
  Composite results such as curves, projections, iteration counts and formatted
  text can differ by more than one unit in the last place where they amplify
  such a difference. Apart from the documented invalid-operation NaN below, NaN
  sign and payload remain unspecified.
  Version 1 cannot be selected inside this release; applications that need its
  exact results stay on the 0.3.2 crates. The maintainer approved behavior
  version 2 with this recorded migration instead of version-1 replay; the
  evidence is in `docs/features/portable-transcendentals/attribution/`.

- Returned one NaN word, `0xfff8000000000000` (the x86-64 default NaN), for
  invalid divisions in expression evaluation and for directions normalized from
  infinite components. Arm targets previously produced a positive NaN here, so
  their text and words differed from x86-64; x86-64 results are unchanged. The
  new `martensite_cmath::quotient` and `INVALID_OPERATION_NAN` provide this (#201).

- Spelled infinities and NaNs in stream text (`format_general` and `IoStream`
  real insertion) by their sign alone (`inf`, `-inf`, `nan`, `-nan`) on every
  target, as the glibc layout it documents. Windows previously printed the
  x86-64 default NaN as UCRT's `-nan(ind)` there; the C `sprintf` emulation
  keeps the host spelling (#201).

### Fixed

- Preserved serialized inverse-hyperbolic domain errors: `acosh(x < 1)` and
  `atanh(|x| > 1)` return `INVALID_OPERATION_NAN` (`0xfff8000000000000`) on every
  target, while NaN operands stay classification-only and valid results stay
  correctly rounded (#203).

- Guarded single-precision powers with NaN arguments before dependency evaluation, while preserving zero-exponent and unit-base identities (#202).

- Fixed symbolic-expression message failures to honor configured stack capture while
  preserving explicitly supplied stacks (#141).

## [0.3.2] - 2026-10-18

### Added

- Added owned storage-stream output seeking for header backpatching while preserving
  EOF, input counts and formatting state (#136).

### Fixed

- Shipped `LICENSE-APACHE` in the binary release archives and `LICENSE-GPL` in
  every source package; the release had still bundled the removed `LICENSE`
  file (#176).
- Checked toolkit unsafe source counts during PR validation, including inactive
  platform code and test fixtures (#81).
- Checked expanded unsafe counts against target-specific compiler baselines and
  retained separate workspace and dependency reports in CI (#81).
- Removed stale compiler inventory reports and streams before starting a new
  capture, including captures that failed before compilation (#118).

### Changed

- Replaced selected toolkit raw ownership, record initialization and native calls
  with safe ownership and established safe interfaces; preserved public APIs and
  excluded candidates with rejected or unproven performance. Recorded the partial
  migration results and outstanding constraints in the safe-toolkit closeout (#80).

## [0.3.1] - 2026-10-16

### Added

- Added the `martensite-tkmath` crate, the math toolkit: geometric primitives
  and transformations, vectors and matrices, linear algebra, root finders,
  minimizers and integrators, B-spline and polynomial libraries, conversion of
  conics and elementary surfaces to B-splines, bounding boxes and bounding
  volume hierarchies, triangulations and polygons, locations, and exact
  predicates for sketch incidences and certified Bezier roots ([#76]).

### Changed

- Changed the `cargo xtask release` semver check for versions before 1.0.0 to
  cargo's compatibility rule: an additive change (`Added`, `Deprecated`, a
  non-breaking `Changed`) needs a patch bump and a breaking change (a removal
  or incompatible change) a minor bump. From 1.0.0 the rules are unchanged
  ([#76]).

### Fixed

- Fixed false incompatible-release errors for Common, Heap and Aligned storage
  on Windows, preserving coherent ring removal and arena-release protection ([#78]).

- Fixed cancelled sequence-clear tokens being accepted by later clear
  invocations, preventing live values from being abandoned ([#77]).

- Fixed `cargo xtask pr` accepting fatal Git errors as clean untracked-file
  whitespace checks; failed comparisons now retain their diagnostics ([#75]).

## [0.3.0] - 2026-10-16

### Added

- Added `install-linux.sh`, `install-macos.sh` and `install-windows.ps1`, which
  install or update every development dependency with the platform's package
  manager.
- Added the `cargo xtask` build driver and the single `Martensite` CI workflow
  for PR validation and multi-target releases.
- Added the `martensite-tkernel` crate, the foundation toolkit: typed errors,
  memory managers and allocators, collections, ASCII and extended strings,
  operating-system services, messages, quantities, units and resources
  ([#72]).
- Added the `martensite-cmath` crate, which calls the platform C math library
  for `asinh`, `acosh`, `atanh` and `remainder` so results match the C
  functions bit for bit ([#72]).

[#72]: https://github.com/Oblikovati/Martensite/pull/72
[#75]: https://github.com/Oblikovati/Martensite/issues/75
[#76]: https://github.com/Oblikovati/Martensite/issues/76
[#77]: https://github.com/Oblikovati/Martensite/issues/77
[#78]: https://github.com/Oblikovati/Martensite/issues/78
