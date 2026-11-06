//! The oriented classes redefine the fields of their base: through virtual members, so
//! that a `StepShapeEdge` handle on an oriented edge answers from the element it orients. A Rust
//! handle on the base reaches the embedded base struct only, so the base holds the element and the
//! orientation, and each member tests the `oriented` flag the oriented constructor raises.
//!
//! The element dereferences stay recorded: `Bounds`, `CfsFaces` and `edge_list` dereference the
//! element without a test, so they fail with a null object failure here, which makes those members
//! fallible on the base too. `EdgeStart` and `EdgeEnd` test it and answer null.
#[allow(
    unused_imports,
    reason = "the classes of the package reference each other"
)]
use super::*;
use crate::step_entity::*;

/// The `edge_list` field.
type OrientedEdgeList = Handle<HArray1<Handle<StepShapeOrientedEdge>>>;

/// The notice of a setter on a redefined field, which then changes nothing.
fn forbidden_setup_notice() {
    println!("Field is redefined, SetUp Forbidden");
}

step_entity! {
    /// Edge.
    StepShapeEdge, StepShapeEdgeState, "StepShapeEdge", StepShapeTopologicalRepresentationItem, {
        edge_start: Handle<StepShapeVertex>,
        edge_end: Handle<StepShapeVertex>,
        oriented: bool,
        edge_element: Handle<StepShapeEdge>,
        orientation: bool,
    }
}
impl StepShapeEdge {
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_edge_start: &Handle<StepShapeVertex>,
        a_edge_end: &Handle<StepShapeVertex>,
    ) {
        {
            let mut state = self.state.write();
            state.edge_start = a_edge_start.clone();
            state.edge_end = a_edge_end.clone();
        }
        self.parent.init(a_name);
    }
    /// `set_edge_start`; forbidden on an oriented edge.
    pub fn set_edge_start(&self, a_edge_start: &Handle<StepShapeVertex>) {
        let mut state = self.state.write();
        if state.oriented {
            return forbidden_setup_notice();
        }
        state.edge_start = a_edge_start.clone();
    }
    /// `EdgeStart`; an oriented edge answers its element's start or end.
    pub fn edge_start(&self) -> Handle<StepShapeVertex> {
        self.oriented_vertex(true)
    }
    /// `set_edge_end`; forbidden on an oriented edge.
    pub fn set_edge_end(&self, a_edge_end: &Handle<StepShapeVertex>) {
        let mut state = self.state.write();
        if state.oriented {
            return forbidden_setup_notice();
        }
        state.edge_end = a_edge_end.clone();
    }
    /// `EdgeEnd`; an oriented edge answers its element's end or start.
    pub fn edge_end(&self) -> Handle<StepShapeVertex> {
        self.oriented_vertex(false)
    }
    /// The start (`first`) or the end, read through the element of an oriented edge.
    fn oriented_vertex(&self, first: bool) -> Handle<StepShapeVertex> {
        let (element, same_sense) = {
            let state = self.state.read();
            if !state.oriented {
                return if first {
                    state.edge_start.clone()
                } else {
                    state.edge_end.clone()
                };
            }
            (state.edge_element.clone(), state.orientation)
        };
        match element.get() {
            None => Handle::default(),
            Some(edge) if first == same_sense => edge.edge_start(),
            Some(edge) => edge.edge_end(),
        }
    }
}

