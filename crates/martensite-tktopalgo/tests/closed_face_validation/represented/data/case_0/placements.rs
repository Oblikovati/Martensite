//! Raw datum and cached-map inputs retain form and coefficient bits.
use super::super::super::types::*;
const T0: TransformInput = TransformInput {
    form: 2,
    scale: 0x3ff0000000000000,
    matrix: [
        [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    ],
    translation: [0xc03b000000000000, 0xc022000033333333, 0x0000000000000000],
};
const T1: TransformInput = TransformInput {
    form: 7,
    scale: 0x3ff0000000000000,
    matrix: [
        [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    ],
    translation: [0x0000000000000000, 0x403e000000000000, 0x4051800000000000],
};
const T2: TransformInput = TransformInput {
    form: 2,
    scale: 0x3ff0000000000000,
    matrix: [
        [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    ],
    translation: [0x0000000000000000, 0x0000000000000000, 0xc024000000000000],
};
const T3: TransformInput = TransformInput {
    form: 0,
    scale: 0x3ff0000000000000,
    matrix: [
        [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    ],
    translation: [0x0000000000000000, 0x0000000000000000, 0x0000000000000000],
};
const T4: TransformInput = TransformInput {
    form: 7,
    scale: 0x3ff0000000000000,
    matrix: [
        [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    ],
    translation: [0xc03b000000000000, 0x4034ffffe6666666, 0x4051800000000000],
};
const T5: TransformInput = TransformInput {
    form: 7,
    scale: 0x3ff0000000000000,
    matrix: [
        [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    ],
    translation: [0xc03b000000000000, 0x4034ffffe6666666, 0x404e000000000000],
};
const T6: TransformInput = TransformInput {
    form: 7,
    scale: 0x3ff0000000000000,
    matrix: [
        [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    ],
    translation: [0x403b000000000000, 0xc034ffffe6666666, 0xc051800000000000],
};
const T7: TransformInput = TransformInput {
    form: 7,
    scale: 0x3ff0000000000000,
    matrix: [
        [0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000],
        [0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000],
    ],
    translation: [0x403b000000000000, 0xc034ffffe6666666, 0xc04e000000000000],
};
pub(super) const L0: LocationInput = LocationInput {
    steps: &[],
    effective: T3,
};
pub(super) const L1: LocationInput = LocationInput {
    steps: &[(0, 1), (1, 1)],
    effective: T4,
};
pub(super) const L2: LocationInput = LocationInput {
    steps: &[(0, 1), (1, 1), (2, 1)],
    effective: T5,
};
pub(super) const L3: LocationInput = LocationInput {
    steps: &[(1, -1), (0, -1)],
    effective: T6,
};
pub(super) const L4: LocationInput = LocationInput {
    steps: &[(2, -1), (1, -1), (0, -1)],
    effective: T7,
};
pub(super) const DATUMS: &[TransformInput] = &[T0, T1, T2];
