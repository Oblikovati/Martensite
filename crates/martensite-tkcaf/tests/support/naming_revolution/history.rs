//! Native revolution seam, cap-component and nondegenerate dangle-edge naming.
use super::{
    super::naming_sweep_history::{cap_names, sweep_history},
    topology::{has_dangle, open_endpoints},
};
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::Explorer,
    topods::{self, Shape, ShapeIterator, Vertex},
    toptools::MapOfShape,
};
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingShapeShapeMap};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeRevol, BRepPrimAPIMakeSweep};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
fn first_seam(maker: &mut BRepPrimAPIMakeRevol, root: &Shape) -> Result<Option<Shape>, Failure> {
    let generated: Vec<Shape> = maker.generated(root)?.iter().cloned().collect();
    for face in generated {
        if face.shape_type()? != ShapeEnum::Face || root.is_same(&face) {
            continue;
        }
        for edge in Explorer::new(&face, ShapeEnum::Edge, ShapeEnum::Shape)? {
            if BRepTool::is_closed_on_face(&topods::edge(&edge)?, &topods::face(&face)?)? {
                return Ok(Some(edge));
            }
        }
    }
    Ok(None)
}
fn seam_names(
    result: &TdfLabel,
    maker: &mut BRepPrimAPIMakeRevol,
    basis: &Shape,
) -> Result<(), Failure> {
    let mut seen = MapOfShape::default();
    for edge in Explorer::new(basis, ShapeEnum::Edge, ShapeEnum::Shape)? {
        if !seen.add(edge.clone()) {
            continue;
        }
        if let Some(seam) = first_seam(maker, &edge)? {
            TNamingBuilder::new(&result.new_child()?)?.generated(&seam)?;
        }
    }
    Ok(())
}
fn revolution_cap(
    result: &TdfLabel,
    shape: &Shape,
    orientations: &TNamingShapeShapeMap,
) -> Result<(), Failure> {
    cap_names(result, shape, orientations)?;
    if shape.is_null() || matches!(shape.shape_type()?, ShapeEnum::Face | ShapeEnum::Compound) {
        return Ok(());
    }
    for child in ShapeIterator::new(shape, true, true) {
        TNamingBuilder::new(&result.new_child()?)?.generated(&child)?;
    }
    Ok(())
}
fn vertex_generations(
    result: &TdfLabel,
    maker: &mut BRepPrimAPIMakeRevol,
    vertex: &Shape,
) -> Result<(), Failure> {
    let generated: Vec<Shape> = maker.generated(vertex)?.iter().cloned().collect();
    for edge in generated {
        if !BRepTool::degenerated(&topods::edge(&edge)?)? {
            TNamingBuilder::new(&result.new_child()?)?.generated_from(vertex, &edge)?;
        }
    }
    Ok(())
}
fn dangle_names(
    result: &TdfLabel,
    maker: &mut BRepPrimAPIMakeRevol,
    dangling: bool,
    endpoints: Option<[Vertex; 2]>,
) -> Result<(), Failure> {
    if !dangling {
        return Ok(());
    }
    let Some([first, last]) = endpoints else {
        return Ok(());
    };
    vertex_generations(result, maker, &first)?;
    vertex_generations(result, maker, &last)
}
/// Complete native root/lateral/seam-or-caps/dangle history; `load_names(...)`.
pub(super) fn load_names(
    result: &TdfLabel,
    maker: &mut BRepPrimAPIMakeRevol,
    basis: &Shape,
    context: &Shape,
) -> Result<(), Failure> {
    let orientations = sweep_history(result, maker, basis, context)?;
    let start = maker.first_shape()?;
    let end = maker.last_shape()?;
    let full = !start.is_null() && !end.is_null() && start == end;
    let dangling = has_dangle(maker.shape()?)?;
    let endpoints = open_endpoints(basis)?;
    if full {
        seam_names(result, maker, basis)?;
    } else {
        revolution_cap(result, &start, &orientations)?;
        revolution_cap(result, &end, &orientations)?;
    }
    dangle_names(result, maker, dangling, endpoints)
}

