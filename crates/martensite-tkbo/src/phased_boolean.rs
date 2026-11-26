// SPDX-License-Identifier: GPL-2.0-only
//! Phased Boolean pipeline (unite tools -> verify hints -> imprint/glue/select) (#42).
use crate::bop_algo::{Glue, Operation};
use crate::brep_algo_api::{BRepAlgoAPIBooleanOperation, BRepAlgoAPIFuse};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// Caller-supplied coincidence hint between one object face and one tool face (#42).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CoincidenceHint {
    /// 1-based face index on the object shape.
    pub object_face_index: usize,
    /// 1-based face index on the combined tool shape.
    pub tool_face_index: usize,
}

/// Result of independently verifying caller-supplied coincidence hints (#42).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifiedHintReport {
    requested: usize,
    verified: usize,
    rejected: usize,
}

impl VerifiedHintReport {
    /// Total hints supplied by the caller.
    pub fn requested(&self) -> usize {
        self.requested
    }
    /// Hints independently verified as coplanar/coincident.
    pub fn verified(&self) -> usize {
        self.verified
    }
    /// Hints rejected because geometry did not agree.
    pub fn rejected(&self) -> usize {
        self.rejected
    }
}

/// Separates multi-tool union, independent hint verification, and Boolean selection (#42).
pub struct ImprintGlueSelectPipeline;

impl ImprintGlueSelectPipeline {
    /// Phase 1: unites multiple `tools` into a single combined tool shape (#42).
    pub fn unite_tools(tools: &[Shape]) -> Result<Shape, Failure> {
        let Some((first, rest)) = tools.split_first() else {
            return Ok(Shape::default());
        };
        let mut combined = first.clone();
        for tool in rest {
            let fuse = BRepAlgoAPIFuse::from_shapes(&combined, tool)?;
            if fuse.is_done() {
                combined = fuse.shape().clone();
            }
        }
        Ok(combined)
    }

    /// Phase 2: independently verifies `hints` without trusting caller claims (#42).
    pub fn verify_hints(
        object: &Shape,
        tool: &Shape,
        hints: &[CoincidenceHint],
        tolerance: f64,
    ) -> Result<VerifiedHintReport, Failure> {
        let mut obj_faces = IndexedMapOfShape::default();
        let mut tool_faces = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(object, ShapeEnum::Face, &mut obj_faces)?;
        topexp::map_shapes_of_type(tool, ShapeEnum::Face, &mut tool_faces)?;

        let mut verified = 0usize;
        let mut rejected = 0usize;
        for hint in hints {
            if hint.object_face_index == 0
                || hint.object_face_index > obj_faces.extent()
                || hint.tool_face_index == 0
                || hint.tool_face_index > tool_faces.extent()
            {
                rejected += 1;
                continue;
            }
            let f_obj = topods::face(obj_faces.find_key(hint.object_face_index)?)?;
            let f_tool = topods::face(tool_faces.find_key(hint.tool_face_index)?)?;
            let a_obj = BRepAdaptorSurface::with_face(&f_obj, true)?;
            let a_tool = BRepAdaptorSurface::with_face(&f_tool, true)?;
            if a_obj.get_type()? == SurfaceType::Plane && a_tool.get_type()? == SurfaceType::Plane {
                let p1 = a_obj.plane()?;
                let p2 = a_tool.plane()?;
                let n1 = p1.axis().direction();
                let n2 = p2.axis().direction();
                let parallel = n1.dot(n2).abs() > 1.0 - 1e-6;
                let dist = p1.distance(p2.location());
                if parallel && dist <= tolerance {
                    verified += 1;
                    continue;
                }
            }
            rejected += 1;
        }
        Ok(VerifiedHintReport {
            requested: hints.len(),
            verified,
            rejected,
        })
    }

    /// Phase 3: runs imprint, glue, and select with verified hints (#42).
    pub fn execute(
        object: &Shape,
        tools: &[Shape],
        operation: Operation,
        hints: &[CoincidenceHint],
        tolerance: f64,
    ) -> Result<(Shape, VerifiedHintReport), Failure> {
        let united_tool = Self::unite_tools(tools)?;
        let report = Self::verify_hints(object, &united_tool, hints, tolerance)?;
        let mut bop = BRepAlgoAPIBooleanOperation::new();
        let mut args = List::default();
        args.append(object.clone());
        let mut tool_list = List::default();
        tool_list.append(united_tool);
        bop.set_arguments(&args);
        bop.set_tools(&tool_list);
        bop.set_operation(operation);
        if report.requested() > 0 && report.rejected() == 0 {
            bop.set_glue(Glue::Shift);
        } else {
            bop.set_glue(Glue::Off);
        }
        bop.build()?;
        Ok((bop.shape().clone(), report))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkmath::gp::Point3;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;

    #[test]
    fn pipeline_unites_multiple_tools_and_rejects_unverified_hints() {
        let base = BRepPrimAPIMakeBox::from_points(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(4.0, 4.0, 4.0),
        )
        .expect("base")
        .shape()
        .expect("shape")
        .clone();
        let t1 = BRepPrimAPIMakeBox::from_points(
            Point3::new(1.0, 1.0, 2.0),
            Point3::new(2.0, 2.0, 5.0),
        )
        .expect("t1")
        .shape()
        .expect("shape")
        .clone();
        let t2 = BRepPrimAPIMakeBox::from_points(
            Point3::new(2.5, 1.0, 2.0),
            Point3::new(3.5, 2.0, 5.0),
        )
        .expect("t2")
        .shape()
        .expect("shape")
        .clone();

        let false_hint = CoincidenceHint {
            object_face_index: 999,
            tool_face_index: 1,
        };
        let (result, report) = ImprintGlueSelectPipeline::execute(
            &base,
            &[t1, t2],
            Operation::Cut,
            &[false_hint],
            1e-6,
        )
        .expect("phased boolean");
        assert!(!result.is_null());
        assert_eq!(report.requested(), 1);
        assert_eq!(report.rejected(), 1);
    }
}
