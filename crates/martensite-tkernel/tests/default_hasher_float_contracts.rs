//! Exact default-hasher bits of the Linux x86-64 libstdc++ hash.
use martensite_tkernel::ncollection::{DataMap, DefaultHashPolicy, HashKey, HashPolicy};

#[cfg(all(
    target_os = "linux",
    target_pointer_width = "64",
    target_endian = "little"
))]
fn native_float_rows(prefix: &str) -> Vec<(u64, usize, bool)> {
    include_str!("fixtures/oracle/tkernel_default_hasher_float_contracts.expected")
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            (fields.next()? == prefix).then(|| {
                let bits = fields.next().unwrap().parse().unwrap();
                let hash = fields.next().unwrap().parse().unwrap();
                let reflexive = fields.next().unwrap() == "1";
                (bits, hash, reflexive)
            })
        })
        .collect()
}

#[test]
#[cfg(all(
    target_os = "linux",
    target_pointer_width = "64",
    target_endian = "little"
))]
fn float32_exact_bits_match_native_zeros_finite_infinities_and_nan_payloads() {
    let rows = native_float_rows("f32");
    assert_eq!(rows.len(), 18);
    for (bits, expected_hash, reflexive) in rows {
        let value = f32::from_bits(bits as u32);
        assert_eq!(value.kernel_hash(), expected_hash, "f32 bits {bits:#010x}");
        assert_eq!(DefaultHashPolicy.equal(&value, &value), reflexive);
    }
}

#[test]
#[cfg(all(
    target_os = "linux",
    target_pointer_width = "64",
    target_endian = "little"
))]
fn float64_shared_dispatch_keeps_existing_native_bit_hashes() {
    let rows = native_float_rows("f64");
    assert_eq!(rows.len(), 9);
    for (bits, expected_hash, reflexive) in rows {
        let value = f64::from_bits(bits);
        assert_eq!(value.kernel_hash(), expected_hash, "f64 bits {bits:#018x}");
        assert_eq!(DefaultHashPolicy.equal(&value, &value), reflexive);
    }
}

#[test]
fn float_default_map_merges_signed_zero_and_retains_nonreflexive_nan_keys() {
    let mut map = DataMap::<f32, i32>::default();
    assert!(map.bind(0.0, 1));
    assert!(!map.bind(-0.0, 2));
    assert_eq!(map.len(), 1);
    assert_eq!(map.find(&0.0), Ok(&2));
    assert!(map.bind(1.25, 3));
    assert_eq!(map.find(&1.25), Ok(&3));
    let nan = f32::from_bits(0x7fc0_0001);
    assert!(map.bind(nan, 4));
    assert!(map.bind(nan, 5));
    assert_eq!(map.len(), 4);
    assert!(!map.is_bound(&nan));
}

#[test]
fn native_explicit_integer_specializations_match_rust_width_representations() {
    assert_eq!(true.kernel_hash(), 1);
    for hash in [
        (-1_i8).kernel_hash(),
        (-1_i16).kernel_hash(),
        (-1_i32).kernel_hash(),
        (-1_i64).kernel_hash(),
        (-1_isize).kernel_hash(),
    ] {
        assert_eq!(hash, usize::MAX);
    }
    assert_eq!(u8::MAX.kernel_hash(), 255);
    assert_eq!(u16::MAX.kernel_hash(), 65535);
    assert_eq!(u32::MAX.kernel_hash(), u32::MAX as usize);
    assert_eq!(u64::MAX.kernel_hash(), usize::MAX);
    assert_eq!(usize::MAX.kernel_hash(), usize::MAX);
    assert_eq!('\u{10ffff}'.kernel_hash(), 0x10ffff);
}

#[test]
fn native_pointer_specialization_hashes_address_and_null_directly() {
    let pointed = 1;
    let pointer = std::ptr::from_ref(&pointed);
    assert_eq!(pointer.kernel_hash(), pointer as usize);
    assert_eq!(std::ptr::null::<i32>().kernel_hash(), 0);
    assert_eq!(std::ptr::null_mut::<i32>().kernel_hash(), 0);
    assert_eq!(pointer.cast_mut().kernel_hash(), pointer.kernel_hash());
    assert!(DefaultHashPolicy.equal(&pointer, &pointer));
}
