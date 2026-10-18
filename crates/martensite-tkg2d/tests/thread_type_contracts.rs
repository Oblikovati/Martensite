//! Compiler checks of structural auto traits, without claims about input isolation.
use martensite_tkg2d::{adaptor2d as adaptor, geom2d as geom, geom2d_adaptor as adapted};
use martensite_tkg2d::{geom2d_eval as eval, geom2d_grid_eval as grid, geom2d_hash as hash};
use martensite_tkmath::gp::{Point2, Point3, Xy, Xyz};

fn require_send_sync<T: ?Sized + Send + Sync>() {}

macro_rules! check_types {
    ($($kind:ty),+ $(,)?) => {
        $(let _: fn() = require_send_sync::<$kind>;)+
    };
}

#[test]
fn geometry_values_and_curve_objects_have_structural_thread_bounds() {
    check_types!(
        geom::Geom2dCurveD1, geom::Geom2dCurveD2, geom::Geom2dCurveD3,
        geom::Geom2dCartesianPoint, geom::Geom2dDirection, geom::Geom2dVectorWithMagnitude,
        geom::Geom2dAxisPlacement, geom::Geom2dTransformation, geom::Geom2dLine,
        geom::Geom2dCircle, geom::Geom2dEllipse, geom::Geom2dHyperbola, geom::Geom2dParabola,
        geom::Geom2dBezierCurve, geom::Geom2dBSplineCurve, geom::Geom2dTrimmedCurve,
        geom::Geom2dOffsetCurve, geom::protected::ProtectedAccess, geom::spline_data::KnotData,
        geom::bspline_curve_core::GivenWeights<'static, 'static>,
        geom::trim_bounds::TrimmedBounds<dyn geom::Geom2dCurve>,
    );
}

#[test]
fn public_geometry_interfaces_have_structural_thread_bounds() {
    check_types!(
        dyn geom::Geom2dGeometry, dyn geom::Geom2dCurve, dyn geom::Geom2dBoundedCurve,
        dyn geom::Geom2dConic, dyn geom::Geom2dPoint, dyn geom::Geom2dVector,
        dyn adaptor::Adaptor2dCurve2d, dyn eval::RepCurveDesc,
    );
}

#[test]
fn adaptors_and_their_public_snapshots_have_structural_thread_bounds() {
    check_types!(
        adaptor::Adaptor2dD3, adaptor::Adaptor2dCurve2dBase, adaptor::Adaptor2dLine2d,
        adaptor::Adaptor2dOffsetCurve, adapted::Geom2dAdaptorCurve,
        adapted::OffsetData, adapted::BezierData, adapted::BSplineData,
        adapted::CurveDataVariant, adapted::SharedBSplineCache,
        adapted::knot_continuity::KnotSpans<'static>,
        adapted::span_cache::CacheSource<'static, Point2>,
        adapted::span_cache::CacheSource<'static, Point3>,
    );
}

#[test]
fn evaluation_descriptors_and_curves_have_structural_thread_bounds() {
    check_types!(
        eval::Map1d, eval::Domain1d, eval::RepCurveKind, eval::RepCurveDescFull,
        eval::RepCurveDescDerivBounded, eval::RepCurveDescMapped,
        eval::MappedCurveState<dyn geom::Geom2dCurve>,
        eval::Geom2dEvalAHTBezierCurve, eval::Geom2dEvalTBezierCurve,
        eval::Geom2dEvalArchimedeanSpiralCurve, eval::Geom2dEvalCircleInvoluteCurve,
        eval::Geom2dEvalLogarithmicSpiralCurve, eval::Geom2dEvalSineWaveCurve,
    );
}

#[test]
fn numeric_basis_storage_has_structural_thread_bounds_in_both_dimensions() {
    check_types!(
        eval::aht_bezier_data::AhtBezierData<Point2>, eval::aht_bezier_data::AhtBezierData<Point3>,
        eval::tbezier_data::TBezierData<Point2>, eval::tbezier_data::TBezierData<Point3>,
        eval::eval_basis::aht::AhtView<'static, Point2>, eval::eval_basis::aht::AhtView<'static, Point3>,
        eval::eval_basis::tbezier::TBezierView<'static, Point2>,
        eval::eval_basis::tbezier::TBezierView<'static, Point3>,
        eval::eval_basis::sums::BasisSums<Xy>, eval::eval_basis::sums::BasisSums<Xyz>,
        eval::eval_basis::sums::BasisDerivatives,
        grid::bezier_arrays::BezierCurveArrays<Point2>,
        grid::bezier_arrays::BezierCurveArrays<Point3>,
    );
}

#[test]
fn grid_evaluators_and_results_have_structural_thread_bounds() {
    check_types!(
        grid::Geom2dGridEvalLine, grid::Geom2dGridEvalCircle, grid::Geom2dGridEvalEllipse,
        grid::Geom2dGridEvalHyperbola, grid::Geom2dGridEvalParabola,
        grid::Geom2dGridEvalBezierCurve, grid::Geom2dGridEvalBSplineCurve,
        grid::Geom2dGridEvalOffsetCurve, grid::Geom2dGridEvalOtherCurve<'static>,
        grid::Geom2dGridEvalCurve<'static>, grid::Geom2dGridEvalEvaluator<'static>,
        grid::Geom2dGridEvalCurveD1, grid::Geom2dGridEvalCurveD2, grid::Geom2dGridEvalCurveD3,
    );
}

#[test]
fn hashers_have_structural_thread_bounds() {
    check_types!(
        hash::Geom2dHashAxisPlacement, hash::Geom2dHashDirectionHasher, hash::Geom2dHashPointHasher,
        hash::Geom2dHashCircleHasher, hash::Geom2dHashEllipseHasher,
        hash::Geom2dHashHyperbolaHasher, hash::Geom2dHashLineHasher, hash::Geom2dHashParabolaHasher,
        hash::Geom2dHashBSplineCurveHasher, hash::Geom2dHashBezierCurveHasher,
        hash::Geom2dHashOffsetCurveHasher, hash::Geom2dHashTrimmedCurveHasher,
        hash::Geom2dHashCurveHasher,
    );
}
