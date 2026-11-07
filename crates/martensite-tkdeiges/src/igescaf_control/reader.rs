//! `IgesCafControlReader`: reads an IGES file into a document, with colors, names and layers.
use super::decode_color;
use crate::iges_basic::IgesBasicSubfigureDef;
use crate::iges_control::IgesControlReader;
use crate::iges_data::{IgesDataDefList, IgesDataDefType, IgesDataIgesEntity, iges_live};
use crate::iges_graph::IgesGraphColor;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape, ShapeIterator};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::IndexedDataMap;
use martensite_tkernel::quantity::{Color, ColorRgba, ColorSpace};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::{ExtendedString, HAsciiString};
use martensite_tkernel::units_methods::LengthUnit;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocColorTool, XCAFDocColorType, XCAFDocDocumentTool, XCAFDocLayerTool, XCAFDocShapeTool,
};
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_algo::{XSAlgoShapeProcessor, xs_algo_units};
use martensite_tkxsbase::xs_control::{XSControlReaderOps, XSControlWorkSession};

const WHERE: &str = "IgesCafControlReader";

/// The shapes already put in the document.
type KnownShapes = IndexedDataMap<Shape, (), ShapeMapHasher>;

/// A compound with a location, or with one somewhere inside, is an assembly in the document.
fn is_composite(shape: &Shape) -> Result<bool, Failure> {
    if shape.shape_type()? != ShapeEnum::Compound {
        return Ok(false);
    }
    if !shape.location().is_identity() {
        return Ok(true);
    }
    let mut parts = ShapeIterator::new(shape, false, false);
    while parts.more() {
        if is_composite(parts.value()?)? {
            return Ok(true);
        }
        parts.next();
    }
    Ok(false)
}

/// The parts of a compound: a compound of the simple ones, a compound of the composite ones.
fn split_parts(
    tool: &XCAFDocShapeTool,
    shape: &Shape,
    known: &mut KnownShapes,
) -> Result<(Compound, Compound, i32, bool), Failure> {
    let builder = BRepBuilder::default();
    let (mut simple, mut composite) = (Compound::default(), Compound::default());
    builder.make_compound(&mut simple);
    builder.make_compound(&mut composite);
    let (mut nb_simple, mut has_composite) = (0, false);
    let mut parts = ShapeIterator::new(shape, false, false);
    while parts.more() {
        let part = parts.value()?.clone();
        parts.next();
        if is_composite(&part)? {
            has_composite = true;
            add_composite_shape(tool, &part, false, known)?;
            builder.add(&mut composite, &part)?;
        } else {
            builder.add(&mut simple, &part)?;
            nb_simple += 1;
        }
    }
    Ok((simple, composite, nb_simple, has_composite))
}

/// Adds a composite shape part by part, its simple parts together.
fn add_composite_shape(
    tool: &XCAFDocShapeTool,
    shape: &Shape,
    consider_location: bool,
    known: &mut KnownShapes,
) -> Result<(), Failure> {
    let location = shape.location().clone();
    let mut bare = shape.clone();
    if !consider_location && !location.is_identity() {
        bare.set_location(&Default::default(), false)?;
    }
    if known.contains(&bare) {
        return Ok(());
    }
    known.add(bare.clone(), ());
    let (simple, composite, nb_simple, has_composite) = split_parts(tool, shape, known)?;
    if nb_simple > 0 && has_composite {
        let _ = tool.add_shape(&simple, false, false)?;
        let builder = BRepBuilder::default();
        let mut made = Compound::default();
        builder.make_compound(&mut made);
        builder.add(&mut made, &simple)?;
        builder.add(&mut made, &composite)?;
        if !location.is_identity() {
            made.set_location(&location, false)?;
        }
        made.set_orientation(shape.orientation());
        let _ = tool.add_shape(&made, has_composite, false)?;
    } else {
        let _ = tool.add_shape(&bare, has_composite, false)?;
    }
    Ok(())
}

/// The color of an entity in a percentage range.
fn entity_color(entity: &IgesDataIgesEntity) -> Result<Option<Color>, Failure> {
    match entity.def_color() {
        IgesDataDefType::DefValue => Ok(Some(decode_color(entity.rank_color()))),
        IgesDataDefType::DefReference => {
            let color = entity.color().cast::<IgesGraphColor>();
            let Some(live) = color.get() else {
                return Ok(None);
            };
            let (mut red, mut green, mut blue) = (0.0, 0.0, 0.0);
            live.rgb_intensity(&mut red, &mut green, &mut blue);
            let range = |value: f64| value.clamp(0.0, 100.0) * 0.01;
            Color::new(range(red), range(green), range(blue), ColorSpace::Srgb).map(Some)
        }
        _ => Ok(None),
    }
}

