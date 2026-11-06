//! The presentations of GD&T read by `STEPCAFControlReader`.
use super::*;

/// `readPMIPresentation`: polyline or tessellated presentation of an Annotation_Curve_Occurrence or a
/// Draughting_Callout; true when it has a shape. The box is of the last annotation only.
pub(in crate::stepcaf_control) fn read_pmi_presentation(
    entity: &Handle<StandardTransient>,
    session: &SessionGraph,
    factor: f64,
    presentation: &mut Shape,
    name: &mut Handle<HAsciiString>,
    bounds: &mut BoundingBox3,
    factors: &StepDataFactors,
) -> Result<bool, Failure> {
    if entity.is_null() {
        return Ok(false);
    }
    let annotations = annotations_of(entity, name)?;
    if annotations.is_empty() {
        return Ok(false);
    }
    let builder = BRepBuilder::default();
    let mut result = Compound::default();
    builder.make_compound(&mut result);
    let mut last_bounds = BoundingBox3::default();
    let mut nb_shapes = 0;
    for (index, annotation) in annotations.iter().enumerate() {
        let live = step_live(annotation, WHERE)?;
        let shape = if annotation
            .cast::<StepVisualAnnotationOccurrence>()
            .is_some()
        {
            annotation_curve_shape(live, session)?
        } else {
            tessellated_annotation_shape(live, factor, factors)?
        };
        if !shape.is_null() {
            nb_shapes += 1;
            builder.add(&mut result, &shape)?;
            if index + 1 == annotations.len() {
                BRepBndLib::add_close(&shape, &mut last_bounds)?;
            }
        }
    }
    *presentation = result.into_shape();
    *bounds = last_bounds;
    Ok(nb_shapes > 0)
}

/// `readAnnotationPlane`: the axes of an annotation plane; false without them.
pub(in crate::stepcaf_control) fn read_annotation_plane(
    plane: &Handle<StepVisualAnnotationPlane>,
    axes: &mut Axis2,
    factors: &StepDataFactors,
) -> Result<bool, Failure> {
    let Some(plane) = plane.get() else {
        return Ok(false);
    };
    let item = plane.item();
    if item.is_null() {
        return Ok(false);
    }
    // retrieve axes from AnnotationPlane
    let mut placement = Handle::<StepGeomAxis2Placement3d>::default();
    if let Some(live) = item.cast::<StepGeomPlane>().get() {
        placement = live.position();
    } else if let Some(planar) = item.cast::<StepVisualPlanarBox>().get() {
        placement = planar.placement().axis2_placement3d();
    }
    if placement.is_null() {
        return Ok(false);
    }
    let axis = StepToGeom::make_axis2_placement(&placement, factors)?;
    *axes = step_live(&axis, WHERE)?.ax2()?;
    Ok(true)
}

/// `set_plane` of a dimension, a datum or a tolerance.
fn set_object_plane(object: &Handle<StandardTransient>, plane: &Axis2) {
    if let Some(live) = object.cast::<XCAFDimTolObjectsDimensionObject>().get() {
        live.set_plane(plane);
    } else if let Some(live) = object.cast::<XCAFDimTolObjectsDatumObject>().get() {
        live.set_plane(plane);
    } else if let Some(live) = object.cast::<XCAFDimTolObjectsGeomToleranceObject>().get() {
        live.set_plane(plane);
    }
}

/// `set_point_text_attach` and `set_presentation` of a dimension, a datum or a tolerance.
fn set_object_presentation(
    object: &Handle<StandardTransient>,
    point: &Point3,
    presentation: &Shape,
    name: &Handle<HAsciiString>,
) {
    if let Some(live) = object.cast::<XCAFDimTolObjectsDimensionObject>().get() {
        live.set_point_text_attach(point);
        live.set_presentation(presentation, name);
    } else if let Some(live) = object.cast::<XCAFDimTolObjectsDatumObject>().get() {
        live.set_point_text_attach(point);
        live.set_presentation(presentation, name);
    } else if let Some(live) = object.cast::<XCAFDimTolObjectsGeomToleranceObject>().get() {
        live.set_point_text_attach(point);
        live.set_presentation(presentation, name);
    }
}

