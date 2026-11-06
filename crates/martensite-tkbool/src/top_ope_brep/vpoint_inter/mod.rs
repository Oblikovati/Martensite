//! Intersection-point views borrow their source for the whole view lifetime.
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;
use martensite_tkgeomalgo::int_patch::IntPatchPoint;
use std::borrow::Cow;
mod accessors;
mod domain;
mod state;

/// A topological view of a surface-intersection point; `TopOpeBRepVPointInter::new()`.
///
/// The source is borrowed, rather than stored as an untracked pointer. A view cannot
/// outlive its source or mutate the source through a formerly const reference.
///
/// ```compile_fail
/// use martensite_tkbool::top_ope_brep::TopOpeBRepVPointInter;
/// use martensite_tkgeomalgo::int_patch::IntPatchPoint;
/// let mut view = TopOpeBRepVPointInter::new();
/// { let point = IntPatchPoint::new(); view.set_point(&point); }
/// view.value().unwrap();
/// ```
#[derive(Clone)]
pub struct TopOpeBRepVPointInter<'a> {
    point: Option<Cow<'a, IntPatchPoint>>,
    shape_index: i32,
    states: [State; 2],
    keep: bool,
    edges_on: [Shape; 2],
    edge_parameters: [f64; 2],
    index: i32,
    shapes: (i32, i32),
}

impl Default for TopOpeBRepVPointInter<'_> {
    fn default() -> Self {
        Self {
            point: None,
            shape_index: 0,
            states: [State::Unknown; 2],
            keep: false,
            edges_on: [Shape::default(), Shape::default()],
            edge_parameters: [0.; 2],
            index: 0,
            shapes: (0, 0),
        }
    }
}

impl<'a> TopOpeBRepVPointInter<'a> {
    /// Starts without a source and with UNKNOWN states; `Self::new().keep() == false`.
    pub fn new() -> Self {
        Self {
            states: [State::Unknown; 2],
            ..Self::default()
        }
    }

    /// Borrows a point and recomputes only its domain mask; `view.set_point(&point)`.
    pub fn set_point(&mut self, point: &'a IntPatchPoint) {
        self.point = Some(Cow::Borrowed(point));
        self.shape_index = i32::from(point.is_on_dom_s1()) + 2 * i32::from(point.is_on_dom_s2());
    }

    /// Owns a point copied from a guarded line; `view.set_owned_point(point)`.
    pub fn set_owned_point(&mut self, point: IntPatchPoint) {
        self.shape_index = i32::from(point.is_on_dom_s1()) + 2 * i32::from(point.is_on_dom_s2());
        self.point = Some(Cow::Owned(point));
    }

    /// Returns the safely borrowed source, or none before SetPoint; `view.point_of_intersection()`.
    pub fn point_of_intersection(&self) -> Option<&IntPatchPoint> {
        self.point.as_deref()
    }

    fn source(&self) -> Result<&IntPatchPoint, Failure> {
        self.point.as_deref().ok_or_else(|| {
            Failure::with_message(
                "Standard_NullObject",
                "TopOpeBRepVPointInter: source point=None; expected SetPoint before point access",
            )
        })
    }
}

fn domain_error(member: &str) -> Failure {
    Failure::with_message(
        "Standard_DomainError",
        &format!("TopOpeBRepVPointInter::{member}"),
    )
}

fn slot(index: i32, member: &str) -> Result<usize, Failure> {
    match index {
        1 => Ok(0),
        2 => Ok(1),
        _ => Err(domain_error(member)),
    }
}

fn source_domain_error(_: Failure) -> Failure {
    // The native IntPatch delegated domain accessors throw a message-less error.
    Failure::with_message("Standard_DomainError", "")
}
