//! `BOPAlgoPaveFiller::PerformFF` (`BOPAlgoPaveFiller_6`): face/face intersection.
use super::face_face_task::{BOPAlgoFaceFaceTask, FaceFaceTaskInput, FaceFaceTaskOutput};
use super::face_face_tasks::{execute_face_inputs_in_scope, face_intersection_scope};
use super::progress_observation::native_user_break;
use super::{BOPAlgoPaveFiller, Glue, missing_structure};
use crate::bop_algo::{ProgressCancellation, ProgressCompletion};
use crate::bop_ds::{InterfFF, Pair};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{Edge, Face, ShapeIterator};
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::{DataMap, Map};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::geom_abs::SurfaceType;

#[path = "face_face_seam.rs"]
mod seam;

#[cfg(test)]
#[path = "face_face_scope_lifetime_contract.rs"]
mod scope_lifetime_contract;

impl BOPAlgoPaveFiller {
    /// Refreshes the face records of all touched faces and intersects every face pair with
    /// interfering boxes, storing the section curves and points (`PerformFF`).
    /// `filler.perform_ff()?`.
    pub fn perform_ff(&mut self) -> Result<(), Failure> {
        let range = ProgressRange::default();
        let observed = ProgressCancellation::default();
        self.perform_ff_with_progress(&range, &observed).map(|_| ())
    }

    /// Runs the actual FF tasks with lexical child ranges and the same ordered storage.
    /// Example: `filler.perform_ff_with_progress(range, observed)?`.
    pub(super) fn perform_ff_with_progress(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let pairs = self.iterator_pairs(ShapeEnum::Face, ShapeEnum::Face)?;
        self.update_touched_face_info(&pairs)?;
        if pairs.is_empty() {
            return Ok(Vec::new());
        }
        let scope = ProgressScope::unnamed(range, 1.0, false);
        self.run_face_intersections(&scope, observed)
    }

    fn run_face_intersections(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Vec<ProgressCompletion>, Failure> {
        let Some(inputs) = self.prepare_face_tasks(scope, observed)? else {
            return Ok(Vec::new());
        };
        // BOPAlgoPaveFiller_6 keeps even a zero-task scope active through storage.
        let tasks_scope = face_intersection_scope(scope, inputs.len());
        let outputs =
            execute_face_inputs_in_scope(inputs, &tasks_scope, self.options.run_parallel())?;
        let (tasks, completions) = unpack_face_outputs(outputs, observed);
        if native_user_break(&self.options, scope, observed) {
            return Ok(completions);
        }
        self.store_face_tasks(tasks, scope, observed)?;
        Ok(completions)
    }

    fn prepare_face_tasks(
        &mut self,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<Option<Vec<FaceFaceTaskInput>>, Failure> {
        let edge_points = self.ee_new_vertices_by_edges()?;
        self.initialize_progress_pairs(ShapeEnum::Face, ShapeEnum::Face)?;
        let mut tasks = Vec::new();
        while self.progress_pair_cursor()?.more() {
            if native_user_break(&self.options, scope, observed) {
                return Ok(None);
            }
            let (first, second) = self.progress_pair_cursor()?.value()?;
            if let Some(task) = self.face_face_input(first, second, &edge_points)? {
                tasks.push(task);
            }
            self.progress_pair_cursor()?.next();
        }
        Ok(Some(tasks))
    }

    fn store_face_tasks(
        &mut self,
        tasks: Vec<BOPAlgoFaceFaceTask<'static>>,
        scope: &ProgressScope<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        for mut task in tasks {
            if native_user_break(&self.options, scope, observed) {
                return Ok(());
            }
            if !self.store_face_task_with_progress(&mut task, scope, observed)? {
                return Ok(());
            }
        }
        Ok(())
    }

    fn update_touched_face_info(&mut self, pairs: &[(i32, i32)]) -> Result<(), Failure> {
        let mut faces: Map<i32> = Map::new(1);
        for (first, second) in pairs {
            faces.add(*first);
            faces.add(*second);
        }
        let structure = self.structure_mut()?;
        for index in 0..structure.nb_source_shapes() {
            let info = structure.shape_info(index)?;
            if info.shape_type() == ShapeEnum::Face && info.has_reference() {
                faces.add(index);
            }
        }
        structure.update_face_info_on_map(&faces)?;
        structure.update_face_info_in_map(&faces)
    }

    /// New vertices of EE interferences keyed by their edge pair (`aEEMap`).
    fn ee_new_vertices_by_edges(&self) -> Result<DataMap<Pair, Vec<i32>>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut map: DataMap<Pair, Vec<i32>> = DataMap::default();
        for record in structure.interf_ee_records().iter() {
            let Some(vertex) = record.get_index_new() else {
                continue;
            };
            let (first, second) = record.indices();
            let key = Pair::new(first, second);
            if let Some(list) = map.change_seek(&key) {
                list.push(vertex);
            } else {
                map.bind(key, vec![vertex]);
            }
        }
        Ok(map)
    }

    /// Builds the task of one face pair, or records an empty interference for glued pairs and
    /// non-interfering planes.
    fn face_face_input(
        &mut self,
        first: i32,
        second: i32,
        edge_points: &DataMap<Pair, Vec<i32>>,
    ) -> Result<Option<FaceFaceTaskInput>, Failure> {
        if self.glue != Glue::Off {
            self.append_empty_face_face_record(first, second, Some(false))?;
            return Ok(None);
        }
        let faces = self.face_pair(first, second)?;
        let types = self.face_surface_types(&faces)?;
        if types == [SurfaceType::Plane, SurfaceType::Plane] && !self.check_planes(first, second)? {
            self.append_empty_face_face_record(first, second, None)?;
            return Ok(None);
        }
        self.configured_face_input([first, second], &faces, types, edge_points)
            .map(Some)
    }

    fn configured_face_input(
        &mut self,
        indices: [i32; 2],
        faces: &[Face; 2],
        types: [SurfaceType; 2],
        edge_points: &DataMap<Pair, Vec<i32>>,
    ) -> Result<FaceFaceTaskInput, Failure> {
        let (shifted, shift) = if types != [SurfaceType::Plane, SurfaceType::Plane] {
            self.shift_faces_to_edge_intersections(faces, edge_points)?
        } else {
            (faces.clone(), 0.0)
        };
        let boxes = self.face_task_bounds(indices)?;
        // With closed edges the curve tolerance must not be less than the applied shift.
        let tolerance = shift.max(self.tolerance_ff(faces, types)?);
        let seeds = self.ef_points(indices[0], indices[1])?;
        Ok(self.owned_face_input(indices, shifted, boxes, tolerance, seeds))
    }

    fn face_task_bounds(&self, indices: [i32; 2]) -> Result<[BoundingBox3; 2], Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        Ok([
            structure.shape_info(indices[0])?.box_bounds().clone(),
            structure.shape_info(indices[1])?.box_bounds().clone(),
        ])
    }

