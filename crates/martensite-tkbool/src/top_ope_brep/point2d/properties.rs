use super::{
    TopOpeBRepDSConfig, TopOpeBRepHctxee2d, TopOpeBRepHctxff2d, TopOpeBRepP2DStatus,
    TopOpeBRepPoint2d,
};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::{Point2, Point3};
impl TopOpeBRepPoint2d {
    /// Stores the is point of segment independently; `point.set_is_point_of_segment(value)`.
    pub fn set_is_point_of_segment(&mut self, value: bool) {
        self.point_of_segment = value;
    }
    /// Reads the stored is point of segment; `point.is_point_of_segment()`.
    pub fn is_point_of_segment(&self) -> bool {
        self.point_of_segment
    }
    /// Stores the status independently; `point.set_status(value)`.
    pub fn set_status(&mut self, value: TopOpeBRepP2DStatus) {
        self.status = value;
    }
    /// Reads the stored status; `point.status()`.
    pub fn status(&self) -> TopOpeBRepP2DStatus {
        self.status
    }
    /// Stores the index independently; `point.set_index(value)`.
    pub fn set_index(&mut self, value: i32) {
        self.index = value;
    }
    /// Reads the stored index; `point.index()`.
    pub fn index(&self) -> i32 {
        self.index
    }
    /// Stores the value independently; `point.set_value(value)`.
    pub fn set_value(&mut self, value: &Point3) {
        self.point = *value;
    }
    /// Reads the stored value; `point.value()`.
    pub fn value(&self) -> &Point3 {
        &self.point
    }
    /// Stores the value2d independently; `point.set_value2d(value)`.
    pub fn set_value2d(&mut self, value: &Point2) {
        self.point2d = *value;
    }
    /// Reads the stored value2d; `point.value2d()`.
    pub fn value2d(&self) -> &Point2 {
        &self.point2d
    }
    /// Stores the keep independently; `point.set_keep(value)`.
    pub fn set_keep(&mut self, value: bool) {
        self.keep = value;
    }
    /// Reads the stored keep; `point.keep()`.
    pub fn keep(&self) -> bool {
        self.keep
    }
    /// Stores the edges config independently; `point.set_edges_config(value)`.
    pub fn set_edges_config(&mut self, value: TopOpeBRepDSConfig) {
        self.config = value;
    }
    /// Reads the stored edges config; `point.edges_config()`.
    pub fn edges_config(&self) -> TopOpeBRepDSConfig {
        self.config
    }
    /// Stores the tolerance independently; `point.set_tolerance(value)`.
    pub fn set_tolerance(&mut self, value: f64) {
        self.tolerance = value;
    }
    /// Reads the stored tolerance; `point.tolerance()`.
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
    /// Stores the hctxff2d independently; `point.set_hctxff2d(value)`.
    pub fn set_hctxff2d(&mut self, value: &Handle<TopOpeBRepHctxff2d>) {
        self.context_ff = value.clone();
    }
    /// Reads the stored hctxff2d; `point.hctxff2d()`.
    pub fn hctxff2d(&self) -> Handle<TopOpeBRepHctxff2d> {
        self.context_ff.clone()
    }
    /// Stores the hctxee2d independently; `point.set_hctxee2d(value)`.
    pub fn set_hctxee2d(&mut self, value: &Handle<TopOpeBRepHctxee2d>) {
        self.context_ee = value.clone();
    }
    /// Reads the stored hctxee2d; `point.hctxee2d()`.
    pub fn hctxee2d(&self) -> Handle<TopOpeBRepHctxee2d> {
        self.context_ee.clone()
    }
}
