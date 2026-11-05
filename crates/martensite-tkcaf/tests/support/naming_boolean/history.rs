//! Source-bound Boolean result and spherical section naming, with shared image loaders.
use super::super::naming_history::{
    load_deleted, load_generated, load_modified, orientation_map, oriented_image,
};
use martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation;
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorSurface,
    topexp::{Explorer, vertices},
    topods::{self, Shape, ShapeIterator},
    toptools::MapOfShape,
};
use martensite_tkcaf::tnaming::{TNamingBuilder, TNamingShapeShapeMap};
use martensite_tkernel::{ncollection::List, standard::Failure};
use martensite_tkg3d::{adaptor3d::Adaptor3dSurface, top_abs::ShapeEnum};
use martensite_tklcaf::tdf::{TdfLabel, TdfTagSource};
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeShape, BRepBuilderAPIMakeShapeBase,
};

struct HistoryMaker<'a> {
    operation: &'a mut BRepAlgoAPIBooleanOperation,
    images: List<Shape>,
}
impl BRepBuilderAPIMakeShape for HistoryMaker<'_> {
    fn make_shape_base(&self) -> &BRepBuilderAPIMakeShapeBase {
        self.operation.make_shape_base()
    }
    fn make_shape_base_mut(&mut self) -> &mut BRepBuilderAPIMakeShapeBase {
        self.operation.make_shape_base_mut()
    }
    fn generated(&mut self, shape: &Shape) -> Result<&List<Shape>, Failure> {
        self.images = self.operation.generated(shape)?.into_iter().collect();
        Ok(&self.images)
    }
    fn modified(&mut self, shape: &Shape) -> Result<&List<Shape>, Failure> {
        self.images = self.operation.modified(shape)?.into_iter().collect();
        Ok(&self.images)
    }
    fn is_deleted(&mut self, shape: &Shape) -> Result<bool, Failure> {
        self.operation.is_deleted(shape)
    }
}
fn load_result(
    result: &TdfLabel,
    operation: &mut BRepAlgoAPIBooleanOperation,
) -> Result<Shape, Failure> {
    TdfTagSource::set(result)?.set_value(0)?;
    let builder = TNamingBuilder::new(result)?;
    let mut shape = operation.shape().clone();
    if shape.shape_type()? == ShapeEnum::Compound && shape.nb_children() == 1 {
        shape = Iterator::next(&mut ShapeIterator::new(&shape, true, true)).unwrap();
    }
    let source = operation.shape1().cloned().unwrap_or_default();
    if source.is_null() {
        builder.generated(&shape)?;
    } else {
        builder.modify(&source, &shape)?;
    }
    Ok(shape)
}
fn effective_type(shape: &Shape) -> Result<ShapeEnum, Failure> {
    let kind = shape.shape_type()?;
    if !matches!(kind, ShapeEnum::Compound | ShapeEnum::CompSolid) {
        return Ok(kind);
    }
    let mut children = ShapeIterator::new(shape, true, true);
    let Some(first) = Iterator::next(&mut children) else {
        return Ok(kind);
    };
    let first_type = effective_type(&first)?;
    if first_type == ShapeEnum::Compound {
        return Ok(first_type);
    }
    for child in children {
        if effective_type(&child)? != first_type {
            return Ok(ShapeEnum::Compound);
        }
    }
    Ok(first_type)
}
fn spherical_face(shape: &Shape) -> Result<bool, Failure> {
    let surface = BRepAdaptorSurface::with_face(&topods::face(shape)?, true)?;
    Ok(surface.get_type()? == SurfaceType::Sphere)
}
fn has_sphere(shape: &Shape, kind: ShapeEnum) -> Result<bool, Failure> {
    if kind == ShapeEnum::Face {
        return spherical_face(shape);
    }
    for face in Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)? {
        if spherical_face(&face)? {
            return Ok(true);
        }
    }
    Ok(false)
}
fn spherical_case(source: &Shape, tool: &Shape) -> Result<bool, Failure> {
    let source_type = effective_type(source)?;
    if source_type == ShapeEnum::Compound || source_type as i32 > ShapeEnum::Face as i32 {
        return Ok(false);
    }
    let tool_type = effective_type(tool)?;
    if tool_type == ShapeEnum::Compound || tool_type as i32 > ShapeEnum::Face as i32 {
        return Ok(false);
    }
    Ok(has_sphere(source, source_type)? || has_sphere(tool, tool_type)?)
}
fn section_vertices(result: &TdfLabel, edge: &Shape, seen: &mut MapOfShape) -> Result<(), Failure> {
    let (first, last) = vertices(&topods::edge(edge)?, true);
    for vertex in [first, last] {
        if seen.add((*vertex).clone()) {
            TNamingBuilder::new(&result.new_child()?)?.generated(&vertex)?;
        }
    }
    Ok(())
}
fn sphere_sections(
    result: &TdfLabel,
    operation: &mut BRepAlgoAPIBooleanOperation,
) -> Result<(), Failure> {
    let shape = operation.shape().clone();
    let orientations = orientation_map(&shape, &[ShapeEnum::Face, ShapeEnum::Edge])?;
    let sections = operation.section_edges()?;
    let vertices_needed = !sections.is_empty() && sections.len() < 3;
    let mut seen = MapOfShape::default();
    for edge in sections.iter() {
        let image = oriented_image(edge, &orientations);
        TNamingBuilder::new(&result.new_child()?)?.generated(&image)?;
        if vertices_needed {
            section_vertices(result, &image, &mut seen)?;
        }
    }
    Ok(())
}
fn section_names(
    result: &TdfLabel,
    operation: &mut BRepAlgoAPIBooleanOperation,
    source: &Shape,
    tool: &Shape,
    orientations: &TNamingShapeShapeMap,
) -> Result<(), Failure> {
    let builder = TNamingBuilder::new(&result.new_child()?)?;
    let mut maker = HistoryMaker {
        operation,
        images: List::default(),
    };
    load_generated(&mut maker, source, ShapeEnum::Face, &builder, orientations)?;
    load_generated(&mut maker, tool, ShapeEnum::Face, &builder, orientations)
}
fn deleted_names(
    result: &TdfLabel,
    maker: &mut HistoryMaker<'_>,
    source: &Shape,
    tool: &Shape,
) -> Result<(), Failure> {
    if !maker.operation.has_deleted() {
        return Ok(());
    }
    let deleted = TNamingBuilder::new(&result.new_child()?)?;
    load_deleted(maker, source, ShapeEnum::Face, &deleted)?;
    load_deleted(maker, tool, ShapeEnum::Face, &deleted)
}
fn solid_names(
    result: &TdfLabel,
    operation: &mut BRepAlgoAPIBooleanOperation,
    source: &Shape,
    tool: &Shape,
    orientations: &TNamingShapeShapeMap,
) -> Result<(), Failure> {
    let modified = TNamingBuilder::new(&result.new_child()?)?;
    let mut maker = HistoryMaker {
        operation,
        images: List::default(),
    };
    load_modified(&mut maker, source, ShapeEnum::Face, &modified, orientations)?;
    load_modified(&mut maker, tool, ShapeEnum::Face, &modified, orientations)?;
    deleted_names(result, &mut maker, source, tool)?;
    if spherical_case(source, tool)? {
        sphere_sections(result, maker.operation)?;
    }
    Ok(())
}
fn naming_orientations(shape: &Shape, section: bool) -> Result<TNamingShapeShapeMap, Failure> {
    let kind = if section {
        ShapeEnum::Edge
    } else {
        ShapeEnum::Face
    };
    orientation_map(shape, &[kind])
}
/// Preserve DNaming root/tag reset and all operation-specific history; `load_names(...)`.
pub(super) fn load_names(
    result: &TdfLabel,
    operation: &mut BRepAlgoAPIBooleanOperation,
    section: bool,
) -> Result<(), Failure> {
    let shape = operation.shape().clone();
    if shape.is_null() {
        return Ok(());
    }
    let source = operation.shape1().cloned().unwrap_or_default();
    let tool = operation.shape2().cloned().unwrap_or_default();
    load_result(result, operation)?;
    let orientations = naming_orientations(&shape, section)?;
    if section {
        return section_names(result, operation, &source, &tool, &orientations);
    }
    solid_names(result, operation, &source, &tool, &orientations)
}

