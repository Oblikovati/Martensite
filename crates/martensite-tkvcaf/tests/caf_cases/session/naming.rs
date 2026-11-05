//! The object, function and attachment commands the orientation scripts build a box with, and the
//! box driver they compute it by: the same document layout and library calls.
use super::commands::{CafSession, entry_of};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::MapOfShape;
use martensite_tkcaf::tnaming::{
    TNaming, TNamingBuilder, TNamingNamedShape, TNamingSelector, TNamingTool,
};
use martensite_tkernel::impl_transient;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::standard::{Failure, Guid, Handle, StandardTransient};
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::top_abs::{ShapeEnum, shape_orientation_to_string, shape_type_to_string};
use martensite_tklcaf::guid_from_text;
use martensite_tklcaf::tdata_std::{
    TDataStdName, TDataStdReal, TDataStdTreeNode, TDataStdUAttribute,
};
use martensite_tklcaf::tdf::{TdfAttributeState, TdfLabel, TdfReference, TdfTagSource};
use martensite_tklcaf::tfunction::{
    TFunctionDriver, TFunctionDriverBase, TFunctionDriverTable, TFunctionFunction, TFunctionLogbook,
};
use martensite_tkmath::toploc::Location;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

const GEOM_OBJECT: &str = "6c6915ab-775f-4475-859e-befd74d26a23";
const BOX_FUNCTION: &str = "12e94543-6dbc-11d4-b9c8-0060b0ee281b";
const ATTACH_FUNCTION: &str = "12e94554-6dbc-11d4-b9c8-0060b0ee281b";
const ARGUMENTS: i32 = 1;
const RESULT: i32 = 2;
const ATTACH_ARGUMENT: i32 = 1;

fn name_label(label: &TdfLabel, name: &str) {
    TDataStdName::set(label, &ExtendedString::new(name)).unwrap();
}

/// The tree node of a label.
fn node_of(label: &TdfLabel) -> Handle<TDataStdTreeNode> {
    let mut node = Handle::default();
    label
        .find_attribute_as(&TDataStdTreeNode::get_default_tree_id(), &mut node)
        .unwrap();
    node
}

/// A new child of `parent`, its own node appended to `node`, under `name`.
fn named_child(parent: &TdfLabel, node: &Handle<TDataStdTreeNode>, name: &str) -> TdfLabel {
    let child = TdfTagSource::new_child_of(parent).unwrap();
    node.append(&TDataStdTreeNode::set(&child).unwrap())
        .unwrap();
    name_label(&child, name);
    child
}

/// A function with its arguments and result labels on a new child of the object.
fn function_on_object(object: &TdfLabel, guid: &Guid) -> Handle<TFunctionFunction> {
    let label = TdfTagSource::new_child_of(object).unwrap();
    let function = TFunctionFunction::set_with_driver(&label, guid).unwrap();
    let node = TDataStdTreeNode::set(&label).unwrap();
    node_of(object).append(&node).unwrap();
    named_child(&label, &node, "Arguments");
    named_child(&label, &node, "Result");
    function
}

fn function_at(label: &TdfLabel) -> Handle<TFunctionFunction> {
    let mut function = Handle::default();
    let found = label.find_attribute_as(&TFunctionFunction::get_id(), &mut function);
    assert!(
        found.unwrap(),
        "label {}: expected a function",
        entry_of(label)
    );
    function
}

fn result_of(function: &Handle<TFunctionFunction>) -> Result<TdfLabel, Failure> {
    function.label().find_child(RESULT, true)
}

fn named_shape_at(label: &TdfLabel) -> Handle<TNamingNamedShape> {
    let mut named = Handle::default();
    label
        .find_attribute_as(&TNamingNamedShape::get_id(), &mut named)
        .unwrap();
    named
}

/// The named shape an object's reference points at.
fn object_value(object: &TdfLabel) -> Handle<TNamingNamedShape> {
    let mut reference: Handle<TdfReference> = Handle::default();
    let found = object.find_attribute_as(&TdfReference::get_id(), &mut reference);
    assert!(
        found.unwrap(),
        "object {}: expected a result",
        entry_of(object)
    );
    named_shape_at(&reference.get().unwrap().get())
}

