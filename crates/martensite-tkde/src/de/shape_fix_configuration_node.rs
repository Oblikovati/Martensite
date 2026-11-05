//! `DeShapeFixConfigurationNode`: the base of the format nodes that heal shapes (STEP,
//! IGES): the `provider.FORMAT.VENDOR.healing.*` resources.
use super::{
    DeConfigurationContext, DeConfigurationNode, DeConfigurationNodeBase, DeShapeFixParameters,
    live,
};
use martensite_tkernel::{
    impl_transient_interface,
    standard::{Failure, Handle, SharedState},
    tcollection::AsciiString,
};

/// The native `THE_CONFIGURATION_SCOPE()` of `DeShapeFixConfigurationNode`.
const CONFIGURATION_SCOPE: &str = "provider";

/// The fields of `DeShapeFixConfigurationNode`: the node fields and `ShapeFixParameters`;
/// `DeShapeFixConfigurationNodeBase::default()` is the default constructor.
#[derive(Default)]
pub struct DeShapeFixConfigurationNodeBase {
    node: DeConfigurationNodeBase,
    parameters: SharedState<DeShapeFixParameters>,
}

impl DeShapeFixConfigurationNodeBase {
    /// `DeShapeFixConfigurationNode(theConfigurationNode)`: the node fields and the healing
    /// parameters of another node; a null node raises `StandardNullObject`;
    /// `DeShapeFixConfigurationNodeBase::copied_from(&node)?`.
    pub fn copied_from(node: &Handle<dyn DeShapeFixConfigurationNode>) -> Result<Self, Failure> {
        let source = live(node, "DEShapeFixConfigurationNode(theConfigurationNode)")?;
        Ok(Self {
            node: DeConfigurationNodeBase::copied_from(&node.cast())?,
            parameters: SharedState::new(source.shape_fix_parameters()),
        })
    }
    /// Copies node and healing values into independent state; `base.copied_state()`.
    pub fn copied_state(&self) -> Self {
        Self {
            node: self.node.copied_state(),
            parameters: SharedState::new(self.shape_fix_parameters()),
        }
    }
    /// The embedded `DeConfigurationNode` fields; `base.configuration_node_base()`.
    pub fn configuration_node_base(&self) -> &DeConfigurationNodeBase {
        &self.node
    }
    /// The public `ShapeFixParameters` field; `base.shape_fix_parameters().tolerance3d`.
    pub fn shape_fix_parameters(&self) -> DeShapeFixParameters {
        *self.parameters.read()
    }
    /// Assigns the public `ShapeFixParameters` field; `base.set_shape_fix_parameters(p)`.
    pub fn set_shape_fix_parameters(&self, parameters: DeShapeFixParameters) {
        *self.parameters.write() = parameters;
    }
}

/// `DeShapeFixConfigurationNode`: a configuration node with shape-healing parameters.
///
/// A concrete node returns `shape_fix_base().configuration_node_base()` from
/// `configuration_node_base`, and its `load` and `save` call
/// [`DeShapeFixConfigurationNode::shape_fix_load`] and
/// [`DeShapeFixConfigurationNode::shape_fix_save`] where the reference calls the base class
/// (de-design.md section 1).
pub trait DeShapeFixConfigurationNode: DeConfigurationNode {
    /// The embedded base with the native fields.
    fn shape_fix_base(&self) -> &DeShapeFixConfigurationNodeBase;

    /// `DeShapeFixConfigurationNode::load(theResource)`: reads the healing resources of
    /// `provider.FORMAT.VENDOR.healing`; a mode takes any integer. Always `true`; a null
    /// context raises `StandardNullObject`.
    fn shape_fix_load(&self, resource: &Handle<DeConfigurationContext>) -> Result<bool, Failure> {
        let context = live(resource, "DEShapeFixConfigurationNode::Load")?;
        let scope = healing_scope(self, "");
        let mut parameters = self.shape_fix_parameters();
        super::shape_fix_resources::load_shape_fix_parameters(context, &scope, &mut parameters);
        self.set_shape_fix_parameters(parameters);
        Ok(true)
    }

    /// `DeShapeFixConfigurationNode::save()`: the healing resources with their comments.
    fn shape_fix_save(&self) -> AsciiString {
        super::shape_fix_resources::save_shape_fix_parameters(
            self.shape_fix_parameters(),
            &healing_scope(self, "."),
        )
    }

    /// The public `ShapeFixParameters` field.
    fn shape_fix_parameters(&self) -> DeShapeFixParameters {
        self.shape_fix_base().shape_fix_parameters()
    }
    /// Assigns the public `ShapeFixParameters` field.
    fn set_shape_fix_parameters(&self, parameters: DeShapeFixParameters) {
        self.shape_fix_base().set_shape_fix_parameters(parameters);
    }
}
impl_transient_interface!(
    dyn DeShapeFixConfigurationNode,
    "DEShapeFixConfigurationNode",
    dyn DeConfigurationNode
);

/// `provider.FORMAT.VENDOR.healing` followed by `suffix`.
fn healing_scope<N: DeShapeFixConfigurationNode + ?Sized>(node: &N, suffix: &str) -> AsciiString {
    let mut scope: AsciiString = AsciiString::new(CONFIGURATION_SCOPE);
    scope += ".";
    scope += &node.get_format();
    scope += ".";
    scope += &node.get_vendor();
    scope += ".healing";
    scope += suffix;
    scope
}
