//! Literal dimension input programs and label grids; names follow source command variables.
use super::{Arrow, HPos, VPos};

/// One dimension of a layout: its points (the circle's three points for a radius or a
/// diameter), label, flyout and arrows.
pub(super) struct Layout {
    pub(super) name: String,
    pub(super) points: Vec<[f64; 3]>,
    pub(super) point_names: Vec<String>,
    pub(super) circle_name: String,
    pub(super) label: (HPos, Option<VPos>),
    pub(super) flyout: f64,
    pub(super) arrow: Arrow,
}

fn named_layout(
    name: &str,
    point_names: Vec<String>,
    points: &[[f64; 3]],
    label: HPos,
    flyout: f64,
    arrow: Arrow,
) -> Layout {
    Layout {
        name: name.to_owned(),
        points: points.to_vec(),
        point_names,
        circle_name: String::new(),
        label: (label, None),
        flyout,
        arrow,
    }
}

/// The second viewer's grid: one dimension per label position, external arrows.
/// Build the declared source inputs without a viewer; `grid("d_", &columns, points, 10.0)`.
pub(super) fn grid(
    prefix: &str,
    columns: &[HPos],
    cell: impl Fn(usize, usize) -> Vec<[f64; 3]>,
    flyout: f64,
) -> Vec<Layout> {
    let mut specs = Vec::new();
    for (r, vertical) in VERTICAL_LAYOUT_POSITIONS.into_iter().enumerate() {
        for (c, horizontal) in columns.iter().enumerate() {
            specs.push(declared_grid_cell(
                prefix,
                specs.len(),
                cell(r, c),
                (*horizontal, vertical),
                flyout,
            ));
        }
    }
    specs
}

fn declared_grid_cell(
    prefix: &str,
    index: usize,
    points: Vec<[f64; 3]>,
    label: (HPos, VPos),
    flyout: f64,
) -> Layout {
    let point_names = (0..points.len())
        .map(|p| format!("p_{}", index * points.len() + p))
        .collect();
    Layout {
        name: format!("{prefix}{index}"),
        points,
        point_names,
        circle_name: format!("c_{index}"),
        label: (label.0, Some(label.1)),
        flyout,
        arrow: Arrow::External,
    }
}

pub(super) const VERTICAL_LAYOUT_POSITIONS: [VPos; 3] = [VPos::Above, VPos::Center, VPos::Below];

pub(super) const THREE: [HPos; 3] = [HPos::Left, HPos::Center, HPos::Right];

/// The three points of the circle through (x, z + h), (x + h, z) and (x + 2h, z + h).
/// Build the declared source inputs without a viewer; `arc(0.0, 0.0, 30.0)`.
pub(super) fn arc(x: f64, z: f64, h: f64) -> [[f64; 3]; 3] {
    [[x, 0., z + h], [x + h, 0., z], [x + 2. * h, 0., z + h]]
}

type LengthLayoutInput = (
    &'static str,
    [&'static str; 2],
    [[f64; 3]; 2],
    HPos,
    f64,
    Arrow,
);

const LENGTH_LAYOUTS: [LengthLayoutInput; 4] = [
    (
        "arrow_d1",
        ["arrow_p1", "arrow_p2"],
        [[0., 0., 0.], [50., 0., 0.]],
        HPos::Fit,
        10.,
        Arrow::Internal,
    ),
    (
        "arrow_d2",
        ["arrow_p3", "arrow_p4"],
        [[100., 0., 0.], [150., 0., 0.]],
        HPos::Fit,
        10.,
        Arrow::External,
    ),
    (
        "arrow_d3",
        ["arrow_p5", "arrow_p6"],
        [[100., 0., 50.], [127., 0., 50.]],
        HPos::Center,
        10.,
        Arrow::Fit,
    ),
    (
        "arrow_d4",
        ["arrow_p7", "arrow_p8"],
        [[0., 0., 50.], [10., 0., 50.]],
        HPos::Fit,
        10.,
        Arrow::Fit,
    ),
];

type AngleLayoutInput = (&'static str, [&'static str; 3], f64, f64, f64, HPos, Arrow);

const ANGLE_LAYOUTS: [AngleLayoutInput; 4] = [
    (
        "ang1",
        ["angle1_p1", "angle1_p2", "angle1_p3"],
        0.,
        0.,
        40.,
        HPos::Fit,
        Arrow::Internal,
    ),
    (
        "ang2",
        ["angle2_p1", "angle2_p2", "angle2_p3"],
        80.,
        0.,
        40.,
        HPos::Fit,
        Arrow::External,
    ),
    (
        "ang3",
        ["angle3_p1", "angle3_p2", "angle3_p3"],
        0.,
        80.,
        20.,
        HPos::Center,
        Arrow::Fit,
    ),
    (
        "ang4",
        ["angle4_p1", "angle4_p2", "angle4_p3"],
        85.,
        85.,
        15.,
        HPos::Fit,
        Arrow::Fit,
    ),
];

