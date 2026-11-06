//! DNamingTransformationDriver CollectShapes/BuildMap/LoadNamingDS, preserving native maps/tags.
use super::super::naming_history::{orientation_map, oriented_image};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{Compound, Shape, ShapeIterator},
    toptools::{MapOfShape, ShapeMapHasher},
};
use martensite_tkcaf::tnaming::{
    TNamingBuilder, TNamingEvolution, TNamingNamedShape, TNamingShapeShapeMap, TNamingTool,
};
use martensite_tkernel::{
    ncollection::DataMap,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::{TdfAttributeState, TdfLabel};
use martensite_tkmath::gp::Transform3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPITransform;

type CaseTags = DataMap<Shape, i32, ShapeMapHasher>;
struct NamedCollection {
    compound: Compound,
    shapes: MapOfShape,
    tags: CaseTags,
    access: TdfLabel,
    primitive: bool,
}
impl NamedCollection {
    fn new(source: &Shape, access: &TdfLabel, primitive: bool) -> Result<Self, Failure> {
        let mut compound = Compound::default();
        BRepBuilder::default().make_compound(&mut compound);
        BRepBuilder::default().add(&mut compound, source)?;
        let mut shapes = MapOfShape::default();
        shapes.add(source.clone());
        Ok(Self {
            compound,
            shapes,
            tags: CaseTags::default(),
            access: access.clone(),
            primitive,
        })
    }
    fn include_named(&mut self, shape: &Shape, tagged: bool) -> Result<(), Failure> {
        let named = TNamingTool::named_shape(shape, &self.access)?;
        if named.is_null() || !self.shapes.add(shape.clone()) {
            return Ok(());
        }
        BRepBuilder::default().add(&mut self.compound, shape)?;
        if self.primitive && tagged {
            self.tags.bind(shape.clone(), named.label().tag());
        }
        Ok(())
    }
    fn include_kind(&mut self, source: &Shape, kind: ShapeEnum) -> Result<(), Failure> {
        for shape in Explorer::new(source, kind, ShapeEnum::Shape)? {
            self.include_named(&shape, true)?;
        }
        Ok(())
    }
    fn collect(&mut self, source: &Shape) -> Result<(), Failure> {
        match source.shape_type()? {
            ShapeEnum::Compound => self.collect_compound(source),
            ShapeEnum::CompSolid | ShapeEnum::Solid | ShapeEnum::Shell => self.collect_kinds(
                source,
                &[ShapeEnum::Face, ShapeEnum::Edge, ShapeEnum::Vertex],
            ),
            ShapeEnum::Face => {
                self.include_named(source, false)?;
                self.collect_kinds(source, &[ShapeEnum::Edge, ShapeEnum::Vertex])
            }
            ShapeEnum::Wire => self.collect_kinds(source, &[ShapeEnum::Edge, ShapeEnum::Vertex]),
            ShapeEnum::Edge => {
                self.include_named(source, false)?;
                self.include_kind(source, ShapeEnum::Vertex)
            }
            ShapeEnum::Vertex => self.include_named(source, false),
            _ => Ok(()),
        }
    }
    fn collect_compound(&mut self, source: &Shape) -> Result<(), Failure> {
        for child in ShapeIterator::new(source, true, true) {
            self.collect(&child)?;
        }
        Ok(())
    }
    fn collect_kinds(&mut self, source: &Shape, kinds: &[ShapeEnum]) -> Result<(), Failure> {
        for kind in kinds {
            self.include_kind(source, *kind)?;
        }
        Ok(())
    }
    fn transform_images(&self, transform: Transform3) -> Result<TNamingShapeShapeMap, Failure> {
        let transformer =
            BRepBuilderAPITransform::with_shape(&self.compound, transform, false, false)?;
        let mut images = TNamingShapeShapeMap::default();
        for shape in self.shapes.iter() {
            if !shape.is_null() {
                images.bind(shape.clone(), transformer.modified_shape(shape)?);
            }
        }
        Ok(images)
    }
}

struct NamingRows {
    result: TdfLabel,
    tags: CaseTags,
    next_tag: i32,
    grouped: [Option<TNamingBuilder>; 3],
}
impl NamingRows {
    fn new(result: &TdfLabel, tags: CaseTags) -> Self {
        let next_tag = tags.items().map(|(_, tag)| *tag).max().unwrap_or(0).max(0);
        Self {
            result: result.clone(),
            tags,
            next_tag,
            grouped: [None, None, None],
        }
    }
    fn primitive_row(&mut self, old: &Shape, new: &Shape) -> Result<(), Failure> {
        let tag = match self.tags.seek(old) {
            Some(tag) => *tag,
            None => {
                self.next_tag += 1;
                self.next_tag
            }
        };
        TNamingBuilder::new(&self.result.find_child(tag, true)?)?.modify(old, new)
    }
    fn grouped_row(&mut self, old: &Shape, new: &Shape) -> Result<(), Failure> {
        let index = match old.shape_type()? {
            ShapeEnum::Face => 0,
            ShapeEnum::Edge => 1,
            ShapeEnum::Vertex => 2,
            _ => return Ok(()),
        };
        if self.grouped[index].is_none() {
            self.grouped[index] = Some(TNamingBuilder::new(
                &self.result.find_child(index as i32 + 1, true)?,
            )?);
        }
        self.grouped[index].as_ref().unwrap().modify(old, new)
    }
}

fn load_images(
    result: &TdfLabel,
    source: &Shape,
    mut images: TNamingShapeShapeMap,
) -> Result<(Shape, TNamingShapeShapeMap), Failure> {
    let image = images.seek(source).cloned().unwrap_or_default();
    if !image.is_null() {
        TNamingBuilder::new(result)?.modify(source, &image)?;
        images.unbind(source);
    }
    Ok((image, images))
}

/// Transform actual named shapes together and preserve native primitive/grouped naming.
/// Example: `load_transformed_names(&result, &source, transform)?`.
pub fn load_transformed_names(
    result: &TdfLabel,
    named: &Handle<TNamingNamedShape>,
    transform: Transform3,
) -> Result<(), Failure> {
    if named.is_null() || named.is_empty() {
        return Ok(());
    }
    let source = named.get().unwrap().get()?;
    if source.is_null() {
        return Ok(());
    }
    let primitive = named.evolution() == TNamingEvolution::Primitive;
    let mut collection = NamedCollection::new(&source, &named.label(), primitive)?;
    collection.collect(&source)?;
    let images = collection.transform_images(transform)?;
    let (result_shape, images) = load_images(result, &source, images)?;
    write_subshape_names(result, &result_shape, images, collection.tags, primitive)
}

fn write_subshape_names(
    result: &TdfLabel,
    shape: &Shape,
    images: TNamingShapeShapeMap,
    tags: CaseTags,
    primitive: bool,
) -> Result<(), Failure> {
    let orientations = orientation_map(
        shape,
        &[ShapeEnum::Face, ShapeEnum::Edge, ShapeEnum::Vertex],
    )?;
    let mut rows = NamingRows::new(result, tags);
    for (old, transformed) in images.items() {
        let new = oriented_image(transformed, &orientations);
        if primitive {
            rows.primitive_row(old, &new)?;
        } else {
            rows.grouped_row(old, &new)?;
        }
    }
    Ok(())
}
