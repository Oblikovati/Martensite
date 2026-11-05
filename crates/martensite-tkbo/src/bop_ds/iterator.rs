//! AABB candidate-pair preparation for Boolean interference algorithms.
use super::{DataStructure, IndexRange, Pair, ShapeInfo};
mod borrowed;
mod construction;
mod extended;
mod self_interference;
use crate::int_tools::Context;
pub use borrowed::BorrowedIterator;
use martensite_tkernel::{
    ncollection::{BaseAllocator, DynamicArray},
    standard::Failure,
};
use martensite_tkg3d::top_abs::ShapeEnum;
pub use self_interference::BOPDSIteratorSI;

#[derive(Clone, Copy)]
struct PreparationOptions<'a> {
    context: Option<&'a Context>,
    check_obb: bool,
    fuzzy: f64,
}

#[derive(Clone, Copy)]
enum PreparationKind {
    AcrossArguments,
    SelfInterference,
}

/// `BOPDSIterator` candidate pairs grouped by native interference type.
/// `let mut iterator = BOPDSIterator::default(); iterator.prepare(&ds)?;`.
pub struct BOPDSIterator {
    allocator: Option<BaseAllocator>,
    preparation_kind: PreparationKind,
    lists: DynamicArray<DynamicArray<Pair>>,
    extended_lists: DynamicArray<DynamicArray<Pair>>,
    use_extended: bool,
    active_pairs: Vec<Pair>,
    shape_types: Vec<ShapeEnum>,
    position: usize,
    length: i32,
    run_parallel: bool,
}

impl BOPDSIterator {
    /// Builds source-range-filtered AABB candidates from an initialized data structure.
    /// `iterator.prepare(&ds)?`.
    pub fn prepare(&mut self, structure: &DataStructure) -> Result<(), Failure> {
        self.prepare_with_context(structure, None, false, 0.0)
    }

    /// Applies the optional cached OBB rejection used by the native iterator.
    /// `iterator.prepare_with_context(&ds, Some(&context), true, fuzzy)?`.
    pub fn prepare_with_context(
        &mut self,
        structure: &DataStructure,
        context: Option<&Context>,
        check_obb: bool,
        fuzzy: f64,
    ) -> Result<(), Failure> {
        self.clear_lists();
        self.shape_types = source_types(structure)?;
        let options = PreparationOptions {
            context,
            check_obb,
            fuzzy,
        };
        // Prepare shares its prefix before the overridable Intersect step.
        if matches!(self.preparation_kind, PreparationKind::SelfInterference) {
            return self_interference::intersect_self_interferences(self, structure, options);
        }
        let candidates = self.candidate_pairs(structure, ShapeInfo::has_brep)?;
        let mut candidate_index = 0;
        for range_index in 0..structure.nb_ranges() {
            let range = structure.range(range_index)?;
            self.add_range_pairs(structure, range, &candidates, &mut candidate_index, options)?;
        }
        Ok(())
    }

    /// Selects one shape-type pair and resets traversal; `iterator.initialize(Vertex, Edge)`.
    pub fn initialize(&mut self, first: ShapeEnum, second: ShapeEnum) {
        let Some(pairs) = candidate_group(
            &self.lists,
            &self.extended_lists,
            self.use_extended,
            first,
            second,
        ) else {
            self.length = 0;
            return;
        };
        self.active_pairs.clear();
        self.active_pairs.extend(pairs.iter().copied());
        self.active_pairs.sort_by_key(Pair::indices);
        self.position = 0;
        self.length = self.active_pairs.len() as i32;
    }

    /// Counts candidates without changing the selected pair, order or cursor.
    /// Example: `iterator.candidate_count(ShapeEnum::Vertex, ShapeEnum::Edge)`.
    pub(crate) fn candidate_count(&self, first: ShapeEnum, second: ShapeEnum) -> usize {
        let group = candidate_group(
            &self.lists,
            &self.extended_lists,
            self.use_extended,
            first,
            second,
        );
        group.map_or(0, DynamicArray::len)
    }

