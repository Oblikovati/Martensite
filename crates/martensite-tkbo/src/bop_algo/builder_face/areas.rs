//! `BOPAlgoBuilderFace::PerformAreas`: draft faces from the loops, holes put into the
//! closest growth face containing them.
use super::BuilderFace;
use crate::bop_algo::ProgressCancellation;
use crate::bop_algo::builder_area::progress::observe_user_break;
#[path = "area_classification.rs"]
mod classification;
#[path = "area_holes.rs"]
mod holes;
use crate::int_tools::Context;
use classification::classify_loops;
use holes::closest_growths;
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::topods::{Face, Shape, ShapeIterator};
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::message::{ProgressRange, ProgressScope};
use martensite_tkernel::ncollection::{IndexedDataMap, IndexedMap, List};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;

/// The surface, location and tolerance every new face is made on.
struct FaceSupport {
    surface: Handle<dyn GeomSurface>,
    location: Location,
    tolerance: f64,
}

impl FaceSupport {
    fn from_face(face: &Face) -> Result<Self, Failure> {
        let (surface, location) = BRepTool::surface_with_location(face)?;
        let tolerance = BRepTool::face_tolerance(face)?;
        Ok(Self {
            surface,
            location,
            tolerance,
        })
    }

    fn make_face(&self) -> Result<Face, Failure> {
        let mut face = Face::default();
        BRepBuilder::default().make_face_on_located_surface(
            &mut face,
            &self.surface,
            &self.location,
            self.tolerance,
        )?;
        Ok(face)
    }
}

/// Lexical area-stage state; no borrowed range is stored on a public builder.
struct FaceAreaScope<'a, 'p> {
    support: FaceSupport,
    context: Handle<Context>,
    progress: ProgressScope<'p>,
    observed: &'a ProgressCancellation,
}

impl<'a, 'p> FaceAreaScope<'a, 'p> {
    fn new(
        face: &Face,
        context: Handle<Context>,
        range: &ProgressRange<'p>,
        observed: &'a ProgressCancellation,
    ) -> Result<Self, Failure> {
        let support = FaceSupport::from_face(face)?;
        let progress = ProgressScope::unnamed(range, 10., false);
        super::required_context(&context)?;
        Ok(Self {
            support,
            context,
            progress,
            observed,
        })
    }

    fn context(&self) -> Result<&Context, Failure> {
        // The handle remains the exact receiver context, including its existing caches.
        super::required_context(&self.context)
    }
}

/// Holes and the growth face chosen for each (`aHoleFaceMap`).
type HoleFaces = IndexedDataMap<Shape, Shape, ShapeMapHasher>;
/// Growth faces and their holes (`aFaceHolesMap`).
type FaceHoles = IndexedDataMap<Shape, List<Shape>, ShapeMapHasher>;

impl BuilderFace {
    /// Builds the draft faces from the actual loops; `face.perform_areas(range, observed)`.
    pub(super) fn perform_areas(
        &mut self,
        range: &ProgressRange<'_>,
        observed: &ProgressCancellation,
    ) -> Result<(), Failure> {
        self.area.areas.clear();
        let work = FaceAreaScope::new(&self.face, self.area.context.clone(), range, observed)?;
        self.build_face_areas(&work)
    }

    fn build_face_areas(&mut self, work: &FaceAreaScope<'_, '_>) -> Result<(), Failure> {
        if self.area.loops.is_empty() {
            return self.infinite_area(&work.support, work.context()?);
        }
        let (growths, holes) = self.classified_face_loops(work)?;
        if self.area.options.has_errors() {
            return Ok(());
        }
        if holes.is_empty() {
            self.append_growths(growths);
            return Ok(());
        }
        self.finish_growth_holes(growths, holes, work)
    }

    fn classified_face_loops(
        &mut self,
        work: &FaceAreaScope<'_, '_>,
    ) -> Result<(Vec<Face>, IndexedMap<Shape, ShapeMapHasher>), Failure> {
        classify_loops(
            &self.area.loops,
            &work.support,
            work.context()?,
            &work.progress.next(5.),
            &mut self.area.options,
            work.observed,
        )
    }

    fn infinite_area(&mut self, support: &FaceSupport, context: &Context) -> Result<(), Failure> {
        if context.is_infinite_face(&self.face)? {
            let face = support.make_face()?;
            if BRepTool::natural_restriction(&self.face)? {
                BRepBuilder::default().natural_restriction(&face, true)?;
            }
            self.area.areas.append(face.into_shape());
        }
        Ok(())
    }

    fn append_growths(&mut self, growths: Vec<Face>) {
        for face in growths {
            self.area.areas.append(face.into_shape());
        }
    }

