//! The reducers: the sharing classes each entity class can be replaced in.
use super::StepTidyEntityReducer;
use super::hashers::{
    StepTidyAxis2Placement3dHasher, StepTidyCartesianPointHasher, StepTidyCircleHasher,
    StepTidyDirectionHasher, StepTidyLineHasher, StepTidyPlaneHasher, StepTidyVectorHasher,
};
use crate::step_data::StepDataSelectType;
use crate::step_entity::{
    step_array1_item, step_array1_set, step_array2_item, step_array2_set, step_live,
};
use crate::step_geom::{
    StepGeomAxis1Placement, StepGeomAxis2Placement3d, StepGeomBSplineCurveWithKnots,
    StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve, StepGeomBSplineSurfaceWithKnots,
    StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface, StepGeomCartesianPoint,
    StepGeomCircle, StepGeomConicalSurface, StepGeomCylindricalSurface, StepGeomDirection,
    StepGeomEllipse, StepGeomLine, StepGeomPcurve, StepGeomPlane, StepGeomSeamCurve,
    StepGeomSphericalSurface, StepGeomSurfaceCurve, StepGeomToroidalSurface, StepGeomTrimmedCurve,
    StepGeomVector,
};
use crate::step_repr::{
    StepReprConstructiveGeometryRepresentation, StepReprDefinitionalRepresentation,
    StepReprItemDefinedTransformation, StepReprRepresentation, StepReprRepresentationItem,
};
use crate::step_shape::{
    StepShapeAdvancedBrepShapeRepresentation, StepShapeAdvancedFace, StepShapeEdgeCurve,
    StepShapeGeometricCurveSet, StepShapeGeometricSet, StepShapeShapeRepresentation,
    StepShapeVertexPoint,
};
use crate::step_visual::{StepVisualPresentationLayerAssignment, StepVisualStyledItem};
use martensite_tkernel::ncollection::{HArray1, HArray2};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::ops::{Deref, DerefMut};

const WHERE: &str = "StepTidy reducer";

