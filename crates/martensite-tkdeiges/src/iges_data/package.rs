//! The set-up of the IGES data package: the basic protocol, its modules, the session's parameters
//! and the template model.
use super::{
    IgesDataDefaultGeneral, IgesDataDefaultSpecific, IgesDataGeneralModule, IgesDataGlobalSection,
    IgesDataIgesModel, IgesDataProtocol, IgesDataProtocolOps, IgesDataSpecificLib,
    IgesDataSpecificModule,
};
use martensite_tkernel::{
    osd::Process,
    standard::{Failure, Handle},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::{
    InterfaceGeneralLib, InterfaceInterfaceModel, InterfaceModelBase, InterfaceProtocol,
    InterfaceStatic,
};
use std::sync::{Mutex, OnceLock, PoisonError};

/// The product line of the files this kernel writes.
const PROCESSOR_VERSION: &str = concat!("Martensite IGES processor ", env!("CARGO_PKG_VERSION"));
/// The system that writes the files.
const SYSTEM_VERSION: &str = concat!("Martensite ", env!("CARGO_PKG_VERSION"));

/// The parameters of the shape fixing process of a file that is read.
const FROM_IGES_FIX_SHAPE: &[(&str, &str)] = &[
    ("Tolerance3d", "&Runtime.Tolerance"),
    ("MaxTolerance3d", "&Runtime.MaxTolerance"),
    ("MinTolerance3d", "1.e-7"),
    ("FixFreeShellMode", "-1"),
    ("FixFreeFaceMode", "-1"),
    ("FixFreeWireMode", "-1"),
    ("FixSameParameterMode", "-1"),
    ("FixSolidMode", "-1"),
    ("FixShellOrientationMode", "-1"),
    ("CreateOpenSolidMode", "1"),
    ("FixShellMode", "-1"),
    ("FixFaceOrientationMode", "-1"),
    ("FixFaceMode", "-1"),
    ("FixWireMode", "-1"),
    ("FixOrientationMode", "-1"),
    ("FixAddNaturalBoundMode", "-1"),
    ("FixMissingSeamMode", "-1"),
    ("FixSmallAreaWireMode", "-1"),
    ("RemoveSmallAreaFaceMode", "-1"),
    ("FixIntersectingWiresMode", "-1"),
    ("FixLoopWiresMode", "-1"),
    ("FixSplitFaceMode", "-1"),
    ("AutoCorrectPrecisionMode", "1"),
    ("ModifyTopologyMode", "0"),
    ("ModifyGeometryMode", "1"),
    ("ClosedWireMode", "1"),
    ("PreferencePCurveMode", "1"),
    ("FixReorderMode", "-1"),
    ("FixSmallMode", "-1"),
    ("FixConnectedMode", "-1"),
    ("FixEdgeCurvesMode", "-1"),
    ("FixDegeneratedMode", "-1"),
    ("FixLackingMode", "-1"),
    ("FixSelfIntersectionMode", "-1"),
    ("RemoveLoopMode", "-1"),
    ("FixReversed2dMode", "-1"),
    ("FixRemovePCurveMode", "-1"),
    ("FixRemoveCurve3dMode", "-1"),
    ("FixAddPCurveMode", "-1"),
    ("FixAddCurve3dMode", "-1"),
    ("FixSeamMode", "-1"),
    ("FixShiftedMode", "-1"),
    ("FixEdgeSameParameterMode", "0"),
    ("FixNotchedEdgesMode", "-1"),
    ("FixTailMode", "0"),
    ("MaxTailAngle", "0.0"),
    ("MaxTailWidth", "-1.0"),
    ("FixSelfIntersectingEdgeMode", "-1"),
    ("FixIntersectingEdgesMode", "-1"),
    ("FixNonAdjacentIntersectingEdgesMode", "-1"),
    ("FixVertexPositionMode", "0"),
    ("FixVertexToleranceMode", "-1"),
];

fn init_static(name: &str, kind: u8, init: &str) -> Result<(), Failure> {
    InterfaceStatic::init_char("XSTEP", name, kind, init).map(|_| ())
}

fn init_text(name: &str, value: &str) -> Result<(), Failure> {
    init_static(name, b't', value)
}

/// An enumeration parameter whose values start at 0, set to its first value.
fn init_switch(name: &str, values: [&str; 2]) -> Result<(), Failure> {
    init_static(name, b'e', "")?;
    init_static(name, b'&', "ematch 0")?;
    for value in values {
        init_static(name, b'&', &format!("eval {value}"))?;
    }
    InterfaceStatic::set_i_val(name, 0).map(|_| ())
}

fn init_session_parameters() -> Result<(), Failure> {
    InterfaceStatic::standards()?;
    init_switch("read.iges.bspline.approxd1.mode", ["Off", "On"])?;
    init_static("read.iges.bspline.continuity", b'i', "1")?;
    init_static("read.iges.bspline.continuity", b'&', "imin 0")?;
    init_static("read.iges.bspline.continuity", b'&', "imax 2")?;
    init_text("write.iges.header.receiver", "")?;
    let mut process = Process::new();
    init_text(
        "write.iges.header.author",
        &String::from_utf8_lossy(process.user_name().as_bytes()),
    )?;
    init_text("write.iges.header.company", "")?;
    init_static("write.iges.unit", b'e', "")?;
    init_static("write.iges.unit", b'&', "enum 1")?;
    for unit in [
        "INCH", "MM", "??", "FT", "MI", "M", "KM", "MIL", "UM", "CM", "UIN",
    ] {
        init_static("write.iges.unit", b'&', &format!("eval {unit}"))?;
    }
    InterfaceStatic::set_c_val("write.iges.unit", "MM")?;
    init_switch("write.iges.brep.mode", ["Faces", "BRep"])?;
    init_switch("write.convertsurface.mode", ["Off", "On"])?;
    init_text("write.iges.resource.name", "IGES")?;
    init_text("read.iges.resource.name", "IGES")?;
    init_text("write.iges.sequence", "ToIGES")?;
    init_text("read.iges.sequence", "FromIGES")?;
    init_text("ToIGES.exec.op", "DirectFaces")?;
    init_text("FromIGES.exec.op", "FixShape")?;
    for (name, value) in FROM_IGES_FIX_SHAPE {
        init_text(&format!("FromIGES.FixShape.{name}"), value)?;
    }
    init_switch("read.iges.onlyvisible", ["Off", "On"])?;
    init_switch("read.iges.faulty.entities", ["Off", "On"])?;
    init_switch("write.iges.plane.mode", ["Plane", "BSpline"])?;
    init_switch("write.iges.offset.mode", ["On", "Off"])?;
    init_text("write.iges.header.product", PROCESSOR_VERSION)
}

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

fn static_text(name: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(InterfaceStatic::c_val(name).as_bytes()))
}

