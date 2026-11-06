//! `RibSlot`: the base of the ribs and slots (linear and revolution forms): the
//! profile built from a wire sketched on the basis, its extreme faces, the gluing or the
//! Boolean operation of the form, and the history of the faces.
mod bound;
mod extreme;
mod extreme_multi;
mod geometry;
mod no_sliding;
mod no_sliding_edges;
mod perform;
mod sliding;
mod sliding_base;

pub(crate) use bound::{max_vertex_tolerance, shell_of};
pub use extreme::Extremes;
pub use geometry::{choice_of_faces, int_par};
pub use sliding::ProfileInput;
pub(crate) use sliding_base::SlidingBase;

use super::form::update_faces_through_bop;
use super::{DescendantMap, PerfSelection, StatusError};
use crate::loc_ope::Gluer;
use martensite_tkbo::brep_algo_api::BRepAlgoAPIBooleanOperation;
use martensite_tkbrep::topods::{self, Face, Shape, Wire};
use martensite_tkbrep::toptools::{MapOfShape, ShapeMapHasher};
use martensite_tkernel::ncollection::{DataMap, List};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShapeBase;

/// `RibSlot`: the data of a rib or slot (the protected members of the reference
/// class), embedded in the linear and revolution forms.
pub struct RibSlot {
    pub(crate) base: BRepBuilderAPIMakeShapeBase,
    pub(crate) first_pnt: Point3,
    pub(crate) last_pnt: Point3,
    pub(crate) fuse: bool,
    pub(crate) sliding: bool,
    pub(crate) map: DescendantMap,
    pub(crate) lf_map: DescendantMap,
    pub(crate) f_shape: Shape,
    pub(crate) l_shape: Shape,
    pub(crate) perf_selection: PerfSelection,
    pub(crate) wire: Wire,
    pub(crate) sbase: Shape,
    pub(crate) skface: Face,
    pub(crate) pbase: Face,
    pub(crate) g_shape: Shape,
    pub(crate) s_until: Shape,
    pub(crate) glued_f: DataMap<Shape, Shape, ShapeMapHasher>,
    pub(crate) new_edges: List<Shape>,
    pub(crate) tgt_edges: List<Shape>,
    pub(crate) faces_for_draft: List<Shape>,
    pub(crate) status_error: StatusError,
}

impl Default for RibSlot {
    /// Neither fused nor sliding, status OK.
    fn default() -> Self {
        Self {
            base: BRepBuilderAPIMakeShapeBase::new(),
            first_pnt: Point3::default(),
            last_pnt: Point3::default(),
            fuse: false,
            sliding: false,
            map: DataMap::default(),
            lf_map: DataMap::default(),
            f_shape: Shape::default(),
            l_shape: Shape::default(),
            perf_selection: PerfSelection::NoSelection,
            wire: Wire::default(),
            sbase: Shape::default(),
            skface: Face::default(),
            pbase: Face::default(),
            g_shape: Shape::default(),
            s_until: Shape::default(),
            glued_f: DataMap::default(),
            new_edges: List::default(),
            tgt_edges: List::default(),
            faces_for_draft: List::default(),
            status_error: StatusError::Ok,
        }
    }
}

/// The shapes of `list` other than `s`.
fn others(list: &List<Shape>, s: &Shape) -> List<Shape> {
    let mut l = List::default();
    for sh in list.iter() {
        if !sh.is_same(s) {
            l.append(sh.clone());
        }
    }
    l
}

impl RibSlot {
    /// Whether `f` has no descendant (no_such_object when unknown);
    /// `rib.rib_is_deleted(&f)?`.
    pub fn rib_is_deleted(&self, f: &Shape) -> Result<bool, Failure> {
        Ok(self.map.find(f)?.is_empty())
    }

    /// The descendants of `f` other than itself (none when unknown);
    /// `rib.rib_modified(&f)?`.
    pub fn rib_modified(&mut self, f: &Shape) -> Result<&List<Shape>, Failure> {
        if self.map.is_bound(f) {
            self.base.my_generated = others(self.map.find(f)?, f);
            return Ok(&self.base.my_generated);
        }
        Ok(&self.base.my_generated)
    }

