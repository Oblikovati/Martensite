//! tests/bugs/moddata_1: the TKMesh-owned cases.
use crate::support::draw::{Draw, Tcl};
use crate::support::memory::{checktrend, meminfo_heap};
use crate::support::qa::occ20627;

/// bugs/moddata_1/bug20627: memory leaks in BRepMesh. Nine rounds of ten rectangle-face
/// meshings; after each round the heap usage joins the list, whose mean step must stay
/// within one byte of zero (plus two standard deviations), else `Memory leak detected`.
pub fn bug20627(draw: &mut Draw) -> Tcl {
    // pload QAcommands
    draw.puts("========");
    draw.puts("OCC20627");
    draw.puts("========");
    draw.puts("");
    let mut listmem: Vec<f64> = Vec::new();
    for _ in 1..10 {
        occ20627(draw, "10")?;
        listmem.push(meminfo_heap(draw)?);
        checktrend(draw, &listmem, "0", 1., "Memory leak detected")?;
    }
    Ok(())
}
