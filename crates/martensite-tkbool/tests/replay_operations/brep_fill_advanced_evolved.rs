//! BRepFillAdvancedEvolved probe operations, the Rust twin of
//! the recorded capture (the records are documented
//! there).
use super::brep_fill_generator::failed;
use super::brep_fill_location_law::at;
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use super::shape::shape;
use super::shapes::tp_shapes;
use martensite_tkbool::brep_fill::BRepFillAdvancedEvolved;
use martensite_tkbrep::topods;

/// `bf_advanced_evolved program spine profile tol solid parallel`.
pub fn advanced_evolved_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let t = match tp_shapes(&mut r) {
        Ok(t) => t,
        Err(error) => return failed(output, error),
    };
    let (spine_index, profile_index) = (r.integer(), r.integer());
    let wires = (|| {
        Ok((
            topods::wire(&at(&t, spine_index)?)?,
            topods::wire(&at(&t, profile_index)?)?,
        ))
    })();
    let (tol, solid, parallel) = (r.real(), r.flag(), r.flag());
    let mut ae = BRepFillAdvancedEvolved::new();
    ae.set_parallel_mode(parallel);
    let performed = wires.and_then(|(spine, profile)| ae.perform(&spine, &profile, tol, solid));
    if let Err(error) = performed {
        return failed(output, error);
    }
    let mut code: u32 = 0;
    output.flag(ae.is_done(Some(&mut code)));
    output.int(code as i32);
    output.attempt(|| shape(output, ae.shape()));
    output.end();
}
