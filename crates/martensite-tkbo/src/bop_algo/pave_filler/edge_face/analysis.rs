//! Common-part checkpoints stop the entire EF stage before its post-treatment tail.
use super::progress::EdgeFaceBoundary;
use super::*;

impl BOPAlgoPaveFiller {
    /// Retains completed partial mutations while distinguishing an actual cancellation stop.
    /// Example: `if !filler.analyze_edge_face_task(task, results, boundary)? { return Ok(None); }`.
    pub(super) fn analyze_edge_face_task(
        &mut self,
        task: &EdgeFaceTask,
        results: &mut EdgeFaceResults,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<bool, Failure> {
        if task.failed() {
            self.record_failed_ef_task(task)?;
            return Ok(true);
        }
        let tolerances = ef_task_tolerances(task)?;
        if task.parts.is_empty() {
            self.record_empty_ef_distance(task, tolerances);
            return Ok(true);
        }
        let bounds = self.ef_analysis_bounds(task, tolerances)?;
        self.analyze_ef_parts(task, &bounds, results, boundary)
    }

    fn record_failed_ef_task(&mut self, task: &EdgeFaceTask) -> Result<(), Failure> {
        let edge = task.edge.clone().into_shape();
        let face = task.face.clone().into_shape();
        self.add_intersection_failed_warning(&edge, &face)
    }

    fn record_empty_ef_distance(&mut self, task: &EdgeFaceTask, tolerances: (f64, f64)) {
        if task.minimal_distance < f64::MAX && task.minimal_distance > tolerances.0 + tolerances.1 {
            let range = task.block.lock().expect("pave-block lock").range();
            let distance = EdgeRangeDistance {
                first: range.0,
                last: range.1,
                distance: task.minimal_distance,
            };
            self.record_distance(Pair::new(task.edge_index, task.face_index), distance);
        }
    }

    fn ef_analysis_bounds(
        &mut self,
        task: &EdgeFaceTask,
        tolerances: (f64, f64),
    ) -> Result<EfBounds, Failure> {
        let (range, vertices, splittable) = ef_block_bounds(task);
        let sides = self.ef_analysis_sides(task, vertices, range)?;
        let line_plane = self.ef_line_plane(task)?;
        Ok(EfBounds {
            vertices,
            splittable,
            sides,
            line_plane,
            tolerances,
        })
    }

    fn ef_analysis_sides(
        &mut self,
        task: &EdgeFaceTask,
        vertices: (i32, i32),
        range: (f64, f64),
    ) -> Result<[Range; 2], Failure> {
        let (mut first, mut last) = task.new_shrunk_range.range();
        if task.parts[0].shape_type() == ShapeEnum::Vertex {
            self.reduce_intersection_range(
                vertices,
                task.edge_index,
                task.face_index,
                &mut first,
                &mut last,
            )?;
        }
        Ok([Range::new(range.0, first), Range::new(last, range.1)])
    }

    fn ef_line_plane(&self, task: &EdgeFaceTask) -> Result<bool, Failure> {
        Ok(
            BRepAdaptorCurve::with_edge(&task.edge)?.get_type()? == CurveType::Line
                && self
                    .context_ref()?
                    .surface_adaptor(&task.face)?
                    .read()
                    .get_type()?
                    == SurfaceType::Plane,
        )
    }

    fn analyze_ef_parts(
        &mut self,
        task: &EdgeFaceTask,
        bounds: &EfBounds,
        results: &mut EdgeFaceResults,
        boundary: &EdgeFaceBoundary<'_, '_>,
    ) -> Result<bool, Failure> {
        for part in &task.parts {
            // BOPAlgoPaveFiller_5 polls before every actual CommonPrt, not just its task.
            if boundary.cancelled(&self.options) {
                return Ok(false);
            }
            match part.shape_type() {
                ShapeEnum::Vertex => self.analyze_ef_vertex(task, part, bounds, results)?,
                ShapeEnum::Edge => self.analyze_ef_edge(task, part, bounds.vertices, results)?,
                _ => {}
            }
        }
        Ok(true)
    }
}

fn ef_block_bounds(task: &EdgeFaceTask) -> ((f64, f64), (i32, i32), bool) {
    let block = task.block.lock().expect("pave-block lock");
    (block.range(), block.indices(), block.is_splittable())
}

fn ef_task_tolerances(task: &EdgeFaceTask) -> Result<(f64, f64), Failure> {
    Ok((
        BRepTool::edge_tolerance(&task.edge)?,
        BRepTool::face_tolerance(&task.face)?,
    ))
}
