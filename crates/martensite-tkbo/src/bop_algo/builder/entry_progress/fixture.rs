//! Whole receiver fixtures preserve construction-owned argument and tool handles.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape;
use crate::bop_algo::{BOPAlgoSplitter, GeneralFuseAlgorithm, Glue};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
/// The actual receiver remains selected through execution; `let case = whole_case(23, false)`.
pub(super) struct WholeCase {
    pub receiver: WholeReceiver,
    pub sources: Vec<Shape>,
}
pub(super) enum WholeReceiver {
    General(BOPAlgoBuilder),
    Splitter(BOPAlgoSplitter),
}
impl WholeReceiver {
    /// Borrow the common state without selecting a substitute program; `case.receiver.builder()`.
    pub(super) fn builder(&self) -> &BOPAlgoBuilder {
        match self {
            Self::General(value) => value,
            Self::Splitter(value) => GeneralFuseAlgorithm::general_fuse(value),
        }
    }
    fn builder_mut(&mut self) -> &mut BOPAlgoBuilder {
        match self {
            Self::General(value) => value,
            Self::Splitter(value) => BuilderHooks::general_fuse(value),
        }
    }
    /// Call the real receiver's public ranged entry; `case.receiver.perform(range)?`.
    pub(super) fn perform(
        &mut self,
        range: &ProgressRange<'_>,
    ) -> Result<ProgressCompletion, Failure> {
        match self {
            Self::General(value) => value.perform_with_progress(range),
            Self::Splitter(value) => value.perform_with_progress(range),
        }
    }
}
/// Retain an explicit tools-only profile rather than converting it into arguments; `whole_case(25, false)`.
pub(super) fn whole_case(kind: usize, parallel: bool) -> WholeCase {
    let sources = whole_sources(kind);
    let mut receiver = if kind >= 20 {
        WholeReceiver::Splitter(BOPAlgoSplitter::new())
    } else {
        WholeReceiver::General(BOPAlgoBuilder::new())
    };
    configure_arguments(&mut receiver, &sources, kind);
    let builder = receiver.builder_mut();
    builder.options_mut().set_run_parallel(parallel);
    builder.set_to_fill_history(kind != 7 && kind != 26);
    builder.set_non_destructive(kind == 8);
    if kind == 9 {
        builder.options_mut().set_fuzzy_value(1e-5);
        builder.options_mut().set_use_obb(true);
    }
    if kind == 10 {
        builder.set_glue(Glue::Full);
    }
    WholeCase { receiver, sources }
}
fn configure_arguments(receiver: &mut WholeReceiver, sources: &[Shape], kind: usize) {
    match receiver {
        WholeReceiver::General(builder) => {
            for source in sources {
                builder.add_argument(source);
            }
        }
        WholeReceiver::Splitter(splitter) => {
            if sources.is_empty() {
                return;
            }
            if kind != 25 {
                splitter.add_argument(&sources[0]);
            }
            splitter.add_tool(&sources[1]);
        }
    }
}
fn whole_sources(kind: usize) -> Vec<Shape> {
    match kind {
        0 | 20 => Vec::new(),
        1 | 2 | 7 | 21 | 25 | 26 => vec![point(0.), point(if kind == 2 { 0. } else { 10. })],
        3 | 9 | 22 => crossing_sources(),
        6 | 24 => [
            include_bytes!("box0.brep").as_slice(),
            include_bytes!("box1.brep").as_slice(),
        ]
        .into_iter()
        .map(|bytes| read_owned_progress_shape(bytes, ShapeEnum::Solid))
        .collect(),
        _ => planar_sources(kind),
    }
}
fn point(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn crossing_sources() -> Vec<Shape> {
    [
        [Point3::new(-1., 0., 0.), Point3::new(1., 0., 0.)],
        [Point3::new(0., -1., 0.), Point3::new(0., 1., 0.)],
    ]
    .map(|ends| {
        BRepBuilderAPIMakeEdge::from_points(ends[0], ends[1])
            .unwrap()
            .edge()
            .unwrap()
            .into_shape()
    })
    .to_vec()
}
fn planar_sources(kind: usize) -> Vec<Shape> {
    let bytes = [
        include_bytes!("../../pave_filler/ranged_intersection/plane0.brep").as_slice(),
        include_bytes!("../../pave_filler/ranged_intersection/plane1.brep").as_slice(),
        include_bytes!("../../pave_filler/ranged_intersection/plane2.brep").as_slice(),
    ];
    (0..if kind == 5 { 3 } else { 2 })
        .map(|index| {
            read_owned_progress_shape(bytes[if kind == 10 { 0 } else { index }], ShapeEnum::Face)
        })
        .collect()
}