step_entity! {
    /// Oriented edge.
    custom_default StepShapeOrientedEdge, StepShapeOrientedEdgeState, "StepShapeOrientedEdge", StepShapeEdge, {}
}
impl Default for StepShapeOrientedEdge {
    fn default() -> Self {
        let object = Self {
            parent: StepShapeEdge::default(),
            state: SharedState::default(),
        };
        object.parent.state.write().oriented = true;
        object
    }
}
impl StepShapeOrientedEdge {
    /// `StepShapeOrientedEdge()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `init`: the inherited start and end stay null.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_edge_element: &Handle<StepShapeEdge>,
        a_orientation: bool,
    ) {
        self.set_edge_element(a_edge_element);
        self.set_orientation(a_orientation);
        self.parent
            .init(a_name, &Handle::default(), &Handle::default());
    }
    /// `set_edge_element`.
    pub fn set_edge_element(&self, a_edge_element: &Handle<StepShapeEdge>) {
        self.parent.state.write().edge_element = a_edge_element.clone();
    }
    /// `edge_element`.
    pub fn edge_element(&self) -> Handle<StepShapeEdge> {
        self.parent.state.read().edge_element.clone()
    }
    /// `set_orientation`.
    pub fn set_orientation(&self, a_orientation: bool) {
        self.parent.state.write().orientation = a_orientation;
    }
    /// `Orientation`.
    pub fn orientation(&self) -> bool {
        self.parent.state.read().orientation
    }
}

/// The accessors a base with a list field shares; `$element` holds the redefining element.
macro_rules! redefined_list {
    ($class:ident, $recorded:literal, $item:ty, $field:ident, $set:ident, $get:ident, $value:ident, $count:ident, $count_recorded:literal) => {
        impl $class {
            /// Sets the list; forbidden on an oriented object.
            pub fn $set(&self, list: &Handle<HArray1<Handle<$item>>>) {
                let mut state = self.state.write();
                if state.oriented {
                    return forbidden_setup_notice();
                }
                state.$field = list.clone();
            }
            /// The list; an oriented object answers from its element.
            ///
            /// # Errors
            /// a null object failure when an oriented object has no element.
            pub fn $get(&self) -> Result<Handle<HArray1<Handle<$item>>>, Failure> {
                let (list, element) = self.list_or_element();
                match element {
                    None => Ok(list),
                    Some(element) => step_live(&element, $recorded)?.$get(),
                }
            }
            /// The item at a 1-based rank; an oriented object answers from its element.
            ///
            /// # Errors
            /// a null object failure for a null list or element, an out of range failure for a bad rank.
            pub fn $value(&self, num: i32) -> Result<Handle<$item>, Failure> {
                let (list, element) = self.list_or_element();
                match element {
                    None => step_array1_item(&list, num, $recorded),
                    Some(element) => step_live(&element, $recorded)?.$value(num),
                }
            }
            #[doc = concat!("`", $count_recorded, "`: 0 for a null list; an oriented object answers from its element.\n\n# Errors\n`Standard_NullObject` when an oriented object has no element.")]
            pub fn $count(&self) -> Result<i32, Failure> {
                let (list, element) = self.list_or_element();
                match element {
                    None => Ok(step_array1_len(&list)),
                    Some(element) => step_live(&element, $recorded)?.$count(),
                }
            }
            /// The own list, or the element when the object is oriented.
            fn list_or_element(&self) -> (Handle<HArray1<Handle<$item>>>, Option<Handle<$class>>) {
                let state = self.state.read();
                let element = state.oriented.then(|| state.element.clone());
                (state.$field.clone(), element)
            }
        }
    };
}

/// The constructor and element accessors of a class that orients an element of `$base`.
macro_rules! oriented_class {
    ($class:ident, $base:ident, $element:ty, $set:ident, $get:ident, $list:ty) => {
        impl Default for $class {
            fn default() -> Self {
                let object = Self {
                    parent: $base::default(),
                    state: SharedState::default(),
                };
                object.parent.state.write().oriented = true;
                object
            }
        }
        impl $class {
            /// A new oriented object.
            pub fn new() -> Self {
                Self::default()
            }
            /// `init`: the inherited list stays null.
            pub fn init(
                &self,
                a_name: &Handle<HAsciiString>,
                a_element: &Handle<$element>,
                a_orientation: bool,
            ) {
                self.$set(a_element);
                self.set_orientation(a_orientation);
                self.parent.init(a_name, &<$list>::default());
            }
            /// Sets the oriented element.
            pub fn $set(&self, a_element: &Handle<$element>) {
                self.parent.state.write().element = a_element.cast();
            }
            /// The oriented element.
            pub fn $get(&self) -> Handle<$element> {
                self.parent.state.read().element.cast()
            }
            /// `set_orientation`.
            pub fn set_orientation(&self, a_orientation: bool) {
                self.parent.state.write().orientation = a_orientation;
            }
            /// `Orientation`.
            pub fn orientation(&self) -> bool {
                self.parent.state.read().orientation
            }
        }
    };
}

