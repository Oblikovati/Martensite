//! `RwObjCafReader`: an OBJ file into an exchange document.
use super::{RwObjMaterial, RwObjReader as _, RwObjShapeReceiver, RwObjTriangulationReader};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
use martensite_tkrwmesh::rw_mesh::{
    RwMeshCafReader, RwMeshCafReaderBase, RwMeshCoordinateSystem, RwMeshInputStream,
    RwMeshNodeAttributes,
};
use martensite_tkservice::image::ImageTexture;
use martensite_tkxcaf::xcaf_doc::{XCAFDocVisMaterial, XCAFDocVisMaterialCommon};

/// Reads OBJ into shapes and a document; the file's unit is undefined and it is taken as Y-up,
/// the most common convention, since the format leaves both open.
///
/// ```no_run
/// use martensite_tkdeobj::rw_obj::RwObjCafReader;
/// use martensite_tkernel::message::ProgressRange;
/// use martensite_tkernel::tcollection::AsciiString;
/// use martensite_tkrwmesh::rw_mesh::RwMeshCafReader as _;
/// let mut reader = RwObjCafReader::new();
/// reader.perform(&AsciiString::new("cube.obj"), &ProgressRange::default()).unwrap();
/// assert!(!reader.single_shape().unwrap().is_null());
/// ```
pub struct RwObjCafReader {
    transient: StandardTransient,
    base: RwMeshCafReaderBase,
    obj_material_map: DataMap<AsciiString, Handle<XCAFDocVisMaterial>>,
    is_single_precision: bool,
}

impl_transient!(RwObjCafReader, "RwObjCafReader", dyn RwMeshCafReader, transient, interfaces: [dyn RwMeshCafReader]);

impl Default for RwObjCafReader {
    fn default() -> Self {
        Self::new()
    }
}

impl RwObjCafReader {
    /// A reader of Y-up files; `RwObjCafReader::new()`.
    pub fn new() -> Self {
        let mut base = RwMeshCafReaderBase::default();
        base.coord_sys_converter
            .set_input_coordinate_system_kind(RwMeshCoordinateSystem::GLTF);
        Self {
            transient: StandardTransient::default(),
            base,
            obj_material_map: DataMap::default(),
            is_single_precision: false,
        }
    }
    /// True with single precision vertices, false by default; `reader.is_single_precision()`.
    pub fn is_single_precision(&self) -> bool {
        self.is_single_precision
    }
    /// Sets the precision of vertices; `reader.set_single_precision(true)`.
    pub fn set_single_precision(&mut self, is_single_precision: bool) {
        self.is_single_precision = is_single_precision;
    }
    /// A reader for the file, ready but for the receiver; `reader.create_reader_context()`.
    pub fn create_reader_context<'r>(&self) -> RwObjTriangulationReader<'r> {
        RwObjTriangulationReader::new()
    }
    /// The document material of an OBJ material, made once per name, the unique key in OBJ.
    fn document_material(
        &mut self,
        material: &RwObjMaterial,
    ) -> Result<Handle<XCAFDocVisMaterial>, Failure> {
        if let Some(known) = self.obj_material_map.seek(&material.name) {
            return Ok(known.clone());
        }
        let mut common = XCAFDocVisMaterialCommon {
            is_defined: true,
            ambient_color: material.ambient_color,
            diffuse_color: material.diffuse_color,
            specular_color: material.specular_color,
            shininess: material.shininess,
            transparency: material.transparency,
            ..XCAFDocVisMaterialCommon::default()
        };
        if !material.diffuse_texture.is_empty() {
            common.diffuse_texture =
                Handle::new(ImageTexture::new(&material.diffuse_texture)).cast();
        }
        let document_material = XCAFDocVisMaterial::new();
        document_material.set_common_material(&common)?;
        document_material.set_raw_name(&Handle::new(HAsciiString::from(material.name.clone())));
        let handle = Handle::new(document_material);
        self.obj_material_map
            .bind(material.name.clone(), handle.clone());
        Ok(handle)
    }
}

impl RwObjShapeReceiver for RwObjCafReader {
    fn bind_named_shape(
        &mut self,
        shape: &Shape,
        name: &AsciiString,
        material: Option<&RwObjMaterial>,
        is_root_shape: bool,
    ) -> Result<(), Failure> {
        if shape.is_null() {
            return Ok(());
        }
        let mut attributes = RwMeshNodeAttributes {
            name: name.clone(),
            ..Default::default()
        };
        if let Some(material) = material {
            // A material is assigned, not a color.
            attributes
                .style
                .set_material(&self.document_material(material)?);
        }
        self.base.attrib_map.bind(shape.clone(), attributes);
        if is_root_shape {
            self.base.root_shapes.push(shape.clone());
        }
        Ok(())
    }
}

impl RwMeshCafReader for RwObjCafReader {
    fn caf_base(&self) -> &RwMeshCafReaderBase {
        &self.base
    }
    fn caf_base_mut(&mut self) -> &mut RwMeshCafReaderBase {
        &mut self.base
    }
    fn perform_mesh(
        &mut self,
        stream: &mut dyn RwMeshInputStream,
        file: &AsciiString,
        progress: &ProgressRange<'_>,
        to_probe: bool,
    ) -> Result<bool, Failure> {
        let limit = match self.base.memory_limit_mib {
            -1 => usize::MAX,
            mib => mib as usize * 1024 * 1024,
        };
        let converter = self.base.coord_sys_converter.clone();
        let mut context = self.create_reader_context();
        context.set_single_precision(self.is_single_precision);
        context.set_create_shapes(true);
        context.set_transformation(&converter);
        context.set_memory_limit(limit);
        context.set_shape_receiver(Some(self));
        let is_done = if to_probe {
            context.probe_stream(stream, file, progress)?
        } else {
            context.read_stream(stream, file, progress)?
        };
        let comments = context.file_comments().clone();
        let external_files: Vec<AsciiString> = context.external_files().iter().cloned().collect();
        drop(context);
        if !comments.is_empty() {
            self.base
                .metadata
                .add(AsciiString::new("Comments"), comments);
        }
        for external in external_files {
            self.base.external_files.add(external);
        }
        Ok(is_done)
    }
}
