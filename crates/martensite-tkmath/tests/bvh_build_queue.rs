//! Bvh build queue tests.
//! `std::thread` workers become scoped threads borrowing the queue.
#![allow(non_snake_case, unused_mut)]
use martensite_tkmath::bvh::BvhBuildQueue;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

include!("bvh_build_queue/suites_0.rs");
include!("bvh_build_queue/suites_1.rs");
