// SPDX-License-Identifier: GPL-2.0-only
//! Structured STEP exchange result with quality metrics, diagnostics, and source
//! identities (#57), plus independent per-face round-trip tests (#58).
use crate::de_step::{DeStepParameters, StepControlStepModelType};
use crate::step_control::{StepControlReader, StepControlWriter};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::IndexedMapOfShape;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use std::io::Cursor;

/// Exchange format identifier tracked in the capability matrix (#58).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExchangeFormat {
    /// STEP AP203.
    StepAp203,
    /// STEP AP214.
    StepAp214,
    /// STEP AP242.
    StepAp242,
    /// IGES 5.3.
    Iges,
    /// Native B-rep archive.
    Brep,
}

/// Direction of an exchange operation (#57).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExchangeDirection {
    /// Reading external format into B-rep.
    Import,
    /// Writing B-rep into external format.
    Export,
}

/// One diagnostic recorded during import or export (#57).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeDiagnostic {
    severity: &'static str,
    message: String,
}

impl ExchangeDiagnostic {
    /// Severity level (`"info"`, `"warning"`, or `"error"`).
    pub fn severity(&self) -> &'static str {
        self.severity
    }
    /// Diagnostic message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Source identity preserved across exchange (#57).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExchangeIdentityEntry {
    source_id: String,
    entity_index: usize,
}

impl ExchangeIdentityEntry {
    /// Caller or format-level source identifier.
    pub fn source_id(&self) -> &str {
        &self.source_id
    }
    /// Canonical 1-based `TopExp::MapShapes` index.
    pub fn entity_index(&self) -> usize {
        self.entity_index
    }
}

/// Operation-result style exchange outcome carrying completion, validation,
/// achieved tolerance, diagnostics, and source identities (#57).
#[derive(Clone, Debug)]
pub struct ExchangeResult {
    format: ExchangeFormat,
    direction: ExchangeDirection,
    completed: bool,
    shape: Shape,
    is_valid: bool,
    achieved_max_tolerance: f64,
    identities: Vec<ExchangeIdentityEntry>,
    diagnostics: Vec<ExchangeDiagnostic>,
}

impl ExchangeResult {
    /// Exchange format.
    pub fn format(&self) -> ExchangeFormat {
        self.format
    }
    /// Import or export direction.
    pub fn direction(&self) -> ExchangeDirection {
        self.direction
    }
    /// True when the underlying reader/writer completed with `RetDone`.
    pub fn is_completed(&self) -> bool {
        self.completed
    }
    /// Exchanged B-rep shape.
    pub fn shape(&self) -> &Shape {
        &self.shape
    }
    /// Whether `BRepCheckAnalyzer` certified the shape valid.
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    /// Largest entity tolerance observed on the exchanged shape.
    pub fn achieved_max_tolerance(&self) -> f64 {
        self.achieved_max_tolerance
    }
    /// Preserved source identity bindings.
    pub fn identities(&self) -> &[ExchangeIdentityEntry] {
        &self.identities
    }
    /// Recorded diagnostics.
    pub fn diagnostics(&self) -> &[ExchangeDiagnostic] {
        &self.diagnostics
    }
}

/// Exports `shape` to STEP AP214 bytes with quality metrics and source identities (#57).
pub fn export_step_checked(
    shape: &Shape,
    source_identities: &[(String, Shape)],
) -> Result<(Vec<u8>, ExchangeResult), Failure> {
    let mut diagnostics = Vec::new();
    let mut identities = Vec::new();
    let mut map = IndexedMapOfShape::default();
    if !shape.is_null() {
        topexp::map_shapes(shape, &mut map, true, true);
    }
    for (id, entity) in source_identities {
        let idx = map.find_index(entity);
        if idx > 0 {
            identities.push(ExchangeIdentityEntry {
                source_id: id.clone(),
                entity_index: idx,
            });
        } else {
            diagnostics.push(ExchangeDiagnostic {
                severity: "warning",
                message: format!("source identity '{id}' not found in exported shape"),
            });
        }
    }

    let writer = StepControlWriter::new()?;
    let xfer = writer.transfer(
        shape,
        StepControlStepModelType::AS_IS,
        true,
        &ProgressRange::default(),
    )?;
    let mut bytes = Vec::new();
    let written = if xfer == IFSelectReturnStatus::RetDone {
        writer.write_stream(&mut bytes)?
    } else {
        diagnostics.push(ExchangeDiagnostic {
            severity: "error",
            message: format!("STEP transfer failed with status {xfer:?}"),
        });
        xfer
    };
    let completed = written == IFSelectReturnStatus::RetDone && !bytes.is_empty();
    let (is_valid, achieved_max_tolerance) = inspect_shape_quality(shape)?;

    Ok((
        bytes,
        ExchangeResult {
            format: ExchangeFormat::StepAp214,
            direction: ExchangeDirection::Export,
            completed,
            shape: shape.clone(),
            is_valid,
            achieved_max_tolerance,
            identities,
            diagnostics,
        },
    ))
}

