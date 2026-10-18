//! The GeomTools probe cases, the Rust twin of the implementation
//! (texts and single-record reads) and the implementation (sets.rs).
pub mod console;
pub mod kinds;
pub mod sets;

use super::common::{Reader, end, int, real};
use console::{Context, attempt_captured, captured};
use kinds::{Curves, Curves2d, GtKind, Surfaces, summary};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::storage::IoStream;
use martensite_tkgeombase::geom_tools;
use std::io::{Read, Seek, SeekFrom};

/// `gt_text`: one `t<hex>` token.
pub fn text(ctx: &Context, bytes: &[u8]) {
    let hex: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    ctx.output.write(format_args!("t{hex} "));
}

/// The bytes written to an in-memory stream.
pub fn written_bytes(stream: IoStream) -> Vec<u8> {
    let mut device = stream.into_device().expect("an open stream");
    device.seek(SeekFrom::Start(0)).expect("in-memory seek");
    let mut bytes = Vec::new();
    device.read_to_end(&mut bytes).expect("in-memory read");
    bytes
}

/// `gt_bytes`: a text given as byte codes.
pub fn bytes(r: &mut Reader<'_>) -> Vec<u8> {
    let n = r.integer();
    (0..n).map(|_| r.integer() as u8).collect()
}

/// `gt_state`: the state after a read, then the unread rest.
pub fn state(ctx: &Context, stream: &mut IoStream) {
    int(ctx.output, i32::from(stream.rdstate()));
    stream.clear(0);
    let mut rest = Vec::new();
    let mut byte = 0;
    loop {
        stream.get(&mut byte);
        if stream.fail() {
            break;
        }
        rest.push(byte);
    }
    text(ctx, &rest);
}

/// `gt_written`: the text `print` writes at `precision`, and its console output.
pub fn written(
    ctx: &Context,
    precision: usize,
    print: impl FnOnce(&mut IoStream) -> Result<(), Failure>,
) -> Vec<u8> {
    let mut stream = IoStream::from_bytes(Vec::new());
    stream.set_precision(precision);
    let console = captured(ctx, || attempt_captured(ctx, || print(&mut stream)));
    let bytes = written_bytes(stream);
    text(ctx, &bytes);
    text(ctx, &console);
    bytes
}

/// `gt_texts`: Dump and Write at 6 and 17, then the compact text read back.
pub fn texts<K: GtKind>(ctx: &Context, g: &Handle<K::Geometry>) {
    written(ctx, 6, |s| K::dump(g, s));
    written(ctx, 6, |s| K::write(g, s));
    written(ctx, 17, |s| K::dump(g, s));
    let compact = written(ctx, 17, |s| K::write(g, s));
    if compact.is_empty() {
        return;
    }
    let mut stream = IoStream::from_bytes(compact);
    let mut back = Handle::default();
    text(ctx, &captured(ctx, || back = K::read(&mut stream)));
    summary::<K>(ctx, &back);
    state(ctx, &mut stream);
}

fn geometry_case<K: GtKind>(ctx: &Context, values: &[f64]) {
    let mut r = Reader::new(values);
    match K::build(&mut r) {
        Ok(g) if g.is_some() => texts::<K>(ctx, &g),
        Ok(_) => {}
        Err(error) => print!("{} ", error.exception_type()),
    }
    end(ctx.output);
}

pub fn curve_case(ctx: &Context, values: &[f64]) {
    geometry_case::<Curves>(ctx, values);
}
pub fn curve2d_case(ctx: &Context, values: &[f64]) {
    geometry_case::<Curves2d>(ctx, values);
}
pub fn surface_case(ctx: &Context, values: &[f64]) {
    geometry_case::<Surfaces>(ctx, values);
}

fn read_one<K: GtKind>(ctx: &Context, stream: &mut IoStream) -> Vec<u8> {
    let mut g = Handle::default();
    let console = captured(ctx, || g = K::read(stream));
    summary::<K>(ctx, &g);
    console
}

/// `gt_read kind bytes`.
pub fn read_case(ctx: &Context, values: &[f64]) {
    let mut r = Reader::new(values);
    let kind = r.integer();
    let mut stream = IoStream::from_bytes(bytes(&mut r));
    let console = match kind {
        0 => read_one::<Curves>(ctx, &mut stream),
        1 => read_one::<Curves2d>(ctx, &mut stream),
        _ => read_one::<Surfaces>(ctx, &mut stream),
    };
    text(ctx, &console);
    state(ctx, &mut stream);
    end(ctx.output);
}

/// `gt_get_real count bytes`.
pub fn get_real_case(ctx: &Context, values: &[f64]) {
    let mut r = Reader::new(values);
    let n = r.integer();
    let mut stream = IoStream::from_bytes(bytes(&mut r));
    for _ in 0..n {
        real(ctx.output, geom_tools::get_real(&mut stream));
        int(ctx.output, i32::from(stream.rdstate()));
    }
    state(ctx, &mut stream);
    end(ctx.output);
}

type Case = for<'a> fn(&Context<'a>, &[f64]);
fn case_for(operation: &str) -> Option<Case> {
    Some(match operation {
        "gt_curve" => curve_case,
        "gt_curve2d" => curve2d_case,
        "gt_surface" => surface_case,
        "gt_read" => read_case,
        "gt_get_real" => get_real_case,
        "gt_set" => sets::set_case_dispatch,
        "gt_set_null" => sets::set_null_case,
        "gt_set_read" => sets::set_read_case,
        "gt_progress" => sets::progress_case,
        "gt_debug_dump" => sets::debug_dump_case,
        "gt_handler" => sets::handler_case,
        _ => return None,
    })
}

/// One original request; this process serializes all calls of the global handler hook.
pub fn run(output: &super::print::Output, operation: &str, values: &[f64]) -> bool {
    let Some(case) = case_for(operation) else {
        return false;
    };
    case(&Context::new(output), values);
    true
}

#[cfg(test)]
pub fn supports_operation(operation: &str) -> bool {
    case_for(operation).is_some()
}
