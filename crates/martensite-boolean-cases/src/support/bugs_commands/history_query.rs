//! `modified`, `generated` and `isdeleted`: what a saved history answers about a shape.
use crate::support::{commands::bound_shape, session::Session};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::History,
    topods::{Compound, Shape},
};
use martensite_tkernel::standard::Handle;

/// `modified name history shape`: binds the compound of what the shape became; `empty` is what the
/// command answers when it became nothing. Returns what it wrote.
/// Example: `modified(s, "v0m", &h, "v0")`.
pub fn modified(s: &mut dyn Session, name: &str, history: &Handle<History>, shape: &str) -> String {
    let initial = bound_shape(s, shape);
    let found = history
        .get()
        .map_or(Ok(Vec::new()), |h| h.modified(&initial));
    published_query(s, name, found.unwrap(), "The shape has not been modified.")
}

/// `generated name history shape`, as [`modified`]; `generated(s, "g1", &h, "f12_1")`.
pub fn generated(
    s: &mut dyn Session,
    name: &str,
    history: &Handle<History>,
    shape: &str,
) -> String {
    let initial = bound_shape(s, shape);
    let found = history
        .get()
        .map_or(Ok(Vec::new()), |h| h.generated(&initial));
    published_query(
        s,
        name,
        found.unwrap(),
        "No shapes were generated from the shape.",
    )
}

/// `isdeleted history shape`: returns what it writes; `isdeleted(s, &h, "v0")`.
pub fn isdeleted(s: &dyn Session, history: &Handle<History>, shape: &str) -> String {
    let initial = bound_shape(s, shape);
    let removed = history
        .get()
        .map_or(Ok(false), |h| h.is_removed(&initial))
        .unwrap();
    let text = if removed { "Deleted." } else { "Not deleted." };
    println!("{text}");
    text.to_owned()
}

fn published_query(s: &mut dyn Session, name: &str, shapes: Vec<Shape>, empty: &str) -> String {
    if shapes.is_empty() {
        println!("{empty}");
        return empty.to_owned();
    }
    if let [only] = shapes.as_slice() {
        s.bind(name, only);
        return String::new();
    }
    let builder = BRepBuilder::default();
    let mut whole = Compound::default();
    builder.make_compound(&mut whole);
    let mut whole = whole.into_shape();
    for shape in &shapes {
        builder.add(&mut whole, shape).unwrap();
    }
    s.bind(name, &whole);
    String::new()
}
