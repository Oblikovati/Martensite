//! Filling an exchange document from the root shapes of a mesh reader.
use super::RwMeshNodeAttributes;
use super::caf_reader::{RwMeshCafDocumentTools, RwMeshCafReaderBase, file_name_of};
use super::caf_reader_attributes::{
    record_sub_shapes, set_name, set_shape_name, set_shape_named_data, set_shape_style,
    shape_type_name,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::Compound;
use martensite_tkbrep::topods::{Shape, ShapeIterator};
use martensite_tkernel::message::Message;
use martensite_tkernel::standard::Failure;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use martensite_tkxcaf::xcaf_prs::{
    XCAF_PRS_DOCUMENT_EXPLORER_FLAGS_NO_STYLE, XCAFPrsDocumentExplorer, XCAFPrsStyle,
};
use std::sync::{Mutex, PoisonError};

/// Held while a reader fills a document with the automatic naming switched off.
static FILL_LOCK: Mutex<()> = Mutex::new(());

fn unlocated(shape: &Shape) -> Result<Shape, Failure> {
    shape.located(&Location::default(), true)
}

fn tool(tools: &RwMeshCafDocumentTools) -> Result<&XCAFDocShapeTool, Failure> {
    tools.shape_tool.get().ok_or_else(|| {
        Failure::with_type(
            "Standard_NullObject",
            "RwMeshCafReader: the document has no shape tool; expected one",
            "",
        )
    })
}

fn referred(label: &TdfLabel) -> Result<TdfLabel, Failure> {
    let mut reference = label.clone();
    XCAFDocShapeTool::get_referred_shape(label, &mut reference)?;
    Ok(reference)
}

impl RwMeshCafReaderBase {
    fn attributes_of(&self, shape: &Shape) -> RwMeshNodeAttributes {
        self.attrib_map.seek(shape).cloned().unwrap_or_default()
    }
    /// True when a compound holds more than plain unnamed faces.
    fn needs_assembly(&self, shape: &Shape) -> Result<bool, Failure> {
        if shape.shape_type()? != ShapeEnum::Compound {
            return Ok(false);
        }
        let mut children = ShapeIterator::new(shape, true, false);
        while children.more() {
            let child = children.value()?;
            if child.shape_type()? != ShapeEnum::Face {
                return Ok(true);
            }
            if self
                .attrib_map
                .seek(child)
                .is_some_and(|a| !a.name.is_empty())
            {
                return Ok(true);
            }
            children.next();
        }
        Ok(false)
    }
    /// Fills the document with the root shapes, unless disabled or empty.
    pub(super) fn fill_document_from_roots(&mut self) -> Result<(), Failure> {
        let Some(document) = self
            .xde_doc
            .get()
            .filter(|_| self.to_fill_doc && !self.root_shapes.is_empty())
        else {
            return Ok(());
        };
        self.reconcile_length_unit()?;
        // The flag is process wide: two readers filling at once would restore it under each other.
        let _fill = FILL_LOCK.lock().unwrap_or_else(PoisonError::into_inner);
        let was_auto_naming = XCAFDocShapeTool::auto_naming();
        XCAFDocShapeTool::set_auto_naming(false);
        let main = document.main()?;
        let mut tools = RwMeshCafDocumentTools {
            shape_tool: XCAFDocDocumentTool::shape_tool(&main)?,
            color_tool: XCAFDocDocumentTool::color_tool(&main)?,
            vis_material_tool: XCAFDocDocumentTool::vis_material_tool(&main)?,
            ..RwMeshCafDocumentTools::default()
        };
        let result = self.add_roots(&mut tools);
        XCAFDocShapeTool::set_auto_naming(was_auto_naming);
        result
    }
    fn add_roots(&self, tools: &mut RwMeshCafDocumentTools) -> Result<(), Failure> {
        for root in &self.root_shapes {
            self.add_shape_into_doc(tools, root, &TdfLabel::default(), &AsciiString::default())?;
        }
        tool(tools)?.update_assemblies()
    }
    fn reconcile_length_unit(&self) -> Result<(), Failure> {
        let mut unit = 1.0;
        let system = self.coord_sys_converter.output_length_unit();
        if !XCAFDocDocumentTool::get_length_unit(&self.xde_doc, &mut unit)? {
            XCAFDocDocumentTool::set_length_unit(&self.xde_doc, system)?;
        } else if unit != system {
            Message::send_warning_text(&AsciiString::new(
                "Warning: Length unit of document not equal to the system length unit",
            ));
        }
        Ok(())
    }
    /// An empty located compound, so children can be added as components one by one.
    fn empty_assembly(shape: &Shape) -> Result<Shape, Failure> {
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        let mut located: Shape = compound.into();
        located.set_location(shape.location(), false)?;
        Ok(located)
    }
    /// The label of a new shape; the second label is an existing component reused.
    fn new_label(
        tools: &mut RwMeshCafDocumentTools,
        shape: &Shape,
        to_add: &Shape,
        parent: &TdfLabel,
        make_assembly: bool,
    ) -> Result<(TdfLabel, TdfLabel), Failure> {
        let shape_tool = tools.shape_tool.clone();
        let shapes = shape_tool.get().ok_or_else(|| {
            Failure::with_type(
                "Standard_NullObject",
                "RwMeshCafReader: the document has no shape tool; expected one",
                "",
            )
        })?;
        if parent.is_null() {
            return Ok((
                shapes.add_shape(to_add, make_assembly, true)?,
                TdfLabel::default(),
            ));
        }
        if !XCAFDocShapeTool::is_assembly(parent)? {
            let label = shapes.add_sub_shape(parent, shape)?;
            if !label.is_null() {
                record_sub_shapes(&label, shape)?;
            }
            return Ok((label, TdfLabel::default()));
        }
        Self::new_component(tools, shapes, shape, to_add, parent, make_assembly)
    }
    fn new_component(
        tools: &mut RwMeshCafDocumentTools,
        shapes: &XCAFDocShapeTool,
        shape: &Shape,
        to_add: &Shape,
        parent: &TdfLabel,
        make_assembly: bool,
    ) -> Result<(TdfLabel, TdfLabel), Failure> {
        let no_location = unlocated(shape)?;
        if let Some(old) = tools.component_map.seek(&no_location).cloned() {
            return Ok((shapes.add_component(parent, &old, shape.location())?, old));
        }
        let label = shapes.add_component_shape(parent, to_add, make_assembly)?;
        let reference = referred(&label)?;
        if !reference.is_null() {
            tools.component_map.bind(no_location, reference);
        }
        Ok((label, TdfLabel::default()))
    }
    /// Puts the instance's attributes when it differs from its product; true when the product
    /// took its name from the instance.
    fn set_instance_attributes(
        &self,
        tools: &RwMeshCafDocumentTools,
        shape: &Shape,
        labels: (&TdfLabel, &TdfLabel),
        product: &RwMeshNodeAttributes,
        parents: (&TdfLabel, &AsciiString),
    ) -> Result<bool, Failure> {
        let (label, ref_label) = labels;
        let (parent, parent_name) = parents;
        let shape_type = shape.shape_type()?;
        let instance = self
            .attrib_map
            .seek(shape)
            .filter(|_| !shape.location().is_identity());
        let Some(instance) = instance else {
            set_shape_name(label, shape_type, &product.name, parent, parent_name)?;
            return Ok(false);
        };
        if !instance.style.is_equal(&product.style) {
            set_shape_style(tools, label, &instance.style)?;
        }
        if instance.named_data.object_address() != product.named_data.object_address() {
            set_shape_named_data(label, &instance.named_data)?;
        }
        set_shape_name(label, shape_type, &instance.name, parent, parent_name)?;
        if product.name.is_empty() && !instance.name.is_empty() {
            // An unnamed product is not nice, so copy the name of its first instance.
            set_shape_name(ref_label, shape_type, &instance.name, parent, parent_name)?;
            return Ok(true);
        }
        if instance.name.is_empty() && !product.name.is_empty() {
            set_shape_name(label, shape_type, &product.name, parent, parent_name)?;
        }
        Ok(false)
    }
    /// Appends a shape to the document, recursively; `base.add_shape_into_doc(&mut tools, &shape, &parent, &name)?`.
    pub fn add_shape_into_doc(
        &self,
        tools: &mut RwMeshCafDocumentTools,
        shape: &Shape,
        parent: &TdfLabel,
        parent_name: &AsciiString,
    ) -> Result<bool, Failure> {
        if shape.is_null() || self.xde_doc.is_null() {
            return Ok(false);
        }
        let make_assembly = self.needs_assembly(shape)?;
        let to_add = if make_assembly {
            Self::empty_assembly(shape)?
        } else {
            shape.clone()
        };
        let (label, old_label) = Self::new_label(tools, shape, &to_add, parent, make_assembly)?;
        if label.is_null() {
            return Ok(false);
        }
        if make_assembly {
            let mut assembly = TdfLabel::default();
            XCAFDocShapeTool::get_referred_shape(&label, &mut assembly)?;
            if !assembly.is_null() {
                tools.original_shape_map.bind(shape.clone(), assembly);
            }
        }
        self.put_attributes(tools, shape, (&label, &old_label), (parent, parent_name))?;
        Ok(true)
    }
    fn put_attributes(
        &self,
        tools: &mut RwMeshCafDocumentTools,
        shape: &Shape,
        labels: (&TdfLabel, &TdfLabel),
        parents: (&TdfLabel, &AsciiString),
    ) -> Result<(), Failure> {
        let (label, old_label) = labels;
        let (parent, parent_name) = parents;
        let ref_label = referred(label)?;
        let product = self.attributes_of(&unlocated(shape)?);
        let has_product_name = label != &ref_label
            && self.set_instance_attributes(
                tools,
                shape,
                (label, &ref_label),
                &product,
                parents,
            )?;
        if !old_label.is_null() {
            return Ok(());
        }
        if !has_product_name {
            set_shape_name(
                &ref_label,
                shape.shape_type()?,
                &product.name,
                parent,
                parent_name,
            )?;
        }
        set_shape_style(tools, &ref_label, &product.style)?;
        set_shape_named_data(&ref_label, &product.named_data)?;
        self.add_children(tools, shape, &ref_label)
    }
    fn add_children(
        &self,
        tools: &mut RwMeshCafDocumentTools,
        shape: &Shape,
        ref_label: &TdfLabel,
    ) -> Result<(), Failure> {
        let is_assembly = XCAFDocShapeTool::is_assembly(ref_label)?;
        let mut children = ShapeIterator::new(shape, true, false);
        while children.more() {
            let child = children.value()?.clone();
            if is_assembly {
                self.add_shape_into_doc(tools, &child, ref_label, &AsciiString::default())?;
            } else {
                self.add_sub_shape_into_doc(tools, &child, ref_label)?;
            }
            children.next();
        }
        Ok(())
    }
    /// Appends a sub-shape with attributes, recursively; shells and wires are only passed through;
    /// `base.add_sub_shape_into_doc(&tools, &face, &parent)?`.
    pub fn add_sub_shape_into_doc(
        &self,
        tools: &RwMeshCafDocumentTools,
        shape: &Shape,
        parent: &TdfLabel,
    ) -> Result<bool, Failure> {
        if shape.is_null() || self.xde_doc.is_null() {
            return Ok(false);
        }
        let shape_type = shape.shape_type()?;
        let attributes = self.attrib_map.seek(&unlocated(shape)?);
        if attributes.is_none() && !matches!(shape_type, ShapeEnum::Shell | ShapeEnum::Wire) {
            return Ok(false);
        }
        let mut children = ShapeIterator::new(shape, true, false);
        while children.more() {
            self.add_sub_shape_into_doc(tools, children.value()?, parent)?;
            children.next();
        }
        let Some(attributes) = attributes else {
            return Ok(false);
        };
        Self::label_sub_shape(tools, shape, parent, attributes)
    }
    fn label_sub_shape(
        tools: &RwMeshCafDocumentTools,
        shape: &Shape,
        parent: &TdfLabel,
        attributes: &RwMeshNodeAttributes,
    ) -> Result<bool, Failure> {
        let label = tool(tools)?.add_sub_shape(parent, shape)?;
        if label.is_null() {
            return Ok(false);
        }
        record_sub_shapes(&label, shape)?;
        let ref_label = referred(&label)?;
        let no_parent = TdfLabel::default();
        let shape_type = shape.shape_type()?;
        set_shape_name(
            &ref_label,
            shape_type,
            &attributes.name,
            &no_parent,
            &AsciiString::default(),
        )?;
        set_shape_style(tools, &ref_label, &attributes.style)?;
        set_shape_named_data(&ref_label, &attributes.named_data)?;
        Ok(true)
    }
    /// Replaces the empty names of the new roots with the file name, with a prefix.
    pub(super) fn generate_root_names(
        &self,
        file: &AsciiString,
        root_lower: i32,
        with_sub_labels: bool,
    ) -> Result<(), Failure> {
        let Some(document) = self.xde_doc.get() else {
            return Ok(());
        };
        let mut root_name = self.root_prefix.clone();
        root_name.append(&file_name_of(file));
        let tool = XCAFDocDocumentTool::shape_tool(&document.main()?)?;
        let roots = tool
            .get()
            .map_or(Ok(Vec::new()), XCAFDocShapeTool::get_free_shapes)?;
        if (roots.len() as i32) < root_lower {
            return Ok(());
        }
        let new_roots = &roots[(root_lower - 1).max(0) as usize..];
        for root in new_roots {
            name_root(root, &root_name)?;
        }
        if with_sub_labels {
            self.name_sub_labels(new_roots)?;
        }
        Ok(())
    }
    fn name_sub_labels(&self, roots: &[TdfLabel]) -> Result<(), Failure> {
        let mut explorer = XCAFPrsDocumentExplorer::with_roots(
            &self.xde_doc,
            roots,
            XCAF_PRS_DOCUMENT_EXPLORER_FLAGS_NO_STYLE,
            &XCAFPrsStyle::default(),
        )?;
        while explorer.more() {
            let ref_label = explorer.current().ref_label.clone();
            if explorer.current_depth() != 0 && !has_name(&ref_label)? {
                let shape = XCAFDocShapeTool::shape_of(&ref_label)?;
                if !shape.is_null() {
                    set_name(&ref_label, &shape_type_name(shape.shape_type()?))?;
                }
            }
            explorer.next()?;
        }
        Ok(())
    }
}

fn has_name(label: &TdfLabel) -> Result<bool, Failure> {
    label.is_attribute(&TDataStdName::get_id())
}

fn name_root(root: &TdfLabel, name: &AsciiString) -> Result<(), Failure> {
    let ref_label = referred(root)?;
    if !has_name(&ref_label)? {
        set_name(&ref_label, name)?;
    }
    if root != &ref_label && !has_name(root)? {
        set_name(root, name)?;
    }
    Ok(())
}
