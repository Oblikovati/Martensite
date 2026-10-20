//! The shape commands (`nbshapes`, `compound`) and the `checknbshapes` check procedure
//! the hidden-line cases run, each logged the way the harness logs it.
use super::restore::Restored;
use super::script::{Reply, Script, ScriptError, ScriptStep};
use super::session::Session;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::brep_tools::BRepToolsShapeSet;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{Compound, Shape};
use martensite_tkbrep::toptools::{IndexedMapOfShape, ShapeSet};
use martensite_tkernel::{standard::Failure, tcollection::AsciiString};
use martensite_tkg3d::top_abs::ShapeEnum;

/// The `checknbshapes` entities, in its order.
pub const ENTITIES: [&str; 9] = [
    "VERTEX",
    "EDGE",
    "WIRE",
    "FACE",
    "SHELL",
    "SOLID",
    "COMPSOLID",
    "COMPOUND",
    "SHAPE",
];

/// `compound name1 ... result`: the non-null shapes of the names in one compound, set as
/// `result`; `compound(&mut draw, &mut view, &["vl", "v1l"], "result")?`.
pub fn compound(
    draw: &mut Script,
    view: &mut dyn Session,
    names: &[&str],
    result: &str,
) -> ScriptStep {
    let words: Vec<&str> = std::iter::once("compound")
        .chain(names.iter().copied())
        .chain([result])
        .collect();
    draw.command(&words, |draw| {
        let builder = BRepBuilder::default();
        let mut compound = Compound::default();
        builder.make_compound(&mut compound);
        for name in names {
            let shape = draw.shape(name);
            if !shape.is_null() {
                builder.add(&mut compound, &shape)?;
            }
        }
        let shape: Shape = compound.into();
        draw.set_shape(result, &shape);
        view.set(result, &Restored::Shape(shape));
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `nbshapes name [-t]`: `Number of shapes in name` then the counts, from
/// BRepTools_ShapeSet::DumpExtent, or with `-t` from `DBRep` DumpExtent (TopExp::MapShapes
/// per type, located shapes distinct); nothing for a null shape.
pub fn nbshapes(draw: &mut Script, name: &str, total: bool) -> ScriptStep<String> {
    let words: &[&str] = if total { &[name, "-t"] } else { &[name] };
    let words: Vec<&str> = std::iter::once("nbshapes")
        .chain(words.iter().copied())
        .collect();
    draw.command(&words, |draw| {
        let shape = draw.shape(name);
        if shape.is_null() {
            return Ok(Reply::ok(""));
        }
        let extent = if total {
            total_extent(&shape)?
        } else {
            shape_set_extent(&shape)?
        };
        Ok(Reply::ok(format!("Number of shapes in {name}\n{extent}")))
    })
}

fn shape_set_extent(shape: &Shape) -> Result<String, Failure> {
    let mut set = BRepToolsShapeSet::new(true, false);
    set.add(shape)?;
    let mut text = AsciiString::default();
    set.dump_extent_string(&mut text)?;
    Ok(String::from_utf8_lossy(text.as_bytes()).into_owned())
}

/// `DBRep` `DumpExtent`: VERTEX up to COMPOUND, then their sum as SHAPE.
fn total_extent(shape: &Shape) -> Result<String, Failure> {
    const LABELS: [(&str, ShapeEnum); 8] = [
        (" VERTEX    : ", ShapeEnum::Vertex),
        (" EDGE      : ", ShapeEnum::Edge),
        (" WIRE      : ", ShapeEnum::Wire),
        (" FACE      : ", ShapeEnum::Face),
        (" SHELL     : ", ShapeEnum::Shell),
        (" SOLID     : ", ShapeEnum::Solid),
        (" COMPSOLID : ", ShapeEnum::CompSolid),
        (" COMPOUND  : ", ShapeEnum::Compound),
    ];
    let mut text = String::new();
    let mut sum = 0;
    for (label, kind) in LABELS {
        let mut map = IndexedMapOfShape::default();
        topexp::map_shapes_of_type(shape, kind, &mut map)?;
        text.push_str(&format!("{label}{}\n", map.extent()));
        sum += map.extent();
    }
    text.push_str(&format!(" SHAPE     : {sum}\n"));
    Ok(text)
}

/// The count after `ENTITY +: +` in an `nbshapes` text.
fn count_of(text: &str, entity: &str) -> Option<i64> {
    let mut rest = text;
    while let Some(at) = rest.find(entity) {
        rest = &rest[at + entity.len()..];
        if let Some(number) = count_after_entity(rest) {
            return Some(number);
        }
    }
    None
}

/// ` +: +(number)` at the start of `text` (at least one blank on each side of `:`).
fn count_after_entity(text: &str) -> Option<i64> {
    let after = text.trim_start_matches(' ');
    let value = after
        .strip_prefix(':')
        .filter(|_| after.len() < text.len())?;
    let trimmed = value.trim_start_matches(' ');
    if trimmed.len() == value.len() {
        return None;
    }
    let digits: String = trimmed
        .chars()
        .take_while(|c| c.is_ascii_digit() || "-+.eE".contains(*c))
        .collect();
    digits.parse::<f64>().ok().map(|number| number as i64)
}

/// The options of one `checknbshapes` call.
#[derive(Clone, Copy)]
pub struct NbShapesCheck<'a> {
    /// The `-vertex` ... `-shape` counts, in [`ENTITIES`] order.
    pub expected: [Option<i64>; 9],
    /// `-t`: count with `nbshapes -t`.
    pub total: bool,
    /// `-m message`.
    pub message: &'a str,
    /// `-ref text`.
    pub reference: Option<&'a str>,
    /// The procedure's `args` as script prints the list (its first log line).
    pub args_text: &'a str,
}

/// `checknbshapes name [-vertex n ...] [-t] [-m message] [-ref text]`: for each entity
/// with an expected count or a count in the reference, an `Error` line when they differ
/// from `nbshapes`, else an `OK` line; `checknbshapes(&mut draw, "vl", &check)?`.
pub fn checknbshapes(draw: &mut Script, name: &str, check: &NbShapesCheck<'_>) -> ScriptStep {
    let NbShapesCheck {
        expected,
        total,
        message,
        reference,
        args_text,
    } = *check;
    draw.puts(&format!("checknbshapes {name} {args_text}"));
    let nb_info = nbshapes(draw, name, total)?;
    for (index, entity) in ENTITIES.iter().enumerate() {
        let Some(found) = count_of(&nb_info, entity) else {
            return Err(ScriptError(format!(
                "Error : command \"nbshapes {name}\" gives an empty result"
            )));
        };
        let mut compare = vec![found];
        compare.extend(expected[index]);
        compare.extend(reference.and_then(|text| count_of(text, entity)));
        if compare.len() == 1 {
            continue;
        }
        compare.sort_unstable();
        compare.dedup();
        if compare.len() != 1 {
            draw.puts(&format!(
                "Error : {message} is WRONG because number of {entity} entities in shape \"{name}\" is {found}"
            ));
        } else {
            draw.puts(&format!(
                "OK : {message} is GOOD because number of {entity} entities is equal to number of expected {entity} entities"
            ));
        }
    }
    Ok(())
}

#[test]
fn count_of_reads_the_entity_counts() {
    let text = "Number of shapes in r\n VERTEX    : 109\n EDGE      : 55\n SHAPE     : 165\n";
    assert_eq!(count_of(text, "VERTEX"), Some(109));
    assert_eq!(count_of(text, "EDGE"), Some(55));
    assert_eq!(count_of(text, "SHAPE"), Some(165));
    assert_eq!(count_of(text, "WIRE"), None);
}
