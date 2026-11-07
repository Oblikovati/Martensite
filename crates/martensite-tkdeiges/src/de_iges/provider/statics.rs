//! The parameters of a node in the settings of the process, and back at the end of a transfer.
use super::{
    DeIgesConfigurationNode, DeIgesParameters, Failure, GLOBAL_IGES_LOCK, InterfaceStatic,
    LengthUnit, PoisonError, iges_data_init,
};
use martensite_tkernel::units_methods::UnitsMethods;
use martensite_tkxsbase::xs_algo::xs_algo_units;
use std::f64::consts::PI;
use std::sync::MutexGuard;

fn set_i(name: &str, value: i32) -> Result<(), Failure> {
    InterfaceStatic::set_i_val(name, value).map(|_| ())
}

fn set_r(name: &str, value: f64) -> Result<(), Failure> {
    InterfaceStatic::set_r_val(name, value).map(|_| ())
}

fn set_c(name: &str, value: &str) -> Result<(), Failure> {
    InterfaceStatic::set_c_val(name, value).map(|_| ())
}

fn text_of(value: &martensite_tkernel::tcollection::AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

/// Puts parameters in the settings; the regularity angle goes in radians.
pub(super) fn set_static(p: &DeIgesParameters) -> Result<(), Failure> {
    set_i(
        "read.iges.bspline.continuity",
        p.read_b_spline_continuity.as_raw(),
    )?;
    set_i("read.precision.mode", p.read_precision_mode.as_raw())?;
    set_r("read.precision.val", p.read_precision_val)?;
    set_i("read.maxprecision.mode", p.read_max_precision_mode.as_raw())?;
    set_r("read.maxprecision.val", p.read_max_precision_val)?;
    set_i(
        "read.stdsameparameter.mode",
        i32::from(p.read_same_param_mode),
    )?;
    set_i("read.surfacecurve.mode", p.read_surface_curve_mode.as_raw())?;
    set_r(
        "read.encoderegularity.angle",
        p.encode_reg_angle * PI / 180.0,
    )?;
    set_i(
        "read.iges.bspline.approxd1.mode",
        i32::from(p.read_approxd1),
    )?;
    set_i(
        "read.iges.faulty.entities",
        i32::from(p.read_faulty_entities),
    )?;
    set_i("read.iges.onlyvisible", i32::from(p.read_only_visible))?;
    set_i("write.iges.brep.mode", p.write_b_rep_mode.as_raw())?;
    set_i(
        "write.convertsurface.mode",
        p.write_convert_surface_mode.as_raw(),
    )?;
    set_c("write.iges.header.author", &text_of(&p.write_header_author))?;
    set_c(
        "write.iges.header.company",
        &text_of(&p.write_header_company),
    )?;
    set_c(
        "write.iges.header.product",
        &text_of(&p.write_header_product),
    )?;
    set_c(
        "write.iges.header.receiver",
        &text_of(&p.write_header_reciever),
    )?;
    set_i("write.precision.mode", p.write_precision_mode.as_raw())?;
    set_r("write.precision.val", p.write_precision_val)?;
    set_i("write.iges.plane.mode", p.write_plane_mode.as_raw())?;
    set_i("write.iges.offset.mode", i32::from(p.write_offset_mode))
}

/// The settings as parameters, the regularity angle in degrees.
fn read_static() -> DeIgesParameters {
    let mut values = DeIgesParameters::default();
    values.read_from_iges_statics();
    values.encode_reg_angle = InterfaceStatic::r_val("read.encoderegularity.angle") * 180.0 / PI;
    values
}

/// The settings of a transfer: the node's while it lives, the old ones after it.
pub(super) struct StaticScope {
    old_values: DeIgesParameters,
    old_unit: i32,
    _only_one: MutexGuard<'static, ()>,
}

impl StaticScope {
    pub(super) fn new(node: &DeIgesConfigurationNode) -> Result<Self, Failure> {
        let only_one = GLOBAL_IGES_LOCK
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        iges_data_init()?;
        let scope = Self {
            old_values: read_static(),
            old_unit: InterfaceStatic::i_val("xstep.cascade.unit"),
            _only_one: only_one,
        };
        let unit = node.global_parameters().length_unit;
        xs_algo_units().set_cascade_length_unit(unit, LengthUnit::Millimeter);
        let name =
            UnitsMethods::dump_length_unit_factor(unit, LengthUnit::Millimeter).to_uppercase();
        set_c("xstep.cascade.unit", &name)?;
        set_static(&node.internal_parameters.snapshot())?;
        Ok(scope)
    }
}

impl Drop for StaticScope {
    fn drop(&mut self) {
        let _ = set_i("xstep.cascade.unit", self.old_unit);
        xs_algo_units().set_cascade_length_unit_code(self.old_unit);
        let _ = set_static(&self.old_values);
    }
}
