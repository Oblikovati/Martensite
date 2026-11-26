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
