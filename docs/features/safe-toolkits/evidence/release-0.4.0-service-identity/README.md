# Service package identity for SDK 0.4.0

The approved metadata commit `b68700e` advances the visualization service package from 0.3.5 to 0.3.6. A fresh Linux compiler capture produced 449 ordered workspace groups and 2,814 raw workspace diagnostics. All groups match the existing baseline after changing only the package identity in 27 service groups. Locations and emitted counts are unchanged.

The archive retains the original baselines, actual raw compiler streams, structured report, metadata diff and contemporaneous command receipts. The first PR command stopped at the expected inventory mismatch with exit 1. Its source was clean before and after execution; the disk guard did not interrupt it.

The four target expectations change only the service package identity. This archive measures Linux. Windows, Intel macOS and ARM macOS must independently match their complete ordered summaries in the exact-candidate release matrix. No warning count or comparator changes, native-platform pass, or complete release qualification follow from the expectation update.

`receipt.json` binds the archive. Its `MANIFEST.json` names every payload and its hash and length. The raw compiler files are immutable execution evidence, not reusable verification scripts.
