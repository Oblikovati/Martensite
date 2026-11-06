//! `RwPlyCafWriter`: an exchange document into a PLY file.
use super::RwPlyPlyWriterContext;
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::{LazyProgressScope, Message, ProgressRange};
use martensite_tkernel::ncollection::{IndexedDataMap, Map, Vec2, Vec3, Vec4};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkmath::gp::{Point3, Xyz};
use martensite_tkmath::toploc::Location;
use martensite_tkrwmesh::rw_mesh::{
    RwMeshCoordinateSystemConverter, RwMeshFaceIterator, RwMeshShapeIterator as _,
};
use martensite_tkxcaf::xcaf_doc::{XCAFDocDocumentTool, XCAFDocShapeTool};
use martensite_tkxcaf::xcaf_prs::{
    XCAF_PRS_DOCUMENT_EXPLORER_FLAGS_ONLY_LEAF_NODES, XCAFPrsDocumentExplorer, XCAFPrsStyle,
};

/// A patch of steps between progress reports.
const PATCH_STEP: f64 = 2048.0;

fn send_fail(text: &str) {
    Message::send_fail_text(&AsciiString::new(text));
}

/// One color component as a byte, by truncation as the format's files have always held it.
fn component_byte(value: f32) -> u8 {
    (value * 255.0) as i32 as u8
}

/// The color of the current face as bytes, white without a color; `face_color_bytes(&faces)`.
pub fn face_color_bytes(faces: &RwMeshFaceIterator) -> Vec4<u8> {
    if !faces.has_face_color() {
        return Vec4::new(255, 255, 255, 255);
    }
    let color = faces.face_color().to_vec4();
    Vec4::new(
        component_byte(color.x),
        component_byte(color.y),
        component_byte(color.z),
        component_byte(color.w),
    )
}

/// What a document holds to write.
#[derive(Default)]
struct FaceSummary {
    nb_nodes: i32,
    nb_elems: i32,
    nb_steps: f64,
}

/// One export: what `perform_roots` was given.
struct Export<'a> {
    document: &'a Handle<TDocStdDocument>,
    roots: &'a [TdfLabel],
    label_filter: Option<&'a Map<AsciiString>>,
}

impl Export<'_> {
    fn explorer(&self) -> Result<XCAFPrsDocumentExplorer, Failure> {
        XCAFPrsDocumentExplorer::with_roots(
            self.document,
            self.roots,
            XCAF_PRS_DOCUMENT_EXPLORER_FLAGS_ONLY_LEAF_NODES,
            &XCAFPrsStyle::default(),
        )
    }
    fn accepts(&self, id: &AsciiString) -> bool {
        self.label_filter.is_none_or(|filter| filter.contains(id))
    }
}

/// Writes a document: triangulations must be computed beforehand.
///
/// ```no_run
/// use martensite_tkdeply::rw_ply::RwPlyCafWriter;
/// use martensite_tkernel::message::ProgressRange;
/// use martensite_tkernel::ncollection::IndexedDataMap;
/// use martensite_tkernel::tcollection::AsciiString;
/// # fn run(document: &martensite_tkernel::standard::Handle<martensite_tklcaf::tdoc_std::TDocStdDocument>) {
/// let mut writer = RwPlyCafWriter::new(&AsciiString::new("model.ply"));
/// writer.set_face_id(true);
/// let information = IndexedDataMap::default();
/// assert!(writer.perform(document, &information, &ProgressRange::default()).unwrap());
/// # }
/// ```
pub struct RwPlyCafWriter {
    transient: StandardTransient,
    file: AsciiString,
    cs_trsf: RwMeshCoordinateSystemConverter,
    default_style: XCAFPrsStyle,
    is_double_prec: bool,
    has_normals: bool,
    has_colors: bool,
    has_tex_coords: bool,
    has_part_id: bool,
    has_face_id: bool,
}
impl_transient!(
    RwPlyCafWriter,
    "RwPlyCafWriter",
    StandardTransient,
    transient
);

