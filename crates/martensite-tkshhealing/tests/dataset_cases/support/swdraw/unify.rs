//! `unifysamedom result shape [s1 s2 ...] [-f] [-e] [-nosafe]
//! [+b] [+i] [-t val] [-a val]` (ShapeUpgrade_UnifySameDomain).
use super::atof;
use martensite_tkbrep::{topods::Shape, toptools::MapOfShape};
use martensite_tkernel::precision;
use martensite_tkshhealing::shape_upgrade::ShapeUpgradeUnifySameDomain;

/// The command on `shape` with the kept shapes and the option words in script order;
/// `unifysamedom(&a, &[], &["+b"])`.
pub fn unifysamedom(shape: &Shape, keep: &[&Shape], options: &[&str]) -> Shape {
    let (mut unify_faces, mut unify_edges, mut concat_bsplines) = (true, true, false);
    let (mut allow_internal, mut safe_input) = (false, true);
    let (mut linear, mut angular) = (precision::confusion(), precision::angular());
    let mut words = options.iter();
    while let Some(word) = words.next() {
        match *word {
            "-f" => unify_faces = false,
            "-e" => unify_edges = false,
            "-nosafe" => safe_input = false,
            "+b" => concat_bsplines = true,
            "+i" => allow_internal = true,
            "-t" => linear = atof(words.next().expect("value expected after -t")),
            "-a" => {
                let degrees = atof(words.next().expect("value expected after -a"));
                angular = degrees * (std::f64::consts::PI / 180.0);
            }
            _ => {}
        }
    }
    let mut kept = MapOfShape::default();
    for shape in keep {
        kept.add((*shape).clone());
    }
    let unifier = ShapeUpgradeUnifySameDomain::empty();
    unifier
        .initialize(shape, unify_edges, unify_faces, concat_bsplines)
        .unwrap_or_else(|failure| panic!("unifysamedom Initialize: {failure:?}"));
    unifier.keep_shapes(&kept).unwrap();
    unifier.set_safe_input_mode(safe_input);
    unifier.allow_internal_edges(allow_internal);
    unifier.set_linear_tolerance(linear);
    unifier.set_angular_tolerance(angular);
    unifier
        .build()
        .unwrap_or_else(|failure| panic!("unifysamedom: {failure:?}"));
    unifier.shape()
}
