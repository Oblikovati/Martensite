# Windows document stream inspection (#248)

The document access boundary rejects existing destinations with named streams
before changing staged metadata. Search handles have one owner and close on every
return path. Only the unnamed byte stream is accepted; enumeration errors retain
their native codes and refuse publication.

The three system-call blocks invoke `FindFirstStreamW`, `FindNextStreamW` and
`FindClose`. Their safety comments cover the live UTF-16 path, writable output
layout and unique handle lifetime. The existing foundation system-boundary
exception applies; no dependency or public interface was added.

A native Windows x64 MSVC capture with Rust 1.99.0 compiled the complete workspace
and all targets with default features. Its only added summary group has six
primary spans: each of the three reviewed blocks appears in the library and its
test compilation. Full diagnostics and byte spans were checked against the source;
every existing workspace summary group remains unchanged. The baseline records
the capture hash and the exact patch-equivalent product revision separately from
its checkout base.

Twelve native Windows access-policy tests passed, including ordinary saves and
refusals that preserve original bytes, destination streams and staged policy. The
document writer's native typed-error and staging-cleanup regression also passed.
The final PR and release checks still apply to their exact revisions.