impl RwPlyCafWriter {
    /// A writer of the named file, with normals, colors and part ids; `RwPlyCafWriter::new(&file)`.
    pub fn new(file: &AsciiString) -> Self {
        Self {
            transient: StandardTransient::default(),
            file: file.clone(),
            cs_trsf: RwMeshCoordinateSystemConverter::new(),
            default_style: XCAFPrsStyle::default(),
            is_double_prec: false,
            has_normals: true,
            has_colors: true,
            has_tex_coords: false,
            has_part_id: true,
            has_face_id: false,
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
    /// True when positions are written as doubles; `writer.is_double_precision()`.
    pub fn is_double_precision(&self) -> bool {
        self.is_double_prec
    }
    /// Sets whether positions are written as doubles; `writer.set_double_precision(true)`.
    pub fn set_double_precision(&mut self, is_double: bool) {
        self.is_double_prec = is_double;
    }
    /// True when normals are written; `writer.has_normals()`.
    pub fn has_normals(&self) -> bool {
        self.has_normals
    }
    /// Sets whether normals are written; `writer.set_normals(false)`.
    pub fn set_normals(&mut self, has_normals: bool) {
        self.has_normals = has_normals;
    }
    /// True when texture coordinates are written; `writer.has_tex_coords()`.
    pub fn has_tex_coords(&self) -> bool {
        self.has_tex_coords
    }
    /// Sets whether texture coordinates are written; `writer.set_tex_coords(true)`.
    pub fn set_tex_coords(&mut self, has_tex_coords: bool) {
        self.has_tex_coords = has_tex_coords;
    }
    /// True when point colors are written; `writer.has_colors()`.
    pub fn has_colors(&self) -> bool {
        self.has_colors
    }
    /// Sets whether point colors are written; `writer.set_colors(false)`.
    pub fn set_colors(&mut self, has_colors: bool) {
        self.has_colors = has_colors;
    }
    /// True when elements carry the index of their part; `writer.has_part_id()`.
    pub fn has_part_id(&self) -> bool {
        self.has_part_id
    }
    /// Sets whether elements carry the index of their part, ruling out face ids;
    /// `writer.set_part_id(true)`.
    pub fn set_part_id(&mut self, has_part_id: bool) {
        self.has_part_id = has_part_id;
        self.has_face_id = self.has_face_id && !self.has_part_id;
    }
    /// True when elements carry the index of their face; `writer.has_face_id()`.
    pub fn has_face_id(&self) -> bool {
        self.has_face_id
    }
    /// Sets whether elements carry the index of their face, ruling out part ids;
    /// `writer.set_face_id(true)`.
    pub fn set_face_id(&mut self, has_face_id: bool) {
        self.has_face_id = has_face_id;
        self.has_part_id = self.has_part_id && !self.has_face_id;
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
                "RwPlyCafWriter: expected a document",
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
            send_fail("Nothing to export into PLY file");
            return Ok(false);
        }
        let export = Export {
            document,
            roots,
            label_filter,
        };
        let summary = self.summarize(&export)?;
        if summary.nb_nodes == 0 {
            send_fail("No mesh data to save");
            return Ok(false);
        }
        self.write_file(&export, &summary, file_info, progress)
    }
    fn summarize(&self, export: &Export<'_>) -> Result<FaceSummary, Failure> {
        let mut summary = FaceSummary::default();
        let mut explorer = export.explorer()?;
        while explorer.more() {
            let node = explorer.current();
            if export.accepts(&node.id) {
                let mut faces = RwMeshFaceIterator::with_label(
                    &node.ref_label,
                    &node.location,
                    true,
                    &node.style,
                )?;
                while faces.more() {
                    if !self.to_skip_face_mesh(&faces) {
                        self.add_face_info(&faces, &mut summary.nb_nodes, &mut summary.nb_elems);
                        summary.nb_steps += f64::from(summary.nb_nodes + summary.nb_elems);
                    }
                    faces.next()?;
                }
            }
            explorer.next()?;
        }
        Ok(summary)
    }
    fn new_context(&self) -> RwPlyPlyWriterContext {
        let mut context = RwPlyPlyWriterContext::new();
        context.set_double_precision(self.is_double_prec);
        context.set_normals(self.has_normals);
        context.set_colors(self.has_colors);
        context.set_tex_coords(self.has_tex_coords);
        context.set_has_surface_id(self.has_part_id || self.has_face_id);
        context
    }
    fn write_file(
        &self,
        export: &Export<'_>,
        summary: &FaceSummary,
        file_info: &IndexedDataMap<AsciiString, AsciiString>,
        progress: &ProgressRange<'_>,
    ) -> Result<bool, Failure> {
        let mut context = self.new_context();
        if !context.open(&self.file, None)
            || !context.write_header(summary.nb_nodes, summary.nb_elems, file_info)
        {
            return Ok(false);
        }
        let mut scope =
            LazyProgressScope::new(progress, "PLY export", summary.nb_steps, PATCH_STEP, false);
        let is_done = self.write_records(export, &mut context, &mut scope)?;
        let is_aborted = scope.is_aborted();
        let is_closed = context.close(is_aborted);
        if is_done && !is_closed {
            let name = String::from_utf8_lossy(self.file.as_bytes()).into_owned();
            send_fail(&format!("Failed to write PLY file\n{name}"));
            return Ok(false);
        }
        Ok(is_done && !is_aborted)
    }
    /// All vertices, then all elements: a PLY file holds its records in the order of its header.
    fn write_records(
        &self,
        export: &Export<'_>,
        context: &mut RwPlyPlyWriterContext,
        scope: &mut LazyProgressScope<'_>,
    ) -> Result<bool, Failure> {
        for write_step in 0..2 {
            context.set_surface_id(0);
            let mut explorer = export.explorer()?;
            while explorer.more() && !scope.is_aborted() {
                let node = explorer.current().clone();
                if export.accepts(&node.id) {
                    if self.has_part_id {
                        context.set_surface_id(context.surface_id() + 1);
                    }
                    if !self.write_shape(
                        context,
                        scope,
                        write_step,
                        &node.ref_label,
                        &node.location,
                        &node.style,
                    )? {
                        return Ok(false);
                    }
                }
                explorer.next()?;
            }
        }
        Ok(true)
    }
    /// True when a face mesh is left out, for being empty; `writer.to_skip_face_mesh(&faces)`.
    pub fn to_skip_face_mesh(&self, faces: &RwMeshFaceIterator) -> bool {
        faces.is_empty_mesh()
    }
    /// Adds the counts of a face to overall counts;
    /// `writer.add_face_info(&faces, &mut nb_nodes, &mut nb_elems)`.
    pub fn add_face_info(
        &self,
        faces: &RwMeshFaceIterator,
        nb_nodes: &mut i32,
        nb_elems: &mut i32,
    ) {
        *nb_nodes += faces.nb_nodes();
        *nb_elems += faces.nb_triangles();
    }
    /// Writes the faces of a label: their vertices at step 0, their elements at step 1;
    /// `writer.write_shape(&mut context, &mut scope, 0, &label, &location, &style)?`.
    pub fn write_shape(
        &self,
        context: &mut RwPlyPlyWriterContext,
        scope: &mut LazyProgressScope<'_>,
        write_step: i32,
        label: &TdfLabel,
        parent_trsf: &Location,
        parent_style: &XCAFPrsStyle,
    ) -> Result<bool, Failure> {
        let mut faces = RwMeshFaceIterator::with_label(label, parent_trsf, true, parent_style)?;
        while faces.more() && !scope.is_aborted() {
            if !self.to_skip_face_mesh(&faces) {
                let is_written = match write_step {
                    0 => self.write_nodes(context, scope, &faces)?,
                    1 => self.write_indices(context, scope, &faces)?,
                    _ => true,
                };
                if !is_written {
                    return Ok(false);
                }
            }
            faces.next()?;
        }
        Ok(true)
    }
    /// The normal of a node in the file's system, as single precision.
    fn node_normal(&self, faces: &RwMeshFaceIterator, node: i32) -> Result<Vec3<f32>, Failure> {
        let [x, y, z] = *faces.normal_transformed(node)?.coordinates();
        let mut normal = Vec3::new(x as f32, y as f32, z as f32);
        self.cs_trsf.transform_normal(&mut normal);
        Ok(normal)
    }
    /// Writes the vertices of a face; `writer.write_nodes(&mut context, &mut scope, &faces)?`.
    pub fn write_nodes(
        &self,
        context: &mut RwPlyPlyWriterContext,
        scope: &mut LazyProgressScope<'_>,
        faces: &RwMeshFaceIterator,
    ) -> Result<bool, Failure> {
        let (mut normal, mut uv) = (Vec3::new(0.0, 0.0, 0.0), Vec2::new(0.0, 0.0));
        let color = face_color_bytes(faces);
        let mut node = faces.node_lower();
        while node <= faces.node_upper() && scope.more() {
            let mut position = Xyz::from(faces.node_transformed(node)?);
            self.cs_trsf.transform_position(&mut position);
            if faces.has_normals() {
                normal = self.node_normal(faces, node)?;
            }
            if faces.has_tex_coords() {
                let parameter = faces.node_tex_coord(node)?;
                uv = Vec2::new(parameter.x() as f32, parameter.y() as f32);
            }
            let point = Point3::new(position.x(), position.y(), position.z());
            if !context.write_vertex(&point, &normal, &uv, &color)? {
                return Ok(false);
            }
            node += 1;
            scope.next();
        }
        Ok(true)
    }
    /// Writes the triangles of a face and moves the indices past its nodes;
    /// `writer.write_indices(&mut context, &mut scope, &faces)?`.
    pub fn write_indices(
        &self,
        context: &mut RwPlyPlyWriterContext,
        scope: &mut LazyProgressScope<'_>,
        faces: &RwMeshFaceIterator,
    ) -> Result<bool, Failure> {
        if self.has_face_id {
            context.set_surface_id(context.surface_id() + 1);
        }
        let lower = faces.elem_lower();
        let mut element = lower;
        while element <= faces.elem_upper() && scope.more() {
            let (n1, n2, n3) = faces.triangle_oriented(element)?.get();
            if !context.write_triangle(&Vec3::new(n1 - lower, n2 - lower, n3 - lower))? {
                return Ok(false);
            }
            element += 1;
            scope.next();
        }
        context.set_vertex_offset(context.vertex_offset() + faces.nb_nodes());
        Ok(true)
    }
}
