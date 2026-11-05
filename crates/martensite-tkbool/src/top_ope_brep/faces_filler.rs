//! Face-intersection data storage keeps the current pair's point search range.
use super::{
    TopOpeBRepFacesIntersector, TopOpeBRepLineInter, TopOpeBRepPointClassifier,
    TopOpeBRepPointGeomTool, TopOpeBRepTypeLineCurve, TopOpeBRepVPointInter,
    TopOpeBRepVPointInterClassifier,
};
use crate::top_ope_brep_ds::{
    ListOfInterference, TopOpeBRepDSCurve, TopOpeBRepDSDataStructure, TopOpeBRepDSHDataStructure,
    TopOpeBRepDSInterference, TopOpeBRepDSKind, TopOpeBRepDSPoint,
};
use crate::top_ope_brep_tool::TopOpeBRepToolShapeClassifier;
use martensite_tkbrep::topods::{self, Face, Shape};
use martensite_tkernel::precision;
use martensite_tkernel::standard::real::{real_first, real_last};
use martensite_tkernel::standard::{Failure, Handle};
use std::sync::RwLockWriteGuard;

/// Storage for one face-pair filling operation; `TopOpeBRepFacesFiller::new()`.
///
/// Keeps face classification, restriction processing, and shared current-line context
/// in one operation-local builder.
pub struct TopOpeBRepFacesFiller<'a> {
    hds: Handle<TopOpeBRepDSHDataStructure>,
    first_pair_point: i32,
    curve_index: i32,
    curve_interferences: ListOfInterference,
    point_classifier: TopOpeBRepPointClassifier,
    shape_classifier: Option<&'a mut TopOpeBRepToolShapeClassifier>,
    owned_shape_classifier: Option<Box<TopOpeBRepToolShapeClassifier>>,
    trace_faces: [i32; 2],
    faces: [Face; 2],
    intersector: Option<intersector_binding::IntersectorBinding<'a>>,
    current_kind: Option<TopOpeBRepTypeLineCurve>,
    bound_line: Option<line_binding::BoundLine<'a>>,
    line_ok: bool,
    line_inl: bool,
    line_is_on_edge: bool,
    last_point_on_zero: bool,
    kept_point_count: i32,
    face_tolerance: f64,
    kpart_context: super::TopOpeBRepKPartContext,
    degenerate_context: degenerate::DegenerateContext,
    restriction_edges: martensite_tkernel::ncollection::List<Shape>,
}

impl Default for TopOpeBRepFacesFiller<'_> {
    fn default() -> Self {
        Self {
            hds: Handle::default(),
            first_pair_point: 1,
            curve_index: 0,
            curve_interferences: ListOfInterference::default(),
            point_classifier: TopOpeBRepPointClassifier::new(),
            shape_classifier: None,
            owned_shape_classifier: None,
            trace_faces: [0, 0],
            faces: [Face::default(), Face::default()],
            intersector: None,
            current_kind: None,
            bound_line: None,
            line_ok: false,
            line_inl: false,
            line_is_on_edge: false,
            last_point_on_zero: false,
            kept_point_count: 0,
            face_tolerance: 1.0e-7,
            kpart_context: Default::default(),
            restriction_edges: Default::default(),
            degenerate_context: Default::default(),
        }
    }
}

impl<'a> TopOpeBRepFacesFiller<'a> {
    /// Encloses every vertex parameter, extending coincident periodic endpoints; `Self::lminmax(&mut line)?`.
    pub fn lminmax(line: &mut TopOpeBRepLineInter) -> Result<(f64, f64), Failure> {
        let mut minimum = real_last();
        let mut maximum = real_first();
        for index in 1..=line.nb_vpoint() {
            let parameter = line.vpoint(index)?.parameter_on_line()?;
            if parameter < minimum {
                minimum = parameter;
            }
            if maximum < parameter {
                maximum = parameter;
            }
        }
        if (minimum - maximum).abs() <= precision::p_confusion()
            && line.is_periodic()
            && line.nb_vpoint() >= 2
        {
            maximum = minimum + line.period()?;
        }
        Ok((minimum, maximum))
    }

    /// Starts without a data structure or external classifier; `Self::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds storage and starts the current face pair's point range; `filler.bind_storage(&hds)?`.
    pub fn bind_storage(
        &mut self,
        hds: &Handle<TopOpeBRepDSHDataStructure>,
    ) -> Result<(), Failure> {
        let storage = hds.get().ok_or_else(missing_storage)?;
        self.first_pair_point = storage.nb_points() + 1;
        self.hds = hds.clone();
        Ok(())
    }

    /// Returns the shared operation storage; `filler.hdata_structure()`.
    pub fn hdata_structure(&self) -> Handle<TopOpeBRepDSHDataStructure> {
        self.hds.clone()
    }

