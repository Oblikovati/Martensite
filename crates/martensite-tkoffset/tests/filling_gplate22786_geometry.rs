//! Geometry feedback on the plate of bug 22786; the whole view is checked elsewhere.
#[path = "bug22786_gplate/checkshape_response.rs"]
mod checkshape_response;
#[path = "bug22786_gplate/default_shape_counts.rs"]
mod default_shape_counts;
#[path = "bug22786_gplate/geometry.rs"]
mod geometry;
#[path = "bug22786_gplate/printed_area.rs"]
mod printed_area;
use martensite_boolean_cases::common_operation;
use martensite_boolean_cases::support::groups::BUGS;
use martensite_boolean_cases::support::process::verdict_of;
use martensite_boolean_cases::support::rules::Verdict;
use martensite_tkg3d::top_abs::ShapeEnum;

const BOUNDARY: &[u8] =
    include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/OCC22786-c.brep");

fn geometry_body() -> Result<(), Box<dyn std::error::Error>> {
    let boundary = common_operation::restored_shape(BOUNDARY, ShapeEnum::Compound);
    let edges = geometry::boundary_edges(&boundary)?;
    let result = geometry::gplate_from_five(&edges)?.into_shape();
    printed_area::check_area(&result, 2.92409);
    println!(
        "{}",
        checkshape_response::returned_checkshape(&result, "result")?
    );
    default_shape_counts::assert_vertex_first_counts(&result, &[4, 4, 1, 1, 0, 0, 0, 0, 10]);
    println!("TEST COMPLETED");
    Ok(())
}

#[test]
fn gplate_22786_geometry_diagnostic() {
    let identity = "gplate_22786_geometry_diagnostic";
    let verdict = verdict_of(identity, &BUGS, || geometry_body().unwrap())
        .unwrap_or_else(|failure| panic!("case {identity}: {failure}"));
    if let Some(verdict) = verdict {
        assert_eq!(verdict, Verdict::Ok, "case {identity}");
    }
}
