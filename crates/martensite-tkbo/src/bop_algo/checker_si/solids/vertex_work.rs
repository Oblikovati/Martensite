//! Vertex/solid workers own their shapes and ranges; each executor slot owns its context.
use super::*;
use crate::bop_algo::{ProgressCancellation, run_launcher_context_tasks};
use martensite_tkbrep::topods::{Solid, Vertex};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    standard::Handle,
};
pub(super) struct VertexSolidInput {
    pub(super) vertex_index: i32,
    pub(super) solid_index: i32,
    pub(super) vertex: Vertex,
    pub(super) solid: Solid,
}
struct VertexSolidWork<'p> {
    input: VertexSolidInput,
    range: Option<ProgressRange<'p>>,
    state: State,
}
/// Join the actual context policy; `execute_vertices(inputs, scope, context, parallel, observed)?`.
pub(super) fn execute_vertices(
    inputs: Vec<VertexSolidInput>,
    scope: &ProgressScope<'_>,
    context: &Handle<Context>,
    parallel: bool,
    observed: &ProgressCancellation,
) -> Result<Vec<(i32, i32, State)>, Failure> {
    let mut work: Vec<_> = inputs
        .into_iter()
        .map(|input| VertexSolidWork {
            input,
            range: Some(scope.next(1.)),
            state: State::Unknown,
        })
        .collect();
    run_launcher_context_tasks(
        &mut work,
        context,
        parallel,
        || Handle::new(Context::default()),
        |item, context| item.perform(context, observed),
    )?;
    Ok(work
        .into_iter()
        .map(|item| (item.input.vertex_index, item.input.solid_index, item.state))
        .collect())
}
impl VertexSolidWork<'_> {
    fn perform(
        &mut self,
        context: &Handle<Context>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        let range = self.range.take().ok_or_else(|| {
            Failure::new(&format!(
                "vertex-solid pair=({}, {}) progress=None; expected one unused owned range",
                self.input.vertex_index, self.input.solid_index
            ))
        })?;
        let scope = ProgressScope::unnamed(&range, 1., false);
        if !scope.more() {
            observed.record_cancelled();
            return Ok(());
        }
        let context = context.get().ok_or_else(|| Failure::with_message("Standard_NullObject", "CheckerSI vertex-solid context=None; expected an initialized caller or fresh worker context"))?;
        self.state = classify_vertex_solid(&self.input.vertex, &self.input.solid, context)?;
        Ok(())
    }
}
