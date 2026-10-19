//! Adv app2var node contracts.
#![allow(non_snake_case)]
#[path = "support/geometric_test_support/mod.rs"]
mod common;
use common::EXPECT_DOUBLE_EQ;
use martensite_tkgeombase::adv_app2var::AdvApp2VarNode;

#[test]
fn node_constructor_initializes_values_to_zero() {
    let aNode = AdvApp2VarNode::with_orders(1, 2);
    for aUOrder in 0..=1 {
        for aVOrder in 0..=2 {
            let aPoint = aNode.point(aUOrder, aVOrder).unwrap();
            EXPECT_DOUBLE_EQ(aPoint.x(), 0.0);
            EXPECT_DOUBLE_EQ(aPoint.y(), 0.0);
            EXPECT_DOUBLE_EQ(aPoint.z(), 0.0);
            EXPECT_DOUBLE_EQ(aNode.error(aUOrder, aVOrder).unwrap(), 0.0);
        }
    }
}
