//! A wireframe top view (+X+Y) of shapes on an in-memory raster: the standard axes, the shapes'
//! edges, a fit, and a dump of the window pixels. No window system is involved; zero-width
//! lines follow the accelerated server rule of the reference captures (pixel-centre ties go to
//! the smaller coordinate).
use martensite_geometry_raster::{TopologyDrawable, TopologyStyle, axes::AxisGlyph};
use martensite_raster_core::{
    MemoryWindowSystem, PaletteColor, PaletteKind, RasterReports, RasterViewer, ZeroWidthLineRule,
    drawable::RasterDrawable,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Axis3, Direction3};
use std::{error::Error, rc::Rc};

/// The view identifier of the top view.
const VIEW: i32 = 1;
/// The fit margin of the view, in pixels.
const FIT_FRAME: i32 = 10;

/// Drawing diagnostics go to the regression's output streams.
struct ProcessStreams;
impl RasterReports for ProcessStreams {
    fn write_out(&self, text: &str) {
        print!("{text}");
    }
    fn write_err(&self, text: &str) {
        eprint!("{text}");
    }
    fn flush(&self) {}
}

/// An 800x800 top view at (465, 20) showing the standard axes; `PlaneCapture::top()?`.
pub struct PlaneCapture {
    viewer: RasterViewer,
}

impl PlaneCapture {
    /// The view with its axes and no shape; `let mut plane = PlaneCapture::top()?;`.
    pub fn top() -> Result<Self, Failure> {
        let mut viewer = RasterViewer::with_adapter(Some(Rc::new(
            MemoryWindowSystem::with_line_rule(ZeroWidthLineRule::Accelerated),
        )));
        viewer.make_view(VIEW, "+X+Y", 465, 20, 800, 800)?;
        let blue = PaletteColor::new(PaletteKind::Bleu);
        let axes: Rc<dyn RasterDrawable> = Rc::new(AxisGlyph::spatial(blue, 20));
        viewer.display(&axes)?;
        Ok(Self { viewer })
    }

    /// Show the edges of `shape` with the standard topology style; `plane.display(&result)?`.
    pub fn display(&mut self, shape: &Shape) -> Result<(), Failure> {
        let style = TopologyStyle::default();
        let drawable: Rc<dyn RasterDrawable> = Rc::new(TopologyDrawable::new(
            shape.clone(),
            style,
            Rc::new(ProcessStreams),
        ));
        self.viewer.display(&drawable)
    }

    /// Fit everything displayed into the view with a 10 pixel margin, then repaint it;
    /// `plane.fit()?`.
    pub fn fit(&mut self) -> Result<(), Failure> {
        self.viewer.fit_view(VIEW, FIT_FRAME)?;
        let zoom = self.viewer.zoom(VIEW);
        self.viewer.set_zoom(VIEW, zoom);
        self.viewer.repaint_view(VIEW)
    }

    /// Draw, without keeping it displayed, the principal frame of inertia of `props` (centre
    /// of mass, third axis as normal, first axis as X) as orange labelled axes of 30 pixels,
    /// as a mass property computation shows it; `plane.draw_inertia_axes(&props)?`.
    pub fn draw_inertia_axes(&mut self, props: &GProps) -> Result<(), Failure> {
        let principal = props.principal_properties()?;
        let normal = Direction3::from_vector(principal.third_axis_of_inertia())?;
        let x_direction = Direction3::from_vector(principal.first_axis_of_inertia())?;
        let frame = Axis3::new(props.centre_of_mass(), normal, x_direction)?;
        let orange = PaletteColor::new(PaletteKind::Orange);
        let axes: Rc<dyn RasterDrawable> = Rc::new(AxisGlyph::spatial_on(frame, orange, 30));
        self.viewer.draw_on_view(VIEW, &axes)
    }

    /// Write the view's pixels as they are now (no repaint: nothing changed a displayed
    /// shape since the last drawing) as image `basename` of the regression's capture folder.
    /// Example: `plane.dump("D1.png")?`.
    pub fn dump(&mut self, basename: &str) -> Result<(), Box<dyn Error>> {
        let path = crate::source_output::capture_path(basename)?;
        let name = path
            .to_str()
            .ok_or_else(|| format!("capture path {path:?}; expected UTF-8"))?;
        if !self.viewer.save_view(VIEW, name)? {
            return Err(format!("top view dump {basename}; expected a written image").into());
        }
        Ok(())
    }
}
