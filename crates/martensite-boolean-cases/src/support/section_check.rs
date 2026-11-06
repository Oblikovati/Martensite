//! `checksection name [-r n]`: the closure of a section, by the vertices met an odd number of
//! times; each is bound as `alone_N`.
use super::session::Session;
use crate::boolean_section_endpoints::odd_incidence_vertices;
use martensite_tkbrep::topods::Shape;

/// A name that holds no shape has no vertices: the command then reports none.
fn alone_vertices(s: &dyn Session, name: &str) -> Vec<Shape> {
    s.shape(name).map_or_else(Vec::new, odd_incidence_vertices)
}

fn publish(s: &mut dyn Session, alone: &[Shape]) {
    let mut names = Vec::with_capacity(alone.len());
    for (index, vertex) in alone.iter().enumerate() {
        names.push(format!("alone_{}", index + 1));
        s.bind(&names[index], vertex);
    }
    println!("{}", names.join(" "));
}

/// `checksection name`; `checksection(s, "result")`.
pub fn checksection(s: &mut dyn Session, name: &str) {
    println!("checksection {name}");
    let alone = alone_vertices(s, name);
    println!(" nb alone Vertices : {}", alone.len());
    publish(s, &alone);
}

/// `checksection name -r expected`, and how many it found; `checksection_expecting(s, "result", 0)`.
pub fn checksection_expecting(s: &mut dyn Session, name: &str, expected: usize) -> usize {
    println!("checksection {name} -r {expected}");
    let alone = alone_vertices(s, name);
    println!(" nb alone Vertices : {}", alone.len());
    if alone.len() == expected {
        println!("Section is OK");
    } else {
        let found = alone.len();
        println!("Error: {expected} vertices are expected but {found} are found.");
    }
    publish(s, &alone);
    alone.len()
}
