//! The placement of `HelixGeom_BuilderHelix::Perform`: the displacement from the standard frame to
//! the helix position that moves every coil into place.
use super::common::{Reader, Sink, attempt, end, real};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Axis2, Axis3, Transform3};

/// `P N X`: `gp_Trsf::SetDisplacement(gp_Ax3(), gp_Ax3(gp_Ax2(P, N, X)))`, then its 12
/// coefficients (rows 1..3, columns 1..4) or the raised exception type.
pub fn placement_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let (location, normal, x) = (r.pnt(), r.dir(), r.dir());
    attempt(out, |out| {
        let target = Axis3::from_axis2(Axis2::with_x_direction(location, normal, x)?);
        let mut trsf = Transform3::default();
        trsf.set_displacement(Axis3::default(), target);
        print_transform(out, &trsf)
    });
    end(out);
}

/// The 12 coefficients of `trsf`, row by row.
fn print_transform(out: &mut Sink, trsf: &Transform3) -> Result<(), Failure> {
    for row in 1..=3 {
        for column in 1..=4 {
            real(out, trsf.value(row, column)?);
        }
    }
    Ok(())
}
