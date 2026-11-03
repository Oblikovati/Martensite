//! Replay owns every datum and checks the evaluator cache before using it.
use super::{budget::*, types::*};
use martensite_tkbrep::brep::face_input_capture::CapturedLocation;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::{
    gp::Transform3,
    predicates::{compare_scalar_triple_product_3d, roots::metric_bounds::MetricEmbedding},
    toploc::{Datum3, Location},
};
use std::{cmp::Ordering, collections::BTreeMap};
pub(super) struct PlacementReplay {
    datums: BTreeMap<usize, Handle<Datum3>>,
}
impl PlacementReplay {
    pub fn new() -> Self {
        Self {
            datums: BTreeMap::new(),
        }
    }
    pub fn location<F: FnMut() -> bool>(
        &mut self,
        input: &CapturedLocation,
        budget: &mut Budget<'_, F>,
    ) -> CheckResult<Location> {
        let mut result = Location::default();
        for (id, power, transform) in input.steps().iter().rev() {
            budget.location_step(*power)?;
            let datum = self.datum(*id, *transform)?;
            let item = Location::from_datum(datum)
                .powered(*power)
                .map_err(native)?;
            result = result.multiplied(&item).map_err(native)?;
        }
        if !same_transform(result.transformation(), input.effective_transform())? {
            return Err(unsupported(
                0,
                "location cache",
                "replay mismatch",
                "bit-identical captured evaluator map",
            ));
        }
        Ok(result)
    }
    fn datum(&mut self, id: usize, transform: Transform3) -> CheckResult<Handle<Datum3>> {
        if let Some(datum) = self.datums.get(&id) {
            if !same_transform(*datum.transformation(), transform)? {
                return Err(invalid(
                    id,
                    "datum alias",
                    "one datum has different coefficients",
                ));
            }
            return Ok(datum.clone());
        }
        let datum = Handle::new(Datum3::new(transform));
        self.datums.insert(id, datum.clone());
        Ok(datum)
    }
}
pub(super) fn native(error: martensite_tkernel::standard::Failure) -> ClosedFaceCompletion {
    unsupported(
        0,
        "placement operation",
        format!("{error:?}"),
        "finite supported affine location arithmetic",
    )
}
fn same_transform(first: Transform3, second: Transform3) -> CheckResult<bool> {
    if first.form() != second.form()
        || first.scale_factor().to_bits() != second.scale_factor().to_bits()
    {
        return Ok(false);
    }
    for row in 1..=3 {
        for column in 1..=4 {
            if first.value(row, column).map_err(native)?.to_bits()
                != second.value(row, column).map_err(native)?.to_bits()
            {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
pub(super) fn embedding(location: &Location) -> CheckResult<MetricEmbedding<3>> {
    let transform = location.transformation();
    let matrix = transform.matrix();
    let t = transform.translation_part();
    let mut linear = [[0.; 3]; 3];
    for row in 0..3 {
        for column in 0..3 {
            linear[row][column] = matrix
                .value(row as i32 + 1, column as i32 + 1)
                .map_err(native)?;
        }
    }
    let scale = transform.scale_factor();
    if !scale.is_finite() || scale == 0. || [t.x(), t.y(), t.z()].iter().any(|v| !v.is_finite()) {
        return Err(unsupported(
            0,
            "placement",
            format!("{transform:?}"),
            "finite invertible affine map",
        ));
    }
    let sign = compare_scalar_triple_product_3d(linear[0], linear[1], linear[2]).map_err(|e| {
        unsupported(
            0,
            "determinant",
            format!("{e:?}"),
            "finite nonzero determinant",
        )
    })?;
    if sign == Ordering::Equal {
        return Err(unsupported(0, "determinant", "zero", "nonzero determinant"));
    }
    Ok(MetricEmbedding {
        linear,
        origin: [t.x(), t.y(), t.z()],
        scale,
    })
}
pub(super) fn identity() -> MetricEmbedding<3> {
    MetricEmbedding {
        linear: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        origin: [0.; 3],
        scale: 1.,
    }
}
