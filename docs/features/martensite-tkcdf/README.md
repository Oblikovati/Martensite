# Document framework

`martensite-tkcdf` provides application documents, references between documents,
document metadata, the application that opens, retrieves and stores documents through
format drivers, persistent-document reader and writer interfaces, and a light XML
parser and writer.

| Module | Capabilities |
| --- | --- |
| `cdm` | Documents, references between documents, metadata of stored documents and the abstract application. |
| `cdf` | The application: format resources, driver lookup, open, retrieve, store and close. |
| `pcdm` | Reader and writer interfaces, storage-driver statuses, container headers and the reader filter. |
| `ldom` | A light DOM: XML parsing and writing, byte order marks and character references. |
| `json_dump` | The `DumpJson` steps the document classes and dependent crates write. |

```rust
use martensite_tkcdf::ldom::LdomParser;

let mut parser = LdomParser::new();
let failed = parser.parse_file(b"/no/such/file.xml");
assert!(failed);
```

## Ownership and thread safety

Documents, applications and references are shared handles: cloning a handle keeps
the node identity, and the shared state sits behind a lock. Mutations go through the
handle, so callers that need isolation must build a new document. An application
records its owning handle once; drop the state guard before calling back into the
application. The crate does not publish immutable modelling results.

Format drivers are registered per application through its resource manager. The
crate ships the interfaces and the stream and container header readers; it does not
ship the binary or XML document storage formats themselves. A concrete driver
implements `PcdmWriter` or `PcdmReader` and registers its format.

## Verification

Run `cargo test -p martensite-tkcdf`. Tests cover the application services, driver
lookup and resource dispatch, document references and metadata, container header
parsing (binary, compressed and text containers, byte for byte), reader filters,
the stream format probe on borrowed and owned input, and the light DOM.

Frozen inputs are protocol bytes under `tests/*/fixtures` and the light DOM corpus
`tests/data/ldom_reference_corpus.txt`, a reviewed list of `key|value` lines for the
parse, write, DOM, stream and character-reference cases. Baseline changes require
review stating the reason and behavior version.

One test reads a public dataset file (`others/bug31340.xml`, a document with a UTF-8
byte order mark) from the `external/martensite-test-data` submodule, or from
`$MARTENSITE_TEST_DATA_DIR`. It fails with the command to fetch the submodule when
the file is missing, and checks the file's byte count before parsing it:

```sh
git submodule update --init external/martensite-test-data
```

The preparation ran on Linux x86-64. The release candidate still needs the full
supported platform matrix.
