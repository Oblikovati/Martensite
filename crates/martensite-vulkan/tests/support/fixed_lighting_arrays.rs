//! Source arrays expose flat polygon/quad provoking vertices and native strip counters.
use martensite_tkernel::{
    quantity::{Color, ColorSpace},
    standard::Handle,
};
use martensite_tkservice::graphic3d::*;
pub fn fixed_array_override(mode: &str) -> Option<Handle<Graphic3dArrayOfPrimitives>> {
    let kind = fixed_array_kind(mode)?;
    let vertices = Handle::new(
        Graphic3dArrayOfPrimitives::new(
            kind,
            4,
            0,
            4,
            GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL | GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR,
        )
        .unwrap(),
    );
    fill_fixed_array_nodes(&vertices, mode);
    let strip = mode.contains("quad-strip") || mode.contains("triangle-strip");
    for index in if strip { [1, 2, 4, 3] } else { [1, 2, 3, 4] } {
        vertices.add_edge(index).unwrap();
    }
    Some(vertices)
}
fn fixed_array_kind(mode: &str) -> Option<Graphic3dTypeOfPrimitiveArray> {
    use Graphic3dTypeOfPrimitiveArray as Kind;
    match mode {
        "lit-clip-msaa-quad-0" | "lit-clip-msaa-quad-8" => Some(Kind::Quadrangles),
        "lit-facet-quad" => Some(Kind::Quadrangles),
        "lit-facet-quad-strip" => Some(Kind::Quadranglestrips),
        "lit-facet-triangle-strip" => Some(Kind::Trianglestrips),
        "lit-facet-fan" => Some(Kind::Trianglefans),
        "lit-facet-polygon" => Some(Kind::Polygons),
        mode if mode.starts_with("lit-strip-") => Some(Kind::Polylines),
        _ => None,
    }
}
fn fill_fixed_array_nodes(vertices: &Graphic3dArrayOfPrimitives, mode: &str) {
    let points = fixed_quad_points(mode);
    let normals = [
        [0., 0., 1.],
        [0.6, 0., 0.8],
        [0., 0.6, 0.8],
        [0.6, 0.6, 0.529150262],
    ];
    let colors = [
        [0.25, 0.5, 0.75],
        [0.75, 0.25, 0.5],
        [0.5, 0.75, 0.25],
        [0.125, 0.375, 0.625],
    ];
    for index in 0..4 {
        let rank = (index + 1) as i32;
        let [x, y, z] = points[index];
        let [nx, ny, nz] = normals[index];
        vertices
            .set_vertice(rank, x as f32, y as f32, z as f32)
            .unwrap();
        vertices.set_vertex_normal(rank, nx, ny, nz).unwrap();
        let [r, g, b] = colors[index];
        vertices
            .set_vertex_color(rank, Color::new(r, g, b, ColorSpace::Rgb).unwrap())
            .unwrap();
    }
}
fn fixed_quad_points(mode: &str) -> [[f64; 3]; 4] {
    if mode.starts_with("lit-clip-msaa-quad-") {
        return [
            [0.0625, 0.0625, 0.],
            [0.5, 0.0625, 0.],
            [0.5, 0.5, 0.],
            [0.0625, 0.5, 0.],
        ];
    }
    [[-8., -8., 0.], [8., -8., 0.], [8., 8., 0.], [-8., 8., 0.]]
}
