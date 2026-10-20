# Application data framework

`martensite-tklcaf` provides the label tree that applications store their data in:
labels and attributes, transactions with undo and redo deltas, standard attributes,
a function mechanism, and documents with external links and multi-document
transactions. It builds on the document framework of `martensite-tkcdf`.

| Module | Capabilities |
| --- | --- |
| `tdf` | Labels, attributes, the data framework, transactions, deltas, copy and closure tools. |
| `tdata_std` | Standard attributes: values, strings, arrays, lists, maps, tree nodes, expressions, variables and relations. |
| `tfunction` | Functions, their drivers, the dependency graph, the logbook of modified labels and the execution iterator. |
| `tdoc_std` | Documents and applications, undo and redo, external links and multi-document transactions. |
| `json` | The `DumpJson` pieces the attributes share. |

```rust
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tklcaf::tdata_std::TDataStdInteger;
use martensite_tklcaf::tdoc_std::TDocStdDocument;

let document = TDocStdDocument::new(&ExtendedString::new("Example"))?;
document.set_undo_limit(10)?;
document.new_command()?;
let root = document.get_data().get().expect("data").root();
TDataStdInteger::set(&root, 7)?;
document.commit_command()?;
assert!(document.undo()?);
# Ok::<(), martensite_tkernel::standard::Failure>(())
```

## Ownership, transactions and thread safety

Labels, attributes and documents are shared handles over locked state; cloning a
handle keeps identity. An attribute changed inside an open transaction is backed up
once, and only when its value changes; committing records a delta that undo and redo
apply. Aborting restores the backed-up values. Setting the value an attribute already
holds produces no delta.

Derived attributes register their creators the first time the registry is read; a
dependent crate calls `TdfDerivedAttribute::register` for its own attributes before
reading the registry. Registration is process-wide.

The framework does not publish immutable modelling results. A caller that needs an
isolated copy uses the copy tools (`TdfCopyLabel`, `TdfClosureTool`) or builds a new
document. Documents are not designed for concurrent mutation from several threads.

Storage formats are not part of this crate: an application registers the readers and
writers of the formats it needs. The tests never read or write a document file.

## Verification

Run `cargo test -p martensite-tklcaf`. Tests cover labels, attribute iteration, copy
and closure, every standard attribute (values, arrays, lists, maps, tree nodes,
expressions), transactions, undo and redo of unchanged and changed values, external
links, the function mechanism, multi-document transactions and dumps.

Regression scenarios exercise every create/modify/forget combination across two
commands and their abort (5,832 variants), undo after a limit change, and the dump
of a multi-document transaction manager. Two timing regressions build 100,000 labels
and check that committing or aborting one modification costs at most a thousandth of
the creation; they run in about twenty seconds in debug builds. The bound was recorded on Linux x86-64;
Windows and macOS CI runners measure one commit or abort above it, so the two timing tests
are ignored there and stay mandatory on Linux.

The preparation ran on Linux x86-64. The release candidate still needs the full
supported platform matrix.
