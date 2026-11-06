//! The presentations of PMI in `STEPCAFControlReader`: annotation curves and tessellated items,
//! annotation planes and connection points.
use super::reader::WHERE;
use super::reader_support::{SessionGraph, first_of};
use crate::step_ap242::{
    StepAP242DraughtingModelItemAssociation, StepAP242GeometricItemSpecificUsage,
};
use crate::step_construct::StepConstruct;
use crate::step_control::StepControlActorRead;
use crate::step_data::{StepDataFactors, StepDataSelectType};
use crate::step_entity::{step_array1_item, step_array2_item, step_live};
use crate::step_geom::{StepGeomAxis2Placement3d, StepGeomCartesianPoint, StepGeomPlane};
use crate::step_repr::{StepReprDerivedShapeAspect, StepReprRepresentation};
use crate::step_shape::{
    StepShapeDimensionalCharacteristicRepresentation, StepShapeDimensionalLocation,
    StepShapeDimensionalSize,
};
use crate::step_to_geom::StepToGeom;
use crate::step_visual::{
    StepVisualAnnotationOccurrence, StepVisualAnnotationPlane,
    StepVisualComplexTriangulatedSurfaceSet, StepVisualDraughtingCallout, StepVisualPlanarBox,
    StepVisualRepositionedTessellatedGeometricSet, StepVisualStyledItem,
    StepVisualTessellatedAnnotationOccurrence, StepVisualTessellatedCurveSet,
    StepVisualTessellatedGeometricSet, StepVisualTessellatedItem,
};
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::{Compound, Face, Shape, Wire};
use martensite_tkernel::ncollection::HArray1;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkmath::bnd::BoundingBox3;
use martensite_tkmath::gp::{self, Axis2, Axis3, Direction3, Point3, Transform3};
use martensite_tkmath::poly::{Triangle, Triangulation};
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_bnd_lib::BRepBndLib;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeShape};
use martensite_tkxcaf::xcaf_dim_tol_objects::{
    XCAFDimTolObjectsDatumObject, XCAFDimTolObjectsDimensionObject,
    XCAFDimTolObjectsGeomToleranceObject,
};
use martensite_tkxsbase::interface::InterfaceInterfaceModel;
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_algo::XSAlgoShapeProcessor;

mod presentations;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use presentations::*;

type TransientArray = Handle<HArray1<Handle<StandardTransient>>>;

/// The indices of a strip or a fan.
fn indices_at(list: &TransientArray, rank: i32) -> Result<Vec<i32>, Failure> {
    let entry = step_array1_item(list, rank, WHERE)?.cast::<HArray1<i32>>();
    Ok(step_live(&entry, WHERE)?.array1().as_slice().to_vec())
}

/// The triangles of a strip that are not degenerate, in two groups to path through the nodes in the
/// same direction.
fn strip_triangles(strip: &[i32]) -> Vec<Triangle> {
    let valid = |index: usize| strip[index] != strip[index - 2] && strip[index] != strip[index - 1];
    let odd = (2..strip.len()).step_by(2).filter(|index| valid(*index));
    let odd = odd.map(|index| Triangle::new(strip[index - 2], strip[index], strip[index - 1]));
    let even = (3..strip.len()).step_by(2).filter(|index| valid(*index));
    let even = even.map(|index| Triangle::new(strip[index - 2], strip[index - 1], strip[index]));
    odd.chain(even).collect()
}

/// The normals of the mesh: one for all the pairs, or one per pair.
fn set_mesh_normals(
    set: &StepVisualComplexTriangulatedSurfaceSet,
    mesh: &Triangulation,
    nb_pairs: i32,
) -> Result<(), Failure> {
    let normals = set.normals();
    let row_of = |row: i32| -> Result<Direction3, Failure> {
        Direction3::new(
            step_array2_item(&normals, row, 1, WHERE)?,
            step_array2_item(&normals, row, 2, WHERE)?,
            step_array2_item(&normals, row, 3, WHERE)?,
        )
    };
    for pair in 1..=nb_pairs {
        let row = if set.nb_normals() == 1 { 1 } else { pair };
        mesh.set_normal(pair, row_of(row)?)?;
    }
    Ok(())
}

