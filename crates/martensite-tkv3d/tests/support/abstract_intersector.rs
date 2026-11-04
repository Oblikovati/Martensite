//! Shared named-fixture guard: abstract geometry must never be called by base tests.
macro_rules! reject_abstract_geometry {
    () => {
        fn build(&self) -> Result<(), martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn is_scalable(&self) -> bool {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn scale_and_transform(
            &self,
            _scale_factor: i32,
            _trsf: &martensite_tkmath::gp::GeneralTransform3,
            _builder: &martensite_tkernel::standard::Handle<
                martensite_tkv3d::select_mgr::SelectMgrFrustumBuilder,
            >,
        ) -> Result<
            martensite_tkernel::standard::Handle<
                dyn martensite_tkv3d::select_mgr::SelectMgrBaseIntersector,
            >,
            martensite_tkernel::standard::Failure,
        > {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn copy_with_builder(
            &self,
            _builder: &martensite_tkernel::standard::Handle<
                martensite_tkv3d::select_mgr::SelectMgrFrustumBuilder,
            >,
        ) -> Result<
            martensite_tkernel::standard::Handle<
                dyn martensite_tkv3d::select_mgr::SelectMgrBaseIntersector,
            >,
            martensite_tkernel::standard::Failure,
        > {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_box_pick(
            &self,
            _min: &martensite_tkernel::ncollection::Vec3<f64>,
            _max: &martensite_tkernel::ncollection::Vec3<f64>,
            _clip: &martensite_tkv3d::select_mgr::SelectMgrViewClipRange,
            _pick: &mut martensite_tkv3d::select_basics::SelectBasicsPickResult,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_box(
            &self,
            _min: &martensite_tkernel::ncollection::Vec3<f64>,
            _max: &martensite_tkernel::ncollection::Vec3<f64>,
            _inside: Option<&mut bool>,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_point_pick(
            &self,
            _point: &martensite_tkmath::gp::Point3,
            _clip: &martensite_tkv3d::select_mgr::SelectMgrViewClipRange,
            _pick: &mut martensite_tkv3d::select_basics::SelectBasicsPickResult,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_point(
            &self,
            _point: &martensite_tkmath::gp::Point3,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_polygon(
            &self,
            _points: &[martensite_tkmath::gp::Point3],
            _sens_type: i32,
            _clip: &martensite_tkv3d::select_mgr::SelectMgrViewClipRange,
            _pick: &mut martensite_tkv3d::select_basics::SelectBasicsPickResult,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_segment(
            &self,
            _p1: &martensite_tkmath::gp::Point3,
            _p2: &martensite_tkmath::gp::Point3,
            _clip: &martensite_tkv3d::select_mgr::SelectMgrViewClipRange,
            _pick: &mut martensite_tkv3d::select_basics::SelectBasicsPickResult,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_triangle(
            &self,
            _p1: &martensite_tkmath::gp::Point3,
            _p2: &martensite_tkmath::gp::Point3,
            _p3: &martensite_tkmath::gp::Point3,
            _sens_type: i32,
            _clip: &martensite_tkv3d::select_mgr::SelectMgrViewClipRange,
            _pick: &mut martensite_tkv3d::select_basics::SelectBasicsPickResult,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_sphere(
            &self,
            _center: &martensite_tkmath::gp::Point3,
            _radius: f64,
            _inside: Option<&mut bool>,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_sphere_pick(
            &self,
            _center: &martensite_tkmath::gp::Point3,
            _radius: f64,
            _clip: &martensite_tkv3d::select_mgr::SelectMgrViewClipRange,
            _pick: &mut martensite_tkv3d::select_basics::SelectBasicsPickResult,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_cylinder_pick(
            &self,
            _bottom_rad: f64,
            _top_rad: f64,
            _height: f64,
            _trsf: &martensite_tkmath::gp::Transform3,
            _is_hollow: bool,
            _clip: &martensite_tkv3d::select_mgr::SelectMgrViewClipRange,
            _pick: &mut martensite_tkv3d::select_basics::SelectBasicsPickResult,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_cylinder(
            &self,
            _bottom_rad: f64,
            _top_rad: f64,
            _height: f64,
            _trsf: &martensite_tkmath::gp::Transform3,
            _is_hollow: bool,
            _inside: Option<&mut bool>,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_circle_pick(
            &self,
            _radius: f64,
            _trsf: &martensite_tkmath::gp::Transform3,
            _is_filled: bool,
            _clip: &martensite_tkv3d::select_mgr::SelectMgrViewClipRange,
            _pick: &mut martensite_tkv3d::select_basics::SelectBasicsPickResult,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
        fn overlaps_circle(
            &self,
            _radius: f64,
            _trsf: &martensite_tkmath::gp::Transform3,
            _is_filled: bool,
            _inside: Option<&mut bool>,
        ) -> Result<bool, martensite_tkernel::standard::Failure> {
            panic!("BaseDefaultsIntersector: unexpected abstract geometry call")
        }
    };
}
pub(crate) use reject_abstract_geometry;
