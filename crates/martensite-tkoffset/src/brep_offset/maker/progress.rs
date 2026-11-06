//! Relative operation weights, normalized without changing their floating-point order.
use super::{BRepOffsetMakeOffset, PI_OPERATION_LAST, normalize_steps, statics};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::geom_abs::JoinType;

impl BRepOffsetMakeOffset {
    /// Allocate the whole progress budget to the eight operation stages.
    ///
    /// `maker.analyze_progress(100., &mut [0.; 8])?` also supports extension adapters.
    pub fn analyze_progress(
        &self,
        whole: f64,
        steps: &mut [f64; PI_OPERATION_LAST],
    ) -> Result<(), Failure> {
        steps.fill(0.);
        *steps = self.operation_progress_weights()?;
        normalize_steps(whole, steps);
        Ok(())
    }

    fn operation_progress_weights(&self) -> Result<[f64; PI_OPERATION_LAST], Failure> {
        let arc = self.my_join == JoinType::Arc;
        let planar = self.is_planar_progress_case(arc)?;
        let (build, intersection, shells) = match (planar, arc) {
            (true, _) => (70., 0., 25.),
            (false, true) => (20., 50., 5.),
            (false, false) => (50., 20., 5.),
        };
        let walls = if self.my_thickening { 5. } else { 0. };
        let sewing = if self.my_is_perform_sewing && self.my_thickening {
            10.
        } else {
            0.
        };
        // The original proportions intentionally need not add to one hundred.
        Ok([1., 2., build, intersection, walls, shells, 5., sewing])
    }

    fn is_planar_progress_case(&self, arc: bool) -> Result<bool, Failure> {
        Ok(self.my_inter
            && !arc
            && self.my_is_planar
            && !self.my_thickening
            && self.my_faces.is_empty()
            && statics::is_solid(&self.my_shape)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkbrep::topods::{Builder, Solid};

    #[test]
    fn planar_solid_full_intersection_weights_match_the_operation_budget() {
        let mut maker = BRepOffsetMakeOffset::new();
        let mut solid = Solid::default();
        Builder.make_solid(&mut solid);
        maker.my_shape = solid.into_shape();
        maker.my_join = JoinType::Intersection;
        maker.my_is_planar = true;
        maker.my_inter = true;
        let mut steps = [f64::NAN; 8];
        maker.analyze_progress(103., &mut steps).unwrap();
        assert_eq!(steps, [1., 2., 70., 0., 0., 25., 5., 0.]);
    }

    #[test]
    fn sewing_progress_requires_both_sewing_and_thickening() {
        let mut maker = BRepOffsetMakeOffset::new();
        maker.my_is_perform_sewing = true;
        let mut steps = [f64::NAN; 8];
        maker.analyze_progress(83., &mut steps).unwrap();
        assert_eq!(steps, [1., 2., 20., 50., 0., 5., 5., 0.]);
        maker.my_thickening = true;
        maker.analyze_progress(98., &mut steps).unwrap();
        assert_eq!(steps, [1., 2., 20., 50., 5., 5., 5., 10.]);
    }
}
