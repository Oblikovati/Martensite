//! `ShapeProcessShapeContext`: the shape of a ShapeProcess sequence, its current result,
//! the map of the modified subshapes and the messages attached to them.
mod record;
pub(super) mod statistics;

use super::ShapeProcessContext;
use crate::shape_extend::ShapeExtendMsgRegistrator;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::{Gravity, Msg};
use martensite_tkernel::ncollection::DataMap;
use martensite_tkernel::standard::{Failure, Handle, SharedState};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::Shape as Continuity;

/// The map of a shape context: original (located) shapes to their current images.
pub type ShapeProcessShapeMap = DataMap<Shape, Shape, ShapeMapHasher>;

/// The private fields of `ShapeProcessShapeContext`.
struct ShapeContextFields {
    shape: Shape,
    result: Shape,
    map: ShapeProcessShapeMap,
    messages: Handle<ShapeExtendMsgRegistrator>,
    until: ShapeEnum,
    non_manifold: bool,
}

/// `ShapeProcessShapeContext`: a ShapeProcessContext (reached through `Deref`) with the
/// processed shape; each operator records its modifications down to the detalisation
/// level (FACE by default).
///
/// ```
/// use martensite_tkbrep::topods::Shape;
/// use martensite_tkshhealing::shape_process::ShapeProcessShapeContext;
/// let context = ShapeProcessShapeContext::with_shape(&Shape::default(), "", "").unwrap();
/// assert!(context.result().is_null());
/// assert_eq!(context.trace_level(), 1);
/// ```
pub struct ShapeProcessShapeContext {
    parent: ShapeProcessContext,
    fields: SharedState<ShapeContextFields>,
}
impl_transient!(
    ShapeProcessShapeContext,
    "ShapeProcess_ShapeContext",
    ShapeProcessContext,
    parent
);

impl std::ops::Deref for ShapeProcessShapeContext {
    type Target = ShapeProcessContext;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

impl ShapeProcessShapeContext {
    /// the context of the resource file and scope,
    /// detalisation FACE, manifold mode; `ShapeProcessShapeContext::new("DE", "read")?`.
    pub fn new(file: &str, scope: &str) -> Result<Self, Failure> {
        Ok(Self {
            parent: ShapeProcessContext::with_file(file, scope)?,
            fields: SharedState::new(ShapeContextFields {
                shape: Shape::default(),
                result: Shape::default(),
                map: ShapeProcessShapeMap::default(),
                messages: Handle::default(),
                until: ShapeEnum::Face,
                non_manifold: false,
            }),
        })
    }

    /// as `new`, then `Init(S)`.
    /// Example: `ShapeProcessShapeContext::with_shape(&shape, "DE", "read")?`.
    pub fn with_shape(shape: &Shape, file: &str, scope: &str) -> Result<Self, Failure> {
        let context = Self::new(file, scope)?;
        context.init(shape);
        Ok(context)
    }

    /// clears the map, starts a new message registrator and makes `shape` the
    /// shape and the result; `context.init(&shape)`.
    pub fn init(&self, shape: &Shape) {
        let mut fields = self.fields.write();
        fields.map = ShapeProcessShapeMap::default();
        fields.messages = Handle::new(ShapeExtendMsgRegistrator::new());
        fields.shape = shape.clone();
        fields.result = shape.clone();
    }

    /// the processed shape.
    pub fn shape(&self) -> Shape {
        self.fields.read().shape.clone()
    }

    /// the current result.
    pub fn result(&self) -> Shape {
        self.fields.read().result.clone()
    }

    /// a copy of the map of the modified shapes.
    pub fn map(&self) -> ShapeProcessShapeMap {
        self.fields.read().map.clone()
    }

    /// the message registrator (shared).
    pub fn messages(&self) -> Handle<ShapeExtendMsgRegistrator> {
        self.fields.read().messages.clone()
    }

    /// Writes the non-const `Messages()` reference; `context.set_messages(&registrator)`.
    pub fn set_messages(&self, messages: &Handle<ShapeExtendMsgRegistrator>) {
        self.fields.write().messages = messages.clone();
    }

    /// the deepest shape type the records descend to.
    pub fn set_detalisation(&self, level: ShapeEnum) {
        self.fields.write().until = level;
    }

    /// `GetDetalisation()`.
    pub fn get_detalisation(&self) -> ShapeEnum {
        self.fields.read().until
    }

    /// the new result, also bound to the shape in the map.
    pub fn set_result(&self, result: &Shape) {
        let mut fields = self.fields.write();
        fields.result = result.clone();
        let shape = fields.shape.clone();
        fields.map.bind(shape, result.clone());
    }

    /// `SetNonManifold(theNonManifold)`.
    pub fn set_non_manifold(&self, non_manifold: bool) {
        self.fields.write().non_manifold = non_manifold;
    }

    /// `IsNonManifold()`.
    pub fn is_non_manifold(&self) -> bool {
        self.fields.read().non_manifold
    }

    /// the message sent to the registrator, if any (the
    /// default gravity is Message_Warning); `context.add_message(&s, &msg, Gravity::Warning)`.
    pub fn add_message(&self, shape: &Shape, message: &Msg, gravity: Gravity) {
        use crate::shape_extend::BasicMsgRegistrator;
        if let Some(registrator) = self.messages().get() {
            registrator.send_shape(shape, message, gravity);
        }
    }

    /// the parameter (trimmed, upper case) read as C0, G1, C1,
    /// G2, C2, C3 or CN; `context.get_continuity("Continuity3d")`.
    pub fn get_continuity(&self, param: &str) -> Option<Continuity> {
        let mut text = self.get_string(param)?;
        text.left_adjust();
        text.right_adjust();
        text.upper_case();
        let names = [
            (&b"C0"[..], Continuity::C0),
            (b"G1", Continuity::G1),
            (b"C1", Continuity::C1),
            (b"G2", Continuity::G2),
            (b"C2", Continuity::C2),
            (b"C3", Continuity::C3),
            (b"CN", Continuity::CN),
        ];
        names
            .into_iter()
            .find(|(name, _)| text.as_bytes() == *name)
            .map(|(_, continuity)| continuity)
    }

    /// `ContinuityVal(param, def)`; `context.continuity_val("Continuity3d", Continuity::C1)`.
    pub fn continuity_val(&self, param: &str, default: Continuity) -> Continuity {
        self.get_continuity(param).unwrap_or(default)
    }
}
