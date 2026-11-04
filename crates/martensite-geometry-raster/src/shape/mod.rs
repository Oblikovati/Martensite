//

//! `TopologyDrawableShape`: a shape drawn as its face isos, edges and free vertices (or
//! as its hidden-line removal).
mod description;
mod edges;
mod faces;
mod hidden;
mod normals;
mod plot;
pub use normals::{MeshNormals, NormalSegment};

use super::edge_face::{EdgeDrawRecord, FaceDrawRecord};
use super::hide_data::HiddenLineRecords;
use super::iso_builder::FaceIsoBuilder;
use crate::picks::ShapePickSink;
use martensite_raster_core::{PaletteColor, PaletteKind, RasterDisplay, RasterReports};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp::{Explorer, map_shapes_and_ancestors};
use martensite_tkbrep::topods::{Shape, edge as to_edge, face as to_face};
use martensite_tkbrep::toptools::IndexedDataMapOfShapeListOfShape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// The colors of a drawable shape: free edges (one face), connected edges (two faces
/// or more, and vertices), isolated edges (no face), isos.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeColors {
    /// `FreeCol`.
    pub free: PaletteColor,
    /// `ConnCol`.
    pub connected: PaletteColor,
    /// `EdgeCol`.
    pub edge: PaletteColor,
    /// `IsosCol`.
    pub isos: PaletteColor,
}

impl ShapeColors {
    /// The colors topology-command gives every shape: green free edges, yellow connected edges,
    /// red isolated edges, blue isos.
    pub fn standard() -> Self {
        Self {
            free: PaletteColor::new(PaletteKind::Vert),
            connected: PaletteColor::new(PaletteKind::Jaune),
            edge: PaletteColor::new(PaletteKind::Rouge),
            isos: PaletteColor::new(PaletteKind::Bleu),
        }
    }
}

/// The hidden-line settings of a drawable shape.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HlrSettings {
    /// `withHLR`: draw the hidden-line removal instead of the wireframe.
    pub with_hlr: bool,
    /// `withRg1`: keep the smooth edges.
    pub with_rg1: bool,
    /// `withRgN`: keep the seam-like edges.
    pub with_rgn: bool,
    /// `withHid`: draw the hidden parts too.
    pub with_hid: bool,
    /// `ang`: the discretisation angle.
    pub angle: f64,
}

/// `TopologyDrawableShape`: a shape with its display settings and the display records
/// (faces with isos, colored edges, per-view hidden lines) built on first drawing.
pub struct TopologyRendering {
    pick_sink: Rc<dyn ShapePickSink>,
    shape: Shape,
    edges: RefCell<Vec<EdgeDrawRecord>>,
    faces: RefCell<Vec<FaceDrawRecord>>,
    hidden: RefCell<Vec<HiddenLineRecords>>,
    size: f64,
    discret: Cell<i32>,
    colors: ShapeColors,
    nb_isos: Cell<i32>,
    display_orientation: Cell<bool>,
    triangulations: Cell<bool>,
    polygons: Cell<bool>,
    hlr: Cell<HlrSettings>,
}

struct TopologyRenderInputs {
    shape: Shape,
    colors: ShapeColors,
    size: f64,
    nb_isos: i32,
    discret: i32,
    pick_sink: Rc<dyn ShapePickSink>,
}

impl TopologyRendering {
    /// `TopologyDrawableShape(C, FreeCol, ConnCol, EdgeCol, IsosCol, size, nbisos,
    /// discret)`: no orientation, triangulation, polygon or hidden-line display (the
    /// angle is 0 until `DisplayHLR`, where C++ leaves it uninitialized).
    ///
    /// ```
    /// use martensite_tkbrep::topods::Shape;
    /// use martensite_geometry_raster::{TopologyRendering, ShapeColors};
    /// use martensite_geometry_raster::picks::LocalShapePicks;
    /// use std::rc::Rc;
    /// let drawable = TopologyRendering::new(Shape::default(), ShapeColors::standard(), 100.0, 2, 30, Rc::new(LocalShapePicks::default()));
    /// assert_eq!((drawable.nb_isos(), drawable.discret()), (2, 30));
    /// ```
    pub fn new(
        shape: Shape,
        colors: ShapeColors,
        size: f64,
        nb_isos: i32,
        discret: i32,
        pick_sink: Rc<dyn ShapePickSink>,
    ) -> Self {
        Self::initial_records(TopologyRenderInputs {
            shape,
            colors,
            size,
            nb_isos,
            discret,
            pick_sink,
        })
    }