    fn finish_growth_holes(
        &mut self,
        mut growths: Vec<Face>,
        holes: IndexedMap<Shape, ShapeMapHasher>,
        work: &FaceAreaScope<'_, '_>,
    ) -> Result<(), Failure> {
        let Some(map) = self.growth_holes_map(&mut growths, &holes, work)? else {
            return Ok(());
        };
        self.publish_draft_faces(growths, &map, work)
    }

    fn nearest_hole_faces(
        &mut self,
        faces: &[Face],
        holes: &IndexedMap<Shape, ShapeMapHasher>,
        work: &FaceAreaScope<'_, '_>,
    ) -> Result<HoleFaces, Failure> {
        closest_growths(
            faces,
            holes,
            work.context()?,
            &work.progress.next(4.),
            &mut self.area.options,
            work.observed,
        )
    }

    fn growth_holes_map(
        &mut self,
        drafts: &mut Vec<Face>,
        holes: &IndexedMap<Shape, ShapeMapHasher>,
        work: &FaceAreaScope<'_, '_>,
    ) -> Result<Option<FaceHoles>, Failure> {
        let hole_faces = self.nearest_hole_faces(drafts, holes, work)?;
        if self.area.options.has_errors() {
            return Ok(None);
        }
        let mut map = holes_by_face(&hole_faces)?;
        if holes.extent() != hole_faces.extent() {
            self.add_unused_holes(&work.support, holes, &hole_faces, &mut map, drafts)?;
        }
        Ok(Some(map))
    }

    fn publish_draft_faces(
        &mut self,
        growths: Vec<Face>,
        holes: &FaceHoles,
        work: &FaceAreaScope<'_, '_>,
    ) -> Result<(), Failure> {
        let scope = ProgressScope::unnamed(&work.progress.next(1.), growths.len() as f64, false);
        for face in growths {
            if observe_user_break(&self.area.options, &scope, work.observed) {
                return Ok(());
            }
            if let Some(face_holes) = holes.seek(&face) {
                add_holes(&face, face_holes, work.support.tolerance, work.context()?)?;
            }
            // The face is a draft that does not contain any internal shapes.
            self.area.areas.append(face.into_shape());
            drop(scope.next(1.));
        }
        Ok(())
    }

    /// Holes contained in no growth face go into a face on the whole surface when the face
    /// is unbounded.
    fn add_unused_holes(
        &self,
        support: &FaceSupport,
        holes: &IndexedMap<Shape, ShapeMapHasher>,
        hole_faces: &HoleFaces,
        face_holes: &mut FaceHoles,
        growths: &mut Vec<Face>,
    ) -> Result<(), Failure> {
        if !self.face_is_unbounded()? {
            return Ok(());
        }
        let face = support.make_face()?;
        let unused = unused_holes(holes, hole_faces);
        face_holes.add(face.clone().into_shape(), unused);
        growths.push(face);
        Ok(())
    }
    fn face_is_unbounded(&self) -> Result<bool, Failure> {
        let mut bounds = BoundingBox3::default();
        BRepBndLib::add(&self.face, &mut bounds, true)?;
        Ok(bounds.is_open_xmin()
            || bounds.is_open_xmax()
            || bounds.is_open_ymin()
            || bounds.is_open_ymax()
            || bounds.is_open_zmin()
            || bounds.is_open_zmax())
    }
}

fn unused_holes(holes: &IndexedMap<Shape, ShapeMapHasher>, hole_faces: &HoleFaces) -> List<Shape> {
    let mut unused = List::default();
    for hole in holes.iter() {
        if !hole_faces.contains(hole) {
            unused.append(hole.clone());
        }
    }
    unused
}

/// The back map from growth faces to their holes.
fn holes_by_face(hole_faces: &HoleFaces) -> Result<FaceHoles, Failure> {
    let mut face_holes: FaceHoles = IndexedDataMap::with_hasher(ShapeMapHasher, 1);
    for index in 1..=hole_faces.extent() {
        let hole = hole_faces.find_key(index)?.clone();
        let face = hole_faces.find_from_index(index)?;
        if let Some(list) = face_holes.change_seek(face) {
            list.append(hole);
            continue;
        }
        let mut list = List::default();
        list.append(hole);
        face_holes.add(face.clone(), list);
    }
    Ok(face_holes)
}

/// Adds the wire of each hole face to `face` and updates its cached classifier.
fn add_holes(
    face: &Face,
    holes: &List<Shape>,
    tolerance: f64,
    context: &Context,
) -> Result<(), Failure> {
    let mut shape = face.clone().into_shape();
    for hole in holes.iter() {
        // The hole face contains only one wire.
        if let Some(wire) = Iterator::next(&mut ShapeIterator::new(hole, true, true)) {
            BRepBuilder::default().add(&mut shape, &wire)?;
        }
    }
    context.fclass2d(face)?.write().init(face, tolerance)
}
