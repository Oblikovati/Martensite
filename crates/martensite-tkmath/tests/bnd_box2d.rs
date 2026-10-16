//! Complete BoundingBox2 suite.
#![allow(non_snake_case, unused_mut, unused_assignments, unused_variables)]
use martensite_tkmath::{
    bnd::{BoundingBox2, BoxLimits2},
    gp::{Direction2, Line2, Point2, Transform2},
};
include!("bnd_box2d/suites_01.rs");
include!("bnd_box2d/suites_11.rs");
include!("bnd_box2d/suites_21.rs");
include!("bnd_box2d/suites_31.rs");