    /// Reports whether `value` can read another pair; `iterator.more()`.
    pub fn more(&self) -> bool {
        self.position < self.active_pairs.len()
    }

    /// Advances to the next selected pair; `iterator.next()`.
    pub fn next(&mut self) {
        self.position += usize::from(self.more());
    }

    /// Reads the current pair using shape types cached during preparation.
    /// `let (first, second) = iterator.value()?`.
    pub fn value(&self) -> Result<(i32, i32), Failure> {
        self.value_from_types(|index| stored_shape_type(&self.shape_types, index))
    }

    /// Orients the current pair using current records in the explicitly borrowed source.
    /// The query does not bind, copy, prepare or advance the source or iterator.
    /// `let pair = iterator.value_with_structure(&structure)?;`
    pub fn value_with_structure(&self, structure: &DataStructure) -> Result<(i32, i32), Failure> {
        self.value_from_types(|index| structure.shape_info(index).map(|info| info.shape_type()))
    }

    fn value_from_types(
        &self,
        shape_type: impl Fn(i32) -> Result<ShapeEnum, Failure>,
    ) -> Result<(i32, i32), Failure> {
        let pair = self
            .active_pairs
            .get(self.position)
            .ok_or_else(|| Failure::new("BOPDSIterator::Value called without a current pair"))?;
        orient_pair(*pair, shape_type)
    }

    /// Reads the selected pair count; `iterator.expected_length()`.
    pub fn expected_length(&self) -> i32 {
        self.length
    }

    /// A half-length task estimate with a minimum of one.
    /// `iterator.block_length()`.
    pub fn block_length(&self) -> i32 {
        if self.length <= 1 {
            return 1;
        }
        (0.5 * f64::from(self.length)) as i32
    }

    /// Sets the parallel-processing option, retained for later intersection stages.
    /// `iterator.set_run_parallel(true)`.
    pub fn set_run_parallel(&mut self, enabled: bool) {
        self.run_parallel = enabled;
    }

    /// Reads the parallel-processing option; `iterator.run_parallel()`.
    pub fn run_parallel(&self) -> bool {
        self.run_parallel
    }

    fn clear_lists(&mut self) {
        for list in self.lists.iter_mut() {
            list.clear(false);
        }
        self.active_pairs.clear();
        self.position = 0;
        self.length = 0;
    }

    /// The sorted box-overlapping pairs of the source shapes that `selected` keeps: the shapes
    /// with a B-rep for `Intersect`, the interfering ones for `BOPDSIteratorSI::Intersect`.
    fn candidate_pairs(
        &self,
        structure: &DataStructure,
        selected: fn(&ShapeInfo) -> bool,
    ) -> Result<Vec<Pair>, Failure> {
        let mut boxes = crate::bop_tools::BoxSet::default();
        boxes.set_size(structure.nb_source_shapes() as usize);
        for index in 0..structure.nb_source_shapes() {
            let info = structure.shape_info(index)?;
            if selected(info) {
                boxes.add(
                    index,
                    martensite_tkmath::bnd::box3_to_bvh(info.box_bounds())?,
                );
            }
        }
        boxes.build()?;
        let mut selector = crate::bop_tools::PairSelector::default();
        selector.set_shared_bvh_set(&mut boxes);
        selector.set_same(true);
        selector.select()?;
        selector.sort();
        Ok(selector
            .pairs()
            .iter()
            .map(|pair| Pair::new(pair.id1, pair.id2))
            .collect())
    }

    fn add_range_pairs(
        &mut self,
        structure: &DataStructure,
        range: &IndexRange,
        candidates: &[Pair],
        candidate_index: &mut usize,
        options: PreparationOptions<'_>,
    ) -> Result<(), Failure> {
        while let Some(pair) = candidates.get(*candidate_index) {
            let (first, second) = pair.indices();
            if !range.contains(first) {
                break;
            }
            *candidate_index += 1;
            if range.contains(second)
                || self.is_contained_pair(structure, first, second)?
                || self.is_out_obb(structure, first, second, options)?
            {
                continue;
            }
            self.add_typed_pair(structure, *pair)?;
        }
        Ok(())
    }

