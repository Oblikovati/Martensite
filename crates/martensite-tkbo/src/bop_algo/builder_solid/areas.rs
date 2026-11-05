//! `BOPAlgoBuilderSolid::PerformAreas`: growth solids from the shells, hole shells put
//! into the closest growth solid containing them.
use super::BuilderSolid;
use super::internal::{is_inside, required_context};
use crate::bop_algo::builder_area::progress::observe_user_break;
use crate::bop_algo::{Options, ProgressCancellation};
use crate::bop_tools::{BoxSelector, BoxSet};
use crate::int_tools::Context;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topexp;
use martensite_tkbrep::topods::{Shape, ShapeIterator, Solid};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::{IndexedDataMap, IndexedMap, List};
use martensite_tkernel::standard::{Failure, real::real_small};
use martensite_tkg3d::top_abs::{ShapeEnum, State};
use martensite_tkmath::bnd::{BoundingBox3, box3_to_bvh};
use martensite_tkmath::bvh::{BvhSetTraverse, BvhVec3d};
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;

/// Holes and the growth solid chosen for each (`aHoleSolidMap`).
type HoleSolids = IndexedDataMap<Shape, Shape, ShapeMapHasher>;

struct SolidAreasScope<'a, 'progress> {
    context: &'a Context,
    outer: &'a ProgressScope<'progress>,
    observed: &'a ProgressCancellation,
}

struct ShellGroups {
    growths: Vec<Shape>,
    holes: IndexedMap<Shape, ShapeMapHasher>,
    hole_faces: IndexedMap<Shape, ShapeMapHasher>,
}

impl ShellGroups {
    fn new() -> Self {
        Self {
            growths: Vec::new(),
            holes: IndexedMap::with_hasher(ShapeMapHasher, 1),
            hole_faces: IndexedMap::with_hasher(ShapeMapHasher, 1),
        }
    }

    fn classify(&mut self, shell: &Shape, context: &Context) -> Result<(), Failure> {
        if is_growth_shell(shell, &self.hole_faces) || !is_hole(shell, context)? {
            self.growths.push(solid_of(shell)?);
        } else {
            self.holes.add(shell.clone());
            topexp::map_shapes_of_type(shell, ShapeEnum::Face, &mut self.hole_faces)?;
        }
        Ok(())
    }
}

impl BuilderSolid {
    /// Builds the solids from the shells (`PerformAreas`).
    pub(super) fn perform_areas(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.area.areas.clear();
        let handle = self.area.context.clone();
        let context = required_context(&handle)?;
        let outer = ProgressScope::with_literal(range, "Building solids", 10., false);
        let stage = SolidAreasScope {
            context,
            outer: &outer,
            observed,
        };
        self.classified_areas(&stage)
    }

    fn classified_areas(&mut self, stage: &SolidAreasScope<'_, '_>) -> Result<(), Failure> {
        let groups = classify_shells(&self.area.loops, &self.area.options, stage)?;
        if self.area.options.has_errors() {
            return Ok(());
        }
        if groups.holes.is_empty() {
            return self.publish_growths(&groups.growths);
        }
        self.nested_holes(&groups, stage)
    }

    fn publish_growths(&mut self, growths: &[Shape]) -> Result<(), Failure> {
        for solid in growths {
            self.area.areas.append(solid.clone());
            self.boxes.bind(solid.clone(), shape_box(solid)?);
        }
        Ok(())
    }

    fn nested_holes(
        &mut self,
        groups: &ShellGroups,
        stage: &SolidAreasScope<'_, '_>,
    ) -> Result<(), Failure> {
        let mut tree = self.hole_tree(&groups.holes)?;
        let hole_solids = self.closest_growths(&groups.growths, &groups.holes, &mut tree, stage)?;
        if self.area.options.has_errors() {
            return Ok(());
        }
        self.place_holes(&groups.growths, &hole_solids, stage)?;
        if self.area.options.has_errors() {
            return Ok(());
        }
        self.add_outer_holes(&groups.holes, &hole_solids)
    }

    fn hole_tree(
        &mut self,
        holes: &IndexedMap<Shape, ShapeMapHasher>,
    ) -> Result<BoxSet<BvhVec3d>, Failure> {
        let mut tree = BoxSet::default();
        tree.set_size(holes.extent());
        for (position, hole) in holes.iter().enumerate() {
            let bounds = shape_box(hole)?;
            tree.add(position as i32 + 1, box3_to_bvh(&bounds)?);
            self.boxes.bind(hole.clone(), bounds);
        }
        tree.build()?;
        Ok(tree)
    }

    fn place_holes(
        &mut self,
        growths: &[Shape],
        holes: &HoleSolids,
        stage: &SolidAreasScope<'_, '_>,
    ) -> Result<(), Failure> {
        let by_solid = holes_by_solid(holes)?;
        let scope = ProgressScope::unnamed(&stage.outer.next(1.), growths.len() as f64, false);
        for solid in growths {
            if observe_user_break(&self.area.options, &scope, stage.observed) {
                return Ok(());
            }
            let _step = scope.next(1.);
            self.place_solid_holes(solid, by_solid.seek(solid), stage.context)?;
            self.area.areas.append(solid.clone());
        }
        Ok(())
    }

    fn place_solid_holes(
        &self,
        solid: &Shape,
        holes: Option<&List<Shape>>,
        context: &Context,
    ) -> Result<(), Failure> {
        let Some(holes) = holes else {
            return Ok(());
        };
        let mut shape = solid.clone();
        for hole in holes.iter() {
            BRepBuilder::default().add(&mut shape, hole)?;
        }
        context
            .solid_classifier(&Solid::unchecked(solid.clone()))?
            .write()
            .load(solid)?;
        Ok(())
    }

