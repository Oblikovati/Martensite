//! Complete checksection response; callers retain and publish the actual odd-incidence handles.
use martensite_tkbrep::topods::Shape;

fn reference_response(actual: usize, reference: Option<i32>) -> String {
    let Some(expected) = reference.filter(|value| *value >= 0) else {
        return String::new();
    };
    if actual == expected as usize {
        return "Section is OK\n".to_owned();
    }
    format!("Error: {expected} vertices are expected but {actual} are found.\n")
}

/// Render every original field and publication name; `checksection_response(&vertices, None)`.
pub fn checksection_response(vertices: &[Shape], reference: Option<i32>) -> String {
    let mut response = format!(" nb alone Vertices : {}\n", vertices.len());
    response.push_str(&reference_response(vertices.len(), reference));
    for index in 1..=vertices.len() {
        response.push_str(&format!("alone_{index} "));
    }
    response.push('\n');
    response
}
