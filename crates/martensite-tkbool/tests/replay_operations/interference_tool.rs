//! Counterpart of the recorded capture.
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use martensite_tkbool::top_ope_brep_ds::*;
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCurve, Geom2dLine};
use martensite_tkg3d::top_abs::Orientation;
use martensite_tkmath::gp::{Direction2, Point2};
type Tool = TopOpeBRepDSInterferenceTool;
type Kind = TopOpeBRepDSKind;

fn interference_fields(output: &BufferedTokens, interference: &Handle<TopOpeBRepDSInterference>) {
    let value = interference.get().unwrap();
    output.text(interference.dynamic_type().unwrap().name().as_bytes());
    output.int(value.support_type() as i32);
    output.int(value.support());
    output.int(value.geometry_type() as i32);
    output.int(value.geometry());
    let transition = value.transition();
    output.int(transition.before() as i32);
    output.int(transition.after() as i32);
    output.int(transition.shape_before() as i32);
    output.int(transition.shape_after() as i32);
    output.int(transition.index_before());
    output.int(transition.index_after());
}

fn interference_curves(
    output: &BufferedTokens,
    transition: &TopOpeBRepDSTransition,
    support: i32,
    geometry: i32,
    parameter: f64,
) {
    let edge = Tool::make_edge_interference(
        transition,
        Kind::Edge,
        support,
        Kind::Vertex,
        geometry,
        parameter,
    );
    let curve = Tool::make_curve_interference(
        transition,
        Kind::Curve,
        support,
        Kind::Point,
        geometry,
        parameter,
    );
    interference_fields(output, &edge);
    output.real(Tool::parameter(&edge).unwrap());
    interference_fields(output, &curve);
    output.real(Tool::parameter(&curve).unwrap());
    let copy = Tool::duplicate_curve_point_interference(&curve).unwrap();
    interference_fields(output, &copy);
    output.flag(copy.object_address() != curve.object_address());
    let alias = curve.clone();
    Tool::set_parameter(&curve, parameter + 1.0).unwrap();
    output.real(Tool::parameter(&alias).unwrap());
    output.real(Tool::parameter(&copy).unwrap());
}

fn interference_surfaces(
    output: &BufferedTokens,
    transition: &TopOpeBRepDSTransition,
    support: i32,
    geometry: i32,
    has_curve: bool,
) {
    let pcurve: Handle<dyn Geom2dCurve> = if has_curve {
        Handle::new(Geom2dLine::from_point(Point2::new(1., 2.), Direction2::X)).cast()
    } else {
        Handle::default()
    };
    let face = Tool::make_face_curve_interference(transition, support, geometry, &pcurve);
    interference_fields(output, &face);
    let typed: Handle<TopOpeBRepDSSurfaceCurveInterference> = face.cast();
    let retained = typed.get().unwrap().pcurve();
    output.flag(retained.object_address() == pcurve.object_address());
    output.flag(retained.is_null());
    if let Some(curve) = retained.get() {
        output.pnt2d(curve.value(0.5).unwrap());
    }
    interference_fields(
        output,
        &Tool::make_solid_surface_interference(transition, support, geometry),
    );
}

fn interference_shapes(
    output: &BufferedTokens,
    transition: &TopOpeBRepDSTransition,
    support: i32,
    geometry: i32,
    parameter: f64,
    config: TopOpeBRepDSConfig,
    bound: bool,
) {
    let edge = Tool::make_edge_vertex_interference(
        transition, support, geometry, bound, config, parameter,
    );
    interference_fields(output, &edge);
    let typed: Handle<TopOpeBRepDSEdgeVertexInterference> = edge.cast();
    output.flag(typed.get().unwrap().gbound());
    output.int(typed.get().unwrap().config() as i32);
    output.real(typed.get().unwrap().parameter());
    let face = Tool::make_face_edge_interference(transition, support, geometry, bound, config);
    interference_fields(output, &face);
    let typed: Handle<TopOpeBRepDSFaceEdgeInterference> = face.cast();
    output.flag(typed.get().unwrap().gbound());
    output.int(typed.get().unwrap().config() as i32);
}

/// Compares every factory and shared parameter edit; `interference_tool_case(output, &[0., 1., 2., 0.5, 0., 1.])`.
pub fn interference_tool_case(output: &BufferedTokens, values: &[f64]) {
    let mut reader = Reader::new(values);
    let mut transition =
        TopOpeBRepDSTransition::from_orientation(Orientation::ALL[reader.integer() as usize]);
    transition.set_index_before(11);
    transition.set_index_after(13);
    let support = reader.integer();
    let geometry = reader.integer();
    let parameter = reader.real();
    let configs = [
        TopOpeBRepDSConfig::UnshGeometry,
        TopOpeBRepDSConfig::SameOriented,
        TopOpeBRepDSConfig::DiffOriented,
    ];
    let config = configs[reader.integer() as usize];
    let bound = reader.integer() != 0;
    interference_curves(output, &transition, support, geometry, parameter);
    interference_surfaces(output, &transition, support, geometry, bound);
    interference_shapes(
        output,
        &transition,
        support,
        geometry,
        parameter,
        config,
        bound,
    );
    output.end();
}
