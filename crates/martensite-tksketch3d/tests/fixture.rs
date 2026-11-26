// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkmath::gp::Point3;
use martensite_tksketch3d::{
    Edit, EditFailure, LineId, MinimumLength, PointId, PointRole, SketchLine, SketchPoint, Snapshot,
};
use std::num::NonZeroU64;

pub(super) fn pid(value: u64) -> PointId {
    PointId::new(NonZeroU64::new(value).unwrap())
}

pub(super) fn lid(value: u64) -> LineId {
    LineId::new(NonZeroU64::new(value).unwrap())
}

pub(super) fn point([x, y, z]: [f64; 3]) -> Point3 {
    Point3::new(x, y, z)
}

pub(super) fn seed(id: u64, xyz: [f64; 3], role: PointRole) -> Edit {
    Edit::InsertPoint(SketchPoint {
        id: pid(id),
        role,
        position: point(xyz),
    })
}

pub(super) fn line(id: u64, first: u64, second: u64) -> Edit {
    Edit::InsertLine(SketchLine {
        id: lid(id),
        endpoints: [pid(first), pid(second)],
    })
}

fn empty(limit: f64) -> Snapshot {
    Snapshot::empty(MinimumLength::new(limit).unwrap())
}

pub(super) fn connect(start: [f64; 3], end: [f64; 3], limit: f64) -> Result<Snapshot, EditFailure> {
    empty(limit).apply(
        &[
            seed(1, start, PointRole::Defining),
            seed(2, end, PointRole::Defining),
            line(3, 1, 2),
        ],
        || false,
    )
}

pub(super) fn baseline() -> Snapshot {
    empty(0.01)
        .apply(
            &[
                line(6, 2, 3),
                seed(3, [4., 3., 0.], PointRole::Defining),
                seed(1, [0., 0., 0.], PointRole::Defining),
                line(5, 1, 2),
                seed(2, [4., 0., 0.], PointRole::SketchMarker),
                seed(4, [8., 8., 8.], PointRole::CenterMarker),
            ],
            || false,
        )
        .unwrap()
}
