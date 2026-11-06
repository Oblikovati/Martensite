//! ChFiDS: the data structures of the fillet and chamfer builders.
//!
//! The value classes (`ChFiDSCommonPoint`, `ChFiDSFaceInterference`, ...) are plain
//! structs whose `Change*` accessors are `&mut` accessors. The transient classes that the
//! builders share and mutate through several handles (`ChFiDSSurfData`,
//! `ChFiDSStripe`, `ChFiDSSpine` and its subclasses) are reached through [`Shared`]
//! handles; `ChFiDSElSpine` is a curve adaptor, a `Handle<ElSpine>` with its state behind
//! a lock like every adaptor. The inheritance `ChFiDSSpine` -> `ChFiDSFilSpine` /
//! `ChFiDSChamfSpine` is the [`SpineObject`] trait with `Deref` to the [`Spine`] part.
mod chamf_spine;
mod circ_section;
mod common_point;
mod el_spine;
mod enums;
pub(crate) mod errors;
mod face_interference;
mod fil_spine;
mod maps;
mod regul;
mod shared;
mod spine;
mod stripe;
mod surf_data;

pub use chamf_spine::ChamfSpine;
pub use circ_section::CircSection;
pub use common_point::CommonPoint;
pub use el_spine::ElSpine;
pub use enums::{ChamfMethod, ChamfMode, ErrorStatus, State, TypeOfConcavity};
pub use face_interference::FaceInterference;
pub use fil_spine::FilSpine;
pub use maps::{Map, StripeMap};
pub use regul::Regul;
pub use shared::{Shared, lock, same_object, shared};
pub use spine::{Spine, SpineHandle, SpineObject};
pub use stripe::{Stripe, StripeHandle, SurfDataSequenceHandle};
pub use surf_data::{SurfData, SurfDataHandle};
