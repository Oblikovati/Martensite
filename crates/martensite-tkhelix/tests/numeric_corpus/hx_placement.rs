//! Direct frozen request execution; row positions preserve the original corpus order.

#[test]
fn hx_placement_request_000() {
    super::run_case(0..49, 0..79);
}

#[test]
fn hx_placement_request_001() {
    super::run_case(49..101, 79..212);
}

#[test]
fn hx_placement_request_002() {
    super::run_case(101..153, 212..311);
}

#[test]
fn hx_placement_request_003() {
    super::run_case(153..210, 311..536);
}

#[test]
fn hx_placement_request_004() {
    super::run_case(210..259, 536..669);
}

#[test]
fn hx_placement_request_005() {
    super::run_case(259..308, 669..697);
}

#[test]
fn hx_placement_request_006() {
    super::run_case(308..359, 697..940);
}

#[test]
fn hx_placement_request_007() {
    super::run_case(359..549, 940..1188);
}

#[test]
fn hx_placement_request_008() {
    super::run_case(549..739, 1188..1437);
}

#[test]
fn hx_placement_request_009() {
    super::run_case(739..931, 1437..1685);
}

#[test]
fn hx_placement_request_010() {
    super::run_case(931..1123, 1685..1932);
}

#[test]
fn hx_placement_request_011() {
    super::run_case(1123..1314, 1932..2181);
}

#[test]
fn hx_placement_request_012() {
    super::run_case(1314..1499, 2181..2428);
}

#[test]
fn hx_placement_request_013() {
    super::run_case(1499..1684, 2428..2675);
}

#[test]
fn hx_placement_request_014() {
    super::run_case(1684..1876, 2675..2923);
}

#[test]
fn hx_placement_request_015() {
    super::run_case(1876..2066, 2923..3173);
}

#[test]
fn hx_placement_request_016() {
    super::run_case(2066..2253, 3173..3420);
}

#[test]
fn hx_placement_request_017() {
    super::run_case(2253..2446, 3420..3669);
}

#[test]
fn hx_placement_request_018() {
    super::run_case(2446..2629, 3669..3915);
}

#[test]
fn hx_placement_request_019() {
    super::run_case(2629..2817, 3915..4162);
}

#[test]
fn hx_placement_request_020() {
    super::run_case(2817..3009, 4162..4409);
}

#[test]
fn hx_placement_request_021() {
    super::run_case(3009..3193, 4409..4655);
}

#[test]
fn hx_placement_request_022() {
    super::run_case(3193..3380, 4655..4903);
}
