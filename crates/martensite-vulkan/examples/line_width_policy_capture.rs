//! Capture the existing wide-line scene under explicit immutable raster-width selections.
#[path = "support/device_options.rs"]
mod device_options;
#[expect(
    dead_code,
    reason = "The shared viewer fixture also exposes retained resources and accessors used by other captures."
)]
#[path = "../tests/support/viewer_scene.rs"]
mod scene;
#[path = "support/view_capture.rs"]
mod view_capture;
#[path = "../tests/support/view_setup.rs"]
mod view_setup;
#[path = "../tests/support/wide_line_scene.rs"]
mod wide_line_scene;
use martensite_tkernel::quantity::{Color, NamedColor};
use martensite_tkservice::graphic3d::Graphic3dStructureSource;
#[path = "support/line_clipping_arguments.rs"]
mod line_clipping_arguments;
use line_clipping_arguments::LineCaptureSelection;
use martensite_vulkan::{Graphic3dRenderDriver, RenderLineClippingBehavior, RenderLineWidthPolicy};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let (folder, policy, clipping) = width_capture_request(&args)?;
    capture_selected_policy(folder, policy, clipping)
}

fn width_capture_request(
    args: &[String],
) -> Result<(&str, RenderLineWidthPolicy, RenderLineClippingBehavior), String> {
    if !(2..=4).contains(&args.len()) {
        return Err(format!(
            "line capture args {args:?}; expected program output-directory [--core] [--clip-v1|--clip-v2]"
        ));
    }
    let selection = LineCaptureSelection::parse(&args[2..]).map_err(|error| {
        format!("{error}; expected program output-directory [--core] [--clip-v1|--clip-v2]")
    })?;
    Ok((&args[1], selection.widths, selection.behavior))
}

fn capture_selected_policy(
    folder: &str,
    policy: RenderLineWidthPolicy,
    clipping: RenderLineClippingBehavior,
) -> Result<(), Box<dyn std::error::Error>> {
    let driver = Graphic3dRenderDriver::with_line_clipping_configuration(
        device_options::validation_options(),
        Default::default(),
        Default::default(),
        policy,
        clipping,
    )?;
    println!(
        "{{\"line_clipping_behavior\":\"{clipping:?}\",\"behavior_version\":{}}}",
        clipping.behavior_version()
    );
    let scene = width_scene(&driver);
    capture_widths(&scene, folder)?;
    assert_width_capture(&driver)?;
    scene.view.remove();
    Ok(())
}

fn width_scene(
    driver: &martensite_tkernel::standard::Handle<Graphic3dRenderDriver>,
) -> scene::ViewerScene {
    let scene = scene::ViewerScene::new(driver);
    scene.structure.erase();
    scene
        .view
        .set_background_color(Color::named(NamedColor::BLACK));
    scene.view.camera().set_scale(64.);
    scene
}
fn assert_width_capture(driver: &Graphic3dRenderDriver) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "MARTENSITE_DEVICE={} POLICY={:?}",
        driver.device_name(),
        driver.line_width_policy()
    );
    let findings = driver.validation_errors()?;
    assert!(findings.is_empty(), "{findings:?}");
    Ok(())
}

fn capture_widths(
    scene: &scene::ViewerScene,
    folder: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for width in [3., 9., 12.] {
        for (name, from, to) in wide_line_scene::SEGMENTS {
            let object =
                wide_line_scene::show_segment(&scene.viewer.structure_manager(), width, [from, to]);
            let prefix = format!("{folder}/width-{width}-line-{name}");
            view_capture::capture_view(&scene.view, 64, 64, &prefix, false)?;
            object.erase();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn policy_selection_is_explicit_and_rejects_unknown_flags() {
        assert_eq!(
            width_capture_request(&["capture".into(), "out".into()])
                .unwrap()
                .1,
            RenderLineWidthPolicy::AspectWidthsV1
        );
        assert_eq!(
            width_capture_request(&["capture".into(), "out".into(), "--core".into()])
                .unwrap()
                .1,
            RenderLineWidthPolicy::SinglePixelV1
        );
        assert!(
            width_capture_request(&["capture".into(), "out".into(), "--unknown".into()])
                .unwrap_err()
                .contains("--unknown")
        );
    }
    #[test]
    fn capture_arity_rejects_missing_directory_and_trailing_arguments() {
        for words in [vec!["capture"], vec!["capture", "out", "--core", "extra"]] {
            let args: Vec<String> = words.into_iter().map(str::to_owned).collect();
            assert!(
                width_capture_request(&args)
                    .unwrap_err()
                    .contains("expected program")
            );
        }
        for words in [vec!["capture", "out"], vec!["capture", "out", "--core"]] {
            let args: Vec<String> = words.into_iter().map(str::to_owned).collect();
            assert_eq!(width_capture_request(&args).unwrap().0, "out");
        }
    }
}
