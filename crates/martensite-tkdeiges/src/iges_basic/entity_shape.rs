//! What the entity classes of the geometric packages add to the shape every entity has.
//!
//! Dozens of classes store a point or an axis as raw coordinates and give it back twice: as stored
//! and through the entity's location. `iges_placement_methods!` writes those members, one grammar
//! line each, so the shape exists once. The other functions are the shared-array reads the entity
//! members make beyond the ones of the base module.
use crate::iges_data::{
    IgesDataIgesEntity, IgesDataParamReader, IgesDataStatus, iges_live, iges_msg,
};
use martensite_tkernel::message::Msg;
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, RuntimeType, RuntimeTyped};
use martensite_tkmath::gp::{Direction3, GeneralTransform3, Point3, Vector3, Xyz};

/// Whether a handle's object is of a kind.
///
/// # Errors
/// A null object failure for a null handle.
pub fn iges_is_kind<T: ?Sized + RuntimeTyped>(
    handle: &Handle<T>,
    kind: &RuntimeType,
    where_called: &str,
) -> Result<bool, Failure> {
    iges_live(handle, where_called)?;
    Ok(handle.is_kind(kind))
}

/// Sends `message` as a fail, completed by why a read of an entity failed.
///
/// A wrong type is reported only by the readers that ask for a type.
///
/// # Errors
/// A failure when the message can't take its argument.
pub fn iges_send_read_status(
    pr: &mut IgesDataParamReader,
    message: &mut Msg,
    status: IgesDataStatus,
    typed: bool,
) -> Result<(), Failure> {
    let reason = match status {
        IgesDataStatus::ReferenceError => "IGES_216",
        IgesDataStatus::EntityError => "IGES_217",
        IgesDataStatus::TypeError if typed => "IGES_218",
        _ => return Ok(()),
    };
    let reason = iges_msg(reason);
    message.arg_extended(reason.value())?;
    pr.send_fail(message);
    Ok(())
}

/// The lower bound of a shared array.
///
/// # Errors
/// A null object failure for a null array.
pub fn iges_array1_lower<T: Send + Sync + 'static>(
    array: &Handle<HArray1<T>>,
    where_called: &str,
) -> Result<i32, Failure> {
    Ok(iges_live(array, where_called)?.array1().lower())
}

/// The upper bound of a shared array.
///
/// # Errors
/// A null object failure for a null array.
pub fn iges_array1_upper<T: Send + Sync + 'static>(
    array: &Handle<HArray1<T>>,
    where_called: &str,
) -> Result<i32, Failure> {
    Ok(iges_live(array, where_called)?.array1().upper())
}

/// The length of a shared array.
///
/// # Errors
/// A null object failure for a null array.
pub fn iges_array1_length<T: Send + Sync + 'static>(
    array: &Handle<HArray1<T>>,
    where_called: &str,
) -> Result<i32, Failure> {
    Ok(iges_live(array, where_called)?.array1().length())
}

/// Sets every item of a shared array.
///
/// # Errors
/// A null object failure for a null array.
pub fn iges_array1_fill<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray1<T>>,
    value: T,
    where_called: &str,
) -> Result<(), Failure> {
    iges_live(array, where_called)?.change_array1().init(value);
    Ok(())
}

macro_rules! array2_bound {
    ($($name:ident: $member:ident),* $(,)?) => {
        $(
            #[doc = concat!("`", stringify!($member), "` of a shared two-dimensional array.\n\n# Errors\nA null object failure for a null array.")]
            pub fn $name<T: Send + Sync + 'static>(
                array: &Handle<HArray2<T>>,
                where_called: &str,
            ) -> Result<i32, Failure> {
                Ok(iges_live(array, where_called)?.array2().$member())
            }
        )*
    };
}
array2_bound! {
    iges_array2_lower_row: lower_row,
    iges_array2_upper_row: upper_row,
    iges_array2_lower_col: lower_col,
    iges_array2_upper_col: upper_col,
    iges_array2_row_length: row_length,
    iges_array2_col_length: col_length,
}

