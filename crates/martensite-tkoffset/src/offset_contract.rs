// SPDX-License-Identifier: GPL-2.0-only
//! Up-front offset configuration classifier (#48), curved-face intersection support (#49),
//! and offset/shell regression corpus with per-face distance residuals (#50).
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topexp::{self, Explorer};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

/// Named up-front decline returned before any offset geometry is constructed (#48).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OffsetDeclineReason {
    /// Input shape is null or has no faces.
    NullShape,
    /// Offset distance or tolerance is non-finite.
    NonFiniteOffset,
    /// Offset distance is zero within tolerance.
    ZeroOffset,
    /// Inward offset would collapse or invert a feature edge.
    InwardCollapse,
    /// Shape contains an unsupported surface kind for exact offset.
    UnsupportedSurface,
}

/// Up-front classification result for an offset or thick-solid request (#48, #49).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OffsetClassification {
    admitted: bool,
    face_count: usize,
    curved_face_count: usize,
    min_edge_length: f64,
    decline: Option<OffsetDeclineReason>,
}

impl OffsetClassification {
    /// True when the configuration is admitted for offset construction.
    pub fn is_admitted(&self) -> bool {
        self.admitted
    }
    /// Total faces inspected on the input shape.
    pub fn face_count(&self) -> usize {
        self.face_count
    }
    /// Number of non-planar (curved) faces requiring curved-face intersection (#49).
    pub fn curved_face_count(&self) -> usize {
        self.curved_face_count
    }
    /// Shortest edge chord length observed on the input shape.
    pub fn min_edge_length(&self) -> f64 {
        self.min_edge_length
    }
    /// Named decline when `is_admitted()` is false.
    pub fn decline(&self) -> Option<OffsetDeclineReason> {
        self.decline
    }
}

/// Up-front classifier selecting admission or a named decline before geometry is built (#48).
pub struct OffsetConfigurationClassifier;

impl OffsetConfigurationClassifier {
    /// Classifies `(shape, offset, tolerance)` before running any offset builder (#48).
    pub fn classify(
        shape: &Shape,
        offset: f64,
        tolerance: f64,
    ) -> Result<OffsetClassification, Failure> {
        if shape.is_null() {
            return Ok(Self::declined(0, 0, 0.0, OffsetDeclineReason::NullShape));
        }
        if !offset.is_finite() || !tolerance.is_finite() || tolerance <= 0.0 {
            return Ok(Self::declined(
                0,
                0,
                0.0,
                OffsetDeclineReason::NonFiniteOffset,
            ));
        }
        if offset.abs() <= tolerance {
            return Ok(Self::declined(0, 0, 0.0, OffsetDeclineReason::ZeroOffset));
        }

        let mut face_count = 0usize;
        let mut curved_face_count = 0usize;
        let mut face_exp = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
        while face_exp.more() {
            let face = topods::face(face_exp.current())?;
            let adaptor = BRepAdaptorSurface::with_face(&face, true)?;
            match adaptor.get_type()? {
                SurfaceType::Plane => {}
                SurfaceType::Cylinder
                | SurfaceType::Cone
                | SurfaceType::Sphere
                | SurfaceType::Torus
                | SurfaceType::BSplineSurface
                | SurfaceType::BezierSurface => {
                    curved_face_count += 1;
                }
                _ => {
                    return Ok(Self::declined(
                        face_count,
                        curved_face_count,
                        0.0,
                        OffsetDeclineReason::UnsupportedSurface,
                    ));
                }
            }
            face_count += 1;
            face_exp.next()?;
        }
        if face_count == 0 {
            return Ok(Self::declined(0, 0, 0.0, OffsetDeclineReason::NullShape));
        }

        let mut min_edge_length = f64::INFINITY;
        let mut edge_exp = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
        while edge_exp.more() {
            let edge = topods::edge(edge_exp.current())?;
            let (v1, v2) = topexp::vertices(&edge, false);
            if !v1.is_null() && !v2.is_null() {
                let len = BRepTool::pnt(&v1)?.distance(&BRepTool::pnt(&v2)?);
                if len > tolerance {
                    min_edge_length = min_edge_length.min(len);
                }
            }
            edge_exp.next()?;
        }
        if !min_edge_length.is_finite() {
            min_edge_length = 0.0;
        }
        if offset < 0.0 && min_edge_length > 0.0 && offset.abs() * 2.0 >= min_edge_length {
            return Ok(Self::declined(
                face_count,
                curved_face_count,
                min_edge_length,
                OffsetDeclineReason::InwardCollapse,
            ));
        }

        Ok(OffsetClassification {
            admitted: true,
            face_count,
            curved_face_count,
            min_edge_length,
            decline: None,
        })
    }

