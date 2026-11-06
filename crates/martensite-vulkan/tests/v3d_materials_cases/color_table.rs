//! A second actual driver owns the command's sixty-by-thirty sample view.
use super::*;
#[path = "../viewer_commands/opengl_commands/color_table.rs"]
mod command;
use martensite_tkservice::aspect::AspectGradientFillMethod;
use std::io::Write;

pub(super) fn write_table() -> Outcome {
    let driver = crate::session_run::run_driver(Default::default())?;
    let session = ViewerSession::with_view_size(&driver, 60, 30)?;
    session.view().set_immediate_update(false);
    session
        .view()
        .set_bg_gradient_style(AspectGradientFillMethod::None, false);
    let mut file = std::fs::File::create(crate::source_output::capture_path("colors_table.html")?)?;
    file.write_all(command::HEADER.as_bytes())?;
    let result = write_samples(&session, &mut file);
    session.view().remove();
    result?;
    file.write_all(command::FOOTER.as_bytes())?;
    Ok(())
}
fn write_samples(session: &ViewerSession, file: &mut std::fs::File) -> Outcome {
    for color in NamedColor::all() {
        let name = Color::string_name(color);
        let image = format!("colors_table_{name}.png");
        let path = crate::source_output::capture_path(&image)?;
        if !command::render_sample(session.view(), Color::named(color), &path.to_string_lossy())? {
            return Err(
                format!("sample image {path:?}; expected successful actual view dump").into(),
            );
        }
        file.write_all(command::html_row(Color::named(color), name, &image).as_bytes())?;
    }
    Ok(())
}
