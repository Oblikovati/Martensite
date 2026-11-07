//! `RwMeshFaceIterator`: the triangulated faces of a shape or a document label.
use super::{RwMeshShapeIterator, RwMeshShapeIteratorBase};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::brep_lprop::BRepLPropSLProps;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkmath::gp::{self, Direction3, Point2, Point3, TransformForm};
use martensite_tkmath::poly::{Triangle, Triangulation, mesh_purpose};
use martensite_tkmath::toploc::Location;
use martensite_tkxcaf::xcaf_prs::XCAFPrsStyle;

/// Iterates the faces with a triangulation, with a label's colors when asked.
///
/// ```no_run
/// use martensite_tkrwmesh::rw_mesh::{RwMeshFaceIterator, RwMeshShapeIterator as _};
/// # fn run(shape: &martensite_tkbrep::topods::Shape) -> Result<(), martensite_tkernel::standard::Failure> {
/// let mut faces = RwMeshFaceIterator::with_shape(shape, &Default::default())?;
/// while faces.more() {
///     println!("{} triangles", faces.nb_triangles());
///     faces.next()?;
/// }
/// # Ok(()) }
/// ```
pub struct RwMeshFaceIterator {
    base: RwMeshShapeIteratorBase,
    face: Face,
    shape: Shape,
    poly_triang: Handle<Triangulation>,
    adaptor: Option<BRepAdaptorSurface>,
    has_normals: bool,
    is_mirrored: bool,
}

impl RwMeshFaceIterator {
    fn started(base: RwMeshShapeIteratorBase) -> Result<Self, Failure> {
        let mut faces = Self {
            base,
            face: Face::default(),
            shape: Shape::default(),
            poly_triang: Handle::default(),
            adaptor: None,
            has_normals: false,
            is_mirrored: false,
        };
        RwMeshShapeIterator::next(&mut faces)?;
        Ok(faces)
    }
    /// Over the faces of a label; `RwMeshFaceIterator::with_label(&label, &location, true, &style)?`.
    pub fn with_label(
        label: &TdfLabel,
        location: &Location,
        to_map_colors: bool,
        style: &XCAFPrsStyle,
    ) -> Result<Self, Failure> {
        Self::started(RwMeshShapeIteratorBase::with_label(
            label,
            location,
            ShapeEnum::Face,
            ShapeEnum::Shape,
            to_map_colors,
            style,
        )?)
    }
    /// Over the faces of a shape; `RwMeshFaceIterator::with_shape(&shape, &style)?`.
    pub fn with_shape(shape: &Shape, style: &XCAFPrsStyle) -> Result<Self, Failure> {
        Self::started(RwMeshShapeIteratorBase::with_shape(
            shape,
            ShapeEnum::Face,
            ShapeEnum::Shape,
            style,
        )?)
    }
    /// The current face; `faces.face()`.
    pub fn face(&self) -> &Face {
        &self.face
    }
    /// The current triangulation; `faces.triangulation()`.
    pub fn triangulation(&self) -> &Handle<Triangulation> {
        &self.poly_triang
    }
    /// True when the current face has no mesh data; `faces.is_empty_mesh()`.
    pub fn is_empty_mesh(&self) -> bool {
        RwMeshShapeIterator::is_empty(self)
    }
    /// The style of the current face; `faces.face_style()`.
    pub fn face_style(&self) -> &XCAFPrsStyle {
        &self.base.style
    }
    /// True when the current face has a color; `faces.has_face_color()`.
    pub fn has_face_color(&self) -> bool {
        self.base.has_color
    }
    /// The color of the current face; `faces.face_color()`.
    pub fn face_color(&self) -> martensite_tkernel::quantity::ColorRgba {
        self.base.color
    }
    /// The triangle count of the current face, zero past the end; `faces.nb_triangles()`.
    pub fn nb_triangles(&self) -> i32 {
        self.poly_triang
            .get()
            .map_or(0, Triangulation::nb_triangles)
    }
    /// A triangle with the face orientation and the mirror applied; `faces.triangle_oriented(1)?`.
    pub fn triangle_oriented(&self, element: i32) -> Result<Triangle, Failure> {
        let triangle = self.triangle(element)?;
        let reversed = self.face.orientation() == Orientation::Reversed;
        if reversed != self.is_mirrored {
            let (n1, n2, n3) = triangle.get();
            return Ok(Triangle::new(n1, n3, n2));
        }
        Ok(triangle)
    }
    /// True when normals can be read; `faces.has_normals()`.
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }
    /// True when the triangulation has UV nodes; `faces.has_tex_coords()`.
    pub fn has_tex_coords(&self) -> bool {
        self.poly_triang
            .get()
            .is_some_and(Triangulation::has_uv_nodes)
    }
    /// A node's normal with transformation and orientation applied; `faces.normal_transformed(1)?`.
    pub fn normal_transformed(&self, node: i32) -> Result<Direction3, Failure> {
        let mut normal = self.normal(node)?;
        if self.base.trsf.form() != TransformForm::Identity {
            normal.transform(self.base.trsf);
        }
        if self.face.orientation() == Orientation::Reversed {
            normal.reverse();
        }
        Ok(normal)
    }
    /// A node's texture coordinates, the origin without UV nodes; `faces.node_tex_coord(1)?`.
    pub fn node_tex_coord(&self, node: i32) -> Result<Point2, Failure> {
        match self.poly_triang.get().filter(|mesh| mesh.has_uv_nodes()) {
            Some(mesh) => mesh.uv_node(node),
            None => Ok(Point2::default()),
        }
    }
    /// A node's normal without transformation; +Z when it is undefined; `faces.normal(1)?`.
    pub fn normal(&self, node: i32) -> Result<Direction3, Failure> {
        let Some(mesh) = self.poly_triang.get() else {
            return Ok(gp::dz());
        };
        if mesh.has_normals() {
            let normal = mesh.normal_vec3(node)?;
            let is_zero = normal.x == 0.0 && normal.y == 0.0 && normal.z == 0.0;
            if is_zero {
                return Ok(gp::dz());
            }
            return Direction3::new(
                f64::from(normal.x),
                f64::from(normal.y),
                f64::from(normal.z),
            );
        }
        if !(self.has_normals && mesh.has_uv_nodes()) {
            return Ok(gp::dz());
        }
        self.surface_normal(mesh.uv_node(node)?)
    }
    fn surface_normal(&self, uv: Point2) -> Result<Direction3, Failure> {
        let Some(adaptor) = &self.adaptor else {
            return Ok(gp::dz());
        };
        let mut props = BRepLPropSLProps::new(adaptor, 1, 1e-12)?;
        props.set_parameters(uv.x(), uv.y())?;
        if props.is_normal_defined()? {
            props.normal()
        } else {
            Ok(gp::dz())
        }
    }
    /// The triangle without orientation; `faces.triangle(1)?`.
    pub fn triangle(&self, element: i32) -> Result<Triangle, Failure> {
        self.mesh()?.triangle(element)
    }
    fn mesh(&self) -> Result<&Triangulation, Failure> {
        self.poly_triang.get().ok_or_else(|| {
            Failure::with_type(
                "Standard_NullObject",
                "RwMeshFaceIterator: no current triangulation; expected more() to be true",
                "",
            )
        })
    }
    fn reset_face(&mut self) {
        self.poly_triang = Handle::default();
        self.face = Face::default();
        self.shape = Shape::default();
        self.has_normals = false;
        self.base.reset_shape();
    }
    fn init_face(&mut self) -> Result<(), Failure> {
        self.has_normals = false;
        self.is_mirrored = self.base.trsf.vectorial_part().determinant() < 0.0;
        let (has_normals, has_uv) = self.poly_triang.get().map_or((false, false), |mesh| {
            (mesh.has_normals(), mesh.has_uv_nodes())
        });
        self.has_normals = has_normals;
        self.adaptor = None;
        if has_uv && !has_normals {
            self.init_surface()?;
        }
        let shape = self.shape.clone();
        self.base.init_shape(&shape);
        Ok(())
    }
    fn init_surface(&mut self) -> Result<(), Failure> {
        let mut forward = self.face.oriented(Orientation::Forward);
        forward.set_location(&Location::default(), true)?;
        let forward = topods::face(&forward)?;
        if BRepTool::surface(&forward)?.is_some() {
            self.adaptor = Some(BRepAdaptorSurface::with_face(&forward, false)?);
            self.has_normals = true;
        }
        Ok(())
    }
}

