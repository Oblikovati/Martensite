//! `RwObjTriangulationReader`: an OBJ file into triangulations and shapes.
use super::{RwObjMaterial, RwObjReader, RwObjReaderBase, RwObjSubMesh, RwObjSubMeshReason};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{self, Compound, Face, Shape, ShapeIterator};
use martensite_tkernel::ncollection::{Vec2, Vec3};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkmath::poly::{Triangle, Triangulation};

/// Takes the shapes a reader makes and their attributes.
pub trait RwObjShapeReceiver {
    /// Registers a shape; a root shape is a free shape;
    /// `receiver.bind_named_shape(&shape, &name, None, false)?`.
    fn bind_named_shape(
        &mut self,
        shape: &Shape,
        name: &AsciiString,
        material: Option<&RwObjMaterial>,
        is_root_shape: bool,
    ) -> Result<(), Failure>;
}

/// Reads an OBJ file into a compound of objects, of groups, of faces with a triangulation each,
/// or into one triangulation; `RwObjTriangulationReader::new()`.
pub struct RwObjTriangulationReader<'r> {
    base: RwObjReaderBase,
    nodes: Vec<Point3>,
    normals: Vec<Vec3<f32>>,
    nodes_uv: Vec<Vec2<f32>>,
    triangles: Vec<Triangle>,
    shape_receiver: Option<&'r mut dyn RwObjShapeReceiver>,
    result_shape: Shape,
    last_object_shape: Shape,
    last_group_shape: Shape,
    last_group_name: AsciiString,
    last_face_material: AsciiString,
    to_create_shapes: bool,
}

impl Default for RwObjTriangulationReader<'_> {
    fn default() -> Self {
        Self::new()
    }
}

/// Adds a sub-shape: expanded, the first one is the parent itself, otherwise the parent is a compound.
fn add_sub_shape(
    parent: &mut Shape,
    sub_shape: &Shape,
    to_expand_compound: bool,
) -> Result<bool, Failure> {
    if sub_shape.is_null() {
        return Ok(false);
    }
    if parent.is_null() && to_expand_compound {
        *parent = sub_shape.clone();
        return Ok(true);
    }
    let builder = BRepBuilder::default();
    let is_compound = !parent.is_null() && parent.shape_type()? == ShapeEnum::Compound;
    let mut compound = if is_compound {
        topods::compound(parent)?
    } else {
        let mut made = Compound::default();
        builder.make_compound(&mut made);
        if !parent.is_null() {
            builder.add(&mut made, parent)?;
        }
        made
    };
    builder.add(&mut compound, sub_shape)?;
    *parent = compound.into();
    Ok(true)
}