    fn owned_face_input(
        &self,
        indices: [i32; 2],
        faces: [Face; 2],
        boxes: [BoundingBox3; 2],
        tolerance: f64,
        seeds: Vec<IntSurfPntOn2S>,
    ) -> FaceFaceTaskInput {
        let parameters = self.face_curve_policy();
        FaceFaceTaskInput {
            indices,
            faces,
            boxes,
            tolerance,
            seeds,
            parameters,
            fuzzy: self.options.fuzzy_value(),
            parallel: self.options.run_parallel(),
        }
    }

    fn face_curve_policy(&self) -> [bool; 3] {
        let attribute = self.section_attribute;
        [
            attribute.approximation(),
            attribute.pcurve_on_s1(),
            attribute.pcurve_on_s2(),
        ]
    }

    pub(super) fn append_empty_face_face_record(
        &mut self,
        first: i32,
        second: i32,
        tangent: Option<bool>,
    ) -> Result<(), Failure> {
        let mut record = InterfFF::default();
        record.set_indices(first, second);
        if let Some(tangent) = tangent {
            record.set_tangent_faces(tangent);
        }
        record.init(0, 0);
        self.structure_mut()?.interf_ff().append(record);
        Ok(())
    }

    pub(in crate::bop_algo::pave_filler) fn face_pair(
        &self,
        first: i32,
        second: i32,
    ) -> Result<[Face; 2], Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        Ok([
            Face::unchecked(structure.shape(first)?.clone()),
            Face::unchecked(structure.shape(second)?.clone()),
        ])
    }

    fn face_surface_types(&self, faces: &[Face; 2]) -> Result<[SurfaceType; 2], Failure> {
        let context = self.context_ref()?;
        Ok([
            context.surface_adaptor(&faces[0])?.read().get_type()?,
            context.surface_adaptor(&faces[1])?.read().get_type()?,
        ])
    }

    /// Native static `ToleranceFF`: the larger face tolerance, at least 5e-6 unless both
    /// surfaces are elementary.
    fn tolerance_ff(&self, faces: &[Face; 2], types: [SurfaceType; 2]) -> Result<f64, Failure> {
        let mut tolerance =
            BRepTool::face_tolerance(&faces[0])?.max(BRepTool::face_tolerance(&faces[1])?);
        let analytic = |kind: SurfaceType| {
            matches!(
                kind,
                SurfaceType::Plane
                    | SurfaceType::Cylinder
                    | SurfaceType::Cone
                    | SurfaceType::Sphere
                    | SurfaceType::Torus
            )
        };
        if !analytic(types[0]) || !analytic(types[1]) {
            tolerance = tolerance.max(5.0e-6);
        }
        Ok(tolerance)
    }

    /// Starting points of the walking from EF intersections between the faces (`GetEFPnts`).
    fn ef_points(&self, first: i32, second: i32) -> Result<Vec<IntSurfPntOn2S>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let mut shapes: Map<i32> = Map::new(1);
        for face in [first, second] {
            shapes.add(face);
            for sub_shape in structure.shape_info(face)?.sub_shapes() {
                shapes.add(*sub_shape);
            }
        }
        let mut points = Vec::new();
        for record in structure.interf_ef_records().iter() {
            if !record.has_index_new() {
                continue;
            }
            let (edge, opposite) = record.indices();
            if !shapes.contains(&edge) || !shapes.contains(&opposite) {
                continue;
            }
            let parameter = record.common_part().vertex_parameter1();
            let face = if opposite == first { second } else { first };
            if let Some(point) = self.ef_point(edge, opposite, face, parameter, face == first)? {
                points.push(point);
            }
        }
        Ok(points)
    }

    fn ef_point(
        &self,
        edge: i32,
        opposite: i32,
        face: i32,
        parameter: f64,
        on_first: bool,
    ) -> Result<Option<IntSurfPntOn2S>, Failure> {
        let structure = self.data_structure.as_ref().ok_or_else(missing_structure)?;
        let context = self.context_ref()?;
        let edge = Edge::unchecked(structure.shape(edge)?.clone());
        let opposite = Face::unchecked(structure.shape(opposite)?.clone());
        let face = Face::unchecked(structure.shape(face)?.clone());
        let (curve, _, _) = BRepTool::curve(&edge)?;
        let point = curve.eval_d0(parameter)?;
        let pcurve = BRepTool::curve_on_face(&edge, &face)?.pcurve;
        let opposite_projector = context.proj_ps(&opposite)?;
        let mut seed = IntSurfPntOn2S::default();
        if let Some(pcurve) = pcurve.get() {
            let uv = pcurve.value(parameter)?;
            let mut projection = opposite_projector.write();
            projection.perform(point)?;
            if !projection.is_done() {
                return Ok(None);
            }
            let (u, v) = projection.lower_distance_parameters()?;
            if on_first {
                seed.set_parameters(uv.x(), uv.y(), u, v);
            } else {
                seed.set_parameters(u, v, uv.x(), uv.y());
            }
            return Ok(Some(seed));
        }
        let face_projector = context.proj_ps(&face)?;
        let mut face_projection = face_projector.write();
        face_projection.perform(point)?;
        let mut projection = opposite_projector.write();
        projection.perform(point)?;
        if !face_projection.is_done() || !projection.is_done() {
            return Ok(None);
        }
        let (u1, v1) = face_projection.lower_distance_parameters()?;
        let (u2, v2) = projection.lower_distance_parameters()?;
        if on_first {
            seed.set_parameters(u1, v1, u2, v2);
        } else {
            seed.set_parameters(u2, v2, u1, v1);
        }
        Ok(Some(seed))
    }
}

/// Iterates the edges of the wires of a face in native TopoDSIterator order.
fn face_edges(face: &Face) -> Result<Vec<Edge>, Failure> {
    let mut edges = Vec::new();
    let mut wires = ShapeIterator::new(face, true, true);
    while wires.more() {
        let mut items = ShapeIterator::new(wires.value()?, true, true);
        while items.more() {
            edges.push(Edge::unchecked(items.value()?.clone()));
            items.next();
        }
        wires.next();
    }
    Ok(edges)
}

fn unpack_face_outputs(
    outputs: Vec<FaceFaceTaskOutput>,
    observed: &ProgressCancellation,
) -> (Vec<BOPAlgoFaceFaceTask<'static>>, Vec<ProgressCompletion>) {
    let mut tasks = Vec::with_capacity(outputs.len());
    let mut completions = Vec::new();
    for output in outputs {
        let (task, completion) = output.into_parts();
        if matches!(completion, ProgressCompletion::Cancelled { .. }) {
            observed.record_cancelled();
        }
        if !matches!(completion, ProgressCompletion::Completed) {
            completions.push(completion);
        }
        tasks.push(task);
    }
    (tasks, completions)
}

#[cfg(test)]
#[path = "face_face_store_contract.rs"]
mod store_contract;
