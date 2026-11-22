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

## String API visibility (#591)

The string-only tests now run inside the toolkit's library test target.
This keeps byte/span, locking and ownership coverage for operations that
have no toolkit consumer. The safe string erasure cases also moved into
the library target. The raw `LinearVector` constructor test stays in its
original integration target and source file.

The executable parity table uses exported methods. It keeps generic ASCII
append and search, and uses `ExtendedString::assign_cat` for the UTF-16
append case. The previous table records the borrowed append entry
points and remains a record of those measurements.

| Still exposed | Consumer that needs migration |
| --- | --- |
| `AsciiString::as_bytes_with_nul` | The planned `tkv3d` filename snapshot needs the terminated borrow. |
| `AsciiString::set_value_char` | `tkbool/tests/shape_set_name_bytes/exact.rs` mutates the input after handing off the name. The `tkv3d` filename fixture also uses this operation. |
| `AsciiString::copy`, `move_from` and `swap` | `tkv3d/tests/textured_shape_file_buffer.rs` still exercises the old allocation observer. Retire these calls when replacing the observer with an owned filename snapshot. |
| `AsciiString::set_value` | `tkmeshvs/tests/drawer_replay.rs` and `invariants.rs` overwrite a string after a drawer handoff. A same-owner `assign_from` with the expected bytes can keep the isolation check. |
| `HExtendedString::is_empty` | The `tktobj` object clone and document tests can inspect the value under `string()`. |
| `ExtendedString::from_real` | The Vulkan viewer relation support can append the `f64` through the public generic `cat` operation, which uses the same conversion. |

The `HAsciiStringHasher` name is no longer reexported. Its type is still
the associated policy of the public handle key. `HExtendedStringHasher`
remains exported for `tktobj`'s name index.
