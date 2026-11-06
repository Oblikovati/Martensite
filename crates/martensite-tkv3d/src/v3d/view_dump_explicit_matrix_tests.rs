//! Actual CPU transactions expose argument forwarding and exceptional cleanup to native comparison.
use super::{
    V3dLightName, V3dStereoDumpOptions, V3dView, V3dViewer,
    light_name_callback_fixture::CallbackNameStorage, light_name_callback_tests::callback_view,
};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkservice::{
    graphic3d::{Graphic3dBufferType, Graphic3dCViewMembers as _, Graphic3dCameraProjection},
    image::ImagePixMap,
    null_driver::NullGraphicDriver,
};
type ExplicitDumpCase = (i32, V3dStereoDumpOptions, usize);

#[test]
fn native_full_overload_matrix_records_real_cpu_transactions() {
    use V3dStereoDumpOptions::{Blended, LeftEye, Mono, RightEye};
    let mut records = Vec::new();
    for flag in [i32::MIN, -1, 0, 1, i32::MAX] {
        for stereo in [Mono, LeftEye, RightEye, Blended] {
            records.extend(capture_explicit_names(flag, stereo));
        }
    }
    for record in &records {
        println!("{record}");
    }
    assert_eq!(
        records.join("\n") + "\n",
        include_str!("../../tests/fixtures/pixmap-full-overload-native.jsonl")
    );
}

fn capture_explicit_names(flag: i32, stereo: V3dStereoDumpOptions) -> Vec<String> {
    let bytes = [0xff, 0x80, 0];
    let text = std::ffi::CStr::from_bytes_with_nul(&bytes).unwrap();
    let names = [
        V3dLightName::from_c_str(c"sun"),
        V3dLightName::from_c_str(c""),
        V3dLightName::from_c_str(text),
        V3dLightName::null(),
        V3dLightName::null(),
    ];
    names
        .into_iter()
        .enumerate()
        .map(|(index, name)| capture_explicit_transaction((flag, stereo, index), name))
        .collect()
}

fn capture_explicit_transaction(case: ExplicitDumpCase, name: V3dLightName<'_>) -> String {
    let control = CallbackNameStorage::new();
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let view = callback_view(&viewer, &control);
    view.camera()
        .set_projection_type(Graphic3dCameraProjection::Stereo);
    let image = ImagePixMap::default();
    let fbo = view.view_base().view().fbo();
    let result = invoke_explicit_transaction(&view, &image, case, name);
    explicit_transaction_row(case, &view, &image, result, &fbo, &control.rows.read())
}

fn invoke_explicit_transaction(
    view: &V3dView,
    image: &ImagePixMap,
    case: ExplicitDumpCase,
    name: V3dLightName<'_>,
) -> Result<bool, Failure> {
    let buffer = if case.2 == 4 {
        Graphic3dBufferType::Rgb
    } else {
        Graphic3dBufferType::ShadowMap
    };
    view.to_pix_map_full(image, 8, 8, buffer, false, 42, case.0, case.1, name)
}

fn explicit_transaction_row(
    case: ExplicitDumpCase,
    view: &V3dView,
    image: &ImagePixMap,
    result: Result<bool, Failure>,
    fbo: &Handle<StandardTransient>,
    consumed: &[String],
) -> String {
    let outcome = explicit_outcome(result);
    let name = explicit_consumed_json(case, consumed);
    let state = explicit_transaction_state(view, image, fbo);
    format!(
        "{{\"flag\":{},\"stereo\":{},\"name\":{},\"outcome\":{outcome:?},\"state\":{state},\"consumed\":{name}}}",
        case.0, case.1 as i32, case.2,
    )
}

fn explicit_outcome(result: Result<bool, Failure>) -> String {
    result.map_or_else(
        |failure| failure.exception_type().to_owned(),
        |value| value.to_string(),
    )
}

fn explicit_consumed_json(case: ExplicitDumpCase, consumed: &[String]) -> String {
    assert_eq!(consumed.len(), usize::from(case.2 < 3));
    consumed
        .first()
        .map_or("null".to_owned(), |row| format!("{row:?}"))
}

fn explicit_transaction_state(
    view: &V3dView,
    image: &ImagePixMap,
    fbo: &Handle<StandardTransient>,
) -> String {
    let driver = view.view();
    format!(
        "{{\"layer\":{},\"single\":{},\"projection\":{},\"width\":{},\"height\":{},\"fbo_same\":{}}}",
        driver.z_layer_target(),
        driver.z_layer_redraw_mode(),
        view.camera().projection_type() as i32,
        image.width(),
        image.height(),
        driver.fbo().object_address() == fbo.object_address(),
    )
}