/// The global section of the template model.
fn template_section() -> IgesDataGlobalSection {
    let date = IgesDataGlobalSection::new_date_string(0, 0, 0, 0, 0, 0, -1);
    let mut section = IgesDataGlobalSection::new();
    section.set_separator(b',');
    section.set_end_mark(b';');
    section.set_send_name(&static_text("write.iges.header.product"));
    section.set_file_name(&text("Filename.iges"));
    section.set_system_id(&text(SYSTEM_VERSION));
    section.set_interface_version(&text(PROCESSOR_VERSION));
    section.set_integer_bits(i32::BITS as i32);
    section.set_max_power10_single(f64::MAX_10_EXP);
    section.set_max_digits_single(f64::DIGITS as i32);
    section.set_max_power10_double(f64::MAX_10_EXP);
    section.set_max_digits_double(f64::DIGITS as i32);
    section.set_receive_name(&static_text("write.iges.header.receiver"));
    section.set_scale(1.0);
    section.set_unit_flag(InterfaceStatic::i_val("write.iges.unit"));
    section.set_unit_name(&static_text("write.iges.unit"));
    section.set_line_weight_grad(1);
    section.set_max_line_weight(0.01);
    section.set_date(&date);
    section.set_resolution(0.0001);
    section.set_max_coord(0.0);
    section.set_author_name(&static_text("write.iges.header.author"));
    section.set_company_name(&static_text("write.iges.header.company"));
    section.set_iges_version(11);
    section.set_drafting_standard(0);
    section.set_last_change_date(&date);
    section.set_application_protocol(&text(""));
    section
}

/// The basic protocol of the norm, set up by [`iges_data_init`] when not done.
///
/// ```
/// use martensite_tkdeiges::iges_data::iges_data_protocol;
/// assert!(iges_data_protocol().is_some());
/// ```
pub fn iges_data_protocol() -> Handle<IgesDataProtocol> {
    static PROTOCOL: OnceLock<Handle<IgesDataProtocol>> = OnceLock::new();
    PROTOCOL
        .get_or_init(|| {
            let protocol = Handle::new(IgesDataProtocol::default());
            let general: Handle<dyn InterfaceProtocol> = protocol.cast();
            let iges: Handle<dyn IgesDataProtocolOps> = protocol.cast();
            let default_general = Handle::new(IgesDataDefaultGeneral::default());
            let module: Handle<dyn martensite_tkxsbase::interface::InterfaceGeneralModule> =
                default_general.cast();
            debug_assert!(
                default_general
                    .cast::<dyn IgesDataGeneralModule>()
                    .is_some()
            );
            InterfaceGeneralLib::set_global(&module, &general);
            let specific: Handle<dyn IgesDataSpecificModule> =
                Handle::new(IgesDataDefaultSpecific::default()).cast();
            IgesDataSpecificLib::set_global(&specific, &iges);
            protocol
        })
        .clone()
}

/// Sets up the package: the protocol and its modules, the session's parameters, the template model.
///
/// # Errors
/// A failure of the session's parameters.
pub fn iges_data_init() -> Result<(), Failure> {
    // The parameters and the template are process-wide: one thread sets them, the others wait.
    static INITIALIZING: Mutex<()> = Mutex::new(());
    let _only_one = INITIALIZING.lock().unwrap_or_else(PoisonError::into_inner);
    let _protocol = iges_data_protocol();
    if InterfaceModelBase::has_template("iges") {
        return Ok(());
    }
    init_session_parameters()?;
    let model = IgesDataIgesModel::new();
    if let Some(live) = model.get() {
        live.set_global_section(&template_section());
    }
    let template: Handle<dyn InterfaceInterfaceModel> = model.cast();
    InterfaceModelBase::set_template("iges", &template);
    Ok(())
}
