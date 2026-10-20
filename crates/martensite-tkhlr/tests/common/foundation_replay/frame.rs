//! The viewing frame of `HLRAlgo_Projector(const gp_Ax2&)`: the transformation from the global frame to the
//! projection frame, and its inverse, that the projector stores.
use super::common::{Reader, Sink, attempt, end, real};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis2, Axis3, Transform3};

/// `P N X`: `gp_Trsf::SetTransformation(gp_Ax3(gp_Ax2(P, N, X)))`, then the 12
/// coefficients of the transformation and of its inverse, or the raised exception type.
pub fn frame_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let (location, normal, x) = (r.pnt(), r.dir(), r.dir());
    attempt(tokens, |tokens| {
        let frame = Axis3::from_axis2(Axis2::with_x_direction(location, normal, x)?);
        let mut trsf = Transform3::default();
        trsf.set_transformation_to(frame);
        print_transform(tokens, &trsf)?;
        print_transform(tokens, &trsf.inverted()?)
    });
    end(tokens);
}

/// The 12 coefficients of `trsf`, row by row.
fn print_transform(tokens: &mut Sink, trsf: &Transform3) -> Result<(), Failure> {
    for row in 1..=3 {
        for column in 1..=4 {
            real(tokens, trsf.value(row, column)?);
        }
    }
    Ok(())
}
