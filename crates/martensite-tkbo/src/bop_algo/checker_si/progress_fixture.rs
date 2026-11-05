//! One owned checker retains identical source construction for whole and prepared stages.
use super::*;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{Compound, Shape},
};
use martensite_tkernel::{ncollection::List, precision, standard::Handle};
use martensite_tkg3d::{
    geom::{GeomSurface, GeomToroidalSurface},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Axis3, Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
    BRepBuilderAPIMakeVertex, BRepBuilderAPINurbsConvert,
};
pub(super) struct CheckerCase {
    pub checker: BOPAlgoCheckerSI,
    pub(super) kind: usize,
    pub(super) prepared: Option<PreparedIncidence>,
}
pub(super) struct PreparedIncidence {
    pub(super) pairs: Vec<(i32, i32)>,
    pub(super) graph: std::collections::BTreeSet<(i32, i32)>,
}
impl CheckerCase {
    /// Dispatch the actual receiver or prepared stage; `case.perform(range)?`.
    pub fn perform(&mut self, range: &ProgressRange<'_>) -> Result<ProgressCompletion, Failure> {
        if self.kind < 40 {
            return Ok(self.checker.perform_on_progress(range));
        }
        let observed = ProgressCancellation::default();
        if (40..=42).contains(&self.kind) {
            return self.checker.check_faces_on_progress(range, &observed, None);
        }
        self.perform_leaf(range, &observed)?;
        Ok(classify_area_progress(
            ProgressOperation::CheckerSI,
            self.checker.options(),
            Ok(()),
            &observed,
        ))
    }
    fn perform_leaf(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        match self.kind {
            43 => self.checker.perform_vz_on_progress(range, observed),
            44 | 47 => self.checker.perform_ez_on_progress(range, observed),
            45 | 48 => self.checker.perform_fz_on_progress(range, observed),
            46 | 49 => self.checker.perform_zz_on_progress(range, observed),
            actual => Err(Failure::out_of_range(&format!(
                "prepared CheckerSI profile={actual}; expected 40..=49"
            ))),
        }
    }
}
/// Build retained self-check inputs and seeded record graphs; `checker_case(43,false)` prepares vertex/solid classification.
pub(super) fn checker_case(kind: usize, parallel: bool) -> CheckerCase {
    let mut checker = BOPAlgoCheckerSI::new();
    checker.options_mut().set_run_parallel(parallel);
    checker.set_level_of_check(if (20..30).contains(&kind) {
        (kind - 20) as i32
    } else {
        9
    });
    let sources = match kind {
        0 => vec![],
        1 => vec![
            checker_vertex(Point3::new(0., 0., 0.)),
            checker_vertex(Point3::new(10., 0., 0.)),
        ],
        _ => vec![checker_input(kind)],
    };
    checker.set_arguments(&sources.into_iter().collect::<List<Shape>>());
    if kind >= 40 {
        let level = checker.level_of_check();
        initialize_checker(&mut checker.filler, level).unwrap();
        seed_checker_graph(&mut checker, kind);
    }
    let prepared = if matches!(kind, 44 | 45) {
        Some(capture_prepared_incidence(&mut checker, kind))
    } else {
        None
    };
    CheckerCase {
        checker,
        kind,
        prepared,
    }
}
fn checker_source_kind(kind: usize) -> usize {
    match kind {
        20..=29 | 45 | 46 | 48 | 49 => 7,
        40 => 11,
        41 => 13,
        42 => 12,
        43 => 9,
        44 | 47 => 10,
        other => other,
    }
}
fn checker_input(kind: usize) -> Shape {
    match checker_source_kind(kind) {
        2 => checker_vertex(Point3::new(0., 0., 0.)),
        3 | 4 => checker_compound([
            checker_vertex(Point3::new(0., 0., 0.)),
            checker_vertex(Point3::new(if kind == 4 { 0. } else { 10. }, 0., 0.)),
        ]),
        5 => checker_compound([
            checker_edge(Point3::new(-1., 0., 0.), Point3::new(1., 0., 0.)),
            checker_edge(Point3::new(0., -1., 0.), Point3::new(0., 1., 0.)),
        ]),
        6 => checker_box(0., 0., 0., 10.),
        7 => checker_compound([checker_box(0., 0., 0., 10.), checker_box(5., 5., 5., 10.)]),
        8 => checker_compound([checker_box(0., 0., 0., 10.), checker_box(2., 2., 2., 4.)]),
        9 => checker_compound([
            checker_box(0., 0., 0., 10.),
            checker_vertex(Point3::new(3., 3., 3.)),
        ]),
        10 => checker_compound([
            checker_box(0., 0., 0., 10.),
            checker_edge(Point3::new(5., 5., -5.), Point3::new(5., 5., 15.)),
        ]),
        11 => checker_nurbs_box(),
        12 => checker_torus(2.),
        13 => checker_torus(5.),
        14 => Shape::default(),
        actual => panic!("CheckerSI source profile={actual}; expected 2..=14 or a mapped profile"),
    }
}
fn checker_box(x: f64, y: f64, z: f64, size: f64) -> Shape {
    let mut maker = BRepPrimAPIMakeBox::at_point(Point3::new(x, y, z), size, size, size).unwrap();
    maker.shape().unwrap().clone()
}
fn checker_vertex(point: Point3) -> Shape {
    BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn checker_edge(first: Point3, second: Point3) -> Shape {
    BRepBuilderAPIMakeEdge::from_points(first, second)
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}
fn checker_compound(shapes: [Shape; 2]) -> Shape {
    let mut compound = Compound::default();
    let builder = BRepBuilder::default();
    builder.make_compound(&mut compound);
    for shape in shapes {
        builder.add(&mut compound, &shape).unwrap();
    }
    compound.into_shape()
}
fn checker_nurbs_box() -> Shape {
    let shape = checker_box(0., 0., 0., 10.);
    let mut converter = BRepBuilderAPINurbsConvert::with_shape(&shape, false).unwrap();
    converter.shape().unwrap().clone()
}
fn checker_torus(major: f64) -> Shape {
    let surface: Handle<dyn GeomSurface> =
        Handle::new(GeomToroidalSurface::new(Axis3::default(), major, 3.).unwrap()).into_cast();
    BRepBuilderAPIMakeFace::from_surface(&surface, precision::confusion())
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}
fn seed_checker_graph(checker: &mut BOPAlgoCheckerSI, kind: usize) {
    if !(44..=46).contains(&kind) {
        return;
    }
    let tag = match kind {
        44 => ShapeEnum::Edge,
        45 => ShapeEnum::Face,
        _ => ShapeEnum::Solid,
    };
    let structure = checker.data_structure_mut().unwrap();
    for index in 0..structure.nb_source_shapes() {
        if structure.shape_info(index).unwrap().shape_type() != tag {
            continue;
        }
        seed_shape_solids(structure, index, kind == 46);
    }
}
fn seed_shape_solids(structure: &mut DataStructure, index: i32, every: bool) {
    for solid in 0..structure.nb_source_shapes() {
        let info = structure.shape_info(solid).unwrap();
        if info.shape_type() != ShapeEnum::Solid || solid == index {
            continue;
        }
        let subs = info.sub_shapes().to_vec();
        for sub in subs {
            structure.add_interference(index, sub);
            if !every {
                break;
            }
        }
    }
}

fn capture_prepared_incidence(checker: &mut BOPAlgoCheckerSI, kind: usize) -> PreparedIncidence {
    let graph = checker
        .data_structure()
        .unwrap()
        .interferences()
        .iter()
        .map(|pair| canonical_pair(pair.indices()))
        .collect();
    let iterator = checker.iterator_mut().unwrap();
    iterator.initialize(
        if kind == 44 {
            ShapeEnum::Edge
        } else {
            ShapeEnum::Face
        },
        ShapeEnum::Solid,
    );
    let mut pairs = Vec::new();
    while iterator.more() {
        pairs.push(iterator.value().unwrap());
        iterator.next();
    }
    PreparedIncidence { pairs, graph }
}
/// Normalize an undirected input relation for set comparison; `canonical_pair((second,first))`.
pub(super) fn canonical_pair((first, second): (i32, i32)) -> (i32, i32) {
    if first <= second {
        (first, second)
    } else {
        (second, first)
    }
}
