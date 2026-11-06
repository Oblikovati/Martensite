//! `StepToGeom::make_ypr_rotation`: a spatial rotation of a kinematic pair as yaw, pitch and roll.
use super::StepToGeom;
use crate::step_basic::{
    StepBasicConversionBasedUnitAndPlaneAngleUnit, StepBasicMeasureWithUnit, StepBasicNamedUnit,
    StepBasicSiUnitAndPlaneAngleUnit, StepBasicSiUnitName,
};
use crate::step_construct::StepConstructUnitContext;
use crate::step_entity::{step_array1_set, step_live, step_new_array1};
use crate::step_kinematics::StepKinematicsSpatialRotation;
use crate::step_repr::{StepReprGlobalUnitAssignedContext, StepReprReprItemAndMeasureWithUnit};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomDirection, GeomVector};
use std::f64::consts::{FRAC_PI_2, PI};

const WHERE: &str = "StepToGeom::MakeYprRotation";

/// The array of the three angles.
fn ypr_array(yaw: f64, pitch: f64, roll: f64) -> Result<Handle<HArray1<f64>>, Failure> {
    let angles = step_new_array1::<f64>(1, 3)?;
    step_array1_set(&angles, 1, yaw, WHERE)?;
    step_array1_set(&angles, 2, pitch, WHERE)?;
    step_array1_set(&angles, 3, roll, WHERE)?;
    Ok(angles)
}

/// The only plane angle unit of a context, null when there is not exactly one.
fn single_plane_angle_unit(
    context: &StepReprGlobalUnitAssignedContext,
) -> Result<Handle<StepBasicNamedUnit>, Failure> {
    let mut found = Vec::new();
    for index in 1..=context.nb_units() {
        let unit = context.units_value(index)?;
        step_live(&unit, WHERE)?;
        if unit
            .cast::<StepBasicConversionBasedUnitAndPlaneAngleUnit>()
            .is_some()
            || unit.cast::<StepBasicSiUnitAndPlaneAngleUnit>().is_some()
        {
            found.push(unit);
        }
    }
    if found.len() != 1 {
        return Ok(Handle::default());
    }
    Ok(found.remove(0))
}

/// The measure of a conversion based unit; a null one is the null dereference.
fn conversion_measure(
    unit: &StepBasicConversionBasedUnitAndPlaneAngleUnit,
) -> Result<Handle<StepBasicMeasureWithUnit>, Failure> {
    let factor = unit.conversion_factor();
    step_live(&factor, WHERE)?;
    let measure = factor.cast::<StepBasicMeasureWithUnit>();
    if measure.is_some() {
        return Ok(measure);
    }
    Ok(factor
        .cast::<StepReprReprItemAndMeasureWithUnit>()
        .get()
        .map(StepReprReprItemAndMeasureWithUnit::get_measure_with_unit)
        .unwrap_or_default())
}

/// The angle in radians, following the conversion units down to the SI unit; `None` when that
/// unit is not the radian.
fn angle_in_radians(
    mut angle: f64,
    context: &StepReprGlobalUnitAssignedContext,
) -> Result<Option<f64>, Failure> {
    let mut unit = single_plane_angle_unit(context)?;
    while let Some(conversion) = unit
        .cast::<StepBasicConversionBasedUnitAndPlaneAngleUnit>()
        .get()
    {
        let measure = conversion_measure(conversion)?;
        let measure = step_live(&measure, WHERE)?;
        angle *= measure.value_component();
        unit = measure.unit_component().named_unit();
    }
    let si_unit = unit.cast::<StepBasicSiUnitAndPlaneAngleUnit>();
    let Some(si_unit) = si_unit
        .get()
        .filter(|live| live.name() == StepBasicSiUnitName::SunRadian)
    else {
        return Ok(None);
    };
    let prefix = if si_unit.has_prefix() {
        StepConstructUnitContext::convert_si_prefix(si_unit.prefix())
    } else {
        1.
    };
    Ok(Some(prefix * angle))
}

