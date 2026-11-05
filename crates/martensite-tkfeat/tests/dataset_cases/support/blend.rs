//! `blend result shape radius edge [radius edge ...]`, with the command's own defaults.
use super::{
    feat_script::{FeatScript, checked},
    session::Session,
};
use martensite_tkbrep::topods;
use martensite_tkernel::{message::ProgressRange, storage::format_general};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::Shape as Continuity;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// `ta`, `t3d`, `t2d`, `fl`: the command file's defaults, which no case changes.
const TOLERANCES: [f64; 4] = [1.0e-2, 1.0e-4, 1.0e-5, 1.0e-3];
/// `tesp` of the command file.
const TESP: f64 = 1.0e-4;
/// `tapp_angle` of the command file.
const TAPP_ANGLE: f64 = 1.0e-2;

fn tolerance_text() -> String {
    let [ta, t3d, t2d, fl] = TOLERANCES.map(|value| format_general(value, 17));
    format!(
        "tolerance ang : {ta}\ntolerance 3d  : {t3d}\ntolerance 2d  : {t2d}\nfleche        : {fl}\ntolblend {ta} {t3d} {t2d} {fl}"
    )
}

/// `blend`: a name that holds no edge is skipped, and the tolerances are what it answers.
/// Example: `blend(s, "s", "s", &[(0.3, "s_4"), (0.3, "s_2")])`.
pub fn blend(s: &mut FeatScript<'_>, result: &str, shape: &str, edges: &[(f64, &str)]) {
    let source = s.shape_or_null(shape);
    let [ta, t3d, t2d, fl] = TOLERANCES;
    let mut maker = checked(BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational));
    maker.set_params(ta, TESP, t2d, t3d, t2d, fl);
    maker.set_continuity(Continuity::C1, TAPP_ANGLE);
    for (radius, name) in edges {
        let edge = s.shape_or_null(name);
        if !edge.is_null() && matches!(edge.shape_type(), Ok(ShapeEnum::Edge)) {
            checked(maker.add_with_radius(*radius, &checked(topods::edge(&edge))));
        }
    }
    println!("{}", tolerance_text());
    checked(maker.build(&ProgressRange::default()));
    if maker.is_done() {
        let shape = checked(maker.shape()).clone();
        s.bind(result, &shape);
    }
}