    fn initial_records(inputs: TopologyRenderInputs) -> Self {
        Self {
            pick_sink: inputs.pick_sink,
            shape: inputs.shape,
            edges: RefCell::new(Vec::new()),
            faces: RefCell::new(Vec::new()),
            hidden: RefCell::new(Vec::new()),
            size: inputs.size,
            discret: Cell::new(inputs.discret),
            colors: inputs.colors,
            nb_isos: Cell::new(inputs.nb_isos),
            display_orientation: Cell::new(false),
            triangulations: Cell::new(false),
            polygons: Cell::new(false),
            hlr: Cell::new(HlrSettings::default()),
        }
    }

    /// `Shape`.
    pub fn shape(&self) -> Shape {
        self.shape.clone()
    }

    /// Borrows the stored shape without changing handle counts; `let shape = rendering.shape_ref();`.
    pub fn shape_ref(&self) -> &Shape {
        &self.shape
    }

    /// `NbIsos`.
    pub fn nb_isos(&self) -> i32 {
        self.nb_isos.get()
    }

    /// `ChangeNbIsos`: rebuilds the face records with `nb_isos` isos per direction;
    /// `out` receives the iso builder reports.
    pub fn change_nb_isos(&self, nb_isos: i32, out: &dyn RasterReports) -> Result<(), Failure> {
        self.nb_isos.set(nb_isos);
        *self.faces.borrow_mut() = self.face_records(out)?;
        Ok(())
    }

    /// `Discret`.
    pub fn discret(&self) -> i32 {
        self.discret.get()
    }

    /// `ChangeDiscret`.
    pub fn change_discret(&self, discret: i32) {
        self.discret.set(discret);
    }

    /// `DisplayOrientation`: color the edges and vertices by orientation and mark the
    /// edge ends with arrows.
    pub fn display_orientation(&self, display: bool) {
        self.display_orientation.set(display);
    }

    /// `DisplayTriangulation(D)`.
    pub fn set_display_triangulation(&self, display: bool) {
        self.triangulations.set(display);
    }

    /// `DisplayTriangulation()`.
    pub fn display_triangulation(&self) -> bool {
        self.triangulations.get()
    }

    /// `DisplayPolygons(D)`.
    pub fn set_display_polygons(&self, display: bool) {
        self.polygons.set(display);
    }

    /// `DisplayPolygons()`.
    pub fn display_polygons(&self) -> bool {
        self.polygons.get()
    }

    /// `DisplayHLR(withHLR, withRg1, withRgN, withHid, ang)`.
    pub fn display_hlr(&self, settings: HlrSettings) {
        self.hlr.set(settings);
    }

    /// `GetDisplayHLR`.
    pub fn hlr_settings(&self) -> HlrSettings {
        self.hlr.get()
    }

    /// `updateDisplayData`: the face records (isos when `NbIsos` is not 0 and the face
    /// has a surface) and the non-degenerated edges colored by their face count.
    fn update_display_data(&self, out: &dyn RasterReports) -> Result<(), Failure> {
        self.faces.borrow_mut().clear();
        self.edges.borrow_mut().clear();
        if self.shape.is_null() {
            return Ok(());
        }
        *self.faces.borrow_mut() = self.face_records(out)?;
        *self.edges.borrow_mut() = self.edge_records()?;
        Ok(())
    }

