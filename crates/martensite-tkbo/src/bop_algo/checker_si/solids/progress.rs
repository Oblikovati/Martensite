//! Native collection, worker and storage checkpoints surround the actual solid predicates.
use super::vertex_work::{VertexSolidInput, execute_vertices};
use super::*;
use crate::bop_algo::{ProgressCancellation, builder_area::progress::observe_user_break};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
impl BOPAlgoCheckerSI {
    /// Classify actual vertex/solid inputs; `checker.perform_vz_on_progress(range, observed)?`.
    pub(in crate::bop_algo::checker_si) fn perform_vz_on_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let outer = ProgressScope::unnamed(range, 1., false);
        let expected = self.start_solid_pairs(ShapeEnum::Vertex)?;
        if expected == 0 {
            return Ok(());
        }
        self.structure_mut()?.interf_vz().set_increment(expected);
        let Some(inputs) = self.collect_vertex_inputs(&outer, observed)? else {
            return Ok(());
        };
        let scope = ProgressScope::with_literal(
            &outer.next(1.),
            "Performing Vertex-Solid intersection",
            inputs.len() as f64,
            false,
        );
        let outputs = execute_vertices(
            inputs,
            &scope,
            &self.filler.context_handle(),
            self.filler.options().run_parallel(),
            observed,
        )?;
        if observe_user_break(self.filler.options(), &outer, observed) {
            return Ok(());
        }
        for (vertex, solid, state) in outputs {
            if state == State::In {
                self.store_vertex_solid(vertex, solid)?;
            }
        }
        Ok(())
    }
    /// Retain Edge/Solid worker polls; `checker.perform_ez_on_progress(range, observed)?`.
    pub(in crate::bop_algo::checker_si) fn perform_ez_on_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.perform_pair_solids(ShapeEnum::Edge, range, observed)
    }
    /// Retain Face/Solid worker polls; `checker.perform_fz_on_progress(range, observed)?`.
    pub(in crate::bop_algo::checker_si) fn perform_fz_on_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.perform_pair_solids(ShapeEnum::Face, range, observed)
    }
    /// Retain Solid/Solid all-subshape predicates; `checker.perform_zz_on_progress(range, observed)?`.
    pub(in crate::bop_algo::checker_si) fn perform_zz_on_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.perform_pair_solids(ShapeEnum::Solid, range, observed)
    }
    fn perform_pair_solids(
        &mut self,
        kind: ShapeEnum,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let outer = ProgressScope::unnamed(range, 1., false);
        let expected = self.start_solid_pairs(kind)?;
        if expected == 0 {
            return Ok(());
        }
        let pairs = take_iterator_pairs(self.solid_iterator()?)?;
        let scope = ProgressScope::with_literal(
            &outer.next(1.),
            solid_label(kind),
            pairs.len() as f64,
            false,
        );
        let structure = self
            .filler
            .data_structure()
            .ok_or_else(super::super::missing_structure)?;
        let found = super::pair_work::execute_pairs(
            structure,
            pairs,
            kind == ShapeEnum::Solid,
            &scope,
            self.filler.options().run_parallel(),
            observed,
        )?;
        if observe_user_break(self.filler.options(), &outer, observed) {
            return Ok(());
        }
        self.store_pair_solids(kind, expected, found)
    }
    fn start_solid_pairs(&mut self, kind: ShapeEnum) -> Result<usize, Failure> {
        let iterator = self.solid_iterator()?;
        iterator.initialize(kind, ShapeEnum::Solid);
        let expected = iterator.expected_length();
        usize::try_from(expected).map_err(|_| {
            Failure::out_of_range(&format!(
                "CheckerSI expected_length={expected}; expected a nonnegative candidate count"
            ))
        })
    }
    fn solid_iterator(&mut self) -> Result<&mut crate::bop_ds::BOPDSIterator, Failure> {
        self.filler.iterator_mut().ok_or_else(|| {
            Failure::with_message(
                "Standard_NullObject",
                "CheckerSI iterator=None; expected an initialized self-interference iterator",
            )
        })
    }
    fn collect_vertex_inputs(
        &mut self,
        outer: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Option<Vec<VertexSolidInput>>, Failure> {
        let mut inputs = Vec::new();
        let mut fence = Map::<Pair>::new(1);
        while self.solid_iterator()?.more() {
            if observe_user_break(self.filler.options(), outer, observed) {
                return Ok(None);
            }
            let pair = self.solid_iterator()?.value()?;
            if let Some(input) = self.vertex_input(pair, &mut fence)? {
                inputs.push(input);
            }
            self.solid_iterator()?.next();
        }
        Ok(Some(inputs))
    }
    fn vertex_input(
        &self,
        (vertex, solid): (i32, i32),
        fence: &mut Map<Pair>,
    ) -> Result<Option<VertexSolidInput>, Failure> {
        let structure = self
            .filler
            .data_structure()
            .ok_or_else(super::super::missing_structure)?;
        if structure.has_interference_shape_subshapes(vertex, solid)? {
            return Ok(None);
        }
        let Some(task) = distinct_vertex_solid_task(structure, vertex, solid, fence) else {
            return Ok(None);
        };
        Ok(Some(VertexSolidInput {
            vertex_index: task.vertex,
            solid_index: task.solid,
            vertex: topods::vertex(structure.shape(task.classified)?)?,
            solid: topods::solid(structure.shape(solid)?)?,
        }))
    }
    fn store_vertex_solid(&mut self, vertex: i32, solid: i32) -> Result<(), Failure> {
        let structure = self.structure_mut()?;
        structure
            .interf_vz()
            .append(InterfVZ::default())
            .set_indices(vertex, solid);
        structure.add_interference(vertex, solid);
        Ok(())
    }
    fn store_pair_solids(
        &mut self,
        kind: ShapeEnum,
        expected: usize,
        found: Vec<(i32, i32)>,
    ) -> Result<(), Failure> {
        let structure = self.structure_mut()?;
        match kind {
            ShapeEnum::Edge => structure.interf_ez().set_increment(expected),
            ShapeEnum::Face => structure.interf_fz().set_increment(expected),
            _ => structure.interf_zz().set_increment(expected),
        }
        for (shape, solid) in found {
            if kind == ShapeEnum::Solid {
                structure
                    .interf_zz()
                    .append(InterfZZ::default())
                    .set_indices(shape, solid);
            } else {
                record_shape_solid(structure, kind, shape, solid);
            }
            structure.add_interference(shape, solid);
        }
        Ok(())
    }
}
fn solid_label(kind: ShapeEnum) -> &'static str {
    match kind {
        ShapeEnum::Edge => "Performing Edge-Solid intersection",
        ShapeEnum::Face => "Performing Face-Solid intersection",
        _ => "Performing Solid-Solid intersection",
    }
}
