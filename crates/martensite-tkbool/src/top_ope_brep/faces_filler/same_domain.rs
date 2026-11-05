//! Restriction coincidence uses the native interior sample and separate face tolerance.
use super::super::TopOpeBRepGeomTool;
use super::*;
use crate::top_ope_brep_tool::fun_tool_pin_c_range;
use martensite_tkbrep::{brep::BRepTool, brep_adaptor::BRepAdaptorCurve};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
impl TopOpeBRepFacesFiller<'_> {
    /// Tests an analytic line sample against restriction arcs; `Self::l_same_domain_erl(&mut line,&edges,1e-7)?`.
    pub fn l_same_domain_erl(
        line: &mut TopOpeBRepLineInter,
        restrictions: &List<Shape>,
        face_tolerance: f64,
    ) -> Result<bool, Failure> {
        if line.type_line_curve() == TopOpeBRepTypeLineCurve::Walking {
            return Ok(false);
        }
        let (first, last) = Self::lminmax(line)?;
        if (first - last).abs() <= precision::p_confusion() {
            return Ok(false);
        }
        let curve = TopOpeBRepGeomTool::make_curve(first, last, line)?;
        let sample = curve
            .get()
            .ok_or_else(missing_interference)?
            .value((1. - 0.417789) * first + 0.417789 * last)?;
        for restriction in restrictions.iter() {
            let edge = topods::edge(restriction)?;
            let tolerance = BRepTool::edge_tolerance(&edge)?;
            let tolerance = if tolerance < face_tolerance {
                face_tolerance
            } else {
                tolerance
            };
            let adaptor = BRepAdaptorCurve::with_edge(&edge)?;
            if fun_tool_pin_c_range(
                sample,
                &adaptor,
                adaptor.first_parameter()?,
                adaptor.last_parameter()?,
                tolerance,
            )? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
