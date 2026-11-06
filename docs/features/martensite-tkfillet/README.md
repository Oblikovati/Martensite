# Fillets and chamfers

`martensite-tkfillet` builds constant and evolving fillets and chamfers on shapes.

| Module | Capabilities |
| --- | --- |
| `brep_fillet_api` | The entry points for 3D fillets and chamfers and for planar 2D fillets. |
| `chfi3d`, `chfi2d` | The builders: spines, corners, tangency and the walk along an edge. |
| `chfi_ds` | The stripe, spine and surface data of a blend. |
| `chfikpart` | Closed-form blends between planes, cylinders, cones and tori. |
| `blend`, `blend_func`, `brep_blend` | The walking functions and the walking itself. |
| `fillet_surf` | Fillet surfaces without topology. |

## Ownership and thread safety

The builders take their shape by handle and build a new one.

## Verification

Run `cargo test -p martensite-tkfillet`. `tests/dataset_cases` holds the blend, chamfer and related
script cases on the contract of [`martensite-boolean-cases`](../martensite-boolean-cases/README.md).
The cases that draw in the 3D viewer are `fillet_view_cases` in `martensite-vulkan`. A case that
fails stays listed with `#[ignore]` and its cause. No recorded baseline of this crate moved on
correctly rounded math; see [the attribution](../martensite-tkbool/corpus-attribution/README.md).