    /// For each hole, the innermost growth solid containing it, among the growths whose
    /// box meets the hole box; binds the box of each growth.
    fn closest_growths(
        &mut self,
        growths: &[Shape],
        holes: &IndexedMap<Shape, ShapeMapHasher>,
        tree: &mut BoxSet<BvhVec3d>,
        stage: &SolidAreasScope<'_, '_>,
    ) -> Result<HoleSolids, Failure> {
        let mut chosen = IndexedDataMap::with_hasher(ShapeMapHasher, 1);
        let range = stage.outer.next(4.);
        let scope =
            ProgressScope::with_literal(&range, "Adding holes", growths.len() as f64, false);
        for solid in growths {
            if observe_user_break(&self.area.options, &scope, stage.observed) {
                return Ok(chosen);
            }
            let _step = scope.next(1.);
            self.growth_holes(solid, holes, tree, &mut chosen, stage.context)?;
        }
        Ok(chosen)
    }

    fn growth_holes(
        &mut self,
        solid: &Shape,
        holes: &IndexedMap<Shape, ShapeMapHasher>,
        tree: &mut BoxSet<BvhVec3d>,
        chosen: &mut HoleSolids,
        context: &Context,
    ) -> Result<(), Failure> {
        for index in self.hole_candidates(solid, tree)? {
            bind_closest_hole(chosen, holes.find_key(index as usize)?, solid, context)?;
        }
        Ok(())
    }

    fn hole_candidates(
        &mut self,
        solid: &Shape,
        tree: &mut BoxSet<BvhVec3d>,
    ) -> Result<Vec<i32>, Failure> {
        let bounds = shape_box(solid)?;
        self.boxes.bind(solid.clone(), bounds.clone());
        let mut selector = BoxSelector::default();
        selector.set_box(box3_to_bvh(&bounds)?);
        selector.set_bvh_set(Some(&mut *tree));
        selector.select()?;
        Ok(selector.indices().iter().copied().collect())
    }

    /// Holes outside every solid become solids of their own with an infinite box; the boxes
    /// of the hole shells are dropped.
    fn add_outer_holes(
        &mut self,
        holes: &IndexedMap<Shape, ShapeMapHasher>,
        hole_solids: &HoleSolids,
    ) -> Result<(), Failure> {
        for hole in holes.iter() {
            if !hole_solids.contains(hole) {
                let solid = solid_of(hole)?;
                self.area.areas.append(solid.clone());
                let mut bounds = BoundingBox3::default();
                bounds.set_whole();
                self.boxes.bind(solid, bounds);
            }
            self.boxes.unbind(hole);
        }
        Ok(())
    }
}

/// Splits the shells into growth solids and hole shells: a shell sharing a face with a hole
/// is a growth, the others are classified from the infinite point.
fn classify_shells(
    loops: &List<Shape>,
    options: &Options,
    stage: &SolidAreasScope<'_, '_>,
) -> Result<ShellGroups, Failure> {
    let mut groups = ShellGroups::new();
    let range = stage.outer.next(5.);
    let scope = ProgressScope::with_literal(&range, "Classify solids", loops.len() as f64, false);
    for shell in loops.iter() {
        if observe_user_break(options, &scope, stage.observed) {
            return Ok(groups);
        }
        let _step = scope.next(1.);
        groups.classify(shell, stage.context)?;
    }
    Ok(groups)
}

fn bind_closest_hole(
    chosen: &mut HoleSolids,
    hole: &Shape,
    solid: &Shape,
    context: &Context,
) -> Result<(), Failure> {
    if !is_inside(hole, solid, context)? {
        return Ok(());
    }
    if let Some(was) = chosen.change_seek(hole) {
        if is_inside(solid, was, context)? {
            *was = solid.clone();
        }
        return Ok(());
    }
    chosen.add(hole.clone(), solid.clone());
    Ok(())
}

fn solid_of(shell: &Shape) -> Result<Shape, Failure> {
    let builder = BRepBuilder::default();
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    let mut solid = solid.into_shape();
    builder.add(&mut solid, shell)?;
    Ok(solid)
}

fn shape_box(shape: &Shape) -> Result<BoundingBox3, Failure> {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(shape, &mut bounds, true)?;
    Ok(bounds)
}

/// Whether the shell shares a face with a hole shell (`IsGrowthShell`).
fn is_growth_shell(shell: &Shape, hole_faces: &IndexedMap<Shape, ShapeMapHasher>) -> bool {
    hole_faces.extent() > 0
        && ShapeIterator::new(shell, true, true).any(|face| hole_faces.contains(&face))
}

/// Whether the infinite point is inside the shell (`IsHole`): the shell is classified as
/// a solid.
fn is_hole(shell: &Shape, context: &Context) -> Result<bool, Failure> {
    let classifier = context.solid_classifier(&Solid::unchecked(shell.clone()))?;
    let mut classifier = classifier.write();
    classifier.perform_infinite_point(real_small())?;
    Ok(classifier.state() == State::In)
}

/// The back map from growth solids to their holes.
fn holes_by_solid(
    hole_solids: &HoleSolids,
) -> Result<IndexedDataMap<Shape, List<Shape>, ShapeMapHasher>, Failure> {
    let mut solid_holes: IndexedDataMap<Shape, List<Shape>, ShapeMapHasher> =
        IndexedDataMap::with_hasher(ShapeMapHasher, 1);
    for index in 1..=hole_solids.extent() {
        let hole = hole_solids.find_key(index)?.clone();
        let solid = hole_solids.find_from_index(index)?;
        if let Some(list) = solid_holes.change_seek(solid) {
            list.append(hole);
            continue;
        }
        let mut list = List::default();
        list.append(hole);
        solid_holes.add(solid.clone(), list);
    }
    Ok(solid_holes)
}
