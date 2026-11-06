//! `RenderClippingState` replays the recorded clipping_state capture (native
//! GlClipping, no GL context) line by line.
use super::*;
use martensite_tkernel::ncollection::Vec4;

const NATIVE: &str = include_str!("../tests/fixtures/clipping_state/native.txt");

struct Planes(Vec<(&'static str, Handle<Graphic3dClipPlane>)>);

impl Planes {
    fn add(&mut self, name: &'static str, a: f64, d: f64) -> Handle<Graphic3dClipPlane> {
        let equation = Vec4::new(a, 1. - a, 0., d);
        let plane = Handle::new(Graphic3dClipPlane::from_equation(equation).unwrap());
        self.0.push((name, plane.clone()));
        plane
    }

    fn name(&self, plane: &Handle<Graphic3dClipPlane>) -> &'static str {
        if plane.is_null() {
            return "none";
        }
        self.0
            .iter()
            .find(|(_, p)| p == plane)
            .map_or("?", |(n, _)| n)
    }
}

fn sequence(planes: &[&Handle<Graphic3dClipPlane>]) -> Handle<Graphic3dSequenceOfHClipPlane> {
    let sequence = Handle::new(Graphic3dSequenceOfHClipPlane::default());
    for plane in planes {
        sequence.append((*plane).clone());
    }
    sequence
}

fn print(out: &mut Vec<String>, names: &Planes, label: &str, state: &RenderClippingState) {
    let b = u8::from;
    out.push(format!(
        "state {label} on={} capping={} any={} chains={} disabled={} filter={} dae={} eae={} \
         sub={} chain={}",
        state.nb_clipping_or_capping_on(),
        b(state.is_capping_on()),
        b(state.is_clipping_or_capping_on()),
        b(state.has_clipping_chains()),
        b(state.has_disabled()),
        b(state.is_capping_filter_on()),
        b(state.is_capping_disable_all_except()),
        b(state.is_capping_enable_all_except()),
        state.capped_sub_plane(),
        names.name(state.capped_chain()),
    ));
    let mut iter = state.iter();
    while iter.more() {
        out.push(format!(
            "iter {label} i={} g={} d={} plane={}",
            iter.plane_index(),
            b(iter.is_global()),
            b(iter.is_disabled()),
            names.name(&iter.value()),
        ));
        iter.next();
    }
}

#[test]
fn clipping_state_replays_the_native_contract() {
    let mut names = Planes(Vec::new());
    let g1 = names.add("g1", 1., -1.);
    let g2 = names.add("g2", 0., -2.);
    let g2b = names.add("g2b", 0.5, -3.);
    let g2c = names.add("g2c", 0.25, -4.);
    let g3 = names.add("g3", 1., -5.);
    let l1 = names.add("l1", 0., -6.);
    let l2 = names.add("l2", 1., -7.);
    let l2b = names.add("l2b", 0.75, -8.);
    g2.set_capping(true);
    Graphic3dClipPlane::set_chain_next_plane(&g2, g2b.clone()).unwrap();
    Graphic3dClipPlane::set_chain_next_plane(&g2b, g2c).unwrap();
    g3.set_on(false).unwrap();
    l1.set_capping(true);
    Graphic3dClipPlane::set_chain_next_plane(&l2, l2b).unwrap();
    let global = sequence(&[&g1, &g2, &g3]);
    let local = sequence(&[&l1, &l2]);
    let single = sequence(&[&l1]);

    let mut out = Vec::new();
    let mut state = RenderClippingState::default();
    let out = &mut out;
    print(out, &names, "default", &state);
    state.init();
    print(out, &names, "init", &state);
    state.reset(&global);
    print(out, &names, "reset", &state);
    state.set_local_planes(&local);
    print(out, &names, "local", &state);
    out.push(format!(
        "set 2 off={}",
        u8::from(state.set_enabled(2, false))
    ));
    out.push(format!(
        "set 2 off={}",
        u8::from(state.set_enabled(2, false))
    ));
    print(out, &names, "g2-off", &state);
    out.push(format!(
        "set 3 off={}",
        u8::from(state.set_enabled(3, false))
    ));
    print(out, &names, "g3-off", &state);
    state.restore_disabled();
    print(out, &names, "restored", &state);
    state.disable_global();
    print(out, &names, "no-global", &state);
    out.push(format!("set 5 on={}", u8::from(state.set_enabled(5, true))));
    out.push(format!(
        "set 5 off={}",
        u8::from(state.set_enabled(5, false))
    ));
    print(out, &names, "l2-off", &state);
    state.restore_disabled();
    print(out, &names, "restored2", &state);
    state.disable_all_except(&g2, 2);
    print(out, &names, "dae", &state);
    state.enable_all_except(&g2, 1);
    print(out, &names, "eae", &state);
    state.enable_all_except(&l1, 1);
    print(out, &names, "eae-single", &state);
    state.reset_capping_filter();
    print(out, &names, "filter-reset", &state);
    state.set_local_planes(&single);
    print(out, &names, "local-single", &state);
    state.enable_all_except(&l1, 1);
    print(out, &names, "eae-local-single", &state);
    state.set_local_planes(&Handle::default());
    print(out, &names, "local-null", &state);
    g1.set_on(false).unwrap();
    state.reset(&global);
    print(out, &names, "g1-switched-off", &state);
    g1.set_on(true).unwrap();
    state.reset(&local);
    print(out, &names, "reset-local-as-global", &state);
    state.reset(&Handle::default());
    print(out, &names, "reset-null", &state);
    state.init();
    print(out, &names, "init2", &state);

    let native: Vec<&str> = NATIVE.lines().collect();
    assert_eq!(out.len(), native.len());
    for (line, (rust, native)) in out.iter().zip(native).enumerate() {
        assert_eq!(rust, native, "line {}", line + 1);
    }
}

#[test]
fn an_index_past_the_planes_is_refused_and_the_cursor_ends_on_null() {
    let mut names = Planes(Vec::new());
    let plane = names.add("p", 1., 0.);
    let mut state = RenderClippingState::default();
    state.reset(&sequence(&[&plane]));
    assert!(!state.set_enabled(0, false));
    assert!(!state.set_enabled(2, false));
    let mut iter = state.iter();
    assert!(iter.more() && iter.value() == plane);
    iter.next();
    assert!(!iter.more() && iter.value().is_null() && !iter.is_global());
}
