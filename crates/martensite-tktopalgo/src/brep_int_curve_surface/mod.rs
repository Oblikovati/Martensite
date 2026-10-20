//! The intersections of a curve with the faces of a
//! shape. `BRepIntCurveSurface_Inter` is [`BRepIntCurveSurfaceInter`]: `IntCurveSurface_HInter` intersects the curve
//! with each face whose box meets it, and `BRepTopAdaptor_TopolTool` keeps the points
//! IN or ON the face.
mod find;
mod inter;

pub use inter::BRepIntCurveSurfaceInter;
