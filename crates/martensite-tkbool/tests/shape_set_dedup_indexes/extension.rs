//! External derived operations can control each oriented index independently.
use super::{collected_shapes, connected_edges, oriented_additions};
use martensite_tkbool::top_ope_brep_build::{TopOpeBRepBuildShapeSet, TopOpeBRepBuildShapeSetBase};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::{ncollection::IndexedMap, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;

#[derive(Clone, Copy)]
enum DedupIndex {
    Start,
    Element,
    Shape,
}

struct ReplayShapeSet {
    base: TopOpeBRepBuildShapeSetBase,
}

impl ReplayShapeSet {
    fn new() -> Result<Self, Failure> {
        Ok(Self {
            base: TopOpeBRepBuildShapeSetBase::new(ShapeEnum::Vertex, false)?,
        })
    }

    fn index(&self, kind: DedupIndex) -> &IndexedMap<Shape> {
        match kind {
            DedupIndex::Start => &self.base.my_omss,
            DedupIndex::Element => &self.base.my_omes,
            DedupIndex::Shape => &self.base.my_omsh,
        }
    }

    fn index_mut(&mut self, kind: DedupIndex) -> &mut IndexedMap<Shape> {
        match kind {
            DedupIndex::Start => &mut self.base.my_omss,
            DedupIndex::Element => &mut self.base.my_omes,
            DedupIndex::Shape => &mut self.base.my_omsh,
        }
    }

    fn add_by_index(&mut self, kind: DedupIndex, shape: &Shape) -> Result<(), Failure> {
        match kind {
            DedupIndex::Start => self.add_start_element(shape),
            DedupIndex::Element => self.add_element(shape),
            DedupIndex::Shape => self.add_shape(shape),
        }
    }

    fn populate(&mut self, shape: &Shape) -> Result<(), Failure> {
        self.add_start_element(shape)?;
        self.add_shape(shape)
    }

    fn observations(&mut self, shape: &Shape) -> Result<[usize; 6], Failure> {
        let incidents =
            usize::try_from(self.max_number_sub_shape(shape)?).expect("nonnegative count");
        Ok([
            self.base.my_omss.len(),
            self.base.my_omes.len(),
            self.base.my_omsh.len(),
            self.start_elements().len(),
            incidents,
            collected_shapes(self)?.len(),
        ])
    }
}

impl TopOpeBRepBuildShapeSet for ReplayShapeSet {
    fn shape_set_base(&self) -> &TopOpeBRepBuildShapeSetBase {
        &self.base
    }
    fn shape_set_base_mut(&mut self) -> &mut TopOpeBRepBuildShapeSetBase {
        &mut self.base
    }
}

#[test]
fn oriented_indexes_are_live_and_repeated_additions_preserve_their_indices() -> Result<(), Failure>
{
    let [edge, _] = connected_edges()?;
    let cases = [
        (DedupIndex::Start, [2, 2, 0, 2, 2, 0]),
        (DedupIndex::Element, [0, 2, 0, 0, 2, 0]),
        (DedupIndex::Shape, [0, 0, 2, 0, 0, 2]),
    ];
    for (kind, expected) in cases {
        let mut set = ReplayShapeSet::new()?;
        let shared = std::ptr::from_ref(set.index(kind));
        assert_eq!(shared, std::ptr::from_mut(set.index_mut(kind)).cast_const());
        for variant in oriented_additions(&edge) {
            set.add_by_index(kind, &variant)?;
        }
        assert_eq!(set.index(kind).find_index(&edge), 1);
        assert_eq!(set.index(kind).find_index(&edge.reversed()), 2);
        assert_eq!(set.observations(&edge)?, expected);
    }
    Ok(())
}

#[test]
fn resetting_each_index_retains_prior_payloads_and_replays_only_that_add_path()
-> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    let cases = [
        (DedupIndex::Start, [1, 1, 1, 2, 1, 1]),
        (DedupIndex::Element, [1, 1, 1, 1, 2, 1]),
        (DedupIndex::Shape, [1, 1, 1, 1, 1, 2]),
    ];
    for (kind, expected) in cases {
        let mut set = ReplayShapeSet::new()?;
        set.populate(&edge)?;
        let before = std::ptr::from_ref(set.index(kind));
        set.index_mut(kind).clear(true);
        set.add_by_index(kind, &edge)?;
        assert_eq!(before, std::ptr::from_ref(set.index(kind)));
        assert_eq!(set.observations(&edge)?, expected);
    }
    Ok(())
}

fn assert_seeded_add(
    kind: DedupIndex,
    expected: [usize; 6],
    reversed: [usize; 6],
    edge: &Shape,
) -> Result<(), Failure> {
    let mut set = ReplayShapeSet::new()?;
    assert_eq!(set.index_mut(kind).add(edge.clone()), 1);
    set.add_by_index(kind, edge)?;
    if matches!(kind, DedupIndex::Element) {
        set.add_start_element(edge)?;
    }
    assert_eq!(set.observations(edge)?, expected);
    set.add_by_index(kind, &edge.reversed())?;
    assert_eq!(set.observations(edge)?, reversed);
    assert_eq!(set.index(kind).find_index(&edge.reversed()), 2);
    Ok(())
}

#[test]
fn seeding_each_index_suppresses_only_matching_oriented_additions() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    assert_seeded_add(
        DedupIndex::Start,
        [1, 0, 0, 0, 0, 0],
        [2, 1, 0, 1, 1, 0],
        &edge,
    )?;
    assert_seeded_add(
        DedupIndex::Element,
        [1, 1, 0, 1, 0, 0],
        [1, 2, 0, 1, 1, 0],
        &edge,
    )?;
    assert_seeded_add(
        DedupIndex::Shape,
        [0, 0, 1, 0, 0, 0],
        [0, 0, 2, 0, 0, 1],
        &edge,
    )?;
    Ok(())
}

#[test]
fn replacing_start_index_controls_replay_without_rebuilding_incidence() -> Result<(), Failure> {
    let [edge, _] = connected_edges()?;
    let mut set = ReplayShapeSet::new()?;
    set.populate(&edge)?;
    let mut replacement = IndexedMap::default();
    replacement.add(edge.reversed());
    set.base.my_omss = replacement;
    set.add_start_element(&edge)?;
    set.add_start_element(&edge.reversed())?;
    assert_eq!(set.observations(&edge)?, [2, 1, 1, 2, 1, 1]);
    assert_eq!(set.base.my_omss.find_index(&edge.reversed()), 1);
    assert_eq!(set.base.my_omss.find_index(&edge), 2);
    assert!(
        set.start_elements()
            .iter()
            .all(|shape| shape.is_equal(&edge))
    );
    Ok(())
}
