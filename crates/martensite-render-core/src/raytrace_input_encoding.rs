//! Serialize only uniquely joined actual upload, pipeline and recorded dispatch inputs.
use super::*;
pub(super) fn encode(
    token: &RayInputObservation,
    rows: &RayInputRows,
    serial: u64,
) -> Result<String, RenderError> {
    let (upload, dispatch, command) = joined(rows)?;
    validate_bindings(upload, dispatch, command)?;
    validate_sources(&dispatch.pipeline)?;
    Ok(format!(
        "{{{},{},{},{},{}}}",
        identity_json(token, serial),
        frame_json(command),
        upload_json(upload, dispatch),
        pipeline_json(&dispatch.pipeline),
        sources_json(&dispatch.pipeline)
    ))
}
fn joined(
    rows: &RayInputRows,
) -> Result<(&RayUploadInputs, &RayDispatchInputs, &RayCommandInputs), RenderError> {
    if let Some(reason) = rows.refusal {
        return Err(rejected(reason, "supported unique ray input joins"));
    }
    match (&rows.upload, &rows.dispatch, &rows.command) {
        (Some(upload), Some(dispatch), Some(command)) => Ok((upload, dispatch, command)),
        _ => Err(rejected(
            "missing upload/dispatch/frame",
            "all three actual ray input observations",
        )),
    }
}

fn validate_bindings(
    upload: &RayUploadInputs,
    dispatch: &RayDispatchInputs,
    command: &RayCommandInputs,
) -> Result<(), RenderError> {
    if !matching_bindings(upload, dispatch, command) {
        return Err(rejected(
            (dispatch.command, &dispatch.sets, dispatch.pipeline.pipeline),
            "one main Render dispatch on its uploaded descriptor set and frame",
        ));
    }
    Ok(())
}
fn matching_bindings(
    upload: &RayUploadInputs,
    dispatch: &RayDispatchInputs,
    command: &RayCommandInputs,
) -> bool {
    let pipeline = &dispatch.pipeline;
    command.command != 0
        && command.command == dispatch.command
        && dispatch.sets == [upload.set]
        && pipeline.pipeline != 0
        && pipeline.layout != 0
        && upload.layout != 0
        && pipeline.scene_layout == upload.layout
        && pipeline.modules.len() == 2
        && !pipeline.modules.contains(&0)
        && pipeline.program == "Render"
}

fn validate_sources(pipeline: &RayPipelineInputs) -> Result<(), RenderError> {
    let expected: Vec<[u64; 3]> = (0..10).map(|id| [id, 4 * id, 4]).collect();
    if pipeline.specialization_entries != expected
        || pipeline.specialization.len() != 40
        || !valid_spirv(&pipeline.vertex)
        || !valid_spirv(&pipeline.fragment)
    {
        return Err(rejected(
            (
                pipeline.specialization.len(),
                pipeline.vertex.len(),
                pipeline.fragment.len(),
            ),
            "ten actual specialization words and two bounded SPIR-V modules",
        ));
    }
    Ok(())
}
pub(super) fn valid_spirv(bytes: &[u8]) -> bool {
    bytes.len() >= 20
        && bytes.len() <= 1024 * 1024
        && bytes.len() % 4 == 0
        && bytes[..4] == 0x07230203u32.to_le_bytes()
}
fn identity_json(token: &RayInputObservation, serial: u64) -> String {
    format!(
        "\"schema\":\"martensite-raytrace-upload-v1\",\"authority\":\"uploaded-inputs-and-recorded-commands\",\"post_vertex_observed\":false,\"view_address\":{},\"view_id\":{},\"frame_generation\":{},\"accepted_publication\":{}",
        token.owner, token.view_id, token.generation, serial
    )
}
fn frame_json(command: &RayCommandInputs) -> String {
    format!(
        "\"command\":{},\"logical_extent\":{:?},\"attachment_extent\":{:?},\"draw\":[6,1,0,0],\"push_bytes\":[],\"viewport_authority\":\"join-command-state-viewport-records\"",
        command.command, command.logical, command.attachment
    )
}
fn upload_json(upload: &RayUploadInputs, dispatch: &RayDispatchInputs) -> String {
    format!(
        "\"uniform_buffer\":{},\"descriptor_set_layout\":{},\"uniform_binding\":0,\"uniform_offset\":0,\"uniform_descriptor_range\":\"VK_WHOLE_SIZE\",\"uniform_bytes\":{:?},\"descriptor_sets\":{:?},\"native_little_endian\":{}",
        upload.buffer,
        upload.layout,
        upload.bytes,
        dispatch.sets,
        cfg!(target_endian = "little")
    )
}
fn pipeline_json(pipeline: &RayPipelineInputs) -> String {
    format!(
        "\"pipeline\":{},\"layout\":{},\"program\":\"{}\",\"modules\":{:?},\"specialization_entries_id_offset_size\":{:?},\"specialization_bytes\":{:?}",
        pipeline.pipeline,
        pipeline.layout,
        pipeline.program,
        pipeline.modules,
        pipeline.specialization_entries,
        pipeline.specialization
    )
}
fn sources_json(pipeline: &RayPipelineInputs) -> String {
    format!(
        "\"vertex_spirv_bytes\":{:?},\"fragment_spirv_bytes\":{:?}",
        pipeline.vertex, pipeline.fragment
    )
}
