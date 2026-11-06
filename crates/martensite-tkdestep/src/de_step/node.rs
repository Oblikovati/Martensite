use super::{DeStepParameters, modes::*, serialization};
use martensite_tkde::de::{
    DeConfigurationContext, DeConfigurationNodeBase, DeShapeFixConfigurationNodeBase,
    DeShapeFixParameters, load_shape_fix_parameters, save_shape_fix_parameters,
};
use martensite_tkernel::{
    ncollection::List,
    standard::{SharedState, StandardTransient},
    tcollection::AsciiString,
};
use std::ops::Deref;
/// Configuration state of `DeStepConfigurationNode`, without a provider backend.
/// Example: `DeStepConfigurationNode::default()`.
pub struct DeStepConfigurationNode {
    pub(super) transient: StandardTransient,
    pub(super) base: DeShapeFixConfigurationNodeBase,
    /// STEP-specific transfer parameters, shared by every holder of the node.
    pub internal_parameters: SharedState<DeStepParameters>,
}
impl Default for DeStepConfigurationNode {
    fn default() -> Self {
        let base = DeShapeFixConfigurationNodeBase::default();
        base.set_shape_fix_parameters(DeStepParameters::default_shape_fix_parameters());
        Self {
            transient: StandardTransient::default(),
            base,
            internal_parameters: SharedState::new(DeStepParameters::default()),
        }
    }
}
impl Deref for DeStepConfigurationNode {
    type Target = DeConfigurationNodeBase;
    fn deref(&self) -> &Self::Target {
        self.base.configuration_node_base()
    }
}
impl DeStepConfigurationNode {
    /// Copies independent node/healing/format settings; `DeStepConfigurationNode::copied_from(&node)`.
    pub fn copied_from(source: &Self) -> Self {
        Self {
            transient: StandardTransient::default(),
            base: source.base.copied_state(),
            internal_parameters: SharedState::new(source.internal_parameters.snapshot()),
        }
    }
    /// Returns healing parameters; `node.shape_fix_parameters()`.
    pub fn shape_fix_parameters(&self) -> DeShapeFixParameters {
        self.base.shape_fix_parameters()
    }
    /// Replaces healing parameters; `node.set_shape_fix_parameters(parameters)`.
    pub fn set_shape_fix_parameters(&self, parameters: DeShapeFixParameters) {
        self.base.set_shape_fix_parameters(parameters);
    }
    /// Updates defined resources in field order; `node.load(&context)`.
    pub fn load(&self, context: &DeConfigurationContext) -> bool {
        self.load_common_and_product(context);
        self.load_shape_and_document(context);
        self.load_write_settings(context);
        let mut healing = self.shape_fix_parameters();
        load_shape_fix_parameters(
            context,
            &"provider.STEP.Martensite.healing".into(),
            &mut healing,
        );
        self.set_shape_fix_parameters(healing);
        true
    }
    fn load_common_and_product(&self, c: &DeConfigurationContext) {
        self.load_read_b_spline_continuity(c);
        self.load_read_precision_val(c);
        self.load_read_max_precision_val(c);
        self.load_read_surface_curve_mode(c);
        self.load_angle_unit(c);
        self.load_read_product_context(c);
        self.load_read_tessellated(c);
        self.load_read_relationship(c);
    }
    fn load_shape_and_document(&self, c: &DeConfigurationContext) {
        self.load_read_constr_relation(c);
        self.load_read_code_page(c);
        self.load_read_ideas(c);
        self.load_read_root_transformation(c);
        self.load_read_name(c);
        self.load_read_props(c);
        self.load_read_product_metadata(c);
    }
    fn load_write_settings(&self, c: &DeConfigurationContext) {
        self.load_write_precision_val(c);
        self.load_write_schema(c);
        self.load_write_product_name(c);
        self.load_write_unit(c);
        self.load_write_subshape_names(c);
        self.load_write_nonmanifold(c);
        self.load_write_layer(c);
        self.load_write_metadata(c);
        self.load_write_vis_material(c);
        self.load_clean_duplicates(c);
    }
    fn load_read_b_spline_continuity(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_b_spline_continuity = DeStepReadModeBSplineContinuity::from_raw(c.integer_val(
            "read.iges.bspline.continuity",
            p.read_b_spline_continuity.as_raw(),
            scope,
        ));
        p.read_precision_mode = DeStepReadModePrecision::from_raw(c.integer_val(
            "read.precision.mode",
            p.read_precision_mode.as_raw(),
            scope,
        ));
    }
    fn load_read_precision_val(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_precision_val = c.real_val("read.precision.val", p.read_precision_val, scope);
        p.read_max_precision_mode = DeStepReadModeMaxPrecision::from_raw(c.integer_val(
            "read.maxprecision.mode",
            p.read_max_precision_mode.as_raw(),
            scope,
        ));
    }
    fn load_read_max_precision_val(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_max_precision_val =
            c.real_val("read.maxprecision.val", p.read_max_precision_val, scope);
        p.read_same_param_mode =
            c.boolean_val("read.stdsameparameter.mode", p.read_same_param_mode, scope);
    }
    fn load_read_surface_curve_mode(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_surface_curve_mode = DeStepReadModeSurfaceCurve::from_raw(c.integer_val(
            "read.surfacecurve.mode",
            p.read_surface_curve_mode.as_raw(),
            scope,
        ));
        p.encode_reg_angle = c.real_val("read.encoderegularity.angle", p.encode_reg_angle, scope);
    }
    fn load_angle_unit(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.angle_unit = DeStepAngleUnitMode::from_raw(c.integer_val(
            "angleunit.mode",
            p.angle_unit.as_raw(),
            scope,
        ));
        p.read_product_mode = c.boolean_val("read.product.mode", p.read_product_mode, scope);
    }
    fn load_read_product_context(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_product_context = DeStepReadModeProductContext::from_raw(c.integer_val(
            "read.product.context",
            p.read_product_context.as_raw(),
            scope,
        ));
        p.read_shape_repr = DeStepReadModeShapeRepr::from_raw(c.integer_val(
            "read.shape.repr",
            p.read_shape_repr.as_raw(),
            scope,
        ));
    }
    fn load_read_tessellated(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_tessellated = DeStepRWModeTessellated::from_raw(c.integer_val(
            "read.tessellated",
            p.read_tessellated.as_raw(),
            scope,
        ));
        p.read_assembly_level = DeStepReadModeAssemblyLevel::from_raw(c.integer_val(
            "read.assembly.level",
            p.read_assembly_level.as_raw(),
            scope,
        ));
    }
    fn load_read_relationship(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_relationship = c.boolean_val("read.shape.relationship", p.read_relationship, scope);
        p.read_shape_aspect = c.boolean_val("read.shape.aspect", p.read_shape_aspect, scope);
    }
    fn load_read_constr_relation(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_constr_relation = c.boolean_val(
            "read.constructivegeom.relationship",
            p.read_constr_relation,
            scope,
        );
        p.read_subshape_names =
            c.boolean_val("read.stepcaf.subshapes.name", p.read_subshape_names, scope);
    }
    fn load_read_code_page(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_code_page = c.integer_val("read.codepage", p.read_code_page, scope);
        p.read_nonmanifold = c.boolean_val("read.nonmanifold", p.read_nonmanifold, scope);
    }
    fn load_read_ideas(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_ideas = c.boolean_val("read.ideas", p.read_ideas, scope);
        p.read_all_shapes = c.boolean_val("read.all.shapes", p.read_all_shapes, scope);
    }
    fn load_read_root_transformation(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_root_transformation = c.boolean_val(
            "read.root.transformation",
            p.read_root_transformation,
            scope,
        );
        p.read_color = c.boolean_val("read.color", p.read_color, scope);
    }
    fn load_read_name(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_name = c.boolean_val("read.name", p.read_name, scope);
        p.read_layer = c.boolean_val("read.layer", p.read_layer, scope);
    }
    fn load_read_props(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_props = c.boolean_val("read.props", p.read_props, scope);
        p.read_metadata = c.boolean_val("read.metadata", p.read_metadata, scope);
    }
    fn load_read_product_metadata(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.read_product_metadata =
            c.boolean_val("read.productmetadata", p.read_product_metadata, scope);
        p.write_precision_mode = DeStepWriteModePrecisionMode::from_raw(c.integer_val(
            "write.precision.mode",
            p.write_precision_mode.as_raw(),
            scope,
        ));
    }
    fn load_write_precision_val(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_precision_val = c.real_val("write.precision.val", p.write_precision_val, scope);
        p.write_assembly = DeStepWriteModeAssembly::from_raw(c.integer_val(
            "write.assembly",
            p.write_assembly.as_raw(),
            scope,
        ));
    }
    fn load_write_schema(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_schema = DeStepWriteModeStepSchema::from_raw(c.integer_val(
            "write.schema",
            p.write_schema.as_raw(),
            scope,
        ));
        p.write_tessellated = DeStepRWModeTessellated::from_raw(c.integer_val(
            "write.tessellated",
            p.write_tessellated.as_raw(),
            scope,
        ));
    }
    fn load_write_product_name(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_product_name = c.string_val("write.product.name", &p.write_product_name, scope);
        p.write_surface_cur_mode =
            c.boolean_val("write.surfacecurve.mode", p.write_surface_cur_mode, scope);
    }
    fn load_write_unit(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_unit = c.integer_val("write.unit", p.write_unit, scope);
        p.write_vertex_mode = DeStepWriteModeVertexMode::from_raw(c.integer_val(
            "write.vertex.mode",
            p.write_vertex_mode.as_raw(),
            scope,
        ));
    }
    fn load_write_subshape_names(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_subshape_names = c.boolean_val(
            "write.stepcaf.subshapes.name",
            p.write_subshape_names,
            scope,
        );
        p.write_color = c.boolean_val("write.color", p.write_color, scope);
    }
    fn load_write_nonmanifold(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_nonmanifold = c.boolean_val("write.nonmanifold", p.write_nonmanifold, scope);
        p.write_name = c.boolean_val("write.name", p.write_name, scope);
    }
    fn load_write_layer(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_layer = c.boolean_val("write.layer", p.write_layer, scope);
        p.write_props = c.boolean_val("write.props", p.write_props, scope);
    }
    fn load_write_metadata(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_metadata = c.boolean_val("write.metadata", p.write_metadata, scope);
        p.write_material = c.boolean_val("write.material", p.write_material, scope);
    }
    fn load_write_vis_material(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.write_vis_material = c.boolean_val("write.vismaterial", p.write_vis_material, scope);
        p.write_model_type = StepControlStepModelType::from_raw(c.integer_val(
            "write.model.type",
            p.write_model_type.as_raw(),
            scope,
        ));
    }
    fn load_clean_duplicates(&self, c: &DeConfigurationContext) {
        let mut p = self.internal_parameters.write();
        let scope = "provider.STEP.Martensite";
        p.clean_duplicates = c.boolean_val("write.cleanduplicates", p.clean_duplicates, scope);
        p.write_scaling_trsf = c.boolean_val("write.scaling.trsf", p.write_scaling_trsf, scope);
    }
    /// Serializes format and healing resources; `let text = node.save();`.
    pub fn save(&self) -> AsciiString {
        let healing = save_shape_fix_parameters(
            self.shape_fix_parameters(),
            &"provider.STEP.Martensite.healing.".into(),
        );
        serialization::save(&self.internal_parameters.read(), healing)
    }
    /// Reports STEP import support; `assert!(node.is_import_supported());`.
    pub fn is_import_supported(&self) -> bool {
        true
    }
    /// Reports STEP export support; `assert!(node.is_export_supported());`.
    pub fn is_export_supported(&self) -> bool {
        true
    }
    /// Reports stream support independently of node settings; `assert!(node.is_stream_supported());`.
    pub fn is_stream_supported(&self) -> bool {
        true
    }
    /// Identifies format; `let format = node.get_format();`.
    pub fn get_format(&self) -> AsciiString {
        "STEP".into()
    }
    /// Identifies vendor; `let vendor = node.get_vendor();`.
    pub fn get_vendor(&self) -> AsciiString {
        "Martensite".into()
    }
    /// Returns all three suffixes; `assert_eq!(node.get_extensions().len(),3);`.
    pub fn get_extensions(&self) -> List<AsciiString> {
        let mut extensions = List::default();
        extensions.append("stp".into());
        extensions.append("step".into());
        extensions.append("stpz".into());
        extensions
    }
    /// Recognizes a STEP signature before the first NUL, excluding IFC; `node.check_content(None)`.
    pub fn check_content(&self, buffer: Option<&[u8]>) -> bool {
        let Some(bytes) = buffer.filter(|bytes| bytes.len() >= 100) else {
            return false;
        };
        let end = bytes
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(bytes.len());
        let text = &bytes[..end];
        !text.windows(3).any(|window| window == b"IFC")
            && text.windows(12).any(|window| window == b"ISO-10303-21")
            && text.windows(11).any(|window| window == b"FILE_SCHEMA")
    }
}
