// SPDX-License-Identifier: GPL-2.0-only
//! A complete boundary inside a Cartesian box also bounds the enclosed solid.
use super::*;
use crate::predicates::roots::metric_bounds::cylindrical_profile::trigonometric;
type SolidBox = [Interval; 3];

pub(super) fn cap_box(
    u: &Interval,
    v: &Interval,
    radius: &Interval,
    z0: &Interval,
    cosine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<SolidBox> {
    let [cos_u, sin_u] = angle_box(u, arithmetic)?;
    let z = arithmetic.multiply(v, cosine)?;
    Ok([
        arithmetic.multiply(radius, &cos_u)?,
        arithmetic.multiply(radius, &sin_u)?,
        arithmetic.add(z0, &z)?,
    ])
}
fn angle_box(angles: &Interval, arithmetic: &mut Arithmetic<'_>) -> Outcome<[Interval; 2]> {
    let mid: num_bigint::BigInt = (&angles.lower + &angles.upper) / 2;
    let center = Interval {
        lower: mid.clone(),
        upper: mid,
    };
    let offset = arithmetic.subtract(angles, &center)?.magnitude();
    let [cosine, sine] =
        trigonometric::scalar(&center, arithmetic).map_err(|cause| match cause {
            CylindricalProfileFailure::Arithmetic(error) => (*error).into(),
            _ => error::unresolved(
                "stored atlas cap phase for box construction",
                angles,
                "bounded trigonometric enclosure",
                arithmetic.fractional_bits(),
            ),
        })?;
    // Both trigonometric functions are 1-Lipschitz. The rectangle includes all filled UV.
    Ok([cosine, sine].map(|value| Interval {
        lower: value.lower - &offset,
        upper: value.upper + &offset,
    }))
}
struct MemberBox<'a> {
    caps: Vec<&'a CapCapture>,
    extent: SolidBox,
}
pub(super) fn certify(
    caps: &[CapCapture],
    sides: &[SideCapture],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<()> {
    let mut members = cap_extents(caps, arithmetic)?;
    for side in sides {
        arithmetic.check_cancelled()?;
        let member = members.get_mut(&side.source.member).ok_or_else(|| {
            error::shape(
                "atlas side member",
                side.source.member.0 as usize,
                "captured cap owner",
            )
        })?;
        for poles in &side.coordinates {
            for pole in poles {
                include(&mut member.extent, pole);
            }
        }
    }
    let mut extents: Vec<_> = members.values().collect();
    extents.sort_unstable_by(|a, b| {
        a.extent[0].lower.cmp(&b.extent[0].lower).then_with(|| {
            a.caps[0]
                .source
                .carrier()
                .id
                .cmp(&b.caps[0].source.carrier().id)
        })
    });
    separated(&extents, arithmetic)
}
fn cap_extents<'a>(
    caps: &'a [CapCapture],
    arithmetic: &mut Arithmetic<'_>,
) -> Outcome<BTreeMap<ConicalActualMemberId, MemberBox<'a>>> {
    let mut members = BTreeMap::new();
    for cap in caps {
        arithmetic.check_cancelled()?;
        let carrier = cap.source.carrier();
        let CapExtent::Atlas(bounds) = &cap.extent else {
            return Err(error::shape(
                "atlas cap box",
                carrier.id.0 as usize,
                "complete filled-cap enclosure",
            ));
        };
        let bounds = bounds.as_ref();
        let member = members.entry(carrier.member).or_insert_with(|| MemberBox {
            caps: Vec::new(),
            extent: bounds.clone(),
        });
        for previous in &member.caps {
            pairs::pair(previous, cap, arithmetic)
                .map_err(|cause| cap_pair(previous, cap, cause))?;
        }
        member.caps.push(cap);
        include(&mut member.extent, bounds);
    }
    Ok(members)
}
fn include(target: &mut SolidBox, source: &SolidBox) {
    for (stored, incoming) in target.iter_mut().zip(source) {
        if incoming.lower < stored.lower {
            stored.lower.clone_from(&incoming.lower);
        }
        if incoming.upper > stored.upper {
            stored.upper.clone_from(&incoming.upper);
        }
    }
}
fn separated(boxes: &[&MemberBox<'_>], arithmetic: &mut Arithmetic<'_>) -> Outcome<()> {
    for (index, first) in boxes.iter().enumerate() {
        arithmetic.check_cancelled()?;
        for second in &boxes[index + 1..] {
            arithmetic.certificate_step()?;
            if first.extent[0].upper < second.extent[0].lower {
                break;
            }
            let cause = positive_gap(&first.extent, &second.extent, arithmetic);
            cause.map_err(|cause| cap_pair(first.caps[0], second.caps[0], cause))?;
        }
    }
    Ok(())
}
fn positive_gap(a: &SolidBox, b: &SolidBox, arithmetic: &mut Arithmetic<'_>) -> Outcome<()> {
    let gap = a
        .iter()
        .zip(b)
        .map(|(a, b)| (&a.lower - &b.upper).max(&b.lower - &a.upper))
        .max()
        .unwrap_or_default();
    let checked = Interval {
        lower: gap.clone(),
        upper: gap,
    };
    numeric::positive(
        "separation of complete stored atlas member boxes",
        &checked,
        arithmetic,
    )
}
fn cap_pair(
    a: &CapCapture,
    b: &CapCapture,
    cause: ConicalActualPatchFailure,
) -> ConicalActualPatchFailure {
    ConicalActualPatchFailure::CapPair {
        caps: [a.source.carrier().id, b.source.carrier().id],
        cause: Box::new(cause),
    }
}
