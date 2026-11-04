//! Checked request parsing remains independent of actor lookup and GPU execution.
use super::*;
pub(super) fn request_header(
    text: &str,
) -> Result<
    (
        &str,
        i32,
        martensite_vulkan::VertexOutputFormat,
        &str,
        group_selection::GroupSelection,
    ),
    Box<dyn Error>,
> {
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() != 4 || lines[0].is_empty() {
        return Err(shape_error(text).into());
    }
    let mode = lines[1].parse::<i32>()?;
    let (format, selection) = group_selection::GroupSelection::parse(lines[2])?;
    Ok((lines[0], mode, format, lines[3], selection))
}
pub(super) fn bounded_request(
    indices: &str,
    format: martensite_vulkan::VertexOutputFormat,
) -> Result<VertexOutputRequest, Box<dyn Error>> {
    // This is a request marker only; source identity is resolved below from actual handles/UIDs.
    let token = NonZeroU64::MIN;
    let indices = indices
        .split(',')
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()?;
    let request = match format {
        martensite_vulkan::VertexOutputFormat::TrianglesClipNormalId32 => {
            VertexOutputRequest::new(indices, 65535 * 32)?
        }
        martensite_vulkan::VertexOutputFormat::PointsClipId20 => {
            VertexOutputRequest::points(indices, 65535 * 20)?
        }
        martensite_vulkan::VertexOutputFormat::TextClipUvId28 => {
            VertexOutputRequest::text(indices, 65535 * 28)?
        }
    };
    Ok(request.for_source_token(token))
}
fn shape_error(actual: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "request={actual:?}; expected four lines: exact actor name, presentation mode, literal auto (triangle32) or points-clip-id20 (plain point20), comma-separated actual EBO indices"
        ),
    )
}
