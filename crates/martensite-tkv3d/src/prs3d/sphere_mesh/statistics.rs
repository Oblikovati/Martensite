/// The statistics printed to the standard output.
pub(crate) fn print_info(points: i32, triangles: i32) {
    let (normals_mb, colors_mb, poly_connect_size, total) = storage_sizes(points, triangles);
    println!("NumberOfPoints:    {points}");
    println!("NumberOfTriangles: {triangles}");
    println!(
        "Amount of memory required for PolyTriangulation without Normals: {} Mb",
        total - normals_mb
    );
    println!("Amount of memory for colors: {colors_mb} Mb");
    println!("Amount of memory for PolyConnect: {poly_connect_size} Mb");
    println!("Amount of graphic card memory required: {total} Mb");
}

fn storage_sizes(points: i32, triangles: i32) -> (i32, i32, i32, i32) {
    let float_size = std::mem::size_of::<f32>() as i32;
    let points_size = points * 3 * float_size;
    let normals_size = points * 3 * float_size;
    let colors_size = points * 3 * float_size;
    let triangles_size = triangles * 3 * std::mem::size_of::<i32>() as i32;
    let poly_connect_size = (points * 4 + triangles * 6 * 4) >> 20;
    let total = (points_size + normals_size + colors_size + triangles_size) >> 20;
    let (normals_mb, colors_mb) = (normals_size >> 20, colors_size >> 20);
    (normals_mb, colors_mb, poly_connect_size, total)
}
