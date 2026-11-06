use super::*;
use martensite_tkernel::{quantity::ColorRgba, standard::Handle};
use martensite_tkmath::bvh::BvhVec3d;

fn structure_fields(method: AspectTypeOfHighlightMethod) -> Graphic3dCStructureFields {
    let manager = Handle::new(
        Graphic3dStructureManager::new(
            &martensite_tkservice::null_driver::NullGraphicDriver::new_handle(),
        )
        .unwrap(),
    );
    let mut fields = Graphic3dCStructureBase::new(&manager)
        .unwrap()
        .fields
        .read()
        .clone();
    fields.highlight = true;
    let style = Handle::new(Graphic3dPresentationAttributes::default());
    style.set_method(method);
    fields.highlight_style = style.cast();
    fields
        .highlight_style
        .set_color(*ColorRgba::new(0., 1., 1., 1.).unwrap().rgb());
    let mut bounds = Graphic3dBndBox3d::default();
    bounds.add(&BvhVec3d::new(-1., 0., 2.));
    bounds.add(&BvhVec3d::new(3., 1., 2.5));
    fields.bnd_box = bounds;
    fields
}

#[test]
fn unit_box_lines_follow_the_native_vertex_buffer() {
    let lines = unit_box_lines();
    // anAxis 0, aCompIter 0: aMin and aMin + X; aCompIter 3 shifts both Y and Z.
    assert_eq!(lines[0], [-0.5, -0.5, -0.5]);
    assert_eq!(lines[1], [0.5, -0.5, -0.5]);
    assert_eq!(lines[6], [-0.5, 0.5, 0.5]);
    // anAxis 1 lifts Z with aLookup1 and X with aLookup2.
    assert_eq!(lines[10], [-0.5, -0.5, 0.5]);
    assert_eq!(lines[11], [-0.5, 0.5, 0.5]);
    // Every one of the 12 edges appears once and joins corners differing in one axis.
    for pair in lines.as_chunks::<2>().0 {
        let differing = (0..3).filter(|&i| pair[0][i] != pair[1][i]).count();
        assert_eq!(differing, 1, "{pair:?}");
    }
}

#[test]
fn the_box_spans_the_world_bounds_in_the_highlight_color() {
    let fields = structure_fields(AspectTypeOfHighlightMethod::Boundbox);
    let camera = Graphic3dCamera::default();
    let draw = bound_box_draw(&fields, &camera, Some([64, 64]))
        .unwrap()
        .unwrap();
    assert_eq!(draw.topology(), crate::PrimitiveTopology::Lines);
    assert_eq!(draw.vertices().len(), 24);
    assert_eq!(draw.vertices()[0].position, [-1., 0., 2.]);
    assert_eq!(draw.vertices()[1].position, [3., 0., 2.]);
    assert!(draw.vertices().iter().all(|v| v.color == [0., 1., 1., 1.]));
}

#[test]
fn color_highlight_and_an_invalid_box_are_not_drawn_as_a_box() {
    let camera = Graphic3dCamera::default();
    let color = structure_fields(AspectTypeOfHighlightMethod::Color);
    assert!(!is_bound_box_highlight(&color));
    assert!(
        bound_box_draw(&color, &camera, Some([64, 64]))
            .unwrap()
            .is_none()
    );
    let mut invalid = structure_fields(AspectTypeOfHighlightMethod::Boundbox);
    invalid.bnd_box = Graphic3dBndBox3d::default();
    assert!(
        bound_box_draw(&invalid, &camera, Some([64, 64]))
            .unwrap()
            .is_none()
    );
}

#[test]
fn a_persistent_structure_box_keeps_the_persistence_and_clamps_camera_depth() {
    let camera = Graphic3dCamera::default();
    let plain = structure_fields(AspectTypeOfHighlightMethod::Boundbox);
    let plain = bound_box_draw(&plain, &camera, Some([64, 64]))
        .unwrap()
        .unwrap();
    for (mode, clamp) in [
        (Graphic3dTransModeFlags::ZOOM_PERS, false),
        (Graphic3dTransModeFlags::CAMERA_PERS, true),
    ] {
        let mut persistent = structure_fields(AspectTypeOfHighlightMethod::Boundbox);
        let persistence = Graphic3dTransformPers::new(mode).unwrap();
        persistent.trsf_pers = Handle::new(persistence).cast();
        let draw = bound_box_draw(&persistent, &camera, Some([64, 64]))
            .unwrap()
            .unwrap();
        assert_eq!(draw.depth_clamp, clamp, "{mode:?}");
        assert_ne!(draw.clip_transform(), plain.clip_transform(), "{mode:?}");
    }
}

#[test]
fn a_transparent_highlight_color_is_rejected() {
    let fields = structure_fields(AspectTypeOfHighlightMethod::Boundbox);
    fields.highlight_style.set_transparency(0.5);
    let failure = bound_box_draw(&fields, &Graphic3dCamera::default(), Some([64, 64])).unwrap_err();
    assert!(failure.to_string().contains("opaque bounding-box"));
}
