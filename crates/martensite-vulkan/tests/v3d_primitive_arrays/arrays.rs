use super::specification::ArraySpec;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED, GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR,
    GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL, Graphic3dArrayOfPrimitives,
};
pub(super) fn build(
    spec: &ArraySpec,
    non_interleaved: bool,
) -> Result<Handle<Graphic3dArrayOfPrimitives>, Failure> {
    let flags = layout_flags(non_interleaved);
    let array = Graphic3dArrayOfPrimitives::create_array_with_bounds(
        spec.kind,
        spec.vertices.len() as i32,
        spec.bounds.len() as i32,
        spec.edges.len() as i32,
        flags,
    )?;
    fill(&array, spec, non_interleaved)?;
    Ok(array)
}
fn layout_flags(non_interleaved: bool) -> i32 {
    GRAPHIC3D_ARRAY_FLAGS_VERTEX_NORMAL
        | GRAPHIC3D_ARRAY_FLAGS_VERTEX_COLOR
        | if non_interleaved {
            GRAPHIC3D_ARRAY_FLAGS_ATTRIBS_DEINTERLEAVED
        } else {
            0
        }
}
fn fill(
    array: &Graphic3dArrayOfPrimitives,
    spec: &ArraySpec,
    non_interleaved: bool,
) -> Result<(), Failure> {
    fill_vertices(array, spec, non_interleaved)?;
    for &count in spec.bounds {
        array.add_bound(count)?;
    }
    for &edge in spec.edges {
        array.add_edge(edge)?;
    }
    Ok(())
}
fn fill_vertices(
    array: &Graphic3dArrayOfPrimitives,
    spec: &ArraySpec,
    non_interleaved: bool,
) -> Result<(), Failure> {
    for (offset, &source) in spec.vertices.iter().enumerate() {
        let [x, y, z] = generated_point([spec.x, 0., 0.], source);
        let rank = insert_vertex(array, offset as i32 + 1, [x, y, z], non_interleaved)?;
        array.set_vertex_normal(rank, 0., 0., -1.)?;
        let [r, g, b] = spec.color;
        array.set_vertex_color_rgb(rank, r, g, b)?;
    }
    Ok(())
}
fn insert_vertex(
    array: &Graphic3dArrayOfPrimitives,
    rank: i32,
    point: [f64; 3],
    non_interleaved: bool,
) -> Result<i32, Failure> {
    let [x, y, z] = point;
    if !non_interleaved {
        return array.add_vertex_f64(x, y, z);
    }
    array.set_vertice(rank, x as f32, y as f32, z as f32)?;
    Ok(rank)
}
pub(super) fn generated_point(origin: [f64; 3], number: usize) -> [f64; 3] {
    let offsets = [
        [-5., 5.],
        [0., 5.],
        [0., 0.],
        [5., 0.],
        [-5., -5.],
        [0., -5.],
    ];
    let [dx, dy] = offsets[number - 1];
    [origin[0] + dx, origin[1] + dy, origin[2]]
}
