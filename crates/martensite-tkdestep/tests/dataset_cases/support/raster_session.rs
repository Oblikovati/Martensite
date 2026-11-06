//! The session the cases run with: the view lives in an in-memory raster, so no display is
//! needed. It draws what the case's view commands ask for, then checks each capture.
use super::{
    raster_image::{ImageCensus, census_of},
    session::{Drawing, Session, ViewStep},
};
use martensite_geometry_raster::{
    CurveLook, PlanarCurveDrawable, PointGlyph, SpatialCurveDrawable, SurfaceDrawable,
    TopologyDrawable, TopologyStyle, axes::AxisGlyph,
};
use martensite_raster_core::{
    MarkerShape, MemoryWindowSystem, PaletteColor, PaletteKind, RasterReports, RasterViewer,
    ZeroWidthLineRule, drawable::RasterDrawable,
};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::{collections::BTreeMap, path::PathBuf, rc::Rc};

/// The only view the checks use.
const VIEW: i32 = 1;
/// `smallview`: the axonometric view at (728, 450), 400 pixels wide and high.
const SMALL_VIEW: (&str, i32, i32, i32, i32) = ("AXON", 728, 450, 400, 400);
/// `smallview -2D-`: the planar view, at the same place and size.
const PLANE_VIEW: &str = "-2D-";
/// The fit margin of the view, in pixels.
const FIT_FRAME: i32 = 10;
/// Keeps the captures of a run in this folder, for inspection; otherwise each is deleted.
pub const KEEP_IMAGES_ENV: &str = "MARTENSITE_GEOMETRY_ARTIFACT_DIR";

/// Drawing diagnostics go to the case's stream, which the parent grades.
struct CaseStreams;
impl RasterReports for CaseStreams {
    fn write_out(&self, text: &str) {
        print!("{text}");
    }
    fn write_err(&self, text: &str) {
        eprint!("{text}");
    }
    fn flush(&self) {}
}

struct ShapeVariable {
    shape: Shape,
    drawable: Rc<dyn RasterDrawable>,
}

/// The discretisation a curve variable takes by default: 30 points, deflection 0.01, mode 0.
const CURVE_SAMPLING: (i32, f64, i32) = (30, 0.01, 0);

fn drawable_of(drawing: Drawing<'_>) -> Rc<dyn RasterDrawable> {
    let look = CurveLook::new(PaletteColor::new(PaletteKind::Vert));
    match drawing {
        Drawing::Curve(curve) => Rc::new(SpatialCurveDrawable::with_look(
            curve.clone(),
            look,
            CURVE_SAMPLING,
        )),
        Drawing::PlaneCurve(curve) => Rc::new(PlanarCurveDrawable::with_look(
            curve.clone(),
            look,
            CURVE_SAMPLING.0,
        )),
        Drawing::Surface(surface) => Rc::new(SurfaceDrawable::new(surface.clone())),
        Drawing::Point(point) => Rc::new(PointGlyph::new(
            point,
            MarkerShape::Plus,
            PaletteColor::new(PaletteKind::Jaune),
        )),
        Drawing::PlanePoint(point) => Rc::new(PointGlyph::new_2d(
            point,
            MarkerShape::Plus,
            PaletteColor::new(PaletteKind::Jaune),
        )),
    }
}

/// Variables and a view on a memory raster; `RasterSession::new("boptuc_simple_A1")`.
pub struct RasterSession {
    viewer: RasterViewer,
    variables: BTreeMap<String, ShapeVariable>,
    drawings: BTreeMap<String, Rc<dyn RasterDrawable>>,
    displayed: Vec<String>,
    case_name: String,
}

