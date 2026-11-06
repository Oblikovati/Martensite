//! Relate each actual hole to its innermost growth using the existing UV tree.
use super::HoleFaces;
use crate::{
    bop_algo::{Options, ProgressCancellation, builder_area::progress::observe_user_break},
    bop_tools::{BoxSelector, BoxSet},
    int_tools::Context,
};
use martensite_tkbrep::{
    brep_tools::add_uv_bounds,
    topods::{Face, Shape},
    toptools::ShapeMapHasher,
};
use martensite_tkernel::{
    message::{ProgressRange, ProgressScope},
    ncollection::{IndexedDataMap, IndexedMap},
    standard::Failure,
};
use martensite_tkmath::{
    bnd::{BoundingBox2, box2_to_bvh},
    bvh::{BvhSetTraverse, BvhVec2d},
};

struct HoleClassifier<'a> {
    tree: BoxSet<BvhVec2d>,
    holes: &'a IndexedMap<Shape, ShapeMapHasher>,
    context: &'a Context,
    hole_faces: HoleFaces,
}

impl<'a> HoleClassifier<'a> {
    fn new(
        holes: &'a IndexedMap<Shape, ShapeMapHasher>,
        context: &'a Context,
    ) -> Result<Self, Failure> {
        let tree = hole_tree(holes)?;
        let hole_faces = IndexedDataMap::with_hasher(ShapeMapHasher, 1);
        Ok(Self {
            tree,
            holes,
            context,
            hole_faces,
        })
    }

    fn classify_face(&mut self, face: &Face) -> Result<(), Failure> {
        let mut bounds = BoundingBox2::default();
        add_uv_bounds(face, &mut bounds)?;
        let mut selector = BoxSelector::default();
        selector.set_box(box2_to_bvh(&bounds)?);
        selector.set_bvh_set(Some(&mut self.tree));
        selector.select()?;
        for index in selector.indices().iter() {
            let hole = self.holes.find_key(*index as usize)?;
            relate_hole(&mut self.hole_faces, hole, face, self.context)?;
        }
        Ok(())
    }
}

fn hole_tree(holes: &IndexedMap<Shape, ShapeMapHasher>) -> Result<BoxSet<BvhVec2d>, Failure> {
    let mut tree = BoxSet::default();
    tree.set_size(holes.extent());
    for (position, hole) in holes.iter().enumerate() {
        let mut bounds = BoundingBox2::default();
        add_uv_bounds(&Face::unchecked(hole.clone()), &mut bounds)?;
        tree.add(position as i32 + 1, box2_to_bvh(&bounds)?);
    }
    tree.build()?;
    Ok(tree)
}

fn relate_hole(
    map: &mut HoleFaces,
    hole: &Shape,
    face: &Face,
    context: &Context,
) -> Result<(), Failure> {
    if !super::super::is_inside(hole, face, context)? {
        return Ok(());
    }
    if let Some(was) = map.change_seek(hole) {
        if super::super::is_inside(face, was, context)? {
            *was = face.clone().into_shape();
        }
        return Ok(());
    }
    map.add(hole.clone(), face.clone().into_shape());
    Ok(())
}

/// Locate the innermost growth; `closest_growths(growths, holes, context, range, options, observed)`.
pub(super) fn closest_growths(
    growths: &[Face],
    holes: &IndexedMap<Shape, ShapeMapHasher>,
    context: &Context,
    range: &ProgressRange<'_>,
    options: &mut Options,
    observed: &ProgressCancellation,
) -> Result<HoleFaces, Failure> {
    let mut classifier = HoleClassifier::new(holes, context)?;
    let scope = ProgressScope::with_literal(range, "Adding holes", growths.len() as f64, false);
    for face in growths {
        if observe_user_break(options, &scope, observed) {
            break;
        }
        classifier.classify_face(face)?;
        drop(scope.next(1.));
    }
    Ok(classifier.hole_faces)
}
