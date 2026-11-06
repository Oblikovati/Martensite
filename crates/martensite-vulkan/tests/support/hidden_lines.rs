//! Hidden-line removal of a shape seen along a projection frame: the visible sharp edges,
//! outlines and smooth (G1) edges, computed exactly or on the shape's triangulation.
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkhlr::hlr_algo::Projector;
use martensite_tkhlr::hlr_brep::{Algo, HLRToShape, PolyAlgo, PolyHLRToShape};
use martensite_tkmath::gp::Axis2;

/// How hidden lines are removed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HiddenLineAlgorithm {
    /// On the exact boundary representation.
    Exact,
    /// On the shape's triangulation.
    Polygonal,
}

/// The visible lines of `shape` projected along `frame`: one compound holding the visible
/// compound (sharp edges, outlines, smooth edges) and an empty hidden compound.
/// Example: `visible_lines(&shape, frame, HiddenLineAlgorithm::Exact)?`.
pub fn visible_lines(
    shape: &Shape,
    frame: Axis2,
    algorithm: HiddenLineAlgorithm,
) -> Result<Shape, Failure> {
    let projector = Projector::with_frame(frame)?;
    let visible = match algorithm {
        HiddenLineAlgorithm::Exact => exact_visible(shape, &projector)?,
        HiddenLineAlgorithm::Polygonal => polygonal_visible(shape, &projector)?,
    };
    nested_result(&visible)
}

/// Sharp, outline, smooth and (exact only) iso compounds in the viewer's display order.
fn exact_visible(shape: &Shape, projector: &Projector) -> Result<Vec<Shape>, Failure> {
    let algo = Handle::new(Algo::new()?);
    let value = algo.get().expect("a new algorithm is never null");
    value.add(shape, 0);
    value.set_projector(projector);
    value.update()?;
    value.hide()?;
    let to_shape = HLRToShape::new(&algo);
    // Index order of the viewer's result slots: iso (1), outline (2), smooth (3), sharp (5).
    Ok(vec![
        to_shape.iso_line_v_compound()?,
        to_shape.out_line_v_compound()?,
        to_shape.rg1_line_v_compound()?,
        to_shape.v_compound()?,
    ])
}

fn polygonal_visible(shape: &Shape, projector: &Projector) -> Result<Vec<Shape>, Failure> {
    let algo = Handle::new(PolyAlgo::new()?);
    let value = algo.get().expect("a new algorithm is never null");
    value.set_projector(projector);
    value.load(shape);
    value.update()?;
    let mut to_shape = PolyHLRToShape::new();
    to_shape.update(&algo)?;
    Ok(vec![
        to_shape.out_line_v_compound()?,
        to_shape.rg1_line_v_compound()?,
        to_shape.v_compound()?,
    ])
}

/// Compound { visible compound { non-null parts }, empty hidden compound }.
fn nested_result(visible: &[Shape]) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let (mut shown, mut hidden, mut result) = (
        Compound::default(),
        Compound::default(),
        Compound::default(),
    );
    builder.make_compound(&mut shown);
    builder.make_compound(&mut hidden);
    builder.make_compound(&mut result);
    for part in visible.iter().filter(|part| !part.is_null()) {
        builder.add(&mut shown, part)?;
    }
    builder.add(&mut result, &shown.into_shape())?;
    builder.add(&mut result, &hidden.into_shape())?;
    Ok(result.into_shape())
}
