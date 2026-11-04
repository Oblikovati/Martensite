//! GlGraduatedTrihedron::renderTickmarkLabels: the tickmarks, the axis
//! name and the `%g` tick values of one axis. The texts are world-space labels of the plain
//! world view, so they are lowered through the ordinary text path of a retained group.
use super::axis_draws::direction;
use super::grid_axes::GridAxes;
use super::trihedron_frame::TrihedronFrame;
use crate::{Graphic3dDrawGroup, RenderError};
use martensite_tkernel::ncollection::Vec3;
use martensite_tkernel::quantity::Color;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::{AsciiString, HAsciiString};
use martensite_tkmath::gp::Point3;
use martensite_tkservice::font::FontFontAspect;
use martensite_tkservice::graphic3d::*;

/// `myAspectLabels` / `myAspectValues`: default aspects with a 0.285 alpha mask, the font and
/// the colour set before each label.
fn label_aspect(
    font: &AsciiString,
    style: FontFontAspect,
    color: Color,
) -> Handle<dyn Graphic3dAspectsSource> {
    let aspect = Handle::new(Graphic3dAspects::default());
    aspect.set_alpha_mode(Graphic3dAlphaMode::MaskBlend, 0.285);
    aspect.set_text_font_aspect(style);
    let font = if font.is_empty() {
        Handle::default()
    } else {
        Handle::new(HAsciiString::from(font.clone()))
    };
    aspect.set_text_font(font);
    aspect.set_interior_color(color);
    aspect.cast()
}

/// A left/bottom aligned label of `height` at a world point.
fn label(height: i32, value: &str, position: Vec3<f32>) -> Handle<Graphic3dText> {
    let text = Handle::new(Graphic3dText::new(height as f32));
    text.set_text_str(value);
    let point = Point3::new(
        f64::from(position.x),
        f64::from(position.y),
        f64::from(position.z),
    );
    text.set_position(&point);
    text.set_horizontal_alignment(Graphic3dHorizontalTextAlignment::Left);
    text.set_vertical_alignment(Graphic3dVerticalTextAlignment::Bottom);
    text
}

/// C `%g` of a float tick value promoted to double (native `Sprintf("%g")`).
/// Example: `tick_value_text(12.0) == "12"`.
pub(super) fn tick_value_text(value: f32) -> String {
    martensite_tkernel::storage::format_general(f64::from(value), 6)
}

impl TrihedronFrame<'_> {
    /// `renderTickmarkLabels`: tickmarks, then the name and values texts of the axis group.
    pub fn tickmark_labels(
        &mut self,
        index: usize,
        grid: &GridAxes,
        dpix: f32,
        group: &Handle<dyn Graphic3dGroup>,
    ) -> Result<(), RenderError> {
        let aspect = &self.data.axes[index];
        if !aspect.to_draw_name && !aspect.to_draw_values {
            return Ok(());
        }
        let outward = (grid.ticks[index] - grid.origin).normalized();
        self.tickmarks(index, grid, outward, dpix)?;
        group.clear(false);
        self.add_name(index, grid, outward, dpix, group);
        self.add_values(index, grid, outward, dpix, group);
        let group = group.cast::<Graphic3dDrawGroup>();
        let group = group.get().ok_or_else(|| {
            crate::error::rejected(group.object_address(), "a Martensite trihedron label group")
        })?;
        let draws = self.group_draws(group)?;
        self.draws.extend(draws);
        Ok(())
    }

    /// The axis name at the middle of the tick edge, `NameOffset + TickmarksLength` pixels out.
    fn add_name(
        &self,
        index: usize,
        grid: &GridAxes,
        outward: Vec3<f32>,
        dpix: f32,
        group: &Handle<dyn Graphic3dGroup>,
    ) {
        let aspect = &self.data.axes[index];
        if !aspect.to_draw_name {
            return;
        }
        let offset = f64::from(aspect.name_offset + aspect.tickmarks_length);
        let size = self.max - self.min;
        let middle = grid.ticks[index]
            + size * grid.axes[index] * 0.5
            + outward * (f64::from(dpix) * offset) as f32;
        let font = &self.data.names_font;
        group.set_primitives_aspect(&label_aspect(
            font,
            self.data.names_style,
            aspect.name_color,
        ));
        let name = aspect.name.to_utf8();
        group.add_text(&label(self.data.names_size, &name, middle), false);
    }

    /// The `%g` value of every tick, `ValuesOffset + TickmarksLength` pixels out.
    fn add_values(
        &self,
        index: usize,
        grid: &GridAxes,
        outward: Vec3<f32>,
        dpix: f32,
        group: &Handle<dyn Graphic3dGroup>,
    ) {
        let aspect = &self.data.axes[index];
        if !aspect.to_draw_values || aspect.tickmarks_number <= 0 {
            return;
        }
        let font = &self.data.values_font;
        group.set_primitives_aspect(&label_aspect(font, self.data.values_style, aspect.color));
        let offset = f64::from(aspect.values_offset + aspect.tickmarks_length);
        let step = self.tick_step(index, grid);
        let origin = grid.ticks[index];
        let start = [origin.x, origin.y, origin.z][index];
        for tick in 0..=aspect.tickmarks_number {
            let along = tick as f32 * step;
            let position =
                origin + direction(index) * along + outward * (f64::from(dpix) * offset) as f32;
            let text = label(
                self.data.values_size,
                &tick_value_text(start + along),
                position,
            );
            group.add_text(&text, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tick_value_text;

    #[test]
    fn tick_values_print_like_c_percent_g() {
        assert_eq!(tick_value_text(0.0), "0");
        assert_eq!(tick_value_text(12.0), "12");
        assert_eq!(tick_value_text(-7.5), "-7.5");
        assert_eq!(tick_value_text(0.1), "0.1");
        assert_eq!(tick_value_text(1234567.0), "1.23457e+06");
    }
}