    fn is_out_obb(
        &self,
        structure: &DataStructure,
        first: i32,
        second: i32,
        options: PreparationOptions<'_>,
    ) -> Result<bool, Failure> {
        if !options.check_obb {
            return Ok(false);
        }
        let Some(context) = options.context else {
            return Ok(false);
        };
        let first_shape = structure.shape(first)?;
        let second_shape = structure.shape(second)?;
        let first_box = context.obb(first_shape, options.fuzzy)?;
        let second_box = context.obb(second_shape, options.fuzzy)?;
        Ok(first_box.read().is_out_box(&second_box.read()))
    }

    fn is_contained_pair(
        &self,
        structure: &DataStructure,
        first: i32,
        second: i32,
    ) -> Result<bool, Failure> {
        let first_info = structure.shape_info(first)?;
        let second_info = structure.shape_info(second)?;
        let first_rank = shape_rank(first_info.shape_type());
        let second_rank = shape_rank(second_info.shape_type());
        Ok(
            (first_rank < second_rank && first_info.has_sub_shape(second))
                || (first_rank > second_rank && second_info.has_sub_shape(first)),
        )
    }

    fn add_typed_pair(&mut self, structure: &DataStructure, pair: Pair) -> Result<(), Failure> {
        let (first, second) = pair.indices();
        let first_type = structure.shape_info(first)?.shape_type();
        let second_type = structure.shape_info(second)?.shape_type();
        if let Some(index) = interference_index(first_type, second_type) {
            self.lists[index].append(Pair::new(first.min(second), first.max(second)));
        }
        Ok(())
    }
}

fn orient_pair(
    pair: Pair,
    shape_type: impl Fn(i32) -> Result<ShapeEnum, Failure>,
) -> Result<(i32, i32), Failure> {
    let (first, second) = pair.indices();
    let first_type = shape_type(first)?;
    let second_type = shape_type(second)?;
    if shape_rank(first_type) < shape_rank(second_type) {
        return Ok((second, first));
    }
    Ok((first, second))
}

fn source_types(structure: &DataStructure) -> Result<Vec<ShapeEnum>, Failure> {
    (0..structure.nb_source_shapes())
        .map(|index| structure.shape_info(index).map(|info| info.shape_type()))
        .collect()
}

fn stored_shape_type(types: &[ShapeEnum], index: i32) -> Result<ShapeEnum, Failure> {
    types.get(index as usize).copied().ok_or_else(|| {
        Failure::new(&format!(
            "BOPDSIterator::Value source index {index} is outside type table length {}",
            types.len()
        ))
    })
}

/// The interference list of two shape types, None for a pair that never interferes.
fn interference_index(first: ShapeEnum, second: ShapeEnum) -> Option<usize> {
    usize::try_from(super::tools::type_pair_to_integer(first, second)).ok()
}

/// The rank of a shape type (`BOPDSTools::TypeToInteger`).
fn shape_rank(shape_type: ShapeEnum) -> i32 {
    super::tools::type_to_integer(shape_type)
}

fn candidate_group<'a>(
    lists: &'a DynamicArray<DynamicArray<Pair>>,
    extended: &'a DynamicArray<DynamicArray<Pair>>,
    use_extended: bool,
    first: ShapeEnum,
    second: ShapeEnum,
) -> Option<&'a DynamicArray<Pair>> {
    let index = interference_index(first, second)?;
    if use_extended && index < BOPDSIterator::nb_ext_interfs() as usize {
        return Some(&extended[index]);
    }
    Some(&lists[index])
}

#[cfg(test)]
#[path = "iterator/candidate_count_contract.rs"]
mod candidate_count_contract;
