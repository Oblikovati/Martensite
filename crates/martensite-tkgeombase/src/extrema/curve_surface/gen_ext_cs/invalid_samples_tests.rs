//! A public generic search must refuse negative grid counts rather than panic (#535).
use super::sampling_contract_tests::{RecordingGridSurface, initialize};
use super::*;
use martensite_tkernel::{impl_transient, standard::Transient};

#[derive(Default)]
struct GenericGridCurve {}
impl_transient!(GenericGridCurve, "GenericGridCurve535", dyn Transient);

impl Adaptor3dCurve for GenericGridCurve {
    fn get_type(&self) -> Result<CurveType, Failure> {
        Ok(CurveType::OtherCurve)
    }

    fn is_closed(&self) -> Result<bool, Failure> {
        Ok(false)
    }

    fn is_periodic(&self) -> Result<bool, Failure> {
        Ok(false)
    }

    fn resolution(&self, resolution: f64) -> Result<f64, Failure> {
        Ok(resolution)
    }

    fn eval_d0(&self, parameter: f64) -> Result<Point3, Failure> {
        Ok(Point3::new(parameter, parameter * parameter, 3.))
    }
}

#[test]
fn public_generic_search_refuses_negative_u_or_v_counts() {
    for counts in [(-1, 4), (2, -1), (-1, -1)] {
        let surface = RecordingGridSurface::default();
        let curve = GenericGridCurve::default();
        let mut extrema = GenExtCS::new();
        initialize(&mut extrema, &surface, counts).unwrap();
        let failure = extrema
            .perform_range(&curve, 2, (0., 1.), 1e-9)
            .unwrap_err();
        assert_grid_refusal(&failure, counts);
        assert!(!extrema.is_done());
        assert_eq!((extrema.usample, extrema.vsample), counts);
    }
}

fn assert_grid_refusal(failure: &Failure, counts: (i32, i32)) {
    assert_eq!(failure.exception_type(), "Standard_OutOfRange");
    assert_eq!(
        failure.message(),
        format!(
            "Extrema_GenExtCS::Perform - grid samples ({}, {}); expected nonnegative U and V counts",
            counts.0, counts.1
        )
    );
}