/// The word `whatis` starts a shape's type and orientation with, then its flags.
fn what_the_shape_is(shape: &Shape) -> String {
    let kind = shape_type_to_string(shape.shape_type().unwrap());
    let orientation = shape_orientation_to_string(shape.orientation());
    let flags = [
        (shape.free().unwrap(), " Free"),
        (shape.modified().unwrap(), " Modified"),
        (shape.orientable().unwrap(), " Orientable"),
        (shape.closed().unwrap(), " Closed"),
        (shape.infinite().unwrap(), " Infinite"),
        (shape.convex().unwrap(), " Convex"),
    ];
    let set: String = flags
        .iter()
        .filter(|flag| flag.0)
        .map(|flag| flag.1)
        .collect();
    format!("shape {kind} {orientation}{set}")
}

impl CafSession {
    /// `AddObject D`: a geometric object on a new child of the main label.
    /// Example: `session.add_object()`.
    pub fn add_object(&self) -> String {
        println!("AddObject D ");
        let label = self.new_object();
        let entry = entry_of(&label);
        println!("{entry}");
        entry
    }

    fn new_object(&self) -> TdfLabel {
        let main = self.document().main().unwrap();
        let root = TDataStdTreeNode::set(&main).unwrap();
        let label = TdfTagSource::new_child_of(&main).unwrap();
        TDataStdUAttribute::set(&label, &guid_from_text(GEOM_OBJECT)).unwrap();
        root.append(&TDataStdTreeNode::set(&label).unwrap())
            .unwrap();
        label
    }

    /// `AddFunction D object Box`: its function, with the object referring to its result.
    /// Example: `session.add_box_function("0:1:1")`.
    pub fn add_box_function(&self, object: &str) -> String {
        println!("AddFunction D {object} Box ");
        let label = self.find_label(object);
        let function = function_on_object(&label, &guid_from_text(BOX_FUNCTION));
        name_label(&function.label(), "Box_Function");
        TdfReference::set(&label, &result_of(&function).unwrap()).unwrap();
        let entry = entry_of(&function.label());
        println!("{entry}");
        entry
    }

    /// `BoxDX`, `BoxDY` or `BoxDZ D object value`: an argument of the object's only function.
    /// Example: `session.box_size("BoxDX", "0:1:1", 1, 190);`.
    pub fn box_size(&self, command: &str, object: &str, position: i32, value: i32) {
        println!("{command} D {object} {value} ");
        let function = function_at(&self.find_label(object).find_child(1, false).unwrap());
        let arguments = function.label().find_child(ARGUMENTS, true).unwrap();
        let label = arguments.find_child(position, true).unwrap();
        TDataStdReal::set(&label, f64::from(value)).unwrap();
        println!("{}", entry_of(&label));
    }

    /// `InitLogBook D`.
    /// Example: `session.init_log_book();`.
    pub fn init_log_book(&self) {
        println!("InitLogBook D ");
        let main = self.document().main().unwrap();
        TFunctionLogbook::set(&main).unwrap().clear().unwrap();
    }

    /// `AddDriver D Box Attach`. Only the box driver can run in the part translated: nothing
    /// computes the attachment function, so its driver is not built.
    /// Example: `session.add_box_driver();`.
    pub fn add_box_driver(&mut self) {
        println!("AddDriver D Box Attach ");
        let table = Handle::new(TFunctionDriverTable::new());
        let driver: Handle<dyn TFunctionDriver> = Handle::new(CaseBoxDriver::default()).cast();
        table.add_driver(&guid_from_text(BOX_FUNCTION), &driver, 0);
        self.drivers = Some(table);
    }

