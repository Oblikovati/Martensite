use super::*;
use martensite_tkernel::tcollection::HAsciiString;

fn label() -> (
    Graphic3dText,
    Handle<dyn Graphic3dAspectsSource>,
    Graphic3dRenderingParams,
) {
    let text = Graphic3dText::new(16.);
    text.set_text_str("label");
    let aspects: Handle<dyn Graphic3dAspectsSource> =
        Handle::new(Graphic3dAspectText3d::default()).cast();
    (text, aspects, Graphic3dRenderingParams::new().unwrap())
}

fn kept(slot: &LabelSlot, inputs: LabelInputs) {
    slot.keep(
        inputs,
        LabelReplay {
            point_size: 16,
            upload: (6, false, Default::default()),
            draws: Vec::new(),
        },
    );
}

#[test]
fn an_unchanged_label_replays_its_record() {
    let (text, aspects, params) = label();
    let slot = LabelSlot::default();
    assert!(slot.unchanged(&text, &aspects, &params).is_none());
    kept(
        &slot,
        LabelInputs::current(&text, &aspects, &params).unwrap(),
    );
    let replay = slot.unchanged(&text, &aspects, &params).unwrap();
    assert_eq!((replay.point_size, replay.upload.0), (16, 6));
}

#[test]
fn a_text_aspects_font_or_resolution_change_drops_the_record() {
    let (text, aspects, params) = label();
    let slot = LabelSlot::default();
    let inputs = LabelInputs::current(&text, &aspects, &params).unwrap();
    kept(&slot, inputs.clone());
    text.set_text_str("other");
    assert!(
        slot.unchanged(&text, &aspects, &params).is_none(),
        "text edit"
    );
    let (text, aspects, mut params) = label();
    kept(
        &slot,
        LabelInputs::current(&text, &aspects, &params).unwrap(),
    );
    let serif = HAsciiString::from(martensite_tkernel::tcollection::AsciiString::from("Serif"));
    aspects.get().unwrap().set_text_font(Handle::new(serif));
    assert!(
        slot.unchanged(&text, &aspects, &params).is_none(),
        "aspects edit"
    );
    kept(
        &slot,
        LabelInputs::current(&text, &aspects, &params).unwrap(),
    );
    params.resolution += 1;
    assert!(
        slot.unchanged(&text, &aspects, &params).is_none(),
        "resolution"
    );
}

#[test]
fn inputs_read_before_an_edit_leave_a_stale_record() {
    let (text, aspects, params) = label();
    let slot = LabelSlot::default();
    let before = LabelInputs::current(&text, &aspects, &params).unwrap();
    text.set_height(20.);
    kept(&slot, before);
    assert!(slot.unchanged(&text, &aspects, &params).is_none());
}
