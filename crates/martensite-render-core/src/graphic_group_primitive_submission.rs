//! Snapshot retained geometry and bind each authored level set to its primary revision.
use super::*;

impl Graphic3dDrawGroup {
    /// Retain an array the driver draws itself rather than a group element (native
    /// GlPrimitiveArray members of GlGraduatedTrihedron): it takes no driver id.
    /// Example: `group.add_driver_primitive_array(&array)`.
    pub(crate) fn add_driver_primitive_array(&self, array: &Graphic3dArrayOfPrimitives) {
        let primitive = PrimitiveSubmission {
            kind: array.primitive_type(),
            indices: array.indices_ref().clone(),
            attributes: array.attributes_ref().clone(),
            bounds: array.bounds_ref().clone(),
            upload: Default::default(),
            uid: 0,
            retained: None,
            mesh_levels: None,
        };
        self.retain_primitive(primitive, false);
    }
    fn retain_primitive(&self, mut primitive: PrimitiveSubmission, evaluate_bounds: bool) {
        if self.is_deleted() {
            return;
        }
        primitive.retained = self.primitive_sources.entry(&primitive);
        if evaluate_bounds && let Ok(bounds) = primitive.group_bounds() {
            bounds.append(&mut self.base.fields.write().bounds);
        }
        self.submissions
            .write()
            .push_command(GroupCommand::Geometry(GroupGeometry::Primitives(primitive)));
        self.update();
    }

    pub(super) fn submit_array(
        &self,
        kind: Graphic3dTypeOfPrimitiveArray,
        buffers: (
            &Handle<dyn Graphic3dIndexBufferSource>,
            &Handle<dyn Graphic3dBufferSource>,
            &Handle<Graphic3dBoundBuffer>,
        ),
        evaluate_bounds: bool,
        mesh_levels: Option<Arc<crate::mesh_lod::MeshAlternates>>,
    ) {
        let (indices, attributes, bounds) = buffers;
        if self.is_deleted() || attributes.is_null() {
            // GlGroup::AddPrimitiveArray returns before creating an element.
            return;
        }
        let uid = self.take_primitive_array_uid();
        let mut primitive = PrimitiveSubmission {
            kind,
            indices: indices.clone(),
            attributes: attributes.clone(),
            bounds: bounds.clone(),
            upload: Default::default(),
            uid,
            retained: None,
            mesh_levels: None,
        };
        primitive.mesh_levels = mesh_levels
            .and_then(|levels| crate::mesh_lod::SubmittedMeshLevels::bind(&primitive, levels));
        let raytraced = crate::driver_dump::is_raytraced_primitive(&primitive);
        self.retain_primitive(primitive, evaluate_bounds);
        if raytraced {
            self.note_raytraced_element();
        }
    }
    pub(super) fn submit_triangle_mesh_levels(
        &self,
        primary: &Handle<Graphic3dArrayOfTriangles>,
        alternates: &[Graphic3dTriangleMeshLevel],
        pixel_error: f64,
    ) {
        let Some(primary) = primary.get().filter(|a| !self.is_deleted() && a.is_valid()) else {
            return;
        };
        let levels = (self.supports_triangle_mesh_levels()
            && !primary.has_vertex_texels()
            && !primary.has_vertex_colors()
            && primary.bounds().is_null())
        .then(|| crate::mesh_lod::MeshAlternates::snapshot(alternates, pixel_error))
        .flatten();
        self.submit_array(
            primary.primitive_type(),
            (
                primary.indices_ref(),
                primary.attributes_ref(),
                primary.bounds_ref(),
            ),
            true,
            levels,
        );
    }
}
