use super::{DeStepParameters, modes::*};
use martensite_tkxsbase::interface::InterfaceStatic;

impl DeStepParameters {
    /// Reads the existing process-wide exchange registry, preserving conversions.
    /// Example: `parameters.init_from_static();`.
    pub fn init_from_static(&mut self) {
        self.read_static_group_0();
        self.read_static_group_1();
        self.read_static_group_2();
    }
    fn read_static_group_0(&mut self) {
        self.read_static_read_b_spline_continuity();
        self.read_static_read_precision_val();
        self.read_static_read_max_precision_val();
        self.read_static_read_surface_curve_mode();
        self.read_static_angle_unit();
        self.read_static_read_product_context();
        self.read_static_read_tessellated();
        self.read_static_read_relationship();
    }
    fn read_static_group_1(&mut self) {
        self.read_static_read_constr_relation();
        self.read_static_read_code_page();
        self.read_static_read_ideas();
        self.read_static_read_root_transformation();
        self.read_static_write_precision_val();
        self.read_static_write_schema();
        self.read_static_write_product_name();
        self.read_static_write_unit();
    }
    fn read_static_group_2(&mut self) {
        self.read_static_write_subshape_names();
        self.read_static_write_nonmanifold();
        self.read_static_write_layer();
        self.read_static_write_model_type();
    }
    fn read_static_read_b_spline_continuity(&mut self) {
        self.read_b_spline_continuity = DeStepReadModeBSplineContinuity::from_raw(
            InterfaceStatic::i_val("read.iges.bspline.continuity"),
        );
        self.read_precision_mode =
            DeStepReadModePrecision::from_raw(InterfaceStatic::i_val("read.precision.mode"));
    }
    fn read_static_read_precision_val(&mut self) {
        self.read_precision_val = InterfaceStatic::r_val("read.precision.val");
        self.read_max_precision_mode =
            DeStepReadModeMaxPrecision::from_raw(InterfaceStatic::i_val("read.maxprecision.mode"));
    }
    fn read_static_read_max_precision_val(&mut self) {
        self.read_max_precision_val = InterfaceStatic::r_val("read.maxprecision.val");
        self.read_same_param_mode = InterfaceStatic::i_val("read.stdsameparameter.mode") == 1;
    }
    fn read_static_read_surface_curve_mode(&mut self) {
        self.read_surface_curve_mode =
            DeStepReadModeSurfaceCurve::from_raw(InterfaceStatic::i_val("read.surfacecurve.mode"));
        self.encode_reg_angle =
            InterfaceStatic::r_val("read.encoderegularity.angle") * 180.0 / std::f64::consts::PI;
    }
    fn read_static_angle_unit(&mut self) {
        self.angle_unit =
            DeStepAngleUnitMode::from_raw(InterfaceStatic::i_val("step.angleunit.mode"));
        self.read_product_mode = InterfaceStatic::i_val("read.step.product.mode") == 1;
    }
    fn read_static_read_product_context(&mut self) {
        self.read_product_context = DeStepReadModeProductContext::from_raw(InterfaceStatic::i_val(
            "read.step.product.context",
        ));
        self.read_shape_repr =
            DeStepReadModeShapeRepr::from_raw(InterfaceStatic::i_val("read.step.shape.repr"));
    }
    fn read_static_read_tessellated(&mut self) {
        self.read_tessellated =
            DeStepRWModeTessellated::from_raw(InterfaceStatic::i_val("read.step.tessellated"));
        self.read_assembly_level = DeStepReadModeAssemblyLevel::from_raw(InterfaceStatic::i_val(
            "read.step.assembly.level",
        ));
    }
    fn read_static_read_relationship(&mut self) {
        self.read_relationship = InterfaceStatic::i_val("read.step.shape.relationship") == 1;
        self.read_shape_aspect = InterfaceStatic::i_val("read.step.shape.aspect") == 1;
    }
    fn read_static_read_constr_relation(&mut self) {
        self.read_constr_relation =
            InterfaceStatic::i_val("read.step.constructivegeom.relationship") == 1;
        self.read_subshape_names = InterfaceStatic::i_val("read.stepcaf.subshapes.name") == 1;
    }
    fn read_static_read_code_page(&mut self) {
        self.read_code_page = InterfaceStatic::i_val("read.step.codepage");
        self.read_nonmanifold = InterfaceStatic::i_val("read.step.nonmanifold") == 1;
    }
    fn read_static_read_ideas(&mut self) {
        self.read_ideas = InterfaceStatic::i_val("read.step.ideas") == 1;
        self.read_all_shapes = InterfaceStatic::i_val("read.step.all.shapes") == 1;
    }
    fn read_static_read_root_transformation(&mut self) {
        self.read_root_transformation =
            InterfaceStatic::i_val("read.step.root.transformation") == 1;
        self.write_precision_mode =
            DeStepWriteModePrecisionMode::from_raw(InterfaceStatic::i_val("write.precision.mode"));
    }
    fn read_static_write_precision_val(&mut self) {
        self.write_precision_val = InterfaceStatic::r_val("write.precision.val");
        self.write_assembly =
            DeStepWriteModeAssembly::from_raw(InterfaceStatic::i_val("write.step.assembly"));
    }
    fn read_static_write_schema(&mut self) {
        self.write_schema =
            DeStepWriteModeStepSchema::from_raw(InterfaceStatic::i_val("write.step.schema"));
        self.write_tessellated =
            DeStepRWModeTessellated::from_raw(InterfaceStatic::i_val("write.step.tessellated"));
    }
    fn read_static_write_product_name(&mut self) {
        self.write_product_name = InterfaceStatic::c_val("write.step.product.name");
        self.write_surface_cur_mode = InterfaceStatic::i_val("write.surfacecurve.mode") == 1;
    }
    fn read_static_write_unit(&mut self) {
        self.write_unit = InterfaceStatic::i_val("write.step.unit");
        self.write_vertex_mode =
            DeStepWriteModeVertexMode::from_raw(InterfaceStatic::i_val("write.step.vertex.mode"));
    }
    fn read_static_write_subshape_names(&mut self) {
        self.write_subshape_names = InterfaceStatic::i_val("write.stepcaf.subshapes.name") == 1;
        self.write_color = InterfaceStatic::i_val("write.color") == 1;
    }
    fn read_static_write_nonmanifold(&mut self) {
        self.write_nonmanifold = InterfaceStatic::i_val("write.step.nonmanifold") == 1;
        self.write_name = InterfaceStatic::i_val("write.name") == 1;
    }
    fn read_static_write_layer(&mut self) {
        self.write_layer = InterfaceStatic::i_val("write.layer") == 1;
        self.write_props = InterfaceStatic::i_val("write.props") == 1;
    }
    fn read_static_write_model_type(&mut self) {
        self.write_model_type =
            StepControlStepModelType::from_raw(InterfaceStatic::i_val("write.model.type"));
    }
}
