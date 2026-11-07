//! `RwMeshShapeIterator`: the part of the document iterators common to faces, edges and vertices.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_doc::XCAFDocShapeTool;
use martensite_tkxcaf::xcaf_prs::{XCAFPrs, XCAFPrsStyle, XCAFPrsStyleMap};

/// The shape types from the most general, the order styles are dispatched in.
const SHAPE_TYPES: [ShapeEnum; 8] = [
    ShapeEnum::Compound,
    ShapeEnum::CompSolid,
    ShapeEnum::Solid,
    ShapeEnum::Shell,
    ShapeEnum::Face,
    ShapeEnum::Wire,
    ShapeEnum::Edge,
    ShapeEnum::Vertex,
];

/// The state of an iterator; `RwMeshShapeIteratorBase::with_shape(&shape, find, avoid, &style)?`.
pub struct RwMeshShapeIteratorBase {
    /// The style of each shape.
    pub styles: DataMap<Shape, XCAFPrsStyle, ShapeMapHasher>,
    /// The style of shapes without one.
    pub def_style: XCAFPrsStyle,
    /// Whether styles are dispatched to shapes.
    pub to_map_colors: bool,
    /// The explorer of the shape.
    pub iter: Explorer,
    /// The location of the current shape.
    pub location: Location,
    /// The transformation of the current shape.
    pub trsf: Transform3,
    /// The style of the current shape.
    pub style: XCAFPrsStyle,
    /// The color of the current shape.
    pub color: ColorRgba,
    /// The type of shape iterated.
    pub shape_type: ShapeEnum,
    /// Whether the current shape has a color.
    pub has_color: bool,
}

fn shape_type_index(shape_type: ShapeEnum) -> usize {
    SHAPE_TYPES
        .iter()
        .position(|t| *t == shape_type)
        .unwrap_or(SHAPE_TYPES.len())
}

impl RwMeshShapeIteratorBase {
    fn empty(shape_type: ShapeEnum, style: &XCAFPrsStyle, to_map_colors: bool) -> Self {
        Self {
            styles: DataMap::default(),
            def_style: style.clone(),
            to_map_colors,
            iter: Explorer::default(),
            location: Location::default(),
            trsf: Transform3::default(),
            style: XCAFPrsStyle::default(),
            color: ColorRgba::default(),
            shape_type,
            has_color: false,
        }
    }
    /// Over the shape of a label at a location, with its styles when asked;
    /// `RwMeshShapeIteratorBase::with_label(&label, &location, find, avoid, true, &style)?`.
    pub fn with_label(
        label: &TdfLabel,
        location: &Location,
        find: ShapeEnum,
        avoid: ShapeEnum,
        to_map_colors: bool,
        style: &XCAFPrsStyle,
    ) -> Result<Self, Failure> {
        let mut base = Self::empty(find, style, to_map_colors);
        let mut shape = Shape::default();
        if !XCAFDocShapeTool::get_shape(label, &mut shape)? || shape.is_null() {
            return Ok(base);
        }
        shape.set_location(location, false)?;
        base.iter.init(&shape, find, avoid)?;
        if to_map_colors {
            base.dispatch_styles(label, location, style)?;
            base.styles.bind(shape, style.clone());
        }
        Ok(base)
    }
    /// Over a shape, always mapping colors;
    /// `RwMeshShapeIteratorBase::with_shape(&shape, find, avoid, &style)?`.
    pub fn with_shape(
        shape: &Shape,
        find: ShapeEnum,
        avoid: ShapeEnum,
        style: &XCAFPrsStyle,
    ) -> Result<Self, Failure> {
        let mut base = Self::empty(find, style, true);
        if !shape.is_null() {
            base.iter.init(shape, find, avoid)?;
        }
        Ok(base)
    }
    /// Binds the styles of a label to the shapes iterated, general types after specific ones;
    /// `base.dispatch_styles(&label, &location, &style)?`.
    pub fn dispatch_styles(
        &mut self,
        label: &TdfLabel,
        location: &Location,
        style: &XCAFPrsStyle,
    ) -> Result<(), Failure> {
        let mut styles = XCAFPrsStyleMap::default();
        XCAFPrs::collect_style_settings(
            label,
            &Location::default(),
            &mut styles,
            &ColorRgba::default(),
        )?;
        let mut type_counts = [0_i32; SHAPE_TYPES.len()];
        for (key, _) in styles.items() {
            type_counts[shape_type_index(key.shape_type()?)] += 1;
        }
        let own_index = shape_type_index(self.shape_type);
        for index in (0..=own_index).rev() {
            if index == own_index || type_counts[index] != 0 {
                self.dispatch_type(&styles, SHAPE_TYPES[index], location, style)?;
            }
        }
        Ok(())
    }
    fn dispatch_type(
        &mut self,
        styles: &XCAFPrsStyleMap,
        shape_type: ShapeEnum,
        location: &Location,
        fallback: &XCAFPrsStyle,
    ) -> Result<(), Failure> {
        for (key, value) in styles.items() {
            if key.shape_type()? != shape_type {
                continue;
            }
            let style = completed_style(value, fallback);
            let located = key.located(location, true)?;
            if shape_type == self.shape_type {
                self.styles.bind(located, style);
                continue;
            }
            let mut shapes = Explorer::new(&located, self.shape_type, ShapeEnum::Shape)?;
            while shapes.more() {
                if !self.styles.is_bound(shapes.current()) {
                    self.styles.bind(shapes.current().clone(), style.clone());
                }
                shapes.next()?;
            }
        }
        Ok(())
    }
    /// Clears the information of the current shape; `base.reset_shape()`.
    pub fn reset_shape(&mut self) {
        self.has_color = false;
        self.color = ColorRgba::default();
        self.style = XCAFPrsStyle::default();
    }
    /// Takes the style and color of a current shape; `base.init_shape(&shape)`.
    pub fn init_shape(&mut self, shape: &Shape) {
        self.has_color = false;
        if !self.to_map_colors {
            return;
        }
        self.style = self.styles.seek(shape).unwrap_or(&self.def_style).clone();
        if let Some(material) = self.style.material().get() {
            self.has_color = true;
            self.color = material.base_color();
        } else if self.style.is_set_color_surf() {
            self.has_color = true;
            self.color = self.style.get_color_surf_rgba();
        } else if self.style.is_set_color_curv() {
            self.has_color = true;
            self.color = ColorRgba::from_rgb(self.style.get_color_curv(), 1.0);
        }
    }
}

