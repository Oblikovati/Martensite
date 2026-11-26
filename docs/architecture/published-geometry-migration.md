# Published Immutable Geometry Migration Notes (#65)

## Summary

`martensite-tkg2d::published` and `martensite-tkg3d::published` introduce lock-free, immutable
published geometry handles:

- `PublishedCurve2d` (`Arc<ImmutableCurve2d>`)
- `PublishedCurve3d` (`Arc<ImmutableCurve3d>`)
- `PublishedSurface` (`Arc<ImmutableSurface>`)

## Key Guarantees

1. **No setters on shared handles:** Published geometry exposes only read/evaluation methods
   (`d0`, `d1`, `d2`, parameter bounds). Shared aliases cannot mutate geometry in place.
2. **Zero locks on evaluation:** All evaluation calls execute directly on immutable values
   without acquiring `SharedState` (`RwLock`).
3. **Copy-on-write editing via operation-local builders:** Call `.to_builder()` to obtain a
   uniquely owned `Curve2dBuilder`, `Curve3dBuilder`, or `SurfaceBuilder`, apply edits, and
   call `.publish()` to produce a new revision without disturbing concurrent readers.

## Migration Path

- Snapshot legacy `Geom*` / `Geom2d*` handles at operation publication boundaries with
  `PublishedCurve3d::from_geom_*` / `PublishedSurface::from_geom_*`.
- Replace in-place `transform` / setter calls on shared geometry with `to_builder()` ->
  `transform(&trsf)` -> `publish()`.
