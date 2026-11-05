//! A copied attribute list owns its map while retaining the attribute handles.
use martensite_tkxsbase::moni_tool::{MoniToolAttrList, MoniToolIntVal};

#[test]
fn copy_construction_shares_attribute_values_and_keeps_map_edits_independent() {
    let mut source = MoniToolAttrList::new();
    source.set_integer_attribute("count", 17);
    let mut copied = source.clone();
    assert!(source.attribute("count") == copied.attribute("count"));
    *copied.attribute("count").cast::<MoniToolIntVal>().c_value() = 29;
    assert_eq!(source.integer_attribute("count"), 29);
    assert_eq!(copied.integer_attribute("count"), 29);
    assert!(copied.remove_attribute("count"));
    assert_eq!(source.integer_attribute("count"), 29);
    assert_eq!(copied.integer_attribute("count"), 0);
    copied.set_integer_attribute("other", 31);
    assert_eq!(source.integer_attribute("other"), 0);
}
