//! Selection regressions of the `wire` grid: picking a box by cursor and by area.
use super::prelude::*;
/// `wire/B1`: the selection regression's commands and image dumps in order.
pub(crate) fn wire_b1() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(3, true)?;
    run.session().move_cursor(140, 100)?;
    run.session().move_cursor(100, 346)?;
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
    run.session().move_cursor(140, 100)?;
    run.session().move_cursor(100, 329)?;
    run.dump("B1_3.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_4.png")?;
    run.session().select_point(100, 329)?;
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
    run.session().move_cursor(300, 300)?;
    run.session().move_cursor(203, 129)?;
    run.dump("B1_6.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_7.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_8.png")?;
    run.session().move_cursor(203, 129)?;
    run.dump("B1_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.finish()
}

/// `wire/F2`: the selection regression's commands and image dumps in order.
pub(crate) fn wire_f2() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(3, true)?;
    run.session().move_cursor(140, 100)?;
    run.session().move_cursor(100, 346)?;
    run.session().select_point(100, 346)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_1.png")?;
    run.session().move_cursor(130, 247)?;
    run.dump("F2_2.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_3.png")?;
    run.session()
        .select_polyline(&[(130, 247)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_4.png")?;
    run.session().move_cursor(100, 346)?;
    run.dump("F2_5.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_6.png")?;
    run.session().move_cursor(130, 247)?;
    run.dump("F2_7.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_8.png")?;
    run.session().move_cursor(300, 351)?;
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
    run.session().move_cursor(140, 100)?;
    run.session().move_cursor(100, 329)?;
    run.dump("F2_11.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_12.png")?;
    run.session().move_cursor(200, 245)?;
    run.dump("F2_13.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_14.png")?;
    run.session().move_cursor(300, 349)?;
    run.dump("F2_15.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_16.png")?;
    run.session()
        .select_polyline(&[(100, 329)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_17.png")?;
    run.session().move_cursor(100, 329)?;
    run.dump("F2_18.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_19.png")?;
    run.session().move_cursor(200, 245)?;
    run.dump("F2_20.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_21.png")?;
    run.session().move_cursor(300, 349)?;
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
    run.session().move_cursor(300, 300)?;
    run.session().move_cursor(214, 112)?;
    run.dump("F2_24.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_25.png")?;
    run.session().move_cursor(104, 203)?;
    run.dump("F2_26.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_27.png")?;
    run.session().move_cursor(209, 35)?;
    run.dump("F2_28.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_29.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_30.png")?;
    run.session().move_cursor(104, 203)?;
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
    run.session().move_cursor(200, 200)?;
    run.session().move_cursor(404, 210)?;
    run.dump("F2_33.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_34.png")?;
    run.session().select_point(221, 97)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_35.png")?;
    run.session().select_point(404, 210)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_36.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.finish()
}
