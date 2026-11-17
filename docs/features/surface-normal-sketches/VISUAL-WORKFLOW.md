# Cylindrical sketch workflow acceptance

The maintainer supplied seven screenshots on 2026-11-07 as the intended
application workflow. The kernel must provide the modelling operations that
make this workflow possible. These images are requirements references, not
numerical geometry oracles; they contain no recoverable exact body dimensions,
font specification or feature-depth direction.

## Observed sequence

1. Start with a cylindrical body and select its lateral face.
2. Define a rectangular sketch area directly on that face. Increase its
   circumferential width while its boundaries remain on the cylinder.
3. Place two lines of text, `This thing` and `works!`, inside the wrapped area.
   The outlines follow the curved surface when viewed from another angle.
4. Produce a body feature from those outlines. The final image shows visible
   letter edges and side walls. Raised and recessed features remain required;
   the image alone does not establish which depth direction was selected.

## Kernel requirements

| Capability | Required result | Delivery |
| --- | --- | --- |
| Face attachment | Resolve a selected cylindrical face into an oriented chart, with a support revision and authored placement | #420 |
| Surface coordinates | Map physical circumferential and axial sketch coordinates to the support and back; preserve placement across the angular seam | #420, #419 where periodic topology is needed |
| Editable sketch area | Change placement and dimensions without losing authored identities; separate layout bounds from actual modelling profiles | #420 and #425 |
| Text outlines | Accept positioned glyph contours, preserve lines and Bezier curves, disconnected glyphs and inner contours; support multi-line placement | #425 and #418 |
| Wrapped regions | Construct the actual curved cap and side geometry for each accepted profile, including holes inside letters | #418 |
| Normal depth | Offset each profile point along its local normal; preserve the same glyph parameter correspondence through depth | #418 |
| Body feature | Non-destructively add or remove the wrapped tool from the selected body, with composed history | #420 |
| Replay | Recompute after text, placement, depth or support edits; refuse ambiguous attachment and invalid intersections | #420 |

The UI owns selection gestures, handles, highlighting and camera interaction.
Martensite must expose the geometry and typed results those interactions need.
A rectangle used to position text must not become a solid rectangular plaque
unless the application explicitly selects that region as a feature.

Font-dependent layout must be reproducible. Acceptance fixtures need a pinned,
redistributable font or owned vector contours with documented provenance.
Installed system fonts and screen pixels cannot define the geometry baseline.
Text shaping/layout and geometry construction should remain separate so an
application can supply already-shaped glyph outlines.

## Existing components and gaps

`FontFTFont` supplies lines and quadratic/cubic Bezier outlines.
`StdPrsBRepFont::plan_text` supplies positioned text items and reusable
`BRepFontPlanarRegion` contours. Degree-2/3 clamped profile splines can preserve
those Bezier curves without flattening them. The adapter belongs above the
modelling toolkit; `tksketch` must not acquire a viewer dependency to obtain text.

The existing formatter supports newlines and alignment, but its `f32` layout
coordinates need a modelling-precision review. The inspected path iterates
characters and advances; complex-script shaping is not established by this
workflow. Accepting externally shaped glyph placement keeps that boundary
explicit. Persisted text identity also needs font content/face and layout
settings: the current in-process font owner cannot identify a replayed feature.
Repeated letters require distinct instance identities and font substitution
must be explicit.

`BRepAdaptorSurface` supplies analytic cylinder recognition and `TNamingSelector`
supplies existing selection machinery. Their use still needs an owned support
revision, located/oriented face handling, trim checks and ambiguity refusal.
Interactive world-point mapping must retain an unwrapped seam choice while
converting to physical arc-length/axial coordinates.

[Task #425](https://github.com/Oblikovati/Martensite/issues/425) tracks the text
adapter and this end-to-end workflow, using #418 and #420 for wrapped geometry
and body integration.

## End-to-end acceptance

Use owned outlines for the displayed two-line text, including curved contours,
letter counters and detached components such as dots and punctuation. Verify
outward emboss and inward engrave on an existing cylinder. Check actual
curve/pcurve agreement, per-face geometry, shared incidences, inner loops,
normal depth, body validity and complete source history. Volume evidence must
include an independent developed-area integral and reported numerical error.
The same case must survive a seam-crossing placement, edits and deterministic
replay, while leaving its input body and earlier published results isolated.

Rendering is useful for application review, but cannot replace those kernel
checks. An apparently wrapped display mesh is not a completed modelling result.
The cylinder rectangle and cone-sector operations establish bounded surface
primitives; neither completes this text-on-body acceptance case.

## Supplied references

The originals in the repository root were preserved. These copies retain their
original bytes and filenames.

| Reference | SHA-256 |
| --- | --- |
| [Screenshot_2026-11-07_14-50-32.png](references/Screenshot_2026-11-07_14-50-32.png) | `bccf61b955c92b60629f9af9f36060a94635b10f349ab91192ba6de43f32ee25` |
| [Screenshot_2026-11-07_14-50-45.png](references/Screenshot_2026-11-07_14-50-45.png) | `b201b02e9bbb8f87b43e7e8bd9b9c0f6cf72c31a667eed3e898bbec6d1a83cb6` |
| [Screenshot_2026-11-07_14-51-02.png](references/Screenshot_2026-11-07_14-51-02.png) | `3920152d83197bada6a94d82a345bdc6b57894ca6ddf81db0a075c3095d00513` |
| [Screenshot_2026-11-07_14-51-11.png](references/Screenshot_2026-11-07_14-51-11.png) | `d5bef3d9eb9a98790cc8b01cb09602d3e2ead11cc921e8ab0c66b8358943c9cc` |
| [Screenshot_2026-11-07_14-52-19.png](references/Screenshot_2026-11-07_14-52-19.png) | `92f15e31abfac65353745c27404d7f1b1458a50ad0e16228418a08eabae1c9da` |
| [Screenshot_2026-11-07_14-52-27.png](references/Screenshot_2026-11-07_14-52-27.png) | `8f8dc4e9e9c00ad0e03d532c3ff167059dcd3412e75adc6e6b5548acb7e98410` |
| [Screenshot_2026-11-07_14-52-37.png](references/Screenshot_2026-11-07_14-52-37.png) | `96b1a2f0d9ecdce7323cb062aa0be69b45f1bd84f3a358f0c6c9ce27183ea600` |
