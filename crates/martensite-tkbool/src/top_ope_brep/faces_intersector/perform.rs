//! Face-pair preparation and the native surface intersection dispatch.
use super::TopOpeBRepFacesIntersector;
use crate::{
    top_ope_brep::TopOpeBRepTypeLineCurve,
    top_ope_brep_tool::{TopOpeBRepToolShapeTool, ftol_face_tolerances3d_on_boxes},
};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dSurface, h_surface_tool::Adaptor3dSurfaceHandle},
    top_abs::Orientation,
};
use martensite_tkgeomalgo::int_patch::IntPatchIntersectionOptions;
use martensite_tkmath::bnd::BoundingBox3;

impl TopOpeBRepFacesIntersector {
    /// Intersects two faces using default boxes; `pair.perform(&first, &second)?`.
    pub fn perform(&mut self, first: &Shape, second: &Shape) -> Result<(), Failure> {
        self.perform_with_boxes(
            first,
            second,
            &BoundingBox3::default(),
            &BoundingBox3::default(),
        )
    }

    /// Intersects two faces with their bounding-box tolerance context; `pair.perform_with_boxes(&first, &second, &b1, &b2)?`.
    pub fn perform_with_boxes(
        &mut self,
        first: &Shape,
        second: &Shape,
        b1: &BoundingBox3,
        b2: &BoundingBox3,
    ) -> Result<(), Failure> {
        self.reset_intersection();
        if !self.forced_tolerances {
            self.tolerances = (
                martensite_tkernel::precision::confusion(),
                martensite_tkernel::precision::confusion(),
            );
        }
        self.initialize_faces(first, second)?;
        self.configure_tolerances(b1, b2)?;
        self.intersect_surfaces()?;
        if !self.intersection.is_done() {
            return Ok(());
        }
        self.prepare_lines()?;
        self.done = true;
        if self.same_domain()? {
            self.same_orientation = TopOpeBRepToolShapeTool::surfaces_same_oriented(
                &self.surfaces[0],
                &self.surfaces[1],
            )?;
        }
        self.collect_restrictions()
    }

    fn initialize_faces(&mut self, first: &Shape, second: &Shape) -> Result<(), Failure> {
        for (index, source) in [first, second].into_iter().enumerate() {
            let mut face = topods::face(source)?;
            face.set_orientation(Orientation::Forward);
            self.faces[index] = face.into_shape();
        }
        for index in 0..2 {
            self.surfaces[index].initialize(&topods::face(&self.faces[index])?, true)?;
        }
        for surface in &self.surfaces {
            surface.get_type()?;
        }
        for index in 0..2 {
            self.domains[index].initialize_surface(&self.surfaces[index].cast())?;
        }
        Ok(())
    }

    fn configure_tolerances(
        &mut self,
        b1: &BoundingBox3,
        b2: &BoundingBox3,
    ) -> Result<(), Failure> {
        let mut uv = 0.01;
        let mut deflection = 0.01;
        if !self.forced_tolerances {
            let t = ftol_face_tolerances3d_on_boxes(
                b1,
                b2,
                &topods::face(&self.faces[0])?,
                &topods::face(&self.faces[1])?,
                &self.surfaces[0],
                &self.surfaces[1],
            )?;
            self.tolerances = (
                if t.tol1 > 1e-4 { 1e-4 } else { t.tol1 },
                if t.tol2 > 1e-4 { 1e-4 } else { t.tol2 },
            );
            uv = t.max_uv;
            deflection = t.deflection;
        }
        self.intersection
            .set_tolerances(self.tolerances.0, self.tolerances.1, uv, deflection);
        Ok(())
    }

    fn intersect_surfaces(&mut self) -> Result<(), Failure> {
        let first: Adaptor3dSurfaceHandle = self.surfaces[0].cast();
        let second: Adaptor3dSurfaceHandle = self.surfaces[1].cast();
        self.intersection.perform(
            &first,
            &self.domains[0],
            &second,
            &self.domains[1],
            self.tolerances,
            IntPatchIntersectionOptions {
                is_geom_int: true,
                is_req_to_keep_rline: true,
                is_req_to_post_wl_proc: false,
            },
        )
    }

    fn collect_restrictions(&mut self) -> Result<(), Failure> {
        self.init_line()?;
        while self.more_line() {
            let edge = {
                let line = self.current_line()?;
                if line.type_line_curve() == TopOpeBRepTypeLineCurve::Restriction {
                    Some(line.arc()?)
                } else {
                    None
                }
            };
            if let Some(edge) = edge {
                self.restrictions.add(edge);
            }
            self.next_line()?;
        }
        Ok(())
    }

    fn reset_intersection(&mut self) {
        self.done = false;
        self.line_index = 1;
        self.line_count = 0;
        self.line_found = false;
        self.restrictions.clear(false);
    }
}
