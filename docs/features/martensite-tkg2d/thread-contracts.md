# Thread bounds of planar geometry types

The compiler checks in `tests/thread_type_contracts.rs` cover the public concrete
types below. They require the actual type to implement both `Send` and `Sync`;
they do not infer those bounds from a handle, `Clone`, an internal lock, or a
container name. No unsafe implementation is added for these checks (#105).

These are structural transfer and shared-reference bounds. They do not establish
immutable geometry, isolated copies, atomic multi-call snapshots, or deterministic
results while another caller edits the same geometry. Handles can still refer to
the same mutable object. Keep edits separate from evaluation and explicitly copy
geometry when an operation needs input isolation. Borrowed evaluators and views
must not outlive their source, even when their types satisfy these bounds.

## Concrete coverage

| Public types | Compiler check |
| --- | --- |
| `Geom2dCurveD1`, `Geom2dCurveD2`, `Geom2dCurveD3`; Cartesian point, direction, vector with magnitude, axis placement, transformation; line, circle, ellipse, hyperbola, parabola, Bezier, B-spline, trimmed and offset curve objects | `geometry_values_have_structural_thread_bounds` and `curve_objects_have_structural_thread_bounds` |
| `ProtectedAccess`, `KnotData`, `GivenWeights`, `TrimmedBounds<dyn Geom2dCurve>` | Same curve-object check |
| Geometry, curve, bounded-curve, conic, point, vector, adaptor and representation-descriptor trait objects | `public_geometry_interfaces_have_structural_thread_bounds` |
| `Adaptor2dD3`, `Adaptor2dCurve2dBase`, `Adaptor2dLine2d`, `Adaptor2dOffsetCurve`, `Geom2dAdaptorCurve` | `adaptors_and_their_public_snapshots_have_structural_thread_bounds` |
| `OffsetData`, `BezierData`, `BSplineData`, `CurveDataVariant`, re-exported `SharedBSplineCache`, `KnotSpans`, `CacheSource<Point2>` and `CacheSource<Point3>` | Same adaptor check |
| `Map1d`, `Domain1d`, `RepCurveKind`, full/derivative-bounded/mapped descriptors, `MappedCurveState<dyn Geom2dCurve>`, all six analytic evaluation curve objects | `evaluation_descriptors_and_curves_have_structural_thread_bounds` |
| `AhtBezierData`, `TBezierData`, `AhtView`, `TBezierView`, `BezierCurveArrays`, each instantiated with `Point2` and `Point3`; `BasisSums<Xy>`, `BasisSums<Xyz>`, `BasisDerivatives` | `numeric_basis_storage_has_structural_thread_bounds_in_both_dimensions` |
| All eight specialized grid evaluators, `Geom2dGridEvalOtherCurve`, `Geom2dGridEvalCurve`, `Geom2dGridEvalEvaluator`, and all three grid derivative-result aliases | `grid_evaluators_and_results_have_structural_thread_bounds` |
| All thirteen geometry hashers, including macro-generated hasher types | `hashers_have_structural_thread_bounds` |

## Conditional generic types

Generic helper types follow the bounds of the values and references they actually
store. The tests above cover the shipped two- and three-dimensional numeric
instantiations; they do not assert that arbitrary caller-defined types are safe
to transfer or share.

| Generic surface | Required distinction |
| --- | --- |
| `EvalCoord`, `EvalPole`, `OffsetBasisCurve`, `SplineCurveKind`, `BezierCurveKind`, `BSplineCurveKind` | These extension traits do not themselves require `Send + Sync`. A caller that crosses a thread boundary must require the appropriate bounds for its implementation and associated types. |
| `BasisSums<C>`, pole storage, basis views, `BezierCurveArrays<P>`, `CacheSource<P>` | Owned elements and borrowed element references determine the auto traits. A borrow also keeps its original lifetime; a compiler assertion using `'static` does not extend a real borrow. |
| `BezierCurveCore<K>`, `BSplineCurveCore<K>` | Bounds depend on `K::Point` and `K::EvalRep`, including any referenced geometry. The private-module test `planar_spline_storage_and_result_aliases_have_direct_thread_bounds` checks the shipped planar storage types directly, without wrapping them in a lock. No guarantee is made for an arbitrary custom representation. |
| `KindVector<K>`, `KindD1<K>`, `KindD2<K>`, `KindD3<K>` | These aliases inherit the associated point/vector bounds. The same private-module test checks all four shipped planar aliases directly; arbitrary implementations need their own bounds. |
| `MappedCurveState<C>`, `TrimmedBounds<B>`, `TrimBasis` | The retained handle and its geometry contract determine the bounds. Structural bounds do not make shared geometry an isolated input. |

## Thread-confined callback view

`geom2d_hash::hash_key::KnotAccess` borrows two erased `Fn` callbacks without a
`Sync` bound. The callback view is thread-confined, even if a particular closure
happens to be shareable. Its rustdoc compile-fail check rejects a `Send + Sync`
requirement. This preserves the existing callback API and does not add a blanket
thread-safety assertion or change callback ownership.
