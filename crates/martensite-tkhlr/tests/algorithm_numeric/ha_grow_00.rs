//! Exact frozen native output, bound to original request row positions.

#[test]
fn ha_grow_request_0543() {
    super::run_case(551074..551191, 3423932..3424137);
}

#[test]
fn ha_grow_request_0544() {
    super::run_case(551191..551308, 3424137..3424342);
}

#[test]
fn ha_grow_request_0545() {
    super::run_case(551308..551425, 3424342..3424551);
}

#[test]
fn ha_grow_request_0546() {
    super::run_case(551425..551542, 3424551..3424756);
}

#[test]
fn ha_grow_request_0547() {
    super::run_case(551542..551659, 3424756..3424777);
}
