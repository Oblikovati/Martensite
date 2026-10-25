# Package output lookup regression evidence

Code commit `4ba2f65ed46e4805b3fda5613a4cbbffb03a8463` fixes Bug #285.
The receipt binds the changed files, test commands and retained logs.

The causal control reverses only the archive lookup argument to the old
checkout-relative `target` directory. The final test file and every expected
result remain unchanged. Cargo successfully packages and verifies all four
fixtures; the default case passes and the three configured cases fail when the
driver looks under the absent checkout directory. The patch and before/reversed/
restored file hashes are retained. Restoring the committed lookup makes the same
four tests pass. The temporary reversal was never committed.

The local PR gate passed unsafe inventories, formatting, strict Clippy, build
and all 187 driver tests. It ran on the working-tree bytes later committed as the
code revision. The frozen causal controls ran after that commit. No package
version, lockfile, SDK isolation rule or integrity check changed. Publication,
merge and native release gates remain coordinator work.
