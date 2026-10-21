//! The original ordered union of initialized edge and face boxes.
use martensite_tkernel::standard::Failure;
use martensite_tkhlr::{
    hlr_algo::{MinMaxIndices, add_min_max, copy_min_max, decode_min_max, encode_min_max},
    hlr_brep::Data,
};
// The box of the shape: the union of the edge boxes and the face boxes.
/// Preserve ordered initialized box union; `shape_box(records, ne, nf)` with initialized edges.
pub fn shape_box(ds: &Data, ne: i32, nf: i32) -> Result<MinMaxIndices, Failure> {
    let (mut smin, mut smax) = (MinMaxIndices::default(), MinMaxIndices::default());
    let (mut tmin, mut tmax) = (MinMaxIndices::default(), MinMaxIndices::default());
    let ed = ds.e_data_array();
    for e in 1..=ne {
        decode_min_max(
            ed.value(e).map_err(|x| x.into_failure("probe"))?.min_max(),
            &mut tmin,
            &mut tmax,
        );
        if e == 1 {
            copy_min_max(&tmin, &tmax, &mut smin, &mut smax);
        } else {
            add_min_max(&tmin, &tmax, &mut smin, &mut smax);
        }
    }
    drop(ed);
    let fd = ds.f_data_array();
    for f in 1..=nf {
        let wb = fd
            .value(f)
            .map_err(|x| x.into_failure("probe"))?
            .wires()
            .get()
            .ok_or_else(|| Failure::new("no wires"))?
            .min_max();
        decode_min_max(&wb, &mut tmin, &mut tmax);
        add_min_max(&tmin, &tmax, &mut smin, &mut smax);
    }
    let mut tot = MinMaxIndices::default();
    encode_min_max(&smin, &smax, &mut tot);
    Ok(tot)
}
