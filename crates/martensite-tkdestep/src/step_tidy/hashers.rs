//! The hashers of the entities StepTidy merges: what makes two of them duplicates.
//!
//! Two entities merge only when they fall in one bucket and compare equal, so the hash decides as
//! well as the equality. The coordinate hash reads only the first three bytes of the first
//! coordinate (`hashBytes(data, size())` with `size()` 3), and the vector and circle hashes the
//! integer part of their value. These keep those inputs; the hash function itself is not the
//! one, and nothing depends on its values, since nothing observes the bucket order.
use crate::step_data::StepDataSelectType;
use crate::step_entity::{AsciiString, HAsciiString};
use crate::step_geom::{
    StepGeomAxis2Placement2d, StepGeomAxis2Placement3d, StepGeomCartesianPoint, StepGeomCircle,
    StepGeomDirection, StepGeomLine, StepGeomPlane, StepGeomVector,
};
use martensite_tkernel::ncollection::HashPolicy;
use martensite_tkernel::standard::Handle;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// The tolerance of the comparisons.
const TOLERANCE: f64 = 1e-12;

/// The Murmur hash seed: what stands for an absent part.
const ABSENT_PART: usize = 0xA329_F1D3;

fn combine(parts: &[usize]) -> usize {
    let mut hasher = DefaultHasher::new();
    parts.hash(&mut hasher);
    hasher.finish() as usize
}

fn hash_text(text: &AsciiString) -> usize {
    let mut hasher = DefaultHasher::new();
    text.as_bytes().hash(&mut hasher);
    hasher.finish() as usize
}

/// The hash with the entity's name in it, when it has one.
fn with_name(hash: usize, name: &Handle<HAsciiString>) -> usize {
    match name.get() {
        Some(live) => combine(&[hash, hash_text(&live.string())]),
        None => hash,
    }
}

/// `IsNull() !=` then `is_same_string`. Two null names dereference null, in a `noexcept`
/// function: here they compare equal, which no file can reach since a read name is never null.
fn same_name(first: &Handle<HAsciiString>, second: &Handle<HAsciiString>) -> bool {
    match (first.get(), second.get()) {
        (Some(a), Some(b)) => *a.string() == *b.string(),
        (None, None) => true,
        _ => false,
    }
}

/// The three bytes the coordinate hash reads.
fn coordinates_hash(coordinates: &[f64; 3]) -> usize {
    let bytes = coordinates[0].to_ne_bytes();
    combine(&[
        usize::from(bytes[0]),
        usize::from(bytes[1]),
        usize::from(bytes[2]),
    ])
}

fn same_coordinates(first: &[f64; 3], second: &[f64; 3], count: i32) -> bool {
    (0..count as usize).all(|index| (first[index] - second[index]).abs() <= TOLERANCE)
}

