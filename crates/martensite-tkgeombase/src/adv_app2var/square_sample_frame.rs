//! Initialized caller sample storage with indexed parameter/result pointer origins.
use super::square_roots::SquareRootBindings;
use super::{AdvApp2VarEvaluationBuffers, AdvApp2VarSquareRootAxis};
use martensite_tkernel::standard::Failure;
/// One operation-local initialized allocation for square parameters and samples.
/// Equal starts preserve pointer identity; `AdvApp2VarSquareSampleFrame::new(vec![7.;64],0,0)?`.
pub struct AdvApp2VarSquareSampleFrame {
    values: Vec<f64>,
    parameter_start: usize,
    sample_start: usize,
    roots: SquareRootBindings,
    tables: super::square_tables::SquareTableBindings,
    intervals: super::square_intervals::SquareIntervalBindings,
}
impl AdvApp2VarSquareSampleFrame {
    /// Own initialized slots with two checked pointer origins;
    /// `AdvApp2VarSquareSampleFrame::new(vec![0.;64],0,0)?` shares both origins.
    pub fn new(
        values: Vec<f64>,
        parameter_start: usize,
        sample_start: usize,
    ) -> Result<Self, Failure> {
        for (role, start) in [("parameter", parameter_start), ("sample", sample_start)] {
            if start > values.len() {
                return Err(Failure::with_message(
                    "Standard_OutOfRange",
                    &format!(
                        "AdvApp2VarSquareSampleFrame: {role} start={start}; expected origin in 0..={}",
                        values.len()
                    ),
                ));
            }
        }
        Ok(Self {
            values,
            parameter_start,
            sample_start,
            roots: SquareRootBindings::default(),
            tables: super::square_tables::SquareTableBindings::default(),
            intervals: super::square_intervals::SquareIntervalBindings::default(),
        })
    }
    /// Bind two initialized caller interval slots in the same owner;
    /// `let samples=samples.with_interval_origin(super::AdvApp2VarSquareIntervalAxis::U,32)?`.
    pub fn with_interval_origin(
        mut self,
        axis: super::AdvApp2VarSquareIntervalAxis,
        start: usize,
    ) -> Result<Self, Failure> {
        if start
            .checked_add(2)
            .is_none_or(|end| end > self.values.len())
        {
            return Err(Failure::with_message(
                "Standard_OutOfRange",
                &format!(
                    "AdvApp2VarSquareSampleFrame: {axis:?} interval origin={start}; expected two initialized slots inside0..{}",
                    self.values.len()
                ),
            ));
        }
        self.intervals.bind(axis, start);
        Ok(self)
    }
    /// Inspect a caller interval independently of callback-local interval copies;
    /// `samples.interval(super::AdvApp2VarSquareIntervalAxis::U,0)?`.
    pub fn interval(
        &self,
        axis: super::AdvApp2VarSquareIntervalAxis,
        index: usize,
    ) -> Result<f64, Failure> {
        Ok(self.values[self.intervals.slot(axis, index, self.values.len())?])
    }
    pub(in crate::adv_app2var) fn input_bindings(
        &self,
    ) -> super::square_intervals::SquareInputBindings {
        super::square_intervals::SquareInputBindings {
            roots: self.roots,
            intervals: self.intervals,
        }
    }
    pub(in crate::adv_app2var) fn require_unbound_intervals(&self) -> Result<(), Failure> {
        if !self.intervals.is_bound() {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            "AdvApp2VarSquareSampleFrame: owned intervals configured=true; expected owned-table entrypoint instead of copied request intervals",
        ))
    }
    pub(in crate::adv_app2var) fn validate_interval_seed(
        &self,
        request: &super::approx_f2var::SampledSquare<'_>,
    ) -> Result<(), Failure> {
        for (axis, expected) in [
            (super::AdvApp2VarSquareIntervalAxis::U, request.uintfn),
            (super::AdvApp2VarSquareIntervalAxis::V, request.vintfn),
        ] {
            if self.intervals.origin(axis).is_none() {
                continue;
            }
            let actual = [self.interval(axis, 0)?, self.interval(axis, 1)?];
            if actual.map(f64::to_bits) != expected.map(f64::to_bits) {
                return Err(Failure::with_message(
                    "Standard_DomainError",
                    &format!(
                        "AdvApp2VarSquareSampleFrame: {axis:?} request interval={expected:?}; expected initialized caller interval={actual:?} bit-for-bit"
                    ),
                ));
            }
        }
        Ok(())
    }
    pub(in crate::adv_app2var) fn write_intervals(
        &self,
        request: &mut super::approx_f2var::SampledSquare<'_>,
    ) {
        for (axis, interval) in [
            (super::AdvApp2VarSquareIntervalAxis::U, &mut request.uintfn),
            (super::AdvApp2VarSquareIntervalAxis::V, &mut request.vintfn),
        ] {
            if self.intervals.origin(axis).is_none() {
                continue;
            }
            *interval = [
                self.interval(axis, 0)
                    .expect("AdvApp2VarSquareSampleFrame: validated interval first slot missing"),
                self.interval(axis, 1)
                    .expect("AdvApp2VarSquareSampleFrame: validated interval second slot missing"),
            ];
        }
    }
    /// Bind all four original caller table origins, allowing shared slots;
    /// `let samples=samples.with_table_origins([32;4])?`.
    pub fn with_table_origins(mut self, origins: [usize; 4]) -> Result<Self, Failure> {
        for start in origins {
            if start > self.values.len() {
                return Err(Failure::with_message(
                    "Standard_OutOfRange",
                    &format!(
                        "AdvApp2VarSquareSampleFrame: table start={start}; expected origin in 0..={}",
                        self.values.len()
                    ),
                ));
            }
        }
        self.tables = super::square_tables::SquareTableBindings::new(origins);
        Ok(self)
    }
    /// Inspect one actual caller table slot; `samples.table(super::AdvApp2VarSquareTable::SumSum,0)?`.
    pub fn table(&self, role: super::AdvApp2VarSquareTable, index: usize) -> Result<f64, Failure> {
        Ok(self.values[self.tables.slot(role, index, self.values.len())?])
    }
    /// Inspect a signed table offset inside the original initialized owner;
    /// `samples.table_signed(super::AdvApp2VarSquareTable::SumSum,-1)?` reads its prefix.
    pub fn table_signed(
        &self,
        role: super::AdvApp2VarSquareTable,
        offset: isize,
    ) -> Result<f64, Failure> {
        Ok(self.values[self.tables.signed_slot(role, offset, self.values.len())?])
    }
    pub(in crate::adv_app2var) fn table_bindings(
        &self,
    ) -> super::square_tables::SquareTableBindings {
        self.tables
    }
    pub(in crate::adv_app2var) fn require_unbound_tables(&self) -> Result<(), Failure> {
        if !self.tables.is_bound() {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            "AdvApp2VarSquareSampleFrame: owned tables configured=true; expected owned-table entrypoint instead of borrowed tables",
        ))
    }
    /// Bind an original caller root origin in this same initialized allocation;
    /// `let samples=samples.with_root_origin(AdvApp2VarSquareRootAxis::U,0)?`.
    pub fn with_root_origin(
        mut self,
        axis: AdvApp2VarSquareRootAxis,
        start: usize,
    ) -> Result<Self, Failure> {
        if start > self.values.len() {
            return Err(Failure::with_message(
                "Standard_OutOfRange",
                &format!(
                    "AdvApp2VarSquareSampleFrame: {axis:?} root start={start}; expected origin in 0..={}",
                    self.values.len()
                ),
            ));
        }
        self.roots.bind(axis, start);
        Ok(self)
    }
    /// Inspect an actual owned root after caller writes; `samples.root(AdvApp2VarSquareRootAxis::U,0)?`.
    pub fn root(&self, axis: AdvApp2VarSquareRootAxis, index: usize) -> Result<f64, Failure> {
        Ok(self.values[self.roots.slot(axis, index, self.values.len())?])
    }
    pub(in crate::adv_app2var) fn validate_owned_root_inputs(
        &self,
        request: &super::approx_f2var::SampledSquare<'_>,
    ) -> Result<(), Failure> {
        if !self.roots.has_roots() {
            return Err(Failure::with_message(
                "Standard_DomainError",
                "AdvApp2VarSquareSampleFrame: owned roots configured=false; expected at least one owned root origin",
            ));
        }
        for (axis, borrowed) in [
            (AdvApp2VarSquareRootAxis::U, request.urootb),
            (AdvApp2VarSquareRootAxis::V, request.vrootb),
        ] {
            if self.roots.origin(axis).is_some() && !borrowed.is_empty() {
                return Err(Failure::with_message(
                    "Standard_DomainError",
                    &format!(
                        "AdvApp2VarSquareSampleFrame: {axis:?} borrowed roots len={}; expected empty borrowed root input with owned root binding",
                        borrowed.len()
                    ),
                ));
            }
        }
        Ok(())
    }
    pub(in crate::adv_app2var) fn root_bindings(&self) -> SquareRootBindings {
        self.roots
    }
    pub(in crate::adv_app2var) fn require_unbound_roots(&self) -> Result<(), Failure> {
        if !self.root_bindings().has_roots() {
            return Ok(());
        }
        Err(Failure::with_message(
            "Standard_DomainError",
            "AdvApp2VarSquareSampleFrame: owned roots configured=true; expected owned-root entrypoint instead of borrowed root inputs",
        ))
    }
    /// Read a current caller parameter slot after return or unwind; `frame.parameter(0)?`.
    pub fn parameter(&self, index: usize) -> Result<f64, Failure> {
        self.slot_value(self.parameter_start, index, "parameter")
    }
    /// Read a current caller sample slot; `frame.sample(0)?`.
    pub fn sample(&self, index: usize) -> Result<f64, Failure> {
        self.slot_value(self.sample_start, index, "sample")
    }
    fn slot_value(&self, start: usize, index: usize, role: &str) -> Result<f64, Failure> {
        start.checked_add(index).and_then(|slot|self.values.get(slot)).copied()
            .ok_or_else(||Failure::with_message("Standard_OutOfRange",&format!("AdvApp2VarSquareSampleFrame: {role} index={index}; expected element in 0..{}",self.values.len()-start)))
    }
    pub(in crate::adv_app2var) fn buffers(&mut self) -> AdvApp2VarEvaluationBuffers<'_> {
        let capacity = self.values.len();
        AdvApp2VarEvaluationBuffers::shared(
            &mut self.values,
            self.parameter_start,
            capacity,
            self.sample_start,
            capacity,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interval_metadata_preserves_actual_slots_and_writes_both_caller_axes() {
        use super::super::AdvApp2VarSquareIntervalAxis::{U, V};
        let values = vec![1., 2., 3.];
        let pointer = values.as_ptr();
        let frame = AdvApp2VarSquareSampleFrame::new(values, 0, 0)
            .unwrap()
            .with_interval_origin(U, 0)
            .unwrap()
            .with_interval_origin(V, 1)
            .unwrap();
        assert_eq!(frame.values.as_ptr(), pointer);
        assert_eq!(
            [frame.interval(U, 0).unwrap(), frame.interval(U, 1).unwrap()],
            [1., 2.]
        );
        assert_eq!(
            [frame.interval(V, 0).unwrap(), frame.interval(V, 1).unwrap()],
            [2., 3.]
        );
        assert!(frame.require_unbound_intervals().is_err());
        let mut request = super::super::approx_f2var::SampledSquare {
            ndimen: 1,
            uintfn: [1., 2.],
            vintfn: [2., 3.],
            nbpntu: 0,
            nbpntv: 0,
            urootb: &[],
            vrootb: &[],
            isofav: 2,
        };
        frame.validate_interval_seed(&request).unwrap();
        request.uintfn = [0., 0.];
        request.vintfn = [0., 0.];
        frame.write_intervals(&mut request);
        assert_eq!(request.uintfn, [1., 2.]);
        assert_eq!(request.vintfn, [2., 3.]);
        println!(
            "{{\"metric\":\"owned_square_interval_metadata\",\"frame_bytes\":{},\"descriptor_bytes\":{},\"callback_view_bytes\":{},\"metadata_heap_allocations\":0,\"float_mutexes\":0}}",
            std::mem::size_of_val(&frame),
            std::mem::size_of::<super::super::square_intervals::SquareIntervalBindings>(),
            std::mem::size_of::<
                crate::adv_app2var::AdvApp2VarSquareBufferEvaluation<'static, 'static>,
            >()
        );
    }
    #[test]
    fn table_metadata_selects_distinct_roles_without_copying_the_owner() {
        use super::super::AdvApp2VarSquareTable as Table;
        let values = vec![1., 2., 3., 4.];
        let pointer = values.as_ptr();
        let frame = AdvApp2VarSquareSampleFrame::new(values, 0, 0)
            .unwrap()
            .with_table_origins([0, 1, 2, 3])
            .unwrap();
        assert_eq!(frame.values.as_ptr(), pointer);
        for (role, expected) in [
            (Table::SumSum, 1.),
            (Table::DifferenceSum, 2.),
            (Table::SumDifference, 3.),
            (Table::DifferenceDifference, 4.),
        ] {
            assert_eq!(frame.table(role, 0).unwrap(), expected);
        }
        assert!(frame.table(Table::DifferenceDifference, 1).is_err());
        assert!(frame.require_unbound_tables().is_err());
        println!(
            "{{\"metric\":\"owned_square_table_metadata\",\"frame_bytes\":{},\"descriptor_bytes\":{},\"callback_view_bytes\":{},\"metadata_heap_allocations\":0,\"float_mutexes\":0}}",
            std::mem::size_of_val(&frame),
            std::mem::size_of::<super::super::square_tables::SquareTableBindings>(),
            std::mem::size_of::<
                crate::adv_app2var::AdvApp2VarSquareBufferEvaluation<'static, 'static>,
            >()
        );
    }
    #[test]
    fn sample_frame_keeps_allocation_and_reports_metadata() {
        let values = vec![7.; 64];
        let pointer = values.as_ptr();
        let mut frame = AdvApp2VarSquareSampleFrame::new(values, 0, 0).unwrap();
        assert_eq!(frame.values.as_ptr(), pointer);
        frame.buffers().set_result(0, 0.25);
        assert_eq!(frame.parameter(0).unwrap(), 0.25);
        fn send_sync<T: Send + Sync>() {}
        send_sync::<AdvApp2VarSquareSampleFrame>();
        println!(
            "{{\"metric\":\"square_sample_owner\",\"inline_bytes\":{},\"payload_bytes\":{},\"indexed_buffer_bytes\":{},\"floating_owner_mutexes\":0}}",
            std::mem::size_of_val(&frame),
            frame.values.capacity() * std::mem::size_of::<f64>(),
            std::mem::size_of::<AdvApp2VarEvaluationBuffers<'_>>()
        );
    }
    #[test]
    fn root_origins_use_same_initialized_slots_and_reject_ambiguous_entrypoint() {
        use AdvApp2VarSquareRootAxis::{U, V};
        let mut frame = AdvApp2VarSquareSampleFrame::new(vec![0.25, 0.5, 0.75, 7.], 1, 2).unwrap();
        assert!(frame.root(U, 0).is_err());
        assert!(frame.require_unbound_roots().is_ok());
        frame = frame
            .with_root_origin(U, 0)
            .unwrap()
            .with_root_origin(V, 0)
            .unwrap();
        assert_eq!(
            (frame.root(U, 0).unwrap(), frame.root(V, 1).unwrap()),
            (0.25, 0.5)
        );
        assert!(frame.root(U, 4).is_err());
        assert!(frame.root(V, usize::MAX).is_err());
        assert!(frame.require_unbound_roots().is_err());
        assert!(frame.with_root_origin(U, 5).is_err());
        fn send_sync<T: Send + Sync>() {}
        send_sync::<AdvApp2VarSquareRootAxis>();
    }
    #[test]
    fn owned_root_metadata_keeps_one_allocation_and_records_descriptor_cost() {
        use AdvApp2VarSquareRootAxis::{U, V};
        let values = vec![7.; 129];
        let pointer = values.as_ptr();
        let frame = AdvApp2VarSquareSampleFrame::new(values, 1, 65)
            .unwrap()
            .with_root_origin(U, 0)
            .unwrap()
            .with_root_origin(V, 0)
            .unwrap();
        assert_eq!(frame.values.as_ptr(), pointer);
        assert!(!std::mem::needs_drop::<SquareRootBindings>());
        println!(
            "{{\"metric\":\"owned_square_root_metadata\",\"frame_bytes\":{},\"payload_capacity_bytes\":{},\"root_descriptor_bytes\":{},\"callback_view_bytes\":{},\"root_metadata_heap_allocations\":0}}",
            std::mem::size_of_val(&frame),
            frame.values.capacity() * std::mem::size_of::<f64>(),
            std::mem::size_of::<SquareRootBindings>(),
            std::mem::size_of::<
                crate::adv_app2var::AdvApp2VarSquareBufferEvaluation<'static, 'static>,
            >()
        );
    }
}

#[cfg(test)]
#[path = "square_signed_table_tests.rs"]
mod signed_table_tests;
