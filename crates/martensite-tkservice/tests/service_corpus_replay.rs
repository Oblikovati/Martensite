//! Frozen service requests replay in process against their reviewed results.
#[path = "support/service_corpus/policy.rs"]
mod policy;
#[path = "support/service_corpus/mod.rs"]
mod service_corpus;
const REQUESTS: &str = concat!(
    include_str!("fixtures/service_corpus/inputs-00.txt"),
    include_str!("fixtures/service_corpus/inputs-01.txt"),
    include_str!("fixtures/service_corpus/inputs-02.txt"),
    include_str!("fixtures/service_corpus/inputs-03.txt"),
    include_str!("fixtures/service_corpus/inputs-04.txt"),
    include_str!("fixtures/service_corpus/inputs-05.txt"),
    include_str!("fixtures/service_corpus/inputs-06.txt"),
    include_str!("fixtures/service_corpus/inputs-07.txt"),
    include_str!("fixtures/service_corpus/inputs-08.txt"),
    include_str!("fixtures/service_corpus/inputs-09.txt"),
    include_str!("fixtures/service_corpus/inputs-10.txt"),
    include_str!("fixtures/service_corpus/inputs-11.txt"),
    include_str!("fixtures/service_corpus/inputs-12.txt"),
);
const EXPECTED: &str = concat!(
    include_str!("fixtures/service_corpus/native-00.txt"),
    include_str!("fixtures/service_corpus/native-01.txt"),
    include_str!("fixtures/service_corpus/native-02.txt"),
    include_str!("fixtures/service_corpus/native-03.txt"),
    include_str!("fixtures/service_corpus/native-04.txt"),
    include_str!("fixtures/service_corpus/native-05.txt"),
    include_str!("fixtures/service_corpus/native-06.txt"),
    include_str!("fixtures/service_corpus/native-07.txt"),
    include_str!("fixtures/service_corpus/native-08.txt"),
    include_str!("fixtures/service_corpus/native-09.txt"),
    include_str!("fixtures/service_corpus/native-10.txt"),
    include_str!("fixtures/service_corpus/native-11.txt"),
    include_str!("fixtures/service_corpus/native-12.txt"),
);

fn replay_family(operation: &str, count: usize) {
    assert_eq!(REQUESTS.lines().count(), 3142);
    assert_eq!(EXPECTED.lines().count(), 3142);
    let mut observed = 0;
    for (index, (request, expected)) in REQUESTS
        .lines()
        .zip(EXPECTED.split_inclusive('\n'))
        .enumerate()
    {
        if request.split_whitespace().next() != Some(operation) {
            continue;
        }
        assert_fixture_result(index, request, expected);
        observed += 1;
    }
    assert_eq!(observed, count, "family {operation}");
}

fn assert_fixture_result(index: usize, request: &str, expected: &str) {
    let actual = service_corpus::run_request(request).unwrap();
    assert!(
        policy::equivalent(expected, &actual),
        "request {index}: {request}; {}",
        policy::difference(expected, &actual)
    );
}

macro_rules! service_families {
    ($($operation:ident: $count:literal),* $(,)?) => { $(
        #[test]
        fn $operation() { replay_family(stringify!($operation), $count); }
    )* };
}
service_families! {
    sv_rgrid:136, sv_cgrid:136, sv_hid:131, sv_pixel:320, sv_rawpixel:100,
    sv_half:77, sv_pbr_lut:7, sv_pbr_bsdf:41, sv_pbr_factor:52, sv_fresnel:32,
    sv_material:26, sv_material_rank:28, sv_material_edit:32, sv_shader_decl:97,
    sv_cubemap:320, sv_segment:28, sv_marker:172, sv_aspects_dump:120, sv_array:165,
    sv_camera:61, sv_camera_fit:40, sv_camera_interp:30, sv_clip:81, sv_clip_capping:40,
    sv_light:84, sv_light_set:41, sv_tpers:120, sv_tutils:60, sv_params:54,
    sv_fontmgr:1, sv_ftfont:40, sv_textfmt:40, sv_culling:60, sv_stats:120,
    sv_scene:180, sv_media:40, sv_mtex:6,
}
#[cfg(target_os = "linux")]
#[test]
fn sv_window() {
    replay_family("sv_window", 24);
}

// Lightweight byte-lock of the reviewed fixtures: updates must be explicit, rather than accepting regenerated outputs from the code under test.
#[test]
fn historical_fixture_payloads_are_frozen_byte_for_byte() {
    assert_eq!(fixture_digest(REQUESTS.as_bytes()), 0xd6165a8f02a44142);
    assert_eq!(fixture_digest(EXPECTED.as_bytes()), 0x2ecb6b7685a8f273);
}

fn fixture_digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3)
    })
}