step_entity! {
    /// Face.
    StepShapeFace, StepShapeFaceState, "StepShapeFace", StepShapeTopologicalRepresentationItem, {
        bounds: Handle<HArray1<Handle<StepShapeFaceBound>>>,
        oriented: bool,
        element: Handle<StepShapeFace>,
        orientation: bool,
    }
}
impl StepShapeFace {
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_bounds: &Handle<HArray1<Handle<StepShapeFaceBound>>>,
    ) {
        self.state.write().bounds = a_bounds.clone();
        self.parent.init(a_name);
    }
}
redefined_list!(
    StepShapeFace,
    "StepShapeFace",
    StepShapeFaceBound,
    bounds,
    set_bounds,
    bounds,
    bounds_value,
    nb_bounds,
    "NbBounds"
);
step_entity! {
    /// Oriented face.
    custom_default StepShapeOrientedFace, StepShapeOrientedFaceState, "StepShapeOrientedFace", StepShapeFace, {}
}
oriented_class!(
    StepShapeOrientedFace,
    StepShapeFace,
    StepShapeFace,
    set_face_element,
    face_element,
    Handle<HArray1<Handle<StepShapeFaceBound>>>
);

step_entity! {
    /// Connected face set.
    StepShapeConnectedFaceSet, StepShapeConnectedFaceSetState, "StepShapeConnectedFaceSet", StepShapeTopologicalRepresentationItem, {
        cfs_faces: Handle<HArray1<Handle<StepShapeFace>>>,
        oriented: bool,
        element: Handle<StepShapeConnectedFaceSet>,
        orientation: bool,
    }
}
impl StepShapeConnectedFaceSet {
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_cfs_faces: &Handle<HArray1<Handle<StepShapeFace>>>,
    ) {
        self.state.write().cfs_faces = a_cfs_faces.clone();
        self.parent.init(a_name);
    }
}
redefined_list!(
    StepShapeConnectedFaceSet,
    "StepShapeConnectedFaceSet",
    StepShapeFace,
    cfs_faces,
    set_cfs_faces,
    cfs_faces,
    cfs_faces_value,
    nb_cfs_faces,
    "NbCfsFaces"
);
step_entity! {
    /// Oriented closed shell.
    custom_default StepShapeOrientedClosedShell, StepShapeOrientedClosedShellState, "StepShapeOrientedClosedShell", StepShapeConnectedFaceSet, {}
}
oriented_class!(
    StepShapeOrientedClosedShell,
    StepShapeConnectedFaceSet,
    StepShapeClosedShell,
    set_closed_shell_element,
    closed_shell_element,
    Handle<HArray1<Handle<StepShapeFace>>>
);
step_entity! {
    /// Oriented open shell.
    custom_default StepShapeOrientedOpenShell, StepShapeOrientedOpenShellState, "StepShapeOrientedOpenShell", StepShapeConnectedFaceSet, {}
}
oriented_class!(
    StepShapeOrientedOpenShell,
    StepShapeConnectedFaceSet,
    StepShapeOpenShell,
    set_open_shell_element,
    open_shell_element,
    Handle<HArray1<Handle<StepShapeFace>>>
);

