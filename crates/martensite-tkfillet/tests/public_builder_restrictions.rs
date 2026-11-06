//! Concrete chamfer receivers retain the native restriction-overload failures.
#[path = "support/public_builder_restrictions.rs"]
mod fixture;
use fixture::{assert_performance, assert_simulation};
use martensite_tkfillet::chfi3d::{ChFi3dBuilderOps, ChFi3dChBuilder};

#[test]
fn first_restriction_simulation_keeps_the_native_failure() {
    let method = <ChFi3dChBuilder as ChFi3dBuilderOps>::simul_surf_rst1;
    assert_simulation(method, "simul_surf_rst1");
}

#[test]
fn second_restriction_simulation_keeps_the_native_failure() {
    let method = <ChFi3dChBuilder as ChFi3dBuilderOps>::simul_surf_rst2;
    assert_simulation(method, "simul_surf_rst2");
}

#[test]
fn two_restriction_simulation_keeps_the_native_failure() {
    let method = <ChFi3dChBuilder as ChFi3dBuilderOps>::simul_surf_rst_rst;
    assert_simulation(method, "simul_surf_rst_rst");
}

#[test]
fn first_restriction_performance_keeps_the_native_failure() {
    let method = <ChFi3dChBuilder as ChFi3dBuilderOps>::perform_surf_rst1;
    assert_performance(method, "perform_surf_rst1");
}

#[test]
fn second_restriction_performance_keeps_the_native_failure() {
    let method = <ChFi3dChBuilder as ChFi3dBuilderOps>::perform_surf_rst2;
    assert_performance(method, "perform_surf_rst2");
}

#[test]
fn two_restriction_performance_keeps_the_native_failure() {
    let method = <ChFi3dChBuilder as ChFi3dBuilderOps>::perform_surf_rst_rst;
    assert_performance(method, "perform_surf_rst_rst");
}

#[test]
fn public_dynamic_adaptor_keeps_the_empty_contour_failure() {
    let expected = "There are no suitable edges for chamfer or fillet";
    fixture::assert_computation(expected);
}

#[test]
fn automatic_builder_destruction_releases_its_input_references() {
    let observation = fixture::builder_lifetime_observation();
    assert_eq!(observation, (true, true));
}
