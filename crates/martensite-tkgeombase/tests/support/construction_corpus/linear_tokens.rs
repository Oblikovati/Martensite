//! One vector reader and scalar/text serializers preserve original token and bound semantics.
use super::{common as p, fixture_input::GeometryFixtureInput, print::Output};
use martensite_tkmath::math::{IntegerVector, Vector};

pub fn vector(output: &Output, v: &Vector<'_>) {
    p::int(output, v.lower());
    p::int(output, v.upper());
    (v.lower()..=v.upper()).for_each(|i| p::real(output, v[i]));
}

pub fn read_vector(r: &mut impl GeometryFixtureInput) -> Vector<'static> {
    let (lower, length) = (r.integer(), r.integer());
    let mut v = Vector::with_bounds(lower, lower + length - 1).unwrap();
    (0..length).for_each(|i| v[lower + i] = r.real());
    v
}

pub fn ivector(output: &Output, v: &IntegerVector<'_>) {
    p::int(output, v.lower());
    p::int(output, v.upper());
    (v.lower()..=v.upper()).for_each(|i| p::int(output, v[i]));
}

pub fn dump(output: &Output, text: &str) {
    output.write(format_args!("{} ", text.replace('\n', "|")));
}