// diam3 first requests hcenter, then hfit; the last source option is effective.
const DIAMETER_LAYOUTS: [(f64, f64, f64, HPos, Arrow); 4] = [
    (0., 0., 30., HPos::Fit, Arrow::Internal),
    (100., 0., 30., HPos::Fit, Arrow::External),
    (0., 80., 22., HPos::Fit, Arrow::Fit),
    (100., 80., 12., HPos::Fit, Arrow::Fit),
];
const RADIUS_LAYOUTS: [(f64, f64, f64, HPos, Arrow); 4] = [
    (0., 0., 30., HPos::Fit, Arrow::Internal),
    (100., 0., 35., HPos::Fit, Arrow::External),
    (0., 80., 33., HPos::Center, Arrow::Fit),
    (120., 80., 15., HPos::Fit, Arrow::Fit),
];

/// Build the declared source inputs without a viewer; `first_circle_layouts(true)`.
pub(super) fn first_circle_layouts(diameter: bool) -> Vec<Layout> {
    let specs = if diameter {
        &DIAMETER_LAYOUTS
    } else {
        &RADIUS_LAYOUTS
    };
    let prefix = if diameter { "diam" } else { "rad" };
    specs
        .iter()
        .enumerate()
        .map(|(i, &(x, z, height, label, arrow))| {
            let name = format!("{prefix}{}", i + 1);
            let circle = format!("circle{}", i + 1);
            let names = (1..=3).map(|j| format!("{circle}_p{j}")).collect();
            let mut spec = named_layout(&name, names, &arc(x, z, height), label, 0., arrow);
            spec.circle_name = circle;
            spec
        })
        .collect()
}

/// Build the declared source inputs without a viewer; `first_length_layouts()`.
pub(super) fn first_length_layouts() -> [Layout; 4] {
    LENGTH_LAYOUTS.map(|(name, names, points, label, flyout, arrow)| {
        named_layout(
            name,
            names.map(str::to_owned).to_vec(),
            &points,
            label,
            flyout,
            arrow,
        )
    })
}

/// Build the declared source inputs without a viewer; `angle_corner(0.0, 0.0, 40.0)`.
pub(super) fn angle_corner(x: f64, z: f64, height: f64) -> [[f64; 3]; 3] {
    [[x, 0., z + height], [x, 0., z], [x + height, 0., z]]
}

/// Build the declared source inputs without a viewer; `first_angle_layouts()`.
pub(super) fn first_angle_layouts() -> [Layout; 4] {
    ANGLE_LAYOUTS.map(|(name, names, x, z, height, label, arrow)| {
        named_layout(
            name,
            names.map(str::to_owned).to_vec(),
            &angle_corner(x, z, height),
            label,
            height,
            arrow,
        )
    })
}

#[cfg(test)]
mod layout_input_contract {
    use super::*;

    #[test]
    fn layout_grid_uses_each_authored_label_pair_once_with_external_arrows() {
        let cells = grid(
            "d_",
            &THREE,
            |row, column| vec![[column as f64, 0., row as f64]],
            10.,
        );
        assert_eq!(cells.len(), 9);
        for (index, cell) in cells.iter().enumerate() {
            assert_eq!(cell.points, [[(index % 3) as f64, 0., (index / 3) as f64]]);
            assert_eq!(
                cell.label,
                (THREE[index % 3], Some(VERTICAL_LAYOUT_POSITIONS[index / 3]))
            );
            assert_eq!(cell.arrow, Arrow::External);
            assert_eq!(cell.flyout.to_bits(), 10_f64.to_bits());
        }
    }

    #[test]
    fn grid_names_use_the_declared_source_arity_and_dimension_prefix() {
        let pair = grid("d_", &THREE, |_, _| vec![[0.; 3]; 2], 10.);
        let triple = grid("r_", &THREE, |_, _| vec![[0.; 3]; 3], 0.);
        assert_eq!(pair[1].point_names, ["p_2", "p_3"]);
        assert_eq!(triple[1].point_names, ["p_3", "p_4", "p_5"]);
        assert_eq!(pair[1].name, "d_1");
        assert_eq!(triple[1].name, "r_1");
        assert_eq!(triple[1].circle_name, "c_1");
        assert_eq!(
            first_length_layouts()[3].point_names,
            ["arrow_p7", "arrow_p8"]
        );
        assert_eq!(
            first_angle_layouts()[3].point_names,
            ["angle4_p1", "angle4_p2", "angle4_p3"]
        );
    }

    #[test]
    fn circle_layouts_keep_the_distinct_radius_and_diameter_source_geometry() {
        let diameters = first_circle_layouts(true);
        let radii = first_circle_layouts(false);
        assert_eq!(
            diameters[3].points,
            [[100., 0., 92.], [112., 0., 80.], [124., 0., 92.]]
        );
        assert_eq!(
            radii[3].points,
            [[120., 0., 95.], [135., 0., 80.], [150., 0., 95.]]
        );
        assert_eq!(diameters[2].label.0, HPos::Fit);
        assert_eq!(radii[2].label.0, HPos::Center);
        assert_eq!(diameters[0].name, "diam1");
        assert_eq!(radii[0].name, "rad1");
    }
}