/// Sets every item of a shared two-dimensional array.
///
/// # Errors
/// A null object failure for a null array.
pub fn iges_array2_fill<T: Clone + Send + Sync + 'static>(
    array: &Handle<HArray2<T>>,
    value: T,
    where_called: &str,
) -> Result<(), Failure> {
    iges_live(array, where_called)?.change_array2().init(value);
    Ok(())
}

/// A unit direction along raw coordinates.
///
/// # Errors
/// A construction failure for coordinates of no length.
pub fn iges_direction(coordinates: Xyz) -> Result<Direction3, Failure> {
    Direction3::new(coordinates.x(), coordinates.y(), coordinates.z())
}

/// The location of an entity without its translation, which is what applies to a direction.
///
/// # Errors
/// A failure when the location is not set up.
pub fn iges_vector_location(entity: &IgesDataIgesEntity) -> Result<GeneralTransform3, Failure> {
    let mut location = entity.location()?;
    location.set_translation_part(Xyz::new(0.0, 0.0, 0.0));
    Ok(location)
}

/// Raw coordinates of a point after the location of an entity.
///
/// # Errors
/// A failure when the location is not set up.
pub fn iges_located_point(
    entity: &IgesDataIgesEntity,
    coordinates: Xyz,
) -> Result<Point3, Failure> {
    if !entity.has_transf() {
        return Ok(Point3::from(coordinates));
    }
    let mut located = coordinates;
    entity.location()?.transforms(&mut located);
    Ok(Point3::from(located))
}

/// Raw coordinates of a vector after the location of an entity, without its translation.
///
/// # Errors
/// A failure when the location is not set up.
pub fn iges_located_vector(
    entity: &IgesDataIgesEntity,
    coordinates: Xyz,
) -> Result<Vector3, Failure> {
    if !entity.has_transf() {
        return Ok(Vector3::from(coordinates));
    }
    let mut located = coordinates;
    iges_vector_location(entity)?.transforms(&mut located);
    Ok(Vector3::from(located))
}

/// Raw coordinates of a direction after the location of an entity, without its translation.
///
/// # Errors
/// A failure when the location is not set up, a construction failure for no length.
pub fn iges_located_direction(
    entity: &IgesDataIgesEntity,
    coordinates: Xyz,
) -> Result<Direction3, Failure> {
    if !entity.has_transf() {
        return iges_direction(coordinates);
    }
    let mut located = coordinates;
    iges_vector_location(entity)?.transforms(&mut located);
    iges_direction(located)
}

/// One patch of sixteen coefficients, as a spline surface holds them.
#[cfg(test)]
pub(crate) fn spline_coefficients() -> Result<Handle<HArray2<Handle<HArray1<f64>>>>, Failure> {
    use crate::iges_data::{iges_array2_set, iges_new_array1, iges_new_array2};
    let patches = iges_new_array2::<Handle<HArray1<f64>>>(1, 1, 1, 1)?;
    iges_array2_set(
        &patches,
        1,
        1,
        iges_new_array1::<f64>(1, 16)?,
        "spline_coefficients",
    )?;
    Ok(patches)
}

/// Operands of a post-order tree: two lines, then an operation.
#[cfg(test)]
pub(crate) fn boolean_tree_operands() -> Result<Handle<HArray1<Handle<IgesDataIgesEntity>>>, Failure>
{
    use crate::iges_data::{iges_array1_set, iges_new_array1};
    use crate::iges_geom::samples_01::sample_iges_geom_line;
    let operands = iges_new_array1(1, 3)?;
    for rank in 1..=2 {
        iges_array1_set(
            &operands,
            rank,
            sample_iges_geom_line()?.cast(),
            "boolean_tree_operands",
        )?;
    }
    Ok(operands)
}

/// Declares a stateless tool class.
macro_rules! iges_tool {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $name;
        impl $name {
            /// A tool, ready to work.
            pub fn new() -> Self {
                Self
            }
        }
    };
}
pub(crate) use iges_tool;

