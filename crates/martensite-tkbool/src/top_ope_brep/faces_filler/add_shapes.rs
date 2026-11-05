//! Finalized line geometry retains both operand incidences and source tolerances.
use super::*;
use crate::top_ope_brep::TopOpeBRepGeomTool;
use crate::top_ope_brep_ds::{TopOpeBRepDSCurve, TopOpeBRepDSInterferenceTool as Tool};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::bnd_lib::Add3dCurve;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::Point3;

impl TopOpeBRepFacesFiller<'_> {
    /// Attaches the current line curve and both face interferences; `filler.add_shapes_line(&mut line)?`.
    pub fn add_shapes_line(&mut self, line: &mut TopOpeBRepLineInter) -> Result<(), Failure> {
        if self.curve_interferences.is_empty() || line.inl() {
            return Ok(());
        }
        let hds = self.hdata_structure();
        let storage = hds.get().ok_or_else(missing_storage)?;
        let faces = self.register_operand_faces(storage);
        let range = storage
            .min_max_on_parameter(&self.curve_interferences)?
            .ok_or_else(missing_interference)?;
        if self.degenerate_current_curve(line, range.0, range.1)? {
            return self.discard_current_curve();
        }
        self.build_current_curve(storage, line, faces, range)
    }

    fn build_current_curve(
        &mut self,
        storage: &TopOpeBRepDSHDataStructure,
        line: &mut TopOpeBRepLineInter,
        faces: [i32; 2],
        range: (f64, f64),
    ) -> Result<(), Failure> {
        let (mut first_pcurve, mut second_pcurve) = (Handle::default(), Handle::default());
        {
            let mut ds = storage.change_ds();
            let curve = ds.change_curve(self.curve_index);
            // The stored curve remains observable after a later rejection or geometry error.
            self.make_current_curves(line, curve, range, &mut first_pcurve, &mut second_pcurve)?;
            if self.is_too_small(curve)? {
                curve.change_keep(false);
                return Ok(());
            }
            curve.set_tolerance(1.0e-8);
        }
        self.attach_curve_interferences(storage, line, faces, &first_pcurve, &second_pcurve)
    }

    fn make_current_curves(
        &self,
        line: &mut TopOpeBRepLineInter,
        curve: &mut TopOpeBRepDSCurve,
        range: (f64, f64),
        first: &mut Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve>,
        second: &mut Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve>,
    ) -> Result<(), Failure> {
        TopOpeBRepGeomTool::make_curves(
            range.0,
            range.1,
            line,
            self.faces[0].as_ref(),
            self.faces[1].as_ref(),
            curve,
            first,
            second,
        )
    }

    fn register_operand_faces(&self, storage: &TopOpeBRepDSHDataStructure) -> [i32; 2] {
        let mut ds = storage.change_ds();
        [
            ds.add_shape_with_rank(self.faces[0].as_ref(), 1),
            ds.add_shape_with_rank(self.faces[1].as_ref(), 2),
        ]
    }

    fn discard_current_curve(&self) -> Result<(), Failure> {
        self.change_data_structure()?
            .change_curve(self.curve_index)
            .change_keep(false);
        Ok(())
    }

    fn is_too_small(&self, curve: &TopOpeBRepDSCurve) -> Result<bool, Failure> {
        let (first, last) = curve.range().ok_or_else(missing_interference)?;
        let adaptor = GeomAdaptorCurve::with_curve(curve.curve())?;
        let mut bounds = BoundingBox3::default();
        Add3dCurve::add_range(&adaptor, first, last, 0.0, &mut bounds)?;
        let low = bounds.corner_min()?;
        let high = bounds.corner_max()?;
        let maximum = (high.x() - low.x())
            .max(high.y() - low.y())
            .max(high.z() - low.z());
        let minimum_tolerance = BRepTool::face_tolerance(&self.faces[0])?
            .min(BRepTool::face_tolerance(&self.faces[1])?);
        Ok(maximum < minimum_tolerance)
    }

    fn degenerate_current_curve(
        &self,
        line: &mut TopOpeBRepLineInter,
        first: f64,
        last: f64,
    ) -> Result<bool, Failure> {
        if (first - last).abs() <= precision::p_confusion() && !line.is_periodic() {
            return Ok(true);
        }
        if line.type_line_curve() != TopOpeBRepTypeLineCurve::Walking
            || line.is_periodic()
            || !line.is_v_closed()
        {
            return Ok(false);
        }
        let first = self
            .curve_interferences
            .iter()
            .next()
            .ok_or_else(missing_interference)?;
        let last = self
            .curve_interferences
            .iter()
            .last()
            .ok_or_else(missing_interference)?;
        let (position, first_tolerance) = self.geometry_point(first)?;
        let (_, last_tolerance) = self.geometry_point(last)?;
        let tolerance = first_tolerance.max(last_tolerance);
        for index in 1..=line.nb_wpoint()? {
            if !line.wpoint(index)?.value()?.is_equal(&position, tolerance) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn geometry_point(
        &self,
        interference: &Handle<TopOpeBRepDSInterference>,
    ) -> Result<(Point3, f64), Failure> {
        let entry = interference.get().ok_or_else(missing_interference)?;
        let storage = self.hds.get().ok_or_else(missing_storage)?;
        if entry.geometry_type() == TopOpeBRepDSKind::Point {
            let point = storage.point(entry.geometry())?;
            return Ok((point.point(), point.tolerance()));
        }
        let ds = storage.ds();
        let vertex = topods::vertex(ds.shape(entry.geometry(), true))?;
        Ok((
            BRepTool::pnt(&vertex)?,
            BRepTool::vertex_tolerance(&vertex)?,
        ))
    }

    fn attach_curve_interferences(
        &mut self,
        storage: &TopOpeBRepDSHDataStructure,
        line: &TopOpeBRepLineInter,
        faces: [i32; 2],
        first_pcurve: &Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve>,
        second_pcurve: &Handle<dyn martensite_tkg2d::geom2d::Geom2dCurve>,
    ) -> Result<(), Failure> {
        storage
            .change_ds()
            .change_curve_interferences(self.curve_index)
            .append_list(&mut self.curve_interferences);
        let mut first_transition = *line.face_face_transition(1)?;
        let mut second_transition = *line.face_face_transition(2)?;
        first_transition.set_index(faces[1]);
        second_transition.set_index(faces[0]);
        let first = Tool::make_face_curve_interference(
            &first_transition,
            faces[1],
            self.curve_index,
            first_pcurve,
        );
        let second = Tool::make_face_curve_interference(
            &second_transition,
            faces[0],
            self.curve_index,
            second_pcurve,
        );
        storage.store_interference(&first, self.faces[0].as_ref())?;
        storage.store_interference(&second, self.faces[1].as_ref())?;
        let mut ds = storage.change_ds();
        let curve = ds.change_curve(self.curve_index);
        curve.set_shapes(self.faces[0].as_ref(), self.faces[1].as_ref());
        curve.set_sci(&first, &second);
        Ok(())
    }
}
