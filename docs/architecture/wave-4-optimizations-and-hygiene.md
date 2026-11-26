# Wave 4 Optimizations, Receipts, and CI Hygiene

This document tracks the Wave 4 (`priority: low`) polish, micro-optimization, and CI hygiene items (`#96`, `#115`, `#116`, `#126`, `#145`, `#253`, `#341`, `#344`, `#373`, `#389`, `#424`, `#427`–`#431`, `#436`, `#443`, `#444`, `#458`, `#471`, `#497`, `#506`, `#511`, `#512`, `#527`, `#545`, `#564`, `#565`).

## Implemented Contracts

1. **Owned `tkernel` Storage & Extension Contracts (`#96`, `#115`, `#116`, `#145`, `#373`):**
   - `martensite_tkernel::storage::owned_extensions` provides `SeekOrigin` + `seek_from_origin`, `CheckedBorrowedInput`, `OwnedDynamicRootView`, `ExceptionAncestry`, and `GapCleanupPolicy`.
2. **Owned Font Coverage Results (`#341`):**
   - `martensite_tkservice::font::owned_result` provides `FontConsumedCoverage` and `FontOperationResult<T>`.
3. **Conical Receipts, Ordered Contact Index, & Polynomial Normalization Cache (`#497`, `#511`, `#512`, `#527`, `#564`, `#565`):**
   - `martensite_tkgeomalgo::wave4_contracts` provides `ConicalSourceReceipt` (recording `request_radius` and `chart_center`), `OrderedContactIndex` (O(log N) binary-search contact cardinality), and `NormalizedPolynomialCache`.
4. **Vendor Named B-rep Adapter (`#545`):**
   - `martensite_tkdestep::vendor_adapter` provides `VendorShapeAdapter` and `NamedVendorShape` for converting external named B-reps (such as Corliss Inventor assemblies) into deterministic named Martensite shapes.

## Audited Notes & Policies

- **`nix` Dependency Audit (`#126`):** Track upstream `nix` release beyond `0.31.3` for `Dir::from_fd` descriptor cleanup on error; no workspace crate currently calls `Dir::from_fd`.
- **Lavapipe Point Size-1 Palette Policy (`#389`):** Pinned software lavapipe drops sub-pixel size-1 point samples on some rasterizer alignments; point palette tests should use size >= 2 or explicitly document lavapipe point-size quantization.
- **CI Bounded Artifact & Runner Hygiene (`#344`, `#471`, `#506`):** Local PR validation should export `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_DEV_DEBUG=0`, `CARGO_PROFILE_TEST_DEBUG=0`, and `CARGO_PROFILE_DEV_STRIP=debuginfo` to match CI artifact bounds.
- **Wrapped-Cone / Cylinder Micro-Optimizations (`#253`, `#424`, `#427`–`#431`, `#436`, `#443`, `#444`, `#458`):** Deferred to targeted profiling passes when customer models exercise those wrapped-support paths.
