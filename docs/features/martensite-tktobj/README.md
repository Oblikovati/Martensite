# Document object model

`martensite-tktobj` provides document objects, partitions, references, attributes and model iteration (#217).

File saves prepare object fields before writing. The storage driver writes into
an isolated directory beside the destination, and a completed file is synced
and renamed into place before its final path is recorded. A missing driver,
preparation error, writer error, or rejected writer status leaves an existing
destination unchanged and leaves a new destination absent (#245). Preparation
errors also restore the document's modification mode.

Publication is atomic for each file. A document and its referenced documents
are not committed as one transaction. The containing directory is not synced,
so this contract does not guarantee recovery after power loss. Drivers must
produce the document as one file; auxiliary files in the staging directory are
not published.

## Verification

Run `cargo test -p martensite-tktobj`. The suite covers object ownership, cloning, iterators, attributes, references, model changes and serialization contracts, including sentinel preservation on failed saves and final-path metadata after successful saves.
