//! `lprops|sprops|vprops shape [-skip]` without an epsilon, with
//! the matrix of inertia it prints (iostream default floats, `%g` 6 digits, `setw(15)`),
//! and the `parseprops` reading of drop small solids — end.
use super::log;
use super::props::Prop;
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::storage::format_general;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn w15(value: f64) -> String {
    format!("{:>15}", format_general(value, 6))
}

/// The command's printed text (mass, centre of gravity and matrix of inertia), logged.
pub fn props_text(kind: Prop, shape: &Shape, skip_shared: bool) -> String {
    let mut g = GProps::default();
    match kind {
        Prop::Length => BRepGProp::linear_properties(shape, &mut g, skip_shared, false),
        Prop::Area => BRepGProp::surface_properties(shape, &mut g, skip_shared, false),
        Prop::Volume => BRepGProp::volume_properties(shape, &mut g, false, skip_shared, false),
    }
    .unwrap_or_else(|failure| panic!("props: {failure:?}"));
    let p = g.centre_of_mass();
    let m = g.matrix_of_inertia();
    let i = |r, c| w15(m.value(r, c).unwrap());
    let text = format!(
        "\n\nMass : {}\n\nCenter of gravity : \nX = {}\nY = {}\nZ = {}\n\nMatrix of Inertia : \n{} {} {}\n{} {} {}\n{} {} {}\n",
        w15(g.mass()),
        w15(p.x()),
        w15(p.y()),
        w15(p.z()),
        i(1, 1),
        i(1, 2),
        i(1, 3),
        i(2, 1),
        i(2, 2),
        i(2, 3),
        i(3, 1),
        i(3, 2),
        i(3, 3)
    );
    log::result(&text);
    text
}

/// `lprops|sprops|vprops shape 1.0e-4 -skip` as `checkprops ... -skip` runs it: the printed
/// mass (`%g`), logged with its `Mass :` line.
fn skip_mass(kind: Prop, shape: &Shape) -> f64 {
    let mut g = GProps::default();
    let eps = 1.0e-4;
    match kind {
        Prop::Length => BRepGProp::linear_properties(shape, &mut g, true, false),
        Prop::Area => BRepGProp::surface_properties_eps(shape, &mut g, eps, true).map(|_| ()),
        Prop::Volume => {
            BRepGProp::volume_properties_eps(shape, &mut g, eps, false, true).map(|_| ())
        }
    }
    .unwrap_or_else(|failure| panic!("props -skip: {failure:?}"));
    let printed = format_general(g.mass(), 6);
    log::result(&format!("\n\nMass : {printed:>15}\n\n"));
    printed.parse().unwrap()
}

/// `checkprops shape -equal other -skip`: the printed `lprops -skip` lengths must match.
pub fn checkprops_equal_skip(shape: &Shape, name: &str, other: &Shape, other_name: &str) {
    log::puts(&format!("checkprops {name} -equal {other_name} -skip"));
    let m = skip_mass(Prop::Length, shape);
    if skip_mass(Prop::Length, other) != m {
        log::puts(&format!(
            "Error: Shape {other_name} is not equal to shape {name}"
        ));
    }
}

/// `checkprops shape -l|-s|-v expected ... -skip`: each printed mass nonzero and within
/// 1% of its expected value.
pub fn checkprops_skip(shape: &Shape, expected: &[(Prop, f64)]) {
    log::puts("checkprops -skip");
    for (kind, mass) in expected {
        let m = skip_mass(*kind, shape);
        let prop = match kind {
            Prop::Length => "length",
            Prop::Area => "area",
            Prop::Volume => "volume",
        };
        if m == 0. {
            log::puts(&format!(
                "Error : The command is not valid. The {prop} is 0."
            ));
        }
        if (*mass != 0. && ((mass - m) / mass).abs() > 1e-2) || (*mass == 0. && m != 0.) {
            log::puts(&format!(
                "Error : The {prop} of result shape is {m}, expected {mass}"
            ));
        }
    }
}

/// `parseprops text`: the centre of gravity and the nine matrix values as printed (the
/// `regexp -inline` list without its full match), or empty when the text does not match.
pub fn parseprops(text: &str) -> Vec<f64> {
    let Some(at) = text.find("Center of gravity :") else {
        return Vec::new();
    };
    let words: Vec<&str> = text[at..].split_whitespace().collect();
    let mut values = Vec::new();
    for label in ["X", "Y", "Z"] {
        let Some(index) = words.iter().position(|w| *w == label) else {
            return Vec::new();
        };
        values.push(words[index + 2].parse().unwrap());
    }
    let Some(matrix) = words.iter().position(|w| *w == "Inertia") else {
        return Vec::new();
    };
    for word in &words[matrix + 2..matrix + 11] {
        values.push(word.parse().unwrap());
    }
    values
}

/// Compares parsed values with a 1e-8 tolerance (the scenario loop
/// starts at index 1 of the `regexp -inline` list, past its full match, so it compares
/// every captured value).
pub fn cmpprops(kind: Prop, a: &Shape, b: &Shape) -> bool {
    let aa = parseprops(&props_text(kind, a, false));
    let bb = parseprops(&props_text(kind, b, false));
    aa.iter().enumerate().all(|(i, value)| {
        let other = bb
            .get(i)
            .unwrap_or_else(|| panic!("expr: lindex {i} of b is empty"));
        (value - other).abs() <= 1e-8
    })
}
