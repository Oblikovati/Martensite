//! Both authored scales retain perspective, automatic-depth queries and actual pixel checks.
use crate::{
    case_view::viewer_session::{DisplayRequest, ViewerSession},
    primitive_shapes::box_at,
    session_run::{SessionRun, banner},
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{
    quantity::{Color, NamedColor},
    standard::Failure,
};
use martensite_tkservice::graphic3d::Graphic3dCameraProjection;
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
type Outcome = Result<(), Box<dyn std::error::Error>>;
const BOX_NAMES: [&str; 6] = ["b0", "b1", "b2", "b3", "b4", "b5"];

pub(super) fn boxes(size: f64) -> Result<Vec<Shape>, Failure> {
    [
        [0., 0., 0.],
        [-1., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [-1., 1., 0.],
        [1., 1., 0.],
    ]
    .into_iter()
    .map(|p| box_at(p.map(|v| v * size), [size; 3]))
    .collect()
}
/// Check the six real boxes at unit and hundredth scale; `perspective_boxes()?`.
pub(crate) fn perspective_boxes() -> Outcome {
    banner(&[
        "========",
        "OCC26401: Visualization - small boxes in front of big one disappear in perspective view",
        "========",
        "",
    ]);
    let first = boxes(1.)?;
    let mut run = SessionRun::new()?;
    display_scene(run.session(), &first)?;
    query_and_check(&mut run, "bug26401_1.png")?;
    let second = boxes(0.01)?;
    run.session().open_view(409, 409)?;
    display_scene(run.session(), &second)?;
    query_and_check(&mut run, "bug26401_2.png")?;
    run.finish()
}
pub(super) fn display_scene(session: &mut ViewerSession, shapes: &[Shape]) -> Result<(), Failure> {
    session.clear()?;
    session.set_axonometric()?;
    session.set_display_mode(1)?;
    session.set_projection(Graphic3dCameraProjection::Perspective)?;
    let named: Vec<_> = BOX_NAMES
        .iter()
        .zip(shapes)
        .map(|(name, shape)| (*name, shape))
        .collect();
    session.display_many(&named, &DisplayRequest::default())?;
    set_colors(session, &["b1", "b2"], NamedColor::GREEN)?;
    set_colors(session, &["b4", "b5"], NamedColor::RED)?;
    set_colors(session, &["b3"], NamedColor::GRAY55)?;
    session.fit()
}
fn set_colors(session: &ViewerSession, names: &[&str], color: NamedColor) -> Result<(), Failure> {
    session.update_context(|context| {
        for name in names {
            let object = session.object(name).ok_or_else(|| {
                martensite_tkernel::standard::exceptions::program_error(&format!(
                    "box name={name:?}; expected authored AisShape binding"
                ))
            })?;
            context.set_color(object, Color::named(color), false)?;
            object.synchronize_aspects();
        }
        Ok(())
    })
}
fn query_and_check(run: &mut SessionRun, image: &'static str) -> Outcome {
    query_depth(run.session());
    run.session().display_trihedron("t")?;
    let color = run.session().pixel_color(230, 230)?;
    if color.red() == 0. && color.green() == 0. && color.blue() == 0. {
        println!("Error : Boxes disappear.");
    }
    run.dump(image)?;
    Ok(())
}

fn query_depth(session: &ViewerSession) {
    let view = session.view().view_base();
    println!(
        "Auto z-fit mode: \nOn: {}\nScale: {}",
        if view.auto_z_fit_mode() {
            "enabled"
        } else {
            "disabled"
        },
        view.auto_z_fit_scale_factor()
    );
    println!(
        "ZNear: {}\nZFar: {}",
        view.camera().z_near(),
        view.camera().z_far()
    );
}
#[cfg(test)]
mod cpu {
    use super::*;
    use crate::case_view::SourceViewOutput;
    #[test]
    fn both_scales_keep_perspective_and_six_named_shapes() {
        let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
        let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
        for size in [1., 0.01] {
            display_scene(&mut session, &boxes(size).unwrap()).unwrap();
            assert_eq!(
                session.view().camera().projection_type(),
                Graphic3dCameraProjection::Perspective
            );
            for name in BOX_NAMES {
                assert!(session.object(name).is_some());
            }
        }
        session.close();
    }
}
