//! Native #F formatting, including its historical spelling and disabled pair-only modes.
use super::groups::SideGroups;
use std::fmt::Write;
const LABELS: [&str; 9] = [
    "Shapes are not suppotrted by BOP: ",
    "Self-Intersections              : ",
    "Check for SI has been aborted   : ",
    "Too small edges                 : ",
    "Bad faces                       : ",
    "Too close vertices              : ",
    "Too close edges                 : ",
    "Shapes with Continuity C0       : ",
    "Invalid Curve on Surface        : ",
];

pub(super) fn summaries(sides: &[SideGroups; 2], single: bool, output: &mut String) {
    for (side, counts) in sides.iter().enumerate() {
        if side == 1 {
            output.push('\n');
        }
        let total = usize::from(counts.cases[0] != 0) + counts.cases[1..].iter().sum::<usize>();
        let name = ["FIRST", "SECOND"][side];
        writeln!(output, "Faulties for {name}  shape found : {total}").unwrap();
        if total == 0 {
            continue;
        }
        output.push_str("---------------------------------\n");
        for group in 0..9 {
            group_line(counts, group, single, output);
        }
    }
}

fn group_line(counts: &SideGroups, group: usize, single: bool, output: &mut String) {
    let cases = counts.cases[group];
    let status = if cases != 0 {
        "YES"
    } else if single && (group == 5 || group == 6) {
        "DISABLED"
    } else {
        "NO"
    };
    write!(output, "{}{status}", LABELS[group]).unwrap();
    if cases != 0 && group != 0 && group != 2 {
        write!(
            output,
            "  Cases({cases})  Total shapes({})",
            counts.totals[group]
        )
        .unwrap();
    }
    output.push('\n');
}
