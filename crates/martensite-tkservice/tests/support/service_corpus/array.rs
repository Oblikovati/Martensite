//! Graphic3d_ArrayOfPrimitives and its buffers.
use super::common::{Reader, end, flag, int, real, text};
use super::print::Output;
use super::tried;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::graphic3d::{
    Graphic3dArrayOfPrimitives, Graphic3dBufferMembers, Graphic3dIndexBufferMembers,
    Graphic3dTypeOfPrimitiveArray,
};

/// The primitive type of a corpus value (0..=13).
fn primitive_type(value: i32) -> Graphic3dTypeOfPrimitiveArray {
    use Graphic3dTypeOfPrimitiveArray as T;
    let all = [
        T::Undefined,
        T::Points,
        T::Segments,
        T::Polylines,
        T::Triangles,
        T::Trianglestrips,
        T::Trianglefans,
        T::LinesAdjacency,
        T::LineStripAdjacency,
        T::TrianglesAdjacency,
        T::TriangleStripAdjacency,
        T::Quadrangles,
        T::Quadranglestrips,
        T::Polygons,
    ];
    all[value as usize]
}

/// Prints an index result or the exception type, as `GA_TRY(ga_int(...))`.
fn shown(output: &Output, result: Result<i32, Failure>) {
    match result {
        Ok(value) => int(output, value),
        Err(error) => output.write(format_args!("{} ", error.exception_type())),
    }
}
fn reals<const N: usize>(r: &mut Reader<'_>) -> [f64; N] {
    std::array::from_fn(|_| r.real())
}

/// One scripted operation (the vertex ones).
fn vertex_op(output: &Output, a: &Graphic3dArrayOfPrimitives, op: i32, r: &mut Reader<'_>) {
    match op {
        1 => {
            let [x, y, z] = reals::<3>(r);
            shown(output, a.add_vertex_f64(x, y, z))
        }
        2 => {
            let v = reals::<6>(r);
            shown(
                output,
                a.add_vertex_normal_f64([v[0], v[1], v[2]], [v[3], v[4], v[5]]),
            )
        }
        3 => {
            let v = reals::<5>(r);
            shown(
                output,
                a.add_vertex_texel_f64([v[0], v[1], v[2]], [v[3], v[4]]),
            )
        }
        4 => {
            let v = reals::<8>(r);
            shown(
                output,
                a.add_vertex_normal_texel_f64([v[0], v[1], v[2]], [v[3], v[4], v[5]], [v[6], v[7]]),
            )
        }
        5 => {
            let i = r.integer();
            let [cr, cg, cb] = reals::<3>(r);
            tried(output, a.set_vertex_color_rgb(i, cr, cg, cb))
        }
        6 => {
            let (i, c) = (r.integer(), r.integer());
            tried(output, a.set_vertex_color32(i, c))
        }
        16 => {
            let i = r.integer();
            let [x, y, z] = reals::<3>(r);
            tried(output, a.set_vertex_normal(i, x, y, z))
        }
        17 => {
            let i = r.integer();
            let [x, y] = reals::<2>(r);
            tried(output, a.set_vertex_texel(i, x, y))
        }
        _ => edge_op(output, a, op, r),
    }
}

/// One scripted operation (the edge, bound and validity ones).
fn edge_op(output: &Output, a: &Graphic3dArrayOfPrimitives, op: i32, r: &mut Reader<'_>) {
    match op {
        7 => shown(output, a.add_edge(r.integer())),
        8 => shown(output, a.add_bound(r.integer())),
        9 => {
            let n = r.integer();
            let [cr, cg, cb] = reals::<3>(r);
            shown(output, a.add_bound_rgb(n, cr, cg, cb))
        }
        10 => {
            let (lo, hi) = (r.integer(), r.integer());
            tried(output, a.add_triangle_strip_edges(lo, hi))
        }
        11 => {
            let (lo, hi, c) = (r.integer(), r.integer(), r.flag());
            tried(output, a.add_triangle_fan_edges(lo, hi, c))
        }
        12 => {
            let (lo, hi, c) = (r.integer(), r.integer(), r.flag());
            tried(output, a.add_polyline_edges(lo, hi, c))
        }
        13 => {
            let i = r.integer();
            let [cr, cg, cb] = reals::<3>(r);
            tried(output, a.set_bound_color_rgb(i, cr, cg, cb))
        }
        14 => {
            let v: [i32; 4] = std::array::from_fn(|_| r.integer());
            shown(output, a.add_quad_triangle_edges(v[0], v[1], v[2], v[3]))
        }
        18 => match a.bound_color_rgb(r.integer()) {
            Ok(color) => color.into_iter().for_each(|value| real(output, value)),
            Err(error) => output.write(format_args!("{} ", error.exception_type())),
        },
        _ => flag(output, a.is_valid()),
    }
}

