//! Selection regressions of the `vertex_edge` grid: picking a box by cursor and by area.
use super::prelude::*;
/// `vertex_edge/B1`: the selection regression's commands and image dumps in order.
pub(crate) fn vertex_edge_b1() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(1, true)?;
    run.session().move_cursor(102, 204)?;
    run.session().set_selection_mode(2, true)?;
    run.session().move_cursor(379, 204)?;
    run.session().move_cursor(26, 305)?;
    run.dump("B1_1.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_2.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.817216, -0.217801, 0.535373]),
        up: Some([-0.420838, 0.411627, 0.808368]),
        at: Some([26.4654518144607, 56.2503832914726, 34.5651685621603]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(92, 231)?;
    run.session().move_cursor(73, 302)?;
    run.dump("B1_3.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_4.png")?;
    run.session().select_point(73, 302)?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_5.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.524772, 0.731256, 0.434393]),
        up: Some([0.716008, -0.104342, -0.691426]),
        at: Some([21.6212460112894, 0.5192504580656, 16.591446657356]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(144, 350)?;
    run.session().move_cursor(161, 197)?;
    run.dump("B1_6.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_7.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_8.png")?;
    run.session().move_cursor(161, 197)?;
    run.dump("B1_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1.png")?;
    run.finish()
}

