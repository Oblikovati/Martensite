//! Keep one basis read guard while copying the same owned admission input.
use super::{BSplineState, GeomBSplineCurve};
use crate::geom::surface_of_linear_extrusion::self_exclusion::{
    BasisAdmission, ClosedSplineExtrusionError,
};
impl GeomBSplineCurve {
    pub(in crate::geom) fn closed_extrusion_basis_capture(
        &self,
        direction: [f64; 3],
        interval: [f64; 2],
    ) -> Result<BasisAdmission, ClosedSplineExtrusionError> {
        let state = self.state.read();
        capture_leased_basis(&state, direction, interval)
    }
}
pub(super) fn capture_leased_basis(
    state: &BSplineState,
    direction: [f64; 3],
    interval: [f64; 2],
) -> Result<BasisAdmission, ClosedSplineExtrusionError> {
    super::stored_input::copy_stored_input(state).into_closed_extrusion_input(direction, interval)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::ClosedSplineExtrusionError as Error;
    use crate::geom::{GeomSurfaceOfLinearExtrusion, GeomSweptSurface};
    use martensite_tkernel::{ncollection::Array1, standard::Handle};
    use martensite_tkmath::{
        gp::{Direction3, Point3},
        predicates::roots::closed_extrusion::ClosedExtrusionPolicy,
    };
    #[test]
    fn actual_inconsistent_flat_storage_refuses_before_numerical_execution() {
        let surface = backtracking_surface();
        let basis = surface.basis_curve().cast::<GeomBSplineCurve>();
        basis
            .get()
            .unwrap()
            .state
            .write()
            .flat_knots
            .set_value(4, 0.75)
            .unwrap();
        let result = surface.closed_quadratic_self_exclusion(
            [-1., 1.],
            ClosedExtrusionPolicy::bounded(),
            || false,
        );
        assert!(matches!(result, Err(Error::FlatSequence { .. })));
    }
    fn backtracking_surface() -> GeomSurfaceOfLinearExtrusion {
        let points = [
            Point3::default(),
            Point3::new(1., 0., 0.),
            Point3::default(),
        ];
        let poles = Array1::from_shared(&points, 1);
        let knots = Array1::from_shared(&[0., 1.], 1);
        let mults = Array1::from_shared(&[3, 3], 1);
        let curve = GeomBSplineCurve::new(&poles, &knots, &mults, 2, false).unwrap();
        GeomSurfaceOfLinearExtrusion::new(
            &Handle::new(curve).cast(),
            Direction3::new(0., 0., 1.).unwrap(),
        )
        .unwrap()
    }
}
