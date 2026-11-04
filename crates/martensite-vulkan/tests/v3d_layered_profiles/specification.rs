//! Literal polyline vertices, text positions and pixel offsets come from both cases.
use martensite_tkservice::aspect::AspectTypeOfTriedronPosition as Corner;
pub(super) struct ScreenPair {
    pub text_name: &'static str,
    pub line_name: &'static str,
    pub text: &'static str,
    pub position: [f64; 3],
    pub corner: Corner,
    pub text_offset: [i32; 2],
    pub line_offset: [i32; 2],
    pub points: &'static [[f64; 3]],
}
const CENTER: [[f64; 3]; 6] = [
    [-50., -50., 0.],
    [-50., 50., 0.],
    [50., 50., 0.],
    [50., 0., 0.],
    [0., -50., 0.],
    [-50., -50., 0.],
];
const LOWER_LEFT: [[f64; 3]; 6] = [
    [0., 0., 0.],
    [0., 100., 0.],
    [100., 100., 0.],
    [100., 50., 0.],
    [50., 0., 0.],
    [0., 0., 0.],
];
const UPPER_LEFT: [[f64; 3]; 6] = [
    [0., -100., 0.],
    [0., 0., 0.],
    [100., 0., 0.],
    [100., -50., 0.],
    [50., -100., 0.],
    [0., -100., 0.],
];
const UPPER_RIGHT: [[f64; 3]; 6] = [
    [-100., -100., 0.],
    [-100., 0., 0.],
    [0., 0., 0.],
    [0., -50., 0.],
    [-50., -100., 0.],
    [-100., -100., 0.],
];
const LOWER_RIGHT: [[f64; 3]; 6] = [
    [-100., 0., 0.],
    [-100., 100., 0.],
    [0., 100., 0.],
    [0., 50., 0.],
    [-50., 0., 0.],
    [-100., 0., 0.],
];
pub(super) const PAIRS: [ScreenPair; 5] = [
    ScreenPair {
        text_name: "tcc",
        line_name: "lcc",
        text: "Center",
        position: [0.; 3],
        corner: Corner::CENTER,
        text_offset: [0; 2],
        line_offset: [0; 2],
        points: &CENTER,
    },
    ScreenPair {
        text_name: "tbl",
        line_name: "lbl",
        text: "Bottom-Left",
        position: [0.; 3],
        corner: Corner::LEFT_LOWER,
        text_offset: [5; 2],
        line_offset: [3; 2],
        points: &LOWER_LEFT,
    },
    ScreenPair {
        text_name: "ttl",
        line_name: "ltl",
        text: "Top-Left",
        position: [-27., 0., 0.],
        corner: Corner::LEFT_UPPER,
        text_offset: [30; 2],
        line_offset: [3; 2],
        points: &UPPER_LEFT,
    },
    ScreenPair {
        text_name: "ttr",
        line_name: "ltr",
        text: "Top-Right",
        position: [-100., 0., 0.],
        corner: Corner::RIGHT_UPPER,
        text_offset: [30; 2],
        line_offset: [3; 2],
        points: &UPPER_RIGHT,
    },
    ScreenPair {
        text_name: "tbr",
        line_name: "lbr",
        text: "Bottom-Right",
        position: [-165., 0., 0.],
        corner: Corner::RIGHT_LOWER,
        text_offset: [5; 2],
        line_offset: [3; 2],
        points: &LOWER_RIGHT,
    },
];
