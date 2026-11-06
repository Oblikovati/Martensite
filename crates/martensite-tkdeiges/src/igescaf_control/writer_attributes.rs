//! The attributes of a document on the entities of its shapes: colors, layers and names.
use super::{IgesCafControlWriter, encode_color};
use crate::iges_basic::IgesBasicName;
use crate::iges_data::{IgesDataIgesEntity, iges_live};
use crate::iges_graph::IgesGraphColor;
use crate::iges_solid::IgesSolidFace;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::IndexedDataMap;
use martensite_tkernel::quantity::{Color, ColorRgba, ColorSpace, NamedColor};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkernel::tcollection::{ExtendedString, HAsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString, TDataStdName};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocLayerTool, XCAFDocShapeTool};
use martensite_tkxcaf::xcaf_prs::{XCAFPrs, XCAFPrsStyle, XCAFPrsStyleMap};
use martensite_tkxsbase::transfer::{TransferFinderProcess, TransferTransientListBinder};
use martensite_tkxsbase::transfer_brep::TransferBRep;
use std::collections::BTreeMap;

const WHERE: &str = "IgesCafControlWriter";

/// The entity of a shape in the process, if it has one.
fn entity_of_shape(
    process: &Handle<TransferFinderProcess>,
    shape: &Shape,
) -> Result<Handle<IgesDataIgesEntity>, Failure> {
    let mapper = TransferBRep::shape_mapper(process, shape)?;
    let mut entity = Handle::default();
    let _ = iges_live(process, WHERE)?.find_typed_transient_as(
        &mapper.cast(),
        IgesDataIgesEntity::static_type(),
        &mut entity,
    );
    Ok(entity)
}

/// The shapes with a name; a shape named again takes the later name.
type NamedShapes = IndexedDataMap<Shape, ExtendedString, ShapeMapHasher>;

fn bind_name(names: &mut NamedShapes, shape: Shape, name: ExtendedString) {
    match names.change_seek(&shape) {
        Some(known) => *known = name,
        None => {
            names.add(shape, name);
        }
    }
}

/// The name of a label, if it has one.
fn name_of_label(label: &TdfLabel) -> Result<Option<ExtendedString>, Failure> {
    let mut attribute = Handle::<TDataStdName>::default();
    if !label.find_attribute_as(&TDataStdName::get_id(), &mut attribute)? {
        return Ok(None);
    }
    Ok(attribute.get().map(TDataStdGenericExtString::get))
}

/// The text of a document string.
fn text_of(name: &ExtendedString) -> String {
    String::from_utf16_lossy(name.as_utf16())
}

/// The style of a shape: what it inherits, overlaid with its own.
fn overlaid_style(inherit: &XCAFPrsStyle, own: &XCAFPrsStyle) -> XCAFPrsStyle {
    let mut style = inherit.clone();
    if own.is_set_color_curv() {
        style.set_color_curv(own.get_color_curv());
    }
    if own.is_set_color_surf() {
        style.set_color_surf(own.get_color_surf());
    }
    style.set_material(own.material());
    style
}

/// The color a style gives a face or a solid: its own, or the base color of its material.
fn surface_color(style: &XCAFPrsStyle) -> Option<Color> {
    if style.is_set_color_surf() {
        return Some(style.get_color_surf());
    }
    let material = style.material().get()?;
    (!material.is_empty()).then(|| *material.base_color().rgb())
}

/// Puts a color on an entity, and on the surface of a face.
fn color_entity(
    entity: &Handle<IgesDataIgesEntity>,
    color_entity: &Handle<IgesGraphColor>,
    rank: i32,
) -> Result<(), Failure> {
    let live = iges_live(entity, WHERE)?;
    live.init_color(&color_entity.cast(), rank);
    let face = entity.cast::<IgesSolidFace>();
    if let Some(body) = face.get()
        && let Some(surface) = body.surface().get()
    {
        surface.init_color(&color_entity.cast(), rank);
    }
    Ok(())
}

/// The colors of a walk over a shape: the settings, and the color entities made so far.
struct ColorWalk<'a> {
    settings: &'a XCAFPrsStyleMap,
    made: BTreeMap<[u64; 3], Handle<IgesGraphColor>>,
    seen: IndexedDataMap<Shape, (), ShapeMapHasher>,
}

