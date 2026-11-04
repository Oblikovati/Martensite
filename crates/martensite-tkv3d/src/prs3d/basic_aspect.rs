//! `Prs3dBasicAspect`, the abstract root of the Prs3d aspects, and the shared helpers of the
//! aspect classes.
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Handle, StandardTransient, Transient};
use martensite_tkservice::graphic3d::Graphic3dAspectsSource;
use martensite_tkservice::json_dump::field_dumped;
use std::io;

/// An aspect of a presentation; every aspect class dumps itself.
pub trait Prs3dBasicAspect: Transient {
    /// `DumpJson`; `aspect.dump_json(&mut stream, -1)?`.
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()>;
}
martensite_tkernel::impl_transient_interface!(
    dyn Prs3dBasicAspect,
    "Prs3dBasicAspect",
    StandardTransient
);

/// The native `*theTarget->Aspect = *theSource->Aspect`: copies every Graphic3dAspects
/// field of `source` into `target`. The source is read before the target is locked, so the
/// same object on both sides is a no-op instead of a deadlock.
/// Example: `prs3d_copy_aspects(&*own.aspect, &*linked.aspect)`.
pub fn prs3d_copy_aspects(
    target: &dyn Graphic3dAspectsSource,
    source: &dyn Graphic3dAspectsSource,
) {
    let fields = source.aspects_base().fields.snapshot();
    *target.aspects_base().fields.write() = fields;
}

/// `DUMP_FIELD_VALUES_DUMPED` of an optional nested aspect: skipped for a null handle.
pub(crate) fn dump_aspect_field<
    T: Prs3dBasicAspect + martensite_tkernel::standard::RuntimeTyped + ?Sized,
>(
    stream: &mut DumpStream,
    depth: i32,
    field: &str,
    aspect: &Handle<T>,
) -> io::Result<()> {
    let Some(aspect) = aspect.get() else {
        return Ok(());
    };
    field_dumped(stream, depth, field, &|o, d| aspect.dump_json(o, d))
}

/// `DUMP_FIELD_VALUES_DUMPED` of an optional Graphic3d aspects handle.
pub(crate) fn dump_graphic_aspects<
    T: Graphic3dAspectsSource + martensite_tkernel::standard::RuntimeTyped + ?Sized,
>(
    stream: &mut DumpStream,
    depth: i32,
    field: &str,
    aspect: &Handle<T>,
) -> io::Result<()> {
    let Some(aspect) = aspect.get() else {
        return Ok(());
    };
    field_dumped(stream, depth, field, &|o, d| aspect.dump_json(o, d))
}