fn completed_style(value: &XCAFPrsStyle, fallback: &XCAFPrsStyle) -> XCAFPrsStyle {
    let mut style = value.clone();
    if !style.is_set_color_curv() && fallback.is_set_color_curv() {
        style.set_color_curv(fallback.get_color_curv());
    }
    if !style.is_set_color_surf() && fallback.is_set_color_surf() {
        style.set_color_surf_rgba(fallback.get_color_surf_rgba());
    }
    if style.material().is_null() && fallback.material().is_some() {
        style.set_material(fallback.material());
    }
    style
}

/// What a document iterator gives: a shape at a time with its style and nodes.
pub trait RwMeshShapeIterator {
    /// The iterator state; `iterator.iterator_base()`.
    fn iterator_base(&self) -> &RwMeshShapeIteratorBase;
    /// The current shape; `iterator.shape()`.
    fn shape(&self) -> &Shape;
    /// True on a valid current shape; `iterator.more()`.
    fn more(&self) -> bool;
    /// Finds the next shape; `iterator.next()?`.
    fn next(&mut self) -> Result<(), Failure>;
    /// True when the current shape has no mesh data; `iterator.is_empty()`.
    fn is_empty(&self) -> bool;
    /// The lower element index; `iterator.elem_lower()`.
    fn elem_lower(&self) -> i32;
    /// The upper element index; `iterator.elem_upper()`.
    fn elem_upper(&self) -> i32;
    /// The number of nodes of the current shape; `iterator.nb_nodes()`.
    fn nb_nodes(&self) -> i32;
    /// The lower node index; `iterator.node_lower()`.
    fn node_lower(&self) -> i32;
    /// The upper node index; `iterator.node_upper()`.
    fn node_upper(&self) -> i32;
    /// The node without transformation; `iterator.node(1)?`.
    fn node(&self, node: i32) -> Result<Point3, Failure>;

    /// The shape explored; `iterator.explored_shape()`.
    fn explored_shape(&self) -> &Shape {
        self.iterator_base().iter.explored_shape()
    }
    /// The style of the current shape; `iterator.style()`.
    fn style(&self) -> &XCAFPrsStyle {
        &self.iterator_base().style
    }
    /// True when the current shape has a color; `iterator.has_color()`.
    fn has_color(&self) -> bool {
        self.iterator_base().has_color
    }
    /// The color of the current shape; `iterator.color()`.
    fn color(&self) -> ColorRgba {
        self.iterator_base().color
    }
    /// The node with the shape's transformation applied; `iterator.node_transformed(1)?`.
    fn node_transformed(&self, node: i32) -> Result<Point3, Failure> {
        Ok(self.node(node)?.transformed(self.iterator_base().trsf))
    }
}
