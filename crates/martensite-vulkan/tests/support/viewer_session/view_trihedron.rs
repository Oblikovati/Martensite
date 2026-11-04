//! The view trihedron: z-buffered or wireframe axes at a window corner or the view centre.
use super::*;

/// View trihedron settings, with the defaults of the interactive viewer's trihedron command
/// (lower left, shaded, white labels, red/green/blue arrows, scale 0.1).
/// Example: `ViewTrihedronSettings { scale: 0.2, ..Default::default() }`.
#[derive(Clone, Copy, Debug)]
pub struct ViewTrihedronSettings {
    /// The window corner, or the view centre.
    pub position: AspectTypeOfTriedronPosition,
    /// Lines instead of shaded tubes and cones.
    pub wireframe: bool,
    /// The X, Y and Z label colours.
    pub labels: [Color; 3],
    /// The X, Y and Z arrow colours.
    pub arrows: [Color; 3],
    /// The size relative to the view.
    pub scale: f64,
}

impl Default for ViewTrihedronSettings {
    fn default() -> Self {
        Self {
            position: AspectTypeOfTriedronPosition::LEFT_LOWER,
            wireframe: false,
            labels: [Color::named(NamedColor::WHITE); 3],
            arrows: [NamedColor::RED, NamedColor::GREEN, NamedColor::BLUE1].map(Color::named),
            scale: 0.1,
        }
    }
}

impl ViewerSession {
    /// Show the view trihedron (size ratio 0.8, arrow diameter 0.05, 12 facets), fit the
    /// depth range and redraw; `session.show_view_trihedron(&Default::default())?`.
    pub fn show_view_trihedron(&self, settings: &ViewTrihedronSettings) -> Result<(), Failure> {
        self.update_view(|view| {
            let handle = view.trihedron(true)?;
            if let Some(trihedron) = handle.get() {
                apply_trihedron_settings(trihedron, settings)?;
                trihedron.display(view)?;
            }
            view.z_fit_all(1.0)
        })
    }
}

fn apply_trihedron_settings(
    trihedron: &V3dTrihedron,
    settings: &ViewTrihedronSettings,
) -> Result<(), Failure> {
    let ([ax, ay, az], [lx, ly, lz]) = (settings.arrows, settings.labels);
    trihedron.set_arrows_color(ax, ay, az)?;
    trihedron.set_labels_colors(lx, ly, lz);
    trihedron.set_size_ratio(0.8);
    trihedron.set_nb_facets(12);
    trihedron.set_arrow_diameter(0.05);
    trihedron.set_scale(settings.scale);
    trihedron.set_position(settings.position)?;
    trihedron.set_wireframe(settings.wireframe);
    Ok(())
}