    /// Borrows the mutable data structure; `filler.change_data_structure()?.nb_points()`.
    pub fn change_data_structure(
        &self,
    ) -> Result<RwLockWriteGuard<'_, TopOpeBRepDSDataStructure>, Failure> {
        Ok(self.hds.get().ok_or_else(missing_storage)?.change_ds())
    }

    /// Returns the reusable point-classification cache; `filler.change_point_classifier().init()`.
    pub fn change_point_classifier(&mut self) -> &mut TopOpeBRepPointClassifier {
        &mut self.point_classifier
    }

    /// Borrows the externally supplied classifier; `filler.p_shape_classifier()`.
    pub fn p_shape_classifier(&self) -> Option<&TopOpeBRepToolShapeClassifier> {
        self.shape_classifier
            .as_deref()
            .or(self.owned_shape_classifier.as_deref())
    }

    /// Keeps a caller-owned classifier for the operation; `filler.set_p_shape_classifier(Some(&mut classifier))`.
    pub fn set_p_shape_classifier(
        &mut self,
        classifier: Option<&'a mut TopOpeBRepToolShapeClassifier>,
    ) {
        self.owned_shape_classifier = None;
        self.shape_classifier = classifier;
    }

    pub(super) fn ensure_shape_classifier(&mut self) {
        if self.shape_classifier.is_none() && self.owned_shape_classifier.is_none() {
            self.owned_shape_classifier = Some(Box::new(TopOpeBRepToolShapeClassifier::default()));
        }
    }

    pub(super) fn shape_classifier_mut(
        &mut self,
    ) -> Result<&mut TopOpeBRepToolShapeClassifier, Failure> {
        self.shape_classifier
            .as_deref_mut()
            .or(self.owned_shape_classifier.as_deref_mut())
            .ok_or_else(|| {
                Failure::with_message(
                    "Standard_NullObject",
                    "FacesFiller: classifier=None; expected a bound shape classifier",
                )
            })
    }

    /// Records both face trace indices verbatim; `filler.set_trace_index(4, 8)`.
    pub fn set_trace_index(&mut self, first: i32, second: i32) {
        self.trace_faces = [first, second];
    }

    /// Returns both trace indices; `assert_eq!(filler.get_trace_index(), [0, 0])`.
    pub fn get_trace_index(&self) -> [i32; 2] {
        self.trace_faces
    }

    /// Starts a fresh current-line curve and interference list; `filler.reset_dsc()`.
    pub fn reset_dsc(&mut self) {
        self.curve_index = 0;
        self.curve_interferences.clear();
    }

    /// Creates one curve lazily and stores each interference on it; `filler.store_curve_interference(&i)?`.
    pub fn store_curve_interference(
        &mut self,
        interference: &Handle<TopOpeBRepDSInterference>,
    ) -> Result<(), Failure> {
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let entry = interference.get().ok_or_else(missing_interference)?;
        if self.curve_index == 0 {
            self.curve_index = storage
                .change_ds()
                .add_curve(&mut TopOpeBRepDSCurve::default());
        }
        entry.set_support(self.curve_index);
        storage.store_interference_in(interference, &mut self.curve_interferences)
    }

    /// Finds a matching point among remaining interference geometries; `filler.get_geometry(&list, &mut position, &point)?`.
    pub fn get_geometry(
        &self,
        list: &ListOfInterference,
        position: &mut usize,
        point: &TopOpeBRepVPointInter<'_>,
    ) -> Result<Option<(i32, TopOpeBRepDSKind)>, Failure> {
        let point = TopOpeBRepPointGeomTool::from_vpoint(point)?;
        self.hds
            .get()
            .ok_or_else(missing_storage)?
            .get_geometry(list, position, &point)
    }

    /// Stores a domain vertex or a new geometric point; `filler.make_geometry(&point, 1)?`.
    pub fn make_geometry(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        shape_index: i32,
    ) -> Result<(TopOpeBRepDSKind, i32), Failure> {
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        if point.is_vertex(shape_index)? {
            let index = storage
                .change_ds()
                .add_shape_with_rank(&point.vertex(shape_index)?, shape_index);
            return Ok((TopOpeBRepDSKind::Vertex, index));
        }
        let index = storage
            .change_ds()
            .add_point(&TopOpeBRepPointGeomTool::from_vpoint(point)?);
        Ok((TopOpeBRepDSKind::Point, index))
    }

    /// Searches the current face pair using the VPoint's retained geometric tolerance; `filler.get_ff_geometry_vpoint(&point)?`.
    pub fn get_ff_geometry_vpoint(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
    ) -> Result<Option<(TopOpeBRepDSKind, i32)>, Failure> {
        self.get_ff_geometry_point(&TopOpeBRepPointGeomTool::from_vpoint(point)?)
    }

    /// Finds only points inserted for the current face pair; `filler.get_ff_geometry_point(&point)?`.
    pub fn get_ff_geometry_point(
        &self,
        point: &TopOpeBRepDSPoint,
    ) -> Result<Option<(TopOpeBRepDSKind, i32)>, Failure> {
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        let ds = storage.ds();
        for index in self.first_pair_point..=ds.nb_points() {
            if point.is_equal(ds.point(index)?) {
                return Ok(Some((TopOpeBRepDSKind::Point, index)));
            }
        }
        Ok(None)
    }
}

fn missing_storage() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "FacesFiller: storage=None; expected a bound HDataStructure",
    )
}

fn missing_interference() -> Failure {
    Failure::with_message(
        "Standard_NullObject",
        "FacesFiller: interference=None; expected a live interference",
    )
}

mod add_shapes;
mod classification;
mod fill_line;
mod restriction;

mod closing;
mod load_line;
mod position_line;
mod same_domain;
mod section_edges;

mod vpr;

mod degenerate;
mod fill_restriction;
mod orchestration;
mod restriction_line;

/// Point geometry discoveries returned by face-intersection vertex processing.
pub use vpr::GeometrySearch as TopOpeBRepFacesFillerGeometrySearch;

mod line_binding;
pub use line_binding::{
    TopOpeBRepFacesFillerLine, TopOpeBRepFacesFillerLineMut, TopOpeBRepFacesFillerTransition,
};

mod line_array;
pub use line_array::TopOpeBRepFacesFillerArrayMut;

mod intersector_binding;
