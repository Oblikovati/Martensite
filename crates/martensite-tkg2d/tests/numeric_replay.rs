//! Complete frozen numeric behavior corpus, executed in-process through Rust geometry.
#[path = "numeric_replay/closed_forms.rs"]
mod closed_forms;
#[path = "numeric_replay/support/mod.rs"]
mod replay_support;

const CORPUS: &[(&str, &str)] = &[
    (
        include_str!("numeric_replay/fixtures/inputs-0.txt"),
        include_str!("numeric_replay/fixtures/expected-0.txt"),
    ),
    (
        include_str!("numeric_replay/fixtures/inputs-1.txt"),
        include_str!("numeric_replay/fixtures/expected-1.txt"),
    ),
    (
        include_str!("numeric_replay/fixtures/inputs-2.txt"),
        include_str!("numeric_replay/fixtures/expected-2.txt"),
    ),
    (
        include_str!("numeric_replay/fixtures/inputs-3.txt"),
        include_str!("numeric_replay/fixtures/expected-3.txt"),
    ),
];

fn paired_requests() -> impl Iterator<Item = (usize, &'static str, &'static str)> {
    CORPUS
        .iter()
        .flat_map(|(requests, expected)| requests.lines().zip(expected.lines()))
        .enumerate()
        .map(|(index, (request, expected))| (index + 1, request, expected))
}

fn replay_family(operation: &str, expected_count: usize) {
    let mut count = 0;
    for (ordinal, request, expected) in paired_requests() {
        let mut fields = request.split_whitespace();
        if fields.next() != Some(operation) {
            continue;
        }
        let values = fields
            .map(|value| value.parse::<f64>().expect("numeric corpus field"))
            .collect::<Vec<_>>();
        assert_replayed(ordinal, request, expected, operation, &values);
        count += 1;
    }
    assert_eq!(count, expected_count, "complete {operation} request family");
}

fn assert_replayed(ordinal: usize, request: &str, expected: &str, operation: &str, values: &[f64]) {
    let actual = replay_support::dispatch::execute(operation, values);
    assert_eq!(actual.lines().count(), 1, "request {ordinal}: {request}");
    assert_eq!(
        canonical_nan_line(&actual),
        canonical_nan_line(&format!("{expected}\n")),
        "request {ordinal}: {request}"
    );
}

fn canonical_nan_line(line: &str) -> String {
    let mut canonical = String::with_capacity(line.len());
    for chunk in line.split_inclusive(|character: char| !is_word(character)) {
        let (last, character) = chunk.char_indices().last().expect("nonempty split chunk");
        let end = if is_word(character) {
            chunk.len()
        } else {
            last
        };
        canonical.push_str(canonical_nan_word(&chunk[..end]));
        canonical.push_str(&chunk[end..]);
    }
    canonical
}

fn is_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn canonical_nan_word(word: &str) -> &str {
    if !(16..=20).contains(&word.len()) || !word.bytes().all(|byte| byte.is_ascii_digit()) {
        return word;
    }
    match word.parse::<u64>() {
        Ok(bits) if f64::from_bits(bits).is_nan() => "NaN",
        _ => word,
    }
}

macro_rules! replay_families {
    ($($name:ident: $count:expr),+ $(,)?) => {$ (
        #[test]
        fn $name() { replay_family(stringify!($name), $count); }
    )+};
}
replay_families! {
    g2_point:40, g2_direction:40, g2_vector:41, g2_axis:40, g2_trsf:40,
    g2_curve:225, g2_transform:225, g2_conic:32, g2_trimmed:48, g2_offset:24,
    g2_offset_utils:64, g2_bezier:49, g2_bspline:104, g2_evalrep:27,
    a2_adaptor:103, a2_load:5, a2_offset_load:5, a2_evalrep:9,
    g2_eval:113, g2_hash:8, g2_hash_frame:43,
    g2_grid:402, g2_grid_adaptor:88, g2_grid_evalrep:9, g2_grid_null:3,
}

#[test]
fn every_request_and_dispatch_family_is_accounted() {
    let mut total = 0;
    for &(requests, expected) in CORPUS {
        assert_eq!(requests.lines().count(), expected.lines().count());
        for request in requests.lines() {
            let family = request.split_whitespace().next().unwrap();
            assert!(
                replay_support::dispatch::OPERATIONS
                    .iter()
                    .any(|(name, _)| *name == family),
                "unknown family {family}"
            );
            total += 1;
        }
    }
    assert_eq!(total, 1787);
    assert_eq!(replay_support::dispatch::OPERATIONS.len(), 25);
}

#[test]
fn comparator_preserves_all_finite_infinite_and_signed_zero_bits() {
    for bits in [
        0,
        1_u64 << 63,
        1_f64.to_bits(),
        1.1_f64.to_bits(),
        f64::INFINITY.to_bits(),
        f64::NEG_INFINITY.to_bits(),
    ] {
        let token = bits.to_string();
        assert_eq!(canonical_nan_line(&token), token);
    }
    assert_ne!(
        canonical_nan_line("0"),
        canonical_nan_line("9223372036854775808")
    );
    assert_ne!(
        canonical_nan_line("4607182418800017408"),
        canonical_nan_line("4607182418800017409")
    );
}

#[test]
fn comparator_classifies_nan_only_at_complete_numeric_word_boundaries() {
    for bits in [0x7ff8000000000001_u64, 0xfff8000000000002_u64] {
        let token = bits.to_string();
        assert_eq!(
            canonical_nan_line(&format!("{{\"bits\":{token}}} \n")),
            "{\"bits\":NaN} \n"
        );
        assert_eq!(
            canonical_nan_line(&format!("h{token}")),
            format!("h{token}")
        );
        assert_eq!(
            canonical_nan_line(&format!("_{token}")),
            format!("_{token}")
        );
    }
    assert_eq!(
        canonical_nan_line("18446744073709551616"),
        "18446744073709551616"
    );
}

#[test]
fn request_context_buffers_and_parameters_are_isolated() {
    let first = replay_support::output::Output::captured();
    let second = replay_support::output::Output::captured();
    first.emit(format_args!("left "));
    first.set_parameters(vec![1., 2.]);
    assert_eq!(first.finish(), "left ");
    assert!(first.finish().is_empty());
    assert!(second.finish().is_empty());
    assert!(second.parameters().is_empty());
    assert_eq!(first.parameters(), vec![1., 2.]);
}

#[test]
fn comparator_retains_text_json_layout_and_spacing() {
    assert_ne!(
        canonical_nan_line("Standard_ConstructionError "),
        canonical_nan_line("Standard_RangeError ")
    );
    assert_ne!(
        canonical_nan_line("{\"x\":1,\"y\":2} "),
        canonical_nan_line("{\"y\":2,\"x\":1} ")
    );
    assert_ne!(canonical_nan_line("0 "), canonical_nan_line("0"));
    assert_eq!(canonical_nan_line(""), "");
}
