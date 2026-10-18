//! Adv app2var iso contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_DOUBLE_EQ;
use martensite_tkgeombase::adv_app2var::AdvApp2VarIso;
use martensite_tkmath::geom_abs::IsoType;

#[test]
fn iso_constructor_with_orders_assigns_orders_by_iso_type() {
    let anIsoU = AdvApp2VarIso::with_orders(IsoType::IsoU, 3, 1);
    assert_eq!(anIsoU.iso_type(), IsoType::IsoU);
    assert_eq!(anIsoU.u_order(), 3);
    assert_eq!(anIsoU.v_order(), 1);
    EXPECT_DOUBLE_EQ(anIsoU.constante(), 0.5);
    EXPECT_DOUBLE_EQ(anIsoU.u0(), 0.0);
    EXPECT_DOUBLE_EQ(anIsoU.u1(), 1.0);
    EXPECT_DOUBLE_EQ(anIsoU.v0(), 0.0);
    EXPECT_DOUBLE_EQ(anIsoU.v1(), 1.0);
    EXPECT_DOUBLE_EQ(anIsoU.t0(), 0.0);
    EXPECT_DOUBLE_EQ(anIsoU.t1(), 1.0);
    assert_eq!(anIsoU.position(), 0);

    let anIsoV = AdvApp2VarIso::with_orders(IsoType::IsoV, 3, 1);
    assert_eq!(anIsoV.iso_type(), IsoType::IsoV);
    assert_eq!(anIsoV.u_order(), 3);
    assert_eq!(anIsoV.v_order(), 1);
}
