//! Exact frozen native output, bound to original request row positions.

#[test]
fn ha_edges_request_0099() {
    super::run_case(26063..26224, 63221..63579);
}

#[test]
fn ha_edges_request_0100() {
    super::run_case(26224..26486, 63579..63961);
}

#[test]
fn ha_edges_request_0101() {
    super::run_case(26486..26851, 63961..64372);
}

#[test]
fn ha_edges_request_0102() {
    super::run_case(26851..27414, 64372..64829);
}
