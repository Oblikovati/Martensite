//! Exact supplied control points from bug23706_10 through _15.
use super::SplineInput;

pub const C10_1: SplineInput = SplineInput {
    poles: &[
        [2.0, 5.0],
        [3.0, 7.0],
        [4.0, 8.0],
        [4.0, 8.0],
        [4.0, 8.0],
        [5.0, 9.0],
        [9.0, 7.0],
    ],
    knots: &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    multiplicities: &[3, 1, 1, 1, 1, 3],
};

pub const C10_2: SplineInput = SplineInput {
    poles: &[
        [-1.0, 2.0],
        [1.0, 11.0],
        [3.0, 9.0],
        [3.0, 9.0],
        [3.0, 9.0],
        [5.0, 7.0],
        [7.0, 4.0],
    ],
    knots: &[2.0, 2.5, 3.0, 3.5, 4.0, 4.5],
    multiplicities: &[3, 1, 1, 1, 1, 3],
};

pub const C11_1: SplineInput = SplineInput {
    poles: &[
        [2.0, 5.0],
        [3.0, 7.0],
        [4.0, 8.0],
        [4.0, 8.0],
        [5.0, 9.0],
        [9.0, 7.0],
    ],
    knots: &[1.0, 2.0, 3.0, 4.0, 5.0],
    multiplicities: &[3, 1, 1, 1, 3],
};

pub const C11_2: SplineInput = SplineInput {
    poles: &[
        [-1.0, 2.0],
        [1.0, 11.0],
        [3.0, 9.0],
        [3.0, 9.0],
        [3.0, 9.0],
        [5.0, 7.0],
        [7.0, 4.0],
    ],
    knots: &[2.0, 2.5, 3.0, 3.5, 4.0],
    multiplicities: &[3, 1, 1, 1, 3],
};

pub const C12_1: SplineInput = SplineInput {
    poles: &[
        [4.0, -3.0],
        [6.0, 8.0],
        [10.0, 11.0],
        [10.0, 11.0],
        [10.0, 11.0],
        [14.0, 14.0],
        [5.0, 8.0],
    ],
    knots: &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    multiplicities: &[3, 1, 1, 1, 1, 3],
};

pub const C12_2: SplineInput = SplineInput {
    poles: &[
        [5.0, 20.0],
        [8.0, 15.0],
        [12.0, 18.0],
        [12.0, 18.0],
        [12.0, 18.0],
        [16.0, 21.0],
        [7.0, 12.0],
    ],
    knots: &[2.0, 2.5, 3.0, 3.5, 4.0, 4.5],
    multiplicities: &[3, 1, 1, 1, 1, 3],
};

pub const C13_1: SplineInput = SplineInput {
    poles: &[
        [2.0, 5.0],
        [3.0, 7.0],
        [4.0, 8.0],
        [4.0, 8.0],
        [4.0, 8.0],
        [5.0, 9.0],
        [9.0, 7.0],
    ],
    knots: &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
    multiplicities: &[3, 1, 1, 1, 1, 3],
};

pub const C13_2: SplineInput = SplineInput {
    poles: &[
        [-1.0, 2.0],
        [1.0, 11.0],
        [3.0, 9.0],
        [3.0, 9.0],
        [3.0, 9.0],
        [5.0, 7.0],
        [7.0, 4.0],
    ],
    knots: &[2.0, 2.5, 3.0, 3.5, 4.0, 4.5],
    multiplicities: &[3, 1, 1, 1, 1, 3],
};

pub const C14_1: SplineInput = SplineInput {
    poles: &[
        [4.0, -3.0],
        [6.0, 8.0],
        [10.0, 11.0],
        [10.0, 11.0],
        [10.0, 11.0],
        [14.0, 14.0],
        [5.0, 8.0],
    ],
    knots: &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
    multiplicities: &[2, 1, 1, 1, 1, 1, 1, 2],
};

pub const C14_2: SplineInput = SplineInput {
    poles: &[
        [5.0, 20.0],
        [8.0, 15.0],
        [12.0, 18.0],
        [12.0, 18.0],
        [12.0, 18.0],
        [16.0, 21.0],
        [7.0, 12.0],
    ],
    knots: &[2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5],
    multiplicities: &[2, 1, 1, 1, 1, 1, 1, 2],
};

pub const C15_1: SplineInput = SplineInput {
    poles: &[
        [4.0, -3.0],
        [6.0, 8.0],
        [10.0, 11.0],
        [10.0, 11.0],
        [14.0, 14.0],
        [5.0, 8.0],
    ],
    knots: &[1.0, 2.0, 3.0, 4.0, 5.0],
    multiplicities: &[3, 1, 1, 1, 3],
};

pub const C15_2: SplineInput = SplineInput {
    poles: &[
        [5.0, 20.0],
        [8.0, 15.0],
        [12.0, 18.0],
        [12.0, 18.0],
        [16.0, 21.0],
        [7.0, 12.0],
    ],
    knots: &[2.0, 2.5, 3.0, 3.5, 4.0],
    multiplicities: &[3, 1, 1, 1, 3],
};