impl RwMeshShapeIterator for RwMeshFaceIterator {
    fn iterator_base(&self) -> &RwMeshShapeIteratorBase {
        &self.base
    }
    fn shape(&self) -> &Shape {
        &self.shape
    }
    fn more(&self) -> bool {
        self.poly_triang.is_some()
    }
    fn next(&mut self) -> Result<(), Failure> {
        while self.base.iter.more() {
            self.shape = self.base.iter.current().clone();
            self.face = topods::face(&self.shape)?;
            let (mesh, location) = BRepTool::triangulation(&self.face, mesh_purpose::NONE)?;
            self.poly_triang = mesh;
            self.base.location = location;
            self.base.trsf = self.base.location.transformation();
            let triangles = self
                .poly_triang
                .get()
                .map_or(0, Triangulation::nb_triangles);
            if triangles == 0 {
                self.reset_face();
                self.base.iter.next()?;
                continue;
            }
            self.init_face()?;
            self.base.iter.next()?;
            return Ok(());
        }
        self.reset_face();
        Ok(())
    }
    fn is_empty(&self) -> bool {
        self.poly_triang
            .get()
            .is_none_or(|mesh| mesh.nb_nodes() < 1 && mesh.nb_triangles() < 1)
    }
    fn elem_lower(&self) -> i32 {
        1
    }
    fn elem_upper(&self) -> i32 {
        self.nb_triangles()
    }
    fn nb_nodes(&self) -> i32 {
        self.poly_triang.get().map_or(0, Triangulation::nb_nodes)
    }
    fn node_lower(&self) -> i32 {
        1
    }
    fn node_upper(&self) -> i32 {
        self.nb_nodes()
    }
    fn node(&self, node: i32) -> Result<Point3, Failure> {
        self.mesh()?.node(node)
    }
}
