# Global property reference fixture

`cylinder-minus-box.brep` is the retained result of cutting a cylinder (radius 10,
height 20) by a box (10, 10, 10), written as a BRep text file.

Its reference volume properties:

- done: true; valid: true
- volume: `5497.7871437821368`
- centre of mass: `(-0.60630454511198362, -0.60630454511198295, 10.714285714285715)`
- inertia matrix, row major:
  `315878.24146288086, 14521.015150373298, -19047.619047619028,`
  `14521.015150373298, 315878.24146287912, -19047.619047619057,`
  `-19047.619047619028, -19047.619047619057, 270847.32688836136`
- symmetry axis: none
- topology counts: 44 vertices, 22 edges, 6 faces, 1 shell, 1 solid

The test reads this file with `BRepTools::Read` and checks its topology and volume
properties. Reading a retained result does not exercise the cut that built it.
