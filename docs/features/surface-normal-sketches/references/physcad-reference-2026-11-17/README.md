# Existing PhysCAD reference observations

This capsule restores the 2026-11-17 reference receipts to the current capability work. It contains no new application run or performance measurement. The source files came from the earlier `wrapped-normal-api` worktree; their hashes and last commits are recorded in `summary.json`. The archive preserves both original JSON files, including rejected observations and protocol corrections.

The application reference covers cylinder and cone rectangles, ramps, holes, disconnected regions, seam profiles, multiline text, joins and cuts. The complex batch used one warmup and three measured repetitions per accepted case. Input outlines and exact profile selection are preserved in the original receipt.

The owned 239-curve cone text case reported 18 selected regions and native validity in all three measured repetitions, with median request latency 0.383420 s. It lacks an independent full-text conical geometry/error certificate. The matching cylinder case failed authored completeness and the exact-volume oracle: it selected 17 regions and returned volume 1.5517560712352576 instead of 1.6898095831274986. The failed rows remain in the archive and are excluded from accepted-workload claims.

These observations guide behavior and workload selection. They do not qualify Martensite performance: the application timer includes IPC and rebuilding, compiler flags are unverified, host pressure was not admitted, and memory is a sampled process-tree sum. The 239-curve text workload is also different from the current 148-span cone `O.` control. The kernel must preserve complete authored profiles even where the reference application loses a region.

No license key, licensing records, vendor implementation or runnable investigation code is retained.
