//! The model the entity reducer and duplicate cleaner tests build on: a work session in the STEP
//! norm, with builders that add a named entity and what it references.
#![allow(
    dead_code,
    reason = "each test target that includes this fixture uses a part of it"
)]
use martensite_tkdestep::step_control::StepControlController;
use martensite_tkdestep::step_data::{StepDataSelectType, StepDataStepModel};
use martensite_tkdestep::step_geom::{
    StepGeomAxis2Placement, StepGeomAxis2Placement3d, StepGeomCartesianPoint, StepGeomCircle,
    StepGeomDirection, StepGeomLine, StepGeomPlane, StepGeomVector,
};
use martensite_tkdestep::step_tidy::StepTidyEntityReducer;
use martensite_tkernel::ncollection::{HArray1, HashPolicy};
use martensite_tkernel::standard::{Handle, RuntimeType, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::collections::HashSet;

pub const ORIGIN: [f64; 3] = [0.0, 0.0, 0.0];
pub const Y_AXIS: [f64; 3] = [0.0, 1.0, 0.0];
pub const Z_AXIS: [f64; 3] = [0.0, 0.0, 1.0];

pub fn name(text: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(text))
}

/// One entity of a one-item array of handles, as every array-holding sharing entity is built.
pub fn one_item<T: RuntimeTyped + Send + Sync + 'static>(
    item: Handle<T>,
) -> Handle<HArray1<Handle<T>>> {
    let items = HArray1::with_bounds(1, 1).expect("bounds");
    items.change_array1().set_value(1, item).expect("set");
    Handle::new(items)
}

/// The entities a reducer reported as replaced.
pub struct ReplacedEntities(Vec<Handle<StandardTransient>>);

impl ReplacedEntities {
    fn holds<T: RuntimeTyped>(&self, entity: &Handle<T>) -> bool {
        let address = entity.object_address();
        self.0.iter().any(|one| one.object_address() == address)
    }
    pub fn assert_empty(&self) {
        assert!(
            self.0.is_empty(),
            "expected nothing replaced, got {} entities",
            self.0.len()
        );
    }
    pub fn assert_one_of<T: RuntimeTyped>(&self, first: &Handle<T>, second: &Handle<T>) {
        assert_eq!(self.0.len(), 1);
        assert!(self.holds(first) || self.holds(second));
    }
}

pub struct Fixture {
    pub ws: Handle<XSControlWorkSession>,
}

impl Fixture {
    pub fn new() -> Self {
        StepControlController::init().expect("controller");
        let ws = XSControlWorkSession::new();
        ws.select_norm("STEP").expect("norm");
        let model = ws.norm_adaptor().new_model();
        ws.set_model(&model, true).expect("model");
        Self { ws }
    }
    pub fn model(&self) -> Handle<StepDataStepModel> {
        self.ws.model().cast()
    }
    /// `addToModel`: the entity and what it references.
    pub fn add<T: RuntimeTyped>(&self, entity: &Handle<T>) {
        let model = self.model();
        model
            .add_with_refs(&entity.cast(), 0, false)
            .expect("add to the model");
    }
    pub fn add_cartesian_point(&self, label: &str, at: [f64; 3]) -> Handle<StepGeomCartesianPoint> {
        let point = Handle::new(StepGeomCartesianPoint::new());
        point.init_3d(&name(label), at[0], at[1], at[2]);
        self.add(&point);
        point
    }
    pub fn add_direction(&self, label: &str, ratios: [f64; 3]) -> Handle<StepGeomDirection> {
        let values = HArray1::with_bounds(1, 3).expect("bounds");
        for (rank, value) in (1..).zip(ratios) {
            values.change_array1().set_value(rank, value).expect("set");
        }
        let direction = Handle::new(StepGeomDirection::new());
        direction
            .init(&name(label), &Handle::new(values))
            .expect("init");
        self.add(&direction);
        direction
    }
    pub fn add_vector(&self, label: &str) -> Handle<StepGeomVector> {
        let vector = Handle::new(StepGeomVector::new());
        vector.init(&name(label), &self.add_direction("", Z_AXIS), 1.0);
        self.add(&vector);
        vector
    }
    /// The placement at `location`, on the fixture's default axis and reference direction.
    pub fn add_placement_at(
        &self,
        label: &str,
        location: [f64; 3],
    ) -> Handle<StepGeomAxis2Placement3d> {
        let placement = Handle::new(StepGeomAxis2Placement3d::new());
        placement.init(
            &name(label),
            &self.add_cartesian_point("", location),
            true,
            &self.add_direction("", Z_AXIS),
            true,
            &self.add_direction("", Y_AXIS),
        );
        self.add(&placement);
        placement
    }
    pub fn add_placement(&self, label: &str) -> Handle<StepGeomAxis2Placement3d> {
        self.add_placement_at(label, ORIGIN)
    }
    pub fn add_line(&self, label: &str) -> Handle<StepGeomLine> {
        let line = Handle::new(StepGeomLine::new());
        line.init(
            &name(label),
            &self.add_cartesian_point("", ORIGIN),
            &self.add_vector(""),
        );
        self.add(&line);
        line
    }
    pub fn add_circle(&self, label: &str) -> Handle<StepGeomCircle> {
        let circle = Handle::new(StepGeomCircle::new());
        circle.init(
            &name(label),
            &placement_select(&self.add_placement("")),
            1.0,
        );
        self.add(&circle);
        circle
    }
    pub fn add_plane(&self, label: &str) -> Handle<StepGeomPlane> {
        let plane = Handle::new(StepGeomPlane::new());
        plane.init(&name(label), &self.add_placement(""));
        self.add(&plane);
        plane
    }
    /// The number of model entities of the kind.
    pub fn count_of_kind(&self, kind: &RuntimeType) -> i32 {
        let model = self.model();
        (1..=model.nb_entities())
            .filter(|rank| model.value(*rank).expect("value").is_kind(kind))
            .count() as i32
    }
    /// Every model entity through the reducer, then `perform`.
    pub fn replaced_by<T, H>(&self, reducer: &mut StepTidyEntityReducer<T, H>) -> ReplacedEntities
    where
        T: RuntimeTyped,
        H: HashPolicy<Handle<T>> + Default,
    {
        let model = self.model();
        for rank in 1..=model.nb_entities() {
            let entity = model.value(rank).expect("value");
            reducer.process_entity(&entity).expect("process");
        }
        let (mut replaced, mut seen) = (Vec::new(), HashSet::new());
        reducer.perform(&mut replaced, &mut seen).expect("perform");
        ReplacedEntities(replaced)
    }
}

/// `StepGeomAxis2Placement` holding the placement.
pub fn placement_select(placement: &Handle<StepGeomAxis2Placement3d>) -> StepGeomAxis2Placement {
    let mut select = StepGeomAxis2Placement::default();
    select.set_value(&placement.cast()).expect("a placement");
    select
}
