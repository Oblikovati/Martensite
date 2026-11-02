use super::{FunctionSetWithDerivatives, Vector};
use crate::math::Function;
pub(super) struct Line<'a, F> {
    function: &'a mut F,
    origin: Vector<'static>,
    direction: Vector<'static>,
    point: &'a mut Vector<'static>,
    residual: &'a mut Vector<'static>,
    pub(super) work: &'a mut super::RootSearchInnerWork,
}
impl<'a, F: FunctionSetWithDerivatives> Line<'a, F> {
    pub(super) fn new(
        function: &'a mut F,
        origin: &Vector<'static>,
        direction: &Vector<'static>,
        point: &'a mut Vector<'static>,
        residual: &'a mut Vector<'static>,
        work: &'a mut super::RootSearchInnerWork,
    ) -> Self {
        Self {
            function,
            origin: origin.clone(),
            direction: direction.clone(),
            point,
            residual,
            work,
        }
    }
}
impl<F: FunctionSetWithDerivatives> Function for Line<'_, F> {
    fn value(&mut self, parameter: f64, value: &mut f64) -> bool {
        for index in 1..=self.point.length() {
            self.point[index] = self.direction[index] * parameter + self.origin[index];
        }
        self.work.evaluations += 1;
        if !self.function.value(self.point, self.residual) {
            self.work.evaluation_declined += 1;
            return false;
        }
        for index in 1..=self.residual.length() {
            if self.residual[index] <= -1e100 || self.residual[index] >= 1e100 {
                self.work.evaluation_declined += 1;
                return false;
            }
        }
        *value = 0.5 * self.residual.norm2_unchecked();
        true
    }
}
