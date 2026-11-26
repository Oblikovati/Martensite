//! Section boundary provenance from real edge/face common blocks (#613).
use martensite_tkbo::{
    bop_algo::ProgressCompletion,
    brep_algo_api::{BRepAlgoAPIBooleanOperation, BRepAlgoAPISection},
};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
    topods::{self, Compound, Face, Shape},
};
use martensite_tkernel::message::{
    ProgressIndicator, ProgressIndicatorHooks, ProgressRange, ProgressScope,
};
use martensite_tkg3d::{
    geom::{GeomCircle, GeomLine},
    top_abs::ShapeEnum,
};
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Point3, Vector3};
use martensite_tkprim::{
    brep_prim_api::{BRepPrimAPIMakeCylinder, BRepPrimAPIMakeOneAxis},
    brep_sweep::BRepSweepPrism,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape,
};
use std::sync::Arc;

fn cylinder() -> Face {
    BRepPrimAPIMakeCylinder::new(4., 6.).face().unwrap()
}

fn plane(origin: Point3, normal: Direction3, u: Direction3) -> Face {
    let frame = Axis3::new(origin, normal, u).unwrap();
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::from_position(frame), (-10., 10., -10., 10.))
        .unwrap()
        .face()
        .unwrap()
}

fn section(first: &Shape, second: &Shape) -> BRepAlgoAPISection {
    let mut section = BRepAlgoAPISection::from_shapes(first, second, None, false).unwrap();
    section.set_non_destructive(true);
    section.set_to_fill_history(false);
    section.build().unwrap();
    assert!(section.is_done() && !section.has_errors());
    section
}

fn edges(shape: &Shape) -> Vec<Shape> {
    Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)
        .unwrap()
        .collect()
}

fn assert_face_pair(section: &BRepAlgoAPISection, edge: &Shape, first: &Shape, second: &Shape) {
    assert!(
        section
            .has_ancestor_face_on1(edge)
            .unwrap()
            .unwrap()
            .is_same(first)
    );
    assert!(
        section
            .has_ancestor_face_on2(edge)
            .unwrap()
            .unwrap()
            .is_same(second)
    );
}

fn assert_no_face_pair(section: &BRepAlgoAPISection, edge: &Shape) {
    assert!(section.has_ancestor_face_on1(edge).unwrap().is_none());
    assert!(section.has_ancestor_face_on2(edge).unwrap().is_none());
}

fn only_edge(section: &BRepAlgoAPISection) -> Shape {
    let edges = edges(section.shape());
    let [edge] = edges.as_slice() else {
        panic!("expected exactly one section edge");
    };
    edge.clone()
}

#[test]
fn boundary_rim_has_both_ancestors_in_operand_order() {
    let cylinder = cylinder();
    let plane = plane(Point3::new(0., 0., 0.), Direction3::Z, Direction3::X);
    for (first, second) in [(&cylinder, &plane), (&plane, &cylinder)] {
        let section = section(first, second);
        let edges = edges(section.shape());
        let [edge] = edges.as_slice() else {
            panic!("expected exactly one circular rim")
        };
        let (carrier, _, _) = BRepTool::curve(&topods::edge(edge).unwrap()).unwrap();
        let circle = carrier
            .downcast_ref::<GeomCircle>()
            .unwrap()
            .circ()
            .unwrap();
        assert_eq!(circle.radius(), 4.);
        assert_eq!(circle.location(), Point3::new(0., 0., 0.));
        assert_face_pair(&section, edge, first, second);
        assert_eq!(section.section_edges().unwrap().len(), 0);
        assert_no_face_pair(&section, &Shape::default());
        assert_no_face_pair(&section, first);
    }
}

#[test]
fn pending_init_swap_preserves_completed_ancestry_until_rebuild() {
    let cylinder = cylinder();
    let plane = plane(Point3::new(0., 0., 0.), Direction3::Z, Direction3::X);
    let mut section = section(&cylinder, &plane);
    let old_rim = only_edge(&section);
    section.init1(&plane);
    assert_face_pair(&section, &old_rim, &cylinder, &plane);
    section.init2(&cylinder);
    assert_face_pair(&section, &old_rim, &cylinder, &plane);
    section.build().unwrap();
    assert!(section.is_done() && !section.has_errors());
    let rebuilt_rim = only_edge(&section);
    assert_face_pair(&section, &rebuilt_rim, &plane, &cylinder);

    section.set_arguments(&Default::default());
    section.build().unwrap();
    assert!(!section.is_done() && section.has_errors());
    assert_no_face_pair(&section, &rebuilt_rim);
}

struct CancelSectionBuild;

impl ProgressIndicatorHooks for CancelSectionBuild {
    fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}

    fn user_break(&self) -> bool {
        true
    }
}

#[test]
fn cancelled_ranged_rebuild_clears_completed_boundary_ancestry() {
    let cylinder = cylinder();
    let plane = plane(Point3::new(0., 0., 0.), Direction3::Z, Direction3::X);
    let mut section = section(&cylinder, &plane);
    let old_rim = only_edge(&section);
    assert_face_pair(&section, &old_rim, &cylinder, &plane);
    let indicator = ProgressIndicator::new(Box::new(CancelSectionBuild));
    let completion = section.build_with_progress(&indicator.start());
    assert!(matches!(completion, ProgressCompletion::Cancelled { .. }));
    assert_no_face_pair(&section, &old_rim);
    section.build().unwrap();
    assert_face_pair(&section, &only_edge(&section), &cylinder, &plane);
}

