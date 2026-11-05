//! Face-local UV classification caches retain the first classifier tolerance.
use martensite_tkbrep::{
    brep_adaptor::BRepAdaptorSurface,
    topods::{Face, Shape},
    toptools::ShapeMapHasher,
};
use martensite_tkernel::{
    ncollection::DataMap,
    standard::{Failure, Handle},
};
use martensite_tkg3d::top_abs::State;
use martensite_tkmath::gp::Point2;
use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorTopolTool;

/// Caches one topology classifier per face; `TopOpeBRepPointClassifier::new()`.
pub struct TopOpeBRepPointClassifier {
    active: Handle<BRepTopAdaptorTopolTool>,
    surface: Handle<BRepAdaptorSurface>,
    face_tools: DataMap<Shape, Handle<BRepTopAdaptorTopolTool>, ShapeMapHasher>,
    state: State,
}

impl Default for TopOpeBRepPointClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl TopOpeBRepPointClassifier {
    /// Starts without a classified point; `Self::new().state() == State::Unknown`.
    pub fn new() -> Self {
        Self {
            active: Handle::default(),
            surface: Handle::new(BRepAdaptorSurface::default()),
            face_tools: DataMap::default(),
            state: State::Unknown,
        }
    }

    /// Clears cached face classifications and resets the state; `classifier.init()`.
    pub fn init(&mut self) {
        self.face_tools.clear(false);
        self.state = State::Unknown;
    }

    /// Loads or reuses a face tool without changing the last state; `classifier.load(&face)?`.
    pub fn load(&mut self, face: &Face) -> Result<(), Failure> {
        if let Ok(tool) = self.face_tools.find(face) {
            self.active = tool.clone();
            return Ok(());
        }
        self.surface.initialize(face, true)?;
        let tool = Handle::new(BRepTopAdaptorTopolTool::with_surface(&self.surface.cast())?);
        self.active = tool.clone();
        self.face_tools.bind(face.clone().into_shape(), tool);
        Ok(())
    }

    /// Classifies UV coordinates and stores the result; `classifier.classify(&face, point, 1e-7)?`.
    pub fn classify(
        &mut self,
        face: &Face,
        point: Point2,
        tolerance: f64,
    ) -> Result<State, Failure> {
        self.state = State::Unknown;
        self.load(face)?;
        self.state = self.active.classify(point, tolerance, true)?;
        Ok(self.state)
    }

    /// Reads the last result without classifying again; `classifier.state()`.
    pub fn state(&self) -> State {
        self.state
    }
}
