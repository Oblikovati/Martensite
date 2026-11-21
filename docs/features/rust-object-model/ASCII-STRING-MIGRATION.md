# AsciiString migration (#588)

`AsciiString` owns its terminated byte vector, with up to 15 content bytes
stored inline. On a 64-bit build the string occupies the same 24 bytes as
a `Vec<u8>`. Callers that need a view borrow `as_bytes()` or
`as_bytes_with_nul()`. Inline bytes move with the string, so an address
captured from a borrow may change when the value moves. Copy the bytes to
keep them across a mutation or ownership transfer. An ownership transfer
still moves a spilled allocation without copying its bytes. The old
`buffer_identity` method and `AsciiStringBufferIdentity` type are gone.

`assign_cat` still accepts any `Into<AsciiString>`. Call
`assign_cat_c_bytes(Some(input.as_bytes()))` to append a borrowed `&str`
through its first NUL without allocating a temporary string. Use `append`
for a borrowed `AsciiString`, which copies its full extent. An explicit byte
slice passed to `assign_cat` also keeps embedded NULs.

## tkv3d filename access

The sole allocation observer outside `tkernel` was
`AisTexturedShapeFileRef`. Migrate `AisTexturedShapeMembers::texture_file`
to return an owned `CString`. Read `AisTexturedShapeFields::texture_file`
under the existing fields guard, borrow the prefix with
`CStr::from_bytes_until_nul(file.as_bytes_with_nul())`, and copy it before
releasing the guard. The stored terminator guarantees that the prefix exists.
Callers can inspect the copy without retaining a read lock across callbacks.
Protected fields stay byte strings, including any bytes beyond the first NUL.
The Rust source search found no other production consumer that retained an
`AsciiString` buffer address across a move. All `HAsciiString::to_c_ptr`
callers in the workspace are tests.

Remove the old observer and its raw pointer method. Update the native filename
tests to verify empty, shortened, replaced and embedded-NUL filenames on each
read, and verify a previously returned copy survives mutation and owner drop.
Keep the texture retirement tests for callback and failure ordering.

## Verification

Five paired release rounds on Linux x86_64 used one logical CPU (CPU 17).
Its sibling was busy less than 1% of the measurement window. Each round
recorded a median of seven case runs. The two Rust binaries used the same
benchmark code and compiler flags. The baseline linked the `tkernel`
release library from `5fb67dbcc`. All case checksums matched OCCT 8.0.

| Case | OCCT ns/op | Previous Rust ns/op | New Rust ns/op |
| --- | ---: | ---: | ---: |
| ASCII span search, both directions | 641.622 | 199.273 | 50.066 |
| ASCII append and search, borrowed input | 11.374 | 1565.979 | 5.957 |
| ASCII append and search, generic input | 11.342 | 1574.007 | 10.823 |
| ASCII integer formatting | 39.381 | 45.495 | 15.962 |
| ASCII real formatting | 79.989 | 175.229 | 70.492 |
| Extended string append and search | 8.419 | 6.649 | 6.801 |
| Shared ASCII append and search | 9.898 | 1609.153 | 14.381 |
| Shared extended construction from ASCII | 29.768 | 49.640 | 43.179 |

The standalone `cargo xtask occt-parity --filter ascii` run also matched all
checksums. It reported both shared-string cases above native time. The
shared-string cases include synchronization around mutable string storage
that the native classes lack. The table covers these workloads. Further
parity cases for consumed operations remain part of #588.

Both storage samples kept one million strings live, appended a two-byte
suffix to each and searched for it. The mixed sample used equal numbers of
empty strings, short names, 54-byte names and signed-integer constructors.
Each row reports the median of five pairs. The checksums matched.

| Storage sample | Previous wall (s) | New wall (s) | Previous peak RSS (KiB) | New peak RSS (KiB) |
| --- | ---: | ---: | ---: | ---: |
| Short names | 0.127097 | 0.031768 | 106436 | 28044 |
| Mixed | 0.126289 | 0.043429 | 117960 | 47756 |

A direct byte comparison against the native real constructor matched all
268,996 inputs. The corpus included 250,000 pseudorandom 64-bit patterns and
values around decimal exponent and rounding transitions. The output SHA-256
was `609669fd73a0c332bb539182093d5137f257d842e3558d923e74cd6c321605bb`.
The test suite also pins native ties, signed zero and the rounded exponent
boundaries. For the lenient extended-string decoder, OCCT and both Rust
revisions decode `80 e2 82 ac bf` to `0080 20ac 00bf`. The mixed case
distinguishes decoding the euro sign from widening each byte. All 5,603
enabled `tkernel` tests passed, with eight ignored.
