//! Actual derived splitting criteria exercise native virtual dispatch.
use martensite_tkernel::{
    impl_transient,
    ncollection::HSequence,
    standard::{Failure, Handle},
};
use martensite_tkshhealing::shape_upgrade::{
    CurveSplit2dData, CurveSplit3dData, SplitCurve, SplitCurve2d, SplitCurve3d, SplitCurveData,
};
macro_rules! midpoint_criterion {
    ($name:ident,$data:ty, $interface:ident,$native:literal) => {
        #[derive(Default)]
        pub struct $name {geometry:$data}
        impl_transient!($name,$native,dyn $interface,interfaces:[dyn SplitCurve,dyn $interface]);
        impl $interface for $name {
            fn curve_geometry(&self) -> &$data { &self.geometry }
        }
        impl SplitCurve for $name {
            fn split_curve_data(&self)->&SplitCurveData {self.geometry.split_curve_data()}
            fn compute(&self)->Result<(),Failure> {
                self.split_curve_data().reset_status();
                let values=self.split_values();
                let (first,last)=values.sequence(|s|(*s.value(1).unwrap(),*s.value(s.len()).unwrap()));
                let middle=Handle::new(HSequence::default());
                middle.append((first+last)*0.5);
                self.set_split_values(&middle)?;
                self.split_curve_data().with_mut(|f|f.status=1);
                Ok(())
            }
            fn build(&self,segment:bool)->Result<(),Failure>{self.geometry.build_geometry(segment)}
        }
    };
}
midpoint_criterion!(
    Midpoint2d,
    CurveSplit2dData,
    SplitCurve2d,
    "Test_EdgeDivideMidpoint2d"
);
midpoint_criterion!(
    Midpoint3d,
    CurveSplit3dData,
    SplitCurve3d,
    "Test_EdgeDivideMidpoint3d"
);
