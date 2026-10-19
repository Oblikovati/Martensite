//! The 3D/2D abstraction of the GeomBndLib templates: `PointOps<TPoint>` and the
//! Shared traits express the bounding-box and adaptor concepts, implemented
//! once for (`Point3`, `BoundingBox3`, `Adaptor3dCurve`) and once for the 2D types.
use super::optimization::{CurveCoordinate, adjust_extr_curve};
use super::sampling::{compute_nb_samples, compute_nb_samples_2d};
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkmath::bnd::{BoundingBox2, BoundingBox3};
use martensite_tkmath::gp::{Point2, Point3};

/// A point of the sampled space (`Point3` or `Point2`) seen as coordinates.
pub trait SamplePoint: Copy {
    /// Number of coordinates (3 or 2).
    const DIM: usize;
    /// Coordinate `k`, 0-based; `point.coordinate(0)`.
    fn coordinate(&self, k: usize) -> f64;
    /// The point of the first `DIM` coordinates; `Point3::from_coordinates(&[1.0, 2.0, 3.0])`.
    fn from_coordinates(values: &[f64]) -> Self;
    /// `PointOps::Mid`: `Point3((P1.XYZ() + P2.XYZ()) / 2.0)`; `Point3::mid(a, b)`.
    fn mid(first: Self, second: Self) -> Self {
        let values: Vec<f64> = (0..Self::DIM)
            .map(|k| (first.coordinate(k) + second.coordinate(k)) / 2.0)
            .collect();
        Self::from_coordinates(&values)
    }
    /// `Distance`; `a.distance_to(b)`.
    fn distance_to(self, other: Self) -> f64;
}

impl SamplePoint for Point3 {
    const DIM: usize = 3;
    fn coordinate(&self, k: usize) -> f64 {
        self.coordinates()[k]
    }
    fn from_coordinates(values: &[f64]) -> Self {
        Point3::new(values[0], values[1], values[2])
    }
    fn distance_to(self, other: Self) -> f64 {
        self.distance(&other)
    }
}

impl SamplePoint for Point2 {
    const DIM: usize = 2;
    fn coordinate(&self, k: usize) -> f64 {
        if k == 0 { self.x() } else { self.y() }
    }
    fn from_coordinates(values: &[f64]) -> Self {
        Point2::new(values[0], values[1])
    }
    fn distance_to(self, other: Self) -> f64 {
        self.distance(other)
    }
}

/// A bounding box of the sampled space (`BoundingBox3` or `BoundingBox2`).
pub trait SampleBounds: Default {
    /// The point type added to the box.
    type Point: SamplePoint;
    /// `Add(point)`; `bounds.add_sample(p)`.
    fn add_sample(&mut self, point: Self::Point);
    /// `Enlarge(tol)`; `bounds.enlarge_by(1e-7)`.
    fn enlarge_by(&mut self, tolerance: f64);
    /// `IsVoid()`; `bounds.void()`.
    fn void(&self) -> bool;
    /// The structured `Get()`: `(min, max)` per coordinate, 0-based; `bounds.min_max()`.
    fn min_max(&self) -> (Vec<f64>, Vec<f64>);
    /// `Update(min..., max...)`; `bounds.update_min_max(&min, &max)`.
    fn update_min_max(&mut self, min: &[f64], max: &[f64]);
}

impl SampleBounds for BoundingBox3 {
    type Point = Point3;
    fn add_sample(&mut self, point: Point3) {
        self.add_point(point);
    }
    fn enlarge_by(&mut self, tolerance: f64) {
        self.enlarge(tolerance);
    }
    fn void(&self) -> bool {
        self.is_void()
    }
    fn min_max(&self) -> (Vec<f64>, Vec<f64>) {
        let l = self.limits();
        (
            vec![l.x_min, l.y_min, l.z_min],
            vec![l.x_max, l.y_max, l.z_max],
        )
    }
    fn update_min_max(&mut self, min: &[f64], max: &[f64]) {
        self.update_bounds([min[0], min[1], min[2]], [max[0], max[1], max[2]]);
    }
}

impl SampleBounds for BoundingBox2 {
    type Point = Point2;
    fn add_sample(&mut self, point: Point2) {
        self.add_point(point);
    }
    fn enlarge_by(&mut self, tolerance: f64) {
        self.enlarge(tolerance);
    }
    fn void(&self) -> bool {
        self.is_void()
    }
    fn min_max(&self) -> (Vec<f64>, Vec<f64>) {
        let l = self.limits();
        (vec![l.x_min, l.y_min], vec![l.x_max, l.y_max])
    }
    fn update_min_max(&mut self, min: &[f64], max: &[f64]) {
        self.update_bounds([min[0], min[1]], [max[0], max[1]]);
    }
}

/// A curve adaptor sampled by the curve-bounds sampling (`Adaptor3dCurve` or
/// `Adaptor2dCurve2d`): its point type, box type and sampling rules.
pub trait SampledCurve: CurveCoordinate {
    /// The box of the curve's space.
    type Bounds: SampleBounds;
    /// `EvalD0(U)` (also what the non-virtual `D0` calls); `curve.sample_point(0.5)?`.
    fn sample_point(&self, u: f64) -> Result<<Self::Bounds as SampleBounds>::Point, Failure>;
    /// `ComputeNbSamplesT`; `curve.sample_count(0.0, 1.0)?`.
    fn sample_count(&self, u1: f64, u2: f64) -> Result<i32, Failure>;
    /// `AdjustExtrT`: the MathOpt PSO + Brent refinement of the curve's dimension;
    /// `curve.adjust_extremum(0.0, 1.0, 2.0, 1, 1e-7, true)?`.
    fn adjust_extremum(
        &self,
        range: (f64, f64),
        extr0: f64,
        index: i32,
        tolerance: f64,
        is_min: bool,
    ) -> Result<f64, Failure> {
        adjust_extr_curve(self, range.0, range.1, extr0, index, tolerance, is_min)
    }
}

impl SampledCurve for dyn Adaptor3dCurve + '_ {
    type Bounds = BoundingBox3;
    fn sample_point(&self, u: f64) -> Result<Point3, Failure> {
        self.eval_d0(u)
    }
    fn sample_count(&self, u1: f64, u2: f64) -> Result<i32, Failure> {
        compute_nb_samples(self, u1, u2)
    }
}

impl SampledCurve for dyn Adaptor2dCurve2d + '_ {
    type Bounds = BoundingBox2;
    fn sample_point(&self, u: f64) -> Result<Point2, Failure> {
        self.eval_d0(u)
    }
    fn sample_count(&self, u1: f64, u2: f64) -> Result<i32, Failure> {
        compute_nb_samples_2d(self, u1, u2)
    }
}
