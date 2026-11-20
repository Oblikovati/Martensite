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
classifications always refuse. These policies concern hit authoring; full-profile
support-trim acceptance for boss/pocket operations remains open.

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
boss. Changes to text content, seam-crossing attached text, inward edit
replay and general selected-face trim certification remain separate gates.

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

This establishes the attachment foundation. #420 still requires the complete
wrapped boss/pocket operation, complete hit-to-sketch integration and profile
support-trim policy, authored-history composition, and deterministic replay of
those operations. The Boolean test exercises native history integration; it does
not qualify wrapped multiline emboss/engrave or close #420. The wider feature
also retains its conical-profile, performance, sanitizer and P6 exit requirements.

The combined local campaign passed 25 attachment/chart tests in 0.27 seconds
(`target/qualification/p420-chart-final.log`). Six chart controls include a real
native ray hit, explicit seam coordinates, arithmetic underflow/overflow, stale
revisions, nonfinite parameters, and outside reporting/refusal. These are separate
from the completed 19-test attachment checkpoint above.

Strict TKSketch all-target Clippy also passed for the chart checkpoint
(`target/qualification/p420-chart-clippy.log`). Independent Standards and Spec
reviews found no remaining defect in the mapping contract.