    /// `ComputeFun D function`: its driver runs on the function's logbook.
    /// Example: `session.compute_fun("0:1:1:1");`.
    pub fn compute_fun(&self, entry: &str) {
        println!("ComputeFun D {entry} ");
        let function = function_at(&self.find_label(entry));
        let table = self.drivers.as_ref().expect("the script added the driver");
        let mut driver: Handle<dyn TFunctionDriver> = Handle::default();
        assert!(table.find_driver(&function.get_driver_guid(), &mut driver, 0));
        driver.init(&function.label());
        let mut log = TFunctionLogbook::set(&function.label()).unwrap();
        assert_eq!(
            driver.execute(&mut log).unwrap(),
            0,
            "the box driver failed"
        );
    }

    /// `GetShape D entry name`: the label's named shape, bound to `name`.
    /// Example: `session.get_shape("0:1:1:1:2", "Box2");`.
    pub fn get_shape(&mut self, entry: &str, name: &str) {
        println!("GetShape D {entry} {name} ");
        let named = named_shape_at(&self.find_label(entry));
        self.bind_shape(name, TNamingTool::get_shape(&named).unwrap());
    }

    /// `explode name kind`, listing the parts it binds, each distinct part once.
    /// Example: `session.explode_listed("Box2", ShapeEnum::Face, "F");`.
    pub fn explode_listed(&mut self, name: &str, kind: ShapeEnum, letter: &str) {
        println!("explode {name} {letter} ");
        let mut seen = MapOfShape::default();
        let shape = self.shape(name).clone();
        seen.add(shape.clone());
        let mut explorer = Explorer::new(&shape, kind, ShapeEnum::Shape).unwrap();
        let mut names = Vec::new();
        while explorer.more() {
            if seen.add(explorer.current().clone()) {
                let part = format!("{name}_{}", names.len() + 1);
                self.bind_shape(&part, explorer.current().clone());
                names.push(part);
            }
            explorer.next().unwrap();
        }
        println!("{}", names.join(" "));
    }

    /// `don first second`, which writes the two checks of its names and shows only them.
    /// Example: `session.show_only("Box2_1_1", "Box2_3_4");`.
    pub fn show_only(&self, first: &str, second: &str) {
        for name in [first, second] {
            self.shape(name);
            println!("isdraw {name} \n1");
        }
        println!("donly {first} {second} \n{first} {second} ");
    }

    /// `AttachShape D name context context 1`: a new object under the context, whose selection
    /// keeps the shape's orientation. Example: `session.attach_keeping_orientation("e", "0:1:1")`.
    pub fn attach_keeping_orientation(&self, name: &str, context: &str) -> String {
        println!("AttachShape D {name} {context} {context} 1 ");
        let container = self.find_label(context);
        let object = self.new_object();
        let node = node_of(&object);
        node.remove().unwrap();
        node_of(&container).append(&node).unwrap();
        name_label(&object, "Auxiliary_Object");
        let function = function_on_object(&object, &guid_from_text(ATTACH_FUNCTION));
        name_label(&function.label(), "ISelection");
        let result = result_of(&function).unwrap();
        TdfReference::set(&object, &result).unwrap();
        result.forget_all_attributes(true).unwrap();
        self.select_in_context(&function, &result, name, &container);
        let entry = entry_of(&object);
        println!("{entry}");
        entry
    }

    fn select_in_context(
        &self,
        function: &Handle<TFunctionFunction>,
        result: &TdfLabel,
        name: &str,
        context: &TdfLabel,
    ) {
        let named = object_value(context);
        let whole = named.get().unwrap().get().unwrap();
        let selector = TNamingSelector::new(result);
        assert!(
            selector
                .select(self.shape(name), &whole, false, true)
                .unwrap()
        );
        let source = function_at(&named.label().father());
        let arguments = function.label().find_child(ARGUMENTS, true).unwrap();
        let argument = arguments.find_child(ATTACH_ARGUMENT, true).unwrap();
        TdfReference::set(&argument, &source.label()).unwrap();
    }

