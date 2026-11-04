//! Preserve xyzList's A=Polygon3D+8, B=5, i=1..N indexing of the actual fixed dump format.
use super::*;

#[derive(Debug)]
pub(super) struct DumpNode {
    pub raw: [String; 3],
    pub coordinates: [f64; 3],
}

fn polygon_dump_word<'a>(words: &[&'a str], index: usize) -> Result<&'a str, BoxError> {
    words.get(index).copied().ok_or_else(|| {
        format!(
            "dump index={index}, word_count={}; expected an actual polygon coordinate token",
            words.len()
        )
        .into()
    })
}

pub(super) fn case_xyz_list_offset(base: usize, rank: usize) -> Result<usize, BoxError> {
    base.checked_add(8)
        .and_then(|a| rank.checked_mul(5).and_then(|step| a.checked_add(step)))
        .ok_or_else(|| {
            format!("dump base={base}, rank={rank}; expected representable A+5*i index").into()
        })
}

fn polygon_dump_node(words: &[&str], index: usize) -> Result<DumpNode, BoxError> {
    let raw = [
        polygon_dump_word(words, index)?.to_owned(),
        polygon_dump_word(words, index + 1)?.to_owned(),
        polygon_dump_word(words, index + 2)?.to_owned(),
    ];
    let mut coordinates = [0.; 3];
    for (axis, text) in raw.iter().enumerate() {
        coordinates[axis] = text.parse::<f64>().map_err(|cause| {
            format!("dump token={text:?}, cause={cause}; expected finite coordinate")
        })?;
        if !coordinates[axis].is_finite() {
            return Err(format!("dump token={text:?}; expected finite coordinate").into());
        }
    }
    Ok(DumpNode { raw, coordinates })
}

pub(super) fn parse_polygon_dump_nodes(dump: &str) -> Result<Vec<DumpNode>, BoxError> {
    let words: Vec<_> = dump.split_whitespace().collect();
    let base = words
        .iter()
        .position(|word| *word == "Polygon3D")
        .ok_or("dump missing Polygon3D; expected actual polygon section")?;
    let count = polygon_dump_node_count(&words, base)?;
    require_final_polygon_coordinate(&words, base, count)?;
    (1..=count)
        .map(|rank| polygon_dump_node(&words, case_xyz_list_offset(base, rank)?))
        .collect()
}

fn polygon_dump_node_count(words: &[&str], base: usize) -> Result<usize, BoxError> {
    let token = polygon_dump_word(words, base + 2)?;
    token.parse::<usize>().map_err(|cause| {
        format!("polygon count={token:?}, cause={cause}; expected nonnegative integer NbNodes")
            .into()
    })
}

fn require_final_polygon_coordinate(
    words: &[&str],
    base: usize,
    count: usize,
) -> Result<(), BoxError> {
    if count == 0 {
        return Ok(());
    }
    let last = case_xyz_list_offset(base, count)?
        .checked_add(2)
        .ok_or("polygon final index overflow; expected XYZ")?;
    polygon_dump_word(words, last)?;
    Ok(())
}