/// The axis parallel either to the x-axis or to the z-axis.
fn rotation_about_x_or_z(
    mut angle: f64,
    (dx, dz): (f64, f64),
    unit_factor: f64,
) -> Result<Handle<HArray1<f64>>, Failure> {
    while angle <= -PI {
        angle += 2. * PI;
    }
    while angle > PI {
        angle -= 2. * PI;
    }
    let yaw = unit_factor * angle;
    let roll = if (angle - PI).abs() >= precision::angular() {
        -yaw
    } else {
        yaw
    };
    if dx.abs() >= precision::confusion() {
        return ypr_array(0., 0., if dx > 0. { yaw } else { roll });
    }
    ypr_array(if dz > 0. { yaw } else { roll }, 0., 0.)
}

/// The axis parallel to the y-axis: it is the pitch axis.
fn rotation_about_y(
    angle: f64,
    dy: f64,
    unit_factor: f64,
) -> Result<Handle<HArray1<f64>>, Failure> {
    let (sin_a, cos_a) = martensite_cmath::sincos(angle);
    let yaw = if cos_a >= 0. { 0.0 } else { unit_factor * PI };
    let pitch = unit_factor * martensite_cmath::atan2(sin_a, cos_a.abs());
    ypr_array(yaw, if dy < 0. { -pitch } else { pitch }, yaw)
}

/// The yaw from the rotation matrix, in the quadrant the signs of the matrix give.
fn yaw_from_matrix(matrix: &[[f64; 3]; 3]) -> f64 {
    let yaw = martensite_cmath::atan2(-matrix[0][1], matrix[0][0]);
    if matrix[0][0] >= 0. {
        return yaw;
    }
    if yaw < 0. || yaw.abs() < precision::angular() {
        yaw + PI
    } else {
        yaw - PI
    }
}

/// `COS(pitch)` from the rotation matrix, by the largest of the trigonometric values at hand.
fn cos_pitch_from_matrix(matrix: &[[f64; 3]; 3], yaw: f64, roll: f64) -> f64 {
    let (sin_y, cos_y) = martensite_cmath::sincos(yaw);
    let (sin_r, cos_r) = martensite_cmath::sincos(roll);
    let (sy, cy, sr, cr) = (sin_y.abs(), cos_y.abs(), sin_r.abs(), cos_r.abs());
    if sy > cy && sy > sr && sy > cr {
        return -matrix[0][1] / sin_y;
    }
    if cy > sr && cy > cr {
        return matrix[0][0] / cos_y;
    }
    if sr > cr {
        return -matrix[1][2] / sin_r;
    }
    matrix[2][2] / cos_r
}

/// The axis not parallel to any axis of the coordinate system: compute the rotation matrix.
fn rotation_about_general_axis(
    angle: f64,
    (dx, dy, dz): (f64, f64, f64),
    unit_factor: f64,
) -> Result<Handle<HArray1<f64>>, Failure> {
    let (sin_a, cos_a) = martensite_cmath::sincos(angle);
    let cm1 = 1. - cos_a;
    let matrix = [
        [
            dx * dx * cm1 + cos_a,
            dx * dy * cm1 - dz * sin_a,
            dx * dz * cm1 + dy * sin_a,
        ],
        [
            dx * dy * cm1 + dz * sin_a,
            dy * dy * cm1 + cos_a,
            dy * dz * cm1 - dx * sin_a,
        ],
        [
            dx * dz * cm1 - dy * sin_a,
            dy * dz * cm1 + dx * sin_a,
            dz * dz * cm1 + cos_a,
        ],
    ];
    let roll = 0.;
    // matrix[0][2] equals SIN(pitch_angle). The test is the absolute value of `m - 1` twice,
    // so the case of pitch -PI/2 is never reached; kept as it is.
    let (yaw, pitch) = if (matrix[0][2] - 1.).abs() < precision::confusion() {
        // Only the sum or difference of roll and yaw angles is relevant and can be evaluated from
        // the matrix. According to IP `rectangular pitch angle' for ypr_rotation, the roll angle is
        // set to zero.
        let yaw = martensite_cmath::atan2(matrix[1][0], matrix[1][1]);
        let yaw = match (matrix[1][1] < 0., yaw <= 0.) {
            (true, true) => yaw + PI,
            (true, false) => yaw - PI,
            (false, _) => yaw,
        };
        (yaw, FRAC_PI_2)
    } else {
        // COS (pitch_angle) not equal to zero
        let yaw = yaw_from_matrix(&matrix);
        (
            yaw,
            martensite_cmath::atan2(matrix[0][2], cos_pitch_from_matrix(&matrix, yaw, roll)),
        )
    };
    ypr_array(yaw * unit_factor, pitch * unit_factor, roll * unit_factor)
}

