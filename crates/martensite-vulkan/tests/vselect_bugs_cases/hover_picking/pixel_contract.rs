use super::*;

pub(super) const PIXELS: [(i32, i32); 3] = [(355, 186), (266, 333), (177, 2)];
pub(super) trait HoverPixelReplies {
    fn rgba_name(&mut self, pixel: (i32, i32)) -> Result<String, Failure>;
}
pub(super) trait HoverOutputLines {
    fn line(&mut self, message: &str);
}
pub(super) struct RenderedHoverPixels<'a>(pub &'a ViewerSession);
pub(super) struct PrintedHoverLines;

impl HoverPixelReplies for RenderedHoverPixels<'_> {
    fn rgba_name(&mut self, (x, y): (i32, i32)) -> Result<String, Failure> {
        let actual = self.0.pixel_rgba_name(x, y)?;
        Ok(actual)
    }
}

impl HoverOutputLines for PrintedHoverLines {
    fn line(&mut self, message: &str) {
        let case_message = message;
        println!("{case_message}");
    }
}

pub(super) fn check_stage(
    replies: &mut impl HoverPixelReplies,
    output: &mut impl HoverOutputLines,
    stage: usize,
) -> Result<(), Failure> {
    let &(index, message) = cycle::STEPS
        .get(stage)
        .ok_or_else(|| program_error(&format!("hover stage={stage}; expected index0..7")))?;
    if !matches_cycle(replies, index)? {
        output.line(message);
    }
    Ok(())
}

fn matches_cycle(replies: &mut impl HoverPixelReplies, index: usize) -> Result<bool, Failure> {
    // Read all three replies before the conjunction; a wrong first color cannot skip I/O.
    let highlighted = replies.rgba_name(PIXELS[index])?;
    let next = replies.rgba_name(PIXELS[(index + 1) % 3])?;
    let last = replies.rgba_name(PIXELS[(index + 2) % 3])?;
    Ok(highlighted == "CYAN 1" && next == "YELLOW 1" && last == "YELLOW 1")
}
