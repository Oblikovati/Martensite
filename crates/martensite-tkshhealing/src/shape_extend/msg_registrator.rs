//! `ShapeExtendBasicMsgRegistrator` and `ShapeExtendMsgRegistrator`: the objects that the
//! Shape Healing tools send their messages to.
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::message::{Gravity, Msg};
use martensite_tkernel::ncollection::{DataMap, HashPolicy, List};
use martensite_tkernel::standard::{Handle, SharedState, StandardTransient, Transient};
use martensite_tkernel::{impl_transient, impl_transient_interface};
use std::fmt;

/// A transient object keyed by handle identity rather than object contents.
#[derive(Clone)]
pub struct TransientKey(pub Handle<StandardTransient>);

impl fmt::Debug for TransientKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "TransientKey({:#x})", self.0.hash_code())
    }
}

/// Hash and compare transient handles by identity.
#[derive(Clone, Copy, Debug, Default)]
pub struct TransientKeyHasher;

impl HashPolicy<TransientKey> for TransientKeyHasher {
    fn hash(&self, key: &TransientKey) -> usize {
        key.0.hash_code()
    }
    fn equal(&self, first: &TransientKey, second: &TransientKey) -> bool {
        first.0 == second.0
    }
}

/// Messages grouped by transient object identity.
pub type TransientMessageMap = DataMap<TransientKey, List<Msg>, TransientKeyHasher>;

/// Messages grouped by shape identity.
pub type ShapeMessageMap = DataMap<Shape, List<Msg>, ShapeMapHasher>;

/// `ShapeExtendBasicMsgRegistrator`: the three virtual `Send` members, whose bodies
/// are empty; a registrator that keeps the messages (ShapeExtendMsgRegistrator, the
/// Data Exchange processors) overrides them.
///
/// ```
/// use martensite_tkernel::message::{Gravity, Msg};
/// use martensite_tkshhealing::shape_extend::{BasicMsgRegistrator, ShapeExtendBasicMsgRegistrator};
/// ShapeExtendBasicMsgRegistrator::new().send(&Msg::new(), Gravity::Info);
/// ```
pub trait BasicMsgRegistrator: Transient {
    /// `Send(object, message, gravity)` (virtual): attaches a message to an object; the
    /// base class does nothing; `registrator.send_transient(&object, &message, gravity)`.
    fn send_transient(
        &self,
        _object: &Handle<StandardTransient>,
        _message: &Msg,
        _gravity: Gravity,
    ) {
    }

    /// `Send(shape, message, gravity)` (virtual): attaches a message to a shape; the base
    /// class does nothing; `registrator.send_shape(&shape, &message, gravity)`.
    fn send_shape(&self, _shape: &Shape, _message: &Msg, _gravity: Gravity) {}

    /// `Send(message, gravity)` (virtual): [`BasicMsgRegistrator::send_transient`] with a
    /// null object; `registrator.send(&message, gravity)`.
    fn send(&self, message: &Msg, gravity: Gravity) {
        self.send_transient(&Handle::default(), message, gravity);
    }
}
impl_transient_interface!(
    dyn BasicMsgRegistrator,
    "ShapeExtend_BasicMsgRegistrator",
    StandardTransient
);

/// The base class `ShapeExtendBasicMsgRegistrator` itself: every message is dropped.
#[derive(Default)]
pub struct ShapeExtendBasicMsgRegistrator {
    parent: StandardTransient,
}
impl_transient!(
    ShapeExtendBasicMsgRegistrator,
    "ShapeExtend_BasicMsgRegistrator",
    StandardTransient,
    parent,
    interfaces: [dyn BasicMsgRegistrator]
);

impl ShapeExtendBasicMsgRegistrator {
    /// `ShapeExtendBasicMsgRegistrator::new()`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl BasicMsgRegistrator for ShapeExtendBasicMsgRegistrator {}

/// The two message maps of a `ShapeExtendMsgRegistrator`.
#[derive(Default)]
struct MessageMaps {
    transients: TransientMessageMap,
    shapes: ShapeMessageMap,
}

/// `ShapeExtendMsgRegistrator`: keeps every message sent for a non-null object or shape,
/// in the order sent, in one list per object and one per shape.
///
/// ```
/// use martensite_tkernel::message::{Gravity, Msg};
/// use martensite_tkshhealing::shape_extend::{BasicMsgRegistrator, ShapeExtendMsgRegistrator};
/// let registrator = ShapeExtendMsgRegistrator::new();
/// registrator.send(&Msg::new(), Gravity::Warning); // a null object: dropped
/// assert!(registrator.map_transient(|map| map.is_empty()));
/// ```
pub struct ShapeExtendMsgRegistrator {
    parent: ShapeExtendBasicMsgRegistrator,
    maps: SharedState<MessageMaps>,
}
impl_transient!(
    ShapeExtendMsgRegistrator,
    "ShapeExtend_MsgRegistrator",
    ShapeExtendBasicMsgRegistrator,
    parent,
    interfaces: [dyn BasicMsgRegistrator]
);

impl Default for ShapeExtendMsgRegistrator {
    fn default() -> Self {
        Self {
            parent: ShapeExtendBasicMsgRegistrator::new(),
            maps: SharedState::new(MessageMaps::default()),
        }
    }
}

impl ShapeExtendMsgRegistrator {
    /// empty maps; `ShapeExtendMsgRegistrator::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read object messages through a callback while holding the map guard;
    /// `registrator.map_transient(|map| map.len())`.
    pub fn map_transient<R>(&self, read: impl FnOnce(&TransientMessageMap) -> R) -> R {
        read(&self.maps.read().transients)
    }

    /// lends the map of shapes and their messages;
    /// `registrator.map_shape(|map| map.len())`.
    pub fn map_shape<R>(&self, read: impl FnOnce(&ShapeMessageMap) -> R) -> R {
        read(&self.maps.read().shapes)
    }
}

impl BasicMsgRegistrator for ShapeExtendMsgRegistrator {
    /// Appends the message to the list of the object (a new list for a new object); a
    /// null object is ignored. The gravity is not kept.
    fn send_transient(&self, object: &Handle<StandardTransient>, message: &Msg, _gravity: Gravity) {
        if object.is_null() {
            return;
        }
        let key = TransientKey(object.clone());
        let mut maps = self.maps.write();
        append_message(&mut maps.transients, key, message);
    }

    /// Appends the message to the list of the shape (a new list for a new shape, keyed
    /// with `TopTools_ShapeMapHasher`); a null shape is ignored.
    fn send_shape(&self, shape: &Shape, message: &Msg, _gravity: Gravity) {
        if shape.is_null() {
            return;
        }
        let mut maps = self.maps.write();
        append_message(&mut maps.shapes, shape.clone(), message);
    }
}

/// The body both `Send` overrides share: `ChangeFind(key).Append(message)` when the key is
/// bound, else `Bind(key, list)` with a one-message list.
fn append_message<K: fmt::Debug, H: HashPolicy<K>>(
    map: &mut DataMap<K, List<Msg>, H>,
    key: K,
    message: &Msg,
) {
    if let Some(list) = map.change_seek(&key) {
        list.append(message.clone());
        return;
    }
    let mut list = List::default();
    list.append(message.clone());
    map.bind(key, list);
}