impl IgesCafControlWriter {
    /// Colors the entities of the shapes of labels; false without a label;
    /// `writer.write_attributes(&labels)?`.
    ///
    /// # Errors
    /// A failure of the document or of the process.
    pub fn write_attributes(&mut self, labels: &[TdfLabel]) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        for label in labels {
            let mut settings = XCAFPrsStyleMap::default();
            let white = ColorRgba::from_rgb(Color::named(NamedColor::WHITE), 1.0);
            XCAFPrs::collect_style_settings(label, &Location::default(), &mut settings, &white)?;
            if settings.is_empty() {
                continue;
            }
            let mut shape = Shape::default();
            if !XCAFDocShapeTool::get_shape(label, &mut shape)? {
                continue;
            }
            let mut walk = ColorWalk {
                settings: &settings,
                made: BTreeMap::new(),
                seen: IndexedDataMap::default(),
            };
            self.make_colors(&shape, &mut walk, &XCAFPrsStyle::default())?;
        }
        Ok(true)
    }
    /// The color entity of a color without a number, one per color of the walk.
    fn color_entity_of(
        &mut self,
        color: &Color,
        walk: &mut ColorWalk<'_>,
    ) -> Result<Handle<IgesGraphColor>, Failure> {
        let [red, green, blue] = color.values(ColorSpace::Srgb);
        let key = [red.to_bits(), green.to_bits(), blue.to_bits()];
        if let Some(known) = walk.made.get(&key) {
            return Ok(known.clone());
        }
        let entity = Handle::new(IgesGraphColor::new());
        let name = Handle::new(HAsciiString::new(Color::string_name(color.name())));
        iges_live(&entity, WHERE)?.init(red * 100.0, green * 100.0, blue * 100.0, &name);
        let _ = self.base.add_entity(&entity.cast())?;
        walk.made.insert(key, entity.clone());
        Ok(entity)
    }
    /// Colors the entity of a shape or the entities of its binder.
    fn color_shape_entities(
        &self,
        shape: &Shape,
        color: &Handle<IgesGraphColor>,
        rank: i32,
    ) -> Result<(), Failure> {
        let process = self.base.transfer_process();
        let mut entity = entity_of_shape(&process, shape)?;
        if entity.is_null() {
            entity = entity_of_shape(&process, &shape.located(&Location::default(), false)?)?;
        }
        if entity.is_some() {
            return color_entity(&entity, color, rank);
        }
        let mapper = TransferBRep::shape_mapper(&process, shape)?;
        let binder = iges_live(&process, WHERE)?
            .find(&mapper.cast())
            .cast::<TransferTransientListBinder>();
        if let Some(list) = binder.get() {
            for number in 1..=list.nb_transients() {
                let item: Handle<IgesDataIgesEntity> = list.transient(number)?.cast();
                if item.is_some() {
                    color_entity(&item, color, rank)?;
                }
            }
        }
        Ok(())
    }
    /// Colors a shape with its style, then its parts with what they inherit.
    fn make_colors(
        &mut self,
        shape: &Shape,
        walk: &mut ColorWalk<'_>,
        inherit: &XCAFPrsStyle,
    ) -> Result<(), Failure> {
        if walk.seen.contains(shape) {
            return Ok(());
        }
        walk.seen.add(shape.clone(), ());
        let style = if walk.settings.contains(shape) {
            overlaid_style(inherit, walk.settings.find_from_key(shape)?)
        } else {
            inherit.clone()
        };
        let kind = shape.shape_type()?;
        let color = match kind {
            ShapeEnum::Face | ShapeEnum::Solid => surface_color(&style),
            ShapeEnum::Edge | ShapeEnum::Wire if style.is_set_color_curv() => {
                Some(style.get_color_curv())
            }
            _ => None,
        };
        if let Some(value) = color {
            let rank = encode_color(&value);
            let entity = if rank == 0 {
                self.color_entity_of(&value, walk)?
            } else {
                Handle::default()
            };
            self.color_shape_entities(shape, &entity, rank)?;
        }
        if kind == ShapeEnum::Edge {
            return Ok(());
        }
        let mut parts = ShapeIterator::new(shape, true, true);
        while parts.more() {
            let part = parts.value()?.clone();
            parts.next();
            self.make_colors(&part, walk, &style)?;
        }
        Ok(())
    }
    /// Puts a level on the entities of a shape, part by part.
    fn attach_layer(
        process: &Handle<TransferFinderProcess>,
        tool: &XCAFDocLayerTool,
        shape: &Shape,
        level: i32,
    ) -> Result<(), Failure> {
        let kind = shape.shape_type()?;
        if kind == ShapeEnum::Compound {
            let mut parts = ShapeIterator::new(shape, true, true);
            while parts.more() {
                let part = parts.value()?.clone();
                parts.next();
                if tool.get_shape_layers(&part)?.is_empty()
                    || part.shape_type()? == ShapeEnum::Compound
                {
                    Self::attach_layer(process, tool, &part, level)?;
                }
            }
            return Ok(());
        }
        let mut targets = Vec::new();
        if kind == ShapeEnum::Solid || kind == ShapeEnum::Shell {
            let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
            while faces.more() {
                targets.push(faces.current().clone());
                faces.next()?;
            }
        } else {
            targets.push(shape.clone());
        }
        for target in targets {
            let entity = entity_of_shape(process, &target)?;
            if let Some(live) = entity.get() {
                live.init_level(&Handle::default(), level);
            }
        }
        Ok(())
    }
    fn layer_shapes(
        &self,
        tool: &XCAFDocLayerTool,
        layer: &TdfLabel,
        level: i32,
    ) -> Result<(), Failure> {
        let process = self.base.transfer_process();
        for label in XCAFDocLayerTool::get_shapes_of_layer(layer)? {
            let mut shape = Shape::default();
            if XCAFDocShapeTool::get_shape(&label, &mut shape)? {
                Self::attach_layer(&process, tool, &shape, level)?;
            }
        }
        Ok(())
    }
    /// Levels from the layers of a document: a layer named by a number is that level, the others
    /// come after the greatest; false without a tool; `writer.write_layers(&labels)?`.
    ///
    /// # Errors
    /// A failure of the document or of the process.
    pub fn write_layers(&mut self, labels: &[TdfLabel]) -> Result<bool, Failure> {
        let Some(first) = labels.first() else {
            return Ok(false);
        };
        let handle = XCAFDocDocumentTool::layer_tool(first)?;
        let Some(tool) = handle.get() else {
            return Ok(false);
        };
        let mut greatest = 0;
        let layer_labels = tool.get_layer_labels()?;
        let name_of = |label: &TdfLabel| -> Result<String, Failure> {
            Ok(tool
                .get_layer(label)?
                .as_ref()
                .map(text_of)
                .unwrap_or_default())
        };
        for layer in &layer_labels {
            if let Ok(level) = name_of(layer)?.trim().parse::<i32>() {
                greatest = greatest.max(level);
                self.layer_shapes(tool, layer, level)?;
            }
        }
        for layer in &layer_labels {
            if name_of(layer)?.trim().parse::<i32>().is_err()
                && !XCAFDocLayerTool::get_shapes_of_layer(layer)?.is_empty()
            {
                greatest += 1;
                self.layer_shapes(tool, layer, greatest)?;
            }
        }
        Ok(true)
    }
    /// The label and the long name of an entity from the name of its shape.
    fn name_entity(&mut self, shape: &Shape, name: &ExtendedString) -> Result<(), Failure> {
        let entity = entity_of_shape(&self.base.transfer_process(), shape)?;
        let Some(live) = entity.get() else {
            return Ok(());
        };
        let text = text_of(name);
        let visible: String = text
            .chars()
            .take(8)
            .map(|c| if c.is_ascii() { c } else { '?' })
            .collect();
        let short = format!("{visible:>8}");
        live.set_label(&Handle::new(HAsciiString::new(&short)), -1);
        let long_text: String = text
            .chars()
            .map(|c| if c.is_ascii() { c } else { '_' })
            .collect();
        let long = Handle::new(IgesBasicName::new());
        iges_live(&long, WHERE)?.init(1, &Handle::new(HAsciiString::new(&long_text)));
        live.add_property(&long)?;
        let _ = self.base.add_entity(&long.cast())?;
        Ok(())
    }
    /// The names of the shapes under a label; a part without a name takes the one above it.
    fn collect_shape_names(
        label: &TdfLabel,
        location: &Location,
        previous: Option<ExtendedString>,
        names: &mut NamedShapes,
    ) -> Result<(), Failure> {
        let own = name_of_label(label)?;
        let mut referred = TdfLabel::default();
        let has_referred = XCAFDocShapeTool::get_referred_shape(label, &mut referred)?;
        if has_referred {
            let moved = location.multiplied(&XCAFDocShapeTool::get_location(label)?)?;
            Self::collect_shape_names(&referred, &moved, own.clone(), names)?;
        }
        let mut components = Vec::new();
        let _ = XCAFDocShapeTool::get_components(label, &mut components, false)?;
        for component in &components {
            Self::collect_shape_names(component, location, own.clone(), names)?;
        }
        let mut subs = Vec::new();
        let _ = XCAFDocShapeTool::get_sub_shapes(label, &mut subs)?;
        for sub in subs {
            let mut shape = Shape::default();
            if XCAFDocShapeTool::get_shape(&sub, &mut shape)?
                && let Some(name) = name_of_label(&sub)?
            {
                bind_name(names, shape, name);
            }
        }
        if !has_referred
            && components.is_empty()
            && let Some(name) = previous
        {
            let mut shape = Shape::default();
            if XCAFDocShapeTool::get_shape(label, &mut shape)? {
                shape.move_by(location, false)?;
                bind_name(names, shape, name);
            }
        }
        Ok(())
    }
    /// Names the entities of named shapes: a short label and a long name property; false without a
    /// label; `writer.write_names(&labels)?`.
    ///
    /// # Errors
    /// A failure of the document or of the process.
    pub fn write_names(&mut self, labels: &[TdfLabel]) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        let mut names = NamedShapes::default();
        for label in labels {
            let mut shape = Shape::default();
            if !XCAFDocShapeTool::get_shape(label, &mut shape)? {
                continue;
            }
            let Some(name) = name_of_label(label)? else {
                continue;
            };
            bind_name(&mut names, shape, name.clone());
            Self::collect_shape_names(label, &Location::default(), Some(name), &mut names)?;
        }
        for rank in 1..=names.len() {
            let (shape, name) = (
                names.find_key(rank)?.clone(),
                names.find_from_index(rank)?.clone(),
            );
            self.name_entity(&shape, &name)?;
        }
        Ok(true)
    }
}
