use martensite_tkbool::top_ope_brep_build::{
    TopOpeBRepBuildPlainShapeSet, TopOpeBRepBuildShapeSet,
};
use martensite_tkernel::{standard::Failure, tcollection::AsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::io::{self, Write};

fn name_set() -> Result<TopOpeBRepBuildPlainShapeSet, Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    set.set_deb_number(-7);
    Ok(set)
}

fn name_spans() -> [Vec<u8>; 7] {
    [
        Vec::new(),
        b"plain".to_vec(),
        "é".as_bytes().to_vec(),
        vec![0x80, 0xff],
        b"A\0B".to_vec(),
        (0..=255).collect(),
        (1..=255).collect(),
    ]
}

fn expected_stream_name(name: &[u8]) -> Vec<u8> {
    let end = name
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(name.len());
    let mut output = b"\x80P(".to_vec();
    output.extend_from_slice(&name[..end]);
    output.extend_from_slice(b",-7)");
    output
}

#[test]
fn exact_name_interface_retains_all_byte_values_and_streams_c_string_prefixes()
-> Result<(), Failure> {
    let mut set = name_set()?;
    let prefix = AsciiString::from_raw(b"\x80P\0Q");
    for span in name_spans() {
        let name = AsciiString::from_raw(&span);
        set.set_deb_name_ascii(&name);
        assert_eq!(set.deb_name_ascii().as_bytes(), span);
        assert_eq!(set.deb_name_ascii().len(), span.len());
        let mut output = Vec::new();
        set.dump_name_ascii(&mut output, &prefix)
            .expect("Vec writes cannot fail");
        assert_eq!(output, expected_stream_name(&span));
    }
    Ok(())
}

#[test]
fn byte_names_copy_inputs_and_expose_the_actual_mutable_string() -> Result<(), Failure> {
    let mut set = name_set()?;
    let mut input = AsciiString::from_raw(&[0x80, 0xff]);
    set.set_deb_name_ascii(&input);
    input.set_value_char(1, b'X')?;
    assert_eq!(set.deb_name_ascii().as_bytes(), [0x80, 0xff]);
    let shared = std::ptr::from_ref(set.deb_name_ascii());
    let mutable = std::ptr::from_mut(set.change_deb_name_ascii());
    assert_eq!(shared, mutable.cast_const());
    set.change_deb_name_ascii()
        .append(&AsciiString::from_raw(&[0, 0xfe]));
    assert_eq!(set.deb_name_ascii().as_bytes(), [0x80, 0xff, 0, 0xfe]);
    Ok(())
}

#[test]
fn checked_text_view_returns_a_conversion_error_without_losing_bytes() -> Result<(), Failure> {
    let mut set = name_set()?;
    set.set_deb_name("é\0text");
    assert_eq!(set.try_deb_name(), Ok("é\0text"));
    set.set_deb_name_ascii(&AsciiString::from_raw(&[0x80, 0xff]));
    let error = set.try_deb_name().expect_err("invalid UTF-8 remains bytes");
    assert_eq!(error.valid_up_to(), 0);
    assert_eq!(error.error_len(), Some(1));
    assert_eq!(set.deb_name_ascii().as_bytes(), [0x80, 0xff]);
    Ok(())
}

#[test]
#[should_panic(expected = "expected valid UTF-8")]
fn legacy_text_view_clearly_rejects_a_new_unrepresentable_byte_state() {
    let mut set = name_set().expect("valid set");
    set.set_deb_name_ascii(&AsciiString::from_raw(&[0xff]));
    let _ = set.deb_name();
}

struct RefusingNameWriter;
impl Write for RefusingNameWriter {
    fn write(&mut self, _bytes: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "name fixture refuses bytes",
        ))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn exact_dump_propagates_the_injected_stream_failure_without_mutating_name() -> Result<(), Failure>
{
    let mut set = name_set()?;
    set.set_deb_name_ascii(&AsciiString::from_raw(&[0xff]));
    let failure = set
        .dump_name_ascii(&mut RefusingNameWriter, &AsciiString::new("P"))
        .expect_err("fixture refuses bytes");
    assert_eq!(failure.kind(), io::ErrorKind::BrokenPipe);
    assert_eq!(set.deb_name_ascii().as_bytes(), [0xff]);
    Ok(())
}

#[test]
fn old_dump_keeps_full_spans_while_exact_dump_uses_the_native_stream_contract()
-> Result<(), Failure> {
    let mut set = name_set()?;
    set.set_deb_name_ascii(&AsciiString::from_raw(b"A\0B"));
    let mut convenience = Vec::new();
    set.dump_name(&mut convenience, "P\0Q").expect("Vec write");
    assert_eq!(convenience, b"P\0Q(A\0B,-7)");
    let mut exact = Vec::new();
    set.dump_name_ascii(&mut exact, &AsciiString::from_raw(b"P\0Q"))
        .expect("Vec write");
    assert_eq!(exact, b"P(A,-7)");
    Ok(())
}

#[derive(Clone, Copy)]
enum IntegerBase {
    Decimal,
    Hexadecimal,
}

struct ConfiguredDumpStream {
    bytes: Vec<u8>,
    integer_base: IntegerBase,
    next_field_width: Option<usize>,
}

impl ConfiguredDumpStream {
    fn decimal() -> Self {
        Self {
            bytes: Vec::new(),
            integer_base: IntegerBase::Decimal,
            next_field_width: None,
        }
    }

    fn hexadecimal() -> Self {
        Self {
            bytes: Vec::new(),
            integer_base: IntegerBase::Hexadecimal,
            next_field_width: None,
        }
    }

    fn with_width(mut self, width: usize) -> Self {
        self.next_field_width = Some(width);
        self
    }

    fn insert_field(&mut self, bytes: &[u8]) {
        let width = self.next_field_width.take().unwrap_or(0);
        self.bytes
            .extend(std::iter::repeat_n(b' ', width.saturating_sub(bytes.len())));
        self.bytes.extend_from_slice(bytes);
    }
}

impl martensite_tkbool::top_ope_brep_build::ShapeSetDumpStream for ConfiguredDumpStream {
    fn insert_ascii(&mut self, text: &AsciiString) -> io::Result<()> {
        let end = text
            .as_bytes()
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(text.len());
        self.insert_field(&text.as_bytes()[..end]);
        Ok(())
    }

    fn insert_literal(&mut self, text: &[u8]) -> io::Result<()> {
        self.bytes.extend_from_slice(text);
        Ok(())
    }

    fn insert_i32(&mut self, value: i32) -> io::Result<()> {
        let text = match self.integer_base {
            IntegerBase::Decimal => value.to_string(),
            IntegerBase::Hexadecimal => format!("{value:x}"),
        };
        self.insert_field(text.as_bytes());
        Ok(())
    }
}

#[test]
fn formatting_aware_dump_replays_configured_ostream_hex_and_width_insertions() -> Result<(), Failure>
{
    let mut set = name_set()?;
    set.set_deb_name_ascii(&AsciiString::new("N"));
    set.set_deb_number(31);

    let mut hexadecimal = ConfiguredDumpStream::hexadecimal();
    set.dump_name_stream(&mut hexadecimal, &AsciiString::new("P"))
        .expect("configured hexadecimal stream accepts insertions");
    assert_eq!(hexadecimal.bytes, b"P(N,1f)");

    let mut padded = ConfiguredDumpStream::decimal().with_width(4);
    set.dump_name_stream(&mut padded, &AsciiString::new("P"))
        .expect("configured padded stream accepts insertions");
    assert_eq!(padded.bytes, b"   P(N,31)");
    Ok(())
}
