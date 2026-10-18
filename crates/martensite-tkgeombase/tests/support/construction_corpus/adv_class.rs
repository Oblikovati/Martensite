//! The AdvApp2Var class probe (Context, Node, Iso, Patch, Network, Framework and
//! ApproxAFunc2Var); the counterpart of the implementation.
use super::adv_eval::Function;
use super::common::{Reader, attempt, end, flag, int, pnt, real, xy};
use super::print::Output;
use martensite_tkernel::ncollection::{Array1, Array2, Sequence};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adv_approx::{AdvApproxCutting, AdvApproxDichoCutting, AdvApproxPrefAndRec};
use martensite_tkgeombase::adv_app2var::{
    AdvApp2VarApproxAFunc2Var, AdvApp2VarContext, AdvApp2VarCriterion,
    AdvApp2VarCriterionRepartition, AdvApp2VarCriterionType, AdvApp2VarFramework, AdvApp2VarIso,
    AdvApp2VarNetwork, AdvApp2VarNode, AdvApp2VarPatch, FrontierTolerances, SubspaceTolerances,
};
use martensite_tkmath::geom_abs::{IsoType, Shape};
use martensite_tkmath::gp::Xy;

const ISO_TYPES: [IsoType; 3] = [IsoType::IsoU, IsoType::IsoV, IsoType::NoneIso];
const SHAPES: [Shape; 7] = [
    Shape::C0,
    Shape::G1,
    Shape::C1,
    Shape::G2,
    Shape::C2,
    Shape::C3,
    Shape::CN,
];

include!("adv_class/context.rs");
include!("adv_class/grid.rs");
include!("adv_class/patch.rs");
include!("adv_class/approximation.rs");