/// The units of a representation, through a new read actor.
pub(in crate::stepcaf_control) fn factors_of_representation(
    representation: &Handle<StepReprRepresentation>,
    tp: &Handle<TransferTransientProcess>,
    model: &Handle<dyn InterfaceInterfaceModel>,
    factors: &StepDataFactors,
) -> Result<StepDataFactors, Failure> {
    XSAlgoShapeProcessor::prepare_for_transfer();
    let actor = StepControlActorRead::new(model);
    let mut local = *factors;
    actor.prepare_units(representation, tp, &mut local)?;
    Ok(local)
}

/// The model of the process.
pub(in crate::stepcaf_control) fn model_of(
    tp: &Handle<TransferTransientProcess>,
) -> Result<Handle<dyn InterfaceInterfaceModel>, Failure> {
    Ok(step_live(tp, WHERE)?.model())
}

/// The association of a GDT with its presentation. The name is lowered in the entity. Its
/// test of the name never fails: `Search` is -1 or a rank, never 0, so the first association stays
/// whatever its name; kept.
fn association_of_gdt(
    session: &SessionGraph,
    gdt: &Handle<StandardTransient>,
) -> Result<Handle<StepAP242DraughtingModelItemAssociation>, Failure> {
    let association: Handle<StepAP242DraughtingModelItemAssociation> =
        first_of(&session.sharings(gdt)?);
    if let Some(live) = association.get() {
        step_live(&live.name(), WHERE)?.lower_case();
    }
    Ok(association)
}

/// `readAnnotation`: the annotation plane and position of a Dimension, Geometric_Tolerance,
/// Datum_Feature or Placed_Datum_Target_Feature.
pub(in crate::stepcaf_control) fn read_annotation(
    session: &SessionGraph,
    gdt: &Handle<StandardTransient>,
    object: &Handle<StandardTransient>,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    if gdt.is_null() || object.is_null() {
        return Ok(());
    }
    // find the proper DraughtingModelItemAssociation
    let association = association_of_gdt(session, gdt)?;
    let Some(association) = association
        .get()
        .filter(|live| live.nb_identified_item() != 0)
    else {
        return Ok(());
    };
    // calculate units
    let local = factors_of_representation(
        &association.used_representation(),
        &session.tp,
        &model_of(&session.tp)?,
        factors,
    )?;
    let factor = local.length_factor();
    // retrieve AnnotationPlane
    let item: Handle<StandardTransient> = association.identified_item_value(1)?.cast();
    if item.is_null() {
        return Ok(());
    }
    let mut plane_axes = Axis2::default();
    let plane: Handle<StepVisualAnnotationPlane> = first_of(&session.sharings(&item)?);
    let has_plane = read_annotation_plane(&plane, &mut plane_axes, &local)?;
    // set plane axes to XCAF
    if has_plane {
        set_object_plane(object, &plane_axes);
    }
    // Retrieve presentation
    let mut bounds = BoundingBox3::default();
    let mut presentation = Shape::default();
    let mut name = Handle::default();
    if !read_pmi_presentation(
        &item,
        session,
        factor,
        &mut presentation,
        &mut name,
        &mut bounds,
        &local,
    )? {
        return Ok(());
    }
    // if Annotation plane location inside bounding box set it to text position
    // else set the center of bounding box to text position 0027372
    let text_point = if bounds.is_void() {
        plane_axes.location()
    } else if has_plane && !bounds.is_out_point(plane_axes.location()) {
        plane_axes.location()
    } else {
        let [x_min, y_min, z_min, x_max, y_max, z_max] = bounds.get()?;
        Point3::new(
            (x_min + x_max) * 0.5,
            (y_min + y_max) * 0.5,
            (z_min + z_max) * 0.5,
        )
    };
    // set point to XCAF
    set_object_presentation(object, &text_point, &presentation, &name);
    Ok(())
}

/// The connection of a GISU: a cartesian point, or the location of a placement.
struct ConnectionPoint {
    axes: Axis2,
    is_point: bool,
    name: Handle<HAsciiString>,
}

