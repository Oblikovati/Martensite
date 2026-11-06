//! What the parts of `STEPCAFControlReader` that read GD&T and properties share.
use super::reader::WHERE;
use crate::step_basic::{
    StepBasicConversionBasedUnitAndMassUnit, StepBasicMeasureWithUnit, StepBasicNamedUnit,
    StepBasicSiUnit, StepBasicSiUnitName, StepBasicUnit,
};
use crate::step_construct::StepConstructUnitContext;
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::step_live;
use crate::step_repr::StepReprReprItemAndMeasureWithUnit;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkxsbase::interface::InterfaceHGraph;
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_control::{XSControlTransferReader, XSControlWorkSession};

/// The reader, the process and the graph of a session. The graph is taken for each query, so no
/// guard is held across what edits it.
pub(super) struct SessionGraph {
    pub(super) reader: Handle<XSControlTransferReader>,
    pub(super) tp: Handle<TransferTransientProcess>,
    graph: Handle<InterfaceHGraph>,
}

impl SessionGraph {
    /// The process and its graph (`aTP->Graph()`).
    pub(super) fn of_process(ws: &Handle<XSControlWorkSession>) -> Result<Self, Failure> {
        let reader = step_live(ws, WHERE)?.transfer_reader();
        let tp = step_live(&reader, WHERE)?.transient_process();
        let graph = step_live(&tp, WHERE)?.h_graph();
        Ok(Self { reader, tp, graph })
    }
    /// The process and the graph of the session (`WS->Graph()`).
    pub(super) fn of_session(ws: &Handle<XSControlWorkSession>) -> Result<Self, Failure> {
        let live = step_live(ws, WHERE)?;
        let reader = live.transfer_reader();
        let tp = step_live(&reader, WHERE)?.transient_process();
        let graph = live.graph()?;
        Ok(Self { reader, tp, graph })
    }
    /// `sharings(ent)`.
    pub(super) fn sharings(
        &self,
        entity: &Handle<StandardTransient>,
    ) -> Result<Vec<Handle<StandardTransient>>, Failure> {
        Ok(step_live(&self.graph, WHERE)?
            .graph()
            .sharings(entity)?
            .content())
    }
    /// `shareds(ent)`.
    pub(super) fn shareds(
        &self,
        entity: &Handle<StandardTransient>,
    ) -> Result<Vec<Handle<StandardTransient>>, Failure> {
        Ok(step_live(&self.graph, WHERE)?
            .graph()
            .shareds(entity)?
            .content())
    }
    /// `typed_sharings(ent, T)`.
    pub(super) fn typed_sharings<T: RuntimeTyped>(
        &self,
        entity: &Handle<StandardTransient>,
    ) -> Result<Vec<Handle<StandardTransient>>, Failure> {
        Ok(step_live(&self.graph, WHERE)?
            .graph()
            .typed_sharings(entity, T::static_type())?
            .content())
    }
    /// The live process.
    pub(super) fn process(&self) -> Result<&TransferTransientProcess, Failure> {
        step_live(&self.tp, WHERE)
    }
    /// The shape of the map item `index`, null when `index` is not one.
    pub(super) fn shape_at(&self, index: i32) -> Result<Shape, Failure> {
        if index <= 0 {
            return Ok(Shape::default());
        }
        Ok(TransferBRep::shape_result(
            &self.process()?.map_item(index)?,
        ))
    }
}

/// The first of the entities that is a `T`, null without one.
pub(super) fn first_of<T: ?Sized + RuntimeTyped>(
    entities: &[Handle<StandardTransient>],
) -> Handle<T> {
    entities
        .iter()
        .map(|entity| entity.cast::<T>())
        .find(Handle::is_some)
        .unwrap_or_default()
}

/// `GetMeasureWithUnit`: the measure of a transient that is one or a representation item with one;
/// null for others.
pub(super) fn measure_with_unit(
    measure: &Handle<StandardTransient>,
) -> Handle<StepBasicMeasureWithUnit> {
    let direct = measure.cast::<StepBasicMeasureWithUnit>();
    if direct.is_some() {
        return direct;
    }
    measure
        .cast::<StepReprReprItemAndMeasureWithUnit>()
        .get()
        .map(StepReprReprItemAndMeasureWithUnit::get_measure_with_unit)
        .unwrap_or_default()
}

/// The named unit of a measure's unit, `None` when it is null or not of the first case.
pub(super) fn named_unit_of(unit: &StepBasicUnit) -> Option<Handle<StepBasicNamedUnit>> {
    if unit.is_null() || unit.case_num(&unit.value()) != 1 {
        return None;
    }
    Some(unit.named_unit())
}

/// The context of a named unit.
pub(super) fn context_of_unit(
    named: &Handle<StepBasicNamedUnit>,
    factors: &StepDataFactors,
) -> Result<StepConstructUnitContext, Failure> {
    let mut context = StepConstructUnitContext::new();
    context.compute_unit_factors(named, factors)?;
    Ok(context)
}

/// `convertAngleValue`: radian to degree, only if the factor is not 1 with some precision, to avoid
/// inaccuracy of calculation.
pub(super) fn convert_angle_value(context: &StepConstructUnitContext, value: &mut f64) {
    let factor = context.plane_angle_factor() * 180. / std::f64::consts::PI;
    if (1. - factor).abs() > martensite_tkernel::precision::confusion() {
        *value *= factor;
    }
}

/// `GetMassConversionFactor`: the factor of a mass unit; false, with 1, for others.
pub(super) fn mass_conversion_factor(
    named: &Handle<StepBasicNamedUnit>,
    factor: &mut f64,
) -> Result<bool, Failure> {
    *factor = 1.;
    step_live(named, WHERE)?;
    let unit = named.cast::<StepBasicConversionBasedUnitAndMassUnit>();
    let Some(unit) = unit.get() else {
        return Ok(false);
    };
    let measure = measure_with_unit(&unit.conversion_factor());
    let measure = step_live(&measure, WHERE)?;
    *factor = measure.value_component();
    let second = measure.unit_component();
    if second.case_num(&second.value()) == 1 {
        let named = second.named_unit();
        step_live(&named, WHERE)?;
        if let Some(si_unit) = named.cast::<StepBasicSiUnit>().get()
            && si_unit.name() == StepBasicSiUnitName::SunGram
            && si_unit.has_prefix()
        {
            *factor *= StepConstructUnitContext::convert_si_prefix(si_unit.prefix());
        }
    }
    Ok(true)
}
