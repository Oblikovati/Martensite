use super::*;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct GeometryDumpCounts {
    pub curves2d: i32,
    pub curves3d: i32,
    pub surfaces: i32,
}

impl GeometryDumpCounts {
    pub fn all_removed(&self) -> bool {
        self.curves2d == 0 && self.curves3d == 0 && self.surfaces == 0
    }
}

fn geometry_section_count(words: &[&str], label: &str) -> Result<i32, BoxError> {
    let matches: Vec<_> = words
        .windows(4)
        .filter(|word| word[0] == "Dump" && word[1] == "of" && word[3] == label)
        .collect();
    if matches.len() != 1 {
        return Err(format!(
            "dump label={label:?}, count={}; expected one geometry section",
            matches.len()
        )
        .into());
    }
    matches[0][2].parse::<i32>().map_err(|cause| {
        format!(
            "dump label={label:?}, token={:?}, cause={cause}; expected integer geometry count",
            matches[0][2]
        )
        .into()
    })
}

pub(super) fn geometry_dump_counts(text: &str) -> Result<GeometryDumpCounts, BoxError> {
    let words: Vec<_> = text.split_whitespace().collect();
    Ok(GeometryDumpCounts {
        curves2d: geometry_section_count(&words, "Curve2ds")?,
        curves3d: geometry_section_count(&words, "Curves")?,
        surfaces: geometry_section_count(&words, "surfaces")?,
    })
}
