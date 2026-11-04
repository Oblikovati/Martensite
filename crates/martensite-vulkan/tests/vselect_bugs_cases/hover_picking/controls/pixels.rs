use super::super::pixel_contract::{HoverOutputLines, HoverPixelReplies, PIXELS, check_stage};
use super::*;

struct NamedFakePixelReplies {
    names: [String; 3],
    reads: Vec<(i32, i32)>,
    fail_read: Option<usize>,
}
#[derive(Default)]
struct NamedFakeHoverOutput(Vec<String>);

impl NamedFakePixelReplies {
    fn new(names: [&str; 3]) -> Self {
        let names = names.map(str::to_owned);
        Self {
            names,
            reads: Vec::new(),
            fail_read: None,
        }
    }
}
impl HoverPixelReplies for NamedFakePixelReplies {
    fn rgba_name(&mut self, pixel: (i32, i32)) -> Result<String, Failure> {
        self.reads.push(pixel);
        if self.fail_read == Some(self.reads.len()) {
            return Err(program_error(&format!(
                "pixel={pixel:?}; expected readable actual reply"
            )));
        }
        let index = PIXELS.iter().position(|p| *p == pixel).unwrap();
        Ok(self.names[index].clone())
    }
}
impl HoverOutputLines for NamedFakeHoverOutput {
    fn line(&mut self, message: &str) {
        let owned = message.to_owned();
        self.0.push(owned);
    }
}

#[test]
fn source_pixel_checks_keep_all_reads_exact_alpha_independent_messages_and_errors() {
    assert_successful_reads();
    for names in [
        ["CYAN", "YELLOW 1", "YELLOW 1"],
        ["CYAN 0", "YELLOW 1", "YELLOW 1"],
        ["CYAN 1", "CYAN 1", "YELLOW 1"],
    ] {
        let mut replies = NamedFakePixelReplies::new(names);
        let mut output = NamedFakeHoverOutput::default();
        check_stage(&mut replies, &mut output, 0).unwrap();
        assert_eq!(replies.reads, PIXELS);
        assert_eq!(output.0, ["ERROR: incorrect highlighting of box b0"]);
    }
    assert_all_messages();
    assert_rgb_negative_literal();
    assert_read_error_and_invalid_stage();
}

fn assert_successful_reads() {
    for (stage, index) in [(0, 0), (1, 1), (2, 2), (3, 0), (4, 2), (5, 1), (6, 0)] {
        let mut names = ["YELLOW 1"; 3];
        names[index] = "CYAN 1";
        let mut replies = NamedFakePixelReplies::new(names);
        let mut output = NamedFakeHoverOutput::default();
        check_stage(&mut replies, &mut output, stage).unwrap();
        assert!(output.0.is_empty());
        assert_eq!(
            replies.reads,
            [
                PIXELS[index],
                PIXELS[(index + 1) % 3],
                PIXELS[(index + 2) % 3]
            ]
        );
    }
}

fn assert_all_messages() {
    let mut output = NamedFakeHoverOutput::default();
    let mut replies = NamedFakePixelReplies::new(["BLACK 1"; 3]);
    for stage in 0..7 {
        check_stage(&mut replies, &mut output, stage).unwrap();
    }
    assert_eq!(replies.reads.len(), 21);
    assert_eq!(
        output.0,
        [
            "ERROR: incorrect highlighting of box b0",
            "ERROR: incorrect highlighting of box b1 after vselnext call",
            "ERROR: incorrect highlighting of box b2 after vselnext call",
            "ERROR: incorrect highlighting of box b0 after vselnext call",
            "ERROR: incorrect highlighting of box b2 after vselprev call",
            "ERROR: incorrect highlighting of box b1 after vselprev call",
            "ERROR: incorrect highlighting of box b0 after vselprev call",
        ]
    );
}

fn assert_read_error_and_invalid_stage() {
    let mut replies = NamedFakePixelReplies::new(["BLACK 1"; 3]);
    replies.fail_read = Some(2);
    let mut output = NamedFakeHoverOutput::default();
    assert!(check_stage(&mut replies, &mut output, 0).is_err());
    assert_eq!(replies.reads, PIXELS[..2]);
    assert!(output.0.is_empty());
    let error = check_stage(&mut replies, &mut output, 7).unwrap_err();
    assert!(error.message().contains("stage=7"));
    assert_eq!(replies.reads.len(), 2);
}

fn assert_rgb_negative_literal() {
    for color in ["CYAN", "CYAN2", "CYAN 1", "BLACK", "cyan"] {
        let mut output = NamedFakeHoverOutput::default();
        super::super::occlusion::check_color(color, &mut output);
        let expected = if color == "CYAN" {
            vec!["ERROR: back box is misdetected"]
        } else {
            vec![]
        };
        assert_eq!(output.0, expected);
    }
}
