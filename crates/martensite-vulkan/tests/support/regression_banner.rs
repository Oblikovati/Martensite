//! Pure original stream headings are shared by graphics and independent raster hosts.
/// Print the regression's banner lines; `banner(&["====", "OCC1: title"])`.
pub(crate) fn banner(lines: &[&str]) {
    for line in lines {
        println!("{line}");
    }
}