/// The type, counts and flags of the array.
fn print_summary(output: &Output, a: &Graphic3dArrayOfPrimitives) {
    text(output, a.string_type().as_bytes());
    [
        a.item_number(),
        a.edge_number(),
        a.edge_number_allocated(),
        a.bound_number(),
        a.bound_number_allocated(),
    ]
    .into_iter()
    .for_each(|value| int(output, value));
    [
        a.has_vertex_normals(),
        a.has_vertex_colors(),
        a.has_vertex_texels(),
        a.has_bound_colors(),
    ]
    .into_iter()
    .for_each(|value| flag(output, value));
}

/// The vertex buffer layout and bytes, and the index values.
fn print_buffers(output: &Output, a: &Graphic3dArrayOfPrimitives) {
    let attributes = a.attributes();
    flag(output, attributes.is_some());
    if let Some(b) = attributes.get() {
        [
            a.vertex_number(),
            a.vertex_number_allocated(),
            b.stride(),
            b.nb_attributes(),
        ]
        .into_iter()
        .for_each(|value| int(output, value));
        flag(output, b.is_interleaved());
        flag(output, b.is_mutable());
        b.data_bytes()
            .iter()
            .for_each(|byte| int(output, i32::from(*byte)));
    }
    let indices = a.indices();
    flag(output, indices.is_some());
    if let Some(ix) = indices.get() {
        int(output, ix.stride());
        flag(output, ix.is_mutable());
        (0..ix.nb_elements()).for_each(|i| int(output, ix.index(i).unwrap_or(0)));
    }
}

/// The bounds and the vertices read back.
fn print_values(output: &Output, a: &Graphic3dArrayOfPrimitives) {
    // Bound colors are printed by op 18 only: the native allocation leaves unset colors
    // uninitialised.
    (1..=a.bound_number()).for_each(|rank| int(output, a.bound(rank).unwrap_or(0)));
    let count = if a.attributes().is_null() {
        0
    } else {
        a.vertex_number()
    };
    for rank in 1..=count {
        a.vertice_xyz(rank)
            .unwrap_or_default()
            .into_iter()
            .for_each(|value| real(output, value));
        a.vertex_color_rgb(rank)
            .unwrap_or_default()
            .into_iter()
            .for_each(|value| real(output, value));
        a.vertex_normal_xyz(rank)
            .unwrap_or_default()
            .into_iter()
            .for_each(|value| real(output, value));
    }
}

/// `sv_array Kind MaxVertexs MaxBounds MaxEdges Flags NbOps Op...`.
pub fn array_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (kind, max_v, max_b, max_e, flags, nb_ops) = (
        primitive_type(r.integer()),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
        r.integer(),
    );
    let array: Handle<Graphic3dArrayOfPrimitives> =
        Graphic3dArrayOfPrimitives::create_array_with_bounds(kind, max_v, max_b, max_e, flags)
            .unwrap_or_default();
    flag(output, array.is_some());
    if let Some(a) = array.get() {
        text(
            output,
            array
                .dynamic_type()
                .map_or("", |kind| kind.name())
                .as_bytes(),
        );
        (0..nb_ops).for_each(|_| {
            let op = r.integer();
            vertex_op(output, a, op, &mut r)
        });
        array_state(output, a);
    }
    end(output);
}

/// `sv_array_state`: the summary, the buffers and the values read back; viewer corpora
/// print the arrays of their presentation tools with it.
pub fn array_state(output: &Output, a: &Graphic3dArrayOfPrimitives) {
    print_summary(output, a);
    print_buffers(output, a);
    print_values(output, a);
}
