//! `ShapeUpgradeTool`: the context and tolerances every ShapeUpgrade tool shares.
use crate::shape_build::ShapeBuildReShape;
use martensite_tkernel::impl_transient;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, SharedState, Transient};

/// The fields of a `ShapeUpgradeTool`.
#[derive(Clone)]
struct ToolFields {
    context: Handle<ShapeBuildReShape>,
    precision: f64,
    min_tol: f64,
    max_tol: f64,
}

/// `ShapeUpgradeTool`: a re-shaping context, a working precision and the tolerance
/// limits, the root of the ShapeUpgrade tools.
///
/// ```
/// use martensite_tkshhealing::shape_upgrade::ShapeUpgradeTool;
/// let tool = ShapeUpgradeTool::new();
/// tool.set_precision(2.0);
/// assert_eq!(tool.max_tolerance(), 2.0);
/// assert_eq!(tool.limit_tolerance(5.0), 2.0);
/// ```
#[derive(Default)]
pub struct ShapeUpgradeTool {
    data: UpgradeToolData,
}

/// Context and tolerances shared by upgrade tools. Example: `UpgradeToolData::default()`.
pub struct UpgradeToolData {
    fields: SharedState<ToolFields>,
}

/// The real owner of the common upgrade tool fields.
pub trait UpgradeTool: Transient {
    /// The common tool fields. Example: `tool.upgrade_tool_data().precision()`.
    fn upgrade_tool_data(&self) -> &UpgradeToolData;
}
parent_interface!(dyn UpgradeTool, ShapeUpgradeTool, []);

impl UpgradeTool for ShapeUpgradeTool {
    fn upgrade_tool_data(&self) -> &UpgradeToolData {
        &self.data
    }
}

impl std::ops::Deref for dyn UpgradeTool {
    type Target = UpgradeToolData;
    fn deref(&self) -> &Self::Target {
        self.upgrade_tool_data()
    }
}

impl std::ops::Deref for ShapeUpgradeTool {
    type Target = UpgradeToolData;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl_transient!(
    ShapeUpgradeTool,
    "ShapeUpgrade_Tool",
    dyn Transient,
    interfaces: [dyn UpgradeTool]
);

impl ShapeUpgradeTool {
    /// no context, precision and minimal tolerance
    /// `precision::confusion()`, maximum tolerance 1;
    /// `ShapeUpgradeTool::new()`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for UpgradeToolData {
    fn default() -> Self {
        Self {
            fields: SharedState::new(ToolFields {
                context: Handle::default(),
                precision: precision::confusion(),
                min_tol: precision::confusion(),
                max_tol: 1.0,
            }),
        }
    }
}

impl UpgradeToolData {
    /// copies the context, the precision and the tolerances of `tool`;
    /// Standard_NullObject for a null tool;
    /// `tool.set(&other)?`.
    pub fn set<T: UpgradeTool + ?Sized>(&self, tool: &Handle<T>) -> Result<(), Failure> {
        let source = tool.get().ok_or_else(|| {
            Failure::with_type(
                "Standard_NullObject",
                "ShapeUpgrade_Tool::Set: null tool",
                "",
            )
        })?;
        let copied = source.upgrade_tool_data().fields.read().clone();
        *self.fields.write() = copied;
        Ok(())
    }

    /// `SetContext(context)`; `tool.set_context(&reshape)`.
    pub fn set_context(&self, context: &Handle<ShapeBuildReShape>) {
        self.fields.write().context = context.clone();
    }

    /// `Context()`; `tool.context()`.
    pub fn context(&self) -> Handle<ShapeBuildReShape> {
        self.fields.read().context.clone()
    }

    /// the precision, the maximal tolerance raised to it and the
    /// minimal one lowered to it; `tool.set_precision(1e-3)`.
    pub fn set_precision(&self, preci: f64) {
        let mut fields = self.fields.write();
        fields.precision = preci;
        if fields.max_tol < fields.precision {
            fields.max_tol = fields.precision;
        }
        if fields.min_tol > fields.precision {
            fields.min_tol = fields.precision;
        }
    }

    /// `Precision()`; `tool.precision()`.
    pub fn precision(&self) -> f64 {
        self.fields.read().precision
    }

    /// `SetMinTolerance(mintol)`; `tool.set_min_tolerance(1e-7)`.
    pub fn set_min_tolerance(&self, mintol: f64) {
        self.fields.write().min_tol = mintol;
    }

    /// `MinTolerance()`; `tool.min_tolerance()`.
    pub fn min_tolerance(&self) -> f64 {
        self.fields.read().min_tol
    }

    /// `SetMaxTolerance(maxtol)`; `tool.set_max_tolerance(1.0)`.
    pub fn set_max_tolerance(&self, maxtol: f64) {
        self.fields.write().max_tol = maxtol;
    }

    /// `MaxTolerance()`; `tool.max_tolerance()`.
    pub fn max_tolerance(&self) -> f64 {
        self.fields.read().max_tol
    }

    /// `std::min(MaxTolerance(), toler)` (the maximal tolerance
    /// when `toler` is NaN, as `std::min`); `tool.limit_tolerance(5.0)`.
    pub fn limit_tolerance(&self, toler: f64) -> f64 {
        let max_tol = self.fields.read().max_tol;
        if toler < max_tol { toler } else { max_tol }
    }
}
