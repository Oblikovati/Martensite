//! Procedural scenes, twin of `occt/geometry.cpp`: the same vertices, edges and aspects, so
//! both renderers receive the same primitives.
use martensite_cmath::{cos, sin};
use martensite_tkernel::quantity::{Color, ColorSpace, NamedColor};
use martensite_tkernel::standard::Handle;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::aspect::{
    AspectInteriorStyle, AspectTypeOfDisplayText, AspectTypeOfLine, AspectTypeOfMarker,
    AspectTypeOfStyleText,
};
use martensite_tkservice::graphic3d::*;
use std::f64::consts::PI;

/// A primitive array of the scene.
pub type Array = Handle<Graphic3dArrayOfPrimitives>;
/// An aspect set of the scene.
pub type Aspect = Handle<dyn Graphic3dAspectsSource>;

fn new_array(kind: Graphic3dTypeOfPrimitiveArray, vertices: i32, edges: i32, flags: i32) -> Array {
    let array = Graphic3dArrayOfPrimitives::new(kind, vertices, 0, edges, flags)
        .unwrap_or_else(|failure| panic!("array of {vertices} vertices: {failure:?}"));
    Handle::new(array)
}

fn rgb(red: f64, green: f64, blue: f64) -> Color {
    Color::new(red, green, blue, ColorSpace::Rgb).expect("colour in 0..=1")
}

/// A latitude-longitude sphere with normals, `2 * lon * lat` triangles; `uv_sphere(1000, 500, 10.)`.
pub fn uv_sphere(lon: i32, lat: i32, radius: f64) -> Array {
    let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL;
    let array = new_array(
        Graphic3dTypeOfPrimitiveArray::Triangles,
        (lon + 1) * (lat + 1),
        6 * lon * lat,
        flags,
    );
    for row in 0..=lat {
        for column in 0..=lon {
            add_sphere_vertex(&array, lon, lat, radius, [row, column]);
        }
    }
    for row in 0..lat {
        for column in 0..lon {
            add_sphere_quad(&array, lon, row, column);
        }
    }
    array
}

fn add_sphere_vertex(array: &Array, lon: i32, lat: i32, radius: f64, [row, column]: [i32; 2]) {
    let polar = PI * f64::from(row) / f64::from(lat);
    let azimuth = 2.0 * PI * f64::from(column) / f64::from(lon);
    let normal = [
        sin(polar) * cos(azimuth),
        sin(polar) * sin(azimuth),
        cos(polar),
    ];
    let point = normal.map(|value| radius * value);
    array
        .add_vertex_normal_f64(point, normal)
        .expect("sphere vertex");
}

fn add_sphere_quad(array: &Array, lon: i32, row: i32, column: i32) {
    let first = row * (lon + 1) + column + 1;
    let (next, below) = (first + 1, first + lon + 1);
    array.add_edges3(first, below, next).expect("sphere edge");
    array
        .add_edges3(next, below, below + 1)
        .expect("sphere edge");
}

/// One axis-aligned box of 12 triangles with face normals; `box_array([0.; 3], 1.)`.
pub fn box_array(center: [f64; 3], half: f64) -> Array {
    let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL;
    let array = new_array(Graphic3dTypeOfPrimitiveArray::Triangles, 24, 36, flags);
    for axis in 0..3 {
        for sign in [-1.0, 1.0] {
            add_box_face(&array, center, half, axis, sign);
        }
    }
    array
}

fn add_box_face(array: &Array, center: [f64; 3], half: f64, axis: usize, sign: f64) {
    let unit = |index: usize, scale: f64| {
        let mut vector = [0.0; 3];
        vector[index % 3] = scale;
        vector
    };
    let (normal, u, v) = (unit(axis, sign), unit(axis + 1, 1.0), unit(axis + 2, 1.0));
    let mut corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    if sign < 0.0 {
        corners.reverse();
    }
    let indices = corners.map(|(a, b)| {
        let point = std::array::from_fn(|i| center[i] + half * (normal[i] + a * u[i] + b * v[i]));
        array
            .add_vertex_normal_f64(point, normal)
            .expect("box vertex")
    });
    array
        .add_edges3(indices[0], indices[1], indices[2])
        .expect("box edge");
    array
        .add_edges3(indices[0], indices[2], indices[3])
        .expect("box edge");
}

