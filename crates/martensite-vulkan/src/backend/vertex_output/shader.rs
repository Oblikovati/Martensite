macro_rules! variant {
    ($name:literal) => {
        include_bytes!(concat!("../../../shaders/", $name)).as_slice()
    };
}
pub(in crate::backend) fn selected(stock: &[u8]) -> Option<&'static [u8]> {
    let pairs = [
        (
            variant!("lighting.vert.spv"),
            variant!("diagnostic.vertex-output.lighting.vert.spv"),
        ),
        (
            variant!("numeric-max-v2.lighting.vert.spv"),
            variant!("diagnostic.vertex-output.numeric-max-v2.lighting.vert.spv"),
        ),
        (
            variant!("factorized-v2.lighting.vert.spv"),
            variant!("diagnostic.vertex-output.factorized-v2.lighting.vert.spv"),
        ),
        (
            variant!("factorized-v2.numeric-max-v2.lighting.vert.spv"),
            variant!("diagnostic.vertex-output.factorized-v2.numeric-max-v2.lighting.vert.spv"),
        ),
        (
            variant!("unsnapped-v2.lighting.vert.spv"),
            variant!("diagnostic.vertex-output.unsnapped-v2.lighting.vert.spv"),
        ),
        (
            variant!("unsnapped-v2.numeric-max-v2.lighting.vert.spv"),
            variant!("diagnostic.vertex-output.unsnapped-v2.numeric-max-v2.lighting.vert.spv"),
        ),
    ];
    pairs
        .into_iter()
        .find_map(|(original, observed)| (stock == original).then_some(observed))
}
pub(in crate::backend) fn selected_fragment(stock: &[u8]) -> Option<&'static [u8]> {
    let pairs = fragment_pairs();
    pairs
        .into_iter()
        .find_map(|(original, observed)| (stock == original).then_some(observed))
}
fn fragment_pairs() -> [(&'static [u8], &'static [u8]); 8] {
    [
        (
            variant!("lighting.frag.spv"),
            variant!("diagnostic.vertex-output.lighting.frag.spv"),
        ),
        (
            variant!("lighting.coordinates-v2.frag.spv"),
            variant!("diagnostic.vertex-output.lighting.coordinates-v2.frag.spv"),
        ),
        (
            variant!("numeric-max-v2.lighting.frag.spv"),
            variant!("diagnostic.vertex-output.numeric-max-v2.lighting.frag.spv"),
        ),
        (
            variant!("numeric-max-v2.lighting.coordinates-v2.frag.spv"),
            variant!("diagnostic.vertex-output.numeric-max-v2.lighting.coordinates-v2.frag.spv"),
        ),
        (
            variant!("unsnapped-v3.lighting.frag.spv"),
            variant!("diagnostic.vertex-output.unsnapped-v3.lighting.frag.spv"),
        ),
        (
            variant!("unsnapped-v3.lighting.coordinates-v2.frag.spv"),
            variant!("diagnostic.vertex-output.unsnapped-v3.lighting.coordinates-v2.frag.spv"),
        ),
        (
            variant!("unsnapped-v3.numeric-max-v2.lighting.frag.spv"),
            variant!("diagnostic.vertex-output.unsnapped-v3.numeric-max-v2.lighting.frag.spv"),
        ),
        (
            variant!("unsnapped-v3.numeric-max-v2.lighting.coordinates-v2.frag.spv"),
            variant!(
                "diagnostic.vertex-output.unsnapped-v3.numeric-max-v2.lighting.coordinates-v2.frag.spv"
            ),
        ),
    ]
}

pub(in crate::backend) fn selected_for_format(
    stock: &[u8],
    format: crate::VertexOutputFormat,
) -> Option<&'static [u8]> {
    match format {
        crate::VertexOutputFormat::TrianglesClipNormalId32 => selected(stock),
        crate::VertexOutputFormat::PointsClipId20 => (stock == variant!("point.vert.spv"))
            .then_some(variant!("diagnostic.vertex-output.point.vert.spv")),
        crate::VertexOutputFormat::TextClipUvId28 => (stock == variant!("text.vert.spv"))
            .then_some(variant!("diagnostic.vertex-output.text.vert.spv")),
    }
}
pub(in crate::backend) fn fragment_for_format(
    stock: &[u8],
    format: crate::VertexOutputFormat,
) -> Option<&'static [u8]> {
    if format == crate::VertexOutputFormat::TrianglesClipNormalId32 {
        return selected_fragment(stock);
    }
    if format == crate::VertexOutputFormat::TextClipUvId28 {
        return (stock == variant!("text.frag.spv"))
            .then_some(variant!("diagnostic.vertex-output.text.frag.spv"));
    }
    [
        (
            variant!("point.frag.spv"),
            variant!("diagnostic.vertex-output.point.frag.spv"),
        ),
        (
            variant!("point.coordinates-v2.frag.spv"),
            variant!("diagnostic.vertex-output.point.coordinates-v2.frag.spv"),
        ),
    ]
    .into_iter()
    .find_map(|(original, observed)| (stock == original).then_some(observed))
}
