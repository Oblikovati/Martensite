use super::*;
use crate::surface_sketch::{
    HistoryRole, SurfaceRegionId,
    wrapped_profile::{Source, pipeline},
};
use std::cell::Cell;

#[derive(Debug)]
enum TestReadFailure {
    Refused { call: u8 },
    Native(martensite_tkernel::standard::Failure),
}
struct RefusingDecoder {
    fail_at: u8,
    reads: Cell<u8>,
}
impl RefusingDecoder {
    fn decode(&self, bytes: &[u8], keys: &[BindingKey]) -> Result<DecodedGraph, TestReadFailure> {
        let call = self.reads.get() + 1;
        self.reads.set(call);
        if call == self.fail_at {
            return Err(TestReadFailure::Refused { call });
        }
        pipeline::decode(bytes, keys).map_err(TestReadFailure::Native)
    }
}

#[test]
fn archive_placement_preserves_the_decoder_failure_and_read_stage() {
    let (bytes, keys) = fixture();
    for (fail_at, expected) in [(1, "canonical decode"), (2, "physical decode")] {
        let decoder = RefusingDecoder {
            fail_at,
            reads: Cell::new(0),
        };
        let result = place(&bytes, &keys, translation(), |bytes, keys| {
            decoder.decode(bytes, keys)
        });
        assert!(matches!(result,
            Err(ArchivePlacementFailure::Decode {
                step, cause: TestReadFailure::Refused { call },
            }) if step == expected && call == fail_at));
        assert_eq!(decoder.reads.get(), fail_at);
    }
}

#[test]
fn cylindrical_archive_decoder_keeps_native_failure_typed() {
    let decoder = RefusingDecoder {
        fail_at: 2,
        reads: Cell::new(0),
    };
    let result = place(&[], &[], translation(), |bytes, keys| {
        decoder.decode(bytes, keys)
    });
    let expected = pipeline::decode(&[], &[]).err().unwrap();
    assert_native_decoder_failure(result.err().unwrap(), expected);
    assert_eq!(decoder.reads.get(), 1);
}

fn assert_native_decoder_failure(
    failure: ArchivePlacementFailure<TestReadFailure>,
    expected: martensite_tkernel::standard::Failure,
) {
    let ArchivePlacementFailure::Decode {
        step,
        cause: TestReadFailure::Native(actual),
    } = failure
    else {
        panic!("unexpected decoder failure: {failure:?}");
    };
    assert_eq!(step, "canonical decode");
    assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
}

fn fixture() -> (Vec<u8>, Vec<BindingKey>) {
    use martensite_tkbrep::topods::{Builder, Compound};
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    let root = compound.into_shape();
    let key = BindingKey {
        source: Source::Region(SurfaceRegionId(8)),
        role: HistoryRole::Solid,
    };
    let bytes = encoding::encode(&root, &[(key, root.clone())]).unwrap();
    (bytes, vec![key])
}
fn translation() -> Transform3 {
    Transform3::from_stored_rigid_coefficients([
        [1., 0., 0., 2.],
        [0., 1., 0., 3.],
        [0., 0., 1., 4.],
    ])
    .unwrap()
}
