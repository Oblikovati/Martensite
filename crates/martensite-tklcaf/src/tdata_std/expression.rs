//! Expression strings and variables shared by the expression and relation attributes.
use super::TDataStdVariable;
use super::support::{cast_attribute, find_or_add};
use crate::guid_from_text;
use crate::json::dump_attribute_json_base;
use crate::live;
use crate::tdf::{TdfAttribute, TdfAttributeBase, TdfLabel, TdfRelocationTable};
use martensite_tkcdf::json_dump::{class_begin, field_dumped, field_string};
use martensite_tkernel::{
    impl_transient, impl_transient_interface_target,
    standard::{
        Failure, Guid, Handle, HandleUpcast, RuntimeType, RuntimeTyped, SharedState,
        dump::DumpStream,
    },
    tcollection::ExtendedString,
};
use std::io;
use std::sync::{Arc, RwLockWriteGuard};

/// `TDataStd_Expression::GetID()`.
const EXPRESSION_ID: Guid = guid_from_text("ce24146a-8e57-11d1-8953-080009dc4425");
/// `TDataStd_Relation::GetID()`.
const RELATION_ID: Guid = guid_from_text("ce24146b-8e57-11d1-8953-080009dc4425");

/// `TDataStd_Expression`; `TDataStdExpression::set(&label)`.
///
/// ```
/// use martensite_tklcaf::{tdata_std::{TDataStdExpression, TDataStdExpressionSource}, tdf::TdfData};
/// use martensite_tkernel::tcollection::ExtendedString;
/// let data = TdfData::new();
/// let expression = TDataStdExpression::set(&data.root()).unwrap();
/// expression.set_expression(&ExtendedString::new("a+b")).unwrap();
/// assert!(expression.name().is_equal(ExtendedString::new("a+b")));
/// ```
#[derive(Default)]
pub struct TDataStdExpression {
    base: TDataStdExpressionBase,
}
impl_transient!(TDataStdExpression, "TDataStd_Expression", dyn TdfAttribute,
    interfaces: [dyn TdfAttribute, dyn TDataStdExpressionSource]);

/// The fields of an expression, for attributes implementing `TDataStdExpressionSource`.
/// Example: `TDataStdExpressionBase::default()`.
#[derive(Default)]
pub struct TDataStdExpressionBase {
    attribute: TdfAttributeBase,
    expression: SharedState<ExtendedString>,
    variables: SharedState<Vec<Handle<dyn TdfAttribute>>>,
}

/// The expression operations inherited by `TDataStd_Relation`.
/// Example: `relation.upcast::<dyn TDataStdExpressionSource>().get_expression()`.
pub trait TDataStdExpressionSource: TdfAttribute {
    /// Borrow the expression state for the concrete attribute.
    /// Example: `let fields = relation.expression_base();`.
    fn expression_base(&self) -> &TDataStdExpressionBase;

    /// `Name`: the expression string. Example: `expression.name()`.
    fn name(&self) -> ExtendedString {
        self.get_expression()
    }

    /// `SetExpression(E)`: backs up the actual attribute when its expression changes.
    /// Example: `expression.set_expression(&ExtendedString::new("a+b"))?`.
    fn set_expression(&self, expression: &ExtendedString) -> Result<(), Failure> {
        super::support::set_backed_up(self, &self.expression_base().expression, expression)
    }

    /// `GetExpression`. Example: `expression.get_expression()`.
    fn get_expression(&self) -> ExtendedString {
        self.expression_base().expression.read().clone()
    }

    /// `GetVariables`: lends the variables through a write guard.
    /// Example: `expression.get_variables().len()`.
    fn get_variables(&self) -> RwLockWriteGuard<'_, Vec<Handle<dyn TdfAttribute>>> {
        self.expression_base().variables.write()
    }
}

impl RuntimeTyped for dyn TDataStdExpressionSource {
    const TYPE_NAME: &'static str = TDataStdExpression::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        TDataStdExpression::static_type()
    }
}
impl_transient_interface_target!(dyn TDataStdExpressionSource);

impl HandleUpcast<dyn TdfAttribute> for dyn TDataStdExpressionSource {
    fn upcast_arc(owner: Arc<Self>) -> Arc<dyn TdfAttribute> {
        owner
    }
}

impl TDataStdExpressionSource for TDataStdExpression {
    fn expression_base(&self) -> &TDataStdExpressionBase {
        &self.base
    }
}

impl TDataStdExpression {
    /// `GetID`.
    pub fn get_id() -> Guid {
        EXPRESSION_ID
    }
    /// `TDataStd_Expression()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Set(label)`: the expression of the label, added when absent.
    pub fn set(label: &TdfLabel) -> Result<Handle<Self>, Failure> {
        find_or_add(label, &EXPRESSION_ID, false, Self::new)
    }
}