/// `vertex_edge/F2`: the selection regression's commands and image dumps in order.
pub(crate) fn vertex_edge_f2() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(1, true)?;
    run.session().move_cursor(102, 204)?;
    run.session().set_selection_mode(2, true)?;
    run.session().move_cursor(379, 204)?;
    run.session().move_cursor(26, 305)?;
    run.session().select_point(26, 305)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_1.png")?;
    run.session().move_cursor(204, 409)?;
    run.dump("F2_2.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_3.png")?;
    run.session()
        .select_polyline(&[(204, 409)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_4.png")?;
    run.session().move_cursor(26, 305)?;
    run.dump("F2_5.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_6.png")?;
    run.session().move_cursor(204, 409)?;
    run.dump("F2_7.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_8.png")?;
    run.session().move_cursor(27, 104)?;
    run.dump("F2_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_10.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.817216, -0.217801, 0.535373]),
        up: Some([-0.420838, 0.411627, 0.808368]),
        at: Some([26.4654518144607, 56.2503832914726, 34.5651685621603]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(92, 231)?;
    run.session().move_cursor(73, 302)?;
    run.dump("F2_11.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_12.png")?;
    run.session().move_cursor(173, 409)?;
    run.dump("F2_13.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_14.png")?;
    run.session().move_cursor(13, 102)?;
    run.dump("F2_15.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_16.png")?;
    run.session()
        .select_polyline(&[(73, 302)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_17.png")?;
    run.session().move_cursor(73, 302)?;
    run.dump("F2_18.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_19.png")?;
    run.session().move_cursor(173, 409)?;
    run.dump("F2_20.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_21.png")?;
    run.session().move_cursor(13, 102)?;
    run.dump("F2_22.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_23.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.524772, 0.731256, 0.434393]),
        up: Some([0.716008, -0.104342, -0.691426]),
        at: Some([21.6212460112894, 0.5192504580656, 16.591446657356]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(144, 350)?;
    run.session().move_cursor(161, 197)?;
    run.dump("F2_24.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_25.png")?;
    run.session().move_cursor(272, 22)?;
    run.dump("F2_26.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_27.png")?;
    run.session().move_cursor(299, 360)?;
    run.dump("F2_28.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_29.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_30.png")?;
    run.session().move_cursor(272, 22)?;
    run.dump("F2_31.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_32.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([-0.444872, -0.214876, 0.86866]),
        up: Some([0.109494, -0.886324, -0.185966]),
        at: Some([71.8115798514333, 53.1349971091326, 8.4539251074103]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(96, 211)?;
    run.session().move_cursor(141, 11)?;
    run.dump("F2_33.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_34.png")?;
    run.session().select_point(406, 44)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_35.png")?;
    run.session().select_point(141, 11)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_36.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2.png")?;
    run.finish()
}

/// `vertex_edge/G3`: the selection regression's commands and image dumps in order.
pub(crate) fn vertex_edge_g3() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(1, true)?;
    run.session().move_cursor(102, 204)?;
    run.session().set_selection_mode(2, true)?;
    run.session().move_cursor(379, 204)?;
    run.session().select_point(379, 204)?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(26, 305)?;
    run.dump("G3_1.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3_2.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.817216, -0.217801, 0.535373]),
        up: Some([-0.420838, 0.411627, 0.808368]),
        at: Some([26.4654518144607, 56.2503832914726, 34.5651685621603]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(92, 231)?;
    run.session().move_cursor(73, 302)?;
    run.dump("G3_3.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3_4.png")?;
    run.session()
        .select_polyline(&[(73, 302)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3_5.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063093),
        proj: Some([0.524772, 0.731256, 0.434393]),
        up: Some([0.716008, -0.104342, -0.691426]),
        at: Some([21.6212460112894, 0.5192504580656, 16.591446657356]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(144, 350)?;
    run.session().move_cursor(161, 197)?;
    run.dump("G3_6.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3_7.png")?;
    run.session()
        .select_polyline(&[(161, 197)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3_8.png")?;
    run.session().move_cursor(161, 197)?;
    run.dump("G3_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3.png")?;
    run.finish()
}

/// `vertex_edge/K4`: the selection regression's commands and image dumps in order.
pub(crate) fn vertex_edge_k4() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(1, true)?;
    run.session().move_cursor(102, 204)?;
    run.session().set_selection_mode(2, true)?;
    run.session().move_cursor(379, 204)?;
    run.session().select_point(379, 204)?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(26, 305)?;
    run.session()
        .select_polyline(&[(26, 305)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_1.png")?;
    run.session().move_cursor(204, 409)?;
    run.dump("K4_2.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_3.png")?;
    run.session()
        .select_polyline(&[(204, 409)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_4.png")?;
    run.session().move_cursor(26, 305)?;
    run.dump("K4_5.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_6.png")?;
    run.session().move_cursor(204, 409)?;
    run.dump("K4_7.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_8.png")?;
    run.session().move_cursor(27, 104)?;
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
    run.session().move_cursor(92, 231)?;
    run.session().move_cursor(73, 302)?;
    run.dump("K4_11.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_12.png")?;
    run.session().move_cursor(173, 409)?;
    run.dump("K4_13.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_14.png")?;
    run.session().move_cursor(13, 102)?;
    run.dump("K4_15.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_16.png")?;
    run.session()
        .select_polyline(&[(73, 302)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_17.png")?;
    run.session().move_cursor(73, 302)?;
    run.dump("K4_18.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_19.png")?;
    run.session().move_cursor(173, 409)?;
    run.dump("K4_20.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_21.png")?;
    run.session().move_cursor(13, 102)?;
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
    run.session().move_cursor(144, 350)?;
    run.session().move_cursor(161, 197)?;
    run.dump("K4_24.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_25.png")?;
    run.session().move_cursor(272, 22)?;
    run.dump("K4_26.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_27.png")?;
    run.session().move_cursor(299, 360)?;
    run.dump("K4_28.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_29.png")?;
    run.session()
        .select_polyline(&[(272, 22)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_30.png")?;
    run.session().move_cursor(272, 22)?;
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
    run.session().move_cursor(96, 211)?;
    run.session().move_cursor(141, 11)?;
    run.dump("K4_33.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_34.png")?;
    run.session()
        .select_polyline(&[(406, 44)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_35.png")?;
    run.session()
        .select_polyline(&[(406, 44)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.session()
        .select_polyline(&[(141, 11)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4_36.png")?;
    run.session()
        .select_polyline(&[(140, 11)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("K4.png")?;
    run.finish()
}
