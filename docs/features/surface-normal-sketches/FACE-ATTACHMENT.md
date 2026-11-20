# Revision-bound cylindrical face attachment (#420)

`surface_sketch::attachment::CylinderFaceAttachment` captures a selected native
cylindrical face and its complete support body into owned bytes. Capture requires
one exact face use with the supplied orientation. Geometrically identical faces
from another body do not satisfy membership. The application supplies the logical
face identity and a revision token scoped to that support body; it must change
the token whenever that body changes and retain the body association.

Capture and restore decode with checked surface, rigid-location and curve
policies, then require byte-identical re-encoding of the root and selected-face
reference. The original ordered body checks run before publication, with their
actual coverage recorded. A successful native check does not imply exact
self-intersection or geometry certification. The native input must be protected
against concurrent mutation during capture. Published attachments retain owned
bytes and reports; native exports decode fresh owners.

`export_native` requires the current revision token. `reattach` checks the source
identity, revision and complete frozen encoding before consulting native history.
It accepts modified/generated face images and an unchanged exact face use when
history does not mark it removed. Exactly one distinct image must belong to the
new body and remain cylindrical. Missing or ambiguous images refuse the operation.
The application owns persistent naming; this API does not choose a nearest face.

Failures carry the operation, logical attachment identity, source revision,
failure class and owned evidence. Validation failures retain the complete report.
Replay failures identify the first byte difference and both lengths. Failed
operations return no attachment geometry.

## Native hit coordinates and seam choice

`develop_parameters` accepts the selected face's native `(u,v)` hit parameters,
the current revision, a seam angle, and an explicit out-of-support policy.
Existing face intersectors already return these parameters. The method maps them
to `(radius * (u - seam_angle), v)` without projecting the world hit again or
reducing its angular branch. The caller retains the chosen unwrapped parameter.
The seam angle defines zero developed arc length; it does not change native trims.

Coordinate measurements enclose rounding of that expression on the supplied
binary64 parameters and native analytic radius. They exclude uncertainty in the
intersector parameters and deviation of the stored frame from an ideal metric
frame. The chart therefore supports authoring; its arithmetic certificate does
not certify a modelling result or a world-point projection.

Native face classification reports the selected face's trim state separately,
with `NativeSearch` coverage. `Refuse` declines outside points. `Report` returns
their coordinates and outside state for application feedback. Unknown or rejected
classifications always refuse. A boss or pocket checks its entire footprint
separately against the selected face's trims.

## Resolving the extrusion chart

Use `attachment.extrusion_support(current_revision, seam, normal)` when
recomputing a wrapped profile. It decodes the recorded face and reads its
native location, axial and radial directions, and radius. The returned
`CylinderSupport` uses the same unwrapped seam angle and axial origin as
`develop_parameters`. `NormalSense` selects increasing or decreasing
radius regardless of the face's topology orientation. An indirect native
frame refuses because the extruder requires a right-handed chart.

Use `normal_extrude_profile_on_chart` for a nonzero seam angle with the
rotation-aware spatial certificate. The extrusion chart describes the
analytic surface. The attachment record still owns the selected trims and
support body. A whole-profile trim check and Boolean history composition
belong to the attached feature operation.

## Checking a rectangular trim band

`attachment.certify_footprint(current_revision, &request)` checks the
authored footprint in the chart of the selected native face. It checks the
revision and request frame against one decoded attachment. Then it reads
every directed trim occurrence and requires one rectangular outer wire
with exact straight pcurve corners. A full band must have two uses of the
same native seam edge and a span equal to the native period. The seam is
periodic in that case. Distinct angular sides of a partial band remain
physical boundaries.

The shared profile reader resolves the selected original curves and their
controls. A positive Bézier basis keeps each selected polynomial within
its control hull, so strict containment of the controls proves strict
containment of the authored bounded material. The partial band's arc
length interval is rounded inward. Refusals identify the region and
curve, the coordinate, and the required interval. A hull that cannot be
proved inside is inconclusive about the actual curve. Supports with
additional wires or unrecognized trim geometry return a typed refusal.

The [local controls](references/cylindrical-trim-band-2026-11-20/summary.json)
checked a full seam crossing, a partial angular boundary, an axial
boundary, a stale revision and mismatched chart, and a cylindrical
support with a real bore trim loop. The original 239-curve outline and
the 229-curve seam-crossing edit also passed footprint admission on their
saved support. The report covers the authored footprint.

`attachment.extrude_within_trim_band(current_revision, &request)` builds
the normal tool in the same chart. It checks the authored controls before
fitting and sends the captured sources to the existing chart extruder.
The extruder verifies that both stored cap pcurves of each fitted span
match the fitted controls. The trim check uses those controls directly.
For a partial band it subtracts the seam angle from the native angular
bounds and rounds inward. The axial bounds stay in native units. A
failed check identifies the region, curve, fitted interval and boundary.

The tool retains the authored report, the number of checked fitted spans
and the chart extruder's spatial placement report. The trim certificate
covers both cap pcurves in the requested chart, including the interior
of each cap enclosed by its wires. The spatial report accounts for
stored placement separately. General curved or holed supports still need
their own trim checks.