/// Imports STEP bytes into a B-rep shape with validation, achieved tolerance, and
/// diagnostics (#57).
pub fn import_step_checked(step_bytes: &[u8]) -> Result<ExchangeResult, Failure> {
    let mut diagnostics = Vec::new();
    let mut reader = StepControlReader::new()?;
    let mut params = DeStepParameters::default();
    params.init_from_static();
    let mut cursor = Cursor::new(step_bytes);
    let read_status = reader.read_stream_with("memory.stp", &params, &mut cursor)?;
    if read_status != IFSelectReturnStatus::RetDone {
        diagnostics.push(ExchangeDiagnostic {
            severity: "error",
            message: format!("STEP read_stream failed with status {read_status:?}"),
        });
        return Ok(ExchangeResult {
            format: ExchangeFormat::StepAp214,
            direction: ExchangeDirection::Import,
            completed: false,
            shape: Shape::default(),
            is_valid: false,
            achieved_max_tolerance: f64::INFINITY,
            identities: Vec::new(),
            diagnostics,
        });
    }
    let transferred = reader.transfer_roots(&ProgressRange::default())?;
    if transferred == 0 {
        diagnostics.push(ExchangeDiagnostic {
            severity: "warning",
            message: "STEP import transferred zero root shapes".to_string(),
        });
    }
    let shape = reader.xs_reader().one_shape();
    let completed = !shape.is_null();
    let (is_valid, achieved_max_tolerance) = inspect_shape_quality(&shape)?;
    Ok(ExchangeResult {
        format: ExchangeFormat::StepAp214,
        direction: ExchangeDirection::Import,
        completed,
        shape,
        is_valid,
        achieved_max_tolerance,
        identities: Vec::new(),
        diagnostics,
    })
}

fn inspect_shape_quality(shape: &Shape) -> Result<(bool, f64), Failure> {
    if shape.is_null() {
        return Ok((false, f64::INFINITY));
    }
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false)?;
    let is_valid = analyzer.is_valid()?;
    let v = BRepTool::max_tolerance(shape, ShapeEnum::Vertex)?;
    let e = BRepTool::max_tolerance(shape, ShapeEnum::Edge)?;
    let f = BRepTool::max_tolerance(shape, ShapeEnum::Face)?;
    Ok((is_valid, v.max(e).max(f)))
}

#[cfg(test)]
mod round_trip_tests {
    use super::*;
    use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
    use martensite_tkbrep::topexp::Explorer;
    use martensite_tkbrep::topods;
    use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
    use martensite_tkmath::geom_abs::SurfaceType;
    use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

    #[test]
    fn step_round_trip_preserves_face_count_surface_types_validity_and_tolerance() {
        let box_shape = BRepPrimAPIMakeBox::from_sizes(10.0, 20.0, 30.0)
            .expect("box")
            .shape()
            .expect("shape")
            .clone();

        let (step_bytes, export_res) =
            export_step_checked(&box_shape, &[("part_body".to_string(), box_shape.clone())])
                .expect("export step");
        assert!(export_res.is_completed());
        assert!(export_res.is_valid());
        assert_eq!(export_res.identities().len(), 1);
        assert_eq!(export_res.identities()[0].source_id(), "part_body");
        assert!(!step_bytes.is_empty());

        let import_res = import_step_checked(&step_bytes).expect("import step");
        assert!(import_res.is_completed());
        assert!(import_res.is_valid());
        assert!(import_res.achieved_max_tolerance() <= 1e-3);

        let mut planes = 0usize;
        let mut explorer =
            Explorer::new(import_res.shape(), ShapeEnum::Face, ShapeEnum::Shape).expect("faces");
        while explorer.more() {
            let face = topods::face(explorer.current()).expect("face");
            let adaptor = BRepAdaptorSurface::with_face(&face, true).expect("adaptor");
            assert_eq!(adaptor.get_type().expect("type"), SurfaceType::Plane);
            planes += 1;
            explorer.next().expect("next");
        }
        assert_eq!(planes, 6);
    }
}
