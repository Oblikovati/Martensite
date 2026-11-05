//! Native Line3D result edge tags and endpoint tags, including empty stale-name clearing.
use martensite_tkbrep::{
    brep::BRepTool,
    topexp::{self, Explorer},
    topods::{self, Shape, Vertex, Wire},
};
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingNamedShape};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
fn matching_edge(wire: &Wire, first: &Shape, last: &Shape) -> Result<Shape, Failure> {
    let first_point = BRepTool::pnt(&topods::vertex(first)?)?;
    let last_point = BRepTool::pnt(&topods::vertex(last)?)?;
    for shape in Explorer::new(wire, ShapeEnum::Edge, ShapeEnum::Shape)? {
        let (start, end) = topexp::vertices(&topods::edge(&shape)?, false);
        if !first_point.is_equal(&BRepTool::pnt(&start)?, precision::confusion()) {
            continue;
        }
        if last_point.is_equal(&BRepTool::pnt(&end)?, precision::confusion()) {
            return Ok(shape);
        }
    }
    Ok(Shape::default())
}
fn record_or_clear(label: &TdfLabel, shape: &Shape) -> Result<(), Failure> {
    if !shape.is_null() {
        return TNamingBuilder::new(label)?.generated(shape);
    }
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    if label.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)? {
        TNamingBuilder::new(label)?;
    }
    Ok(())
}
fn closure_names(
    wire: &Wire,
    vertices: &[Shape],
    closed: bool,
) -> Result<(Shape, Vertex, Vertex), Failure> {
    if !closed {
        let (first, last) = topexp::wire_vertices(wire);
        return Ok((Shape::default(), first, last));
    }
    let edge = matching_edge(wire, vertices.last().unwrap(), &vertices[0])?;
    if edge.is_null() {
        return Ok((edge, Vertex::default(), Vertex::default()));
    }
    let (last, first) = topexp::vertices(&topods::edge(&edge)?, false);
    Ok((edge, first, last))
}
pub(super) fn load_line_names(
    result: &TdfLabel,
    wire: &Wire,
    vertices: &[Shape],
    closed: bool,
) -> Result<(), Failure> {
    if wire.is_null() {
        return Ok(());
    }
    TNamingBuilder::new(result)?.generated(wire)?;
    if vertices.len() < 2 {
        return Ok(());
    }
    let (edges, first, last) = edge_names(wire, vertices, closed)?;
    for (index, edge) in edges.iter().enumerate() {
        record_or_clear(&result.find_child(index as i32 + 1, true)?, edge)?;
    }
    let length = vertices.len() as i32;
    record_or_clear(&result.find_child(length + 1, true)?, &first)?;
    record_or_clear(&result.find_child(length + 2, true)?, &last)
}
fn edge_names(
    wire: &Wire,
    vertices: &[Shape],
    closed: bool,
) -> Result<(Vec<Shape>, Vertex, Vertex), Failure> {
    let mut edges = Vec::with_capacity(vertices.len());
    for pair in vertices.windows(2) {
        edges.push(matching_edge(wire, &pair[0], &pair[1])?);
    }
    let (edge, first, last) = closure_names(wire, vertices, closed)?;
    edges.push(edge);
    Ok((edges, first, last))
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkernel::tcollection::ExtendedString;
    use martensite_tklcaf::tdoc_std::TDocStdDocument;
    use martensite_tkmath::gp::Point3;
    use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeWire};
    #[test]
    fn null_wire_history_loader_leaves_result_label_without_named_attribute() {
        let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
        let result = document.main().unwrap();
        load_line_names(&result, &Wire::default(), &[], false).unwrap();
        let mut named: Handle<TNamingNamedShape> = Handle::default();
        assert!(
            !result
                .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
                .unwrap()
        );
    }
    #[test]
    fn short_history_array_names_only_existing_real_wire_root() {
        let document = TDocStdDocument::new(&ExtendedString::new("BinOcaf")).unwrap();
        let result = document.main().unwrap();
        let edge = BRepBuilderAPIMakeEdge::from_points(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        )
        .unwrap()
        .edge()
        .unwrap();
        let wire = BRepBuilderAPIMakeWire::from_edge(&edge)
            .unwrap()
            .wire()
            .unwrap();
        load_line_names(&result, &wire, &[], false).unwrap();
        let mut named: Handle<TNamingNamedShape> = Handle::default();
        assert!(
            result
                .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
                .unwrap()
        );
        assert!(named.get().unwrap().get().unwrap().is_same(&wire));
        assert!(
            !martensite_tklcaf::tdf::TdfChildIterator::new(&result, false)
                .unwrap()
                .more()
        );
    }
}
