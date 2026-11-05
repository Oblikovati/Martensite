//! Canonical source placements serve both historical entries and ordinary workers.
use super::tool_cuts::{FIRST_AXIS, SECOND_AXIS, ToolCut};

pub(crate) const TOOL_CUT_TODOS: &[&str] = &[
    "Error :  is WRONG because number of WIRE entities in shape \"result\" is 10",
    "Error :  is WRONG because number of FACE entities in shape \"result\" is 10",
];

#[derive(Clone, Copy, Debug)]
pub(crate) enum SourceCut {
    Tilted,
    RaisedTodo,
    Tightened,
}

/// Select literal placement/settings without altering the general cut pipeline; `tool_cut_spec(SourceCut::Tilted)`.
pub(crate) fn tool_cut_spec(source: SourceCut) -> ToolCut {
    match source {
        SourceCut::Tilted => tilted(),
        SourceCut::RaisedTodo => raised(false, "bug29807_b4a.png"),
        SourceCut::Tightened => raised(true, "bug29807_b4b.png"),
    }
}

fn tilted() -> ToolCut {
    ToolCut {
        rotations: &[(FIRST_AXIS, 5.)],
        lift: None,
        remove_location: true,
        tighten_vertex: false,
        area: 4014.54,
        volume: 9463.99,
        image: "bug29807_b2.png",
    }
}

fn raised(tighten_vertex: bool, image: &'static str) -> ToolCut {
    ToolCut {
        rotations: &[(FIRST_AXIS, -5.), (SECOND_AXIS, 20.)],
        lift: Some(0.3),
        remove_location: false,
        tighten_vertex,
        area: 4012.74,
        volume: 9465.07,
        image,
    }
}

#[cfg(test)]
pub(crate) fn source_tool(
    source: SourceCut,
) -> Result<martensite_tkbrep::topods::Shape, Box<dyn std::error::Error>> {
    super::tool_cuts::placed_tool(&tool_cut_spec(source))
}
