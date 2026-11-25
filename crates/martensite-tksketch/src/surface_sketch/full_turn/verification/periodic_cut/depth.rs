// SPDX-License-Identifier: GPL-2.0-only
//! Qualify both clipped lifts of the aperture on each actual periodic face.
use super::*;
use crate::surface_sketch::FullTurnSupport;
use martensite_tkbrep::brep::EdgePCurve;

pub(super) fn check(proof: &Verification<'_>, depth: PeriodicDepth) -> Checked<()> {
    let face = proof.retained(
        HistorySource::Region(proof.band.region),
        primitive::face_role(depth.index()),
    )?;
    walls::one_wire(face)?;
    let face = topods::face(face).map_err(|e| native("periodic depth face", e))?;
    let surface = BRepAdaptorSurface::with_face(&face, true)
        .map_err(|e| native("periodic depth carrier", e))?;
    let stock = &proof.prepared.stock;
    let index = if depth.index() == stock.sweep.first_depth {
        3
    } else {
        1
    };
    require_surface(&surface, SurfaceType::Cylinder)?;
    verify_cylinder(stock, &surface, index)?;
    let uses = directed_edges(&face)?;
    let row = DepthFace {
        proof,
        depth,
        face,
        surface,
        uses,
    };
    row.check()
}

struct DepthFace<'a, 'b> {
    proof: &'a Verification<'b>,
    depth: PeriodicDepth,
    face: topods::Face,
    surface: BRepAdaptorSurface,
    uses: Vec<topods::Edge>,
}

impl DepthFace<'_, '_> {
    fn check(&self) -> Checked<()> {
        let lower_direction = self.rims()?;
        self.seams()?;
        let [a, b] = *self.proof.crossings;
        self.aperture_pieces(a.id, 0, !lower_direction)?;
        self.aperture_pieces(b.id, 1, lower_direction)?;
        let right = self.vertical(1)?;
        let left = self.vertical(3)?;
        if right == left {
            return Err(incomplete(
                "oppositely directed vertical aperture coedges",
                format!("{right:?} and {left:?}"),
            ));
        }
        Ok(())
    }

    fn rims(&self) -> Checked<bool> {
        let [lower, upper] = self.proof.band.rims.map(|id| {
            self.proof.retained(
                HistorySource::Curve(id),
                primitive::edge_role(self.depth.index()),
            )
        });
        let [lower] = self.coedges(lower?)?;
        let [upper] = self.coedges(upper?)?;
        for edge in [&lower, &upper] {
            trim::verify_rim_trim(self.stock(), &self.face, &self.surface, edge, true)?;
        }
        let a = trim::increasing(&lower, &self.pcurve(&lower)?, 0)?;
        let b = trim::increasing(&upper, &self.pcurve(&upper)?, 0)?;
        if a == b {
            return Err(incomplete(
                "opposite stock rim directions",
                format!("{a:?} and {b:?}"),
            ));
        }
        Ok(a)
    }

    fn seams(&self) -> Checked<()> {
        let ends = seam_endpoints(self.proof.band.rims, self.proof.crossings);
        let axial = self.proof.prepared.aperture.rectangle.z;
        let stock_axial = self.stock_axial()?;
        let points = [[stock_axial[0], axial[0]], [axial[1], stock_axial[1]]];
        for (ends, meridian) in ends.into_iter().zip(points) {
            let key = PeriodicImageKey::SeamSegment {
                region: self.proof.band.region,
                depth: self.depth,
                ends,
            };
            let shape = self.proof.resolve(key)?;
            let [a, b] = self.coedges(shape)?;
            let expected = meridian.map(|z| self.proof.point(self.depth, z, 0.));
            trim::verify_seam_fragment(
                self.stock(),
                &self.face,
                &self.surface,
                [&a, &b],
                expected,
            )?;
        }
        Ok(())
    }

    fn stock_axial(&self) -> Checked<[f64; 2]> {
        let FullTurnSupport::Cylinder { axial, .. } = self.proof.band.support else {
            return Err(incomplete("cylindrical periodic stock", "unexpected cone"));
        };
        Ok(axial)
    }

    fn aperture_pieces(&self, id: PeriodicCrossingId, rim: usize, direction: bool) -> Checked<()> {
        let axial = self.proof.hole.axial[rim];
        let crossing = PeriodicImageKey::CrossingVertex {
            depth: self.depth,
            crossing: id,
        };
        let root = topods::vertex(self.proof.resolve(crossing)?)
            .map_err(|e| native("named crossing", e))?;
        let point = BRepTool::pnt(&root).map_err(|e| native("crossing point", e))?;
        close(
            distance(point, self.proof.point(self.depth, axial, 0.)),
            0.,
            tolerance(self.stock()),
            "crossing meridian point",
        )?;
        let pieces = self.proof.piece_keys(id, self.depth);
        let increasing = id.transition == SheetTransition::Increasing;
        for (key, negative) in pieces.into_iter().zip([increasing, !increasing]) {
            self.aperture_piece(key, axial, negative, direction)?;
        }
        Ok(())
    }

