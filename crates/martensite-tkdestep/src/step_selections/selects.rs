//! `STEPSelectionsSelectAssembly`, `STEPSelectionsSelectFaces` and
//! `STEPSelectionsSelectGSCurves`: explorations that keep what a shape is made of.
use super::{entities_of, explore_selection, explore_selection_members};
use crate::step_entity::step_live;
use crate::step_geom::{
    StepGeomCompositeCurve, StepGeomCompositeCurveSegment, StepGeomCurve, StepGeomSurface,
};
use crate::step_repr::{
    StepReprMappedItem, StepReprNextAssemblyUsageOccurrence, StepReprProductDefinitionShape,
};
use crate::step_shape::{
    StepShapeContextDependentShapeRepresentation, StepShapeFaceSurface, StepShapeGeometricSet,
    StepShapeShapeDefinitionRepresentation, StepShapeShapeRepresentation,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkxsbase::if_select::{IFSelectSelectExplore, IFSelectSelectExploreBase};
use martensite_tkxsbase::interface::{InterfaceEntityIterator, InterfaceGraph};
use std::cell::Cell;

/// The tail of each `explore`: the shared entities are explored next.
fn explore_shareds(
    start: &Handle<StandardTransient>,
    g: &InterfaceGraph,
    explored: &mut InterfaceEntityIterator,
) -> Result<bool, Failure> {
    let shareds = entities_of(g.shareds(start)?)?;
    for shared in &shareds {
        explored.add_item(shared);
    }
    Ok(!shareds.is_empty())
}

/// Whether the product_definition_shape is of a next_assembly_usage_occurrence.
fn is_of_nauo(pds: &Handle<StepReprProductDefinitionShape>) -> bool {
    pds.get().is_some_and(|live| {
        live.definition()
            .product_definition_relationship()
            .cast::<StepReprNextAssemblyUsageOccurrence>()
            .is_some()
    })
}

explore_selection!(
    /// `STEPSelectionsSelectAssembly`: the components of assemblies.
    ///
    /// ```
    /// use martensite_tkdestep::step_selections::STEPSelectionsSelectAssembly;
    /// use martensite_tkxsbase::if_select::IFSelectSelection;
    /// let label = STEPSelectionsSelectAssembly::default().label();
    /// assert!(String::from_utf8_lossy(label.as_bytes()).contains("Assembly components"));
    /// ```
    STEPSelectionsSelectAssembly,
    "STEPSelectionsSelectAssembly"
);
explore_selection_members!(STEPSelectionsSelectAssembly);

impl IFSelectSelectExplore for STEPSelectionsSelectAssembly {
    fn explore_base(&self) -> &IFSelectSelectExploreBase {
        &self.base
    }
    fn explore(
        &self,
        _level: i32,
        start: &Handle<StandardTransient>,
        g: &InterfaceGraph,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<bool, Failure> {
        if start.is_null() {
            return Ok(false);
        }
        let cdsr: Handle<StepShapeContextDependentShapeRepresentation> = start.cast();
        if let Some(live) = cdsr.get() {
            return Ok(is_of_nauo(&live.represented_product_relation()));
        }
        if start.cast::<StepReprMappedItem>().is_some() {
            let rep = entities_of(g.sharings(start)?)?
                .into_iter()
                .find(|ent| ent.cast::<StepShapeShapeRepresentation>().is_some());
            let Some(rep) = rep else {
                return Ok(false);
            };
            let sdr: Option<Handle<StepShapeShapeDefinitionRepresentation>> =
                entities_of(g.sharings(&rep)?)?
                    .into_iter()
                    .find(|ent| {
                        ent.cast::<StepShapeShapeDefinitionRepresentation>()
                            .is_some()
                    })
                    .map(|ent| ent.cast());
            let Some(sdr) = sdr else {
                return Ok(false);
            };
            let definition = step_live(&sdr, "STEPSelectionsSelectAssembly")?
                .definition()
                .property_definition();
            return Ok(is_of_nauo(&definition.cast()));
        }
        explore_shareds(start, g, explored)
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new("Assembly components")
    }
}

explore_selection!(
    /// `STEPSelectionsSelectFaces`: the faces, and the surfaces outside faces.
    STEPSelectionsSelectFaces,
    "STEPSelectionsSelectFaces"
);
explore_selection_members!(STEPSelectionsSelectFaces);

impl IFSelectSelectExplore for STEPSelectionsSelectFaces {
    fn explore_base(&self) -> &IFSelectSelectExploreBase {
        &self.base
    }
    fn explore(
        &self,
        _level: i32,
        start: &Handle<StandardTransient>,
        g: &InterfaceGraph,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<bool, Failure> {
        if start.is_null() {
            return Ok(false);
        }
        if start.cast::<StepShapeFaceSurface>().is_some() {
            return Ok(true);
        }
        if start.cast::<StepGeomSurface>().is_some() {
            let mut in_face_or_surface = false;
            for sharing in entities_of(g.sharings(start)?)? {
                if sharing.cast::<StepShapeGeometricSet>().is_some() {
                    return Ok(true);
                }
                if sharing.cast::<StepGeomSurface>().is_some()
                    || sharing.cast::<StepShapeFaceSurface>().is_some()
                {
                    in_face_or_surface = true;
                }
            }
            return Ok(!in_face_or_surface);
        }
        explore_shareds(start, g, explored)
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new("Faces")
    }
}

thread_local! {
    /// `flag`: a thread-local of the file, set by every constructor and cleared by the
    /// first composite curve found in a geometric set.
    static GS_CURVES_FLAG: Cell<i32> = const { Cell::new(0) };
}

/// `STEPSelectionsSelectGSCurves`: the curves in geometric sets.
pub struct STEPSelectionsSelectGSCurves {
    transient: StandardTransient,
    base: IFSelectSelectExploreBase,
}
martensite_tkernel::impl_transient!(STEPSelectionsSelectGSCurves, "STEPSelectionsSelectGSCurves", dyn IFSelectSelectExplore, transient, interfaces: [dyn martensite_tkxsbase::if_select::IFSelectSelection, dyn martensite_tkxsbase::if_select::IFSelectSelectDeduct, dyn IFSelectSelectExplore]);
explore_selection_members!(STEPSelectionsSelectGSCurves);

impl Default for STEPSelectionsSelectGSCurves {
    fn default() -> Self {
        GS_CURVES_FLAG.with(|flag| flag.set(1));
        Self {
            transient: StandardTransient::default(),
            base: IFSelectSelectExploreBase::new(-1),
        }
    }
}

impl martensite_tkxsbase::if_select::IFSelectSelectDeduct for STEPSelectionsSelectGSCurves {
    fn deduct_base(&self) -> &martensite_tkxsbase::if_select::IFSelectSelectDeductBase {
        &self.base.deduct
    }
}

impl IFSelectSelectExplore for STEPSelectionsSelectGSCurves {
    fn explore_base(&self) -> &IFSelectSelectExploreBase {
        &self.base
    }
    fn explore(
        &self,
        _level: i32,
        start: &Handle<StandardTransient>,
        g: &InterfaceGraph,
        explored: &mut InterfaceEntityIterator,
    ) -> Result<bool, Failure> {
        if start.is_null() {
            return Ok(false);
        }
        if start.cast::<StepGeomCurve>().is_some() {
            if start.cast::<StepGeomCompositeCurve>().is_some() {
                return explore_composite_curve(start, g, explored);
            }
            for sharing in entities_of(g.sharings(start)?)? {
                if sharing.cast::<StepShapeGeometricSet>().is_some()
                    || sharing.cast::<StepGeomCompositeCurveSegment>().is_some()
                {
                    return Ok(true);
                }
            }
        }
        explore_shareds(start, g, explored)
    }
    fn explore_label(&self) -> AsciiString {
        AsciiString::new("Curves in GS")
    }
}

/// A composite curve counts when a geometric set holds it.
fn explore_composite_curve(
    start: &Handle<StandardTransient>,
    g: &InterfaceGraph,
    explored: &mut InterfaceEntityIterator,
) -> Result<bool, Failure> {
    let set = entities_of(g.sharings(start)?)?
        .into_iter()
        .find(|ent| ent.cast::<StepShapeGeometricSet>().is_some());
    let Some(set) = set else {
        return Ok(false);
    };
    if GS_CURVES_FLAG.with(Cell::get) != 0 {
        explored.add_item(&set);
        GS_CURVES_FLAG.with(|flag| flag.set(0));
    }
    explore_shareds(start, g, explored)
}
