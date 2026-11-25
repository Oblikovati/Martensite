# Sketch3D scenario map

This file assigns each of the 242 scenario definitions in the pinned corpus ([CORPUS.sha256](CORPUS.sha256)) to a class and a phase of the [implementation plan](IMPLEMENTATION-PLAN.md). Scenario Outlines count once and are marked `(outline)`.

The end-to-end scenarios belong to `oblikovati-rust`. Martensite uses this map to derive focused kernel fixtures and records the source scenario with each fixture. Source line numbers refer to the assessed corpus snapshot.

## Classes

| Class | Meaning | Count |
| --- | --- | ---: |
| K | The steps state a committed geometry, relation, measurement, refusal or undo result that a kernel test can assert. | 71 |
| I | The steps state that accepted geometry stays unchanged after a cancel or an uncommitted input. The kernel asserts snapshot isolation. | 20 |
| P | The steps show a preview and commit nothing. The kernel supplies preview geometry. | 2 |
| A | Dialog, tooltip, search, picking, camera or menu behavior. The application owns the assertion. | 149 |

Class K does not mean that the expected numbers exist. Many captured results are visual. A K scenario needs an independent expected result before it can close a phase gate.

Fixture-scoping notes:

- Derive Phase 1 point and straight-segment fixtures from `fix:10, 20, 29, 39, 77, 87` and `dimension:20, 40`. The app can run the full setups in Phase 2 once the arcs created in the shared `Background`/`Given` text are available.
- `dimension:9` searches for both Dimension and Driven Dimension, so it closes in Phase 2 once Driven Dimension lands.
- `point:70` stays `K` in Phase 1 because the kernel model owns typed point roles (`Defining`, `SketchMarker`, `CenterMarker`, matching app ADR-0017), while marker glyphs and downstream Hole consumption belong to the app.
- `show-constraints:131` closes in Phase 2 for implemented constraint types; blocked Phase B filter entries (`Smooth (G2)`, `Symmetric`, `Reference`) remain unpopulated until their captures land.
- Phase `B` scenarios (including `silhouette-curve:83`) stay out of Phase 6 closure until their captures are accepted and assigned to a phase by a reviewed plan update.

## Counts by phase

| Phase | K | I | P | A | Total |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 First slice | 25 | 3 | 0 | 56 | 84 |
| 1 and 2 (shared outlines) | 0 | 2 | 0 | 2 | 4 |
| 2 Curves and relations | 24 | 7 | 0 | 31 | 62 |
| 3 Sources and attachment | 9 | 1 | 0 | 14 | 24 |
| 3b Silhouette | 2 | 0 | 0 | 4 | 6 |
| 4 Edits | 10 | 4 | 0 | 9 | 23 |
| 5 Generators | 0 | 0 | 2 | 15 | 17 |
| B Blocked on capture | 1 | 3 | 0 | 18 | 22 |
| **Total** | **71** | **20** | **2** | **149** | **242** |

The evidence column maps the corpus Gherkin tags in priority order: `@needs_capture` -> `needs capture`, `@needs_fixture` -> `needs fixture`, `@observed_live` -> `observed`, otherwise `untagged`. Secondary tags (`@expected_contract`, `@historical_capture`, `@fixture_variation`, `@fixture_specific`) stay in the `.feature` files.

## Scenarios

