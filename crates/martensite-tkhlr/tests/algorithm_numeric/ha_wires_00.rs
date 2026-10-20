//! Exact frozen native output, bound to original request row positions.

#[test]
fn ha_wires_request_0103() {
    super::run_case(27414..27536, 64829..64933);
}

#[test]
fn ha_wires_request_0104() {
    super::run_case(27536..27690, 64933..65046);
}

#[test]
fn ha_wires_request_0105() {
    super::run_case(27690..27895, 65046..65163);
}
