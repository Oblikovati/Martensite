//! The precision mutation stays in one isolated executable; Drop restores even on assertion unwind.
use super::construction_corpus::replay::{checked_request, fixture_hash};
use martensite_tkgeombase::adv_app2var::math_base as mb;
const REQUESTS: &str = include_str!("../data/polynomial_arithmetic_corpus/requests.txt");
const ANSWERS: &str = include_str!("../data/polynomial_arithmetic_corpus/answers.txt");
struct PrecisionRestore(mb::Mmprcsn);
impl Drop for PrecisionRestore {
    fn drop(&mut self) {
        let p = self.0;
        mb::mmwprcs_(p.eps1, p.eps2, p.eps3, p.eps4, p.niterm, p.niterr);
    }
}
pub fn replay_sequence() {
    let _restore = PrecisionRestore(mb::mmprcsn());
    assert_eq!(fixture_hash([REQUESTS].into_iter()), 12384740419766685367);
    assert_eq!(fixture_hash([ANSWERS].into_iter()), 14118886555394596554);
    assert_eq!(REQUESTS.lines().count(), 198);
    assert_eq!(ANSWERS.lines().count(), 198);
    for (request, expected) in REQUESTS.lines().zip(ANSWERS.split_inclusive('\n')) {
        let family = request.split_whitespace().next().unwrap();
        assert!(checked_request(request, expected, family).is_some());
    }
    assert_eq!(mb::mmprcsn(), mb::MMPRCSN_DEFAULT);
}
fn real_output(operation: &str, values: &[f64]) -> Vec<f64> {
    super::construction_corpus::run_values(operation, values)
        .unwrap()
        .split_whitespace()
        .map(|s| f64::from_bits(s.parse().unwrap()))
        .collect()
}
pub fn independent_invariants() {
    let normalized = real_output("aa_norms", &[2., 3., 4., 5., 12., 2., 3.]);
    assert_eq!(&normalized[..3], &[5., 63., 8.]);
    let point = real_output(
        "aa_mmpocrb",
        &[3., 3., 2., 2., 1., 2., 99., 3., 4., 99., 5., 6., 99.],
    );
    assert_eq!(point, [27., 34.]); // [1+3t+5t², 2+4t+6t²] at t=2.
    assert_eq!(real_output("aa_mdsptpt", &[2., 1., 2., 4., 6.]), [5.]);
}
pub fn independent_layout_and_reader() {
    assert_eq!(
        real_output("aa_mmcvinv", &[3., 3., 2., 1., 3., 5., 2., 4., 6.]),
        [1., 2., 7.5, 3., 4., 7.5, 5., 6., 7.5]
    );
    let mut reads = 0;
    let mut next = || {
        reads += 1;
        2.
    };
    assert_eq!(
        super::construction_corpus::math_base::read_reals(3, &mut next),
        [2.; 3]
    );
    assert!(super::construction_corpus::math_base::read_reals(-1, &mut next).is_empty());
    assert_eq!(reads, 3);
    assert_eq!(super::construction_corpus::math_base::fill(-1), [7.5]);
}

pub fn restoration_on_unwind() {
    let before = mb::mmprcsn();
    let failure = std::panic::catch_unwind(|| {
        let _restore = PrecisionRestore(before);
        mb::mmwprcs_(1., 2., 3., 4., 5, 6);
        panic!("named injected assertion unwind");
    });
    assert!(failure.is_err());
    assert_eq!(mb::mmprcsn(), before);
}
