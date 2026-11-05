//! `pipe`, `bsmooth`, the `mksweep` family and the projection and thick-shell commands of the
//! viewer scripts.
use crate::support::restore::input_bytes;
use crate::variables::Variables;
use martensite_tkbool::brep_fill::TypeOfContact;
use martensite_tkbrep::topods;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::{Array1, HArray1, List};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::arrays::one_based;
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCurve};
use martensite_tkgeomalgo::geom_fill::GeomFillTrihedron;
use martensite_tkgeombase::app_def::{
    MultiLine, MultiPointConstraint, Variational, VariationalOptions,
};
use martensite_tkgeombase::app_par_curves::{Constraint, ConstraintCouple, MultiCurveEval as _};
use martensite_tkmath::geom_abs::{JoinType, Shape as Continuity};
use martensite_tkmath::gp::Point3;
use martensite_tkoffset::brep_offset::{BRepOffsetMakeOffset, BRepOffsetMode, MakeOffsetOptions};
use martensite_tkoffset::brep_offset_api::{
    BRepOffsetAPIMakePipe, BRepOffsetAPIMakePipeShell, BRepOffsetAPINormalProjection,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape as _, BRepBuilderAPITransitionMode,
};

/// `pipe result spine profile`; `pipe(&mut variables, "result", "c1", "c2")`.
pub fn pipe(variables: &mut Variables, result: &str, spine: &str, profile: &str) {
    let spine = topods::wire(&variables.get(spine)).unwrap();
    let profile = variables.get(profile);
    let trihedron = GeomFillTrihedron::IsCorrectedFrenet;
    let mut builder = BRepOffsetAPIMakePipe::with_mode(&spine, &profile, trihedron, false).unwrap();
    variables.set(result, builder.shape().unwrap().clone());
}

/// The points of a `bsmooth` file: their count, `3d`, then their coordinates.
fn point_constraints(file: &str) -> Vec<MultiPointConstraint> {
    let text = String::from_utf8(input_bytes(file)).unwrap();
    let mut words = text.split_whitespace();
    let count: usize = words.next().unwrap().parse().unwrap();
    assert_eq!(words.next(), Some("3d"), "{file}: expected 3d points");
    let reals: Vec<f64> = words.map(|word| word.parse().unwrap()).collect();
    assert_eq!(
        reals.len(),
        3 * count,
        "{file}: coordinates of {count} points"
    );
    let constraint_of = |p: &[f64]| {
        let constraint = MultiPointConstraint::with_counts(1, 0).unwrap();
        constraint
            .set_point(1, Point3::new(p[0], p[1], p[2]))
            .unwrap();
        constraint
    };
    reals.chunks_exact(3).map(constraint_of).collect()
}

/// `bsmooth name tolerance file`, for a positive tolerance over 3D points.
/// Example: `smooth_curve("points.txt", 1.)`.
pub fn smooth_curve(file: &str, tolerance: f64) -> Handle<dyn GeomCurve> {
    let points = point_constraints(file);
    let line = MultiLine::from_multi_points(&Array1::from_shared(&points, 1));
    let mut constraints = one_based(points.len() as i32).unwrap();
    for index in 1..=points.len() as i32 {
        let couple = ConstraintCouple::new(index, Constraint::NoConstraint);
        constraints.set_value(index, couple).ok().unwrap();
    }
    let table = Handle::new(HArray1::from_owned(constraints));
    let last = points.len() as i32;
    let options = VariationalOptions::default();
    let mut variation = Variational::new(&line, 1, last, &table, options).unwrap();
    variation.set_tolerance(tolerance).unwrap();
    variation.set_with_min_max(true).unwrap();
    variation.approximate().unwrap();
    let curve = variation.value().unwrap();
    let mut poles = one_based(curve.nb_poles()).unwrap();
    curve.curve_3d(1, &mut poles).unwrap();
    let (knots, mults) = (curve.knots().unwrap(), curve.multiplicities().unwrap());
    let degree = curve.degree().unwrap();
    let bspline = GeomBSplineCurve::new(&poles, &knots, &mults, degree, false).unwrap();
    Handle::new(bspline).cast()
}

