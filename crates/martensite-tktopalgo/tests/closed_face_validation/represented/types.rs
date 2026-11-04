//! Explicit immutable inputs for located closed-strip graph fixtures.
#[derive(Clone, Copy, Debug)]
pub(super) struct TransformInput {
    pub form: i32,
    pub scale: u64,
    pub matrix: [[u64; 3]; 3],
    pub translation: [u64; 3],
}
#[derive(Clone, Copy, Debug)]
pub(super) struct LocationInput {
    pub steps: &'static [(usize, i32)],
    pub effective: TransformInput,
}
#[derive(Clone, Copy, Debug)]
pub(super) struct OccurrenceInput {
    pub owner: usize,
    pub location: LocationInput,
    pub orientation: i32,
}
#[derive(Clone, Copy, Debug)]
pub(super) struct SplineInput<const D: usize> {
    pub poles: &'static [[u64; D]],
    pub weights: &'static [u64],
    pub knots: &'static [u64],
    pub multiplicities: &'static [i32],
    pub flat_knots: &'static [u64],
    pub domain: [u64; 2],
    pub degree: i32,
    pub periodic: bool,
    pub rational: bool,
    pub evaluation_override: bool,
    pub lower_bounds: [i32; 5],
}
#[derive(Clone, Copy, Debug)]
pub(super) enum OwnerInput {
    Face {
        state: u16,
        children: &'static [OccurrenceInput],
        surface: usize,
        location: LocationInput,
        tolerance: u64,
        natural: bool,
    },
    Wire {
        state: u16,
        children: &'static [OccurrenceInput],
    },
    Edge {
        state: u16,
        children: &'static [OccurrenceInput],
        tolerance: u64,
        flags: i32,
        representations: &'static [usize],
    },
    Vertex {
        state: u16,
        children: &'static [OccurrenceInput],
        point: [u64; 3],
        tolerance: u64,
    },
    Spatial {
        location: LocationInput,
        range: [u64; 2],
        curve: usize,
    },
    Parametric {
        location: LocationInput,
        range: [u64; 2],
        curve: usize,
        surface: usize,
        endpoints: [[u64; 2]; 2],
    },
    Seam {
        location: LocationInput,
        range: [u64; 2],
        curves: [usize; 2],
        surface: usize,
        continuity: i32,
        endpoints: [[[u64; 2]; 2]; 2],
    },
    Line3 {
        origin: [u64; 3],
        direction: [u64; 3],
    },
    Line2 {
        origin: [u64; 2],
        direction: [u64; 2],
    },
    Spline3(SplineInput<3>),
    Spline2(SplineInput<2>),
    Plane {
        origin: [u64; 3],
        x: [u64; 3],
        y: [u64; 3],
        normal: [u64; 3],
    },
    Extrusion {
        basis: usize,
        direction: [u64; 3],
    },
}
#[derive(Clone, Copy, Debug)]
pub(super) struct GraphInput {
    pub root: OccurrenceInput,
    pub owners: &'static [OwnerInput],
    pub datums: &'static [TransformInput],
}
pub(super) fn scalars<const D: usize>(bits: [u64; D]) -> [f64; D] {
    bits.map(f64::from_bits)
}
