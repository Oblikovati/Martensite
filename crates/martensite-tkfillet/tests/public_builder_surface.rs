//! External-consumer witnesses for the public builder receiver and method signatures.
use martensite_tkfillet::chfi3d::{ChFi3dBuilder, ChFi3dBuilderOps};

#[test]
fn declared_public_builder_members_are_externally_reachable() {
    let split = ChFi3dBuilder::split_kpart;
    let corner = ChFi3dBuilder::perform_two_cornerby_inter;
    std::hint::black_box((split, corner));
}

#[test]
fn computation_uses_the_actual_dynamic_operation_receiver() {
    let compute = <dyn ChFi3dBuilderOps>::compute;
    std::hint::black_box(compute);
}

#[path = "public_builder_restrictions.rs"]
mod restrictions;
