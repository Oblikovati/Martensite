//! Selection regressions of the `edge_solid` grid: picking a box by cursor and by area.
use super::prelude::*;
/// `edge_solid/B1`: the selection regression's commands and image dumps in order.
pub(crate) fn edge_solid_b1() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(2, true)?;
    run.session().set_selection_mode(6, true)?;
    run.session().move_cursor(110, 352)?;
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
    run.session().move_cursor(120, 350)?;
    run.dump("B1_3.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_4.png")?;
    run.session().select_point(120, 350)?;
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
    run.session().move_cursor(225, 98)?;
    run.dump("B1_6.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_7.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_8.png")?;
    run.session().move_cursor(225, 98)?;
    run.dump("B1_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1.png")?;
    run.finish()
}

/// `edge_solid/F2`: the selection regression's commands and image dumps in order.
pub(crate) fn edge_solid_f2() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(2, true)?;
    run.session().set_selection_mode(6, true)?;
    run.session().move_cursor(110, 352)?;
    run.session().select_point(110, 352)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_1.png")?;
    run.session().move_cursor(29, 204)?;
    run.dump("F2_2.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_3.png")?;
    run.session()
        .select_polyline(&[(29, 204)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_4.png")?;
    run.session().move_cursor(110, 352)?;
    run.dump("F2_5.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_6.png")?;
    run.session().move_cursor(29, 204)?;
    run.dump("F2_7.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_8.png")?;
    run.session().move_cursor(204, 306)?;
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
    run.session().move_cursor(120, 350)?;
    run.dump("F2_11.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_12.png")?;
    run.session().move_cursor(50, 220)?;
    run.dump("F2_13.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_14.png")?;
    run.session().move_cursor(150, 330)?;
    run.dump("F2_15.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_16.png")?;
    run.session()
        .select_polyline(&[(120, 350)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_17.png")?;
    run.session().move_cursor(120, 350)?;
    run.dump("F2_18.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_19.png")?;
    run.session().move_cursor(50, 220)?;
    run.dump("F2_20.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_21.png")?;
    run.session().move_cursor(150, 330)?;
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
    run.session().move_cursor(225, 98)?;
    run.dump("F2_24.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_25.png")?;
    run.session().move_cursor(270, 326)?;
    run.dump("F2_26.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_27.png")?;
    run.session().move_cursor(342, 111)?;
    run.dump("F2_28.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_29.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_30.png")?;
    run.session().move_cursor(270, 326)?;
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
    run.session().move_cursor(310, 87)?;
    run.dump("F2_33.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_34.png")?;
    run.session().select_point(237, 100)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_35.png")?;
    run.session().select_point(310, 87)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_36.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2.png")?;
    run.finish()
}

/// `edge_solid/G3`: the selection regression's commands and image dumps in order.
pub(crate) fn edge_solid_g3() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(2, true)?;
    run.session().set_selection_mode(6, true)?;
    run.session().move_cursor(260, 110)?;
    run.session().move_cursor(110, 352)?;
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
    run.session().move_cursor(120, 350)?;
    run.dump("G3_3.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3_4.png")?;
    run.session().select_point(120, 350)?;
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
    run.session().move_cursor(225, 98)?;
    run.dump("G3_6.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3_7.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3_8.png")?;
    run.session().move_cursor(225, 98)?;
    run.dump("G3_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("G3.png")?;
    run.finish()
}
