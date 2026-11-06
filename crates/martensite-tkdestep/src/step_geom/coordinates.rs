//! The two entities that keep their coordinates inline (at most three) rather than in an array:
//! the classes differ only in names, so they share one macro here.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_entity::*;
use martensite_tkernel::standard::exceptions;

/// Implements a class over the pair `myNbCoord` and `myCoords`.
macro_rules! inline_coordinates {
    ($class:ident, $recorded:literal, $set:ident, $set_array:ident, $get:ident, $value:ident, $set_count:ident, $count:ident) => {
        impl $class {
            #[doc = concat!("`", $recorded, "::Init`.\n\n# Errors\n`Standard_NullObject` for a null array.")]
            pub fn init(
                &self,
                the_name: &Handle<HAsciiString>,
                the_coordinates: &Handle<HArray1<f64>>,
            ) -> Result<(), Failure> {
                self.$set(the_coordinates)?;
                self.parent.init(the_name);
                Ok(())
            }
            #[doc = concat!("`", $recorded, "::Init2D`.")]
            pub fn init_2d(&self, the_name: &Handle<HAsciiString>, the_x: f64, the_y: f64) {
                {
                    let mut state = self.state.write();
                    state.my_nb_coord = 2;
                    state.my_coords = [the_x, the_y, 0.0];
                }
                self.parent.init(the_name);
            }
            #[doc = concat!("`", $recorded, "::Init3D`.")]
            pub fn init_3d(&self, the_name: &Handle<HAsciiString>, the_x: f64, the_y: f64, the_z: f64) {
                {
                    let mut state = self.state.write();
                    state.my_nb_coord = 3;
                    state.my_coords = [the_x, the_y, the_z];
                }
                self.parent.init(the_name);
            }
            #[doc = concat!("`", $recorded, "`: the count follows the array, and at most three values are kept.\n\n# Errors\n`Standard_NullObject` for a null array.")]
            pub fn $set(&self, the_coordinates: &Handle<HArray1<f64>>) -> Result<(), Failure> {
                let values = step_live(the_coordinates, $recorded)?.array1();
                let mut state = self.state.write();
                state.my_nb_coord = values.length();
                for (slot, value) in state.my_coords.iter_mut().zip(values.iter()) {
                    *slot = *value;
                }
                Ok(())
            }
            #[doc = concat!("`", $recorded, "`: the array overload, which keeps the count as it is.")]
            pub fn $set_array(&self, the_coordinates: &[f64; 3]) {
                self.state.write().my_coords = *the_coordinates;
            }
            #[doc = concat!("The three stored values of `", $recorded, "`.")]
            pub fn $get(&self) -> [f64; 3] {
                self.state.read().my_coords
            }
            #[doc = concat!("The value at a 1-based rank of `", $recorded, "`.\n\n# Errors\nAn out of range failure outside 1 to 3.")]
            pub fn $value(&self, the_ind: i32) -> Result<f64, Failure> {
                let coordinates = self.state.read().my_coords;
                usize::try_from(the_ind - 1)
                    .ok()
                    .and_then(|offset| coordinates.get(offset).copied())
                    .ok_or_else(|| {
                        exceptions::out_of_range(&format!("{}: rank={the_ind}; expected 1 to 3", $recorded))
                    })
            }
            #[doc = concat!("Sets the count of `", $recorded, "`.")]
            pub fn $set_count(&self, the_size: i32) {
                self.state.write().my_nb_coord = the_size;
            }
            #[doc = concat!("The count of `", $recorded, "`.")]
            pub fn $count(&self) -> i32 {
                self.state.read().my_nb_coord
            }
        }
    };
}

step_entity! {
    /// Cartesian point.
    StepGeomCartesianPoint, StepGeomCartesianPointState, "StepGeomCartesianPoint", StepGeomPoint, {
        my_nb_coord: i32,
        my_coords: [f64; 3],
    }
}
inline_coordinates!(
    StepGeomCartesianPoint,
    "StepGeomCartesianPoint",
    set_coordinates,
    set_coordinates_array,
    coordinates,
    coordinates_value,
    set_nb_coordinates,
    nb_coordinates
);

step_entity! {
    /// Direction.
    StepGeomDirection, StepGeomDirectionState, "StepGeomDirection", StepGeomGeometricRepresentationItem, {
        my_nb_coord: i32,
        my_coords: [f64; 3],
    }
}
inline_coordinates!(
    StepGeomDirection,
    "StepGeomDirection",
    set_direction_ratios,
    set_direction_ratios_array,
    direction_ratios,
    direction_ratios_value,
    set_nb_direction_ratios,
    nb_direction_ratios
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_array_setter_keeps_at_most_three_values_and_the_full_count() {
        let values = HArray1::<f64>::with_bounds(1, 4).unwrap();
        for rank in 1..=4 {
            values
                .change_array1()
                .set_value(rank, f64::from(rank))
                .unwrap();
        }
        let point = StepGeomCartesianPoint::new();
        point
            .init(&Handle::new(HAsciiString::new("p")), &Handle::new(values))
            .unwrap();
        assert_eq!(point.nb_coordinates(), 4);
        assert_eq!(point.coordinates(), [1.0, 2.0, 3.0]);
        assert_eq!(point.name().string().as_bytes(), b"p");
    }

    #[test]
    fn a_null_array_and_a_bad_rank_fail_with_the_exceptions() {
        let direction = StepGeomDirection::new();
        let error = direction
            .set_direction_ratios(&Handle::default())
            .unwrap_err();
        assert!(error.is_kind("Standard_NullObject"));
        direction.init_2d(&Handle::default(), 1.0, 0.0);
        assert_eq!(direction.nb_direction_ratios(), 2);
        assert!(
            direction
                .direction_ratios_value(4)
                .unwrap_err()
                .is_kind("Standard_OutOfRange")
        );
        assert!(
            direction
                .direction_ratios_value(0)
                .unwrap_err()
                .is_kind("Standard_OutOfRange")
        );
    }

    #[test]
    fn a_point_casts_to_its_ancestors() {
        let point = Handle::new(StepGeomCartesianPoint::new());
        assert!(point.is_kind_name("StepReprRepresentationItem"));
        assert!(point.cast::<StepGeomPoint>().is_some());
    }
}
