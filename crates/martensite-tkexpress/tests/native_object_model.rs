//! Native ownership and class identity across expression and interpreter boundaries.
use martensite_tkernel::standard::{Handle, RuntimeType, RuntimeTyped, Transient};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkexpress::expr::{
    ExprBinaryExpression, ExprDifference, ExprEqual, ExprGeneralExpression, ExprGeneralRelation,
    ExprHandle, ExprNamedExpression, ExprNamedUnknown, ExprNumericValue, ExprPolyExpression,
    ExprSine, ExprSingleRelation, ExprSum, ExprUnaryExpression,
};
use martensite_tkexpress::expr_intrp::{
    ExprIntrpGenExp, ExprIntrpGenerator, ExprIntrpGeneratorObject, parse::parse,
};

#[test]
fn expression_ownership_and_borrowed_views_keep_the_original_allocation() {
    let concrete = ExprNumericValue::new(2.0);
    let weak = concrete.downgrade();
    let expression = concrete.upcast::<dyn ExprGeneralExpression>();
    let root = expression.upcast::<dyn Transient>();
    assert_eq!(root.object_address(), concrete.object_address());
    assert_eq!(weak.ref_count(), 3);
    let view = expression.view::<ExprNumericValue>();
    assert_eq!(view.object_address(), concrete.object_address());
    assert_eq!(weak.ref_count(), 3);
    assert_eq!(expression.shallow_simplified().unwrap(), concrete);
    assert_eq!(weak.ref_count(), 3);
    drop((concrete, expression));
    assert_eq!(root.into_downcast::<ExprNumericValue>().get_value(), 2.0);
    assert!(weak.upgrade().is_null());
}

fn assert_expression_class(expression: &ExprHandle, class: &str, parent: &str) {
    assert!(expression.is_instance_name(class));
    assert!(expression.is_kind_name(parent));
    assert!(expression.is_kind_name("Expr_GeneralExpression"));
    assert!(std::ptr::eq(
        expression.dynamic_type().unwrap(),
        RuntimeType::registered_by_name(class).unwrap()
    ));
}

#[test]
fn unary_supertrait_upcast_keeps_class_and_ancestry() {
    let value = ExprNumericValue::expression(2.0);
    let unary = ExprSine::new(&value).into_upcast::<dyn ExprUnaryExpression>();
    let expression = unary.into_upcast::<dyn ExprGeneralExpression>();
    assert_expression_class(&expression, "Expr_Sine", "Expr_UnaryExpression");
}

#[test]
fn binary_supertrait_upcast_keeps_class_and_ancestry() {
    let value = ExprNumericValue::expression(2.0);
    let binary = ExprDifference::new(&value, &value).into_upcast::<dyn ExprBinaryExpression>();
    let expression = binary.into_upcast::<dyn ExprGeneralExpression>();
    assert_expression_class(&expression, "Expr_Difference", "Expr_BinaryExpression");
}

#[test]
fn poly_supertrait_upcast_keeps_class_and_ancestry() {
    let value = ExprNumericValue::expression(2.0);
    let poly = ExprSum::new_pair(&value, &value).into_upcast::<dyn ExprPolyExpression>();
    let expression = poly.into_upcast::<dyn ExprGeneralExpression>();
    assert_expression_class(&expression, "Expr_Sum", "Expr_PolyExpression");
}

#[test]
fn named_supertrait_upcast_keeps_class_and_ancestry() {
    let named =
        ExprNamedUnknown::new(&AsciiString::new("x")).into_upcast::<dyn ExprNamedExpression>();
    let expression = named.into_upcast::<dyn ExprGeneralExpression>();
    assert_expression_class(&expression, "Expr_NamedUnknown", "Expr_NamedExpression");
}

#[test]
fn relation_supertrait_upcast_keeps_class_and_ancestry() {
    let value = ExprNumericValue::expression(2.0);
    let relation = ExprEqual::new(&value, &value).into_upcast::<dyn ExprSingleRelation>();
    let relation = relation.into_upcast::<dyn ExprGeneralRelation>();
    assert!(relation.is_instance_name("Expr_Equal"));
    assert!(relation.is_kind_name("Expr_SingleRelation"));
    assert_eq!(relation.string().unwrap(), "2 = 2");
}

#[test]
fn generator_interface_preserves_the_interpreters_allocation_and_type() {
    let interpreter = ExprIntrpGenExp::create();
    let generator = interpreter.upcast::<dyn ExprIntrpGeneratorObject>();
    assert_eq!(generator.object_address(), interpreter.object_address());
    assert!(generator.is_instance_name("ExprIntrp_GenExp"));
    assert!(generator.is_kind(ExprIntrpGenerator::static_type()));
    assert!(std::ptr::eq(
        <dyn ExprIntrpGeneratorObject>::static_type(),
        ExprIntrpGenerator::static_type()
    ));
    assert!(generator.downcast::<ExprIntrpGenerator>().is_null());
    assert_eq!(generator.downcast::<ExprIntrpGenExp>(), interpreter);
}

#[test]
fn generator_interface_lends_the_interpreters_original_definitions() {
    let interpreter = ExprIntrpGenExp::create();
    let weak = interpreter.downgrade();
    let generator = interpreter.upcast::<dyn ExprIntrpGeneratorObject>();
    let result = parse(&generator, &AsciiString::new("x+1"), &mut std::io::sink());
    assert!(result.accepted);
    assert_eq!(generator.get_named_list().len(), 1);
    let name = AsciiString::new("x");
    assert_eq!(generator.get_named(&name), interpreter.get_named(&name));
    assert_eq!(weak.ref_count(), 3);
    drop((result, generator, interpreter));
    assert!(weak.upgrade().is_null());
}

#[test]
fn standalone_generator_and_expression_text_round_trip_keep_their_contracts() {
    let generator =
        Handle::new(ExprIntrpGenerator::new()).into_upcast::<dyn ExprIntrpGeneratorObject>();
    let mut result = parse(
        &generator,
        &AsciiString::new("Sin(x)+2"),
        &mut std::io::sink(),
    );
    assert!(result.accepted);
    let original = result.recept.pop();
    let printed = original.string().unwrap();
    assert_eq!(printed, "Sin(x)+2");
    let mut replay = parse(&generator, &printed, &mut std::io::sink());
    assert!(replay.accepted);
    let replayed: ExprHandle = replay.recept.pop();
    assert_eq!(replayed.string().unwrap(), printed);
    assert!(original.is_identical(&replayed).unwrap());
    assert_eq!(generator.get_named_list().len(), 1);
}
