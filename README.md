# Martensite

Martensite is a Geometry Kernel providing services for the 3D surface and solid modeling.

It started as a 1:1 porting of OpenCASCADE APIs to Rust with OpenCASCADE test suite as
validation oracle. Providing a stable and production tested set of toolkits on which
Martensite can build on top and innovate.

Martensite has no commitment to keep API parity with OpenCASCADE.

The improvememt roadmap for this project can be found on [Milestones](https://github.com/Oblikovati/Martensite/milestones)

- [Contributing](CONTRIBUTING.md)
- [Agentic Development Governance](AGENTS.md)

## Toolkits

| Crate | Capabilities |
| --- | --- |
| `martensite-cmath` | Portable rounding for transcendental functions behind a typed Rust interface. |
| `martensite-tkernel` | Errors, memory, collections, strings, operating-system services, quantities and units. |
| `martensite-tkexpress` | Symbolic expressions, differentiation, relations and a text interpreter. |
| `martensite-tkg2d` | Planar curves, splines, offsets, adaptors, evaluation and hashing. |
| `martensite-tkmath` | Geometry, linear algebra, solvers, splines, bounding volumes, meshes and exact predicates. |
| `martensite-tkcdf` | Application documents, document references and metadata, format drivers and a light XML parser. |
| `martensite-tklcaf` | Labels and attributes, transactions with undo and redo, standard attributes, functions and documents. |
| `martensite-tkservice` | Windows and events, images, fonts, the presentation data model and graphic driver interfaces. |
| `martensite-tkbrep` | Topological shapes, their exploration and tools, geometric shapes, the shape graph and the binary shape stream. |
| `martensite-tkgeomalgo` | Curve and surface intersections, hatching, tangent constraints, filling, plates, laws and sketch splines. |
| `martensite-tktopalgo` | Classification, shape construction and sewing, checks, extrema, bounding boxes and global properties. |
| `martensite-tkhlr` | Hidden line removal, contour lines of surfaces and the visible and hidden lines as shapes. |
| `martensite-tkprim` | Primitive solids (boxes, wedges, cylinders, cones, spheres, tori, revolutions, half-spaces) and sweeps. |
| `martensite-tkhelix` | Helix curves, their B-spline approximation and composite helix wires. |
| `martensite-tkshhealing` | Shape analysis, repair, geometry conversion and subdivision. |
| `martensite-tkmesh` | Mesh elements, discretization, triangulation and incremental meshing. |
| `martensite-tktobj` | Document objects, partitions, references, attributes and model iteration. |

See the [foundation guide](docs/features/martensite-tkernel/README.md) and
[math guide](docs/features/martensite-tkmath/README.md), and
[symbolic expression guide](docs/features/martensite-tkexpress/README.md), and
[planar geometry guide](docs/features/martensite-tkg2d/README.md), and the
[document framework](docs/features/martensite-tkcdf/README.md),
[application data framework](docs/features/martensite-tklcaf/README.md) and
[visualization services](docs/features/martensite-tkservice/README.md) guides, and the
[boundary representation](docs/features/martensite-tkbrep/README.md),
[geometric algorithm](docs/features/martensite-tkgeomalgo/README.md),
[topological algorithm](docs/features/martensite-tktopalgo/README.md),
[hidden line removal](docs/features/martensite-tkhlr/README.md),
[primitive solid](docs/features/martensite-tkprim/README.md) and
[helix](docs/features/martensite-tkhelix/README.md) guides for module and test details.
The [shape healing](docs/features/martensite-tkshhealing/README.md),
[mesh construction](docs/features/martensite-tkmesh/README.md) and
[document object model](docs/features/martensite-tktobj/README.md) guides describe
the added toolkits and their verification limits.

## Getting started

Install the development dependencies with the installer for your OS. Each
installer can run again later to update them.

```bash
./install-linux.sh                          # Debian, Ubuntu, Fedora, Arch, openSUSE
./install-macos.sh                          # macOS (Homebrew)
source ~/.local/share/martensite/env.sh     # Linux and macOS, in each new shell
```

```powershell
powershell -ExecutionPolicy Bypass -File .\install-windows.ps1   # Windows (winget)
```

Build and test with cargo. Before you push, run the same gate that CI runs:

```bash
cargo build
cargo test
cargo xtask pr
```

Development and test builds optimize SHA-256 to keep isolated dataset tests from
spending most of their time fingerprinting their executables (#228). Kernel code
keeps its normal debug profile; executable identity checks remain enabled.

CI has one workflow, `.github/workflows/martensite.yml`, and all its checks are
`cargo xtask` commands. [CONTRIBUTING](CONTRIBUTING.md) explains the gates and
the release process.

PR validation checks toolkit Rust source tokens against
`docs/features/safe-toolkits/unsafe-source-baseline.json` when source, build
tooling or inventory inputs change. Unsafe removals must update that inventory;
increases require review against the temporary exception policy in `AGENTS.md`.
The check includes inactive platform code and macro definitions, but does not
expand macros or inspect dependencies. Compiler checks remain necessary (#81).

The [safe-toolkit migration closeout](docs/features/safe-toolkits/closeout.md)
records the accepted refactors, rejected performance trade-offs, and remaining
unsafe code. This release does not claim a fully safe toolkit implementation.

The source code is licensed GPL-2.0-only (`LICENSE-GPL`). Official release
binaries downloaded from this repository whose files match a published hash are
also offered under the Apache License 2.0 (`LICENSE-APACHE`). Each binary
archive contains `LICENSE-APACHE`, and each source package contains
`LICENSE-GPL`. Source files carry no per-file copyright or license header.