/// The cell of item `index` on a 1000-wide grid centred on the origin.
fn grid_cell(index: i32) -> (f64, f64) {
    (
        f64::from(index % 1000) - 500.0,
        f64::from(index / 1000) - 500.0,
    )
}

/// `count` vertical segments on the grid; `segment_field(1_000_000)`.
pub fn segment_field(count: i32) -> Array {
    let flags = GRAPHIC3D_ARRAY_FLAGS_NONE;
    let array = new_array(Graphic3dTypeOfPrimitiveArray::Segments, 2 * count, 0, flags);
    for index in 0..count {
        let (x, y) = grid_cell(index);
        let top = 1.0 + f64::from(index % 7) * 0.5;
        array.add_vertex_f64(x, y, 0.0).expect("segment vertex");
        array.add_vertex_f64(x, y, top).expect("segment vertex");
    }
    array
}

/// `count` coloured points of a wave on the grid; `point_field(1_000_000)`.
pub fn point_field(count: i32) -> Array {
    let flags = GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR;
    let array = new_array(Graphic3dTypeOfPrimitiveArray::Points, count, 0, flags);
    for index in 0..count {
        let (x, y) = grid_cell(index);
        let z = 20.0 * sin(x / 50.0) * cos(y / 50.0);
        let level = (z + 20.0) / 40.0;
        let color = rgb(level, 0.5, 1.0 - level);
        array
            .add_vertex_pnt_color(&Point3::new(x, y, z), color)
            .expect("point vertex");
    }
    array
}

/// A solid, lit, plastic aspect; `lit_aspect([0.8, 0.5, 0.2], 0.)` (transparency 0..1).
pub fn lit_aspect(color: [f64; 3], transparency: f32) -> Aspect {
    let mut material = Graphic3dMaterialAspect::new(Graphic3dNameOfMaterial::Plastified);
    material
        .set_transparency(transparency)
        .expect("transparency");
    let [red, green, blue] = color;
    let aspect = Graphic3dAspectFillArea3d::new(
        AspectInteriorStyle::Solid,
        rgb(red, green, blue),
        Color::named(NamedColor::BLACK),
        AspectTypeOfLine::Solid,
        1.0,
        material.clone(),
        material,
    )
    .expect("fill aspect");
    Handle::new(aspect).cast()
}

/// A white solid line of width 1.
pub fn line_aspect() -> Aspect {
    let white = Color::named(NamedColor::WHITE);
    let aspect = Graphic3dAspectLine3d::new(white, AspectTypeOfLine::Solid, 1.0).expect("line");
    Handle::new(aspect).cast()
}

/// A point marker of scale 1, coloured by its vertices.
pub fn point_aspect() -> Aspect {
    let white = Color::named(NamedColor::WHITE);
    let aspect =
        Graphic3dAspectMarker3d::new(AspectTypeOfMarker::Point, white, 1.0).expect("marker");
    Handle::new(aspect).cast()
}

/// White 16-pixel DejaVu Sans text.
pub fn text_aspect() -> Aspect {
    let aspect = Graphic3dAspectText3d::new(
        Color::named(NamedColor::WHITE),
        Some("DejaVu Sans"),
        1.,
        0.,
        AspectTypeOfStyleText::Normal,
        AspectTypeOfDisplayText::Normal,
    );
    Handle::new(aspect).cast()
}

/// Label `index` of a 40-column grid of labels, `L000` to `L999`.
pub fn text_label(index: i32) -> Handle<Graphic3dText> {
    let text = Handle::new(Graphic3dText::new(16.));
    text.set_text_str(&format!("L{index:03}"));
    let (x, y) = (
        f64::from(index % 40) * 10.0 - 200.0,
        f64::from(index / 40) * 10.0 - 125.0,
    );
    text.set_position(&Point3::new(x, y, 0.0));
    text
}