    /// For an edge or vertex, the faces made of it (through the edges of
    /// the profile it became when it is in the map of the profile); none for a face;
    /// `rib.rib_generated(&e)?`.
    pub fn rib_generated(&mut self, s: &Shape) -> Result<&List<Shape>, Failure> {
        if s.shape_type()? == ShapeEnum::Face {
            return Ok(&self.base.my_generated);
        }
        self.base.my_generated.clear();
        if self.lf_map.is_empty() || !self.lf_map.is_bound(s) {
            if self.map.is_bound(s) {
                // check if filter on face or not
                self.base.my_generated = others(self.map.find(s)?, s);
            }
            return Ok(&self.base.my_generated);
        }
        let mut list = List::default();
        for e in self.lf_map.find(s)?.iter() {
            if self.map.is_bound(e) {
                list.append_list(&mut others(self.map.find(e)?, s));
            }
        }
        self.base.my_generated = list;
        Ok(&self.base.my_generated)
    }

    /// The history through the faces the gluer made of them.
    pub(crate) fn update_descendants_glue(&mut self, g: &Gluer) -> Result<(), Failure> {
        let keys: Vec<Shape> = self.map.items().map(|(k, _)| k.clone()).collect();
        for orig in keys {
            let mut newdsc = MapOfShape::default();
            for fdsc in self.map.find(&orig)?.iter() {
                for d in g.descendant_faces(&topods::face(fdsc)?)?.iter() {
                    newdsc.add(d.clone());
                }
            }
            let list = self.map.change_find(&orig)?;
            list.clear();
            for d in newdsc.iter() {
                list.append(d.clone());
            }
        }
        Ok(())
    }

    /// The history through the faces kept in the
    /// result `s` or modified by the Boolean operation (the faces themselves skipped when
    /// `skip_face`); `rib.update_descendants_bop(&cut, &res, false)?`.
    pub(crate) fn update_descendants_bop(
        &mut self,
        a_bop: &BRepAlgoAPIBooleanOperation,
        s: &Shape,
        skip_face: bool,
    ) -> Result<(), Failure> {
        update_faces_through_bop(&mut self.map, a_bop, s, skip_face, false)
    }

    /// The faces at the start of the form (none without);
    /// `rib.first_shape()?`.
    pub fn first_shape(&self) -> Result<List<Shape>, Failure> {
        if self.f_shape.is_null() {
            return Ok(self.base.my_generated.clone());
        }
        self.map.find(&self.f_shape).cloned()
    }

    /// The faces at the end of the form (none without);
    /// `rib.last_shape()?`.
    pub fn last_shape(&self) -> Result<List<Shape>, Failure> {
        if self.l_shape.is_null() {
            return Ok(self.base.my_generated.clone());
        }
        self.map.find(&self.l_shape).cloned()
    }

    /// The faces of the form to draft; `rib.faces_for_draft()`.
    pub fn faces_for_draft(&self) -> &List<Shape> {
        &self.faces_for_draft
    }

    /// The edges created by the gluing; `rib.new_edges()`.
    pub fn new_edges(&self) -> &List<Shape> {
        &self.new_edges
    }

    /// The tangent edges created by the gluing; `rib.tgt_edges()`.
    pub fn tgt_edges(&self) -> &List<Shape> {
        &self.tgt_edges
    }

    /// Why the form failed; `rib.current_status_error()`.
    pub fn current_status_error(&self) -> StatusError {
        self.status_error
    }
}

/// Implements `BRepBuilderAPIMakeShape` for a form with the `RibSlot` history;
/// the form holds its data in a field named `rib`.
macro_rules! impl_rib_make_shape {
    ($feature:ty) => {
        impl martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape for $feature {
            fn make_shape_base(
                &self,
            ) -> &martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShapeBase {
                &self.rib.base
            }
            fn make_shape_base_mut(
                &mut self,
            ) -> &mut martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShapeBase {
                &mut self.rib.base
            }
            fn modified(
                &mut self,
                s: &martensite_tkbrep::topods::Shape,
            ) -> Result<
                &martensite_tkernel::ncollection::List<martensite_tkbrep::topods::Shape>,
                martensite_tkernel::standard::Failure,
            > {
                self.rib.rib_modified(s)
            }
            fn generated(
                &mut self,
                s: &martensite_tkbrep::topods::Shape,
            ) -> Result<
                &martensite_tkernel::ncollection::List<martensite_tkbrep::topods::Shape>,
                martensite_tkernel::standard::Failure,
            > {
                self.rib.rib_generated(s)
            }
            fn is_deleted(
                &mut self,
                s: &martensite_tkbrep::topods::Shape,
            ) -> Result<bool, martensite_tkernel::standard::Failure> {
                self.rib.rib_is_deleted(s)
            }
        }
    };
}
pub(crate) use impl_rib_make_shape;
