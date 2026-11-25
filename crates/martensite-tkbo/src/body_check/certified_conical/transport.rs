// SPDX-License-Identifier: GPL-2.0-only
//! Factor out one common affine map without altering any retained coefficients.
use super::*;
use martensite_tkbrep::topods;
use martensite_tkmath::toploc::Location;

pub(super) fn strip_member(
    member: &ConicalNativeMember,
    frame: &CertifiedRotatedFrameTransport,
) -> Result<ConicalNativeMember, ConicalBindingError> {
    let datum = member.shape.location();
    check_datum(datum, frame).map_err(|cause| member_error(member, cause))?;
    let shape = strip(&member.shape, datum).map_err(|cause| member_error(member, cause))?;
    let sides = member.sides.iter().map(|(id, face)| {
        strip_face(face, datum)
            .map(|face| (*id, face))
            .map_err(|cause| binding::error(Some(member.id), Some(*id), None, cause))
    });
    let caps = member
        .caps
        .iter()
        .map(|cap| strip_cap(member.id, cap, datum));
    Ok(ConicalNativeMember {
        id: member.id,
        shape,
        sides: sides.collect::<Result<_, _>>()?,
        caps: caps.collect::<Result<_, _>>()?,
    })
}

fn check_datum(datum: &Location, frame: &CertifiedRotatedFrameTransport) -> Result<(), Failure> {
    if !datum.is_identity() && (datum.first_power()? != 1 || !datum.next_location().is_identity()) {
        return Err(native::rejected("expected one common datum at power one"));
    }
    let transform = datum.transformation();
    let actual = transform
        .matrix()
        .coefficients()
        .map(|row| row.map(f64::to_bits));
    let expected = frame.input().stored.map(|row| row.map(f64::to_bits));
    if actual != expected
        || transform.scale_factor() != 1.
        || !transform
            .translation_part()
            .coordinates()
            .iter()
            .all(|value| value.is_finite())
    {
        return Err(native::rejected(
            "placed datum does not match the certified positive matrix",
        ));
    }
    Ok(())
}

fn strip(shape: &Shape, datum: &Location) -> Result<Shape, Failure> {
    if !shape.location().is_equal(datum) {
        return Err(native::rejected(
            "placed member binding does not share its root datum",
        ));
    }
    shape.located(&Location::default(), false)
}

fn strip_face(face: &Face, datum: &Location) -> Result<Face, Failure> {
    topods::face(&strip(face, datum)?)
}

fn strip_cap(
    member: ConicalActualMemberId,
    cap: &ConicalNativeCap,
    datum: &Location,
) -> Result<ConicalNativeCap, ConicalBindingError> {
    let face = strip_face(&cap.face, datum)
        .map_err(|cause| binding::error(Some(member), None, Some(cap.id), cause))?;
    Ok(ConicalNativeCap {
        id: cap.id,
        face,
        material: cap.material.clone(),
        curves: cap.curves.clone(),
    })
}

fn member_error(member: &ConicalNativeMember, cause: Failure) -> ConicalBindingError {
    binding::error(Some(member.id), None, None, cause)
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkmath::gp::Transform3;

    #[test]
    fn one_translation_datum_preserves_the_verified_identity_matrix() {
        let transform = Transform3::translation([2., -1., 3.]);
        let proof = certify_frame_transport_at_angle(
            FrameTransportInput {
                axis: [0., 0., 1.],
                radial: [1., 0., 0.],
                stored: *transform.matrix().coefficients(),
            },
            0.,
        )
        .unwrap();
        let datum = Location::new(transform);
        assert!(check_datum(&Location::default(), &proof).is_ok());
        assert!(check_datum(&datum, &proof).is_ok());
        let repeated = datum.powered(2).unwrap();
        assert!(
            check_datum(&repeated, &proof)
                .unwrap_err()
                .message()
                .contains("datum at power one")
        );
    }
}
