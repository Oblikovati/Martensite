# Coherent face inputs

Validation capture needs one simultaneous read view of every mutable cell that describes an admitted face. The capture protocol gathers a finite typed owner inventory, acquires nonblocking read leases in deterministic cell order, and revalidates references while all leases remain held. A changed reference, busy or poisoned cell, unsupported representation, cancellation, or exhausted capture limit leaves the capture incomplete.

The geometry leases retain actual line, plane, extrusion and spline cells. Their copy methods use the retained guard without calling another getter. Closed quadratic spline admission shares the existing owned-input checks. Existing blocking geometry methods keep their behavior.

The finite face domain includes extrusion supports with clamped quadratic spline bases, line or stored spline pcurves and line or supported spline spatial curves. Plane supports and stored two-dimensional spline layouts may also be retained as other representation maps; their presence does not grant them an extrusion certificate. Vertex point-representation lists must be empty. Locations contain immutable datum chains and are copied under their containing representation lease. The snapshot also retains the actual cached effective transform, whose rounded product can differ from exact recomposition of the datum chain. Evaluation overrides are retained explicitly and cannot pass stored-spline admission. Other representation kinds remain unsupported.

Cancellation callbacks run before acquisition and after every guard has been dropped. No callback or exact numerical calculation runs while leases are held. Guards stay on the capturing thread; published numeric inputs own their fields and can be shared between threads. A snapshot describes the captured input only and cannot certify a later mutation.

Feature #213 tracks this capture prerequisite. Support self-exclusion and face trim, seam, geometry-consistency and tolerance validation remain separate operations.

## Owned output

`brep::face_input_capture::capture_closed_extrusion_face` returns the root occurrence and typed field copies. Owner and datum labels are invocation-local aliases joined by actual source identity. They preserve repeated occurrences and shared references, with no process pointers in the published input. Canonical traversal assigns labels before address-sorted lease acquisition, so allocation addresses cannot change the serialized graph.

The snapshot includes topology state words and ordered child occurrences, face and edge tolerances, edge validation flags, both seam pcurves and cached endpoint fields, representation ranges, location chains and their effective cached maps, vertex points, line axes, spline arrays and layout, and extrusion direction and basis binding. Meshes are outside this geometry-validation input contract. Vertex point representations are explicitly unsupported when nonempty. Capture does not establish topological validity, trim closure, geometric consistency, tolerance consistency, or support injectivity.

The tests use real mutable cells for writer contention, poison, changed bindings, source mutation and post-capture cancellation callbacks. Independent stored values cover repeated topology and distinct seam bindings. Compilation, affected-package checks and thread-sanitizer validation are required before publication.