    /// One `TopologyFace` per face of the shape.
    fn face_records(&self, out: &dyn RasterReports) -> Result<Vec<FaceDrawRecord>, Failure> {
        let mut records = Vec::new();
        let mut explorer = Explorer::new(&self.shape, ShapeEnum::Face, ShapeEnum::Shape)?;
        while explorer.more() {
            let mut face = to_face(explorer.current())?;
            explorer.next()?;
            if self.nb_isos.get() == 0 || BRepTool::surface_with_location(&face)?.0.is_null() {
                records.push(FaceDrawRecord::new(face, 0, self.colors.edge));
                continue;
            }
            face.set_orientation(Orientation::Forward);
            let builder = FaceIsoBuilder::new(&face, self.size, self.nb_isos.get())?;
            let mut record = FaceDrawRecord::new(face, builder.nb_domains(), self.colors.isos);
            builder.load_isos(&mut record, out)?;
            records.push(record);
        }
        Ok(records)
    }

    /// One `TopologyEdge` per non-degenerated edge: isolated, free or connected color.
    fn edge_records(&self) -> Result<Vec<EdgeDrawRecord>, Failure> {
        let ancestors = self.edge_ancestors()?;
        let mut records = Vec::new();
        for index in 1..=ancestors.extent() {
            let edge = to_edge(ancestors.find_key(index)?)?;
            if BRepTool::degenerated(&edge)? {
                continue;
            }
            let color = match ancestors.find_from_index(index)?.extent() {
                0 => self.colors.edge,
                1 => self.colors.free,
                _ => self.colors.connected,
            };
            records.push(EdgeDrawRecord::new(edge, color));
        }
        Ok(records)
    }

    fn edge_ancestors(&self) -> Result<IndexedDataMapOfShapeListOfShape, Failure> {
        let mut ancestors = IndexedDataMapOfShapeListOfShape::default();
        map_shapes_and_ancestors(
            &self.shape,
            ShapeEnum::Edge,
            ShapeEnum::Face,
            &mut ancestors,
        )?;
        Ok(ancestors)
    }

    /// The draw with the iso reports on `out` (`DrawOn` prints them on `std::cout`).
    pub fn draw_with_reports(
        &self,
        display: &mut RasterDisplay<'_>,
        out: &dyn RasterReports,
    ) -> Result<(), Failure> {
        if self.shape.is_null() {
            self.draw_null_topology(display);
            return Ok(());
        }
        if self.faces.borrow().is_empty() || self.edges.borrow().is_empty() {
            self.update_display_data(out)?;
        }
        if self.hlr.get().with_hlr {
            return self.display_hidden_lines(display);
        }
        let mut halt = false;
        self.draw_faces(display, &mut halt)?;
        self.draw_edges(display, &mut halt, out)?;
        self.draw_vertices(display, &mut halt)
    }
    fn draw_null_topology(&self, display: &mut RasterDisplay<'_>) {
        display.set_color(self.colors.connected);
        display.draw_string_3d(
            martensite_tkmath::gp::Point3::new(0.0, 0.0, 0.0),
            "Null Shape",
        );
    }
}

/// `TopologyColorOrientation`: red FORWARD, blue REVERSED, pink EXTERNAL, orange
/// INTERNAL.
///
/// ```
/// use martensite_geometry_raster::color_orientation;
/// use martensite_raster_core::PaletteKind;
/// use martensite_tkg3d::top_abs::Orientation;
/// assert_eq!(color_orientation(Orientation::Reversed).id(), PaletteKind::Bleu);
/// ```
pub fn color_orientation(orientation: Orientation) -> PaletteColor {
    PaletteColor::new(match orientation {
        Orientation::Forward => PaletteKind::Rouge,
        Orientation::Reversed => PaletteKind::Bleu,
        Orientation::External => PaletteKind::Rose,
        Orientation::Internal => PaletteKind::Orange,
    })
}

impl TopologyRendering {
    /// Returns the configured colors; `let colors = rendering.colors();`.
    pub fn colors(&self) -> ShapeColors {
        self.colors
    }
    /// Returns the finite stand-in for infinite geometry; `let size = rendering.infinity_size();`.
    pub fn infinity_size(&self) -> f64 {
        self.size
    }
}
