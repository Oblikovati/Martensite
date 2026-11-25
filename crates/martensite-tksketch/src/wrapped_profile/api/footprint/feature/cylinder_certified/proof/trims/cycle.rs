//! Recognize a complete directed trim cycle by its native vertex incidences.
use super::*;
use martensite_tkbrep::topexp::{first_vertex, last_vertex};

pub(super) fn trace_wire(
    attempt: Attempt,
    wire: &Shape,
    identify: impl Fn(&Shape) -> Checked<Option<WrappedHistoryKey>>,
) -> Checked<Option<Path>> {
    let shapes = assembly::children(attempt, wire)?;
    let mut path = Vec::with_capacity(shapes.len());
    for shape in &shapes {
        if let Some(key) = identify(shape)? {
            let edge = attempt.native(
                VALIDATION,
                "directed native cap edge occurrence",
                topods::edge(shape),
            )?;
            let direction = match edge.orientation() {
                Orientation::Forward => 0,
                Orientation::Reversed => 1,
                _ => {
                    return Err(missing(
                        attempt,
                        VALIDATION,
                        "ordinary directed cap wire incidence",
                    ));
                }
            };
            path.push((
                (key, direction),
                [first_vertex(&edge, true), last_vertex(&edge, true)],
            ));
        }
    }
    if path.is_empty() {
        return Ok(None);
    }
    require(
        attempt,
        VALIDATION,
        path.len() == shapes.len(),
        "cap loops preserve exact stock or tool ancestry without mixed owners",
    )?;
    canonical_cycle(attempt, &path).map(Some)
}

fn canonical_cycle(
    attempt: Attempt,
    path: &[((WrappedHistoryKey, u8), [topods::Vertex; 2])],
) -> Checked<Path> {
    let start = path
        .iter()
        .min_by_key(|item| item.0)
        .ok_or_else(|| missing(attempt, VALIDATION, "nonempty directed source ring"))?;
    let mut keys = Vec::with_capacity(path.len());
    let mut current = start;
    let mut visited = BTreeSet::new();
    for _ in 0..path.len() {
        require(
            attempt,
            VALIDATION,
            visited.insert(current.0),
            "one directed use of each retained loop edge",
        )?;
        keys.push(current.0);
        let mut next = path.iter().filter(|edge| edge.1[0].is_same(&current.1[1]));
        current = next
            .next()
            .ok_or_else(|| missing(attempt, VALIDATION, "closed directed vertex incidence"))?;
        require(
            attempt,
            VALIDATION,
            next.next().is_none(),
            "one next directed edge at the loop vertex",
        )?;
    }
    require(
        attempt,
        VALIDATION,
        start.0 == current.0,
        "exactly closed decoded source ring",
    )?;
    Ok(keys)
}
