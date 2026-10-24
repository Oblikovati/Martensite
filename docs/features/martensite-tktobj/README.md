# Document object model

`martensite-tktobj` provides document objects, partitions, references, attributes and model iteration (#217).

File saves prepare object fields before writing. The storage driver writes into
an isolated directory beside the destination, and a completed file is synced
and renamed into place before its final path is recorded. A missing driver,
preparation error, writer error, or rejected writer status leaves an existing
destination unchanged and leaves a new destination absent (#245). Preparation
errors also restore the document's modification mode. Saving through a symbolic
link preserves the link, including when its target file does not yet exist.
Publication failures distinguish missing output, denied permissions, missing
destinations, and invalid paths; diagnostics retain the operation, destination,
I/O error kind, OS error code, and cause text.

Saving requires write access to the file and create/rename access to its parent
directory. On Linux, replacement preserves and verifies the owner, group, mode,
POSIX access ACL, and SELinux/Smack file labels. New files receive the final
parent's inherited access policy only when their staged policy matches a default
file created in the same private staging directory. Custom writer policies are
refused for both existing and absent destinations before publication; a restrictive policy is never widened to a default. Staged output must be a regular file; symbolic
links and directories are refused. These checks use the TKERNEL operating-system
interface; the document framework owns writing, syncing, and publication. Unknown system/security access
attributes and set-user/group-ID files are refused before publication. Content
capabilities and integrity signatures are not copied onto changed bytes.

Publication is atomic for each file. A document and its referenced documents
are not committed as one transaction. The containing directory is not synced,
so this contract does not guarantee recovery after power loss. Drivers must
produce the document as one file; auxiliary files in the staging directory are
not published.

## Verification

Run `cargo test -p martensite-tktobj`. The suite covers object ownership, cloning, iterators, attributes, references, model changes and serialization contracts, including sentinel preservation on failed saves and final-path metadata after successful saves.
