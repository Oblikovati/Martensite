//! The bounding box of a shape, from its triangulations and
//! polygons or from its exact geometry.
//!
//! [`BRepBndLib`] holds the static members of BRepBndLib.cxx and `AddOBB` of BRepBndLib_1.cxx (section 14).
mod add;
mod add_obb;
mod add_optimal;
mod face_box;
mod uv_bounds;

/// `BRepBndLib`: the functions that add a shape to a `Bnd_Box`. They extend the box in
/// place; an exception of an adaptor or of BndLib is returned as an error and leaves
/// the box as far as the native loop got.
///
/// ```
/// use martensite_tkbrep::brep::BRepBuilder;
/// use martensite_tkbrep::topods::Vertex;
/// use martensite_tkmath::bnd::BoundingBox3;
/// use martensite_tkmath::gp::Point3;
/// use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
/// let mut vertex = Vertex::default();
/// BRepBuilder::default().make_vertex_at(&mut vertex, Point3::new(1.0, 2.0, 3.0), 0.5)?;
/// let mut bounds = BoundingBox3::default();
/// BRepBndLib::add(&vertex, &mut bounds, true)?;
/// assert_eq!(bounds.get()?, [0.5, 1.5, 2.5, 1.5, 2.5, 3.5]);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub struct BRepBndLib;