    /// `whatis name`: the type, orientation and flags of the shape bound to `name`.
    /// Example: `session.whatis("E1")`.
    pub fn whatis(&self, name: &str) -> String {
        println!("dtyp {name} ");
        let answer = what_the_shape_is(self.shape(name));
        println!("{answer}");
        answer
    }

    /// `erase`, on an empty view; `session.erase();`.
    pub fn erase(&self) {
        println!("erase ");
    }
}

/// The box computation behind `ComputeFun` of a box function.
#[derive(Default)]
struct CaseBoxDriver {
    transient: StandardTransient,
    base: TFunctionDriverBase,
}
impl_transient!(CaseBoxDriver, "DNamingBoxDriver", dyn TFunctionDriver, transient,
    interfaces: [dyn TFunctionDriver]);

fn real_argument(function: &Handle<TFunctionFunction>, position: i32) -> Result<f64, Failure> {
    let arguments = function.label().find_child(ARGUMENTS, true)?;
    let label = arguments.find_child(position, true)?;
    let mut found: Handle<TDataStdReal> = Handle::default();
    label.find_attribute_as(&TDataStdReal::get_id(), &mut found)?;
    Ok(found.get().map_or(0., |real| real.get()))
}

fn checked_box(
    function: &Handle<TFunctionFunction>,
) -> Result<Option<BRepPrimAPIMakeBox>, Failure> {
    let sizes = [1, 2, 3].map(|position| real_argument(function, position));
    let [dx, dy, dz] = [sizes[0].clone()?, sizes[1].clone()?, sizes[2].clone()?];
    let mut maker = BRepPrimAPIMakeBox::from_sizes(dx, dy, dz)?;
    maker.build(&ProgressRange::default())?;
    if !maker.is_done() {
        function.set_failure(11)?;
        return Ok(None);
    }
    let solid = maker.solid()?.into_shape();
    if !BRepCheckAnalyzer::new(&solid, true, false, false)?.is_valid_shape(&solid)? {
        function.set_failure(12)?;
        return Ok(None);
    }
    Ok(Some(maker))
}

fn load_box_names(result: &TdfLabel, maker: &mut BRepPrimAPIMakeBox) -> Result<(), Failure> {
    TNamingBuilder::new(result)?.generated(&maker.solid()?.into_shape())?;
    let faces = [
        maker.bottom_face()?,
        maker.top_face()?,
        maker.front_face()?,
        maker.right_face()?,
        maker.back_face()?,
        maker.left_face()?,
    ];
    for (index, face) in faces.into_iter().enumerate() {
        let child = result.find_child(index as i32 + 1, true)?;
        TNamingBuilder::new(&child)?.generated(&face.into_shape())?;
    }
    Ok(())
}

fn previous_location(result: &TdfLabel) -> Result<Location, Failure> {
    let mut previous: Handle<TNamingNamedShape> = Handle::default();
    result.find_attribute_as(&TNamingNamedShape::get_id(), &mut previous)?;
    if previous.is_null() || previous.is_empty() {
        return Ok(Location::default());
    }
    Ok(previous.get().unwrap().get()?.location().clone())
}

impl TFunctionDriver for CaseBoxDriver {
    fn driver_base(&self) -> &TFunctionDriverBase {
        &self.base
    }
    fn validate(&self, _log: &Handle<TFunctionLogbook>) -> Result<(), Failure> {
        Ok(())
    }
    fn must_execute(&self, _log: &Handle<TFunctionLogbook>) -> Result<bool, Failure> {
        Ok(true)
    }
    fn execute(&self, log: &mut Handle<TFunctionLogbook>) -> Result<i32, Failure> {
        let function = function_at(&self.label());
        let result = result_of(&function)?;
        let location = previous_location(&result)?;
        let Some(mut maker) = checked_box(&function)? else {
            return Ok(-1);
        };
        load_box_names(&result, &mut maker)?;
        if !location.is_identity() {
            TNaming::displace(&result, &location, true)?;
        }
        log.set_valid(&result, true)?;
        function.set_failure(0)?;
        Ok(0)
    }
}