step_entity! {
    /// Path.
    StepShapePath, StepShapePathState, "StepShapePath", StepShapeTopologicalRepresentationItem, {
        edge_list: Handle<HArray1<Handle<StepShapeOrientedEdge>>>,
        oriented: bool,
        path_element: Handle<StepShapeEdgeLoop>,
        orientation: bool,
    }
}
impl StepShapePath {
    /// `init`.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_edge_list: &Handle<HArray1<Handle<StepShapeOrientedEdge>>>,
    ) {
        self.state.write().edge_list = a_edge_list.clone();
        self.parent.init(a_name);
    }
    /// `set_edge_list`; forbidden on an oriented path.
    pub fn set_edge_list(&self, a_edge_list: &Handle<HArray1<Handle<StepShapeOrientedEdge>>>) {
        let mut state = self.state.write();
        if state.oriented {
            return forbidden_setup_notice();
        }
        state.edge_list = a_edge_list.clone();
    }
    /// `edge_list`; an oriented path answers its element's list in either orientation
    /// (the code leaves building the reversed list for later).
    ///
    /// # Errors
    /// a null object failure when an oriented path has no element.
    pub fn edge_list(&self) -> Result<Handle<HArray1<Handle<StepShapeOrientedEdge>>>, Failure> {
        let (list, element, _) = self.path_parts();
        match element {
            None => Ok(list),
            Some(element) => {
                Ok(step_live(&element, "StepShapeOrientedPath::EdgeList")?.edge_list())
            }
        }
    }
    /// `edge_list_value`; an oriented path counts from the other end when reversed.
    ///
    /// # Errors
    /// a null object failure for a null list or element, an out of range failure for a bad rank.
    pub fn edge_list_value(&self, num: i32) -> Result<Handle<StepShapeOrientedEdge>, Failure> {
        let (list, element, same_sense) = self.path_parts();
        let Some(element) = element else {
            return step_array1_item(&list, num, "StepShapePath::EdgeListValue");
        };
        let path = step_live(&element, "StepShapeOrientedPath::EdgeListValue")?;
        if same_sense {
            return path.edge_list_value(num);
        }
        path.edge_list_value(path.nb_edge_list() - num + 1)
    }
    /// `nb_edge_list`; 0 for a null list, and an oriented path answers from its element.
    ///
    /// # Errors
    /// a null object failure when an oriented path has no element.
    pub fn nb_edge_list(&self) -> Result<i32, Failure> {
        let (list, element, _) = self.path_parts();
        match element {
            None => Ok(step_array1_len(&list)),
            Some(element) => {
                Ok(step_live(&element, "StepShapeOrientedPath::NbEdgeList")?.nb_edge_list())
            }
        }
    }
    /// The own list, the element of an oriented path, and its orientation.
    fn path_parts(&self) -> (OrientedEdgeList, Option<Handle<StepShapeEdgeLoop>>, bool) {
        let state = self.state.read();
        let element = state.oriented.then(|| state.path_element.clone());
        (state.edge_list.clone(), element, state.orientation)
    }
}
step_entity! {
    /// Oriented path.
    custom_default StepShapeOrientedPath, StepShapeOrientedPathState, "StepShapeOrientedPath", StepShapePath, {}
}
impl Default for StepShapeOrientedPath {
    fn default() -> Self {
        let object = Self {
            parent: StepShapePath::default(),
            state: SharedState::default(),
        };
        object.parent.state.write().oriented = true;
        object
    }
}
impl StepShapeOrientedPath {
    /// `StepShapeOrientedPath()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `init`: the inherited list stays null.
    pub fn init(
        &self,
        a_name: &Handle<HAsciiString>,
        a_path_element: &Handle<StepShapeEdgeLoop>,
        a_orientation: bool,
    ) {
        self.set_path_element(a_path_element);
        self.set_orientation(a_orientation);
        self.parent.init(a_name, &Handle::default());
    }
    /// `set_path_element`.
    pub fn set_path_element(&self, a_path_element: &Handle<StepShapeEdgeLoop>) {
        self.parent.state.write().path_element = a_path_element.clone();
    }
    /// `path_element`.
    pub fn path_element(&self) -> Handle<StepShapeEdgeLoop> {
        self.parent.state.read().path_element.clone()
    }
    /// `set_orientation`.
    pub fn set_orientation(&self, a_orientation: bool) {
        self.parent.state.write().orientation = a_orientation;
    }
    /// `Orientation`.
    pub fn orientation(&self) -> bool {
        self.parent.state.read().orientation
    }
}

#[cfg(test)]
#[path = "redefined_tests.rs"]
mod tests;