The [fitted-tool checks](references/cylindrical-trim-band-2026-11-20/fitted-tool.json)
decoded both caps of a tool on a partial cylinder with a shifted seam.
The tool matched the standalone chart extrusion bytes and authored
history. A second tool crossed the paired seam on a full cylinder. All
49 selected tests and strict all-target TKSketch Clippy passed.

## Applying a wrapped boss or pocket

`attachment.apply_wrapped_profile(revision, kind, &request)` uses one
decoded support for trim checks and the material operation. Supply depths
from zero to a positive finite depth. A boss follows the normal out of
the solid and a pocket follows the normal into it. On a reversed cylinder
face those senses are reversed relative to the radius.

The operation builds the same trim-checked tool, then calls the native
non-destructive Fuse or Cut. The result must contain one solid without
loose topology. The frozen solid passes ordered body validation at the
attachment's recorded resolution before publication. The result keeps
the validation modes and the tool's spatial and volume evidence. Its
Boolean validation has sampled and native-search stages.

The history joins native Boolean images and removals to the selected
face, the other members of the input solid, and the semantic extrusion
keys. The extrusion history carries those keys back to the authored
regions, curves and vertices. Result references address the immutable
archive. Support member references address the exact revision's decoded
solid. Persistent names across edits belong to the application. Fresh
exports decode independent owners.

Cancellation is polled during tool construction and at the boundaries
of native Boolean construction, history and validation. A native failure
keeps its original cause when cancellation is requested after it failed.

The [public operation checks](references/cylindrical-trim-band-2026-11-20/boolean-feature.json)
passed 44 selected tests and strict Clippy in TKSketch and the TKV3D test
target. The existing multiline edit tests now use this entry point. The
boss replays after support, position and depth edits. The pocket replays
after removing punctuation and moving the first glyph across the seam.
Both fresh processes reproduce the solid archive, extrusion archive and
composed history byte for byte.

## Multiline boss edit and replay

The [local qualification](references/multiline-attached-replay-2026-11-20/summary.json)
passed all 27 attachment controls and the full edited boss. The fixture
retains 239 authored curves and 18 selected regions. It follows an explicit
history image from a radius-10 support to a radius-12 support, restores the
revised record, translates the layout in the developed chart and changes
depth from 0.25 to 0.375. The previous support revision refuses access to
the new chart. Every exposed cap survives the edit, and the original body
encoding and authored region and vertex keys are preserved.

The edited tool and Boolean body replayed byte for byte in a fresh process,
along with the ordered authored history. Both builds checked the exact
fixture volume oracle for the tool, the ordered native body checks, and the
membership of Boolean history images. The three-body test took 387.69 s
in the assertion-enabled build. That includes the original body, edited
body and process replay, and is not an extrusion benchmark.

This covers edits to support, placement and depth for the outward text
boss.

## Multiline pocket content edit across the seam

The [pocket replay](references/multiline-content-edit-2026-11-20/summary.json)
starts from the original 239-curve, 18-region pocket. It removes the
exclamation mark's 10 lines and two regions. A translation of -0.3125 in
developed arc length places the first remaining outline across the native
cylinder seam. The edited Boolean cut keeps one valid solid and all 16
remaining floor faces. Authored region and vertex keys for retained text
are identical. The original body, the two requests and the two frozen
extrusion tools retain their bytes or values.

Independent Green integration gives retained area `1265699/196608`. The
inward radial depth integral is `79/320`, so the volume oracle for the
edited tool is `99990221/62914560`. The test checks that the certified
tool volume encloses this value. The native Boolean volume estimate is
used to check the direction of the cut.

The fresh process reads the saved support BRep from its parent, restores
the selected face record, and rebuilds the edited feature. Both the tool
and cut body match byte for byte, as does the ordered authored history.
The test took 416.24 s for the original pocket, edited pocket and process
replay in the assertion-enabled build. General selected-face trim
certification and periodic profiles beyond this fixture remain open.

## Evidence and remaining work

The local attachment campaign passed 19 tests in 0.24 seconds, including a real
non-destructive Boolean cut whose native history reattaches the trimmed cylinder
face. The original source encoding remains unchanged after that cut. Other
controls cover oblique intrinsic cylinders, ambiguous and removed history,
revision refusals, original-input and export mutation, corrupt records, and
public-type `Send + Sync` assertions. Strict TKSketch all-target Clippy passed
after nested native/encoding diagnostics were boxed. The logs are
`target/qualification/p420-attachment-final.log` and
`target/qualification/p420-attachment-clippy.log`.

This earlier attachment campaign checked native history rebinding. The
remaining #420 work covers general support trims and the complete application
hit-to-sketch workflow. The wider feature retains its conical-profile,
performance, sanitizer and P6 exit requirements.

The combined local campaign passed 25 attachment/chart tests in 0.27 seconds
(`target/qualification/p420-chart-final.log`). Six chart controls include a real
native ray hit, explicit seam coordinates, arithmetic underflow/overflow, stale
revisions, nonfinite parameters, and outside reporting/refusal. These are separate
from the completed 19-test attachment checkpoint above.

Strict TKSketch all-target Clippy also passed for the chart checkpoint
(`target/qualification/p420-chart-clippy.log`). Independent Standards and Spec
reviews found no remaining defect in the mapping contract.
