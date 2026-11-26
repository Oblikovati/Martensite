# Blend General Fuse Assembly (#44)

`GeneralFuseBlendAssembler` in `martensite_tkfillet::brep_fillet_api` assembles filleted
and chamfered solids directly through `BRepAlgoAPIBuilderAlgo` (General Fuse) with
`non_destructive: true`, `fill_history: true`, `HistoryGraph` provenance (`#18`),
`OperationError` (`#6`), and sealed `OperationResult` (`#27`), with zero `TopOpeBRepBuild`
dependency.

## Strangler Migration Gate

- **Current status:** `GeneralFuseBlendAssembler` is live and verified against analytic
  volume, face count, and `BRepCheckAnalyzer` validity for fillet and chamfer solids.
- **Deletion unlock:** Remaining legacy `TopOpeBRepDS` interference-writer paths inside
  `chfi3d` are scheduled for full removal once `#45` (blend regression corpus) and `#46`
  (blend spine/marcher decoupling) complete in Wave 2A.
