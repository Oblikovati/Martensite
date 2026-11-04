//! Selection regressions of the `edge_face` grid, second part: picking a box by cursor and by area.
use super::prelude::*;

/// `edge_face/K4`: the selection regression's commands and image dumps in order.
pub(crate) fn edge_face_k4() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(2, true)?;
    run.session().set_selection_mode(4, true)?;
    run.session().move_cursor(260, 110)?;
    run.session().move_cursor(110, 352)?;
    run.session().select_point(110, 352)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_1.png")?;
    run.session().move_cursor(29, 204)?;
    run.dump("K4_2.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_3.png")?;
    run.session()
        .select_polyline(&[(29, 204)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_4.png")?;
    run.session().move_cursor(110, 352)?;
    run.dump("K4_5.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_6.png")?;
    run.session().move_cursor(29, 204)?;
    run.dump("K4_7.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_8.png")?;
    run.session().move_cursor(204, 306)?;
    run.dump("K4_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_10.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.817216, -0.217801, 0.535373]),
        up: Some([-0.420838, 0.411627, 0.808368]),
        at: Some([26.4654518144607, 56.2503832914726, 34.5651685621603]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(120, 350)?;
    run.dump("K4_11.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_12.png")?;
    run.session().move_cursor(50, 220)?;
    run.dump("K4_13.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_14.png")?;
    run.session().move_cursor(150, 330)?;
    run.dump("K4_15.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_16.png")?;
    run.session()
        .select_polyline(&[(120, 350)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_17.png")?;
    run.session().move_cursor(120, 350)?;
    run.dump("K4_18.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_19.png")?;
    run.session().move_cursor(50, 220)?;
    run.dump("K4_20.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_21.png")?;
    run.session().move_cursor(150, 330)?;
    run.dump("K4_22.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_23.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.524772, 0.731256, 0.434393]),
        up: Some([0.716008, -0.104342, -0.691426]),
        at: Some([21.6212460112894, 0.5192504580656, 16.591446657356]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(225, 98)?;
    run.dump("K4_24.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_25.png")?;
    run.session().move_cursor(270, 326)?;
    run.dump("K4_26.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_27.png")?;
    run.session().move_cursor(342, 111)?;
    run.dump("K4_28.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_29.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_30.png")?;
    run.session().move_cursor(270, 326)?;
    run.dump("K4_31.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_32.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([-0.444872, -0.214876, 0.86866]),
        up: Some([0.109494, -0.886324, -0.185966]),
        at: Some([71.8115798514333, 53.1349971091326, 8.4539251074103]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(310, 87)?;
    run.dump("K4_33.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_34.png")?;
    run.session().select_point(237, 95)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_35.png")?;
    run.session().select_point(310, 87)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_36.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4.png")?;
    run.finish()
}

/// `edge_face/L5`: the selection regression's commands and image dumps in order.
pub(crate) fn edge_face_l5() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(2, true)?;
    run.session().set_selection_mode(4, true)?;
    run.session().move_cursor(260, 110)?;
    run.session().select_point(260, 110)?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(110, 352)?;
    run.dump("L5_1.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("L5_2.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.817216, -0.217801, 0.535373]),
        up: Some([-0.420838, 0.411627, 0.808368]),
        at: Some([26.4654518144607, 56.2503832914726, 34.5651685621603]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(120, 350)?;
    run.dump("L5_3.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("L5_4.png")?;
    run.session()
        .select_polyline(&[(120, 350)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("L5_5.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.524772, 0.731256, 0.434393]),
        up: Some([0.716008, -0.104342, -0.691426]),
        at: Some([21.6212460112894, 0.5192504580656, 16.591446657356]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(225, 98)?;
    run.dump("L5_6.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("L5_7.png")?;
    run.session()
        .select_polyline(&[(225, 98)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("L5_8.png")?;
    run.session().move_cursor(225, 98)?;
    run.dump("L5_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("L5.png")?;
    run.finish()
}

/// `edge_face/P6`: the selection regression's commands and image dumps in order.
pub(crate) fn edge_face_p6() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(2, true)?;
    run.session().set_selection_mode(4, true)?;
    run.session().move_cursor(260, 110)?;
    run.session().select_point(260, 110)?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(110, 352)?;
    run.session()
        .select_polyline(&[(110, 352)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_1.png")?;
    run.session().move_cursor(29, 204)?;
    run.dump("P6_2.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_3.png")?;
    run.session()
        .select_polyline(&[(29, 204)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_4.png")?;
    run.session().move_cursor(110, 352)?;
    run.dump("P6_5.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_6.png")?;
    run.session().move_cursor(29, 204)?;
    run.dump("P6_7.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_8.png")?;
    run.session().move_cursor(204, 306)?;
    run.dump("P6_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_10.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.817216, -0.217801, 0.535373]),
        up: Some([-0.420838, 0.411627, 0.808368]),
        at: Some([26.4654518144607, 56.2503832914726, 34.5651685621603]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(120, 350)?;
    run.dump("P6_11.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_12.png")?;
    run.session().move_cursor(50, 220)?;
    run.dump("P6_13.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_14.png")?;
    run.session().move_cursor(150, 330)?;
    run.dump("P6_15.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_16.png")?;
    run.session()
        .select_polyline(&[(120, 350)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_17.png")?;
    run.session().move_cursor(120, 350)?;
    run.dump("P6_18.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_19.png")?;
    run.session().move_cursor(50, 220)?;
    run.dump("P6_20.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_21.png")?;
    run.session().move_cursor(150, 330)?;
    run.dump("P6_22.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_23.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.524772, 0.731256, 0.434393]),
        up: Some([0.716008, -0.104342, -0.691426]),
        at: Some([21.6212460112894, 0.5192504580656, 16.591446657356]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(225, 98)?;
    run.dump("P6_24.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_25.png")?;
    run.session().move_cursor(270, 326)?;
    run.dump("P6_26.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_27.png")?;
    run.session().move_cursor(342, 111)?;
    run.dump("P6_28.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_29.png")?;
    run.session()
        .select_polyline(&[(270, 326)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_30.png")?;
    run.session().move_cursor(270, 326)?;
    run.dump("P6_31.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_32.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([-0.444872, -0.214876, 0.86866]),
        up: Some([0.109494, -0.886324, -0.185966]),
        at: Some([71.8115798514333, 53.1349971091326, 8.4539251074103]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(310, 87)?;
    run.dump("P6_33.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_34.png")?;
    run.session()
        .select_polyline(&[(237, 99)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_35.png")?;
    run.session()
        .select_polyline(&[(237, 99)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.session()
        .select_polyline(&[(310, 87)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6_36.png")?;
    run.session()
        .select_polyline(&[(310, 87)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("P6.png")?;
    run.finish()
}