/// `nproject result wire face tolerance continuity`.
/// Example: `normal_projection(&mut variables, "result", "w", "f", 1.e-4, Continuity::C1)`.
pub fn normal_projection(
    variables: &mut Variables,
    (result, wire, face): (&str, &str, &str),
    tolerance: f64,
    continuity: Continuity,
) {
    let mut projection = BRepOffsetAPINormalProjection::with_shape(&variables.get(face));
    projection.add(&variables.get(wire)).unwrap();
    projection.set_params(
        tolerance,
        martensite_cmath::pow(tolerance, 2.0 / 3.0),
        continuity,
        14,
        16,
    );
    projection.build(&ProgressRange::default()).unwrap();
    let mut wires = List::default();
    if projection.build_wire(&mut wires).unwrap() {
        println!(" BuildWire OK ");
    }
    variables.set(result, projection.shape().unwrap().clone());
}

/// `thickshell result shape value i`: a thickening with intersection joins.
/// Example: `thick_shell(&mut variables, "result", "a", 1.)`.
pub fn thick_shell(variables: &mut Variables, result: &str, shape: &str, value: f64) {
    let mut builder = BRepOffsetMakeOffset::new();
    let options = MakeOffsetOptions {
        mode: BRepOffsetMode::Skin,
        intersection: false,
        self_inter: false,
        join: JoinType::Intersection,
        thickening: true,
        remove_int_edges: false,
    };
    let tolerance = martensite_tkernel::precision::confusion();
    builder.initialize(&variables.get(shape), value, tolerance, options);
    builder
        .make_offset_shape(&ProgressRange::default())
        .unwrap();
    assert!(
        !builder.shape().is_null(),
        "thickshell {shape}: null result"
    );
    variables.set(result, builder.shape().clone());
}

/// `mksweep`, its options, `addsweep` and `buildsweep`, on one builder; the commands' statics.
pub struct Sweep {
    builder: BRepOffsetAPIMakePipeShell,
}

impl Sweep {
    /// `mksweep spine`; `Sweep::start(&variables.get("spine"))`.
    pub fn start(spine: &Shape) -> Self {
        let spine = topods::wire(spine).unwrap();
        Self {
            builder: BRepOffsetAPIMakePipeShell::new(&spine).unwrap(),
        }
    }

    /// `setsweep -G guide 0 0`: a guide with plane equivalence and no contact.
    pub fn guide(&mut self, guide: &Shape) {
        let guide = topods::wire(guide).unwrap();
        self.builder
            .set_mode_auxiliary(&guide, false, TypeOfContact::NoContact)
            .unwrap();
    }

    /// `setsweep -SM segments`.
    pub fn max_segments(&mut self, segments: i32) {
        self.builder.set_max_segments(segments);
    }

    /// `setsweep -DM degree`.
    pub fn max_degree(&mut self, degree: i32) {
        self.builder.set_max_degree(degree);
    }

    /// `addsweep section`.
    pub fn add(&mut self, section: &Shape) {
        self.builder.add(section, false, false).unwrap();
    }

    /// `buildsweep -S [-C]`: the solid of the sweep.
    pub fn build_solid(&mut self, right_corner: bool) -> Shape {
        if right_corner {
            self.builder
                .set_transition_mode(BRepBuilderAPITransitionMode::RightCorner);
        }
        self.builder.build(&ProgressRange::default()).unwrap();
        assert!(self.builder.is_done(), "buildsweep: not done");
        if !self.builder.make_solid().unwrap() {
            println!(" BuildSweep : It is impossible to make a solid !");
        }
        self.builder.shape().unwrap().clone()
    }

    /// `errorsweep`: the reached surface tolerance.
    pub fn error_on_surface(&self) -> f64 {
        self.builder.error_on_surface()
    }
}
