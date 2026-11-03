# Coherent face inputs

Validation capture needs one simultaneous read view of every mutable cell that describes an admitted face. The capture protocol gathers a finite typed owner inventory, acquires nonblocking read leases in deterministic cell order, and revalidates references while all leases remain held. A changed reference, busy or poisoned cell, unsupported representation, cancellation, or exhausted capture limit leaves the capture incomplete.

The geometry leases retain actual line, extrusion and spline cells. Their copy methods use the retained guard without calling another getter. Closed quadratic spline admission shares the existing owned-input checks. Existing blocking geometry methods keep their behavior.

The finite face domain includes extrusion supports with clamped quadratic spline bases, line pcurves and line or supported spline spatial curves. Vertex point-representation lists must be empty. Locations contain immutable datum chains and are copied under their containing representation lease. Other representation kinds remain unsupported.

Cancellation callbacks run before acquisition and after every guard has been dropped. No callback or exact numerical calculation runs while leases are held. Guards stay on the capturing thread; published numeric inputs own their fields and can be shared between threads. A snapshot describes the captured input only and cannot certify a later mutation.

Feature #213 tracks this capture prerequisite. Support self-exclusion and face trim, seam, geometry-consistency and tolerance validation remain separate operations.
