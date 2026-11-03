//! Owned located closed-strip graph inputs.
mod curves;
mod placements;
use super::super::types::*;
use placements::*;
pub(super) const INPUT: GraphInput = GraphInput {
    root: OccurrenceInput {
        owner: 0,
        location: L0,
        orientation: 1,
    },
    owners: &[
        OwnerInput::Face {
            state: 164,
            children: &[OccurrenceInput {
                owner: 1,
                location: L0,
                orientation: 0,
            }],
            surface: 6,
            location: L0,
            tolerance: 0x3e7ad7f29abcaf48,
            natural: false,
        },
        OwnerInput::Wire {
            state: 421,
            children: &[
                OccurrenceInput {
                    owner: 2,
                    location: L0,
                    orientation: 0,
                },
                OccurrenceInput {
                    owner: 2,
                    location: L0,
                    orientation: 1,
                },
                OccurrenceInput {
                    owner: 4,
                    location: L1,
                    orientation: 1,
                },
                OccurrenceInput {
                    owner: 4,
                    location: L2,
                    orientation: 0,
                },
            ],
        },
        OwnerInput::Edge {
            state: 166,
            children: &[
                OccurrenceInput {
                    owner: 3,
                    location: L1,
                    orientation: 0,
                },
                OccurrenceInput {
                    owner: 3,
                    location: L2,
                    orientation: 1,
                },
            ],
            tolerance: 0x3e7ad7f29abcaf48,
            flags: 3,
            representations: &[8, 11],
        },
        OwnerInput::Vertex {
            state: 1447,
            children: &[],
            point: [0x0000000000000000, 0x40273851eb851eb8, 0x0000000000000000],
            tolerance: 0x3e7ad7f29abcaf48,
        },
        OwnerInput::Edge {
            state: 422,
            children: &[
                OccurrenceInput {
                    owner: 3,
                    location: L0,
                    orientation: 0,
                },
                OccurrenceInput {
                    owner: 3,
                    location: L0,
                    orientation: 1,
                },
            ],
            tolerance: 0x3e7ad7f29abcaf48,
            flags: 3,
            representations: &[13, 16, 18, 20],
        },
        OwnerInput::Spline3(curves::C0),
        OwnerInput::Extrusion {
            basis: 5,
            direction: [0x8000000000000000, 0x8000000000000000, 0x3ff0000000000000],
        },
        OwnerInput::Line3 {
            origin: [0x4022000000000000, 0x40404e146e147ae1, 0x4051800000000000],
            direction: [0x0000000000000000, 0x0000000000000000, 0xbff0000000000000],
        },
        OwnerInput::Spatial {
            location: L0,
            range: [0x0000000000000000, 0x4024000000000000],
            curve: 7,
        },
        OwnerInput::Line2 {
            origin: [0x0000000000000000, 0x0000000000000000],
            direction: [0x0000000000000000, 0xbff0000000000000],
        },
        OwnerInput::Line2 {
            origin: [0x403527e7b33f9274, 0x0000000000000000],
            direction: [0x0000000000000000, 0xbff0000000000000],
        },
        OwnerInput::Seam {
            location: L0,
            range: [0x0000000000000000, 0x4024000000000000],
            surface: 6,
            curves: [9, 10],
            continuity: 0,
            endpoints: [
                [
                    [0x0000000000000000, 0x0000000000000000],
                    [0x0000000000000000, 0xc024000000000000],
                ],
                [
                    [0x403527e7b33f9274, 0x0000000000000000],
                    [0x403527e7b33f9274, 0xc024000000000000],
                ],
            ],
        },
        OwnerInput::Spline3(curves::C1),
        OwnerInput::Spatial {
            location: L0,
            range: [0x0000000000000000, 0x403527e7b33f9274],
            curve: 12,
        },
        OwnerInput::Plane {
            origin: [0x0000000000000000, 0x0000000000000000, 0x0000000000000000],
            x: [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
            y: [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
            normal: [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
        },
        OwnerInput::Spline2(curves::C2),
        OwnerInput::Parametric {
            location: L0,
            range: [0x0000000000000000, 0x403527e7b33f9274],
            surface: 14,
            curve: 15,
            endpoints: [
                [0x0000000000000000, 0x40273851eb851eb8],
                [0x0000000000000000, 0x40273851eb851eb8],
            ],
        },
        OwnerInput::Line2 {
            origin: [0x0000000000000000, 0x0000000000000000],
            direction: [0x3ff0000000000000, 0x0000000000000000],
        },
        OwnerInput::Parametric {
            location: L3,
            range: [0x0000000000000000, 0x403527e7b33f9274],
            surface: 6,
            curve: 17,
            endpoints: [
                [0x0000000000000000, 0x0000000000000000],
                [0x403527e7b33f9274, 0x0000000000000000],
            ],
        },
        OwnerInput::Line2 {
            origin: [0x0000000000000000, 0xc024000000000000],
            direction: [0x3ff0000000000000, 0x0000000000000000],
        },
        OwnerInput::Parametric {
            location: L4,
            range: [0x0000000000000000, 0x403527e7b33f9274],
            surface: 6,
            curve: 19,
            endpoints: [
                [0x0000000000000000, 0xc024000000000000],
                [0x403527e7b33f9274, 0xc024000000000000],
            ],
        },
    ],
    datums: DATUMS,
};