impl RasterSession {
    /// A session with no variable and no view yet; the name prefixes its captures' files.
    /// Example: `let mut session = RasterSession::new("boptuc_simple_A1");`.
    pub fn new(case_name: &str) -> Self {
        // Zero-width lines follow the accelerated server rule of the reference captures.
        let system = MemoryWindowSystem::with_line_rule(ZeroWidthLineRule::Accelerated);
        let mut session = Self {
            viewer: RasterViewer::with_adapter(Some(Rc::new(system))),
            variables: BTreeMap::new(),
            drawings: BTreeMap::new(),
            displayed: Vec::new(),
            case_name: case_name.to_owned(),
        };
        session.display_axes();
        session
    }

    /// The protected coordinate axes: shown from the start, and again after each erase.
    fn display_axes(&mut self) {
        let blue = PaletteColor::new(PaletteKind::Bleu);
        let glyphs = [AxisGlyph::spatial(blue, 20), AxisGlyph::planar(blue, 20)];
        for glyph in glyphs {
            let drawable: Rc<dyn RasterDrawable> = Rc::new(glyph);
            self.viewer.display(&drawable).unwrap();
        }
    }

    fn erase_all(&mut self) {
        self.viewer.clear();
        self.displayed.clear();
    }

    fn display_only(&mut self, names: &[&str]) {
        let zoom = self.viewer.zoom(VIEW);
        self.erase_all();
        for name in names {
            // A name that is not bound displays nothing and is not an error.
            if self.is_bound(name) {
                self.display_named("donly", name);
            }
        }
        self.display_axes();
        let kept = self.viewer.zoom(VIEW);
        assert_eq!(
            kept.to_bits(),
            zoom.to_bits(),
            "donly {names:?}: expected the same zoom"
        );
    }

    fn display_more(&mut self, names: &[&str]) {
        for name in names {
            self.display_named("disp", name);
        }
    }

    fn is_bound(&self, name: &str) -> bool {
        self.variables.contains_key(name) || self.drawings.contains_key(name)
    }

    fn drawable_named(&self, command: &str, name: &str) -> Rc<dyn RasterDrawable> {
        self.variables
            .get(name)
            .map(|variable| &variable.drawable)
            .or_else(|| self.drawings.get(name))
            .unwrap_or_else(|| panic!("{command} {name}: expected a bound variable"))
            .clone()
    }

    fn display_named(&mut self, command: &str, name: &str) {
        let drawable = self.drawable_named(command, name);
        self.viewer.display(&drawable).unwrap();
        self.displayed.push(name.to_owned());
    }

    fn open_view(&mut self, kind: &str) {
        let (_, x, y, width, height) = SMALL_VIEW;
        self.viewer.delete_view(VIEW);
        self.viewer
            .make_view(VIEW, kind, x, y, width, height)
            .unwrap();
        let zoom = self.viewer.zoom(VIEW);
        assert_eq!(
            zoom, 1.,
            "smallview: expected the default zoom before a fit"
        );
    }

    fn forget(&mut self, name: &str) {
        if let Some(previous) = self.variables.remove(name) {
            self.viewer.remove_drawable(&previous.drawable);
        }
        if let Some(previous) = self.drawings.remove(name) {
            self.viewer.remove_drawable(&previous);
        }
        self.displayed.retain(|displayed| displayed != name);
    }

    /// Whether a displayed variable draws anything in the open view: a drawing of the other
    /// kind of view does not, and a shape shows only in space.
    fn draws_in_view(&self, name: &str) -> bool {
        let planar = self.viewer.has_view(VIEW) && !self.viewer.is_3d(VIEW);
        match (self.variables.get(name), self.drawings.get(name)) {
            (Some(variable), _) => !planar && shows_geometry(&variable.shape),
            (_, Some(drawing)) => drawing.is_3d() != planar,
            (None, None) => false,
        }
    }

    fn fit(&mut self) {
        self.viewer.fit_view(VIEW, FIT_FRAME).unwrap();
        let zoom = self.viewer.zoom(VIEW);
        self.viewer.set_zoom(VIEW, zoom);
        self.viewer.repaint_view(VIEW).unwrap();
    }

