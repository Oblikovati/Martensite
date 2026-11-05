//! Finite same-domain graphs have independently constructed terminal representatives.
use martensite_tkbo::bop_ds::DataStructure;
const KEYS: [i32; 6] = [i32::MIN, i32::MAX, 0, 42, -1, 1];

fn successors(mut ordinal: usize) -> [Option<usize>; 6] {
    let mut links = [None; 6];
    for (source, link) in links.iter_mut().enumerate() {
        let options = 6 - source;
        let selected = ordinal % options;
        ordinal /= options;
        *link = (selected != 0).then_some(source + selected);
    }
    assert_eq!(ordinal, 0);
    links
}

fn terminal_indices(links: &[Option<usize>; 6]) -> [usize; 6] {
    let mut terminals = [0; 6];
    for source in (0..6).rev() {
        terminals[source] = links[source].map_or(source, |target| terminals[target]);
    }
    terminals
}

fn assert_lookup_contract(ds: &DataStructure, links: &[Option<usize>; 6]) {
    let terminals = terminal_indices(links);
    for (source, key) in KEYS.into_iter().enumerate() {
        let mut output = 987;
        assert_eq!(
            ds.has_shape_sd_into(key, &mut output),
            links[source].is_some()
        );
        let expected_output = links[source].map_or(987, |_| KEYS[terminals[source]]);
        assert_eq!(output, expected_output);
        assert_eq!(ds.same_domain_index(key), KEYS[terminals[source]]);
    }
    let mut absent = 987;
    assert!(!ds.has_shape_sd_into(999, &mut absent));
    assert_eq!(absent, 987);
    assert_eq!(ds.same_domain_index(999), 999);
}

fn populate(ds: &mut DataStructure, links: &[Option<usize>; 6]) {
    for (source, target) in links.iter().enumerate() {
        ds.add_shape_sd(KEYS[source], KEYS[source]);
        if let Some(target) = target {
            ds.add_shape_sd(KEYS[source], KEYS[*target]);
        }
        ds.add_shape_sd(KEYS[source], KEYS[source]);
    }
}

fn assert_direct_bindings(ds: &DataStructure, links: &[Option<usize>; 6]) {
    for (source, target) in links.iter().enumerate() {
        let observed = ds
            .shape_sd_items()
            .find(|(key, _)| **key == KEYS[source])
            .map(|(_, value)| *value);
        assert_eq!(observed, target.map(|target| KEYS[target]));
    }
}

fn rebind_to_last(ds: &mut DataStructure) -> [Option<usize>; 6] {
    let links = [Some(5), Some(5), Some(5), Some(5), Some(5), None];
    for source in KEYS {
        ds.add_shape_sd(source, KEYS[5]);
    }
    links
}

#[test]
fn all_720_ordered_acyclic_maps_preserve_terminals_rebindings_and_output_sentinels() {
    for ordinal in 0..720 {
        let links = successors(ordinal);
        let mut ds = DataStructure::default();
        populate(&mut ds, &links);
        assert_lookup_contract(&ds, &links);
        assert_direct_bindings(&ds, &links);
        let rebound = rebind_to_last(&mut ds);
        assert_lookup_contract(&ds, &rebound);
        assert_direct_bindings(&ds, &rebound);
    }
}
