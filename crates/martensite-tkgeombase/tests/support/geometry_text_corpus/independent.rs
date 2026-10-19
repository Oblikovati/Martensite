//! Closed-form geometry and byte expectations, independent of the retained corpus.
use crate::construction_corpus::geom_tools::console::{Context, captured};
use crate::construction_corpus::geom_tools::written_bytes;
use crate::construction_corpus::print::Output;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::geom::{GeomCircle, GeomCurve, GeomLine};
use martensite_tkgeombase::geom_tools;
use martensite_tkmath::gp::{Axis2, Direction3, Point3};

pub fn line_roundtrip_and_precision() {
    let curve: Handle<dyn GeomCurve> = Handle::new(GeomLine::from_point(
        Point3::new(1.0, 2.0, 3.0),
        Direction3::new(1.0, 0.0, 0.0).unwrap(),
    ))
    .into_cast();
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(6);
    geom_tools::write_curve(&curve, &mut stream).unwrap();
    assert_eq!(stream.precision(), 6);
    assert_eq!(written_bytes(stream), b"1 1 2 3 1 0 0 \n");
    let mut input = IoStream::from_bytes(b"1 1 2 3 1 0 0 \n".to_vec());
    let back = geom_tools::read_curve(&mut input);
    assert_eq!(back.value(4.0).unwrap(), Point3::new(5.0, 2.0, 3.0));
    assert!(!input.fail());
}

pub fn circle_roundtrip_and_remainder() {
    let curve: Handle<dyn GeomCurve> =
        Handle::new(GeomCircle::new(Axis2::default(), 2.0).unwrap()).into_cast();
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(17);
    geom_tools::write_curve(&curve, &mut stream).unwrap();
    let mut bytes = written_bytes(stream);
    bytes.extend_from_slice(b"tail");
    let mut input = IoStream::from_bytes(bytes);
    let back = geom_tools::read_curve(&mut input);
    assert_eq!(back.value(0.0).unwrap(), Point3::new(2.0, 0.0, 0.0));
    check_circle_quarter(&back);
    let mut remaining = Vec::new();
    let mut byte = 0;
    while !input.get(&mut byte).fail() {
        remaining.push(byte);
    }
    assert_eq!(remaining, b"\ntail");
}

pub fn failed_read_and_console_isolation() {
    let output = Output::default();
    let ctx = Context::new(&output);
    let mut input = IoStream::from_bytes(b"99 tail".to_vec());
    let console = captured(&ctx, || {
        assert!(geom_tools::read_curve(&mut input).is_null())
    });
    assert!(console.is_empty());
    assert!(!input.fail());
    let mut truncated = IoStream::from_bytes(b"1 0 0".to_vec());
    assert!(geom_tools::read_curve(&mut truncated).is_null());
    // GetReal stops at existing EOF, so construction refusal does not add FAIL.
    assert_eq!(truncated.rdstate(), martensite_tkernel::storage::state::EOF);
    assert!(captured(&ctx, || {}).is_empty());
}

pub fn handler_scope_restores_during_unwind() {
    let original = geom_tools::get_undefined_type_handler();
    let result = std::panic::catch_unwind(|| {
        let previous = geom_tools::get_undefined_type_handler();
        let output = Output::default();
        let ctx = Context::new(&output);
        assert!(geom_tools::get_undefined_type_handler() != previous);
        ctx.console().write(b"request-only");
        panic!("independent handler-scope unwind");
    });
    assert!(result.is_err());
    assert!(geom_tools::get_undefined_type_handler() == original);
    let output = Output::default();
    let ctx = Context::new(&output);
    assert!(captured(&ctx, || {}).is_empty());
}

fn check_circle_quarter(back: &Handle<dyn GeomCurve>) {
    let point = back.value(std::f64::consts::FRAC_PI_2).unwrap();
    assert!((point.x()).abs() < 1e-14);
    assert_eq!(point.y(), 2.0);
    assert_eq!(point.z(), 0.0);
}
