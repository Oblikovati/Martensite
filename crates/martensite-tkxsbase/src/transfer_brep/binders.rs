//! The binders of shape results: one shape (`BinderOfShape`, `ShapeBinder` with its
//! typed accessors) or a list of shapes (`ShapeListBinder`).
use super::TransferBRepShapeInfo;
use crate::transfer::{TransferBinder, TransferBinderBase};
use martensite_tkbrep::topods::{
    self, CompSolid, Compound, Edge, Face, Shape, Shell, Solid, Vertex, Wire,
};
use martensite_tkernel::{
    impl_transient, impl_transient_interface_target,
    standard::{Failure, HandleUpcast, RuntimeType, RuntimeTyped, SharedState, exceptions},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use std::ops::Deref;
use std::sync::{Arc, RwLockWriteGuard};

/// The shared status and shape slot of a single-shape binder.
#[derive(Default)]
pub struct TransferBRepShapeResult {
    base: TransferBinderBase,
    shape: SharedState<Shape>,
}

impl TransferBRepShapeResult {
    fn with_result(result: &Shape) -> Self {
        let state = Self {
            base: TransferBinderBase::default(),
            shape: SharedState::new(result.clone()),
        };
        // New binders have no consumers, so marking the result present cannot fail.
        let _ = state.base.set_result_present();
        state
    }

    /// Replace the shape while it is still unused, e.g. `binder.set_result(&shape)`.
    pub fn set_result(&self, result: &Shape) -> Result<(), Failure> {
        *self.c_result()? = result.clone();
        Ok(())
    }

    /// Copy the published shape, e.g. `let shape = binder.result();`.
    pub fn result(&self) -> Shape {
        self.shape.read().clone()
    }

    /// Mark the result present and borrow its shape for writing, e.g. `*binder.c_result()? = shape;`.
    pub fn c_result(&self) -> Result<RwLockWriteGuard<'_, Shape>, Failure> {
        self.base.set_result_present()?;
        Ok(self.shape.write())
    }

    fn result_type_name(&self) -> String {
        TransferBRepShapeInfo::type_name(&self.shape.read()).to_owned()
    }
}

/// The shared shape-result interface of single-shape binders.
pub trait TransferBRepShapeBinderOps:
    TransferBinder + Deref<Target = TransferBRepShapeResult>
{
}

impl RuntimeTyped for dyn TransferBRepShapeBinderOps {
    const TYPE_NAME: &'static str = TransferBRepBinderOfShape::TYPE_NAME;

    fn static_type() -> &'static RuntimeType {
        TransferBRepBinderOfShape::static_type()
    }
}
impl_transient_interface_target!(dyn TransferBRepShapeBinderOps);
impl HandleUpcast<dyn TransferBinder> for dyn TransferBRepShapeBinderOps {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn TransferBinder> {
        owner
    }
}

macro_rules! impl_shape_binder {
    ($binder:ty) => {
        impl Deref for $binder {
            type Target = TransferBRepShapeResult;

            fn deref(&self) -> &Self::Target {
                &self.result
            }
        }

        impl TransferBinder for $binder {
            fn binder_base(&self) -> &TransferBinderBase {
                &self.result.base
            }

            fn result_type(&self) -> &'static RuntimeType {
                TransferBRepShapeInfo::type_of(&self.result.shape.read())
            }

            fn result_type_name(&self) -> String {
                self.result.result_type_name()
            }
        }

        impl TransferBRepShapeBinderOps for $binder {}
    };
}

/// `TransferBRepBinderOfShape`: one shape result.
///
/// ```
/// use martensite_tkxsbase::transfer::TransferBinder;
/// use martensite_tkxsbase::transfer_brep::TransferBRepBinderOfShape;
/// let binder = TransferBRepBinderOfShape::new();
/// assert!(!binder.has_result() && binder.result_type_name() == "TopoDS_Shape");
/// ```
#[derive(Default)]
pub struct TransferBRepBinderOfShape {
    result: TransferBRepShapeResult,
}
impl_transient!(TransferBRepBinderOfShape, "TransferBRepBinderOfShape", dyn TransferBinder, interfaces: [dyn TransferBRepShapeBinderOps, dyn TransferBinder]);
impl_shape_binder!(TransferBRepBinderOfShape);

