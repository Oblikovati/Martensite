//! Selection regressions of the `face` grid: picking a box by cursor and by area.
use super::prelude::*;
/// `face/B1`: the selection regression's commands and image dumps in order.
pub(crate) fn face_b1() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(4, true)?;
    run.session().move_cursor(150, 250)?;
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
    run.session().move_cursor(100, 250)?;
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
    run.session().move_cursor(350, 128)?;
    run.dump("B1_6.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_7.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1_8.png")?;
    run.session().move_cursor(350, 128)?;
    run.dump("B1_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("B1.png")?;
    run.finish()
}

/// `face/F2`: the selection regression's commands and image dumps in order.
pub(crate) fn face_f2() -> Result<(), Box<dyn Error>> {
    let mut run = SessionRun::new()?;
    run.session().clear()?;
    let t_box = box_sized([100., 100., 100.])?;
    run.session()
        .display("t_box", &t_box, &DisplayRequest::default())?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().set_selection_mode(4, true)?;
    run.session().move_cursor(150, 250)?;
    run.session().select_point(100, 347)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_1.png")?;
    run.session().move_cursor(90, 89)?;
    run.dump("F2_2.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_3.png")?;
    run.session()
        .select_polyline(&[(90, 89)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_4.png")?;
    run.session().move_cursor(150, 250)?;
    run.dump("F2_5.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_6.png")?;
    run.session().move_cursor(90, 89)?;
    run.dump("F2_7.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_8.png")?;
    run.session().move_cursor(250, 250)?;
    run.session().move_cursor(350, 250)?;
    run.dump("F2_9.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_10.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063090),
        proj: Some([0.411696, -0.786621, 0.462736]),
        up: Some([-0.373037, 0.31875, 0.871669]),
        at: Some([38.160233803417, 72.5566786768416, 36.6684620085254]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(200, 200)?;
    run.dump("F2_11.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_12.png")?;
    run.session().move_cursor(30, 290)?;
    run.dump("F2_13.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_14.png")?;
    run.session().move_cursor(300, 300)?;
    run.dump("F2_15.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_16.png")?;
    run.session()
        .select_polyline(&[(200, 200)], AisSelectionScheme::Xor)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_17.png")?;
    run.session().move_cursor(200, 200)?;
    run.dump("F2_18.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_19.png")?;
    run.session().move_cursor(30, 290)?;
    run.dump("F2_20.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_21.png")?;
    run.session().move_cursor(300, 300)?;
    run.dump("F2_22.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_23.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063090),
        proj: Some([0.826197, -0.328587, -0.456835]),
        up: Some([-0.325366, 0.383482, -0.864270]),
        at: Some([7.09632267265386, 62.6518178738573, 76.0263734322149]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(250, 300)?;
    run.dump("F2_24.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_25.png")?;
    run.session().move_cursor(300, 250)?;
    run.dump("F2_26.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_27.png")?;
    run.session().move_cursor(200, 200)?;
    run.dump("F2_28.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_29.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_30.png")?;
    run.session().move_cursor(300, 250)?;
    run.dump("F2_31.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_32.png")?;
    run.session().set_view_parameters(&ViewParameters {
        scale: Some(6.063090),
        proj: Some([-0.333605, 0.937786, -0.108455]),
        up: Some([-0.146382, -0.163422, -0.975587]),
        at: Some([63.9550768175051, -11.0454729423178, 55.0758857080164]),
        ..ViewParameters::default()
    })?;
    run.session().fit()?;
    run.session().move_cursor(0, 0)?;
    run.session().move_cursor(100, 150)?;
    run.dump("F2_33.png")?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_34.png")?;
    run.session().select_point(200, 90)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_35.png")?;
    run.session().select_point(100, 150)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2_36.png")?;
    run.session().select_point(0, 0)?;
    run.session().move_cursor(0, 0)?;
    run.dump("F2.png")?;
    run.finish()
}
