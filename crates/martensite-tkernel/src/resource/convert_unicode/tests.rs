use super::{
    euc_to_unicode, gb_to_unicode, sjis_to_unicode, unicode_to_euc, unicode_to_gb, unicode_to_sjis,
};

#[test]
fn double_byte_decoders_keep_ascii_pairs() {
    let ascii_pair = (0x41, 0x42);
    for decode in [sjis_to_unicode, euc_to_unicode, gb_to_unicode] {
        assert_eq!(decode(ascii_pair), ascii_pair);
    }
}

#[test]
fn double_byte_encoders_keep_null_pairs() {
    for encode in [unicode_to_sjis, unicode_to_euc, unicode_to_gb] {
        assert_eq!(encode((0, 0)), (0, 0));
    }
}
