// SPDX-License-Identifier: GPL-2.0-only
//! Qualify both clipped lifts of the aperture on each actual periodic face.
use super::*;
use martensite_tkbrep::brep::EdgePCurve;

pub(super) fn check(proof: &Verification<'_>, depth: PeriodicDepth) -> Checked<()> {
    let face = proof.retained(
        HistorySource::Region(proof.band.region),
        primitive::face_role(depth.index()),
    )?;
    let face = topods::face(face).map_err(|e| native("periodic depth face", e))?;
    let surface = BRepAdaptorSurface::with_face(&face, true)
        .map_err(|e| native("periodic depth carrier", e))?;
    let stock = &proof.prepared.stock;
    let index = if depth.index() == stock.sweep.first_depth {
        3
    } else {
        1
    };
    let carrier = carrier_type(stock.cylinder, index);
    require_surface(&surface, carrier)?;
    verify_carrier(stock, &surface, carrier, index)?;
    let topology = ExpectedTopology::new(proof.prepared);
    let uses = DirectedWires::read(
        &face,
        [
            1 + topology.clear,
            4 + 8 * topology.crossed + 4 * topology.clear,
        ],
    )?;
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
    uses: DirectedWires,
}

impl DepthFace<'_, '_> {
    fn check(&self) -> Checked<()> {
        let lower_direction = self.rims()?;
        self.seams()?;
        for aperture in &self.proof.prepared.holes {
            self.check_aperture(aperture, lower_direction)?;
        }
        Ok(())
    }

    fn check_aperture(&self, hole: &ApprovedHole, lower_direction: bool) -> Checked<()> {
        let Some(crossings) = &hole.crossings else {
            return self.check_clear_wire(hole, lower_direction);
        };
        for (rim, direction) in [!lower_direction, lower_direction].into_iter().enumerate() {
            let id = crossings.roots[rim].id;
            self.aperture_pieces(hole, id, rim, direction, crossings.angular_spans)?;
        }
        let right = self.vertical(hole, 1)?;
        let left = self.vertical(hole, 3)?;
        if right == left {
            return Err(incomplete(
                "oppositely directed vertical aperture coedges",
                format!("{right:?} and {left:?}"),
            ));
        }
        Ok(())
    }

    fn check_clear_wire(&self, hole: &ApprovedHole, lower_direction: bool) -> Checked<()> {
        let [a, b, c, d] = hole.source.boundaries.map(|id| {
            self.proof.retained(
                HistorySource::Curve(id),
                primitive::edge_role(self.depth.index()),
            )
        });
        let edges = self.uses.clear_wire([a?, b?, c?, d?])?;
        trim::verify_aperture_loop(
            self.stock(),
            &self.face,
            &self.surface,
            lower_direction,
            &edges,
        )
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
        for rail in &self.proof.prepared.rails {
            let key = PeriodicImageKey::SeamSegment {
                region: self.proof.band.region,
                depth: self.depth,
                ends: rail.ends,
            };
            let shape = self.proof.resolve(key)?;
            let [a, b] = self.coedges(shape)?;
            let endpoints = rail.meridian.map(|m| self.proof.seam_point(self.depth, m));
            trim::verify_seam_fragment(
                self.stock(),
                &self.face,
                &self.surface,
                [&a, &b],
                endpoints,
            )?;
        }
        Ok(())
    }

    fn aperture_pieces(
        &self,
        hole: &ApprovedHole,
        id: PeriodicCrossingId,
        rim: usize,
        direction: bool,
        spans: [[f64; 2]; 2],
    ) -> Checked<()> {
        let meridian = hole.source.meridian[rim];
        let crossing = PeriodicImageKey::CrossingVertex {
            depth: self.depth,
            crossing: id,
        };
        let root = topods::vertex(self.proof.resolve(crossing)?)
            .map_err(|e| native("named crossing", e))?;
        let point = BRepTool::pnt(&root).map_err(|e| native("crossing point", e))?;
        close(
            distance(point, self.proof.seam_point(self.depth, meridian)),
            0.,
            tolerance(self.stock()),
            "crossing meridian point",
        )?;
        let pieces = self.proof.piece_keys(id, self.depth);
        let increasing = id.transition == SheetTransition::Increasing;
        for (key, negative) in pieces.into_iter().zip([increasing, !increasing]) {
            self.aperture_piece(
                hole,
                key,
                rim,
                negative,
                direction,
                spans[usize::from(!negative)],
            )?;
        }
        Ok(())
    }

    fn aperture_piece(
        &self,
        hole: &ApprovedHole,
        key: PeriodicImageKey,
        rim: usize,
        negative: bool,
        direction: bool,
        span: [f64; 2],
    ) -> Checked<()> {
        let [edge] = self.coedges(self.proof.resolve(key)?)?;
        self.aperture_circle(hole, &edge, rim, span)?;
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

    fn aperture_circle(
        &self,
        hole: &ApprovedHole,
        edge: &topods::Edge,
        rim: usize,
        span: [f64; 2],
    ) -> Checked<()> {
        let curve =
            BRepAdaptorCurve::with_edge(edge).map_err(|e| native("split aperture arc", e))?;
        check_curve_kind(&curve, true)?;
        let circle = curve
            .circle()
            .map_err(|e| native("split aperture circle", e))?;
        axis_direction(self.stock(), circle.axis())?;
        let [radius, axial] = hole.admitted.meridian(self.depth, rim);
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
        let actual = curve_span(&curve)?;
        let bound = angular(self.stock());
        if actual < span[0] - bound || actual > span[1] + bound {
            return Err(incomplete(
                "original split arc interval",
                format!("{actual} outside {span:?}"),
            ));
        }
        Ok(())
    }

    fn vertical(&self, hole: &ApprovedHole, curve: usize) -> Checked<bool> {
        let shape = self.proof.retained(
            HistorySource::Curve(hole.source.boundaries[curve]),
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
            .edges()
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
