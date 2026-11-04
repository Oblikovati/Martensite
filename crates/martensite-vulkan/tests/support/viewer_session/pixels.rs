//! Reading rendered pixels of the active view.
use super::*;
use martensite_tkernel::quantity::ColorRgba;
use martensite_tkservice::image::{ImageFormat, ImagePixMap};

impl ViewerSession {
    /// The nearest named colour of RGB pixel (x, y), y from the top; `session.pixel_color_name(165, 200)?`.
    pub fn pixel_color_name(&self, x: i32, y: i32) -> Result<String, Failure> {
        Ok(Color::string_name(self.pixel_color(x, y)?.name()).to_owned())
    }

    /// The nearest named colour and the alpha of RGBA pixel (x, y), as "CYAN 1".
    /// Example: `session.pixel_rgba_name(94, 140)? == "CYAN 1"`.
    pub fn pixel_rgba_name(&self, x: i32, y: i32) -> Result<String, Failure> {
        let color = self.read_pixel(x, y, ImageFormat::Rgba, Graphic3dBufferType::Rgba)?;
        let name = Color::string_name(color.rgb().name());
        Ok(format!("{name} {}", f64::from(color.alpha())))
    }

    /// Linear RGBA components of pixel (x, y).
    /// Example: `session.pixel_rgba(165, 263)? == [0., 1., 1., 1.]`.
    pub fn pixel_rgba(&self, x: i32, y: i32) -> Result<[f64; 4], Failure> {
        let color = self.read_pixel(x, y, ImageFormat::Rgba, Graphic3dBufferType::Rgba)?;
        let rgb = color.rgb();
        Ok([rgb.red(), rgb.green(), rgb.blue(), f64::from(color.alpha())])
    }

    /// RGB colour of pixel (x, y) rendered at the window size; `session.pixel_color(1, 1)?`.
    pub fn pixel_color(&self, x: i32, y: i32) -> Result<Color, Failure> {
        let color = self.read_pixel(x, y, ImageFormat::Rgb, Graphic3dBufferType::Rgb)?;
        Ok(*color.rgb())
    }

    fn read_pixel(
        &self,
        x: i32,
        y: i32,
        format: ImageFormat,
        buffer: Graphic3dBufferType,
    ) -> Result<ColorRgba, Failure> {
        let (width, height) = self.source_window_size();
        if x < 0 || x >= width || y < 0 || y > height {
            return Err(martensite_tkernel::standard::exceptions::program_error(
                &format!("pixel ({x}; {y}); expected inside the {width} x {height} view"),
            ));
        }
        let image = ImagePixMap::default();
        if !image.init_trash(format, width as usize, height as usize, 0) {
            return Err(martensite_tkernel::standard::exceptions::program_error(
                &format!("image {width} x {height}; expected an allocatable RGB buffer"),
            ));
        }
        let view = self.view().view_base();
        if !view.to_pix_map_sized(&image, width, height, buffer, true)? {
            return Err(martensite_tkernel::standard::exceptions::program_error(
                &format!("view dump {width} x {height}; expected a rendered image"),
            ));
        }
        image.pixel_color(x, y, true)
    }
}
