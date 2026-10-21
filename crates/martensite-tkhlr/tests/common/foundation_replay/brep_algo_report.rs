//! Ordered shape bounds and extraction reports reuse the canonical compound emitter.
use super::common::{Sink, attempt, flag, int};
use super::{brep_record_print::compound, hlr_algo::print_minmax};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkhlr::hlr_brep::{Algo, HLRToShape, TypeOfResultingEdge};
const RESULT_TYPES: [TypeOfResultingEdge; 5] = [
    TypeOfResultingEdge::IsoLine,
    TypeOfResultingEdge::OutLine,
    TypeOfResultingEdge::Rg1Line,
    TypeOfResultingEdge::RgNLine,
    TypeOfResultingEdge::Sharp,
];

pub(super) fn bounds(tokens: &mut Sink, algorithm: &Algo, shapes: &[Shape]) -> Result<(), Failure> {
    int(tokens, algorithm.nb_shapes());
    let handle = algorithm.data_structure();
    flag(tokens, handle.is_null());
    if let Some(records) = handle.get() {
        int(tokens, records.nb_edges());
        int(tokens, records.nb_faces());
    }
    for index in 1..=algorithm.nb_shapes() {
        shape_bounds(tokens, algorithm, index, &shapes[index as usize - 1])?;
    }
    Ok(())
}
fn shape_bounds(
    tokens: &mut Sink,
    algorithm: &Algo,
    index: i32,
    source: &Shape,
) -> Result<(), Failure> {
    let bounds = algorithm.shape_bounds(index)?;
    let (v1, v2, e1, e2, f1, f2) = bounds.bounds();
    for value in [v1, v2, e1, e2, f1, f2] {
        int(tokens, value);
    }
    print_minmax(tokens, bounds.min_max());
    int(tokens, algorithm.index(source));
    Ok(())
}

pub(super) fn results(tokens: &mut Sink, handle: &Handle<Algo>, first: &Shape) {
    let extraction = HLRToShape::new(handle);
    for kind in RESULT_TYPES {
        for visible in [true, false] {
            attempt(tokens, |tokens| {
                compound(tokens, &extraction.compound_of_edges(kind, visible, false)?)
            });
        }
    }
    attempt(tokens, |tokens| {
        compound(tokens, &extraction.out_line_v_compound_3d()?)
    });
    attempt(tokens, |tokens| {
        compound(tokens, &extraction.v_compound_of(first)?)
    });
    attempt(tokens, |tokens| {
        compound(tokens, &extraction.h_compound_of(first)?)
    });
}
