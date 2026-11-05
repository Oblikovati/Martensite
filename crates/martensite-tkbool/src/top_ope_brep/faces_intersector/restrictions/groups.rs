//! Group adjacent walking pieces using their endpoint vertex and restriction identity.
use super::{Side, build::build_group};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkgeomalgo::int_patch::{
    IntPatchIType, IntPatchLine, IntPatchPoint, IntPatchPointLine, IntPatchWLine,
};

pub(in super::super) fn convert_groups(
    lines: &mut Vec<Handle<dyn IntPatchLine>>,
    sides: [Side<'_>; 2],
) -> Result<(), Failure> {
    let mut group = Vec::new();
    let mut restrictions = Vec::new();
    for side in sides {
        let mut index = 0;
        while index < lines.len() {
            let source = lines[index].clone();
            if source.arc_type() == IntPatchIType::Walking {
                step(&source, side, &mut group, &mut restrictions, lines)?;
            }
            index += 1;
        }
    }
    lines.extend(restrictions);
    Ok(())
}

fn step(
    source: &Handle<dyn IntPatchLine>,
    side: Side<'_>,
    group: &mut Vec<Handle<dyn IntPatchLine>>,
    restrictions: &mut Vec<Handle<dyn IntPatchLine>>,
    lines: &mut Vec<Handle<dyn IntPatchLine>>,
) -> Result<(), Failure> {
    let walking = source.cast::<IntPatchWLine>();
    let first = walking.vertex(1)?;
    let last = walking.vertex(walking.nb_vertex())?;
    let appended = append_segment(source, &walking, &first, side, group)?;
    if !appended || !side.is_vertex(&last) {
        return Ok(());
    }
    if let Some(restriction) = build_group(group, side)? {
        restrictions.push(restriction.cast());
        remove_sources(lines, group);
    }
    group.clear();
    Ok(())
}

fn append_segment(
    source: &Handle<dyn IntPatchLine>,
    line: &IntPatchWLine,
    first: &IntPatchPoint,
    side: Side<'_>,
    group: &mut Vec<Handle<dyn IntPatchLine>>,
) -> Result<bool, Failure> {
    if !side.is_vertex(first) {
        let Some(previous) = group.last() else {
            return Ok(false);
        };
        let previous = previous.cast::<IntPatchWLine>();
        let last = previous.vertex(previous.nb_vertex())?;
        let tolerance = if last.tolerance() > first.tolerance() {
            last.tolerance()
        } else {
            first.tolerance()
        };
        if !last.value().is_equal(&first.value(), tolerance) {
            group.clear();
            return Ok(false);
        }
    }
    if !side.appendable(line)? {
        return Ok(false);
    }
    group.push(source.clone());
    Ok(true)
}

fn remove_sources(lines: &mut Vec<Handle<dyn IntPatchLine>>, group: &[Handle<dyn IntPatchLine>]) {
    for source in group {
        if let Some(index) = lines.iter().position(|line| line == source) {
            lines.remove(index);
        }
    }
}