/// `createMesh`: a triangulation from a ComplexTriangulatedSurfaceSet. The number of pairs (point,
/// normal) can exceed the points: one point may have several normals when the underlying surface is
/// not C1 continuous. The size counts all the triangles of a strip, even the degenerate.
fn create_mesh(
    set: &StepVisualComplexTriangulatedSurfaceSet,
    factor: f64,
) -> Result<Handle<Triangulation>, Failure> {
    let coordinates = set.coordinates();
    let nodes = step_live(&coordinates, WHERE)?.points();
    let (strips, fans) = (set.triangle_strips(), set.triangle_fans());
    let nb_normals = set.nb_normals();
    let nb_pairs = if nb_normals > 1 {
        set.pnmax()
    } else {
        step_live(&nodes, WHERE)?.array1().length()
    };
    let mut size = 0;
    let mut triangles = Vec::new();
    for rank in 1..=set.nb_triangle_strips() {
        let strip = indices_at(&strips, rank)?;
        size += strip.len() as i32 - 2;
        triangles.extend(strip_triangles(&strip));
    }
    for rank in 1..=set.nb_triangle_fans() {
        let fan = indices_at(&fans, rank)?;
        size += fan.len() as i32 - 2;
        triangles
            .extend((2..fan.len()).map(|index| Triangle::new(fan[0], fan[index - 1], fan[index])));
    }
    let mesh = Handle::new(Triangulation::with_sizes(
        nb_pairs,
        size,
        false,
        nb_normals > 0,
    )?);
    let live = step_live(&mesh, WHERE)?;
    for pair in 1..=nb_pairs {
        let index = if nb_normals > 1 && set.nb_pnindex() > 0 {
            set.pnindex_value(pair)?
        } else {
            pair
        };
        live.set_node(
            pair,
            Point3::from(step_array1_item(&nodes, index, WHERE)? * factor),
        )?;
    }
    for (index, triangle) in triangles.into_iter().enumerate() {
        live.set_triangle(index as i32 + 1, triangle)?;
    }
    if nb_normals > 0 {
        set_mesh_normals(set, live, nb_pairs)?;
    }
    Ok(mesh)
}

/// The annotations of a presentation: itself, or the content of a callout, up to the first annotation
/// occurrence. Sets the name of the presentation.
fn annotations_of(
    entity: &Handle<StandardTransient>,
    name: &mut Handle<HAsciiString>,
) -> Result<Vec<Handle<StepVisualStyledItem>>, Failure> {
    let mut annotations = Vec::new();
    if entity.cast::<StepVisualAnnotationOccurrence>().is_some()
        || entity
            .cast::<StepVisualTessellatedAnnotationOccurrence>()
            .is_some()
    {
        let styled = entity.cast::<StepVisualStyledItem>();
        if let Some(live) = styled.get() {
            *name = live.name();
            annotations.push(styled);
        }
    } else if let Some(callout) = entity.cast::<StepVisualDraughtingCallout>().get() {
        *name = callout.name();
        for rank in 1..=callout.nb_contents() {
            let content = callout.contents_value(rank)?;
            let occurrence = content.value().cast::<StepVisualAnnotationOccurrence>();
            if occurrence.is_some() {
                annotations.push(occurrence.cast());
                break;
            }
            let tessellated = content.tessellated_annotation_occurrence();
            if tessellated.is_some() {
                annotations.push(tessellated.cast());
            }
        }
    }
    Ok(annotations)
}

/// The placement of a repositioned tessellated set, identity when it is the standard one.
fn reposition_of(
    set: &Handle<StepVisualTessellatedGeometricSet>,
    factors: &StepDataFactors,
) -> Result<Transform3, Failure> {
    let mut transformation = Transform3::default();
    let repositioned = set.cast::<StepVisualRepositionedTessellatedGeometricSet>();
    let Some(repositioned) = repositioned.get() else {
        return Ok(transformation);
    };
    let location = StepToGeom::make_axis2_placement(&repositioned.location(), factors)?;
    let Some(location) = location.get() else {
        return Ok(transformation);
    };
    let origin = Axis3::from_axis2(gp::xoy());
    let target = Axis3::from_axis2(location.ax2()?);
    if target.location().square_distance(&origin.location()) >= precision::square_confusion()
        || !target
            .direction()
            .is_equal(&origin.direction(), precision::angular())
        || !target
            .x_direction()
            .is_equal(&origin.x_direction(), precision::angular())
        || !target
            .y_direction()
            .is_equal(&origin.y_direction(), precision::angular())
    {
        transformation.set_transformation_between(target, origin);
    }
    Ok(transformation)
}