impl TransferBRepBinderOfShape {
    /// Construct an empty binder with `TransferBRepBinderOfShape::new()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a populated binder with `TransferBRepBinderOfShape::with_result(&shape)`.
    pub fn with_result(result: &Shape) -> Self {
        Self {
            result: TransferBRepShapeResult::with_result(result),
        }
    }
}

/// A single-shape binder with typed accessors.
///
/// ```
/// use martensite_tkxsbase::transfer_brep::TransferBRepShapeBinder;
/// use martensite_tkg3d::top_abs::ShapeEnum;
/// assert_eq!(TransferBRepShapeBinder::new().shape_type().unwrap(), ShapeEnum::Shape);
/// ```
#[derive(Default)]
pub struct TransferBRepShapeBinder {
    result: TransferBRepShapeResult,
}
impl_transient!(TransferBRepShapeBinder, "TransferBRepShapeBinder", dyn TransferBRepShapeBinderOps, interfaces: [dyn TransferBRepShapeBinderOps, dyn TransferBinder]);
impl_shape_binder!(TransferBRepShapeBinder);

impl TransferBRepShapeBinder {
    /// `TransferBRepShapeBinder()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// `TransferBRepShapeBinder(res)`.
    pub fn with_result(result: &Shape) -> Self {
        Self {
            result: TransferBRepShapeResult::with_result(result),
        }
    }
    /// `ShapeType`: `TopAbsSHAPE` without result.
    pub fn shape_type(&self) -> Result<ShapeEnum, Failure> {
        if !self.has_result() {
            return Ok(ShapeEnum::Shape);
        }
        self.result().shape_type()
    }
    /// `Vertex`: `StandardTypeMismatch` for another type.
    pub fn vertex(&self) -> Result<Vertex, Failure> {
        topods::vertex(&self.result())
    }
    /// `Edge`.
    pub fn edge(&self) -> Result<Edge, Failure> {
        topods::edge(&self.result())
    }
    /// `Wire`.
    pub fn wire(&self) -> Result<Wire, Failure> {
        topods::wire(&self.result())
    }
    /// `Face`.
    pub fn face(&self) -> Result<Face, Failure> {
        topods::face(&self.result())
    }
    /// `Shell`.
    pub fn shell(&self) -> Result<Shell, Failure> {
        topods::shell(&self.result())
    }
    /// `Solid`.
    pub fn solid(&self) -> Result<Solid, Failure> {
        topods::solid(&self.result())
    }
    /// `CompSolid`.
    pub fn comp_solid(&self) -> Result<CompSolid, Failure> {
        topods::comp_solid(&self.result())
    }
    /// `Compound`.
    pub fn compound(&self) -> Result<Compound, Failure> {
        topods::compound(&self.result())
    }
}

/// `TransferBRepShapeListBinder`: a list of shape results (`None` is the native null
/// list of the constructor from a null sequence).
///
/// ```
/// use martensite_tkxsbase::transfer_brep::TransferBRepShapeListBinder;
/// use martensite_tkbrep::topods::Shape;
/// let list = TransferBRepShapeListBinder::new();
/// list.add_result(&Shape::default()).unwrap();
/// assert_eq!(list.nb_shapes().unwrap(), 1);
/// ```
pub struct TransferBRepShapeListBinder {
    base: TransferBinderBase,
    theres: SharedState<Option<Vec<Shape>>>,
}
impl_transient!(TransferBRepShapeListBinder, "TransferBRepShapeListBinder", dyn TransferBinder, interfaces: [dyn TransferBinder]);

impl Default for TransferBRepShapeListBinder {
    fn default() -> Self {
        Self::new()
    }
}

/// The native null dereference of a null list.
fn null_list(member: &str) -> Failure {
    exceptions::null_object(&format!(
        "TransferBRepShapeListBinder::{member}: null list; expected a list of shapes"
    ))
}