    fn aperture_piece(
        &self,
        key: PeriodicImageKey,
        axial: f64,
        negative: bool,
        direction: bool,
    ) -> Checked<()> {
        let [edge] = self.coedges(self.proof.resolve(key)?)?;
        self.aperture_circle(&edge, axial, negative)?;
        let trim = self.pcurve(&edge)?;
        if trim::increasing(&edge, &trim, 0)? != direction {
            return Err(incomplete(
                "aperture arc opposes stock rim direction",
                format!("{key:?}"),
            ));
        }
        self.check_lift(&trim, Some(negative))?;
        trim::verify_representation(self.stock(), &self.surface, &edge, &trim)
    }

    fn aperture_circle(&self, edge: &topods::Edge, axial: f64, negative: bool) -> Checked<()> {
        let curve =
            BRepAdaptorCurve::with_edge(edge).map_err(|e| native("split aperture arc", e))?;
        check_curve_kind(&curve, true)?;
        let circle = curve
            .circle()
            .map_err(|e| native("split aperture circle", e))?;
        axis_direction(self.stock(), circle.axis())?;
        let aperture = &self.proof.prepared.aperture;
        let radius = aperture.radii[self.depth.index()];
        let azimuth =
            self.proof.hole.arc_length[usize::from(!negative)] / aperture.rectangle.support.radius;
        close(
            circle.radius(),
            radius,
            tolerance(self.stock()),
            "split arc depth radius",
        )?;
        close(
            distance(circle.location(), axis_point(self.stock(), axial)),
            0.,
            tolerance(self.stock()),
            "split arc centre",
        )?;
        close(
            curve_span(&curve)?,
            azimuth.abs(),
            angular(self.stock()),
            "original split arc interval",
        )
    }

    fn vertical(&self, curve: usize) -> Checked<bool> {
        let shape = self.proof.retained(
            HistorySource::Curve(self.proof.hole.boundaries[curve]),
            primitive::edge_role(self.depth.index()),
        )?;
        let [edge] = self.coedges(shape)?;
        let trim = self.pcurve(&edge)?;
        let direction = trim::increasing(&edge, &trim, 1)?;
        self.check_lift(&trim, None)?;
        trim::verify_representation(self.stock(), &self.surface, &edge, &trim)?;
        Ok(direction)
    }

    fn check_lift(&self, trim: &EdgePCurve, cut_side: Option<bool>) -> Checked<()> {
        let a = self
            .surface
            .first_u_parameter()
            .map_err(|e| native("periodic lower lift", e))?;
        let b = self
            .surface
            .last_u_parameter()
            .map_err(|e| native("periodic upper lift", e))?;
        let first = trim::trim_uv(trim, trim.first)?;
        let last = trim::trim_uv(trim, trim.last)?;
        let [min, max] = [first.x().min(last.x()), first.x().max(last.x())];
        let tolerance = angular(self.stock());
        let clear = a + tolerance < min && max < b - tolerance;
        let attached = match cut_side {
            Some(true) => a + tolerance < min && (max - b).abs() <= tolerance,
            Some(false) => (min - a).abs() <= tolerance && max < b - tolerance,
            None => clear,
        };
        if !attached {
            return Err(incomplete(
                "aperture coedge on its certified sheet side",
                format!("{min}..{max} in {a}..{b}, {cut_side:?}"),
            ));
        }
        Ok(())
    }

    fn coedges<const N: usize>(&self, shape: &topods::Shape) -> Checked<[topods::Edge; N]> {
        let occurrences = self
            .uses
            .iter()
            .filter(|edge| shape.is_same(edge))
            .cloned()
            .collect::<Vec<_>>();
        occurrences.try_into().map_err(|found: Vec<_>| {
            incomplete(
                "named directed periodic coedges",
                format!("{} uses, expected {N}", found.len()),
            )
        })
    }

    fn pcurve(&self, edge: &topods::Edge) -> Checked<EdgePCurve> {
        let trim = BRepTool::curve_on_face(edge, &self.face)
            .map_err(|e| native("split aperture pcurve", e))?;
        if !trim.is_stored {
            return Err(incomplete(
                "stored split aperture pcurve",
                "implicit pcurve",
            ));
        }
        Ok(trim)
    }

    fn stock(&self) -> &BandSection {
        &self.proof.prepared.stock
    }
}

fn directed_edges(face: &topods::Face) -> Checked<Vec<topods::Edge>> {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)
        .map_err(|e| native("seam-cut face coedges", e))?;
    let mut found = Vec::new();
    while explorer.more() {
        found
            .push(topods::edge(explorer.current()).map_err(|e| native("seam-cut face coedge", e))?);
        explorer
            .next()
            .map_err(|e| native("next seam-cut coedge", e))?;
    }
    if found.len() != 12 {
        return Err(incomplete(
            "complete seam-cut aperture boundary",
            format!("{} directed uses, expected 12", found.len()),
        ));
    }
    Ok(found)
}