/// `retrieveConnectionPointFromGISU`: `None` without a usable item. The axes keep the last ones when
/// the item is a point.
fn connection_point(
    gisu: &Handle<StepAP242GeometricItemSpecificUsage>,
    factor: f64,
    axes: Axis2,
) -> Result<Option<ConnectionPoint>, Failure> {
    let Some(gisu) = gisu.get().filter(|live| live.nb_identified_item() != 0) else {
        return Ok(None);
    };
    let item = gisu.identified_item_value(1)?;
    if let Some(point) = item.cast::<StepGeomCartesianPoint>().get() {
        let mut moved = axes;
        moved.set_location(Point3::new(
            point.coordinates_value(1)? * factor,
            point.coordinates_value(2)? * factor,
            point.coordinates_value(3)? * factor,
        ));
        return Ok(Some(ConnectionPoint {
            axes: moved,
            is_point: true,
            name: point.name(),
        }));
    }
    // try Axis2Placement3d.location instead of CartesianPoint
    let placement = item.cast::<StepGeomAxis2Placement3d>();
    let Some(live) = placement.get() else {
        return Ok(None);
    };
    let axis = StepToGeom::make_axis2_placement(&placement, &StepDataFactors::default())?;
    Ok(Some(ConnectionPoint {
        axes: step_live(&axis, WHERE)?.ax2()?,
        is_point: false,
        name: live.name(),
    }))
}

/// The GISU of a derived shape aspect, null without one.
fn gisu_of_aspect(
    session: &SessionGraph,
    aspect: &Handle<StepReprDerivedShapeAspect>,
) -> Result<Handle<StepAP242GeometricItemSpecificUsage>, Failure> {
    if aspect.is_null() {
        return Ok(Handle::default());
    }
    Ok(first_of(&session.sharings(&aspect.cast())?))
}

/// `readConnectionPoints`: the connection points of a dimension.
pub(in crate::stepcaf_control) fn read_connection_points(
    session: &SessionGraph,
    gdt: &Handle<StandardTransient>,
    object: &Handle<XCAFDimTolObjectsDimensionObject>,
    factors: &StepDataFactors,
) -> Result<(), Failure> {
    let Some(live_object) = object.get().filter(|_| gdt.is_some()) else {
        return Ok(());
    };
    let mut factor = 1.;
    let characteristic: Handle<StepShapeDimensionalCharacteristicRepresentation> =
        first_of(&session.sharings(gdt)?);
    if let Some(characteristic) = characteristic.get() {
        let representation = characteristic.representation();
        if representation.is_some() {
            let local = factors_of_representation(
                &representation.cast(),
                &session.tp,
                &step_live(&session.reader, WHERE)?.model(),
                factors,
            )?;
            factor = local.length_factor();
        }
    }
    if let Some(size) = gdt.cast::<StepShapeDimensionalSize>().get() {
        // retrieve derived geometry
        let aspect = size.applies_to().cast::<StepReprDerivedShapeAspect>();
        if aspect.is_null() {
            return Ok(());
        }
        let gisu = gisu_of_aspect(session, &aspect)?;
        if let Some(found) = connection_point(&gisu, factor, Axis2::default())? {
            if found.is_point {
                live_object.set_point(&found.axes.location());
            } else {
                live_object.set_connection_axis(&found.axes);
            }
            live_object.set_connection_name(&found.name);
        }
    } else if let Some(location) = gdt.cast::<StepShapeDimensionalLocation>().get() {
        // retrieve derived geometry
        let first_aspect = location
            .relating_shape_aspect()
            .cast::<StepReprDerivedShapeAspect>();
        let second_aspect = location
            .related_shape_aspect()
            .cast::<StepReprDerivedShapeAspect>();
        if first_aspect.is_null() && second_aspect.is_null() {
            return Ok(());
        }
        let first_gisu = gisu_of_aspect(session, &first_aspect)?;
        let second_gisu = gisu_of_aspect(session, &second_aspect)?;
        let mut axes = Axis2::default();
        // first point
        if let Some(found) = connection_point(&first_gisu, factor, axes)? {
            axes = found.axes;
            if found.is_point {
                live_object.set_point(&axes.location());
            } else {
                live_object.set_connection_axis(&axes);
            }
            live_object.set_connection_name(&found.name);
        }
        // second point
        if let Some(found) = connection_point(&second_gisu, factor, axes)? {
            if found.is_point {
                live_object.set_point2(&found.axes.location());
            } else {
                live_object.set_connection_axis2(&found.axes);
            }
            live_object.set_connection_name2(&found.name);
        }
    }
    Ok(())
}