impl TransferBRepShapeListBinder {
    /// `TransferBRepShapeListBinder()`: an empty list.
    pub fn new() -> Self {
        Self::with_list(Some(Vec::new()))
    }
    /// `TransferBRepShapeListBinder(list)`: `None` is a null list.
    pub fn with_list(list: Option<Vec<Shape>>) -> Self {
        Self {
            base: TransferBinderBase::default(),
            theres: SharedState::new(list),
        }
    }
    /// `AddResult(res)`.
    pub fn add_result(&self, res: &Shape) -> Result<(), Failure> {
        self.theres
            .write()
            .as_mut()
            .ok_or_else(|| null_list("AddResult"))?
            .push(res.clone());
        Ok(())
    }
    /// `Result`: a copy of the list (`None` when null).
    pub fn result(&self) -> Option<Vec<Shape>> {
        self.theres.read().clone()
    }
    /// `SetResult(num, res)`: `StandardOutOfRange` out of the list.
    pub fn set_result(&self, num: i32, res: &Shape) -> Result<(), Failure> {
        let mut st = self.theres.write();
        let list = st.as_mut().ok_or_else(|| null_list("SetResult"))?;
        let len = list.len();
        let slot = usize::try_from(num - 1).ok().and_then(|i| list.get_mut(i));
        *slot.ok_or_else(|| {
            exceptions::out_of_range(&format!(
                "TransferBRepShapeListBinder::SetResult: {num}; expected [1, {len}]"
            ))
        })? = res.clone();
        Ok(())
    }
    /// `NbShapes`.
    pub fn nb_shapes(&self) -> Result<i32, Failure> {
        Ok(self
            .theres
            .read()
            .as_ref()
            .ok_or_else(|| null_list("NbShapes"))?
            .len() as i32)
    }
    /// `Shape(num)`: `StandardOutOfRange` out of the list.
    pub fn shape(&self, num: i32) -> Result<Shape, Failure> {
        let st = self.theres.read();
        let list = st.as_ref().ok_or_else(|| null_list("Shape"))?;
        let found = usize::try_from(num - 1).ok().and_then(|i| list.get(i));
        found.cloned().ok_or_else(|| {
            exceptions::out_of_range(&format!(
                "TransferBRepShapeListBinder::Shape: {num}; expected [1, {}]",
                list.len()
            ))
        })
    }
    /// `ShapeType(num)`.
    pub fn shape_type(&self, num: i32) -> Result<ShapeEnum, Failure> {
        self.shape(num)?.shape_type()
    }
    /// `Vertex(num)`.
    pub fn vertex(&self, num: i32) -> Result<Vertex, Failure> {
        topods::vertex(&self.shape(num)?)
    }
    /// `Edge(num)`.
    pub fn edge(&self, num: i32) -> Result<Edge, Failure> {
        topods::edge(&self.shape(num)?)
    }
    /// `Wire(num)`.
    pub fn wire(&self, num: i32) -> Result<Wire, Failure> {
        topods::wire(&self.shape(num)?)
    }
    /// `Face(num)`.
    pub fn face(&self, num: i32) -> Result<Face, Failure> {
        topods::face(&self.shape(num)?)
    }
    /// `Shell(num)`.
    pub fn shell(&self, num: i32) -> Result<Shell, Failure> {
        topods::shell(&self.shape(num)?)
    }
    /// `Solid(num)`.
    pub fn solid(&self, num: i32) -> Result<Solid, Failure> {
        topods::solid(&self.shape(num)?)
    }
    /// `CompSolid(num)`.
    pub fn comp_solid(&self, num: i32) -> Result<CompSolid, Failure> {
        topods::comp_solid(&self.shape(num)?)
    }
    /// `Compound(num)`.
    pub fn compound(&self, num: i32) -> Result<Compound, Failure> {
        topods::compound(&self.shape(num)?)
    }
}

impl TransferBinder for TransferBRepShapeListBinder {
    fn binder_base(&self) -> &TransferBinderBase {
        &self.base
    }
    /// `IsMultiple`: more than one shape (a null list is natively dereferenced: false).
    fn is_multiple(&self) -> bool {
        self.nb_shapes().is_ok_and(|n| n > 1)
    }
    /// `ResultType`: `TransferBRepShapeListBinder` itself.
    fn result_type(&self) -> &'static RuntimeType {
        Self::static_type()
    }
    /// `ResultTypeName`: "list(Shape)".
    fn result_type_name(&self) -> String {
        String::from("list(TopoDS_Shape)")
    }
}
