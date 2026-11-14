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
wrapped boss/pocket operation, analytic hit-to-developed mapping with seam and
support-trim policy, authored-history composition, and deterministic replay of
those operations. The Boolean test exercises native history integration; it does
not qualify wrapped multiline emboss/engrave or close #420. The wider feature
also retains its conical-profile, performance, sanitizer and P6 exit requirements.