impl<'r> RwObjTriangulationReader<'r> {
    /// A reader creating shapes; `RwObjTriangulationReader::new()`.
    pub fn new() -> Self {
        Self {
            base: RwObjReaderBase::default(),
            nodes: Vec::new(),
            normals: Vec::new(),
            nodes_uv: Vec::new(),
            triangles: Vec::new(),
            shape_receiver: None,
            result_shape: Shape::default(),
            last_object_shape: Shape::default(),
            last_group_shape: Shape::default(),
            last_group_name: AsciiString::default(),
            last_face_material: AsciiString::default(),
            to_create_shapes: true,
        }
    }
    /// Sets whether shapes are created rather than one triangulation;
    /// `reader.set_create_shapes(false)`.
    pub fn set_create_shapes(&mut self, to_create_shapes: bool) {
        self.to_create_shapes = to_create_shapes;
    }
    /// Sets the receiver of shapes; `reader.set_shape_receiver(Some(&mut receiver))`.
    pub fn set_shape_receiver(&mut self, receiver: Option<&'r mut dyn RwObjShapeReceiver>) {
        self.shape_receiver = receiver;
    }
    /// The triangulation of the data collected, `None` without triangles;
    /// `reader.get_triangulation()?`.
    pub fn get_triangulation(&self) -> Result<Option<Handle<Triangulation>>, Failure> {
        if self.triangles.is_empty() {
            return Ok(None);
        }
        let has_uv = self.nodes.len() == self.nodes_uv.len();
        let mesh = Triangulation::with_sizes(
            self.nodes.len() as i32,
            self.triangles.len() as i32,
            has_uv,
            false,
        )?;
        for (index, node) in self.nodes.iter().enumerate() {
            mesh.set_node(index as i32 + 1, *node)?;
        }
        if has_uv {
            for (index, uv) in self.nodes_uv.iter().enumerate() {
                mesh.set_uv_node(
                    index as i32 + 1,
                    Point2::new(f64::from(uv.x), f64::from(uv.y)),
                )?;
            }
        }
        if self.nodes.len() == self.normals.len() {
            self.fill_normals(&mesh)?;
        }
        for (index, triangle) in self.triangles.iter().enumerate() {
            mesh.set_triangle(index as i32 + 1, *triangle)?;
        }
        Ok(Some(Handle::new(mesh)))
    }
    /// Tiny normals become +Z, and when all are tiny there are no normals.
    fn fill_normals(&self, mesh: &Triangulation) -> Result<(), Failure> {
        mesh.add_normals()?;
        let mut invalid = 0;
        for (index, normal) in self.normals.iter().enumerate() {
            let square = normal.x * normal.x + normal.y * normal.y + normal.z * normal.z;
            let value = if square > 0.001 {
                *normal
            } else {
                invalid += 1;
                Vec3::new(0.0, 0.0, 1.0)
            };
            mesh.set_normal_vec3(index as i32 + 1, value)?;
        }
        if invalid == self.nodes.len() {
            mesh.remove_normals();
        }
        Ok(())
    }
    /// The result: the face of one triangulation, or the shapes, with an unnamed single object
    /// unwrapped; `reader.result_shape()?`.
    pub fn result_shape(&self) -> Result<Shape, Failure> {
        if !self.to_create_shapes {
            let Some(mesh) = self.get_triangulation()? else {
                return Ok(Shape::default());
            };
            let mut face = Face::default();
            BRepBuilder::default().make_face_with_triangulation(&mut face, &mesh)?;
            return Ok(face.into());
        }
        let is_single = !self.result_shape.is_null()
            && self.result_shape.shape_type()? == ShapeEnum::Compound
            && self.result_shape.nb_children() == 1
            && self.base.active_sub_mesh.object.is_empty();
        if is_single {
            return Ok(ShapeIterator::new(&self.result_shape, true, true)
                .value()?
                .clone());
        }
        Ok(self.result_shape.clone())
    }
    fn group_material(&self) -> Result<Option<RwObjMaterial>, Failure> {
        let is_face = self.last_group_shape.shape_type()? == ShapeEnum::Face;
        if is_face && !self.last_face_material.is_empty() {
            return Ok(self.base.materials.seek(&self.last_face_material).cloned());
        }
        Ok(None)
    }
    /// Moves the current group into the current object and tells the receiver.
    fn flush_group(&mut self) -> Result<(), Failure> {
        if add_sub_shape(&mut self.last_object_shape, &self.last_group_shape, false)? {
            let material = self.group_material()?;
            let group = self.last_group_shape.clone();
            let name = self.last_group_name.clone();
            if let Some(receiver) = self.shape_receiver.as_deref_mut() {
                receiver.bind_named_shape(&group, &name, material.as_ref(), false)?;
            }
        }
        self.last_group_shape = Shape::default();
        Ok(())
    }
    fn flush_face(
        &mut self,
        mesh: &Handle<Triangulation>,
        sub_mesh: &RwObjSubMesh,
    ) -> Result<(), Failure> {
        self.nodes.clear();
        self.nodes_uv.clear();
        self.normals.clear();
        self.triangles.clear();
        if sub_mesh.group != self.last_group_name {
            self.flush_group()?;
            self.last_group_name = sub_mesh.group.clone();
        }
        let mut face = Face::default();
        BRepBuilder::default().make_face_with_triangulation(&mut face, mesh)?;
        let shape: Shape = face.into();
        add_sub_shape(&mut self.last_group_shape, &shape, true)?;
        self.last_face_material = sub_mesh.material.clone();
        let material = self.base.materials.seek(&sub_mesh.material).cloned();
        if let Some(receiver) = self.shape_receiver.as_deref_mut() {
            receiver.bind_named_shape(&shape, &AsciiString::default(), material.as_ref(), false)?;
        }
        Ok(())
    }
    fn flush_object(&mut self, sub_mesh: &RwObjSubMesh) -> Result<(), Failure> {
        self.flush_group()?;
        self.last_group_name = AsciiString::default();
        if add_sub_shape(&mut self.result_shape, &self.last_object_shape, false)? {
            let object = self.last_object_shape.clone();
            if let Some(receiver) = self.shape_receiver.as_deref_mut() {
                receiver.bind_named_shape(&object, &sub_mesh.object, None, true)?;
            }
        }
        self.last_object_shape = Shape::default();
        Ok(())
    }
}

impl RwObjReader for RwObjTriangulationReader<'_> {
    fn obj_base(&self) -> &RwObjReaderBase {
        &self.base
    }
    fn obj_base_mut(&mut self) -> &mut RwObjReaderBase {
        &mut self.base
    }
    fn add_mesh(
        &mut self,
        mesh: &RwObjSubMesh,
        reason: RwObjSubMeshReason,
    ) -> Result<bool, Failure> {
        if !self.to_create_shapes {
            return Ok(false);
        }
        if let Some(triangulation) = self.get_triangulation()? {
            self.flush_face(&triangulation, mesh)?;
        }
        if reason == RwObjSubMeshReason::NewObject {
            self.flush_object(mesh)?;
        }
        Ok(true)
    }
    fn get_node(&self, index: i32) -> Point3 {
        self.nodes[(index - 1) as usize]
    }
    fn add_node(&mut self, point: Point3) -> i32 {
        self.nodes.push(point);
        self.nodes.len() as i32
    }
    fn set_node_normal(&mut self, index: i32, normal: Vec3<f32>) {
        let slot = (index - 1) as usize;
        if self.normals.len() <= slot {
            self.normals.resize(slot + 1, Vec3::new(0.0, 0.0, 0.0));
        }
        self.normals[slot] = normal;
    }
    fn set_node_uv(&mut self, index: i32, uv: Vec2<f32>) {
        let slot = (index - 1) as usize;
        if self.nodes_uv.len() <= slot {
            self.nodes_uv.resize(slot + 1, Vec2::new(0.0, 0.0));
        }
        self.nodes_uv[slot] = uv;
    }
    fn add_element(&mut self, n1: i32, n2: i32, n3: i32, n4: i32) {
        self.triangles.push(Triangle::new(n1, n2, n3));
        if n4 != -1 {
            self.triangles.push(Triangle::new(n1, n3, n4));
        }
    }
}
