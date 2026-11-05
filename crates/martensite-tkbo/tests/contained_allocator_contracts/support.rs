use martensite_tkbo::bop_algo::{
    BOPAlgoBOP, BOPAlgoBuilder, BOPAlgoCellsBuilder, BOPAlgoMakerVolume, BOPAlgoSection,
    BOPAlgoSplitter, BuilderArea, BuilderFace, BuilderSolid,
};
use martensite_tkbrep::topods::{Builder, Compound, Shape};
use martensite_tkernel::ncollection::{BaseAllocator, List};

pub fn policy_cases() -> [Option<BaseAllocator>; 3] {
    [
        None,
        Some(BaseAllocator::common()),
        Some(BaseAllocator::fresh_heap()),
    ]
}
pub fn normalized(policy: &Option<BaseAllocator>) -> BaseAllocator {
    policy.clone().unwrap_or_else(BaseAllocator::common)
}
pub fn source_shapes() -> List<Shape> {
    let mut compound = Compound::default();
    Builder.make_compound(&mut compound);
    let mut shapes = List::with_allocator(BaseAllocator::fresh_heap());
    shapes.append(compound.into_shape());
    shapes
}
pub fn assert_builder_metadata(builder: &BOPAlgoBuilder, expected: &BaseAllocator) {
    for actual in [
        builder.arguments().allocator(),
        builder.images().allocator(),
        builder.origins().allocator(),
        builder.shapes_sd().allocator(),
        builder.in_parts().allocator(),
    ] {
        assert_eq!(actual, expected);
    }
}
pub fn assert_area_metadata(area: &BuilderArea, expected: &BaseAllocator) {
    for actual in [
        area.shapes().allocator(),
        area.loops().allocator(),
        area.areas().allocator(),
    ] {
        assert_eq!(actual, expected);
    }
}

pub fn assert_general_fuse_inheritance(policy: &Option<BaseAllocator>) {
    let expected = normalized(policy);
    let bop = BOPAlgoBOP::with_allocator(policy.clone());
    assert_builder_metadata(&bop, &expected);
    let cells = BOPAlgoCellsBuilder::with_allocator(policy.clone());
    assert_builder_metadata(&cells, &expected);
    let section = BOPAlgoSection::with_allocator(policy.clone());
    assert_builder_metadata(&section, &expected);
    let splitter = BOPAlgoSplitter::with_allocator(policy.clone());
    assert_builder_metadata(&splitter, &expected);
    let maker = BOPAlgoMakerVolume::with_allocator(policy.clone());
    assert_builder_metadata(&maker, &expected);
    assert_eq!(maker.faces().allocator(), &BaseAllocator::common());
}
pub fn assert_area_inheritance(policy: &Option<BaseAllocator>) {
    let expected = normalized(policy);
    let face = BuilderFace::with_allocator(policy.clone());
    assert_area_metadata(&face, &expected);
    let solid = BuilderSolid::with_allocator(policy.clone());
    assert_area_metadata(&solid, &expected);
    assert_eq!(solid.boxes_map().allocator(), &BaseAllocator::common());
}