| Folder:line | Scenario | Class | Phase | Evidence |
| --- | --- | --- | --- | --- |
| 3d-transform:14 | Open 3D Transform for an active sketch | A | B | observed |
| 3d-transform:24 | Choose a translation or rotation mode | A | B | observed |
| 3d-transform:37 | Choose a transform space | A | B | observed |
| 3d-transform:51 | Show transform controls after picking a sketch segment | A | B | observed |
| 3d-transform:59 | Expose a rotation angle input from a gizmo handle drag | A | B | observed |
| 3d-transform:70 | Expose two coordinate inputs from a square gizmo handle drag | A | B | observed |
| 3d-transform:82 | Show Repeat 3D Transform in a curve context menu | A | B | observed |
| 3d-transform:88 | Cancel 3D Transform before selecting geometry | I | B | observed |
| 3d-transform:101 | Cancel an empty 3D Transform with the red X control | A | B | observed |
| 3d-transform:109 | Enter the Locate reference picker and cancel | A | B | observed |
| 3d-transform:119 | Enter the Orient reference picker and cancel | A | B | observed |
| 3d-transform:129 | Locate the transform gizmo at a sketch endpoint | I | B | observed |
| 3d-transform:143 | Orient the transform triad to the browser Y Axis | I | B | observed |
| arc:11 | Choose the center-point construction method from the Arc flyout | A | 2 | observed |
| arc:24 | Place a three-point arc and verify continued placement | K | 2 | observed |
| arc:37 | Inspect three-point arc-specific context actions | A | 2 | observed |
| arc:45 | Cancel a center-point arc after choosing its start point | I | 2 | observed |
| arc:61 | Place a center-point arc and verify continued placement | K | 2 | observed |
| arc:77 | Place a center-point arc with Dynamic Dimension disabled | K | 2 | observed |
| arc:99 | Inspect center-point arc-specific context actions | A | 2 | observed |
| bend:9 | Read the Bend command help | A | 4 | observed |
| bend:15 | Open and close the Bend palette on an empty active sketch | A | 4 | observed |
| bend:27 | Bend a connected right-angle corner and undo or redo the result | K | 4 | observed |
| bend:46 | Apply Bend to multiple corners in one palette session | K | 4 | observed |
| coincident:8 | Find the active-context Coincident command | A | 1 | observed |
| coincident:20 | Make two free points coincident | K | 1 | observed |
| coincident:35 | Undo and redo a point coincidence | K | 1 | observed |
| coincident:45 | Read the Coincident ribbon tooltip | A | 1 | observed |
| coincident:55 | Cancel Coincident after selecting one curve | A | 1 | observed |
| colinear:8 | Find Colinear in command search | A | 2 | observed |
| colinear:21 | Apply Colinear to two offset lines from the prepared fixture | K | 2 | observed |
| colinear:42 | Undo a Colinear relation on the prepared fixture | K | 2 | observed |
| colinear:58 | Start Colinear and cancel before selecting geometry | A | 2 | observed |
| construction:11 | Find the 3D Sketch Construction format control | A | 2 | observed |
| construction:22 | Toggle a selected straight segment into and out of construction format | K | 2 | observed |
| construction:35 | Toggle a selected three-point arc into and out of construction format | K | 2 | observed |
| curve-on-face:10 | Create a spline on a solid face and finish it at a new endpoint | K | 3 | observed |
| curve-on-face:25 | Reject adjoining duplicate spline points | K | 3 | observed |
| curve-on-face:38 | Inspect available curve-specific context actions | A | 3 | observed |
| curve-on-face:48 | Identify a face curve in a viewport pick | A | 3 | observed |
| curve-on-face:55 | Read Curve On Face guidance from the ribbon tooltip | A | 3 | observed |
| dimension:9 | Find Dimension and Driven Dimension in command search | A | 2 | observed |
| dimension:20 | Create a length dimension on a straight segment | K | 1 | observed |
| dimension:40 | Change a length dimension and close its editor unchanged | K | 1 | observed |
| dimension:58 | Create an angular dimension between connected segments | K | 2 | observed |
| dimension:73 | Create a radius dimension on a three-point arc | K | 2 | observed |
| dimension:87 | Close a Driven Dimension editor and retain its annotation | K | 2 | observed |
| equal:12 | Find Equal in command search | A | 2 | observed |
| equal:23 | Select two line segments for an Equal relation | A | 2 | observed |
| equal:33 | Make two unequal straight segments equal | K | 2 | observed |
| equal:45 | Apply Equal to two connected straight segments | K | 2 | observed |
| equal:61 | End Equal after two Escapes in the earlier fresh-pair result capture | A | 2 | observed |
| equal:69 | Cancel Equal from the Constrain gallery before selecting geometry | A | 2 | observed |
| equal:79 | Cancel Equal after selecting its first curve | I | 2 | observed |
| equal:95 | Make a smaller three-point arc equal to a larger arc | K | 2 | observed |
| equation-curve:10 | Find Equation Curve in the 3D Sketch command search results | A | 5 | observed |
| equation-curve:16 | Show Equation Curve fields with the initial Cartesian values | A | 5 | observed |
| equation-curve:31 | Change the coordinate system equation field labels (outline) | A | 5 | observed |
| equation-curve:51 | Retain the last coordinate system after cancelling and reopening Equation Curve | A | 5 | observed |
| equation-curve:68 | Preview a curve from valid Cartesian equations | P | 5 | observed |
| equation-curve:85 | Show the floating toolbar options | A | 5 | observed |
| equation-curve:92 | Keep Equation Curve open after Escape while an equation field is focused | A | 5 | observed |
| extend:9 | Observe green feedback over a curve in Extend without a boundary | I | 4 | observed |
| extend:24 | Read Extend guidance from the ribbon tooltip | A | 4 | observed |
| extend:32 | Extend a horizontal segment to a vertical sketch boundary | K | 4 | observed |
| extend:51 | Undo an accepted extension and restore its fixture | K | 4 | observed |
| fix:10 | Start and cancel Fix before selecting geometry | A | 1 | observed |
| fix:20 | Fix a point in place | K | 1 | observed |
| fix:29 | Fix a straight sketch segment in place | K | 1 | observed |
| fix:39 | Attempt to drag a fixed straight segment | K | 1 | observed |
| fix:49 | Fix a free arc in place | K | 2 | observed |
| fix:59 | Attempt to drag a fixed arc | K | 2 | observed |
| fix:69 | Find Fix through command search | A | 1 | observed |
| fix:77 | Remove a Fix relation while retaining the segment | K | 1 | observed |
| fix:87 | Show the Fix glyph for a fixed segment | A | 1 | observed |
| helical-curve:10 | Show Helical Curve parameters before selecting an axis | A | 5 | observed |
| helical-curve:22 | Close Helical Curve parameters with Cancel before picking an axis | A | 5 | observed |
| helical-curve:30 | List the available helix definitions | A | 5 | observed |
| helical-curve:40 | Preview a helix after defining its axis and start point | P | 5 | observed |
| helical-curve:56 | Enable flat end-angle controls | A | 5 | observed |
| helical-curve:73 | Change the helix definition to Pitch and Height | A | 5 | observed |
| helical-curve:83 | Change the helix definition to Revolution and Height | A | 5 | observed |
| helical-curve:94 | Change the helix definition to Spiral | A | 5 | observed |
| helical-curve:106 | Switch helix parameters to a table and add a row | A | 5 | observed |
| helical-curve:119 | Escape from an active parameter cell before closing the dialog | A | 5 | observed |
| include-geometry:13 | Read the Include Geometry ribbon help | A | 3 | observed |
| include-geometry:20 | Include a solid model edge in the active 3D sketch | K | 3 | observed |
| include-geometry:30 | End Include Geometry after accepting a model edge | K | 3 | observed |
| include-geometry:41 | Inspect the included curve context actions | A | 3 | observed |
| include-geometry:47 | Inspect the included curve properties without changing them | A | 3 | untagged |
| insert-acad:12 | Open the drawing import file picker | A | B | observed |
| insert-acad:22 | Cancel drawing insertion | A | B | observed |
| insert-points:12 | Open the point-data file picker | A | B | observed |
| insert-points:23 | Inspect point-import options | A | B | observed |
| insert-points:36 | Cancel point-data import | A | B | observed |
| intersection-curve:13 | Open the intersection geometry dialog | A | 3 | observed |
| intersection-curve:25 | Select a face and an origin plane as intersection geometry | A | 3 | observed |
| intersection-curve:37 | Report a failed face and plane intersection | K | 3 | observed |
| intersection-curve:50 | Return to the definition dialog after a failed intersection | A | 3 | observed |
| intersection-curve:62 | Create an intersection curve from a solid face and an origin plane | K | 3 | observed |
| intersection-curve:77 | Undo a newly created intersection curve | K | 3 | observed |
| intersection-curve:85 | Cancel the geometry definition with Escape | A | 3 | observed |
| intersection-curve:95 | Cancel after selecting the first intersection geometry | A | 3 | observed |
| line:9 | Start Line placement with a visible 3D coordinate triad | A | 1 | observed |
| line:17 | End Line before choosing a first point | A | 1 | observed |
| line:29 | Cycle Line guide orientations with Shift+Space | A | 1 | observed |
| line:41 | Place a straight segment using dynamic length input | K | 1 | observed |
| line:56 | Accept a segment when Dynamic Dimension is hidden | K | 1 | observed |
| line:72 | Cancel Line after its first point | I | 1 | observed |
| line:84 | Enter a numeric length for a straight segment | K | 1 | observed |
| line:96 | Enter a second numeric value after locking Line length | K | 1 | observed |
| line:112 | Undo the most recently accepted Line segment | K | 1 | observed |
| line:120 | Toggle Ortho Mode guide lines while Line is active (outline) | A | 1 | observed |
| line:134 | Show the dashed triad axis when Ortho guides are hidden | A | 1 | observed |
| line:145 | Cancel a Line reposition preview | I | 1 | observed |
| line:157 | Repeat Line from a selected segment's context menu | A | 1 | observed |
| line:171 | Inspect the context list for a selected straight segment | A | 1 | observed |
| line:182 | Create multiple connected Line segments in one placement session | K | 1 | observed |
| line:192 | Undo and redo the most recent connected Line segment | K | 1 | observed |
| line:202 | Reject coincident Line endpoints | K | 1 | observed |
| line:216 | Place a Line endpoint at a visible sketch point | K | 1 | observed |
| line:227 | Choose a 2D sketch point from overlapping Line candidates | K | 1 | needs fixture |
| line:246 | Connect points on two station sketches with a 3D line | K | 1 | needs capture |
| mirror:12 | Mirror a selected sketch curve across the YZ origin plane | K | 4 | observed |
| mirror:34 | Apply Mirror using the XZ origin plane | K | 4 | observed |
| mirror:52 | Apply Mirror using the XY origin plane | K | 4 | observed |
| mirror:68 | Offer Mirror as the repeat action after using Mirror | A | 4 | observed |
| mirror:75 | Close the empty Mirror palette with Done | A | 4 | observed |
| mirror:88 | Cancel Mirror after choosing a geometry and origin plane | I | 4 | observed |
| mirror:108 | Close Mirror from the title bar after selecting a plane | I | 4 | observed |
| mirror:118 | Press Escape after selecting a Mirror source and plane | I | 4 | observed |
| mirror:128 | Select multiple curves as Mirror sources | A | 4 | observed |
| mirror:144 | Apply Mirror to multiple selected straight curves across the YZ plane | K | 4 | observed |
| on-face:9 | Find On Face without confusing it with Curve On Face | A | 3 | observed |
| on-face:22 | Constrain a free line endpoint to a solid face | K | 3 | observed |
| on-face:40 | Cancel On Face after selecting a source point | I | 3 | observed |
| parallel:8 | Make two nonparallel line segments parallel | K | 1 | observed |
| parallel:25 | Display a Parallel relation glyph | A | 1 | observed |
| parallel:41 | Find Parallel in command search | A | 1 | observed |
| parallel:53 | Read the Parallel ribbon tooltip | A | 1 | observed |
| parallel-x:8 | Find X Parallel among the axis-specific constraints | A | 2 | observed |
| parallel-x:20 | Read the Parallel with X Axis tooltip | A | 2 | observed |
| parallel-x:28 | Constrain one line segment parallel with the X axis | K | 2 | observed |
| parallel-xy-plane:8 | Apply XY Parallel to a free line | K | 2 | observed |
| parallel-xy-plane:25 | Find XY Parallel in command search | A | 2 | observed |
| parallel-xz-plane:8 | Apply XZ Parallel to a straight line | K | 2 | observed |
| parallel-xz-plane:27 | Find XZ Parallel in command search | A | 2 | observed |
| parallel-xz-plane:38 | Inspect the XZ Parallel relation cue | A | 2 | observed |
| parallel-y:8 | Read the Parallel with Y Axis tooltip | A | 2 | observed |
| parallel-y:16 | Find Y Parallel in command search | A | 2 | observed |
| parallel-y:28 | Constrain one line segment parallel with the Y axis | K | 2 | observed |
| parallel-yz-plane:8 | Apply YZ Parallel to a free line | K | 2 | observed |
| parallel-yz-plane:25 | Find YZ Parallel in command search | A | 2 | observed |
| parallel-z:8 | Read the Parallel with Z Axis tooltip | A | 2 | observed |
| parallel-z:16 | Find Z Parallel in command search | A | 2 | observed |
| parallel-z:28 | Constrain one line segment parallel with the Z axis | K | 2 | observed |
| parallel-z:44 | Inspect the Z-axis relation from a front view | A | 2 | observed |
| perpendicular:9 | Find and start Perpendicular in command search | A | 2 | observed |
| perpendicular:20 | Start Perpendicular and cancel before selecting geometry | A | 2 | observed |
| perpendicular:32 | Cancel Perpendicular after selecting its first line | I | 2 | observed |
| perpendicular:49 | Constrain two separate line segments as perpendicular | K | 2 | observed |
| perpendicular:63 | Display a Perpendicular relation glyph | A | 2 | observed |
| point:8 | Show coordinate fields during multiple point placements | K | 1 | observed |
| point:21 | Create a point by entering XYZ coordinates | K | 1 | observed |
| point:38 | Start Point and cancel before placing a point | A | 1 | observed |
| point:48 | Read the Center Point control tooltip | A | 1 | observed |
| point:58 | Toggle Center Point creation mode | A | 1 | observed |
| point:70 | Create distinct sketch point and center point markers | K | 1 | observed |
| point:87 | Show coordinate fields only when Dynamic Dimension is enabled | A | 1 | observed |
| point:109 | Continue placing points when Dynamic Dimension is disabled | K | 1 | observed |
| point:123 | Undo and redo a placed point | K | 1 | observed |
| point:135 | Observe the point pick cue after Point placement at a 3D sketch endpoint (outline) | A | 1 | observed |
| project-to-surface:11 | Project a selected 3D curve onto a selected face along an axis | K | 3 | observed |
| project-to-surface:41 | Inspect stored selections and alternate Output icon choices | A | 3 | observed |
| shared-tool-lifecycle:14 | Find a command for the active 3D sketch context | A | 1 | observed |
| shared-tool-lifecycle:25 | Choose a constraint from the 3D sketch gallery | A | 2 | observed |
| shared-tool-lifecycle:37 | Cycle the active 3D sketch placement guide | A | 1 | observed |
| shared-tool-lifecycle:54 | Enter the 3D sketch environment from a part | A | 1 | untagged |
| shared-tool-lifecycle:74 | Start a new 3D sketch in a populated part | A | 1 | untagged |
| shared-tool-lifecycle:99 | Start 3D Sketch again while a 3D sketch is active | A | 1 | untagged |
| shared-tool-lifecycle:126 | Enter an existing selected 3D sketch from the ribbon | A | 1 | observed |
| shared-tool-lifecycle:153 | Reopen a finished 3D sketch from its browser context menu | A | 1 | observed |
| shared-tool-lifecycle:177 | Pick and clear a curve while editing a 3D sketch | A | 1 | observed |
| shared-tool-lifecycle:187 | Exit an awaiting 3D sketch command without changing accepted geometry | I | 2 | needs capture |
| shared-tool-lifecycle:199 | End repeated placement while keeping accepted geometry | I | 1 | untagged |
| shared-tool-lifecycle:226 | Escape ends repeated placement without removing accepted geometry (outline) | I | 1, 2 | untagged |
| shared-tool-lifecycle:259 | Escape discards an unaccepted placement (outline) | I | 1, 2 | untagged |
| shared-tool-lifecycle:287 | Undo Finish 3D Sketch to resume sketch editing | A | 1 | untagged |
| shared-tool-lifecycle:312 | Pick and clear a 3D sketch curve (outline) | A | 1, 2 | untagged |
| shared-tool-lifecycle:363 | Pick and clear a displayed 3D sketch curve in the part workspace | A | 1 | observed |
| shared-tool-lifecycle:386 | Disambiguate a 3D sketch curve that overlaps a solid edge | A | 1 | needs fixture |
| shared-tool-lifecycle:403 | Show overlapping curve and edge candidates while editing a 3D sketch | A | 1 | needs fixture |
| shared-tool-lifecycle:420 | Identify a point candidate on a 3D sketch curve | A | 1 | needs fixture |
| shared-tool-lifecycle:435 | Identify a point from another sketch (outline) | A | 1 | needs capture |
| shared-tool-lifecycle:447 | Surface a point candidate from another visible sketch | A | 1 | needs fixture |
| shared-tool-lifecycle:464 | Choose a curve from a cross-sketch endpoint pick list | A | 1 | needs fixture |
| shared-tool-lifecycle:493 | Pick a visible sketch curve over a solid while editing | A | 1 | needs fixture |
| shared-tool-lifecycle:515 | Select and clear a solid face in the part workspace | A | 1 | observed |
| shared-tool-lifecycle:543 | Finish a 3D sketch before selecting a solid face | A | 1 | untagged |
| shared-tool-lifecycle:576 | Zoom the part viewport with the mouse wheel | A | 1 | observed |
| shared-tool-lifecycle:602 | Zoom the viewport while editing a 3D sketch | A | 1 | observed |
| shared-tool-lifecycle:633 | Change to top view and restore the previous viewport orientation | A | 1 | untagged |
| shared-tool-lifecycle:658 | Select a sketch point that overlaps a solid face | A | 1 | needs fixture |
| shared-tool-lifecycle:681 | Surface a sketch curve candidate over a solid face | A | 1 | needs fixture |
| shared-tool-lifecycle:703 | Identify a solid edge candidate | A | 1 | untagged |
| shared-tool-lifecycle:721 | Identify a solid vertex candidate | A | 1 | untagged |
| shared-tool-lifecycle:738 | Select a solid face and disambiguate a curve from its edge | A | 1 | needs fixture |
| shared-tool-lifecycle:787 | Choose the solid edge at a crowded sketch-point corner | A | 1 | needs fixture |
| shared-tool-lifecycle:817 | Open the solid face marking and context menus | A | 1 | observed |
| shared-tool-lifecycle:844 | Choose among solid face and body candidates | A | 1 | observed |
| shared-tool-lifecycle:873 | Select the solid body from overlapping candidates | A | 1 | observed |
| shared-tool-lifecycle:894 | Enter an existing 3D sketch by double-clicking its browser entry | A | 1 | observed |
| show-constraints:7 | Find Constraint Visibility in command search | A | 1 | observed |
| show-constraints:18 | Invoke Show Constraints with no geometry selected | A | 1 | observed |
| show-constraints:27 | Read the Show Constraints ribbon tooltip | A | 1 | observed |
| show-constraints:35 | Show a constraint on selected sketch geometry (outline) | A | 1, 2 | observed |
| show-constraints:62 | Show a relation glyph for a line endpoint constrained to a face | A | 3 | observed |
| show-constraints:76 | Filter displayed constraints and restore all available types | A | 2 | observed |
| show-constraints:110 | Apply only the Equal visibility filter to the connected line pair | A | 2 | observed |
| show-constraints:131 | Filter constraint glyphs by an individual available type (outline) | A | 2 | observed |
| silhouette-curve:11 | Read the Silhouette Curve help tooltip | A | 3b | observed |
| silhouette-curve:18 | Create a silhouette curve from a selected body and direction axis | K | 3b | observed |
| silhouette-curve:37 | Open with a single unselected body in the part | A | 3b | observed |
| silhouette-curve:46 | Use a linear origin axis as the pull direction | A | 3b | observed |
| silhouette-curve:55 | Inspect silhouette exclusions then cancel the pending changes | A | B | observed |
| silhouette-curve:75 | Keep the silhouette definition dialog open after Escape | A | 3b | observed |
| silhouette-curve:83 | Recover from an update error caused by the current exclusion definition | K | B | observed |
| silhouette-curve:101 | Apply a silhouette and keep the definition dialog open | K | 3b | observed |
| smooth-g2:9 | Find Smooth (G2) in the 3D Sketch command search results | A | B | observed |
| spline:9 | Cancel a spline preview before a curve is visible | I | 2 | observed |
| spline:27 | Create and finish a spline with a right click | K | 2 | observed |
| spline:42 | Cancel a one-point Spline placement with Dynamic Dimension disabled | I | 2 | observed |
| spline:57 | Inspect spline-specific context menu actions | A | 2 | observed |
| split:9 | Read Split guidance from the ribbon tooltip | A | B | observed |
| tangent:11 | Find Tangent among the active-context commands | A | 2 | observed |
| tangent:23 | Apply Tangent between a spline and its connected line | K | 2 | observed |
| tangent:38 | Cancel Tangent after choosing the spline | I | 2 | observed |
| trim:10 | Start Trim and cancel without selecting geometry | A | 4 | observed |
| trim:18 | Read the Trim command tooltip shortcut | A | 4 | observed |
| trim:23 | Start Trim with its X shortcut and cancel | A | 4 | observed |
| trim:33 | Remove a branch at a crossing and end Trim | K | 4 | observed |
| trim:45 | Undo a trimmed branch | K | 4 | observed |
