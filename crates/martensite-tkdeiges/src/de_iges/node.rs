use super::{
    DeIgesParameters,
    modes::{
        DeIgesReadModeBSplineContinuity, DeIgesReadModeMaxPrecision, DeIgesReadModePrecision,
        DeIgesReadModeSurfaceCurve, DeIgesWriteModeBRep, DeIgesWriteModeConvertSurface,
        DeIgesWriteModePlaneMode, DeIgesWriteModePrecisionMode,
    },
    serialization,
};
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
/// The settings of the IGES format, and the provider they make.
/// Example: `DeIgesConfigurationNode::default()`.
pub struct DeIgesConfigurationNode {
    pub(super) transient: StandardTransient,
    pub(super) base: DeShapeFixConfigurationNodeBase,
    /// IGES-specific transfer parameters, shared by every holder of the node.
    pub internal_parameters: SharedState<DeIgesParameters>,
}
impl Default for DeIgesConfigurationNode {
    fn default() -> Self {
        let base = DeShapeFixConfigurationNodeBase::default();
        base.set_shape_fix_parameters(DeIgesParameters::default_shape_fix_parameters());
        Self {
            transient: StandardTransient::default(),
            base,
            internal_parameters: SharedState::new(DeIgesParameters::default()),
        }
    }
}
impl Deref for DeIgesConfigurationNode {
    type Target = DeConfigurationNodeBase;
    fn deref(&self) -> &Self::Target {
        self.base.configuration_node_base()
    }
}
impl DeIgesConfigurationNode {
    /// Copies independent node/healing/format settings; `DeIgesConfigurationNode::copied_from(&node)`.
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
        self.load_read_settings(context);
        self.load_write_settings(context);
        let mut healing = self.shape_fix_parameters();
        load_shape_fix_parameters(
            context,
            &"provider.IGES.Martensite.healing".into(),
            &mut healing,
        );
        self.set_shape_fix_parameters(healing);
        true
    }
    fn load_read_settings(&self, c: &DeConfigurationContext) {
        self.load_continuity_and_precision_mode(c);
        self.load_precision_value_and_limit_mode(c);
        self.load_maximum_precision_and_same_parameter(c);
        self.load_surface_curve_and_regularity(c);
        self.load_bspline_and_faulty_entities(c);
        self.load_visibility_and_color(c);
        self.load_names_and_layers(c);
    }
    fn load_write_settings(&self, c: &DeConfigurationContext) {
        self.load_brep_and_surface_conversion(c);
        self.load_author_and_company(c);
        self.load_product_and_receiver(c);
        self.load_precision_mode_and_value(c);
        self.load_planes_and_offsets(c);
        self.load_color_and_names(c);
        self.load_layers(c);
    }
    fn load_continuity_and_precision_mode(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.read_b_spline_continuity = DeIgesReadModeBSplineContinuity::from_raw(c.integer_val(
            "read.iges.bspline.continuity",
            p.read_b_spline_continuity.as_raw(),
            scope,
        ));
        p.read_precision_mode = DeIgesReadModePrecision::from_raw(c.integer_val(
            "read.precision.mode",
            p.read_precision_mode.as_raw(),
            scope,
        ));
    }
    fn load_precision_value_and_limit_mode(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.read_precision_val = c.real_val("read.precision.val", p.read_precision_val, scope);
        p.read_max_precision_mode = DeIgesReadModeMaxPrecision::from_raw(c.integer_val(
            "read.maxprecision.mode",
            p.read_max_precision_mode.as_raw(),
            scope,
        ));
    }
    fn load_maximum_precision_and_same_parameter(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.read_max_precision_val =
            c.real_val("read.maxprecision.val", p.read_max_precision_val, scope);
        p.read_same_param_mode =
            c.boolean_val("read.stdsameparameter.mode", p.read_same_param_mode, scope);
    }
    fn load_surface_curve_and_regularity(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.read_surface_curve_mode = DeIgesReadModeSurfaceCurve::from_raw(c.integer_val(
            "read.surfacecurve.mode",
            p.read_surface_curve_mode.as_raw(),
            scope,
        ));
        p.encode_reg_angle = c.real_val("read.encoderegularity.angle", p.encode_reg_angle, scope);
    }
    fn load_bspline_and_faulty_entities(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.read_approxd1 = c.boolean_val("read.bspline.approxd1.mode", p.read_approxd1, scope);
        p.read_faulty_entities =
            c.boolean_val("read.fau_lty.entities", p.read_faulty_entities, scope);
    }
    fn load_visibility_and_color(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.read_only_visible = c.boolean_val("read.onlyvisible", p.read_only_visible, scope);
        p.read_color = c.boolean_val("read.color", p.read_color, scope);
    }
    fn load_names_and_layers(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.read_name = c.boolean_val("read.name", p.read_name, scope);
        p.read_layer = c.boolean_val("read.layer", p.read_layer, scope);
    }
    fn load_brep_and_surface_conversion(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.write_b_rep_mode = DeIgesWriteModeBRep::from_raw(c.integer_val(
            "write.brep.mode",
            p.write_b_rep_mode.as_raw(),
            scope,
        ));
        p.write_convert_surface_mode = DeIgesWriteModeConvertSurface::from_raw(c.integer_val(
            "write.convertsurface.mode",
            p.write_convert_surface_mode.as_raw(),
            scope,
        ));
    }
    fn load_author_and_company(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.write_header_author = c.string_val("write.header.author", &p.write_header_author, scope);
        p.write_header_company =
            c.string_val("write.header.company", &p.write_header_company, scope);
    }
    fn load_product_and_receiver(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.write_header_product =
            c.string_val("write.header.product", &p.write_header_product, scope);
        p.write_header_reciever =
            c.string_val("write.header.receiver", &p.write_header_reciever, scope);
    }
    fn load_precision_mode_and_value(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.write_precision_mode = DeIgesWriteModePrecisionMode::from_raw(c.integer_val(
            "write.precision.mode",
            p.write_precision_mode.as_raw(),
            scope,
        ));
        p.write_precision_val = c.real_val("write.precision.val", p.write_precision_val, scope);
    }
    fn load_planes_and_offsets(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.write_plane_mode = DeIgesWriteModePlaneMode::from_raw(c.integer_val(
            "write.plane.mode",
            p.write_plane_mode.as_raw(),
            scope,
        ));
        p.write_offset_mode = c.boolean_val("write.offset", p.write_offset_mode, scope);
    }
    fn load_color_and_names(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.write_color = c.boolean_val("write.color", p.write_color, scope);
        p.write_name = c.boolean_val("write.name", p.write_name, scope);
    }
    fn load_layers(&self, c: &DeConfigurationContext) {
        let mut guard = self.internal_parameters.write();
        let p = &mut *guard;
        let scope = "provider.IGES.Martensite";
        p.write_layer = c.boolean_val("write.layer", p.write_layer, scope);
    }
    /// Serializes format and healing resources; `let text = node.save();`.
    pub fn save(&self) -> AsciiString {
        let healing = save_shape_fix_parameters(
            self.shape_fix_parameters(),
            &"provider.IGES.Martensite.healing.".into(),
        );
        serialization::save(&self.internal_parameters.snapshot(), healing)
    }
    /// Reports IGES import support; `assert!(node.is_import_supported());`.
    pub fn is_import_supported(&self) -> bool {
        true
    }
    /// Reports IGES export support; `assert!(node.is_export_supported());`.
    pub fn is_export_supported(&self) -> bool {
        true
    }
    /// Identifies format; `let format = node.get_format();`.
    pub fn get_format(&self) -> AsciiString {
        "IGES".into()
    }
    /// Identifies vendor; `let vendor = node.get_vendor();`.
    pub fn get_vendor(&self) -> AsciiString {
        "Martensite".into()
    }
    /// Returns both suffixes; `assert_eq!(node.get_extensions().len(),2);`.
    pub fn get_extensions(&self) -> List<AsciiString> {
        let mut extensions = List::default();
        extensions.append("igs".into());
        extensions.append("iges".into());
        extensions
    }
    /// Whether streams can be read and written; `assert!(node.is_stream_supported());`.
    pub fn is_stream_supported(&self) -> bool {
        true
    }
    /// Recognizes an IGES start record; `node.check_content(None)`.
    pub fn check_content(&self, buffer: Option<&[u8]>) -> bool {
        let Some(bytes) = buffer.filter(|bytes| bytes.len() >= 83) else {
            return false;
        };
        if bytes[72] != b'S' {
            return false;
        }
        let mut index = 73;
        while index < 80 && matches!(bytes[index], b' ' | b'0') {
            index += 1;
        }
        bytes[index] == b'1' && !bytes[index + 1].is_ascii_alphanumeric()
    }
}
