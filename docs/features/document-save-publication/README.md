# Document save publication

Bug [#245](https://github.com/Oblikovati/Martensite/issues/245) changed document
saving to write into a private sibling directory before replacing the requested
file. Missing drivers, writer errors, incomplete writer output and preparation
failures leave the existing file intact. Successful publication uses one
same-filesystem rename after access-policy verification and file synchronization.

The document writer resolves destination links, including a final dangling link,
and retains the requested path in document metadata. The save requires write
access to the destination parent and permission to restore the destination's
access policy. This can refuse a save that an in-place write previously allowed.

## Access-policy boundary

TKERNEL exposes two safe functions in `osd`:

| Function | Contract | Native fixtures |
| --- | --- | --- |
| `create_private_document_directory(&Path) -> io::Result<()>` | Creates an exclusive directory that protects staged document bytes. Existing paths are refused. | Owner-only staging and inherited-grant removal. |
| `preserve_document_access(&Path, &Path) -> io::Result<()>` | Copies and verifies access policy on a regular staged file. An existing destination must be a resolved regular file. The function does not publish either file. | Ownership, group, mode, ACL, refused policies, unchanged original bytes and staged-file validation. |

For either destination route, the staged file must first match an exclusively
created default-policy reference in its private directory. Ordinary private
staging defaults can then receive the existing destination's policy. An absent
destination receives the final parent's ordinary creation policy. A writer's
custom policy is refused before publication; a restrictive writer mode cannot
become broader.

Linux preserves owner, group, mode, POSIX access ACL, SELinux and Smack labels.
Windows preserves owner, group, ordered DACL and supported access-related SACL
sections, including inheritance protection. macOS preserves owner, group, mode,
ordered ACL and quarantine attributes. Unsupported access policy is refused.
Content-bound capabilities and signatures are not transplanted to replacement
bytes. The platform implementations verify their results before publication.

Windows destinations with named streams, including `Zone.Identifier`, are refused
before access metadata changes or publication (#248). Refusal retains the
original bytes and stream contents, and leaves the staged policy untouched.
Stream-enumeration errors also refuse publication and retain their native error
codes. Ordinary files containing only the unnamed byte stream remain supported.

The CDF writer converts operating-system failures into named `Failure` classes
for missing output, missing destination, permission refusal, invalid destination
or other publication failure. Diagnostics retain the save phase, requested path,
I/O error kind, native code when available and cause text.

## Limits and validation

Publication is per file. It is not a transaction across referenced documents.
Metadata attachment callbacks run after publication and can still fail. File
synchronization does not establish directory crash durability. Callers own the
destination namespace; these functions do not promise protection against another
actor concurrently replacing its path components.

The tests use owned files with independently specified bytes and access policies.
They cover failed writer preparation, missing and incomplete output, existing and
dangling links, restrictive writer policy, ownership and ACL preservation, and
unchanged bytes after refusal. Linux tests run locally. Windows and macOS tests
are compiled locally; execution on each supported native platform remains a
release gate. Cross-compilation alone does not establish native behavior.