#[test]
fn a_coincident_edge_from_another_operation_has_no_ancestry() {
    let first_cylinder = cylinder();
    let second_cylinder = cylinder();
    let plane = plane(Point3::new(0., 0., 0.), Direction3::Z, Direction3::X);
    let result = section(&first_cylinder, &plane);
    let unrelated = only_edge(&section(&second_cylinder, &plane));
    assert!(!unrelated.is_same(&only_edge(&result)));
    assert_no_face_pair(&result, &unrelated);
}

#[test]
fn transverse_edges_keep_their_face_face_ancestors() {
    let cylinder = cylinder();
    let plane = plane(Point3::new(0., 0., 0.), Direction3::X, Direction3::Y);
    let section = section(&cylinder, &plane);
    let cuts = edges(section.shape());
    assert_eq!(cuts.len(), 2);
    let ff = section.section_edges().unwrap();
    assert_eq!(ff.len(), 2);
    for edge in cuts {
        assert!(ff.iter().any(|generated| edge.is_same(generated)));
        assert_face_pair(&section, &edge, &cylinder, &plane);
    }
}

#[test]
fn tangent_sweep_inherits_faces_from_the_cylinder_seam() {
    let cylinder = cylinder();
    let source =
        BRepBuilderAPIMakeEdge::from_points(Point3::new(4., -1., -1.), Point3::new(4., -1., 7.))
            .unwrap()
            .edge()
            .unwrap();
    let mut prism = BRepSweepPrism::new(&source, Vector3::new(0., 8., 0.), true, false).unwrap();
    let tool = prism.shape().unwrap();
    let section = section(&cylinder, &tool);
    let cuts = edges(section.shape());
    let [edge] = cuts.as_slice() else {
        panic!("expected one seam edge")
    };
    let (curve, first, last) = BRepTool::curve(&topods::edge(edge).unwrap()).unwrap();
    assert!(curve.downcast_ref::<GeomLine>().is_some());
    let a = curve.value(first).unwrap();
    let b = curve.value(last).unwrap();
    assert_eq!([a.x(), b.x()], [4., 4.]);
    assert!(a.y().abs() <= 4e-15 && b.y().abs() <= 4e-15);
    assert_eq!([a.z().min(b.z()), a.z().max(b.z())], [0., 6.]);
    assert!(section.section_edges().unwrap().is_empty());
    assert_face_pair(&section, edge, &cylinder, &tool);
}

#[test]
fn overlapping_opposite_faces_leave_the_boundary_pair_unresolved() {
    let cylinder = cylinder();
    let first = plane(Point3::new(-1., 0., 0.), Direction3::Z, Direction3::X);
    let second = plane(Point3::new(1., 0., 0.), Direction3::Z, Direction3::X);
    let builder = BRepBuilder::default();
    let mut support = Compound::default();
    builder.make_compound(&mut support);
    builder.add(&mut support, &first).unwrap();
    builder.add(&mut support, &second).unwrap();
    let mut section = section(&cylinder, &support);
    let cuts = edges(section.shape());
    let [edge] = cuts.as_slice() else {
        panic!("expected one shared circular rim")
    };
    let structure = section.ds_filler().unwrap().data_structure().unwrap();
    let common_faces = [structure.index(&first), structure.index(&second)];
    assert!(structure.pave_blocks_pool().iter().flatten().any(|block| {
        let Some(common) = structure.common_block(block) else {
            return false;
        };
        let representative = {
            let common = common.lock().unwrap();
            if !common_faces.iter().all(|id| common.faces().contains(id)) {
                return false;
            }
            Arc::clone(common.pave_block1().unwrap())
        };
        let image = representative.lock().unwrap().edge();
        edge.is_same(structure.shape(image).unwrap())
    }));
    assert_no_face_pair(&section, edge);
    builder.remove(&mut support, &second).unwrap();
    let mut remaining_faces = Explorer::new(&support, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    assert!(remaining_faces.current().is_same(&first));
    remaining_faces.next().unwrap();
    assert!(!remaining_faces.more());
    assert_no_face_pair(&section, edge);
    section.set_arguments(&[first.clone().into_shape()].into_iter().collect());
    section.set_tools(&[cylinder.clone().into_shape()].into_iter().collect());
    assert_no_face_pair(&section, edge);

    // The Boolean base entry must also freeze the newly completed section's roles.
    let completed =
        BRepAlgoAPIBooleanOperation::build_with_progress(&mut section, &ProgressRange::default());
    assert!(matches!(completed, ProgressCompletion::Completed));
    let rebuilt_rim = only_edge(&section);
    assert_face_pair(&section, &rebuilt_rim, &first, &cylinder);
}

#[test]
fn reused_filler_requires_the_same_completed_operand_roots() {
    let cylinder = cylinder();
    let plane = plane(Point3::new(0., 0., 0.), Direction3::Z, Direction3::X);
    let mut original = section(&cylinder, &plane);
    let filler = original.take_ds_filler().unwrap();
    let mut reversed =
        BRepAlgoAPISection::from_shapes(&plane, &cylinder, Some(filler), true).unwrap();
    let rim = only_edge(&reversed);
    assert_face_pair(&reversed, &rim, &plane, &cylinder);

    let builder = BRepBuilder::default();
    let mut substitute = Compound::default();
    builder.make_compound(&mut substitute);
    builder.add(&mut substitute, &plane).unwrap();
    let filler = reversed.take_ds_filler().unwrap();
    let mismatched =
        BRepAlgoAPISection::from_shapes(&cylinder, &substitute, Some(filler), true).unwrap();
    assert!(mismatched.is_done());
    assert_no_face_pair(&mismatched, &only_edge(&mismatched));
}
