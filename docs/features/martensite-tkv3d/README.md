# Interactive visualization

`martensite-tkv3d` turns shapes into interactive, selectable presentations and shows them
in views. It talks to a renderer only through the graphic driver interfaces of
`martensite-tkservice`, so it runs on the null driver in tests and on `martensite-vulkan`
in applications.

| Module | Capabilities |
| --- | --- |
| `ais` | Interactive objects (shapes, datums, labels, manipulators, the view cube, the colour scale, animations) and the interactive context. |
| `prs3d`, `std_prs`, `dsg_prs`, `prs_dim` | Presentation aspects and drawers, curve, face and hidden-line presentation algorithms, and dimensions and relations. |
| `prs_mgr` | Presentable objects, their presentations and the presentation manager. |
| `select3d`, `select_basics`, `select_mgr`, `std_select` | Sensitive entities, the selecting volume, owners, filters, selection modes and the viewer selector. |
| `v3d` | The viewer, its views, lights, grids, trihedron and image dumps. |
| `brep_font` | Glyph outline regularization and glyph and text topology builders. |

## Ownership and thread safety

The viewer, views, interactive context and presentations are thread-confined: they hold
shared handles to scene data and call back into application overrides, so a view belongs to
the thread that created it. Modelling results are read, never changed. Hidden-line
presentations use `martensite-tkhlr` and meshed presentations use `martensite-tkmesh`.

## Unsafe code

The crate denies `unsafe` code except in two places, recorded in `AGENTS.md`:

- `V3dLightName::from_ptr` is a public borrowed view of externally maintained C strings.
- `ais::textured_shape_retirement` erases the lifetime of one destructor borrow, used only
  inside the scope of that destructor.

## Verification

Run `cargo test -p martensite-tkv3d`. The tests use the null driver, so they need no GPU.
Frozen requests and answers under `tests/fixtures` are replayed bit for bit. Their numeric
values are recorded baselines; a change to one needs a reviewed
baseline update that states its reason and behaviour version.
