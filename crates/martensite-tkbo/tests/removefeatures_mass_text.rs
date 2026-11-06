//! Independent reference stream baselines for parsed six-significant-digit property masses.
#[path = "removefeatures_cases/mass_text.rs"]
mod mass_text;

#[test]
fn mass_text_matches_cpp_ties_and_extreme_exponents() {
    for row in include_str!("removefeatures_cases/mass_text_cpp.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();
        let input = f64::from_bits(u64::from_str_radix(fields[0], 16).unwrap());
        let expected = u64::from_str_radix(fields[1], 16).unwrap();
        assert_eq!(
            mass_text::draw_mass(input).to_bits(),
            expected,
            "reference mass {} for input {input:?}",
            fields[2]
        );
    }
}