impl StepToGeom {
    /// `make_ypr_rotation`: yaw, pitch and roll of a spatial rotation; null when the rotation or the
    /// context cannot give them.
    pub fn make_ypr_rotation(
        rotation: &StepKinematicsSpatialRotation,
        context: &Handle<StepReprGlobalUnitAssignedContext>,
    ) -> Result<Handle<HArray1<f64>>, Failure> {
        // If rotation is already a ypr_rotation, return it immediately
        let ypr = rotation.ypr_rotation();
        if let Some(live) = ypr.get()
            && live.array1().length() == 3
        {
            return Ok(ypr);
        }
        let about = rotation.rotation_about_direction();
        let Some(about) = about.get() else {
            return Ok(Handle::default());
        };
        let direction = about.direction_of_axis();
        let direction = step_live(&direction, WHERE)?;
        let Some(context) = context
            .get()
            .filter(|_| direction.nb_direction_ratios() == 3)
        else {
            return Ok(Handle::default());
        };
        // rotation is a rotation_about_direction
        let axis = GeomDirection::new(
            direction.direction_ratios_value(1)?,
            direction.direction_ratios_value(2)?,
            direction.direction_ratios_value(3)?,
        )?;
        let start_angle = about.rotation_angle();
        if start_angle.abs() < precision::angular() {
            // a zero rotation is converted trivially
            return ypr_array(0., 0., 0.);
        }
        let Some(angle) = angle_in_radians(start_angle, context)? else {
            return Ok(Handle::default());
        };
        let unit_factor = start_angle / angle;
        let (dx, dy, dz) = (axis.x(), axis.y(), axis.z());
        // axis parallel either to x-axis or to z-axis?
        if dy.abs() < precision::confusion() && (dx * dz).abs() < precision::square_confusion() {
            return rotation_about_x_or_z(angle, (dx, dz), unit_factor);
        }
        // axis parallel to y-axis - use y-axis as pitch axis
        if dy.abs() >= precision::confusion()
            && dx.abs() < precision::confusion()
            && dz.abs() < precision::confusion()
        {
            return rotation_about_y(angle, dy, unit_factor);
        }
        rotation_about_general_axis(angle, (dx, dy, dz), unit_factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::step_basic::StepBasicSiPrefix;
    use crate::step_data::StepDataSelectType;
    use crate::step_geom::StepGeomDirection;
    use crate::step_kinematics::StepKinematicsRotationAboutDirection;
    use martensite_tkernel::tcollection::HAsciiString;

    fn radian_context() -> Handle<StepReprGlobalUnitAssignedContext> {
        let unit = Handle::new(StepBasicSiUnitAndPlaneAngleUnit::new());
        unit.init(
            false,
            StepBasicSiPrefix::SpMilli,
            StepBasicSiUnitName::SunRadian,
        );
        let units = step_new_array1::<Handle<StepBasicNamedUnit>>(1, 1).unwrap();
        step_array1_set(&units, 1, unit.cast(), WHERE).unwrap();
        let context = Handle::new(StepReprGlobalUnitAssignedContext::new());
        context.init(
            &Handle::new(HAsciiString::new("")),
            &Handle::new(HAsciiString::new("")),
            &units,
        );
        context
    }

    fn rotation_about(axis: [f64; 3], angle: f64) -> StepKinematicsSpatialRotation {
        let ratios = step_new_array1::<f64>(1, 3).unwrap();
        for (index, value) in axis.iter().enumerate() {
            step_array1_set(&ratios, index as i32 + 1, *value, WHERE).unwrap();
        }
        let direction = Handle::new(StepGeomDirection::new());
        direction
            .init(&Handle::new(HAsciiString::new("")), &ratios)
            .unwrap();
        let about = Handle::new(StepKinematicsRotationAboutDirection::new());
        about.init(&Handle::new(HAsciiString::new("")), &direction, angle);
        let mut rotation = StepKinematicsSpatialRotation::default();
        rotation.set_value(&about.cast()).unwrap();
        rotation
    }

    fn angles(rotation: &StepKinematicsSpatialRotation) -> [f64; 3] {
        let made = StepToGeom::make_ypr_rotation(rotation, &radian_context()).unwrap();
        let live = made.get().unwrap();
        let items = live.array1();
        [
            *items.value(1).unwrap(),
            *items.value(2).unwrap(),
            *items.value(3).unwrap(),
        ]
    }

    #[test]
    fn a_rotation_about_z_is_a_yaw_and_about_x_a_roll() {
        assert_eq!(angles(&rotation_about([0., 0., 1.], 0.5)), [0.5, 0., 0.]);
        assert_eq!(angles(&rotation_about([1., 0., 0.], 0.5)), [0., 0., 0.5]);
    }

    #[test]
    fn a_rotation_about_y_is_a_pitch_and_a_negative_axis_reverses_it() {
        assert_eq!(
            angles(&rotation_about([0., 1., 0.], 0.5)),
            [
                0.,
                martensite_cmath::atan2(martensite_cmath::sin(0.5), martensite_cmath::cos(0.5)),
                0.
            ]
        );
        let pitch = angles(&rotation_about([0., -1., 0.], 0.5))[1];
        assert_eq!(
            pitch,
            -(martensite_cmath::atan2(martensite_cmath::sin(0.5), martensite_cmath::cos(0.5)))
        );
    }

    #[test]
    fn a_zero_rotation_is_converted_trivially_whatever_the_units() {
        assert_eq!(angles(&rotation_about([1., 2., 3.], 0.)), [0., 0., 0.]);
    }

    #[test]
    fn a_general_axis_keeps_the_roll_at_zero() {
        let made = angles(&rotation_about([1., 1., 1.], 0.5));
        assert_eq!(made[2], 0.);
        assert!(made[0].abs() > 0. && made[1].abs() > 0.);
    }

    #[test]
    fn a_rotation_given_as_angles_comes_back_as_it_is() {
        let given = step_new_array1::<f64>(1, 3).unwrap();
        let mut rotation = StepKinematicsSpatialRotation::default();
        rotation.set_value(&given.cast()).unwrap();
        let made = StepToGeom::make_ypr_rotation(&rotation, &Handle::default()).unwrap();
        assert!(made == given);
    }

    #[test]
    fn no_context_gives_a_null_and_no_plane_angle_unit_too() {
        let rotation = rotation_about([1., 1., 1.], 0.5);
        assert!(
            StepToGeom::make_ypr_rotation(&rotation, &Handle::default())
                .unwrap()
                .is_null()
        );
        let units = step_new_array1::<Handle<StepBasicNamedUnit>>(1, 0).unwrap();
        let context = Handle::new(StepReprGlobalUnitAssignedContext::new());
        context.init(
            &Handle::new(HAsciiString::new("")),
            &Handle::new(HAsciiString::new("")),
            &units,
        );
        assert!(
            StepToGeom::make_ypr_rotation(&rotation, &context)
                .unwrap()
                .is_null()
        );
    }
}
