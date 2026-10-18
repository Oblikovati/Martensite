//! Caller intervals are read only at the source's inner sampling boundary.
use super::AdvApp2VarEvaluationBuffers;
use super::approx_f2var::SampledSquare;
use super::square_roots::SquareRootBindings;
use martensite_tkernel::standard::Failure;
/// An original caller interval axis; `AdvApp2VarSquareIntervalAxis::U`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvApp2VarSquareIntervalAxis {
    /// Caller U interval, independent of callback-local interval copies.
    U,
    /// Caller V interval.
    V,
}
impl AdvApp2VarSquareIntervalAxis {
    pub(super) fn index(self) -> usize {
        match self {
            Self::U => 0,
            Self::V => 1,
        }
    }
}
#[derive(Clone, Copy, Default)]
pub(super) struct SquareIntervalBindings {
    origins: [Option<usize>; 2],
}
impl SquareIntervalBindings {
    pub(super) fn bind(&mut self, axis: AdvApp2VarSquareIntervalAxis, start: usize) {
        self.origins[axis.index()] = Some(start);
    }
    pub(super) fn is_bound(self) -> bool {
        self.origins.iter().any(Option::is_some)
    }
    pub(super) fn origin(self, axis: AdvApp2VarSquareIntervalAxis) -> Option<usize> {
        self.origins[axis.index()]
    }
    pub(super) fn slot(
        self,
        axis: AdvApp2VarSquareIntervalAxis,
        index: usize,
        capacity: usize,
    ) -> Result<usize, Failure> {
        let start=self.origin(axis).ok_or_else(||Failure::with_message("Standard_DomainError",&format!("AdvApp2VarSquareSampleFrame: {axis:?} interval configured=false; expected owned interval origin")))?;
        if index >= 2 {
            return Err(Failure::with_message(
                "Standard_OutOfRange",
                &format!(
                    "AdvApp2VarSquareSampleFrame: {axis:?} interval index={index}; expected0..2"
                ),
            ));
        }
        start.checked_add(index).filter(|slot|*slot<capacity).ok_or_else(||Failure::with_message("Standard_OutOfRange",&format!("AdvApp2VarSquareSampleFrame: {axis:?} interval origin={start}; expected two initialized slots inside0..{capacity}")))
    }
    fn read(
        self,
        axis: AdvApp2VarSquareIntervalAxis,
        fallback: [f64; 2],
        buffers: &AdvApp2VarEvaluationBuffers<'_>,
    ) -> [f64; 2] {
        let Some(start) = self.origin(axis) else {
            return fallback;
        };
        [
            buffers.allocation_value(start).expect(
                "AdvApp2VarSquareSampleFrame: interval missing; expected initialized first slot",
            ),
            buffers.allocation_value(start + 1).expect(
                "AdvApp2VarSquareSampleFrame: interval missing; expected initialized second slot",
            ),
        ]
    }
    pub(super) fn capture<'roots>(
        self,
        mut geometry: SampledSquare<'roots>,
        buffers: &AdvApp2VarEvaluationBuffers<'_>,
    ) -> SampledSquare<'roots> {
        geometry.uintfn = self.read(AdvApp2VarSquareIntervalAxis::U, geometry.uintfn, buffers);
        geometry.vintfn = self.read(AdvApp2VarSquareIntervalAxis::V, geometry.vintfn, buffers);
        geometry
    }
}
#[derive(Clone, Copy, Default)]
pub(crate) struct SquareInputBindings {
    pub(crate) roots: SquareRootBindings,
    pub(super) intervals: SquareIntervalBindings,
}
