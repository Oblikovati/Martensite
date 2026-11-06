//! `feat_featprism_cpu700`: each statement is one line of its script. Generated; hand cases are in `feat_featprism_cpu700_hand`.
#[allow(unused_imports)]
use crate::support::{
    blend::blend,
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::{
        compound, explode, make_box, pcone, pcylinder, prism, psphere, trotate, ttranslate,
    },
    construction::{copy, edge, fsameparameter, mkplane, mkplane_only_plane, vertex, wire},
    feat_definitions::{
        addslide_prism, addslide_revol, featdprism, featlf, featprism, featrevol, featrf,
    },
    feat_perform::{
        Kas, featperform_all, featperform_from_until, featperform_until, featperformval,
        featperformval_until,
    },
    feat_script::{FeatScript, caught},
    inspect::{renamevar, whatis},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcut, bfuse, bsection, bsection_no_approximation},
    profile::{Code, profile},
    properties::Prop,
    restore::restore,
    sorted_explode::nexplode,
    splitshape::splitshape,
    surface_variables::{
        circle, mkedge, mkface, mkface_bounded, mksurface, plane, plane_with_x, plane_xoy,
        restore_surface, trim,
    },
};
#[allow(unused_imports)]
use martensite_tkg3d::top_abs::ShapeEnum;

mod part_01;
pub use part_01::*;
