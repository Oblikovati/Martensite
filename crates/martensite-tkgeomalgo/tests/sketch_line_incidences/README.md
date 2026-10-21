# Certified sketch incidence fixtures

These fixtures pin the certified contacts of sketch curves against frozen reference
intersections. Each `*-cases.csv` file lists the cases; the matching `*-contacts.csv`
file holds the contacts that an independent intersector computed for them at
tolerance 1e-9 (`id;count;x,y|...;segments`, binary64 bit patterns). No external
program runs during the tests.

## Line cases

`line-cases.csv` has 38 ordinary-range cases of an infinite source line against
circles (crossing, tangent and missing), a counterclockwise quarter arc, an ellipse
with a skew axis, crossing, endpoint and parallel segments, and every retained fit
and control spline recipe of `../sketch_spline` crossed by three lines. Arcs are
trimmed at the `atan2` angles of their sector rays, as an application builds them.
`line-contacts.csv` holds their reference contacts.

The tests certify each case on the supporting line with a 1e-12 world error limit.
They require the same contact count and, for every reference point, a distinct
certified contact within its achieved error plus 1e-8. The slack reflects that the
reference points are rounded solver output, not certified roots. The same check runs
against the mapped Rust `InterCurveCurve`, and against 40 cases built from the owned
sketch constructor: three fit spacings and control intent, open and closed, plus open
and closed curves with an active derivative handle (triple knots). No count mismatch
or overlap occurs in this ordinary corpus.

```sh
cargo test -p martensite-tkgeomalgo --test toolkit sketch_line_incidences::reference_lines
```

## Conic and spline sources

`curve-cases.csv` has 76 ordinary-range cases with a bounded curve source: a full
circle, a counterclockwise and a clockwise ray arc and a rotated ellipse, each against
three lines, two circles, a ray arc, an ellipse and two spline recipes; and every fit
and control recipe as a source against a segment, a circle, an arc, an ellipse and
another recipe shifted by `(0.3125, 0.1875)`. The shift keeps spline pairs off their
shared fit stations, whose exact coincidence depends on fit residuals of order 1e-16.

`curve-contacts.csv` appends, after the segment count, the end points of each
coincidence segment. The reference intersector reports five circle/circle crossings
in four cases as tolerance-length coincidence segments instead of points; the test
reads each such segment as one location at its middle, with half its length as extra
slack. Certified contacts are compared on the segment domain, so arcs and splines
stay bounded; counts must agree, and every reference location must lie within the
certified error, the segment radius and 1e-8 of a distinct contact. The mapped Rust
intersector is compared the same way.

```sh
cargo test -p martensite-tkgeomalgo --test toolkit sketch_line_incidences::reference_curves
```

## Periodic splines and offsets

`offset-cases.csv` has 108 ordinary-range cases. A segment, a circle and a (3, 4)
ellipse source meet offsets of two segments, a circle, a counterclockwise and a
clockwise arc, an axis-aligned and a (3, 4) ellipse and fit recipes (the segment every
recipe at distances 0.125 and -0.1; the conics the three open recipes at 0.125, since
a conic against a closed recipe's offset gives a degree-20 norm with 2,700-bit
coefficients that takes about 15 s per span to isolate in a debug build). Offset
sources (segment, circle, both arcs, both ellipses, three fit recipes) meet lines, a
circle, an ellipse, an arc and a segment offset; a periodic cubic meets lines and
conics as a source and a target. An offset is the offset curve of the untrimmed basis
trimmed like the basis, with clockwise arcs on a clockwise circle so that the offset
side follows the travel direction; a periodic spec is a periodic B-spline curve. The
offset of an unbounded line is left out, because its domain is infinite. A source
coincident with its target is left out too: the kernel names it `UncertifiedOverlap`.

The test certifies each case on the segment domain with a 1e-12 world error limit:
counts agree, and every reference point lies within the certified error plus 1e-8 of
a distinct contact. No coincidence zone occurs. The same check runs against the
mapped Rust `InterCurveCurve` on the same mapped curves.

```sh
cargo test -p martensite-tkgeomalgo --test toolkit sketch_line_incidences::reference_offsets
```
