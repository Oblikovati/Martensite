//! What the RW classes of the STEP entity packages share.
//!
//! Each class is a stateless `read_step`, `WriteStep` and `share`, so `step_rw_class!` keeps
//! only the name and its `new`. Each member returns `result`: reading a parameter outside its record
//! is an out of range failure, and an entity member can fail with the null dereference.
//!
//! This module is also the prelude of the generated RW files: each of them names entities of any
//! package through the bodies it shares, so one import brings them all.
#![allow(
    unused_imports,
    reason = "a prelude: each generated RW file uses a part of it"
)]
pub(crate) use crate::rw_shared_bodies::*;
pub(crate) use crate::rw_shared_bodies_visual::*;
pub(crate) use crate::rw_step_ap203::*;
pub(crate) use crate::rw_step_ap214::*;
pub(crate) use crate::rw_step_ap242::*;
pub(crate) use crate::rw_step_basic::*;
pub(crate) use crate::rw_step_dim_tol::*;
pub(crate) use crate::rw_step_element::*;
pub(crate) use crate::rw_step_fea::*;
pub(crate) use crate::rw_step_geom::*;
pub(crate) use crate::rw_step_kinematics::*;
pub(crate) use crate::rw_step_repr::*;
pub(crate) use crate::rw_step_shape::*;
pub(crate) use crate::rw_step_visual::*;
pub(crate) use crate::step_ap203::*;
pub(crate) use crate::step_ap214::*;
pub(crate) use crate::step_ap242::*;
pub(crate) use crate::step_basic::*;
pub(crate) use crate::step_data::*;
pub(crate) use crate::step_dim_tol::*;
pub(crate) use crate::step_element::*;
pub(crate) use crate::step_entity::*;
pub(crate) use crate::step_fea::*;
pub(crate) use crate::step_geom::*;
pub(crate) use crate::step_kinematics::*;
pub(crate) use crate::step_members::StepReaderComplexNames;
pub(crate) use crate::step_repr::*;
pub(crate) use crate::step_shape::*;
pub(crate) use crate::step_visual::*;
pub(crate) use martensite_tkernel::tcollection::AsciiString;
pub(crate) use martensite_tkxsbase::interface::{
    InterfaceCheck, InterfaceEntityIterator, InterfaceFileReaderDataParams, InterfaceParamType,
    InterfaceShareTool,
};

/// An optional real of a record: its value and whether it is there, 0 when it is not.
pub(crate) fn step_read_optional_real(
    data: &StepDataStepReaderData,
    num: i32,
    param: i32,
    field: &str,
    ach: &Handle<InterfaceCheck>,
) -> Result<(f64, bool), Failure> {
    if !data.is_param_defined(num, param)? {
        return Ok((0.0, false));
    }
    let mut value = 0.0;
    data.read_real(num, param, field, ach, &mut value)?;
    Ok((value, true))
}

/// Writes an optional real: its value when it is there, the undefined parameter when it is not.
pub(crate) fn step_send_optional_real(
    sw: &mut StepDataStepWriter,
    has: bool,
    value: f64,
) -> Result<(), Failure> {
    if has {
        sw.send_real(value)
    } else {
        sw.send_undef()
    }
}

/// An optional actuated direction of a record: its value and whether it is there. A text that is no
/// value of the enumeration adds a failure message and leaves the default. `reported` and `field`
/// name the parameter in the messages.
pub(crate) fn step_read_actuated_direction(
    data: &StepDataStepReaderData,
    num: i32,
    param: i32,
    reported: i32,
    field: &str,
    ach: &Handle<InterfaceCheck>,
) -> Result<(StepKinematicsActuatedDirection, bool), Failure> {
    let mut direction = StepKinematicsActuatedDirection::AdNotActuated;
    if !data.is_param_defined(num, param)? {
        return Ok((direction, false));
    }
    if data.param_type(num, param)? != InterfaceParamType::ParamEnum {
        ach.add_fail_text(
            &format!("Parameter #{reported} ({field}) is not enumeration"),
            "",
        );
        return Ok((direction, true));
    }
    match data.param_c_value(num, param)?.as_slice() {
        b".BIDIRECTIONAL." => direction = StepKinematicsActuatedDirection::AdBidirectional,
        b".POSITIVE_ONLY." => direction = StepKinematicsActuatedDirection::AdPositiveOnly,
        b".NEGATIVE_ONLY." => direction = StepKinematicsActuatedDirection::AdNegativeOnly,
        b".NOT_ACTUATED." => {}
        _ => ach.add_fail_text(
            &format!("Parameter #{reported} ({field}) has not allowed value"),
            "",
        ),
    }
    Ok((direction, true))
}

/// Declares a stateless RW class.
macro_rules! step_rw_class {
    ($class:ident, $recorded:literal) => {
        #[doc = concat!("`", $recorded, "`: reads and writes one entity type of a STEP file.")]
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $class;
        impl $class {
            #[doc = concat!("`", $recorded, "()`.")]
            pub fn new() -> Self {
                Self
            }
        }
    };
}
pub(crate) use step_rw_class;

/// Declares the two text conversions of an enumeration, from its table of STEP texts.
macro_rules! step_enum_text {
    ($class:ident, $enum:ty, $recorded:literal, { $($variant:ident => $text:literal),+ $(,)? }) => {
        #[doc = concat!("`", $recorded, "`: the STEP texts of an enumeration.")]
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $class;
        impl $class {
            /// `convert_to_string`: the STEP text of a value.
            pub fn convert_to_string(value: $enum) -> &'static [u8] {
                match value {
                    $(<$enum>::$variant => $text.as_bytes(),)+
                }
            }
            /// `convert_to_enum`: true and the value when the text is one of the table, else false.
            pub fn convert_to_enum(text: &[u8], result: &mut $enum) -> bool {
                $(if text == $text.as_bytes() {
                    *result = <$enum>::$variant;
                    return true;
                })+
                false
            }
        }
    };
}
pub(crate) use step_enum_text;

/// `start_entity(type)` from the literal.
pub(crate) fn step_start_entity(
    writer: &mut crate::step_data::StepDataStepWriter,
    source_type: &str,
) -> Result<(), crate::step_entity::Failure> {
    writer.start_entity(&AsciiString::new(source_type))
}
