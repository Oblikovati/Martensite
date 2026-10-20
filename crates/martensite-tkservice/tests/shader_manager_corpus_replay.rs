//! Frozen shader manager requests replay against digests of their recorded program text.
//!
//! Each request builds one standard program (or light key, lighting function or point sprite
//! function) and prints its state and every stage source; the fixture holds the FNV-1a 64
//! digest and the byte length of each recorded output line.
#[path = "support/service_corpus/mod.rs"]
mod service_corpus;

const REQUESTS: &str = include_str!("fixtures/shader_manager_corpus/requests.txt");
const DIGESTS: &str = include_str!("fixtures/shader_manager_corpus/digests.txt");

fn digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

fn expected(line: &str) -> (u64, usize) {
    let (hash, length) = line.split_once(' ').expect("`digest length` fixture lines");
    let hash = u64::from_str_radix(hash, 16).expect("hexadecimal digest");
    (hash, length.parse().expect("decimal length"))
}

#[test]
fn every_shader_manager_request_reproduces_its_recorded_text() {
    assert_eq!(REQUESTS.lines().count(), 1920);
    assert_eq!(DIGESTS.lines().count(), 1920);
    for (index, (request, line)) in REQUESTS.lines().zip(DIGESTS.lines()).enumerate() {
        let actual = service_corpus::run_request(request).unwrap();
        let (hash, length) = expected(line);
        assert_eq!(
            (digest(actual.as_bytes()), actual.len()),
            (hash, length),
            "request {index}: {request}"
        );
    }
}

// Fixture updates must be explicit, not regenerated from the code under test.
#[test]
fn shader_manager_fixture_payloads_are_frozen_byte_for_byte() {
    let locks = (digest(REQUESTS.as_bytes()), digest(DIGESTS.as_bytes()));
    assert_eq!(locks, (REQUESTS_LOCK, DIGESTS_LOCK));
}

const REQUESTS_LOCK: u64 = 0x4f32ab106cc48c23;
const DIGESTS_LOCK: u64 = 0x13b273fa9a431c06;
