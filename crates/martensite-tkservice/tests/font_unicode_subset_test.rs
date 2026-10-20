//! Behaviour tests of `Font_UnicodeSubset`.
#![allow(non_snake_case)]
use martensite_tkservice::font::FontUnicodeSubset;

#[test]
fn Font_UnicodeSubsetTest__CountIncludesEveryDeclaredSubset() {
    assert_eq!(4, FontUnicodeSubset::Nb as i32);
    assert!(FontUnicodeSubset::Western < FontUnicodeSubset::Nb);
    assert!(FontUnicodeSubset::Korean < FontUnicodeSubset::Nb);
    assert!(FontUnicodeSubset::Cjk < FontUnicodeSubset::Nb);
    assert!(FontUnicodeSubset::Arabic < FontUnicodeSubset::Nb);
}
