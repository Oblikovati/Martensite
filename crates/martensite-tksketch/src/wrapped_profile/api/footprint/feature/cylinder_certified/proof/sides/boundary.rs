//! Prove each literal wall trim and register adjacency by decoded shared edge identity.
use super::*;
use martensite_tkbrep::{
    brep::access::vertex_tshape,
    topexp::{first_vertex, last_vertex},
};
use martensite_tkg3d::geom::GeomBezierCurve;

pub(super) struct Boundary<'a> {
    index: &'a body::Index,
    history: &'a CylinderFeatureHistory,
    connectors: BTreeMap<CylinderBodyRef, Vec<WrappedHistoryKey>>,
}

impl<'a> Boundary<'a> {
    pub(super) fn new(index: &'a body::Index, candidate: &'a Candidate) -> Self {
        Self {
            index,
            history: &candidate.history,
            connectors: BTreeMap::new(),
        }
    }

    pub(super) fn add(
        &mut self,
        attempt: Attempt,
        source: WrappedHistoryKey,
        face: &Face,
        mut rails: [&Shape; 2],
        sense: CylindricalRadialSense,
    ) -> Checked<()> {
        if sense == CylindricalRadialSense::Inward {
            rails.swap(0, 1);
        }
        let wires = assembly::children(attempt, face)?;
        let [wire] = wires.as_slice() else {
            return Err(refusal::cardinality(
                attempt,
                VALIDATION,
                "one exact ruled-wall trim wire",
                [wires.len(), 1],
                [history_entity(source)],
            ));
        };
        let edges = assembly::children(attempt, wire)?;
        require(
            attempt,
            VALIDATION,
            edges.len() == 4,
            "the four complete unit-square wall edges",
        )?;
        let mut slots = BTreeSet::new();
        for shape in edges {
            self.edge(attempt, face, source, rails, &mut slots, &shape)?;
        }
        Ok(())
    }

    fn edge(
        &mut self,
        attempt: Attempt,
        face: &Face,
        source: WrappedHistoryKey,
        rails: [&Shape; 2],
        slots: &mut BTreeSet<usize>,
        shape: &Shape,
    ) -> Checked<()> {
        let edge = attempt.native(VALIDATION, "exact wall wire boundary", topods::edge(shape))?;
        let slot = unit_boundary(attempt, &edge, face)?;
        require(
            attempt,
            VALIDATION,
            slots.insert(slot),
            "one occurrence per unit-square boundary",
        )?;
        if slot < 2 {
            return require(
                attempt,
                VALIDATION,
                edge.is_same(rails[slot]),
                "wall shares its actual capped edge",
            );
        }
        let connector = self
            .index
            .reference(&edge, attempt)?
            .ok_or_else(|| missing(attempt, VALIDATION, "decoded indexed wall connector"))?;
        let owns = self.history.images.get(&connector).is_some_and(|parents| parents.iter().any(|parent|
            matches!(parent.source, CylinderFeatureSource::Tool(key) if key.role == HistoryRole::NormalEdge)));
        require(attempt, VALIDATION, owns, "authored connector ancestry")?;
        matches_connector(attempt, &edge, rails, slot - 2)?;
        self.connectors.entry(connector).or_default().push(source);
        Ok(())
    }

    pub(super) fn pairs(self, attempt: Attempt) -> Checked<BTreeSet<[WrappedHistoryKey; 2]>> {
        let mut result = BTreeSet::new();
        for (_, faces) in self.connectors {
            let &[a, b] = faces.as_slice() else {
                return Err(refusal::cardinality(
                    attempt,
                    VALIDATION,
                    "two wall occurrences per decoded connector",
                    [faces.len(), 2],
                    faces.into_iter().map(history_entity),
                ));
            };
            require(
                attempt,
                VALIDATION,
                a != b && result.insert([a.min(b), a.max(b)]),
                "one shared connector per distinct actual side pair",
            )?;
        }
        Ok(result)
    }
}

fn unit_boundary(attempt: Attempt, edge: &topods::Edge, face: &Face) -> Checked<usize> {
    let (_, control) = assembly::polynomial_trace(attempt, face, edge)?;
    let expected = [
        [[0., 0.], [1., 0.]],
        [[0., 1.], [1., 1.]],
        [[0., 0.], [0., 1.]],
        [[1., 0.], [1., 1.]],
    ];
    expected
        .iter()
        .position(|pair| pair.as_slice() == control)
        .ok_or_else(|| {
            missing(
                attempt,
                VALIDATION,
                "exact directed unit-side boundary trace",
            )
        })
}

fn matches_connector(
    attempt: Attempt,
    edge: &topods::Edge,
    rails: [&Shape; 2],
    interval_end: usize,
) -> Checked<()> {
    let world = attempt.native(
        VALIDATION,
        "native ruled edge connector",
        BRepTool::curve_with_location(edge),
    )?;
    let connector = world.curve.downcast::<GeomBezierCurve>();
    require(
        attempt,
        VALIDATION,
        !connector.is_null()
            && !connector.has_eval_representation()
            && connector.nb_poles() == 2
            && [world.first, world.last] == [0., 1.],
        "literal full-domain linear ruled connector",
    )?;
    let vertices = [first_vertex(edge, false), last_vertex(edge, false)];
    for (depth, shape) in rails.into_iter().enumerate() {
        let rail = attempt.native(
            VALIDATION,
            "connector incident cap edge",
            topods::edge(shape),
        )?;
        let expected = if interval_end == 0 {
            first_vertex(&rail, false)
        } else {
            last_vertex(&rail, false)
        };
        let rail = attempt.native(
            VALIDATION,
            "actual connector rail",
            BRepTool::curve_with_location(&rail),
        )?;
        let vertex = &vertices[depth];
        require(
            attempt,
            VALIDATION,
            vertex.is_same(&expected)
                && rail.location.is_equal(&world.location)
                && vertex.location().is_equal(&world.location),
            "one exact located corner shared by both trim domains",
        )?;
        bind_vertex(
            attempt,
            vertex,
            &connector,
            &rail.curve.downcast(),
            depth,
            interval_end,
        )?;
    }
    Ok(())
}

fn bind_vertex(
    attempt: Attempt,
    vertex: &topods::Vertex,
    connector: &GeomBezierCurve,
    rail: &GeomBezierCurve,
    depth: usize,
    end: usize,
) -> Checked<()> {
    require(
        attempt,
        VALIDATION,
        !vertex.is_null(),
        "a retained native corner vertex",
    )?;
    let rail_index = if end == 0 { 1 } else { rail.nb_poles() };
    let rail_pole = attempt.native(VALIDATION, "native corner rail pole", rail.pole(rail_index))?;
    let connector_pole = attempt.native(
        VALIDATION,
        "native corner connector pole",
        connector.pole(depth as i32 + 1),
    )?;
    let weight = attempt.native(
        VALIDATION,
        "native linear connector weight",
        connector.weight(depth as i32 + 1),
    )?;
    let native = attempt
        .native(
            VALIDATION,
            "decoded corner binding",
            vertex_tshape(vertex, "cylindrical corner"),
        )?
        .pnt();
    let bits = rail_pole.coordinates().map(f64::to_bits);
    require(
        attempt,
        VALIDATION,
        native.coordinates().map(f64::to_bits) == bits
            && connector_pole.coordinates().map(f64::to_bits) == bits
            && weight == 1.,
        "literal same raw connector, rail endpoint and vertex coordinates",
    )
}
