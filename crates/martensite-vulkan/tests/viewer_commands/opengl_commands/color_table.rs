//! Shared table bytes and actual sample rendering keep both command clients identical.
use martensite_tkernel::{
    quantity::Color,
    standard::{Failure, Handle},
    storage::format_general,
    tcollection::AsciiString,
};
use martensite_tkservice::{graphic3d::Graphic3dBufferType, image::ImageAlienPixMap};
use martensite_tkv3d::v3d::{V3dImageDumpOptions, V3dStereoDumpOptions, V3dViewSource};

pub(crate) const HEADER: &str = concat!(
    "<html>\n<head><title>Color table</title></head>\n<body>\n",
    "<table border='1'><tbody>\n<tr>\n<th>HTML</th>\n<th>Name</th>\n",
    "<th>Color name</th>\n<th>sRGB hex</th>\n<th>sRGB dec</th>\n<th>RGB linear</th>\n</tr>\n"
);
pub(crate) const FOOTER: &str = "</tbody></table>\n</body>\n</html>\n";

/// Save the real temporary view; `render_sample(&view, color, "sample_RED.png")?`.
pub(crate) fn render_sample(
    view: &Handle<dyn V3dViewSource>,
    color: Color,
    path: &str,
) -> Result<bool, Failure> {
    let Some(view) = view.get() else {
        return Ok(true);
    };
    view.set_immediate_update(false);
    view.set_background_color(color);
    let params = sample_options();
    let image = ImageAlienPixMap::default();
    if !view.to_pix_map(&image, &params)? {
        return Ok(false);
    }
    image.save(&AsciiString::from(path))
}

/// Keep the command's exact RGB, mono, adjusted sixty-by-thirty request; `sample_options()`.
pub(crate) fn sample_options() -> V3dImageDumpOptions<'static> {
    V3dImageDumpOptions {
        width: 60,
        height: 30,
        buffer_type: Graphic3dBufferType::Rgb,
        stereo_options: V3dStereoDumpOptions::Mono,
        to_adjust_aspect: true,
        ..Default::default()
    }
}

/// Return one source HTML row; `html_row(color, "RED", "table_RED.png")`.
pub(crate) fn html_row(color: Color, name: &str, source: &str) -> String {
    let hex = String::from_utf8_lossy(Color::to_hex(color, true).as_bytes()).into_owned();
    let [r, g, b] = color.rgb().map(|value| (value * 255.) as i32);
    format!(
        "<tr>\n<td style='background-color:{hex}'><pre>       </pre></td>\n<td><img src='{source}'></img></td>\n<td style='text-align:left'>{name}</td>\n<td style='text-align:left'><pre>{hex}</pre></td>\n<td style='text-align:left'>({r} {g} {b})</td>\n<td style='text-align:left'>({} {} {})</td>\n</tr>\n",
        format_general(color.red(), 6),
        format_general(color.green(), 6),
        format_general(color.blue(), 6)
    )
}

#[cfg(test)]
mod controls {
    use super::*;
    #[test]
    fn sample_options_preserve_full_command_request() {
        let options = sample_options();
        assert_eq!((options.width, options.height), (60, 30));
        assert_eq!(options.buffer_type, Graphic3dBufferType::Rgb);
        assert_eq!(options.stereo_options, V3dStereoDumpOptions::Mono);
        assert!(options.to_adjust_aspect);
    }
    #[test]
    fn null_sample_view_keeps_the_existing_no_render_success_path() {
        let color = Color::named(martensite_tkernel::quantity::NamedColor::RED);
        assert!(render_sample(&Handle::default(), color, "no-file-written.png").unwrap());
    }
    #[test]
    fn red_row_preserves_all_six_columns_and_sample_reference() {
        let color = Color::named(martensite_tkernel::quantity::NamedColor::RED);
        let row = html_row(color, "RED", "table_RED.png");
        assert!(row.contains("src='table_RED.png'"));
        assert!(row.contains("<pre>#FF0000</pre>"));
        assert!(row.contains("(255 0 0)</td>"));
        assert!(row.contains("(1 0 0)</td>"));
        assert_eq!(row.matches("<td").count(), 6);
    }
}
