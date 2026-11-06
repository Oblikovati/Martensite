//! `createGDTObjectInXCAF` of `STEPCAFControlReader`: labels for dimensions and tolerances on the
//! shapes they refer to.
use super::reader::{ReaderState, WHERE};
use super::reader_gdt_common::{collect_shape_aspect, find_shape_index_for_dgt, read_datums};
use super::reader_support::{SessionGraph, context_of_unit, measure_with_unit, named_unit_of};
use crate::geom_to_step::shared_array1;
use crate::step_ap242::StepAP242GeometricItemSpecificUsage;
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_dim_tol::{
    StepDimTolAngularityTolerance, StepDimTolCircularRunoutTolerance,
    StepDimTolCoaxialityTolerance, StepDimTolConcentricityTolerance,
    StepDimTolCylindricityTolerance, StepDimTolFlatnessTolerance,
    StepDimTolGeoTolAndGeoTolWthDatRefAndModGeoTolAndPosTol, StepDimTolGeometricTolerance,
    StepDimTolGeometricToleranceWithDatumReference, StepDimTolLineProfileTolerance,
    StepDimTolModifiedGeometricTolerance, StepDimTolParallelismTolerance,
    StepDimTolPerpendicularityTolerance, StepDimTolPositionTolerance, StepDimTolRoundnessTolerance,
    StepDimTolStraightnessTolerance, StepDimTolSurfaceProfileTolerance,
    StepDimTolSymmetryTolerance, StepDimTolTotalRunoutTolerance,
};
use crate::step_entity::{step_array1_item, step_live};
use crate::step_repr::{
    StepReprAllAroundShapeAspect, StepReprProductDefinitionShape, StepReprPropertyDefinition,
    StepReprReprItemAndLengthMeasureWithUnit, StepReprRepresentationItem, StepReprShapeAspect,
    StepReprValueRange,
};
use crate::step_shape::{
    StepShapeAdvancedBrepShapeRepresentation, StepShapeAdvancedFace,
    StepShapeDimensionalCharacteristicRepresentation, StepShapeDimensionalLocation,
    StepShapeDimensionalSize, StepShapeEdgeCurve, StepShapeShapeDefinitionRepresentation,
};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::{AsciiString, ExtendedString, HAsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tklcaf::tdoc_std::TDocStdDocument;
use martensite_tkxcaf::xcaf_dim_tol_objects::XCAFDimTolObjectsGeomToleranceModif;
use martensite_tkxcaf::xcaf_doc::{
    XCAFDocDimTolTool, XCAFDocDimension, XCAFDocDocumentTool, XCAFDocGeomTolerance,
    XCAFDocShapeTool,
};
use martensite_tkxsbase::xs_control::XSControlWorkSession;

mod entities;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use entities::*;

/// The name of a GD&T entity, `None` for an entity that is invalid (protection against invalid input).
fn semantic_name_of(entity: &Handle<StandardTransient>) -> Option<Handle<HAsciiString>> {
    let mut name = Handle::default();
    if let Some(tolerance) = entity.cast::<StepDimTolGeometricTolerance>().get() {
        if tolerance.toleranced_shape_aspect().is_null() {
            return None;
        }
        name = tolerance.name();
    }
    if let Some(size) = entity.cast::<StepShapeDimensionalSize>().get() {
        if size.applies_to().is_null() {
            return None;
        }
        name = size.name();
    }
    if let Some(location) = entity.cast::<StepShapeDimensionalLocation>().get() {
        if location.related_shape_aspect().is_null() || location.relating_shape_aspect().is_null() {
            return None;
        }
        name = location.name();
    }
    Some(name)
}

/// The value of the rank of a value range, multiplied by the length factor when it is a length.
/// `None` is the `continue` of the search.
fn range_value(
    items: &Handle<martensite_tkernel::ncollection::HArray1<Handle<StepReprRepresentationItem>>>,
    rank: i32,
    value: &mut f64,
    factors: &StepDataFactors,
) -> Result<Option<()>, Failure> {
    let item = step_array1_item(items, rank, WHERE)?;
    if item.is_null() {
        return Ok(None);
    }
    let Some(length) = item
        .cast::<StepReprReprItemAndLengthMeasureWithUnit>()
        .get()
        .map(|live| live.get_measure_with_unit())
    else {
        return Ok(Some(()));
    };
    let measure = step_live(&length, WHERE)?;
    *value = measure.value_component();
    let Some(named) = named_unit_of(&measure.unit_component()) else {
        return Ok(None);
    };
    *value *= context_of_unit(&named, factors)?.length_factor();
    Ok(Some(()))
}

/// The bounds of a dimensional size, from its value ranges.
fn size_bounds(
    session: &SessionGraph,
    size: &Handle<StandardTransient>,
    factors: &StepDataFactors,
) -> Result<(Handle<HAsciiString>, f64, f64), Failure> {
    let (mut name, mut first, mut second) = (Handle::default(), -1., -1.);
    for sharing in session.sharings(size)? {
        let characteristic = sharing.cast::<StepShapeDimensionalCharacteristicRepresentation>();
        let Some(characteristic) = characteristic.get() else {
            continue;
        };
        let representation = characteristic.representation();
        let Some(representation) = representation.get().filter(|live| live.nb_items() > 0) else {
            continue;
        };
        let range = representation.items_value(1)?.cast::<StepReprValueRange>();
        let Some(range) = range.get() else {
            continue;
        };
        name = range.name();
        let items = range.item_element();
        let Some(live_items) = items.get() else {
            continue;
        };
        let length = live_items.array1().length();
        if length > 0 && range_value(&items, 1, &mut first, factors)?.is_none() {
            continue;
        }
        if length > 1 {
            range_value(&items, 2, &mut second, factors)?;
        }
    }
    Ok((name, first, second))
}

/// The kind number `set_dim_tol` takes for a tolerance with a datum reference.
fn kind_of_reference_tolerance(tolerance: &Handle<StandardTransient>) -> i32 {
    if tolerance.cast::<StepDimTolAngularityTolerance>().is_some() {
        24
    } else if tolerance
        .cast::<StepDimTolCircularRunoutTolerance>()
        .is_some()
    {
        25
    } else if tolerance.cast::<StepDimTolCoaxialityTolerance>().is_some() {
        26
    } else if tolerance
        .cast::<StepDimTolConcentricityTolerance>()
        .is_some()
    {
        27
    } else if tolerance.cast::<StepDimTolParallelismTolerance>().is_some() {
        28
    } else if tolerance
        .cast::<StepDimTolPerpendicularityTolerance>()
        .is_some()
    {
        29
    } else if tolerance.cast::<StepDimTolSymmetryTolerance>().is_some() {
        30
    } else if tolerance.cast::<StepDimTolTotalRunoutTolerance>().is_some() {
        31
    } else {
        0
    }
}

/// The kind number of a tolerance without a datum reference, `None` for another kind.
fn kind_of_plain_tolerance(tolerance: &Handle<StandardTransient>) -> Option<i32> {
    if tolerance
        .cast::<StepDimTolCylindricityTolerance>()
        .is_some()
    {
        Some(38)
    } else if tolerance.cast::<StepDimTolFlatnessTolerance>().is_some() {
        Some(39)
    } else if tolerance.cast::<StepDimTolLineProfileTolerance>().is_some() {
        Some(40)
    } else if tolerance.cast::<StepDimTolPositionTolerance>().is_some() {
        Some(41)
    } else if tolerance.cast::<StepDimTolRoundnessTolerance>().is_some() {
        Some(42)
    } else if tolerance
        .cast::<StepDimTolStraightnessTolerance>()
        .is_some()
    {
        Some(43)
    } else if tolerance
        .cast::<StepDimTolSurfaceProfileTolerance>()
        .is_some()
    {
        Some(44)
    } else {
        None
    }
}

/// What the old version of the structure needs, for one representation item.
struct OldVersion<'a> {
    session: &'a SessionGraph,
    shape_tool: &'a Handle<XCAFDocShapeTool>,
    dim_tol_tool: &'a XCAFDocDimTolTool,
    factors: &'a StepDataFactors,
}

