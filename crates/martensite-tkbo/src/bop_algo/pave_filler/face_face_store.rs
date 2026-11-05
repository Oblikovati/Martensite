//! Serial FF result storage keeps preparation and precision-transform ordering explicit.
use super::progress_observation::native_user_break;
use super::{BOPAlgoPaveFiller, face_face_task::BOPAlgoFaceFaceTask, missing_structure};
use crate::bop_algo::{Options, ParallelAlgo, ProgressCancellation};
use crate::bop_ds::InterfFF;
use crate::int_tools::{Curve, PntOn2Faces, Tools};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkernel::message::ProgressScope;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::bnd::BoundingBox3;

type PreparedFaceGeometry = (bool, Vec<Curve>, Vec<PntOn2Faces>);

impl BOPAlgoPaveFiller {
    #[cfg(test)]
    /// Stores one already-computed task in the existing serial FF result order.
    /// Example: `filler.store_face_face_task(&mut task)?` after task execution.
    pub(super) fn store_face_face_task(
        &mut self,
        task: &mut BOPAlgoFaceFaceTask<'static>,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::default();
        let observed = ProgressCancellation::default();
        self.store_face_task_with_progress(task, &scope, &observed)
            .map(|_| ())
    }

    /// Publishes the native mutable record before curve validation; false is an actual stop.
    /// Example: `filler.store_face_task_with_progress(task, scope, observed)?`.
    pub(super) fn store_face_task_with_progress(
        &mut self,
        task: &mut BOPAlgoFaceFaceTask<'static>,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        if !task.intersector().is_done() || task.parallel_options().has_errors() {
            self.store_failed_face_pair(task)?;
            return Ok(true);
        }
        let geometry = prepared_face_geometry(task)?;
        let record =
            self.initial_face_record(task.indices(), geometry.0, &geometry.1, &geometry.2)?;
        let index = self.publish_face_record(record)?;
        self.complete_face_record(index, task, &geometry, scope, observed)
    }

    fn publish_face_record(&mut self, record: InterfFF) -> Result<usize, Failure> {
        let records = self.structure_mut()?.interf_ff();
        let index = records.len();
        records.append(record);
        Ok(index)
    }

    fn complete_face_record(
        &mut self,
        index: usize,
        task: &BOPAlgoFaceFaceTask<'static>,
        geometry: &PreparedFaceGeometry,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<bool, Failure> {
        let options = &self.options;
        let structure = self.data_structure.as_mut().ok_or_else(missing_structure)?;
        let record = structure
            .interf_ff()
            .change_value(index)
            .map_err(|error| error.into_failure("PaveFiller::complete_face_record"))?;
        if !append_section_curves(record, &geometry.1, task, options, scope, observed)? {
            return Ok(false);
        }
        append_section_points(record, &geometry.2);
        Ok(true)
    }

    fn store_failed_face_pair(
        &mut self,
        task: &BOPAlgoFaceFaceTask<'static>,
    ) -> Result<(), Failure> {
        let [first, second] = task.indices();
        self.append_empty_face_face_record(first, second, None)?;
        let faces = (
            task.face1().clone().into_shape(),
            task.face2().clone().into_shape(),
        );
        self.add_intersection_failed_warning(&faces.0, &faces.1)
    }

    fn initial_face_record(
        &mut self,
        indices: [i32; 2],
        tangent: bool,
        curves: &[Curve],
        points: &[PntOn2Faces],
    ) -> Result<InterfFF, Failure> {
        if !curves.is_empty() || !points.is_empty() {
            self.structure_mut()?
                .add_interference(indices[0], indices[1]);
        }
        let mut record = InterfFF::default();
        record.set_indices(indices[0], indices[1]);
        record.set_tangent_faces(tangent);
        record.init(curves.len() as i32, points.len() as i32);
        Ok(record)
    }
}

fn prepared_face_geometry(
    task: &mut BOPAlgoFaceFaceTask<'static>,
) -> Result<PreparedFaceGeometry, Failure> {
    let tangent = task.intersector().tangent_faces();
    task.intersector_mut().prepare_lines_3d(false)?;
    task.apply_trsf()?;
    let curves: Vec<Curve> = task.intersector().lines()?.to_vec();
    let points: Vec<PntOn2Faces> = task.intersector().points().to_vec();
    Ok((tangent, curves, points))
}

/// Valid section curves with their boxes enlarged by the face pair tolerance plus the
/// maximal vertex tolerance of both faces.
fn append_section_curves(
    record: &mut InterfFF,
    curves: &[Curve],
    task: &BOPAlgoFaceFaceTask<'static>,
    options: &Options,
    scope: &ProgressScope<'_>,
    observed: &ProgressCancellation,
) -> Result<bool, Failure> {
    let tolerance = task.tol_ff();
    let expansion = section_curve_expansion(task, !curves.is_empty())?;
    for curve in curves {
        if native_user_break(options, scope, observed) {
            return Ok(false);
        }
        append_valid_section_curve(record, curve, tolerance, expansion)?;
    }
    Ok(true)
}

fn section_curve_expansion(
    task: &BOPAlgoFaceFaceTask<'static>,
    has_curves: bool,
) -> Result<f64, Failure> {
    let tolerance = task.tol_ff();
    let mut expansion = tolerance;
    if has_curves {
        let first = BRepTool::max_tolerance(task.face1(), ShapeEnum::Vertex)?;
        let second = BRepTool::max_tolerance(task.face2(), ShapeEnum::Vertex)?;
        expansion += first.max(second);
    }
    Ok(expansion)
}

fn append_valid_section_curve(
    record: &mut InterfFF,
    curve: &Curve,
    tolerance: f64,
    expansion: f64,
) -> Result<(), Failure> {
    let mut bounds = BoundingBox3::default();
    if !Tools::check_curve(curve, &mut bounds)? {
        return Ok(());
    }
    bounds.enlarge(expansion);
    let stored = record.change_curves().appended();
    stored.set_curve(curve);
    stored.set_box(&bounds);
    stored.set_tolerance(curve.tolerance().max(tolerance));
    Ok(())
}

fn append_section_points(record: &mut InterfFF, points: &[PntOn2Faces]) {
    for point in points {
        record.change_points().appended().set_pnt(*point.p1().pnt());
    }
}