impl TDataStdExpressionBase {
    /// The shared body of `Restore`: the other expression's string, and its variables cast
    /// to `TDataStd_Variable` (an attribute of another class becomes null, as natively).
    fn restore_expression(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        let other =
            cast_attribute::<dyn TDataStdExpressionSource>(with, "TDataStd_Expression::Restore")?;
        let other = live(&other, "TDataStd_Expression::Restore")?;
        let expression = other.get_expression();
        let variables = other
            .expression_base()
            .variables
            .read()
            .iter()
            .map(|v| v.downcast::<TDataStdVariable>().into_upcast())
            .collect();
        *self.expression.write() = expression;
        *self.variables.write() = variables;
        Ok(())
    }
    /// The shared body of `Paste`: `Into` takes the string and gains the relocation of each
    /// variable (null when the table has none); `Standard_NullObject` when `Into` is not an
    /// expression.
    fn paste_expression(
        &self,
        into: &Handle<dyn TdfAttribute>,
        table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        let target = into.cast::<dyn TDataStdExpressionSource>();
        let target = live(&target, "TDataStd_Expression::Paste")?;
        let expression = self.expression.read().clone();
        target.set_expression(&expression)?;
        let variables = self.variables.read().clone();
        for variable in variables {
            let mut relocated = Handle::default();
            live(table, "TDataStd_Expression::Paste")?.has_attribute_relocation(
                &variable.downcast::<TDataStdVariable>().into_upcast(),
                &mut relocated,
            );
            target.get_variables().push(relocated);
        }
        Ok(())
    }
    /// The variable dumps of `DumpJson`, each as `"<field>": {...}`.
    fn dump_variables(&self, stream: &mut DumpStream, depth: i32, field: &str) -> io::Result<()> {
        let variables = self.variables.read().clone();
        for variable in &variables {
            let child = variable
                .get()
                .map(|v| move |s: &mut DumpStream, d: i32| v.dump_json(s, d));
            field_dumped(stream, depth, field, child.as_ref().map(|c| c as _))?;
        }
        Ok(())
    }
}

impl TdfAttribute for TDataStdExpression {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base.attribute
    }
    fn id(&self) -> Guid {
        EXPRESSION_ID
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        self.base.restore_expression(with)
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).into_upcast()
    }
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        self.base.paste_expression(into, table)
    }
    /// `Dump`: `"Expression"`.
    fn dump(&self, stream: &mut dyn io::Write) -> io::Result<()> {
        write!(stream, "Expression")
    }
    /// `DumpJson`: the string and the dump of every variable.
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "TDataStd_Expression")?;
        dump_attribute_json_base(self, stream, depth)?;
        field_string(stream, "myExpression", &self.get_expression().to_utf8())?;
        self.base.dump_variables(stream, depth, "anAttribute.get()")
    }
}

/// `TDataStd_Relation`: an expression that states a relation; `TDataStdRelation::set(&label)`.
///
/// ```
/// use martensite_tklcaf::{tdata_std::{TDataStdExpressionSource, TDataStdRelation}, tdf::TdfData};
/// use martensite_tkernel::tcollection::ExtendedString;
/// let data = TdfData::new();
/// let relation = TDataStdRelation::set(&data.root()).unwrap();
/// relation.set_relation(&ExtendedString::new("a=b")).unwrap();
/// let expression = relation.upcast::<dyn TDataStdExpressionSource>();
/// assert_eq!(expression.get_expression(), ExtendedString::new("a=b"));
/// ```
#[derive(Default)]
pub struct TDataStdRelation {
    base: TDataStdExpressionBase,
}
impl_transient!(TDataStdRelation, "TDataStd_Relation", dyn TDataStdExpressionSource,
    interfaces: [dyn TdfAttribute, dyn TDataStdExpressionSource]);

impl TDataStdExpressionSource for TDataStdRelation {
    fn expression_base(&self) -> &TDataStdExpressionBase {
        &self.base
    }
}

impl TDataStdRelation {
    /// `GetID`.
    pub fn get_id() -> Guid {
        RELATION_ID
    }
    /// `TDataStd_Relation()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// `Set(label)`: the relation of the label, added when absent.
    pub fn set(label: &TdfLabel) -> Result<Handle<Self>, Failure> {
        find_or_add(label, &RELATION_ID, false, Self::new)
    }
    /// Borrow the expression operations of the relation.
    /// Example: `relation.expression().get_expression()`.
    pub fn expression(&self) -> &dyn TDataStdExpressionSource {
        self
    }
    /// `SetRelation(R)`: `SetExpression`.
    pub fn set_relation(&self, relation: &ExtendedString) -> Result<(), Failure> {
        self.set_expression(relation)
    }
    /// `GetRelation`: `GetExpression`.
    pub fn get_relation(&self) -> ExtendedString {
        self.get_expression()
    }
}

impl TdfAttribute for TDataStdRelation {
    fn attribute_base(&self) -> &TdfAttributeBase {
        &self.base.attribute
    }
    fn id(&self) -> Guid {
        RELATION_ID
    }
    fn restore(&self, with: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        self.base.restore_expression(with)
    }
    /// `NewEmpty`: the registered derived-attribute creator, a new relation.
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        Handle::new(Self::new()).into_upcast()
    }
    fn paste(
        &self,
        into: &Handle<dyn TdfAttribute>,
        table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        self.base.paste_expression(into, table)
    }
    /// `Dump`: `"Relation"`.
    fn dump(&self, stream: &mut dyn io::Write) -> io::Result<()> {
        write!(stream, "Relation")
    }
    /// `DumpJson`: the relation string and the dump of every variable.
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "TDataStd_Relation")?;
        dump_attribute_json_base(self, stream, depth)?;
        field_string(stream, "GetRelation()", &self.get_relation().to_utf8())?;
        self.base.dump_variables(stream, depth, "aVariable.get()")
    }
}
