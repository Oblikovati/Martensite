# Exchange Format Capability Matrix (#58)

| Format | Direction | Analytic Solids | Freeform (NURBS/Bezier) | Open Shells & Wires | Units & Assembly Metadata | Platforms |
| --- | --- | --- | --- | --- | --- | --- |
| STEP AP203 | Import / Export | Supported (checked) | Supported | Supported | Units, product names | Linux, macOS, Windows |
| STEP AP214 | Import / Export | Supported (checked) | Supported | Supported | Units, colors, assemblies | Linux, macOS, Windows |
| STEP AP242 | Import / Export | Supported (checked) | Supported | Supported | Units, PMI/GD&T, assemblies | Linux, macOS, Windows |
| IGES 5.3 | Import / Export | Supported | Supported | Supported | Units, layers | Linux, macOS, Windows |
| Native B-rep (`PersistentBodyArchive`) | Save / Restore | Exact round-trip | Exact round-trip | Exact round-trip | Persistent entity names | Linux, macOS, Windows |

## Independent Round-Trip Verification

- `martensite-tkdestep::exchange_contract` checks per-face surface types, face count, achieved max tolerance, BRepCheck validity, and preserved source identities across STEP export/import round-trips (`#57`, `#58`).
- `martensite-tkshhealing::explicit_repair::repair_on_copy` runs healing explicitly on a detached copy and returns a `HistoryGraph` remap from original to repaired entities (`#59`).
