# Cone chart atlas for disconnected regions

`DirectedRegionAtlasRequireCertifiedV4` selects a separate rational fit
chart for each disconnected region. Each region uses its complete authored
boundary, including its holes. The fit center can change from one region
to the next. The developed coordinates, physical angular datum, and
normal-depth convention are shared across the request.

Use `normal_extrude_cone_profile_with_selection` and a directed planar
proof selector such as `GeometricCompositeV6`. The atlas uses the same
source capture, rational side construction, analytic caps, and
frozen-body publication sequence as the common chart.
Use `region_chart_domain_receipt` with an authored region ID to inspect
the executed chart. The domain policy entry also selects the directed
path when given `PositiveXRegionAtlasV5`.

## Periodic separation

The reference uses the shared displacement bound for every member. It
expands each authored physical angular interval by the angular effect of
that displacement at the member's certified distance from the apex. It shifts each interval by an integer multiple of the bounded
physical period and sorts the shifted intervals. It checks every adjacent
gap and the gap from the last interval back to the first through a full
period. The family can span more than one revolution in the authored lift
when these checks prove the intervals disjoint on the physical cone.

For the stored-solid proof, each analytic cap contributes a Cartesian
enclosure of its full UV rectangle. Each ruled side contributes the hull
of its actual two positive-weight control rows. A Cartesian box enclosing
the complete boundary encloses the resulting solid. The certificate
requires strictly disjoint member boxes and retains the existing
within-member side, cap, and incidence checks. An overlapping box can
leave separation unresolved even when the solids are disjoint.

The atlas fits one positive-X chart per connected region. A connected
region that needs several fit charts requires the connected atlas path.
The [placed cone entry](CONE-PLACEMENT.md) can place the completed atlas
in an oblique frame. Cone-face attachment remains separate work.

## Local verification

Two holed rectangles on a 0.125-radian cone published in both normal
directions. Their fit centers were more than one revolution apart in the
authored lift. The two results retained the authored depth roles, and
the exported vertices passed independent world-position checks. The
outward result reproduced the encoded body, history, and budget in a
separate process. A developed copy rotated by one developed period
returned a typed stored-family separation refusal naming both regions.

A curved polynomial O and a dot also published on separate region
charts. The O retained its counter. Their authored area receipts
enclosed the independent values 18.3 and 4.0, respectively.

244 Math and 133 Sketch cone tests passed before the glyph control
was added. The four focused atlas controls then passed together.
Legacy and recentered replay signatures matched checkpoint `33bd219d`
byte for byte.

On the existing outward directed O. fixture, three alternating
pairs measured a median API time of 5.358 s at `33bd219d` and 5.483 s
with the atlas implementation. Median peak RSS was 66,876 KiB and
66,812 KiB. Each run charged 989,355 arithmetic operations and used 444
integration cells and 148 retained spans. The samples leave the
five-second and non-degradation gates open.

## Shared endpoints between local charts

An authored incidence carries one developed direction and slant. Its
half-angle tangent depends on the chart center and the selected turns.
The endpoint table checks the raw authored point bits once per use and
records the most recent projection for that incidence. It recomputes
the projection from the retained direction and slant when any chart
input changes. The fit therefore shares the incidence across charts
without reusing a tangent from a different chart.

The boundary control uses two adjacent line spans on a 0.125-radian
cone. Their combined physical lift spans 6.68 radians. The common
chart refuses that input, and both local charts issue bounded
boundary certificates. The two fits have distinct tangent endpoints
and bit-identical native endpoint coordinates at both depths. The
unwrapped cap UV endpoints also agree bit for bit after the existing
endpoint canonicalization. Chart domain admission accumulates its two
control-hull ranges in a single pass and owns only their extrema.

An earlier 8.64-radian control exposed a fitter limit. Each original
span fits within a half-angle chart, but the undivided Hermite fit
produces a nonpositive Bernstein weight. Local chart planning must
account for the stored weight bound as well as the angular interval.
A chart assigned to a dyadic subspan needs its receipt bound to the
exact restricted original controls. The current domain receipt names
whole original spans.

For a connected region, the existing joins and filled-domain proofs
bind one fit center per member and require a physical interval narrower
than one turn. The local endpoint control establishes the stored join
needed by a connected atlas. The remaining proof must bind the local
fits back to one closed unwrapped cap domain and exclude overlap of
periodic material images. A common UV domain gives the cap a global
parameterization while the rational fits keep their local charts.

The new regression refused on `c80cb77c` and passed with chart-local
endpoint projections. The legacy, recentered, disconnected-atlas and
oblique-placed replay workers produced identical archives and reports
before and after the change. Three alternate pairs on the same pinned
CPU measured the standalone O. median at 4.721 s before and 4.709 s
after. Median peak RSS was 67,812 and 67,380 KiB, respectively.
The 239-curve standalone text control passed with 3,057,275 arithmetic
operations and 1,017 integration cells. The temporary baseline build
and replay files were removed.