/// Declares a hasher and the `HashPolicy` over the two member functions given.
macro_rules! tidy_hasher {
    ($(#[$meta:meta])* $class:ident, $entity:ty, |$key:ident| $hash:expr, |$a:ident, $b:ident| $equal:expr) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default)]
        pub struct $class;
        impl HashPolicy<Handle<$entity>> for $class {
            fn hash(&self, $key: &Handle<$entity>) -> usize {
                $hash
            }
            fn equal(&self, $a: &Handle<$entity>, $b: &Handle<$entity>) -> bool {
                $equal
            }
        }
    };
}

tidy_hasher!(
    /// Cartesian point hasher.
    StepTidyCartesianPointHasher,
    StepGeomCartesianPoint,
    |point| with_name(coordinates_hash(&point.coordinates()), &point.name()),
    |a, b| {
        same_name(&a.name(), &b.name())
            && a.nb_coordinates() == b.nb_coordinates()
            && same_coordinates(&a.coordinates(), &b.coordinates(), a.nb_coordinates())
    }
);
tidy_hasher!(
    /// Direction hasher.
    StepTidyDirectionHasher,
    StepGeomDirection,
    |direction| with_name(coordinates_hash(&direction.direction_ratios()), &direction.name()),
    |a, b| {
        same_name(&a.name(), &b.name())
            && a.nb_direction_ratios() == b.nb_direction_ratios()
            && same_coordinates(&a.direction_ratios(), &b.direction_ratios(), a.nb_direction_ratios())
    }
);
tidy_hasher!(
    /// Vector hasher.
    StepTidyVectorHasher,
    StepGeomVector,
    |vector| {
        let parts = [StepTidyDirectionHasher.hash(&vector.orientation()), vector.magnitude() as i32 as usize];
        with_name(combine(&parts), &vector.name())
    },
    |a, b| {
        same_name(&a.name(), &b.name())
            && (a.magnitude() - b.magnitude()).abs() <= TOLERANCE
            && StepTidyDirectionHasher.equal(&a.orientation(), &b.orientation())
    }
);
tidy_hasher!(
    /// Line hasher.
    StepTidyLineHasher,
    StepGeomLine,
    |line| {
        let parts = [StepTidyCartesianPointHasher.hash(&line.pnt()), StepTidyVectorHasher.hash(&line.dir())];
        with_name(combine(&parts), &line.name())
    },
    |a, b| {
        same_name(&a.name(), &b.name())
            && StepTidyCartesianPointHasher.equal(&a.pnt(), &b.pnt())
            && StepTidyVectorHasher.equal(&a.dir(), &b.dir())
    }
);
tidy_hasher!(
    /// Axis2 placement3d hasher.
    StepTidyAxis2Placement3dHasher,
    StepGeomAxis2Placement3d,
    |placement| {
        let direction = |has: bool, value: Handle<StepGeomDirection>| {
            if has { StepTidyDirectionHasher.hash(&value) } else { ABSENT_PART }
        };
        let parts = [
            StepTidyCartesianPointHasher.hash(&placement.location()),
            direction(placement.has_axis(), placement.axis()),
            direction(placement.has_ref_direction(), placement.ref_direction()),
        ];
        with_name(combine(&parts), &placement.name())
    },
    |a, b| {
        same_name(&a.name(), &b.name())
            && StepTidyCartesianPointHasher.equal(&a.location(), &b.location())
            && a.has_axis() == b.has_axis()
            && (!a.has_axis() || StepTidyDirectionHasher.equal(&a.axis(), &b.axis()))
            && a.has_ref_direction() == b.has_ref_direction()
            && (!a.has_ref_direction() || StepTidyDirectionHasher.equal(&a.ref_direction(), &b.ref_direction()))
    }
);
tidy_hasher!(
    /// Axis2 placement2d hasher.
    StepTidyAxis2Placement2dHasher,
    StepGeomAxis2Placement2d,
    |placement| {
        let reference = if placement.has_ref_direction() {
            StepTidyDirectionHasher.hash(&placement.ref_direction())
        } else {
            ABSENT_PART
        };
        let parts = [StepTidyCartesianPointHasher.hash(&placement.location()), reference];
        with_name(combine(&parts), &placement.name())
    },
    |a, b| {
        same_name(&a.name(), &b.name())
            && StepTidyCartesianPointHasher.equal(&a.location(), &b.location())
            && a.has_ref_direction() == b.has_ref_direction()
            && (!a.has_ref_direction() || StepTidyDirectionHasher.equal(&a.ref_direction(), &b.ref_direction()))
    }
);
tidy_hasher!(
    /// Plane hasher.
    StepTidyPlaneHasher,
    StepGeomPlane,
    |plane| with_name(StepTidyAxis2Placement3dHasher.hash(&plane.position()), &plane.name()),
    |a, b| same_name(&a.name(), &b.name()) && StepTidyAxis2Placement3dHasher.equal(&a.position(), &b.position())
);
tidy_hasher!(
    /// Circle hasher.
    StepTidyCircleHasher,
    StepGeomCircle,
    |circle| {
        let position = circle.position();
        let (placed_2d, placed_3d) = (position.axis2_placement2d(), position.axis2_placement3d());
        let position_hash = if placed_2d.is_some() {
            StepTidyAxis2Placement2dHasher.hash(&placed_2d)
        } else if placed_3d.is_some() {
            StepTidyAxis2Placement3dHasher.hash(&placed_3d)
        } else {
            ABSENT_PART
        };
        with_name(combine(&[position_hash, circle.radius() as i32 as usize]), &circle.name())
    },
    |a, b| {
        let (first, second) = (a.position(), b.position());
        same_name(&a.name(), &b.name())
            && first.case_number() == second.case_number()
            && (first.case_number() != 1
                || StepTidyAxis2Placement2dHasher.equal(&first.axis2_placement2d(), &second.axis2_placement2d()))
            && (first.case_number() != 2
                || StepTidyAxis2Placement3dHasher.equal(&first.axis2_placement3d(), &second.axis2_placement3d()))
            && (a.radius() - b.radius()).abs() <= TOLERANCE
    }
);
