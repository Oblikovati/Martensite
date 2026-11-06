use super::{
    DeIgesParameters,
    modes::{
        DeIgesReadModeBSplineContinuity, DeIgesReadModeMaxPrecision, DeIgesReadModePrecision,
        DeIgesReadModeSurfaceCurve, DeIgesWriteModeBRep, DeIgesWriteModeConvertSurface,
        DeIgesWriteModePlaneMode, DeIgesWriteModePrecisionMode,
    },
};
use martensite_tkxsbase::interface::InterfaceStatic;

impl DeIgesParameters {
    /// Reads the existing process-wide exchange registry, preserving the conversions.
    /// Example: `parameters.init_from_static();`.
    pub fn init_from_static(&mut self) {
        self.read_static_group_0();
        self.read_static_group_1();
    }
    /// Reads the settings the transfers of IGES use, which name the format in their keys;
    /// `parameters.read_from_iges_statics();`.
    pub fn read_from_iges_statics(&mut self) {
        self.init_from_static();
        self.read_faulty_entities = InterfaceStatic::i_val("read.iges.faulty.entities") == 1;
        self.read_only_visible = InterfaceStatic::i_val("read.iges.onlyvisible") == 1;
        self.write_b_rep_mode =
            DeIgesWriteModeBRep::from_raw(InterfaceStatic::i_val("write.iges.brep.mode"));
        self.write_header_author = InterfaceStatic::c_val("write.iges.header.author");
        self.write_header_company = InterfaceStatic::c_val("write.iges.header.company");
        self.write_header_product = InterfaceStatic::c_val("write.iges.header.product");
        self.write_header_reciever = InterfaceStatic::c_val("write.iges.header.receiver");
        self.write_plane_mode =
            DeIgesWriteModePlaneMode::from_raw(InterfaceStatic::i_val("write.iges.plane.mode"));
        self.write_offset_mode = InterfaceStatic::i_val("write.iges.offset.mode") == 1;
    }
    fn read_static_group_0(&mut self) {
        self.read_static_read_b_spline_continuity();
        self.read_static_read_precision_val();
        self.read_static_read_max_precision_val();
        self.read_static_read_surface_curve_mode();
        self.read_static_read_approxd1();
        self.read_static_read_only_visible();
        self.read_static_read_name();
        self.read_static_write_b_rep_mode();
    }
    fn read_static_group_1(&mut self) {
        self.read_static_write_header_author();
        self.read_static_write_header_product();
        self.read_static_write_precision_mode();
        self.read_static_write_plane_mode();
        self.read_static_write_color();
        self.read_static_write_layer();
    }
    fn read_static_read_b_spline_continuity(&mut self) {
        self.read_b_spline_continuity = DeIgesReadModeBSplineContinuity::from_raw(
            InterfaceStatic::i_val("read.iges.bspline.continuity"),
        );
        self.read_precision_mode =
            DeIgesReadModePrecision::from_raw(InterfaceStatic::i_val("read.precision.mode"));
    }
    fn read_static_read_precision_val(&mut self) {
        self.read_precision_val = InterfaceStatic::r_val("read.precision.val");
        self.read_max_precision_mode =
            DeIgesReadModeMaxPrecision::from_raw(InterfaceStatic::i_val("read.maxprecision.mode"));
    }
    fn read_static_read_max_precision_val(&mut self) {
        self.read_max_precision_val = InterfaceStatic::r_val("read.maxprecision.val");
        self.read_same_param_mode = InterfaceStatic::i_val("read.stdsameparameter.mode") == 1;
    }
    fn read_static_read_surface_curve_mode(&mut self) {
        self.read_surface_curve_mode =
            DeIgesReadModeSurfaceCurve::from_raw(InterfaceStatic::i_val("read.surfacecurve.mode"));
        self.encode_reg_angle = InterfaceStatic::r_val("read.encoderegularity.angle");
    }
    fn read_static_read_approxd1(&mut self) {
        self.read_approxd1 = InterfaceStatic::i_val("read.bspline.approxd1.mode") == 1;
        self.read_faulty_entities = InterfaceStatic::i_val("read.fau_lty.entities") == 1;
    }
    fn read_static_read_only_visible(&mut self) {
        self.read_only_visible = InterfaceStatic::i_val("read.onlyvisible") == 1;
        self.read_color = InterfaceStatic::i_val("read.color") == 1;
    }
    fn read_static_read_name(&mut self) {
        self.read_name = InterfaceStatic::i_val("read.name") == 1;
        self.read_layer = InterfaceStatic::i_val("read.layer") == 1;
    }
    fn read_static_write_b_rep_mode(&mut self) {
        self.write_b_rep_mode =
            DeIgesWriteModeBRep::from_raw(InterfaceStatic::i_val("write.brep.mode"));
        self.write_convert_surface_mode = DeIgesWriteModeConvertSurface::from_raw(
            InterfaceStatic::i_val("write.convertsurface.mode"),
        );
    }
    fn read_static_write_header_author(&mut self) {
        self.write_header_author = InterfaceStatic::c_val("write.header.author");
        self.write_header_company = InterfaceStatic::c_val("write.header.company");
    }
    fn read_static_write_header_product(&mut self) {
        self.write_header_product = InterfaceStatic::c_val("write.header.product");
        self.write_header_reciever = InterfaceStatic::c_val("write.header.receiver");
    }
    fn read_static_write_precision_mode(&mut self) {
        self.write_precision_mode =
            DeIgesWriteModePrecisionMode::from_raw(InterfaceStatic::i_val("write.precision.mode"));
        self.write_precision_val = InterfaceStatic::r_val("write.precision.val");
    }
    fn read_static_write_plane_mode(&mut self) {
        self.write_plane_mode =
            DeIgesWriteModePlaneMode::from_raw(InterfaceStatic::i_val("write.plane.mode"));
        self.write_offset_mode = InterfaceStatic::i_val("write.offset") == 1;
    }
    fn read_static_write_color(&mut self) {
        self.write_color = InterfaceStatic::i_val("write.color") == 1;
        self.write_name = InterfaceStatic::i_val("write.name") == 1;
    }
    fn read_static_write_layer(&mut self) {
        self.write_layer = InterfaceStatic::i_val("write.layer") == 1;
    }
}