impl OldVersion<'_> {
    /// The label of the shape an item made, `None` when not found.
    fn label_of_item(&self, item: &Handle<StandardTransient>) -> Result<Option<TdfLabel>, Failure> {
        let shape = self
            .session
            .shape_at(self.session.process()?.map_index(item))?;
        if shape.is_null() {
            return Ok(None);
        }
        let mut label = TdfLabel::default();
        let found =
            step_live(self.shape_tool, WHERE)?.search(&shape, &mut label, [true, true, true])?;
        Ok(found.then_some(label))
    }
    /// A dimensional size on an edge curve.
    fn read_dimension(
        &self,
        entity: &Handle<StandardTransient>,
        item: &Handle<StepReprRepresentationItem>,
    ) -> Result<(), Failure> {
        let edge = item.cast::<StepShapeEdgeCurve>();
        if edge.is_null() {
            return Ok(());
        }
        let (name, first, mut second) = size_bounds(self.session, entity, self.factors)?;
        if first < 0. {
            return Ok(());
        }
        if second < 0. {
            second = first;
        }
        // now we know edge_curve and value range therefore we can create corresponding D&GT labels
        let Some(label) = self.label_of_item(&edge.cast())? else {
            return Ok(());
        };
        let size = entity.cast::<StepShapeDimensionalSize>();
        self.dim_tol_tool.set_dim_tol_values(
            &label,
            1,
            &shared_array1(vec![first, second]),
            &name,
            &step_live(&size, WHERE)?.name(),
        )?;
        Ok(())
    }
}
