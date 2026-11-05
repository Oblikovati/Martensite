//! Borrowed views of walking-line points with explicit initialization checks.
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;
use martensite_tkmath::gp::{Point2, Point3};

/// A walking-point view that cannot outlive its source; `TopOpeBRepWPointInter::new()`.
///
/// ```compile_fail
/// use martensite_tkbool::top_ope_brep::TopOpeBRepWPointInter;
/// use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;
/// let mut point = IntSurfPntOn2S::new();
/// let mut view = TopOpeBRepWPointInter::new();
/// view.set(&point);
/// point.set_parameters(1., 2., 3., 4.);
/// view.value().unwrap();
/// ```
#[derive(Default)]
pub struct TopOpeBRepWPointInter<'a> {
    point: Option<&'a IntSurfPntOn2S>,
}

impl<'a> TopOpeBRepWPointInter<'a> {
    /// Starts without a source instead of an indeterminate pointer; `Self::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Borrows the walking point; `view.set(&point)`.
    pub fn set(&mut self, point: &'a IntSurfPntOn2S) {
        self.point = Some(point);
    }
    /// Reads first-surface UV coordinates; `view.parameters_on_s1()?`.
    pub fn parameters_on_s1(&self) -> Result<(f64, f64), Failure> {
        Ok(self.source()?.parameters_on_surface(true))
    }
    /// Reads second-surface UV coordinates; `view.parameters_on_s2()?`.
    pub fn parameters_on_s2(&self) -> Result<(f64, f64), Failure> {
        Ok(self.source()?.parameters_on_surface(false))
    }
    /// Reads both UV pairs without reordering; `view.parameters()?`.
    pub fn parameters(&self) -> Result<(f64, f64, f64, f64), Failure> {
        Ok(self.source()?.parameters())
    }
    /// Returns first-surface UV as a point; `view.value_on_s1()?`.
    pub fn value_on_s1(&self) -> Result<Point2, Failure> {
        let (u, v) = self.parameters_on_s1()?;
        Ok(Point2::new(u, v))
    }
    /// Returns second-surface UV as a point; `view.value_on_s2()?`.
    pub fn value_on_s2(&self) -> Result<Point2, Failure> {
        let (u, v) = self.parameters_on_s2()?;
        Ok(Point2::new(u, v))
    }
    /// Reads the spatial point; `view.value()?`.
    pub fn value(&self) -> Result<Point3, Failure> {
        Ok(self.source()?.value())
    }
    /// Exposes the checked borrowed source without a mutable const-cast; `view.point_on_two_surfaces()`.
    pub fn point_on_two_surfaces(&self) -> Option<&'a IntSurfPntOn2S> {
        self.point
    }
    fn source(&self) -> Result<&IntSurfPntOn2S, Failure> {
        self.point.ok_or_else(|| {
            Failure::with_message(
                "Standard_NullObject",
                "TopOpeBRepWPointInter: source point=None; expected Set before point access",
            )
        })
    }
}