    fn capture(&mut self, image: &str) {
        self.viewer.repaint_view(VIEW).unwrap();
        let path = self.image_path(image);
        let name = path.to_str().expect("capture path: expected UTF-8");
        let saved = self.viewer.save_view(VIEW, name).unwrap();
        assert!(saved, "xwd {name}: expected a written image");
        let bytes = std::fs::read(&path).unwrap();
        if std::env::var_os(KEEP_IMAGES_ENV).is_none() {
            std::fs::remove_file(&path).unwrap();
        }
        self.require_window_size(name, &bytes);
        let census = census_of(&bytes).unwrap_or_else(|cause| panic!("xwd {name}: {cause}"));
        self.require_visible(image, census);
    }

    fn require_window_size(&self, name: &str, bytes: &[u8]) {
        let (_, _, width, height) = self.viewer.pos_size(VIEW).expect("capture of an open view");
        let header = format!("P6\n{width} {height}\n255\n");
        assert!(
            bytes.starts_with(header.as_bytes()),
            "xwd {name}: expected an image of the window, {header:?}"
        );
    }

    fn image_path(&self, image: &str) -> PathBuf {
        let file = format!("{}-{image}.ppm", self.case_name);
        let Some(folder) = std::env::var_os(KEEP_IMAGES_ENV) else {
            let unique = format!("{}-{file}", std::process::id());
            return std::env::temp_dir().join(unique);
        };
        std::fs::create_dir_all(&folder).unwrap();
        PathBuf::from(folder).join(file)
    }

    fn require_visible(&self, image: &str, census: ImageCensus) {
        assert!(census.drawn > 0, "xwd {image}: expected a drawn image");
        let expected = self
            .displayed
            .iter()
            .filter(|name| self.draws_in_view(name))
            .collect::<Vec<_>>();
        assert!(
            expected.is_empty() || census.shape > 0,
            "xwd {image}: {census:?}; expected pixels of the displayed {expected:?}"
        );
    }
}

/// Whether drawing `shape` draws anything: an empty compound has no edge and no vertex.
fn shows_geometry(shape: &Shape) -> bool {
    [ShapeEnum::Edge, ShapeEnum::Vertex]
        .into_iter()
        .any(|kind| Explorer::new(shape, kind, ShapeEnum::Shape).unwrap().more())
}

impl Session for RasterSession {
    fn unbind(&mut self, name: &str) {
        self.forget(name);
    }

    fn bind(&mut self, name: &str, shape: &Shape) {
        assert!(!shape.is_null(), "{name}: expected a shape, got a null one");
        self.forget(name);
        let drawable: Rc<dyn RasterDrawable> = Rc::new(TopologyDrawable::new(
            shape.clone(),
            TopologyStyle::default(),
            Rc::new(CaseStreams),
        ));
        self.viewer.display(&drawable).unwrap();
        let variable = ShapeVariable {
            shape: shape.clone(),
            drawable,
        };
        self.variables.insert(name.to_owned(), variable);
        self.displayed.push(name.to_owned());
    }

    fn bind_drawing(&mut self, name: &str, drawing: Drawing<'_>) {
        self.forget(name);
        let drawable = drawable_of(drawing);
        self.viewer.display(&drawable).unwrap();
        self.drawings.insert(name.to_owned(), drawable);
        self.displayed.push(name.to_owned());
    }

    fn shape(&self, name: &str) -> Option<&Shape> {
        self.variables.get(name).map(|variable| &variable.shape)
    }

    fn view(&mut self, step: ViewStep<'_>) {
        match step {
            ViewStep::SmallView => self.open_view(SMALL_VIEW.0),
            ViewStep::PlaneView => self.open_view(PLANE_VIEW),
            ViewStep::Clear => {
                self.erase_all();
                self.display_axes();
            }
            ViewStep::Donly(names) => self.display_only(names),
            ViewStep::Disp(names) => self.display_more(names),
            ViewStep::Fit => self.fit(),
            ViewStep::Xwd(image) => self.capture(image),
        }
    }
}