fn set_layers(
    tool: &XCAFDocLayerTool,
    label: &TdfLabel,
    entity: &IgesDataIgesEntity,
) -> Result<(), Failure> {
    match entity.def_level() {
        IgesDataDefList::DefOne => {
            let name = ExtendedString::new(&entity.level().to_string());
            tool.set_layer(label, &name, false)?;
        }
        IgesDataDefList::DefSeveral => {
            let list = entity.level_list();
            let levels = iges_live(&list, WHERE)?;
            for rank in 1..=levels.nb_level_numbers()? {
                let name = ExtendedString::new(&levels.level_number(rank)?.to_string());
                tool.set_layer(label, &name, false)?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// Names a label with the text of an entity, without its surrounding blanks.
fn set_name(label: &TdfLabel, name: &Handle<HAsciiString>) -> Result<(), Failure> {
    let text = iges_live(name, WHERE)?.string();
    let name = String::from_utf8_lossy(text.as_bytes()).trim().to_string();
    let _ = TDataStdName::set(label, &ExtendedString::new(&name))?;
    Ok(())
}

/// Reads an IGES file into a document.
///
/// ```no_run
/// use martensite_tkdeiges::igescaf_control::IgesCafControlReader;
/// let reader = IgesCafControlReader::new().unwrap();
/// assert!(reader.get_color_mode());
/// ```
pub struct IgesCafControlReader {
    base: IgesControlReader,
    color_mode: bool,
    name_mode: bool,
    layer_mode: bool,
}

impl IgesCafControlReader {
    /// A reader on a new session, with all modes; `IgesCafControlReader::new()?`.
    ///
    /// # Errors
    /// A failure of the init of the norm.
    pub fn new() -> Result<Self, Failure> {
        Ok(Self {
            base: IgesControlReader::new()?,
            color_mode: true,
            name_mode: true,
            layer_mode: true,
        })
    }
    /// A reader on a session, with all modes; `IgesCafControlReader::with_session(&ws, true)?`.
    ///
    /// # Errors
    /// A failure of the init of the norm.
    pub fn with_session(ws: &Handle<XSControlWorkSession>, scratch: bool) -> Result<Self, Failure> {
        Ok(Self {
            base: IgesControlReader::with_session(ws, scratch)?,
            color_mode: true,
            name_mode: true,
            layer_mode: true,
        })
    }
    /// The reader of shapes under it; `reader.base_mut()`.
    pub fn base_mut(&mut self) -> &mut IgesControlReader {
        &mut self.base
    }
    /// Sets whether colors are read; `reader.set_color_mode(true)`.
    pub fn set_color_mode(&mut self, mode: bool) {
        self.color_mode = mode;
    }
    /// Whether colors are read; `reader.get_color_mode()`.
    pub fn get_color_mode(&self) -> bool {
        self.color_mode
    }
    /// Sets whether names are read; `reader.set_name_mode(true)`.
    pub fn set_name_mode(&mut self, mode: bool) {
        self.name_mode = mode;
    }
    /// Whether names are read; `reader.get_name_mode()`.
    pub fn get_name_mode(&self) -> bool {
        self.name_mode
    }
    /// Sets whether layers are read; `reader.set_layer_mode(true)`.
    pub fn set_layer_mode(&mut self, mode: bool) {
        self.layer_mode = mode;
    }
    /// Whether layers are read; `reader.get_layer_mode()`.
    pub fn get_layer_mode(&self) -> bool {
        self.layer_mode
    }
    fn prepared_unit(&self, document: &Handle<TDocStdDocument>) -> Result<(), Failure> {
        let model = self.base.iges_model()?;
        let mut scale = 1.0;
        if !XCAFDocDocumentTool::get_length_unit_in(document, &mut scale, LengthUnit::Millimeter)? {
            XSAlgoShapeProcessor::prepare_for_transfer();
            scale = xs_algo_units().get_cascade_length_unit(LengthUnit::Millimeter);
            XCAFDocDocumentTool::set_length_unit_in(document, scale, LengthUnit::Millimeter)?;
        }
        let live = iges_live(&model, WHERE)?;
        let mut section = live.global_section();
        section.set_cascade_unit(scale);
        live.set_global_section(&section);
        Ok(())
    }
    /// The attributes of one entity with a shape, on the label of the shape.
    fn apply_entity(
        &self,
        entity: &Handle<IgesDataIgesEntity>,
        shape: &Shape,
        (shapes, colors, layers): (
            &XCAFDocShapeTool,
            &Handle<XCAFDocColorTool>,
            &Handle<XCAFDocLayerTool>,
        ),
    ) -> Result<(), Failure> {
        let body = iges_live(entity, WHERE)?;
        let color = if self.color_mode && colors.is_some() {
            entity_color(body)?
        } else {
            None
        };
        let color = color.map(|value| ColorRgba::from_rgb(value, 1.0));
        let use_layers = self.layer_mode && layers.is_some();
        let mut label = TdfLabel::default();
        if let (Some(value), Some(tool)) = (color.as_ref(), colors.get()) {
            let _ = tool.add_color(value)?;
        }
        let found = shapes.search_using_map(shape, &mut label, false, color.is_some())?;
        if !found {
            self.apply_on_parts(body, shape, shapes, (&color, colors), (use_layers, layers))?;
        } else {
            if let (Some(value), Some(tool)) = (color.as_ref(), colors.get()) {
                tool.set_color(&label, value, XCAFDocColorType::ColorGen)?;
            }
            if self.name_mode && body.has_name() {
                set_name(&label, &body.name_value()?)?;
            }
            if use_layers {
                set_layers(iges_live(layers, WHERE)?, &label, body)?;
            }
        }
        self.apply_subfigure_name(entity, shape, shapes)
    }
    fn apply_on_parts(
        &self,
        body: &IgesDataIgesEntity,
        shape: &Shape,
        shapes: &XCAFDocShapeTool,
        (color, colors): (&Option<ColorRgba>, &Handle<XCAFDocColorTool>),
        (use_layers, layers): (bool, &Handle<XCAFDocLayerTool>),
    ) -> Result<(), Failure> {
        let (Some(value), Some(tool)) = (color.as_ref(), colors.get()) else {
            return Ok(());
        };
        let mut parts = ShapeIterator::new(shape, true, true);
        while parts.more() {
            let mut label = TdfLabel::default();
            if shapes.search_using_map(parts.value()?, &mut label, false, true)? {
                tool.set_color(&label, value, XCAFDocColorType::ColorGen)?;
                if use_layers {
                    set_layers(iges_live(layers, WHERE)?, &label, body)?;
                }
            }
            parts.next();
        }
        Ok(())
    }
    fn apply_subfigure_name(
        &self,
        entity: &Handle<IgesDataIgesEntity>,
        shape: &Shape,
        shapes: &XCAFDocShapeTool,
    ) -> Result<(), Failure> {
        let subfigure = entity.cast::<IgesBasicSubfigureDef>();
        let Some(live) = subfigure.get() else {
            return Ok(());
        };
        let name = live.name();
        if !self.name_mode || name.is_null() {
            return Ok(());
        }
        let mut label = TdfLabel::default();
        if shapes.search(shape, &mut label, [true, true, false])? {
            set_name(&label, &name)?;
        }
        Ok(())
    }
    /// Translates the model read to the document; gives false without a shape;
    /// `reader.transfer(&document, &range)?`.
    ///
    /// # Errors
    /// A failure of the session or of the document.
    pub fn transfer(
        &mut self,
        document: &Handle<TDocStdDocument>,
        range: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        self.prepared_unit(document)?;
        let _ = self.base.transfer_roots(range)?;
        let reader = self.base.xs_reader();
        let nb_shapes = reader.nb_shapes();
        if nb_shapes <= 0 {
            return Ok(false);
        }
        let main = iges_live(document, WHERE)?.main()?;
        let shape_tool = XCAFDocDocumentTool::shape_tool(&main)?;
        let Some(shapes) = shape_tool.get() else {
            return Ok(false);
        };
        for rank in 1..=nb_shapes {
            let shape = reader.shape(rank)?;
            if is_composite(&shape)? {
                add_composite_shape(shapes, &shape, true, &mut KnownShapes::default())?;
            } else {
                let _ = shapes.add_shape(&shape, false, true)?;
            }
        }
        let colors = XCAFDocDocumentTool::color_tool(&main)?;
        let layers = XCAFDocDocumentTool::layer_tool(&main)?;
        let ws = reader.ws();
        let transfer = iges_live(&ws, WHERE)?.transfer_reader();
        let process_handle = iges_live(&transfer, WHERE)?.transient_process();
        let process = iges_live(&process_handle, WHERE)?;
        let model = self.base.iges_model()?;
        let live_model = iges_live(&model, WHERE)?;
        for rank in 1..=live_model.nb_entities() {
            let entity = live_model.entity(rank);
            if entity.is_null() {
                continue;
            }
            let binder = process.find(&entity.cast());
            if binder.is_null() {
                continue;
            }
            let shape = TransferBRep::shape_result(&binder);
            if !shape.is_null() {
                self.apply_entity(&entity, &shape, (shapes, &colors, &layers))?;
            }
        }
        if let Some(tool) = colors.get() {
            tool.reverse_chains_of_tree_nodes()?;
        }
        shapes.update_assemblies()?;
        Ok(true)
    }
    /// Reads a file and translates it to the document; `reader.perform("a.igs", &document, &range)?`.
    ///
    /// # Errors
    /// A failure of the reading or of the document.
    pub fn perform(
        &mut self,
        file_name: &str,
        document: &Handle<TDocStdDocument>,
        range: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        if self.base.read_file(file_name)? != IFSelectReturnStatus::RetDone {
            return Ok(false);
        }
        self.transfer(document, range)
    }
}
