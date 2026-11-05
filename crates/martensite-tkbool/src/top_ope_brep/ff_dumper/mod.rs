//! Explicit release/debug diagnostics avoid global streams and dangling filler pointers.
use super::{TopOpeBRepFacesFiller, TopOpeBRepLineInter, TopOpeBRepVPointInter};
use crate::top_ope_brep_ds::TopOpeBRepDSKind;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::standard::{Failure, StandardTransient};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::io::Write;
mod lines;
mod points;

/// The descriptor parent remains the standard transient base; `TopOpeBRepFFDumperBaseType::default()`.
pub type TopOpeBRepFFDumperBaseType = StandardTransient;

/// Borrows an operation for optional diagnostics; `TopOpeBRepFFDumperView::new(None)` is silent.
/// The release pointer is safely absent: reference leaves that pointer uninitialized.
pub struct TopOpeBRepFFDumperView<'filler, 'operation> {
    filler: Option<&'filler TopOpeBRepFacesFiller<'operation>>,
    cache: DiagnosticCache,
}
#[derive(Default)]
struct DiagnosticCache {
    debug: bool,
    faces: [Shape; 2],
    edges: [Vec<(Shape, i32)>; 2],
    line_index: i32,
}

impl<'filler, 'operation> TopOpeBRepFFDumperView<'filler, 'operation> {
    /// Creates the release-mode no-op object; `Self::new(Some(&filler))` retains no pointer.
    pub fn new(_filler: Option<&'filler TopOpeBRepFacesFiller<'operation>>) -> Self {
        Self {
            filler: None,
            cache: DiagnosticCache::default(),
        }
    }
    /// Enables the source debug configuration explicitly; `Self::new_debug(&filler)?`.
    pub fn new_debug(filler: &'filler TopOpeBRepFacesFiller<'operation>) -> Result<Self, Failure> {
        let mut result = Self::new(None);
        result.cache.debug = true;
        result.init(Some(filler))?;
        Ok(result)
    }
    /// Rebinds debug diagnostics and resets only for changed faces; `dumper.init(Some(&filler))?`.
    pub fn init(
        &mut self,
        filler: Option<&'filler TopOpeBRepFacesFiller<'operation>>,
    ) -> Result<(), Failure> {
        if !self.cache.debug {
            return Ok(());
        }
        self.filler = filler;
        let filler = self.require_filler()?;
        let faces = [
            filler.face(1)?.as_ref().clone(),
            filler.face(2)?.as_ref().clone(),
        ];
        if same_faces(&self.cache.faces, &faces) {
            return Ok(());
        }
        self.cache.faces = faces;
        self.cache.edges = [
            edge_indices(&self.cache.faces[0])?,
            edge_indices(&self.cache.faces[1])?,
        ];
        self.cache.line_index = 0;
        Ok(())
    }
    /// Returns the safely retained debug operation; `dumper.p_faces_filler_dummy()`.
    pub fn p_faces_filler_dummy(&self) -> Option<&TopOpeBRepFacesFiller<'operation>> {
        self.filler
    }
    /// Returns the last exploration index, or zero outside operand 1/2; `dumper.explore_index(&edge, 1)?`.
    pub fn explore_index(&self, shape: &Shape, operand: i32) -> Result<i32, Failure> {
        if !self.cache.debug || !(1..=2).contains(&operand) {
            return Ok(0);
        }
        self.cache.edges[(operand - 1) as usize].iter().find(|(edge, _)| edge.is_same(shape))
            .map(|(_, index)| *index).ok_or_else(|| Failure::with_message("Standard_NoSuchObject",
                &format!("FFDumper::ExploreIndex: operand={operand}, shape absent; expected mapped edge")))
    }
    fn require_filler(&self) -> Result<&'filler TopOpeBRepFacesFiller<'operation>, Failure> {
        self.filler.ok_or_else(|| {
            Failure::with_message(
                "Standard_NullObject",
                "FFDumper: filler=None; expected a borrowed operation in debug mode",
            )
        })
    }
}
fn edge_indices(face: &Shape) -> Result<Vec<(Shape, i32)>, Failure> {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let mut result: Vec<(Shape, i32)> = Vec::new();
    let mut index = 0;
    while explorer.more() {
        index += 1;
        let edge = explorer.current();
        if let Some((_, previous)) = result.iter_mut().find(|(seen, _)| seen.is_same(edge)) {
            *previous = index;
        } else {
            result.push((edge.clone(), index));
        }
        explorer.next()?;
    }
    Ok(result)
}
fn emit(output: &mut impl Write, text: std::fmt::Arguments<'_>) -> Result<(), Failure> {
    output.write_fmt(text).map_err(|error| {
        Failure::with_message(
            "Standard_Failure",
            &format!("FFDumper: output error={error}; expected writable diagnostic stream"),
        )
    })
}
fn real(value: f64) -> String {
    String::from_utf8_lossy(
        martensite_tkernel::tcollection::AsciiString::from_real(value).as_bytes(),
    )
    .into_owned()
}

fn same_faces(first: &[Shape; 2], second: &[Shape; 2]) -> bool {
    first.iter().zip(second).all(|(old, new)| old.is_equal(new))
}

mod owned;
pub use owned::{TopOpeBRepFFDumper, TopOpeBRepFacesFillerContext};