/// One wire per curve of a tessellated curve set, into the compound.
fn add_curve_set(
    curves: &StepVisualTessellatedCurveSet,
    factor: f64,
    compound: &mut Compound,
) -> Result<(), Failure> {
    let builder = BRepBuilder::default();
    let coordinates = curves.coord_list();
    let Some(coordinates) = coordinates.get() else {
        return Ok(());
    };
    let points = coordinates.points();
    let Some(live_points) = points.get().filter(|live| live.array1().length() != 0) else {
        return Ok(());
    };
    let nb_points = live_points.array1().length();
    let strips = curves.curves();
    let Some(strips) = strips.get() else {
        return Ok(());
    };
    for rank in 0..strips.length() {
        let indices = strips
            .value(rank)
            .map_err(|error| error.into_failure(WHERE))?;
        let indices = step_live(indices, WHERE)?;
        let indices: Vec<i32> = indices.sequence(|items| items.iter().copied().collect());
        let mut wire = Wire::default();
        builder.make_wire(&mut wire);
        for pair in indices.windows(2) {
            if pair[0] > nb_points || pair[1] > nb_points {
                continue;
            }
            let first = Point3::from(step_array1_item(&points, pair[0], WHERE)? * factor);
            let second = Point3::from(step_array1_item(&points, pair[1], WHERE)? * factor);
            let maker = BRepBuilderAPIMakeEdge::from_points(first, second)?;
            if maker.is_done() {
                let edge = maker.edge()?;
                builder.add(&mut wire, &edge)?;
            }
        }
        builder.add(compound, &wire)?;
    }
    Ok(())
}

/// The shape of a tessellated annotation.
fn tessellated_annotation_shape(
    item: &StepVisualStyledItem,
    factor: f64,
    factors: &StepDataFactors,
) -> Result<Shape, Failure> {
    let set = item.item().cast::<StepVisualTessellatedGeometricSet>();
    let Some(live) = set.get() else {
        return Ok(Shape::default());
    };
    let transformation = reposition_of(&set, factors)?;
    let builder = BRepBuilder::default();
    let mut compound = Compound::default();
    builder.make_compound(&mut compound);
    let list = live.items();
    let items: Vec<Handle<StepVisualTessellatedItem>> = list
        .get()
        .map_or_else(Vec::new, |array| array.as_slice().to_vec());
    for tessellated in items {
        if let Some(curves) = tessellated.cast::<StepVisualTessellatedCurveSet>().get() {
            add_curve_set(curves, factor, &mut compound)?;
        } else if let Some(surfaces) = tessellated
            .cast::<StepVisualComplexTriangulatedSurfaceSet>()
            .get()
        {
            let mesh = create_mesh(surfaces, factor)?;
            let mut face = Face::default();
            builder.make_face_with_triangulation(&mut face, &mesh)?;
            builder.add(&mut compound, &face)?;
        }
    }
    compound.moved(&Location::new(transformation), false)
}

/// The shape of an annotation curve: from the map of the transfer, else translated now.
fn annotation_curve_shape(
    item: &StepVisualStyledItem,
    session: &SessionGraph,
) -> Result<Shape, Failure> {
    let curve = item.item();
    let shape = StepConstruct::find_shape(&session.tp, &curve)?;
    if !shape.is_null() {
        return Ok(shape);
    }
    let reader = step_live(&session.reader, WHERE)?;
    let actor = reader.actor();
    let binder =
        step_live(&actor, WHERE)?.transfer(&curve.cast(), &session.tp, &Default::default())?;
    match binder.get() {
        Some(live) if live.has_result() => Ok(TransferBRep::shape_result(&binder)),
        _ => Ok(Shape::default()),
    }
}
