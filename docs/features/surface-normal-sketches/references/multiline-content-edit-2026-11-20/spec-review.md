# Spec review: multiline content/seam/pocket replay

Reviewed the three-path test-only delta against `811ed5968fd5905859f8e73a82e904b067093eec`. All files match `/tmp/martensite-multiline-content-edit-20261120/source.json`, SHA256 `146062861754b6ba11e7a72efc0fe855696876cf07c1b31ba5c374f8935e2462`, before and after inspection.

Verdict: **one coverage blocker; fixture/edit/replay design otherwise accepted.**

The removed standalone pocket test previously required 18 reports, unchanged encoded result bytes across native export/Boolean attachment, and unchanged input request. The replacement original-pocket publication retains its volume/body/history/cap assertions but shared `replay::publish` omits those three checks. Preserve them there: compare report count with selected region count, snapshot the fully placed request before execution, and snapshot encoded bytes before export/Boolean then compare afterward. This covers original 18 and edited 16 regions without another operation. Dropping the standalone registration is acceptable after its assertions are preserved.

The authored filter removes prefix15 from curves, vertices and selections consistently. Direct fixture inspection confirms it is the exclamation instance, with ten curves and two material regions. Independent exact Green re-integration gives remaining area1265699/196608 and inward volume99990221/62914560; the original pocket constant663607031/402653184 is retained unchanged.

The first connected T outline has vertices on both sides of the explicitly applied native seam shift; the witness evaluates the same horizontal shift used by publication. Both resulting bodies still require ordered validation, one solid, all selected cap faces and output-bound tool history. Changed body/tool bytes and exact surviving authored region/vertex keys prevent a no-op edit from passing.

The child reads the parent’s actual saved BRep record bytes through stdin; fixture metadata is asserted before transport and restored explicitly. Exact ignored-worker invocation, process status and required worker output prefix prevent an empty test selection from qualifying replay. Full signatures compare edited body/tool bytes and authored history.

The exact case is compiling/running; no pass or performance claim is inferred. This establishes a specific content/seam scenario, not general support-trim certification. No builds or edits were performed.

## Corrected source follow-up

Final source/spec verdict: **ACCEPT; coverage finding resolved.** Three paths match refreshed freeze `7e09264c2f13961283004dd446902d124956ba2ca060f829706d51b30d9de38a`. Shared publish checks report count and original request equality; attach_checked snapshots tool bytes before export/Boolean and verifies them afterward for either normal. The original full pocket volume baseline remains reused. Fresh-worker completion is pending; no final execution pass is inferred.
