//! Source integer reads and writes stay sequential even with an owned backend.
use super::sampling::SampledSquare;
use crate::adv_app2var::AdvApp2VarEvaluatorFunc2Var;
#[derive(Clone, Copy)]
pub(crate) enum SquareInteger {
    Dimension,
    URootCount,
    VRootCount,
    FavoriteIso,
    ErrorCode,
}
pub(crate) trait SquareIntegerStorage {
    fn integer(&self, field: SquareInteger) -> i32;
    fn set_integer(&mut self, field: SquareInteger, value: i32);
    fn root_counts(&self, transposed: bool) -> [i32; 2] {
        let counts = [
            self.integer(SquareInteger::URootCount),
            self.integer(SquareInteger::VRootCount),
        ];
        if transposed {
            [counts[1], counts[0]]
        } else {
            counts
        }
    }
    fn snapshot<'roots>(
        &self,
        geometry: SampledSquare<'roots>,
        transposed: bool,
    ) -> SampledSquare<'roots> {
        let [nbpntu, nbpntv] = self.root_counts(transposed);
        SampledSquare {
            ndimen: self.integer(SquareInteger::Dimension),
            nbpntu,
            nbpntv,
            ..geometry
        }
    }
}
pub(super) struct BorrowedSquareIntegers<'call, 'roots> {
    pub(super) request: &'call mut SampledSquare<'roots>,
    pub(super) error: &'call mut i32,
    pub(super) evaluator: &'call dyn AdvApp2VarEvaluatorFunc2Var,
}
impl SquareIntegerStorage for BorrowedSquareIntegers<'_, '_> {
    fn integer(&self, field: SquareInteger) -> i32 {
        match field {
            SquareInteger::Dimension => self.request.ndimen,
            SquareInteger::URootCount => self
                .evaluator
                .caller_square_root_counts()
                .map_or(self.request.nbpntu, |counts| counts[0]),
            SquareInteger::VRootCount => self
                .evaluator
                .caller_square_root_counts()
                .map_or(self.request.nbpntv, |counts| counts[1]),
            SquareInteger::FavoriteIso => self.request.isofav,
            SquareInteger::ErrorCode => *self.error,
        }
    }
    fn set_integer(&mut self, field: SquareInteger, value: i32) {
        match field {
            SquareInteger::Dimension => self.request.ndimen = value,
            SquareInteger::URootCount => self.request.nbpntu = value,
            SquareInteger::VRootCount => self.request.nbpntv = value,
            SquareInteger::FavoriteIso => self.request.isofav = value,
            SquareInteger::ErrorCode => *self.error = value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn borrowed_integer_backend_retains_every_source_field_and_axis_order() {
        let mut request = SampledSquare {
            ndimen: 3,
            uintfn: [0., 1.],
            vintfn: [0., 1.],
            nbpntu: 3,
            nbpntv: 3,
            urootb: &[0.; 3],
            vrootb: &[0.; 3],
            isofav: 2,
        };
        let evaluator =
            |_: &crate::adv_app2var::AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32| {};
        let mut code = -1;
        let geometry = request;
        let mut integers = BorrowedSquareIntegers {
            request: &mut request,
            error: &mut code,
            evaluator: &evaluator,
        };
        for (field, value) in [
            (SquareInteger::Dimension, 1),
            (SquareInteger::URootCount, 2),
            (SquareInteger::VRootCount, 4),
            (SquareInteger::FavoriteIso, 1),
            (SquareInteger::ErrorCode, 201),
        ] {
            integers.set_integer(field, value);
            assert_eq!(integers.integer(field), value);
        }
        assert_eq!(integers.root_counts(false), [2, 4]);
        assert_eq!(integers.root_counts(true), [4, 2]);
        let captured = integers.snapshot(geometry, true);
        assert_eq!(
            (captured.ndimen, captured.nbpntu, captured.nbpntv),
            (1, 4, 2)
        );
        assert_eq!(integers.request.isofav, 1);
        assert_eq!(*integers.error, 201);
    }
}
