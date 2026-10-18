//! Caller table addressing and numerical scratch capture have distinct lifetimes.
use super::curve::IsoApproxRequest;
#[derive(Clone, Copy)]
pub(super) struct CurveCallerLayout {
    pub(super) dimension: i32,
    pub(super) subspaces: i32,
    pub(super) root_count: i32,
    pub(super) continuity: i32,
    pub(super) coefficient_limit: i32,
}
impl CurveCallerLayout {
    pub(super) fn capture(request: &IsoApproxRequest<'_>) -> Self {
        Self {
            dimension: request.ndimen,
            subspaces: request.nbsesp,
            root_count: request.nbroot,
            continuity: request.iordre,
            coefficient_limit: request.ncflim,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn caller_layout_owns_five_scalar_values_without_borrowing_mutable_inputs() {
        let mut request = crate::adv_app2var::slice_callback_fixture::request();
        let layout = CurveCallerLayout::capture(&request);
        let before = (
            request.ndimen,
            request.nbsesp,
            request.nbroot,
            request.iordre,
            request.ncflim,
        );
        request.ndimen = 0;
        request.nbsesp = 0;
        request.nbroot = 0;
        request.iordre = 0;
        request.ncflim = 0;
        assert_eq!(
            [
                request.ndimen,
                request.nbsesp,
                request.nbroot,
                request.iordre,
                request.ncflim
            ],
            [0; 5]
        );
        assert_eq!(
            (
                layout.dimension,
                layout.subspaces,
                layout.root_count,
                layout.continuity,
                layout.coefficient_limit
            ),
            before
        );
        assert_eq!(
            std::mem::size_of::<CurveCallerLayout>(),
            5 * std::mem::size_of::<i32>()
        );
        fn assert_send_sync<Value: Send + Sync>() {}
        assert_send_sync::<CurveCallerLayout>();
    }
}