/// Declares a reducer on the generic one: its constructor registers the replacers.
macro_rules! reducer {
    ($(#[$meta:meta])* $class:ident, $entity:ty, $hasher:ty, [$($sharing:ty => $replacer:expr),+ $(,)?]) => {
        $(#[$meta])*
        pub struct $class(StepTidyEntityReducer<$entity, $hasher>);
        impl $class {
            #[doc = concat!("`", stringify!($class), "(theWS)`.")]
            pub fn new(ws: &Handle<XSControlWorkSession>) -> Self {
                let mut base = StepTidyEntityReducer::<$entity, $hasher>::new(ws);
                $(base.register_replacer(<$sharing>::static_type(), $replacer);)+
                Self(base)
            }
        }
        impl Deref for $class {
            type Target = StepTidyEntityReducer<$entity, $hasher>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $class {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

/// A replacer on one field: `if (get() == old) { set(new); true }`.
macro_rules! field_replacer {
    ($sharing:ty, $get:ident, $set:ident) => {
        |old, new, sharing| {
            let typed: Handle<$sharing> = sharing.cast();
            let live = step_live(&typed, WHERE)?;
            if live.$get().object_address() == old.object_address() {
                live.$set(&new.cast());
                return Ok(true);
            }
            Ok(false)
        }
    };
}

/// A replacer on the item list of a representation.
macro_rules! items_replacer {
    ($sharing:ty) => {
        |old, new, sharing| {
            let typed: Handle<$sharing> = sharing.cast();
            replace_in_items(&step_live(&typed, WHERE)?.items(), &old.cast(), &new.cast())
        }
    };
}

/// A replacer on a control-point net.
macro_rules! net_replacer {
    ($sharing:ty) => {
        |old, new, sharing| {
            let typed: Handle<$sharing> = sharing.cast();
            replace_in_net(&step_live(&typed, WHERE)?.control_points_list(), old, new)
        }
    };
}

/// A replacer on a control-point list.
macro_rules! list_replacer {
    ($sharing:ty) => {
        |old, new, sharing| {
            let typed: Handle<$sharing> = sharing.cast();
            replace_in_list(&step_live(&typed, WHERE)?.control_points_list(), old, new)
        }
    };
}

/// Replaces the item where the array holds it.
fn replace_in_items(
    items: &Handle<HArray1<Handle<StepReprRepresentationItem>>>,
    old: &Handle<StepReprRepresentationItem>,
    new: &Handle<StepReprRepresentationItem>,
) -> Result<bool, Failure> {
    replace_in_list(items, old, new)
}

/// Replaces the entity everywhere the array holds it.
fn replace_in_list<T: RuntimeTyped + Send + Sync + 'static>(
    list: &Handle<HArray1<Handle<T>>>,
    old: &Handle<T>,
    new: &Handle<T>,
) -> Result<bool, Failure> {
    let live = step_live(list, WHERE)?;
    let (lower, upper) = {
        let items = live.array1();
        (items.lower(), items.upper())
    };
    let mut replaced = false;
    for rank in lower..=upper {
        if step_array1_item(list, rank, WHERE)?.object_address() == old.object_address() {
            step_array1_set(list, rank, new.clone(), WHERE)?;
            replaced = true;
        }
    }
    Ok(replaced)
}

/// Replaces the entity everywhere the net holds it.
fn replace_in_net<T: RuntimeTyped + Send + Sync + 'static>(
    net: &Handle<HArray2<Handle<T>>>,
    old: &Handle<T>,
    new: &Handle<T>,
) -> Result<bool, Failure> {
    let live = step_live(net, WHERE)?;
    let (rows, cols) = {
        let items = live.array2();
        (
            (items.lower_row(), items.upper_row()),
            (items.lower_col(), items.upper_col()),
        )
    };
    let mut replaced = false;
    for row in rows.0..=rows.1 {
        for col in cols.0..=cols.1 {
            if step_array2_item(net, row, col, WHERE)?.object_address() == old.object_address() {
                step_array2_set(net, row, col, new.clone(), WHERE)?;
                replaced = true;
            }
        }
    }
    Ok(replaced)
}

/// `replaceGeometricCurveSet`: it casts to the parent geometric_set.
fn point_in_geometric_set(
    old: &Handle<StepGeomCartesianPoint>,
    new: &Handle<StepGeomCartesianPoint>,
    sharing: &Handle<StandardTransient>,
) -> Result<bool, Failure> {
    let typed: Handle<StepShapeGeometricSet> = sharing.cast();
    let elements = step_live(&typed, WHERE)?.elements();
    let live = step_live(&elements, WHERE)?;
    let (lower, upper) = {
        let items = live.array1();
        (items.lower(), items.upper())
    };
    let mut replaced = false;
    for rank in lower..=upper {
        let mut element = step_array1_item(&elements, rank, WHERE)?;
        let point = element.point();
        if point.is_some() && point.object_address() == old.object_address() {
            element.set_value(&new.cast())?;
            step_array1_set(&elements, rank, element, WHERE)?;
            replaced = true;
        }
    }
    Ok(replaced)
}

/// The layered items of a presentation_layer_assignment.
fn in_layer_assignment<T: RuntimeTyped>(
    old: &Handle<T>,
    new: &Handle<T>,
    sharing: &Handle<StandardTransient>,
) -> Result<bool, Failure> {
    let typed: Handle<StepVisualPresentationLayerAssignment> = sharing.cast();
    let items = step_live(&typed, WHERE)?.assigned_items();
    let live = step_live(&items, WHERE)?;
    let (lower, upper) = {
        let list = live.array1();
        (list.lower(), list.upper())
    };
    let mut replaced = false;
    for rank in lower..=upper {
        let mut item = step_array1_item(&items, rank, WHERE)?;
        if item.representation_item().object_address() == old.object_address() {
            item.set_value(&new.cast())?;
            step_array1_set(&items, rank, item, WHERE)?;
            replaced = true;
        }
    }
    Ok(replaced)
}

/// The item of a styled_item: `set_item` takes a representation item.
fn in_styled_item<T: RuntimeTyped>(
    old: &Handle<T>,
    new: &Handle<T>,
    sharing: &Handle<StandardTransient>,
) -> Result<bool, Failure> {
    let typed: Handle<StepVisualStyledItem> = sharing.cast();
    let live = step_live(&typed, WHERE)?;
    if live.item().object_address() == old.object_address() {
        live.set_item(&new.cast());
        return Ok(true);
    }
    Ok(false)
}

/// ReplaceCircle and replaceEllipse: `aSelector` is a copy of the position, so
/// `set_value` on it leaves the entity as it was, and they report success. Kept: the old placement
/// then stays in the model through the curve's reference.
macro_rules! conic_position_replacer {
    ($sharing:ty) => {
        |old, new, sharing| {
            let typed: Handle<$sharing> = sharing.cast();
            let mut selector = step_live(&typed, WHERE)?.position();
            if selector.axis2_placement3d().object_address() == old.object_address() {
                selector.set_value(&new.cast())?;
                return Ok(true);
            }
            Ok(false)
        }
    };
}

reducer!(
    /// Cartesian point reducer.
    StepTidyCartesianPointReducer, StepGeomCartesianPoint, StepTidyCartesianPointHasher, [
        StepGeomAxis1Placement => field_replacer!(StepGeomAxis1Placement, location, set_location),
        StepGeomAxis2Placement3d => field_replacer!(StepGeomAxis2Placement3d, location, set_location),
        StepShapeVertexPoint => field_replacer!(StepShapeVertexPoint, vertex_geometry, set_vertex_geometry),
        StepShapeGeometricCurveSet => point_in_geometric_set,
        StepVisualPresentationLayerAssignment => in_layer_assignment,
        StepVisualStyledItem => in_styled_item,
        StepGeomBSplineCurveWithKnots => list_replacer!(StepGeomBSplineCurveWithKnots),
        StepGeomLine => field_replacer!(StepGeomLine, pnt, set_pnt),
        StepGeomBSplineSurfaceWithKnots => net_replacer!(StepGeomBSplineSurfaceWithKnots),
        StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve => list_replacer!(StepGeomBSplineCurveWithKnotsAndRationalBSplineCurve),
        StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface => net_replacer!(StepGeomBSplineSurfaceWithKnotsAndRationalBSplineSurface),
        StepReprRepresentation => items_replacer!(StepReprRepresentation),
    ]
);

/// `replaceAxis2Placement3d` of a direction: the axis, else the reference direction.
fn direction_in_placement(
    old: &Handle<StepGeomDirection>,
    new: &Handle<StepGeomDirection>,
    sharing: &Handle<StandardTransient>,
) -> Result<bool, Failure> {
    let typed: Handle<StepGeomAxis2Placement3d> = sharing.cast();
    let live = step_live(&typed, WHERE)?;
    if live.axis().object_address() == old.object_address() {
        live.set_axis(new);
        return Ok(true);
    }
    if live.ref_direction().object_address() == old.object_address() {
        live.set_ref_direction(new);
        return Ok(true);
    }
    Ok(false)
}

reducer!(
    /// Direction reducer.
    StepTidyDirectionReducer, StepGeomDirection, StepTidyDirectionHasher, [
        StepGeomAxis1Placement => field_replacer!(StepGeomAxis1Placement, axis, set_axis),
        StepGeomAxis2Placement3d => direction_in_placement,
        StepGeomVector => field_replacer!(StepGeomVector, orientation, set_orientation),
    ]
);

/// `replaceItemDefinedTransformation`: either item, or both.
fn placement_in_transformation(
    old: &Handle<StepGeomAxis2Placement3d>,
    new: &Handle<StepGeomAxis2Placement3d>,
    sharing: &Handle<StandardTransient>,
) -> Result<bool, Failure> {
    let typed: Handle<StepReprItemDefinedTransformation> = sharing.cast();
    let live = step_live(&typed, WHERE)?;
    let mut replaced = false;
    if live.transform_item1().object_address() == old.object_address() {
        live.set_transform_item1(&new.cast());
        replaced = true;
    }
    if live.transform_item2().object_address() == old.object_address() {
        live.set_transform_item2(&new.cast());
        replaced = true;
    }
    Ok(replaced)
}

reducer!(
    /// Axis2 placement3d reducer.
    StepTidyAxis2Placement3dReducer, StepGeomAxis2Placement3d, StepTidyAxis2Placement3dHasher, [
        StepGeomPlane => field_replacer!(StepGeomPlane, position, set_position),
        StepReprItemDefinedTransformation => placement_in_transformation,
        StepGeomCylindricalSurface => field_replacer!(StepGeomCylindricalSurface, position, set_position),
        StepShapeShapeRepresentation => items_replacer!(StepShapeShapeRepresentation),
        StepReprConstructiveGeometryRepresentation => items_replacer!(StepReprConstructiveGeometryRepresentation),
        StepGeomCircle => conic_position_replacer!(StepGeomCircle),
        StepVisualPresentationLayerAssignment => in_layer_assignment,
        StepVisualStyledItem => in_styled_item,
        StepGeomEllipse => conic_position_replacer!(StepGeomEllipse),
        StepGeomConicalSurface => field_replacer!(StepGeomConicalSurface, position, set_position),
        StepGeomToroidalSurface => field_replacer!(StepGeomToroidalSurface, position, set_position),
        StepShapeAdvancedBrepShapeRepresentation => items_replacer!(StepShapeAdvancedBrepShapeRepresentation),
        StepGeomSphericalSurface => field_replacer!(StepGeomSphericalSurface, position, set_position),
    ]
);

reducer!(
    /// Vector reducer.
    StepTidyVectorReducer, StepGeomVector, StepTidyVectorHasher, [
        StepGeomLine => field_replacer!(StepGeomLine, dir, set_dir),
    ]
);

reducer!(
    /// Line reducer.
    StepTidyLineReducer, StepGeomLine, StepTidyLineHasher, [
        StepShapeEdgeCurve => field_replacer!(StepShapeEdgeCurve, edge_geometry, set_edge_geometry),
        StepGeomTrimmedCurve => field_replacer!(StepGeomTrimmedCurve, basis_curve, set_basis_curve),
        StepGeomSurfaceCurve => field_replacer!(StepGeomSurfaceCurve, curve3d, set_curve3d),
        StepReprDefinitionalRepresentation => items_replacer!(StepReprDefinitionalRepresentation),
        StepGeomSeamCurve => field_replacer!(StepGeomSeamCurve, curve3d, set_curve3d),
    ]
);

reducer!(
    /// Plane reducer.
    StepTidyPlaneReducer, StepGeomPlane, StepTidyPlaneHasher, [
        StepShapeAdvancedFace => field_replacer!(StepShapeAdvancedFace, face_geometry, set_face_geometry),
        StepGeomPcurve => field_replacer!(StepGeomPcurve, basis_surface, set_basis_surface),
    ]
);

reducer!(
    /// Circle reducer.
    StepTidyCircleReducer, StepGeomCircle, StepTidyCircleHasher, [
        StepShapeEdgeCurve => field_replacer!(StepShapeEdgeCurve, edge_geometry, set_edge_geometry),
        StepGeomSurfaceCurve => field_replacer!(StepGeomSurfaceCurve, curve3d, set_curve3d),
        StepGeomSeamCurve => field_replacer!(StepGeomSeamCurve, curve3d, set_curve3d),
    ]
);
