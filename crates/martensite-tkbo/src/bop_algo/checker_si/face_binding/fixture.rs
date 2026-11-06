// SPDX-License-Identifier: GPL-2.0-only
use super::types::*;
use martensite_tkg3d::top_abs::Orientation::{Forward, Reversed};

pub(super) fn placement() -> AffinePlacement {
    AffinePlacement {
        linear: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        origin: [0.; 3],
        scale: 1.,
    }
}
pub(super) fn spline() -> OwnedSpline {
    OwnedSpline {
        degree: 2,
        poles: vec![
            [1., 0., 0.],
            [1., 1., 0.],
            [0., 1., 0.],
            [-1., 1., 0.],
            [-1., 0., 0.],
            [-1., -1., 0.],
            [0., -1., 0.],
            [1., -1., 0.],
            [1., 0., 0.],
        ],
        weights: vec![1.; 9],
        knots: vec![0., 1., 2., 3., 4.],
        multiplicities: vec![3, 2, 2, 2, 3],
        periodic: false,
        domain: [0., 4.],
    }
}
pub(super) fn packet() -> OwnedFacePacket {
    let relations = (0..4)
        .flat_map(|left| {
            (left + 1..4).map(move |right| OccurrenceRelation {
                left,
                right,
                same: (left, right) == (0, 1),
                location_equal: true,
            })
        })
        .collect();
    OwnedFacePacket {
        raw_face_tolerance: 0.25,
        vertex_relations: vec![],
        invocation: FaceInvocationIdentity {
            tshape_token: 1,
            location_datums: vec![],
        },
        basis: spline(),
        direction: [0., 0., 1.],
        support_placement: placement(),
        uv_bounds: [0., 4., -2., 0.],
        face_orientation: Reversed,
        face_tolerance: 0.25,
        wire_count: 1,
        edges: edges(),
        relations,
    }
}
fn edges() -> Vec<OwnedEdgeUse> {
    let seam = OwnedSpaceCurve::Line(OwnedLine {
        origin: [1., 0., 0.],
        direction: [0., 0., -1.],
    });
    let first = edge([0., 0.], [0., -1.], [0., 2.], Forward, seam, 1, true);
    let mut last = first.clone();
    last.pcurve.origin[0] = 4.;
    last.canonical_orientation = Reversed;
    last.stored_branch = 2;
    vec![first, last, cap(0., Reversed), cap(-2., Forward)]
}
fn cap(v: f64, orientation: martensite_tkg3d::top_abs::Orientation) -> OwnedEdgeUse {
    let mut cap = edge(
        [0., v],
        [1., 0.],
        [0., 4.],
        orientation,
        OwnedSpaceCurve::Spline(spline()),
        1,
        false,
    );
    cap.placement.origin = [0., 0., v];
    cap
}

fn edge(
    origin: [f64; 2],
    direction: [f64; 2],
    range: [f64; 2],
    canonical_orientation: martensite_tkg3d::top_abs::Orientation,
    curve: OwnedSpaceCurve,
    stored_branch: u8,
    geometric_seam: bool,
) -> OwnedEdgeUse {
    OwnedEdgeUse {
        raw_tolerance: 0.25,
        vertices: vec![],
        context: context(),
        canonical_orientation,
        pcurve: OwnedLine { origin, direction },
        range,
        curve,
        curve_range: range,
        placement: placement(),
        tolerance: 0.25,
        stored_branch,
        geometric_seam,
    }
}

fn context() -> EdgeOccurrenceContext {
    let identity = ShapeOccurrenceIdentity {
        tshape_token: 1,
        location_datums: vec![],
        orientation: Forward,
    };
    EdgeOccurrenceContext {
        face: identity.clone(),
        wire: identity.clone(),
        edge: identity,
        wire_position: 0,
        edge_position: 0,
    }
}
