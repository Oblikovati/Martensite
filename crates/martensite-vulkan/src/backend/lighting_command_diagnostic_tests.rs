use super::*;

#[test]
fn user_programs_never_claim_the_stock_lighting_descriptor_was_bound() {
    assert_eq!(binding_status(true, true), "unknown-user-program");
    assert_eq!(binding_status(true, false), "unknown-user-program");
    assert_eq!(binding_status(false, false), "absent");
    assert_eq!(binding_status(false, true), "pending-stock-bind");
}

#[test]
fn uploaded_bytes_preserve_negative_zero_nan_and_every_source_byte() {
    let bytes = [0x8000_0000u32, 0x7fc0_0123, 0x3fa0_0000]
        .into_iter()
        .flat_map(u32::to_ne_bytes)
        .collect::<Vec<_>>();
    let observation = DescriptorArguments {
        set: 71,
        buffer: 91,
        bytes: &bytes,
    };
    let expected = format!(
        "{{\"set\":71,\"buffer\":91,\"binding\":0,\"byte_count\":12,\"uploaded_bytes\":{:?}}}",
        bytes
    );
    assert_eq!(optional_json(Some(&observation)), expected);
    assert_eq!(optional_json(None), "null");
}

#[test]
fn descriptor_binding_joins_actual_command_pipeline_layout_and_set() {
    let json = binding_json(
        (
            vk::CommandBuffer::from_raw(3),
            vk::Pipeline::from_raw(7),
            vk::PipelineLayout::from_raw(11),
        ),
        vk::DescriptorSet::from_raw(71),
    );
    assert_eq!(
        json,
        "{\"kind\":\"draw-state-lighting-bound\",\"command\":3,\"pipeline\":7,\"pipeline_layout\":11,\"set_index\":0,\"set\":71}"
    );
}

#[test]
fn actual_encoded_ambient_and_source_count_keep_their_independent_byte_offsets() {
    let bytes = ambient_state().encode();
    assert_eq!(bytes.len(), 304);
    let ambient = [0.25f32, -0.0, 0.75, 1.]
        .into_iter()
        .flat_map(f32::to_ne_bytes)
        .collect::<Vec<_>>();
    assert_eq!(&bytes[144..160], ambient);
    assert_eq!(&bytes[288..304], &[0u8; 16]);
}

fn ambient_state() -> crate::RenderLighting {
    let mut state = crate::RenderLighting::new(
        vec![[0., 0., 1.]; 4],
        crate::RenderMaterial::default(),
        crate::identity_clip(),
        crate::identity_clip(),
        false,
        crate::RenderShading::PhongFacet,
    )
    .unwrap();
    state.lights.ambient = [0.25, -0.0, 0.75, 1.];
    state
}

#[test]
fn equal_lighting_bytes_cannot_hide_a_changed_descriptor_binding() {
    let bytes = [1, 0, 255];
    let first = DescriptorArguments {
        set: 71,
        buffer: 91,
        bytes: &bytes,
    };
    let second = DescriptorArguments {
        set: 72,
        buffer: 92,
        bytes: &bytes,
    };
    assert_ne!(first.json(), second.json());
    assert_eq!(first.bytes, second.bytes);
}
