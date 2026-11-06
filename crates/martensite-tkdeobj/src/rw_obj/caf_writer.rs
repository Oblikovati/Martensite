//! `RwObjCafWriter`: an exchange document into an OBJ file and its MTL file.
use super::{RwObjObjMaterialMap, RwObjObjWriterContext};
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::{LazyProgressScope, Message, ProgressRange};
use martensite_tkernel::ncollection::{IndexedDataMap, Map, Vec2, Vec3};
use martensite_tkernel::osd::CadPath;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::Xyz;
use martensite_tkmath::toploc::Location;
use martensite_tkrwmesh::rw_mesh::{
    RwMesh, RwMeshCoordinateSystemConverter, RwMeshFaceIterator, RwMeshMaterialMap as _,
    RwMeshShapeIterator as _,
};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use martensite_tkxcaf::xcaf_prs::{
    XCAF_PRS_DOCUMENT_EXPLORER_FLAGS_ONLY_LEAF_NODES, XCAFPrsDocumentExplorer, XCAFPrsStyle,
};

/// A patch of steps between progress reports, about 100 KiB of text.
const PATCH_STEP: f64 = 2048.0;

fn send_fail(text: &str) {
    Message::send_fail_text(&AsciiString::new(text));
}

fn xyz_to_vec(point: Xyz) -> Vec3<f32> {
    Vec3::new(point.x() as f32, point.y() as f32, point.z() as f32)
}

/// What a document holds to write.
#[derive(Default)]
struct FaceSummary {
    nb_nodes: i32,
    nb_elems: i32,
    nb_steps: f64,
    to_create_mat_file: bool,
}

/// Writes a document: triangulations must be computed beforehand.
///
/// ```no_run
/// use martensite_tkdeobj::rw_obj::RwObjCafWriter;
/// use martensite_tkernel::message::ProgressRange;
/// use martensite_tkernel::ncollection::IndexedDataMap;
/// use martensite_tkernel::tcollection::AsciiString;
/// # fn run(document: &martensite_tkernel::standard::Handle<martensite_tklcaf::tdoc_std::TDocStdDocument>) {
/// let mut writer = RwObjCafWriter::new(&AsciiString::new("model.obj"));
/// let information = IndexedDataMap::default();
/// assert!(writer.perform(document, &information, &ProgressRange::default()).unwrap());
/// # }
/// ```
pub struct RwObjCafWriter {
    transient: StandardTransient,
    file: AsciiString,
    cs_trsf: RwMeshCoordinateSystemConverter,
    default_style: XCAFPrsStyle,
}

impl_transient!(
    RwObjCafWriter,
    "RwObjCafWriter",
    StandardTransient,
    transient
);

