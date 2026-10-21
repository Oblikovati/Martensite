//! The topology commands the cases call: `explode`, `mkcurve`, `mksurface`, `pcurve`
//! and `mk2dcurve`, on the boundary representation case helpers.
use crate::support::brep_commands;
use crate::support::script::{Reply, Script, ScriptStep};
use crate::support::session::CurveColor;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::{
    precision,
    standard::{Failure, Handle},
};
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dTrimmedCurve};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};

/// `explode name`: `name_1`, `name_2`, ... for the direct sub-shapes.
pub fn explode(d: &mut Script, name: &str) -> ScriptStep<usize> {
    explode_parts(d, name, None)
}

/// `explode name type` (`e`, `f`, ...): each sub-shape of the type once.
pub fn explode_type(d: &mut Script, name: &str, kind: ShapeEnum) -> ScriptStep<usize> {
    explode_parts(d, name, Some(kind))
}

fn explode_parts(d: &mut Script, name: &str, kind: Option<ShapeEnum>) -> ScriptStep<usize> {
    let letter = kind.map(|kind| match kind {
        ShapeEnum::Edge => "e",
        ShapeEnum::Face => "f",
        other => panic!("explode type {other:?}: expected an edge or face exploration"),
    });
    let mut args = vec!["explode", name];
    args.extend(letter);
    let mut count = 0;
    d.command(&args, |d| {
        let shape = d.shape(name);
        let parts = match kind {
            None => brep_commands::explode(&shape),
            Some(kind) => brep_commands::explode_type(&shape, kind),
        };
        let mut text = String::new();
        for (index, part) in parts.into_iter().enumerate() {
            let part_name = format!("{name}_{}", index + 1);
            text.push_str(&format!("{part_name} "));
            d.set(&part_name, part);
            count += 1;
        }
        Ok(Reply::ok(text))
    })?;
    Ok(count)
}

/// `mkcurve name edge`: the edge's 3D curve trimmed to its range.
pub fn mkcurve(d: &mut Script, name: &str, edge: &str) -> ScriptStep {
    d.command(&["mkcurve", name, edge], |d| {
        let curve = brep_commands::mkcurve(&d.shape(edge));
        d.set(name, curve);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `mksurface name face`: the face's surface.
pub fn mksurface(d: &mut Script, name: &str, face: &str) -> ScriptStep {
    d.command(&["mksurface", name, face], |d| {
        let surface = brep_commands::mksurface(&d.shape(face));
        d.set(name, surface);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// The curve color of an edge orientation.
fn orientation_color(orientation: Orientation) -> CurveColor {
    match orientation {
        Orientation::Forward => CurveColor::Rouge,
        Orientation::Reversed => CurveColor::Bleu,
        Orientation::External => CurveColor::Rose,
        Orientation::Internal => CurveColor::Orange,
    }
}

/// `pcurve face`: the orientation legend, then `face_1`, `face_2`, ... for the p-curves
/// of the forward face's edges, each in the color of its edge's orientation.
pub fn pcurve_face(d: &mut Script, face: &str) -> ScriptStep {
    d.command(&["pcurve", face], |d| {
        d.cout("\nrouge  FORWARD\nbleu   REVERSED\nrose   EXTERNAL\norange INTERNAL\n");
        let mut forward = d.shape(face);
        forward.set_orientation(Orientation::Forward);
        let edges = brep_commands::explode_all(&forward, ShapeEnum::Edge);
        let curves = brep_commands::pcurve(&forward);
        for (index, (curve, edge)) in curves.into_iter().zip(edges).enumerate() {
            let color = orientation_color(edge.orientation());
            d.set_colored(&format!("{face}_{}", index + 1), curve, color);
        }
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// `pcurve name edge face`: the p-curve of the edge on the face, in the color of the
/// edge's orientation.
pub fn pcurve(d: &mut Script, name: &str, edge: &str, face: &str) -> ScriptStep {
    d.command(&["pcurve", name, edge, face], |d| {
        let edge_shape = d.shape(edge);
        let color = orientation_color(edge_shape.orientation());
        let edge = topods::edge(&edge_shape)?;
        let face = topods::face(&d.shape(face))?;
        let found = BRepTool::curve_on_face(&edge, &face)?;
        let curve = pcurve_variable(found.pcurve, found.first, found.last)?;
        d.set_colored(name, curve, color);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}

/// The p-curve trimmed to the edge range unless the non-periodic curve is shorter.
fn pcurve_variable(
    c: Handle<dyn Geom2dCurve>,
    f: f64,
    l: f64,
) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let curve = c.get().expect("pcurve: the edge has no pcurve on the face");
    let (mut fr, mut lr, mut periodic) = (
        curve.first_parameter(),
        curve.last_parameter(),
        curve.is_periodic(),
    );
    if let Some(trimmed) = c.cast::<Geom2dTrimmedCurve>().get() {
        let basis = trimmed.basis_curve();
        let basis = basis.get().expect("pcurve basis");
        (fr, lr, periodic) = (
            basis.first_parameter(),
            basis.last_parameter(),
            basis.is_periodic(),
        );
    }
    let tolerance = precision::p_confusion();
    if !periodic && (fr - f > tolerance || l - lr > tolerance) {
        return Ok(c);
    }
    Ok(Handle::new(Geom2dTrimmedCurve::new(&c, f, l, true, true)?).cast())
}

/// `mk2dcurve name edge index`: the edge's `index`-th p-curve, trimmed to its range.
pub fn mk2dcurve(d: &mut Script, name: &str, edge: &str, index: i32) -> ScriptStep {
    let index_text = index.to_string();
    d.command(&["mk2dcurve", name, edge, &index_text], |d| {
        let edge_shape = topods::edge(&d.shape(edge))?;
        let found = BRepTool::curve_on_surface_by_index(&edge_shape, index)?;
        if found.pcurve.is_null() {
            return Ok(Reply::error(format!("{edge} has no 2d curve\n")));
        }
        let trimmed = Geom2dTrimmedCurve::new(&found.pcurve, found.first, found.last, true, true)?;
        let trimmed: Handle<dyn Geom2dCurve> = Handle::new(trimmed).cast();
        d.set(name, trimmed);
        Ok(Reply::ok(""))
    })?;
    Ok(())
}