#[cfg(test)]
mod checks {
    use super::*;
    use martensite_tkmath::gp::{Axis1, Direction3, Point3};
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakePolygon;
    fn basis_edge() -> Shape {
        let polygon = BRepBuilderAPIMakePolygon::from_two_points(
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 4.0),
        )
        .unwrap();
        (**polygon.edge()).clone()
    }
    fn maker(basis: &Shape, full: bool) -> BRepPrimAPIMakeRevol {
        let axis = Axis1::new(
            Point3::new(0.0, 0.0, 0.0),
            Direction3::new(0.0, 0.0, 1.0).unwrap(),
        );
        if full {
            return BRepPrimAPIMakeRevol::full(basis, axis, true).unwrap();
        }
        BRepPrimAPIMakeRevol::new(basis, axis, std::f64::consts::FRAC_PI_2, true).unwrap()
    }
    fn child_names(
        result: &TdfLabel,
    ) -> Vec<(martensite_tkcaf::tnaming::TNamingEvolution, ShapeEnum)> {
        let mut children = martensite_tklcaf::tdf::TdfChildIterator::new(result, false).unwrap();
        let mut names = Vec::new();
        while children.more() {
            let label = children.value();
            children.next();
            let record = martensite_tkcaf::tnaming::TNamingIterator::from_label(&label).unwrap();
            assert!(record.more());
            names.push((
                record.evolution().unwrap(),
                record.new_shape().unwrap().shape_type().unwrap(),
            ));
        }
        names
    }
    #[test]
    fn full_open_edge_names_one_seam_and_both_nondegenerate_boundary_edges() {
        use martensite_tkcaf::tnaming::TNamingEvolution::{Generated, Primitive};
        let basis = basis_edge();
        let mut maker = maker(&basis, true);
        let document = super::super::super::document();
        let result = document.main().unwrap();
        load_names(&result, &mut maker, &basis, &basis).unwrap();
        assert_eq!(
            child_names(&result),
            [
                (Generated, ShapeEnum::Face),
                (Primitive, ShapeEnum::Edge),
                (Generated, ShapeEnum::Edge),
                (Generated, ShapeEnum::Edge)
            ]
        );
    }
    fn partial_edge_history() -> Vec<(martensite_tkcaf::tnaming::TNamingEvolution, ShapeEnum)> {
        use martensite_tkcaf::tnaming::TNamingEvolution::{Generated, Primitive};
        vec![
            (Generated, ShapeEnum::Face),
            (Primitive, ShapeEnum::Edge),
            (Primitive, ShapeEnum::Vertex),
            (Primitive, ShapeEnum::Vertex),
            (Primitive, ShapeEnum::Edge),
            (Primitive, ShapeEnum::Vertex),
            (Primitive, ShapeEnum::Vertex),
            (Generated, ShapeEnum::Edge),
            (Generated, ShapeEnum::Edge),
        ]
    }
    #[test]
    fn partial_open_edge_names_all_cap_vertices_and_both_boundary_generations() {
        let basis = basis_edge();
        let mut maker = maker(&basis, false);
        let document = super::super::super::document();
        let result = document.main().unwrap();
        load_names(&result, &mut maker, &basis, &basis).unwrap();
        assert_eq!(child_names(&result), partial_edge_history());
    }
    fn wire_history(full: bool) -> Vec<(martensite_tkcaf::tnaming::TNamingEvolution, ShapeEnum)> {
        use martensite_tkcaf::tnaming::TNamingEvolution::{Generated, Primitive};
        if full {
            return vec![
                (Generated, ShapeEnum::Face),
                (Primitive, ShapeEnum::Edge),
                (Generated, ShapeEnum::Edge),
                (Generated, ShapeEnum::Edge),
            ];
        }
        vec![
            (Generated, ShapeEnum::Face),
            (Primitive, ShapeEnum::Wire),
            (Primitive, ShapeEnum::Edge),
            (Primitive, ShapeEnum::Wire),
            (Primitive, ShapeEnum::Edge),
            (Generated, ShapeEnum::Edge),
            (Generated, ShapeEnum::Edge),
        ]
    }
    #[test]
    fn open_wire_names_all_cap_edges_or_seam_and_both_endpoint_generations() {
        for full in [false, true] {
            let edge = basis_edge();
            let mut wire = martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeWire::new();
            wire.add_edge(&topods::edge(&edge).unwrap()).unwrap();
            let basis = (*wire.wire().unwrap()).clone();
            let mut maker = maker(&basis, full);
            let document = super::super::super::document();
            let result = document.main().unwrap();
            load_names(&result, &mut maker, &basis, &basis).unwrap();
            assert_eq!(child_names(&result), wire_history(full));
        }
    }
}