/// The placement members of an entity class, one grammar line each, inside its `impl` block.
///
/// `has`: whether a handle field is set. `item2`: the array item at a row and a column.
/// `point`, `vector`, `direction`: the raw field as that value. `crossed`: the direction normal to
/// two raw fields. The `located_` forms apply the location of the entity first.
macro_rules! iges_placement_methods {
    () => {};
    (has $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("Whether the `", stringify!($field), "` field is set.")]
        pub fn $name(&self) -> bool {
            !self.state.read().$field.is_null()
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (item2 $name:ident = $field:ident: $ty:ty; $($rest:tt)*) => {
        #[doc = concat!("The item at a row and a column of `", stringify!($field), "`.\n\n# Errors\nA null object failure for a null array, a failure for a bad rank.")]
        pub fn $name(&self, row: i32, column: i32) -> Result<$ty, martensite_tkernel::standard::Failure> {
            let items = self.state.read().$field.clone();
            $crate::iges_data::iges_array2_item(&items, row, column, stringify!($name))
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (point $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The `", stringify!($field), "` field as a point.")]
        pub fn $name(&self) -> martensite_tkmath::gp::Point3 {
            martensite_tkmath::gp::Point3::from(self.state.read().$field)
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (vector $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The `", stringify!($field), "` field as a vector.")]
        pub fn $name(&self) -> martensite_tkmath::gp::Vector3 {
            martensite_tkmath::gp::Vector3::from(self.state.read().$field)
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (direction $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The `", stringify!($field), "` field as a direction.\n\n# Errors\nA construction failure for no length.")]
        pub fn $name(&self) -> Result<martensite_tkmath::gp::Direction3, martensite_tkernel::standard::Failure> {
            $crate::iges_basic::entity_shape::iges_direction(self.state.read().$field)
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (crossed $name:ident = $first:ident, $second:ident; $($rest:tt)*) => {
        #[doc = concat!("The direction normal to `", stringify!($first), "` and `", stringify!($second), "`.\n\n# Errors\nA construction failure for parallel fields.")]
        pub fn $name(&self) -> Result<martensite_tkmath::gp::Direction3, martensite_tkernel::standard::Failure> {
            let state = self.state.read();
            $crate::iges_basic::entity_shape::iges_direction(state.$first.crossed(state.$second))
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (located_point $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The `", stringify!($field), "` field as a point, after the location of this entity.\n\n# Errors\nA failure when the location is not set up.")]
        pub fn $name(&self) -> Result<martensite_tkmath::gp::Point3, martensite_tkernel::standard::Failure> {
            let coordinates = self.state.read().$field;
            $crate::iges_basic::entity_shape::iges_located_point($crate::iges_data::IgesDataEntityKind::iges_entity(self), coordinates)
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (located_vector $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The `", stringify!($field), "` field as a vector, after the location of this entity without its translation.\n\n# Errors\nA failure when the location is not set up.")]
        pub fn $name(&self) -> Result<martensite_tkmath::gp::Vector3, martensite_tkernel::standard::Failure> {
            let coordinates = self.state.read().$field;
            $crate::iges_basic::entity_shape::iges_located_vector($crate::iges_data::IgesDataEntityKind::iges_entity(self), coordinates)
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (located_direction $name:ident = $field:ident; $($rest:tt)*) => {
        #[doc = concat!("The `", stringify!($field), "` field as a direction, after the location of this entity without its translation.\n\n# Errors\nA failure when the location is not set up, a construction failure for no length.")]
        pub fn $name(&self) -> Result<martensite_tkmath::gp::Direction3, martensite_tkernel::standard::Failure> {
            let coordinates = self.state.read().$field;
            $crate::iges_basic::entity_shape::iges_located_direction($crate::iges_data::IgesDataEntityKind::iges_entity(self), coordinates)
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
    (located_crossed $name:ident = $first:ident, $second:ident; $($rest:tt)*) => {
        #[doc = concat!("The direction normal to `", stringify!($first), "` and `", stringify!($second), "`, after the location of this entity without its translation.\n\n# Errors\nA failure when the location is not set up, a construction failure for parallel fields.")]
        pub fn $name(&self) -> Result<martensite_tkmath::gp::Direction3, martensite_tkernel::standard::Failure> {
            let coordinates = {
                let state = self.state.read();
                state.$first.crossed(state.$second)
            };
            $crate::iges_basic::entity_shape::iges_located_direction($crate::iges_data::IgesDataEntityKind::iges_entity(self), coordinates)
        }
        $crate::iges_basic::entity_shape::iges_placement_methods!($($rest)*);
    };
}
pub(crate) use iges_placement_methods;