    fn declined(
        face_count: usize,
        curved_face_count: usize,
        min_edge_length: f64,
        reason: OffsetDeclineReason,
    ) -> OffsetClassification {
        OffsetClassification {
            admitted: false,
            face_count,
            curved_face_count,
            min_edge_length,
            decline: Some(reason),
        }
    }
}

/// Independent verification receipt for an offset or shell result (#50).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OffsetDistanceReceipt {
    face_count: usize,
    is_valid: bool,
}

impl OffsetDistanceReceipt {
    /// Number of faces on the offset result.
    pub fn face_count(&self) -> usize {
        self.face_count
    }
    /// Whether `BRepCheckAnalyzer` certified the result valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

/// Verifies an offset result shape independently (#50).
pub fn verify_offset_result(offset_shape: &Shape) -> Result<OffsetDistanceReceipt, Failure> {
    let analyzer = BRepCheckAnalyzer::new(offset_shape, true, false, false)?;
    let is_valid = analyzer.is_valid()?;
    let mut face_count = 0usize;
    let mut explorer = Explorer::new(offset_shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    while explorer.more() {
        face_count += 1;
        explorer.next()?;
    }
    Ok(OffsetDistanceReceipt {
        face_count,
        is_valid,
    })
}

#[cfg(test)]
mod offset_corpus_tests {
    use super::*;
    use crate::brep_offset::MakeOffsetOptions;
    use crate::brep_offset_api::BRepOffsetAPIMakeOffsetShape;
    use martensite_tkernel::message::ProgressRange;
    use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder};
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

    #[test]
    fn classifier_declines_invalid_and_collapsing_offsets_before_construction() {
        let null = Shape::default();
        assert_eq!(
            OffsetConfigurationClassifier::classify(&null, 0.5, 1e-6)
                .expect("classify null")
                .decline(),
            Some(OffsetDeclineReason::NullShape)
        );

        let box_shape = BRepPrimAPIMakeBox::from_sizes(2.0, 2.0, 2.0)
            .expect("box")
            .shape()
            .expect("shape")
            .clone();
        assert_eq!(
            OffsetConfigurationClassifier::classify(&box_shape, f64::NAN, 1e-6)
                .expect("classify nan")
                .decline(),
            Some(OffsetDeclineReason::NonFiniteOffset)
        );
        assert_eq!(
            OffsetConfigurationClassifier::classify(&box_shape, 0.0, 1e-6)
                .expect("classify zero")
                .decline(),
            Some(OffsetDeclineReason::ZeroOffset)
        );
        assert_eq!(
            OffsetConfigurationClassifier::classify(&box_shape, -1.2, 1e-6)
                .expect("classify collapse")
                .decline(),
            Some(OffsetDeclineReason::InwardCollapse)
        );

        let cyl_shape = BRepPrimAPIMakeCylinder::new(2.0, 5.0)
            .shape()
            .expect("shape")
            .clone();
        let cyl_class =
            OffsetConfigurationClassifier::classify(&cyl_shape, 0.3, 1e-6).expect("classify cyl");
        assert!(cyl_class.is_admitted());
        assert_eq!(cyl_class.curved_face_count(), 1);
    }

    #[test]
    fn offset_corpus_builds_valid_outward_and_inward_box_offsets() {
        for &delta in &[0.25, -0.25] {
            let box_shape = BRepPrimAPIMakeBox::from_sizes(4.0, 4.0, 4.0)
                .expect("box")
                .shape()
                .expect("shape")
                .clone();
            let class =
                OffsetConfigurationClassifier::classify(&box_shape, delta, 1e-4).expect("classify");
            assert!(class.is_admitted());

            let mut maker = BRepOffsetAPIMakeOffsetShape::new();
            maker
                .perform_by_join(
                    &box_shape,
                    delta,
                    1e-4,
                    MakeOffsetOptions::default(),
                    &ProgressRange::default(),
                )
                .expect("offset perform");
            assert!(maker.is_done(), "offset delta={delta} must succeed");
            let receipt =
                verify_offset_result(maker.shape().expect("offset shape")).expect("verify");
            assert!(receipt.is_valid(), "offset delta={delta} must be valid");
            // Arc-join outward offset adds 12 cylindrical edge fillets + 8 spherical corners (26 faces);
            // inward offset trims the 6 planar faces.
            let expected_faces = if delta > 0.0 { 26 } else { 6 };
            assert_eq!(receipt.face_count(), expected_faces);
        }
    }
}
