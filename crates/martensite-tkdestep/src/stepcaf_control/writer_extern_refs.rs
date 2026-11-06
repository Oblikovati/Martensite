//! `STEPCAFControlWriter::writeExternRefs`: the references of the main file to its external files.
use super::writer::{WHERE, WriterState};
use super::writer_support::{finder_process_of, typed_result_of};
use crate::de_step::DeStepWriteModeStepSchema;
use crate::step_construct::StepConstructExternRefs;
use crate::step_data::StepDataStepModel;
use crate::step_entity::step_live;
use crate::step_shape::StepShapeShapeDefinitionRepresentation;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tklcaf::tdf::TdfLabel;
use martensite_tkxcaf::xcaf_doc::XCAFDocShapeTool;
use martensite_tkxsbase::xs_control::XSControlWorkSession;

impl WriterState {
    /// `writeExternRefs`.
    pub(super) fn write_extern_refs(
        &self,
        ws: &Handle<XSControlWorkSession>,
        labels: &[TdfLabel],
    ) -> Result<bool, Failure> {
        if labels.is_empty() {
            return Ok(false);
        }
        let fp = finder_process_of(ws)?;
        let mut tool = StepConstructExternRefs::with_ws(ws)?;
        let model: Handle<StepDataStepModel> = step_live(ws, WHERE)?.model().cast();
        let schema = step_live(&model, WHERE)?.internal_parameters().write_schema;
        let format = if schema == DeStepWriteModeStepSchema::AP203 {
            "STEP AP203"
        } else {
            "STEP AP214"
        };
        // Iterate on requested shapes
        for label in labels {
            if XCAFDocShapeTool::is_assembly(label)? {
                continue; // skip assemblies
            }
            // get extern file
            let Some(ext_file) = self.lab_ef.get(label) else {
                continue; // should never be
            };
            // find SDR
            let Some(shape) = self.labels.get(label) else {
                continue; // not recorded as translated, skip
            };
            let sdr = typed_result_of::<StepShapeShapeDefinitionRepresentation>(&fp, shape)?;
            let Some(sdr) = sdr.get() else {
                continue;
            };
            // try to get PD from SDR
            let property = sdr.definition().property_definition();
            let Some(property) = property.get() else {
                continue;
            };
            let definition = property.definition().product_definition();
            if definition.is_null() {
                continue;
            }
            // add extern ref
            let name = step_live(ext_file, WHERE)?.get_name();
            let name =
                String::from_utf8_lossy(step_live(&name, WHERE)?.string().as_bytes()).into_owned();
            tool.add_extern_ref(&name, &definition, format)?;
        }
        tool.write_extern_refs(schema.as_raw())?;
        Ok(true)
    }
}
