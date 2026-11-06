//! The private state of `DeConfigurationNode`: `GlobalParameters`, `myIsEnabled` and the
//! owning handle `this` converts to.
use super::DeConfigurationNode;
use martensite_tkernel::standard::{Failure, Handle, SharedState, WeakHandle, exceptions};
use std::sync::OnceLock;

/// `DeConfigurationNode::DESectionGlobal`: the units of a transfer.
///
/// ```
/// use martensite_tkde::de::DeSectionGlobal;
/// assert_eq!(DeSectionGlobal::default().length_unit, 1.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeSectionGlobal {
    /// `LengthUnit`: the target unit as a scale of millimetres, 1.0 (MM) by default.
    pub length_unit: f64,
    /// `SystemUnit`: the unit used when the file's unit is unknown, 1.0 (MM) by default.
    pub system_unit: f64,
}
impl Default for DeSectionGlobal {
    fn default() -> Self {
        Self {
            length_unit: 1.0,
            system_unit: 1.0,
        }
    }
}

/// The locked fields: `GlobalParameters` and `myIsEnabled`.
#[derive(Clone, Copy, Debug)]
struct NodeState {
    global_parameters: DeSectionGlobal,
    is_enabled: bool,
}

/// The fields of `DeConfigurationNode`; `DeConfigurationNodeBase::default()` is the
/// default constructor (enabled, units 1.0).
pub struct DeConfigurationNodeBase {
    this: OnceLock<WeakHandle<dyn DeConfigurationNode>>,
    state: SharedState<NodeState>,
}
impl Default for DeConfigurationNodeBase {
    fn default() -> Self {
        Self::with_state(DeSectionGlobal::default(), true)
    }
}

impl DeConfigurationNodeBase {
    fn with_state(global_parameters: DeSectionGlobal, is_enabled: bool) -> Self {
        Self {
            this: OnceLock::new(),
            state: SharedState::new(NodeState {
                global_parameters,
                is_enabled,
            }),
        }
    }
    /// `DeConfigurationNode(theConfigurationNode)`: copies the units and the loading
    /// status of another node; a null node raises `StandardNullObject` (a native null
    /// dereference); `DeConfigurationNodeBase::copied_from(&node)?`.
    pub fn copied_from(node: &Handle<dyn DeConfigurationNode>) -> Result<Self, Failure> {
        let source: &dyn DeConfigurationNode = node.get().ok_or_else(|| {
            exceptions::null_object(
                "DEConfigurationNode(theConfigurationNode): null node; expected a live node",
            )
        })?;
        Ok(Self::with_state(
            source.global_parameters(),
            source.is_enabled(),
        ))
    }
    /// Copies enabled state and unit settings without retaining an owning handle.
    /// Example: `let copy = DeConfigurationNodeBase::default().copied_state();`.
    pub fn copied_state(&self) -> Self {
        Self::with_state(self.global_parameters(), self.is_enabled())
    }
    /// Records the owning handle of this node, once (ocaf-design.md section 2);
    /// `node.configuration_node_base().bind_self(&node)`. `DeWrapper::bind` binds every node
    /// it receives.
    pub fn bind_self(&self, node: &Handle<dyn DeConfigurationNode>) {
        let _ = self.this.set(node.downgrade());
    }
    /// The handle `this` converts to natively; `StandardNullObject` before
    /// [`DeConfigurationNodeBase::bind_self`]; `base.this()?`.
    pub fn this(&self) -> Result<Handle<dyn DeConfigurationNode>, Failure> {
        let this = self.this.get().map(WeakHandle::upgrade).unwrap_or_default();
        if this.is_null() {
            return Err(exceptions::null_object(
                "DEConfigurationNode: the node has no owning handle; expected bind_self after creation",
            ));
        }
        Ok(this)
    }
    /// `IsEnabled()`; `base.is_enabled()`.
    pub fn is_enabled(&self) -> bool {
        self.state.read().is_enabled
    }
    /// `SetEnabled(theIsLoaded)`; `base.set_enabled(false)`.
    pub fn set_enabled(&self, is_loaded: bool) {
        self.state.write().is_enabled = is_loaded;
    }
    /// The `GlobalParameters` field; `base.global_parameters().length_unit`.
    pub fn global_parameters(&self) -> DeSectionGlobal {
        self.state.read().global_parameters
    }
    /// Assigns the `GlobalParameters` field; `base.set_global_parameters(units)`.
    pub fn set_global_parameters(&self, parameters: DeSectionGlobal) {
        self.state.write().global_parameters = parameters;
    }
}

/// Wraps a new node in its owning handle and binds `this`; the Rust form of
/// `new TheNode()` for a node that registers itself or builds providers;
/// `let node = new_configuration_node(MyNode::default());`.
pub fn new_configuration_node<T>(node: T) -> Handle<T>
where
    T: DeConfigurationNode + martensite_tkernel::standard::RuntimeTyped,
{
    let handle: Handle<T> = Handle::new(node);
    if let Some(object) = handle.get() {
        object.configuration_node_base().bind_self(&handle.cast());
    }
    handle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copying_state_keeps_independent_fields_and_no_owning_handle() {
        let source = DeConfigurationNodeBase::default();
        source.set_enabled(false);
        let units = DeSectionGlobal {
            length_unit: 25.4,
            system_unit: 0.001,
        };
        source.set_global_parameters(units);
        let copied = source.copied_state();
        source.set_enabled(true);
        source.set_global_parameters(DeSectionGlobal::default());
        assert!(!copied.is_enabled());
        assert_eq!(copied.global_parameters(), units);
        assert_eq!(
            copied.this().unwrap_err().exception_type(),
            "Standard_NullObject"
        );
    }

    #[test]
    fn a_new_base_is_enabled_with_unit_scales() {
        let base = DeConfigurationNodeBase::default();
        assert!(base.is_enabled());
        assert_eq!(base.global_parameters(), DeSectionGlobal::default());
        base.set_enabled(false);
        base.set_global_parameters(DeSectionGlobal {
            length_unit: 25.4,
            system_unit: 0.001,
        });
        assert!(!base.is_enabled());
        assert_eq!(base.global_parameters().length_unit, 25.4);
    }

    #[test]
    fn an_unbound_base_has_no_this_and_a_null_source_is_refused() {
        let base = DeConfigurationNodeBase::default();
        assert_eq!(
            base.this()
                .err()
                .map(|f| f.exception_type().to_owned())
                .as_deref(),
            Some("Standard_NullObject")
        );
        let null: Handle<dyn DeConfigurationNode> = Handle::default();
        assert!(DeConfigurationNodeBase::copied_from(&null).is_err());
    }
}