impl RwObjCafWriter {
    /// A writer of the named file; `RwObjCafWriter::new(&file)`.
    pub fn new(file: &AsciiString) -> Self {
        Self {
            transient: StandardTransient::default(),
            file: file.clone(),
            cs_trsf: RwMeshCoordinateSystemConverter::new(),
            default_style: XCAFPrsStyle::default(),
        }
    }
    /// The conversion to the file's system; `writer.coordinate_system_converter()`.
    pub fn coordinate_system_converter(&self) -> &RwMeshCoordinateSystemConverter {
        &self.cs_trsf
    }
    /// The conversion to the file's system, to change; `writer.change_coordinate_system_converter()`.
    pub fn change_coordinate_system_converter(&mut self) -> &mut RwMeshCoordinateSystemConverter {
        &mut self.cs_trsf
    }
    /// Sets the conversion to the file's system; `writer.set_coordinate_system_converter(&converter)`.
    pub fn set_coordinate_system_converter(&mut self, converter: &RwMeshCoordinateSystemConverter) {
        self.cs_trsf = converter.clone();
    }
    /// The material of nodes with only a color; `writer.default_style()`.
    pub fn default_style(&self) -> &XCAFPrsStyle {
        &self.default_style
    }
    /// Sets the material of nodes with only a color; `writer.set_default_style(&style)`.
    pub fn set_default_style(&mut self, style: &XCAFPrsStyle) {
        self.default_style = style.clone();
    }
    /// Writes the free shapes of a document; false on a writing failure;
    /// `writer.perform(&document, &information, &progress)?`.
    pub fn perform(
        &mut self,
        document: &Handle<TDocStdDocument>,
        file_info: &IndexedDataMap<AsciiString, AsciiString>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let document_ref = document.get().ok_or_else(|| {
            Failure::with_type(
                "Standard_NullObject",
                "RwObjCafWriter: expected a document",
                "",
            )
        })?;
        let tool = XCAFDocDocumentTool::shape_tool(&document_ref.main()?)?;
        let roots = tool
            .get()
            .map_or(Ok(Vec::new()), XCAFDocShapeTool::get_free_shapes)?;
        self.perform_roots(document, &roots, None, file_info, progress)
    }
    /// Writes root labels, optionally only the nodes of a filter of explorer child identifiers;
    /// `writer.perform_roots(&document, &roots, None, &information, &progress)?`.
    pub fn perform_roots(
        &mut self,
        document: &Handle<TDocStdDocument>,
        roots: &[TdfLabel],
        label_filter: Option<&Map<AsciiString>>,
        file_info: &IndexedDataMap<AsciiString, AsciiString>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let mut unit = 1.0;
        if XCAFDocDocumentTool::get_length_unit(document, &mut unit)? {
            self.cs_trsf.set_input_length_unit(unit);
        }
        if roots.is_empty() || label_filter.is_some_and(Map::is_empty) {
            send_fail("Nothing to export into OBJ file");
            return Ok(false);
        }
        let summary = self.summarize(document, roots, label_filter)?;
        if summary.nb_nodes == 0 || summary.nb_elems == 0 {
            send_fail("No mesh data to save");
            return Ok(false);
        }
        self.write_document(document, roots, label_filter, file_info, progress, &summary)
    }
    fn explorer(
        document: &Handle<TDocStdDocument>,
        roots: &[TdfLabel],
    ) -> Result<XCAFPrsDocumentExplorer, Failure> {
        XCAFPrsDocumentExplorer::with_roots(
            document,
            roots,
            XCAF_PRS_DOCUMENT_EXPLORER_FLAGS_ONLY_LEAF_NODES,
            &XCAFPrsStyle::default(),
        )
    }
    fn summarize(
        &self,
        document: &Handle<TDocStdDocument>,
        roots: &[TdfLabel],
        label_filter: Option<&Map<AsciiString>>,
    ) -> Result<FaceSummary, Failure> {
        let mut summary = FaceSummary::default();
        let mut explorer = Self::explorer(document, roots)?;
        while explorer.more() {
            let node = explorer.current();
            if label_filter.is_none_or(|filter| filter.contains(&node.id)) {
                let mut faces = RwMeshFaceIterator::with_label(
                    &node.ref_label,
                    &node.location,
                    true,
                    &node.style,
                )?;
                while faces.more() {
                    if !self.to_skip_face_mesh(&faces) {
                        self.add_face_info(&faces, &mut summary);
                    }
                    faces.next()?;
                }
            }
            explorer.next()?;
        }
        Ok(summary)
    }
    /// True when a face mesh is left out, for being empty; `writer.to_skip_face_mesh(&faces)`.
    pub fn to_skip_face_mesh(&self, faces: &RwMeshFaceIterator) -> bool {
        faces.is_empty_mesh()
    }
    fn add_face_info(&self, faces: &RwMeshFaceIterator, summary: &mut FaceSummary) {
        summary.nb_nodes += faces.nb_nodes();
        summary.nb_elems += faces.nb_triangles();
        summary.nb_steps += f64::from(faces.nb_nodes() + faces.nb_triangles());
        if faces.has_normals() {
            summary.nb_steps += f64::from(faces.nb_nodes());
        }
        if faces.has_tex_coords() {
            summary.nb_steps += f64::from(faces.nb_nodes());
        }
        summary.to_create_mat_file = summary.to_create_mat_file
            || faces.has_face_color()
            || (faces.face_style().base_color_texture().is_some() && faces.has_tex_coords());
    }
    #[allow(clippy::too_many_arguments)] // The whole request of one export, passed once.
    fn write_document(
        &self,
        document: &Handle<TDocStdDocument>,
        roots: &[TdfLabel],
        label_filter: Option<&Map<AsciiString>>,
        file_info: &IndexedDataMap<AsciiString, AsciiString>,
        progress: &ProgressRange<'_>,
        summary: &FaceSummary,
    ) -> Result<bool, Failure> {
        let (folder, file_name) = CadPath::folder_and_file_from_path(self.file.as_bytes());
        let (base_name, _) = CadPath::file_name_and_extension(file_name.as_bytes());
        let material_short = AsciiString::from_raw(&[base_name.as_bytes(), b".mtl"].concat());
        let material_full =
            AsciiString::from_raw(&[folder.as_bytes(), material_short.as_bytes()].concat());
        let library = if summary.to_create_mat_file {
            material_short
        } else {
            AsciiString::default()
        };
        let mut writer = RwObjObjWriterContext::new(&self.file);
        let mut materials = RwObjObjMaterialMap::new(&material_full);
        materials.set_default_style(&self.default_style);
        if !writer.is_opened()
            || !writer.write_header(summary.nb_nodes, summary.nb_elems, &library, file_info)
        {
            return Ok(false);
        }
        let root_depth = root_depth_of(roots)?;
        let mut scope =
            LazyProgressScope::new(progress, "OBJ export", summary.nb_steps, PATCH_STEP, false);
        let is_done = self.write_nodes(
            document,
            roots,
            label_filter,
            root_depth,
            &mut writer,
            &mut materials,
            &mut scope,
        )?;
        let is_closed = writer.close();
        if is_done && !is_closed {
            let name = String::from_utf8_lossy(self.file.as_bytes()).into_owned();
            send_fail(&format!("Failed to write OBJ file\n{name}"));
            return Ok(false);
        }
        Ok(is_done && !scope.is_aborted())
    }
    #[allow(clippy::too_many_arguments)] // The state of one export, passed once.
    fn write_nodes(
        &self,
        document: &Handle<TDocStdDocument>,
        roots: &[TdfLabel],
        label_filter: Option<&Map<AsciiString>>,
        root_depth: i32,
        writer: &mut RwObjObjWriterContext,
        materials: &mut RwObjObjMaterialMap,
        scope: &mut LazyProgressScope<'_>,
    ) -> Result<bool, Failure> {
        let mut explorer = Self::explorer(document, roots)?;
        while explorer.more() && !scope.is_aborted() {
            let node = explorer.current().clone();
            if label_filter.is_none_or(|filter| filter.contains(&node.id)) {
                let name = node_path_name(&explorer, root_depth)?;
                if !self.write_shape(
                    writer,
                    materials,
                    scope,
                    &node.ref_label,
                    &node.location,
                    &node.style,
                    &name,
                )? {
                    return Ok(false);
                }
            }
            explorer.next()?;
        }
        Ok(true)
    }
    /// Writes the faces of a label, in a group of its name;
    /// `writer.write_shape(&mut context, &mut materials, &mut scope, &label, &location, &style, &name)?`.
    #[allow(clippy::too_many_arguments)] // One argument per item of the node being written.
    pub fn write_shape(
        &self,
        writer: &mut RwObjObjWriterContext,
        materials: &mut RwObjObjMaterialMap,
        scope: &mut LazyProgressScope<'_>,
        label: &TdfLabel,
        parent_trsf: &Location,
        parent_style: &XCAFPrsStyle,
        name: &AsciiString,
    ) -> Result<bool, Failure> {
        let mut to_create_group = true;
        let mut faces = RwMeshFaceIterator::with_label(label, parent_trsf, true, parent_style)?;
        while faces.more() && !scope.is_aborted() {
            if !self.to_skip_face_mesh(&faces) {
                writer.nb_faces += 1;
                let (has_normals, has_tex_coords) = (faces.has_normals(), faces.has_tex_coords());
                if writer.nb_faces != 1 {
                    to_create_group = to_create_group
                        || has_normals != writer.has_normals()
                        || has_tex_coords != writer.has_tex_coords();
                }
                writer.set_normals(has_normals);
                writer.set_tex_coords(has_tex_coords);
                if to_create_group && !writer.write_group(name) {
                    return Ok(false);
                }
                to_create_group = false;
                if !self.write_face(writer, materials, scope, &faces)? {
                    return Ok(false);
                }
            }
            faces.next()?;
        }
        Ok(true)
    }
    fn write_face(
        &self,
        writer: &mut RwObjObjWriterContext,
        materials: &mut RwObjObjMaterialMap,
        scope: &mut LazyProgressScope<'_>,
        faces: &RwMeshFaceIterator,
    ) -> Result<bool, Failure> {
        let has_material =
            faces.has_face_color() || faces.face_style().base_color_texture().is_some();
        let material = if has_material {
            materials.add_material(faces.face_style())?
        } else {
            AsciiString::default()
        };
        if &material != writer.active_material() {
            writer.write_active_material(&material);
        }
        let is_written = self.write_positions(writer, scope, faces)?
            && (!writer.has_normals() || self.write_normals(writer, scope, faces)?)
            && (!writer.has_tex_coords() || self.write_text_coords(writer, scope, faces)?)
            && self.write_indices(writer, scope, faces)?;
        if is_written {
            writer.flush_face(faces.nb_nodes());
        }
        Ok(is_written)
    }
    /// Writes the vertex positions of a face; `writer.write_positions(&mut context, &mut scope, &faces)?`.
    pub fn write_positions(
        &self,
        writer: &mut RwObjObjWriterContext,
        scope: &mut LazyProgressScope<'_>,
        faces: &RwMeshFaceIterator,
    ) -> Result<bool, Failure> {
        let mut node = faces.node_lower();
        while node <= faces.node_upper() && scope.more() {
            let mut position = Xyz::from(faces.node_transformed(node)?);
            self.cs_trsf.transform_position(&mut position);
            if !writer.write_vertex(&xyz_to_vec(position)) {
                return Ok(false);
            }
            node += 1;
            scope.next();
        }
        Ok(true)
    }
    /// Writes the vertex normals of a face; `writer.write_normals(&mut context, &mut scope, &faces)?`.
    pub fn write_normals(
        &self,
        writer: &mut RwObjObjWriterContext,
        scope: &mut LazyProgressScope<'_>,
        faces: &RwMeshFaceIterator,
    ) -> Result<bool, Failure> {
        let mut node = faces.node_lower();
        while node <= faces.node_upper() && scope.more() {
            let normal = faces.normal_transformed(node)?;
            let [x, y, z] = *normal.coordinates();
            let mut value = xyz_to_vec(Xyz::new(x, y, z));
            self.cs_trsf.transform_normal(&mut value);
            if !writer.write_normal(&value) {
                return Ok(false);
            }
            node += 1;
            scope.next();
        }
        Ok(true)
    }
    /// Writes the texture coordinates of a face;
    /// `writer.write_text_coords(&mut context, &mut scope, &faces)?`.
    pub fn write_text_coords(
        &self,
        writer: &mut RwObjObjWriterContext,
        scope: &mut LazyProgressScope<'_>,
        faces: &RwMeshFaceIterator,
    ) -> Result<bool, Failure> {
        let mut node = faces.node_lower();
        while node <= faces.node_upper() && scope.more() {
            let uv = faces.node_tex_coord(node)?;
            if !writer.write_tex_coord(&Vec2::new(uv.x() as f32, uv.y() as f32)) {
                return Ok(false);
            }
            node += 1;
            scope.next();
        }
        Ok(true)
    }
    /// Writes the triangles of a face; `writer.write_indices(&mut context, &mut scope, &faces)?`.
    pub fn write_indices(
        &self,
        writer: &mut RwObjObjWriterContext,
        scope: &mut LazyProgressScope<'_>,
        faces: &RwMeshFaceIterator,
    ) -> Result<bool, Failure> {
        let lower = faces.elem_lower();
        let mut element = lower;
        while element <= faces.elem_upper() && scope.more() {
            let (n1, n2, n3) = faces.triangle_oriented(element)?.get();
            if !writer.write_triangle(&Vec3::new(n1 - lower, n2 - lower, n3 - lower)) {
                return Ok(false);
            }
            element += 1;
            scope.next();
        }
        Ok(true)
    }
}

/// One level is skipped for a single root named after an OBJ file, so a reimported file keeps its
/// groups' names.
fn root_depth_of(roots: &[TdfLabel]) -> Result<i32, Failure> {
    let [root] = roots else {
        return Ok(0);
    };
    let mut reference = root.clone();
    XCAFDocShapeTool::get_referred_shape(root, &mut reference)?;
    Ok(i32::from(
        RwMesh::read_name_attribute(&reference)?.ends_with(".obj"),
    ))
}

/// The node's name under the names of its parents.
fn node_path_name(
    explorer: &XCAFPrsDocumentExplorer,
    root_depth: i32,
) -> Result<AsciiString, Failure> {
    let mut name = RwMesh::read_name_attribute(&explorer.current().ref_label)?;
    let mut depth = explorer.current_depth() - 1;
    while depth >= root_depth {
        let parent = RwMesh::read_name_attribute(&explorer.current_at(depth)?.ref_label)?;
        if !parent.is_empty() {
            name = AsciiString::from_raw(&[parent.as_bytes(), b"/", name.as_bytes()].concat());
        }
        depth -= 1;
    }
    Ok(name)
}
