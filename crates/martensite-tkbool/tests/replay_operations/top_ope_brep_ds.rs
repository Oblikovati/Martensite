//! TopOpeBRepDS probe operations, the Rust twin of the recorded capture
//! (the records are documented there).
use super::buffered_tokens::BufferedTokens;
use super::common::Reader;
use martensite_tkbool::top_ope_brep_ds::{
    TopOpeBRepDSConfig, TopOpeBRepDSKind, TopOpeBRepDSTransition, is_geometry, is_topology,
    kind_to_shape, shape_to_kind, sprint_config, sprint_kind, sprint_kind_index,
    sprint_orientation, sprint_shape_type_index, sprint_state,
};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum, State};

/// The kinds in their native order.
const KINDS: [TopOpeBRepDSKind; 12] = [
    TopOpeBRepDSKind::Point,
    TopOpeBRepDSKind::Curve,
    TopOpeBRepDSKind::Surface,
    TopOpeBRepDSKind::Vertex,
    TopOpeBRepDSKind::Edge,
    TopOpeBRepDSKind::Wire,
    TopOpeBRepDSKind::Face,
    TopOpeBRepDSKind::Shell,
    TopOpeBRepDSKind::Solid,
    TopOpeBRepDSKind::CompSolid,
    TopOpeBRepDSKind::Compound,
    TopOpeBRepDSKind::Unknown,
];

/// The configurations in their native order.
const CONFIGS: [TopOpeBRepDSConfig; 3] = [
    TopOpeBRepDSConfig::UnshGeometry,
    TopOpeBRepDSConfig::SameOriented,
    TopOpeBRepDSConfig::DiffOriented,
];

/// `bl_transition`.
fn transition(output: &BufferedTokens, t: &TopOpeBRepDSTransition) {
    output.int(t.before() as i32);
    output.int(t.after() as i32);
    output.int(t.shape_before() as i32);
    output.int(t.shape_after() as i32);
    output.int(t.index_before());
    output.int(t.index_after());
}

/// `ds_transition sb sa onb ona ib ia o`.
pub fn transition_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let (sb, sa) = (
        State::ALL[r.integer() as usize],
        State::ALL[r.integer() as usize],
    );
    let (onb, ona) = (
        ShapeEnum::ALL[r.integer() as usize],
        ShapeEnum::ALL[r.integer() as usize],
    );
    let (ib, ia) = (r.integer(), r.integer());
    let o = Orientation::ALL[r.integer() as usize];
    let mut t = TopOpeBRepDSTransition::new(sb, sa, onb, ona);
    t.set_index_before(ib);
    t.set_index_after(ia);
    for s in State::ALL {
        output.int(t.orientation(s, ShapeEnum::Face) as i32);
    }
    transition(output, &t.complement());
    output.flag(t.is_unknown());
    output.attempt(|| t.index().map(|value| output.int(value)));
    let mut c = t;
    c.set_orientation(o);
    transition(output, &c);
    transition(output, &TopOpeBRepDSTransition::from_orientation(o));
    output.end();
}

/// `ds_kind k i`.
pub fn kind_case(output: &BufferedTokens, values: &[f64]) {
    let mut r = Reader::new(values);
    let (k, i) = (r.integer(), r.integer());
    let kind = KINDS[k as usize];
    output.text(sprint_kind(kind).as_bytes());
    output.text(sprint_kind_index(kind, i, "<", ">").as_bytes());
    output.flag(is_geometry(kind));
    output.flag(is_topology(kind));
    output.int(kind_to_shape(kind) as i32);
    if k <= 8 {
        let shape = ShapeEnum::ALL[k as usize];
        output.attempt(|| shape_to_kind(shape).map(|v| output.int(v as i32)));
        output.attempt(|| sprint_shape_type_index(shape, i).map(|v| output.text(v.as_bytes())));
    }
    if k < 4 {
        output.text(sprint_state(State::ALL[k as usize]).as_bytes());
        output.text(sprint_orientation(Orientation::ALL[k as usize]).as_bytes());
    }
    if k < 3 {
        output.text(sprint_config(CONFIGS[k as usize]).as_bytes());
    }
    output.end();
}