#[cfg(test)]
mod checks {
    use super::*;
    use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeSphere};
    #[test]
    fn sphere_workaround_requires_a_real_spherical_face_and_supported_body_kinds() {
        let mut sphere = BRepPrimAPIMakeSphere::new(2.0).unwrap();
        let sphere = sphere.shape().unwrap().clone();
        let mut cuboid = BRepPrimAPIMakeBox::from_sizes(2.0, 3.0, 4.0).unwrap();
        let cuboid = cuboid.shape().unwrap().clone();
        assert!(spherical_case(&sphere, &cuboid).unwrap());
        assert!(spherical_case(&cuboid, &sphere).unwrap());
        assert!(!spherical_case(&cuboid, &cuboid).unwrap());
        let edge =
            Iterator::next(&mut Explorer::new(&sphere, ShapeEnum::Edge, ShapeEnum::Shape).unwrap())
                .unwrap();
        assert!(!spherical_case(&edge, &sphere).unwrap());
        assert_eq!(effective_type(&sphere).unwrap(), ShapeEnum::Solid);
    }
    fn assert_seam_vertex(shape: &Shape) {
        // Equal radius2 spheres at x0/x2 intersect at x1; their seam y0 cuts z=±sqrt3.
        let point =
            martensite_tkbrep::brep::BRepTool::pnt(&topods::vertex(shape).unwrap()).unwrap();
        assert!((point.x() - 1.0).abs() < 1e-12);
        assert!(point.y().abs() < 1e-12);
        assert!((point.z().abs() - 3.0_f64.sqrt()).abs() < 1e-12);
    }
    fn increment_primitive(counts: &mut [usize; 2], shape: &Shape) {
        match shape.shape_type().unwrap() {
            ShapeEnum::Edge => counts[0] += 1,
            ShapeEnum::Vertex => {
                assert_seam_vertex(shape);
                counts[1] += 1;
            }
            _ => panic!("spherical naming child must be EDGE or VERTEX"),
        }
    }
    fn is_primitive(names: &martensite_tkcaf::tnaming::TNamingIterator) -> bool {
        names.evolution().unwrap() == martensite_tkcaf::tnaming::TNamingEvolution::Primitive
    }
    fn assert_primitive_counts(result: &TdfLabel) {
        let mut counts = [0, 0];
        let mut children = martensite_tklcaf::tdf::TdfChildIterator::new(result, false).unwrap();
        while children.more() {
            let label = children.value();
            children.next();
            let names = martensite_tkcaf::tnaming::TNamingIterator::from_label(&label).unwrap();
            if !names.more() {
                continue;
            }
            if !is_primitive(&names) {
                continue;
            }
            increment_primitive(&mut counts, &names.new_shape().unwrap());
        }
        assert_eq!(counts, [2, 2]);
    }
    #[test]
    fn spherical_section_edge_and_unique_vertices_are_named_from_actual_intersection() {
        let mut first = BRepPrimAPIMakeSphere::new(2.0).unwrap();
        let mut second = BRepPrimAPIMakeSphere::at_center(
            martensite_tkmath::gp::Point3::new(2.0, 0.0, 0.0),
            2.0,
        )
        .unwrap();
        let mut operation = martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse::from_shapes(
            first.shape().unwrap(),
            second.shape().unwrap(),
        )
        .unwrap();
        assert!(operation.is_done());
        let sections = operation.section_edges().unwrap();
        assert_eq!(sections.len(), 2);
        let document = super::super::super::document();
        let result = document.main().unwrap();
        load_names(&result, &mut operation, false).unwrap();
        assert_primitive_counts(&result);
    }
}
