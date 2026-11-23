//! Per-pass record state: the shared camera parts reproduce the per-group functions bit for
//! bit, and the parameter comparison sees a change in every kind of field.
use super::*;
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;

fn moved_camera() -> Graphic3dCamera {
    let camera = Graphic3dCamera::default();
    camera.set_eye(&Point3::new(3., -2., 5.)).unwrap();
    camera
}

#[test]
fn camera_parts_compose_the_clip_transform_of_ordinary_clip() {
    let camera = moved_camera();
    let frame = RecordFrame::default();
    let parts = frame.camera_parts(&camera).unwrap();
    let model = Mat4::<f64>::identity();
    let clip = (parts.projection_view * Mat4::<f32>::from_other(&model)).data();
    let expected = crate::persistent_transform::ordinary_clip(&camera, model);
    assert_eq!(clip.map(f32::to_bits), expected.map(f32::to_bits));
    assert!(parts.unit_axial_scale);
    assert!(parts.factors.is_some_and(|f| f.has_camera_isometry()));
}

#[test]
fn camera_parts_are_read_once_per_pass_for_its_camera() {
    let camera = moved_camera();
    let frame = RecordFrame::default();
    let first = *frame.camera_parts(&camera).unwrap();
    camera.set_eye(&Point3::new(-4., 1., 2.)).unwrap();
    let kept = frame.camera_parts(&camera).unwrap();
    assert_eq!(first.view.data(), kept.view.data(), "one read per pass");
    // The groups of a second camera in the same pass lower instead of replaying.
    assert!(frame.camera_parts(&moved_camera()).is_none());
}

#[test]
fn same_rendering_sees_float_integer_flag_and_handle_changes() {
    let base = Graphic3dRenderingParams::new().unwrap();
    assert!(same_rendering(&base, &base.clone()));
    let edits: [fn(&mut Graphic3dRenderingParams); 4] = [
        |p| p.line_feather += 0.5,
        |p| p.nb_msaa_samples = 4,
        |p| p.to_enable_depth_prepass = !p.to_enable_depth_prepass,
        |p| p.stats.stats_position = Handle::default(),
    ];
    for edit in edits {
        let mut changed = base.clone();
        edit(&mut changed);
        assert!(!same_rendering(&base, &changed));
    }
}

#[test]
fn records_of_one_pass_share_parameters_and_reuse_a_match() {
    let params = Graphic3dRenderingParams::new().unwrap();
    let frame = RecordFrame::default();
    let shared = frame.recorded_rendering(&params);
    assert!(Arc::ptr_eq(&shared, &frame.recorded_rendering(&params)));
    assert!(frame.rendering_matches(&shared, &params));
    let mut other = params.clone();
    other.nb_msaa_samples = 8;
    assert!(!frame.rendering_matches(&shared, &other));
}
