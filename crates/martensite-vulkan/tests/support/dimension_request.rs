//! A dimension built as the interactive viewer's dimension command builds it: the measured
//! objects (shapes, or displayed points and circles), a fresh dimension aspect with the
//! requested text, font and units, then the dimension parameters, displayed under a name.
use crate::case_view::viewer_session::ViewerSession;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis3, Plane3, Point3, xoy, yoz, zox};
use martensite_tkv3d::ais::{
    AisCircle, AisInteractiveObject, AisPoint, AisShape, AisShapeParameters as _,
};
use martensite_tkv3d::prs_dim::{
    PrsDimAngleDimension, PrsDimDiameterDimension, PrsDimDimensionMembers as _,
    PrsDimDimensionSource, PrsDimLengthDimension, PrsDimRadiusDimension,
};
use martensite_tkv3d::prs3d::Prs3dDimensionAspect;
use std::error::Error;

use super::dimension_params::DimensionStyle;

/// What the dimension measures.
#[allow(dead_code)] // Each host builds only the kinds its regressions measure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum DimensionKind {
    Length,
    Angle,
    Radius,
    Diameter,
}

impl DimensionKind {
    /// The command flag naming the kind, as its messages print it; `Length.flag() == "-length"`.
    pub(crate) fn flag(self) -> &'static str {
        match self {
            Self::Length => "-length",
            Self::Angle => "-angle",
            Self::Radius => "-radius",
            Self::Diameter => "-diameter",
        }
    }
}

/// One measured operand: a modelling shape, or an object displayed under a name.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Measured<'a> {
    Shape(&'a Shape),
    Object(&'a str),
}

/// A standard working plane through the origin.
#[allow(dead_code)] // Each host uses only the planes its regressions name.
#[derive(Clone, Copy, Debug)]
pub(crate) enum WorkingPlane {
    Xoy,
    Yoz,
    Zox,
}

/// The dimension command's options; `None` keeps the default.
#[derive(Clone, Debug)]
pub(crate) struct DimensionRequest<'a> {
    pub kind: DimensionKind,
    pub shapes: Vec<Measured<'a>>,
    pub plane: Option<WorkingPlane>,
    pub font: Option<&'a str>,
    pub text_3d: Option<bool>,
    pub value: Option<f64>,
    pub model_units: Option<&'a str>,
    pub display_units: Option<&'a str>,
    pub flyout: Option<f64>,
    /// The aspect options shared with the dimension parameter command.
    pub style: DimensionStyle,
}

impl<'a> DimensionRequest<'a> {
    /// A dimension of `kind` over `shapes` with every option at its default.
    pub(crate) fn new(kind: DimensionKind, shapes: &[Measured<'a>]) -> Self {
        Self {
            kind,
            shapes: shapes.to_vec(),
            plane: None,
            font: None,
            text_3d: None,
            value: None,
            model_units: None,
            display_units: None,
            flyout: None,
            style: DimensionStyle::default(),
        }
    }

    fn aspect(&self) -> Result<Prs3dDimensionAspect, Failure> {
        let aspect = Prs3dDimensionAspect::new()?;
        if let Some(text_3d) = self.text_3d {
            aspect.make_text_3d(text_3d);
        }
        if let (Some(font), Some(text)) = (self.font, aspect.text_aspect().get()) {
            text.set_font(font);
        }
        self.style.apply(&aspect);
        Ok(aspect)
    }

    fn working_plane(&self) -> Option<Plane3> {
        let frame = match self.plane? {
            WorkingPlane::Xoy => xoy(),
            WorkingPlane::Yoz => yoz(),
            WorkingPlane::Zox => zox(),
        };
        Some(Plane3::from_position(Axis3::from_axis2(frame)))
    }

    fn apply_parameters(&self, dim: &dyn PrsDimDimensionSource) {
        if let Some(flyout) = self.flyout {
            dim.set_flyout(flyout);
        }
        if let Some(value) = self.value {
            dim.set_custom_value(value);
        }
        if let Some(units) = self.model_units {
            dim.set_model_units(&AsciiString::from(units));
        }
        if let Some(units) = self.display_units {
            dim.set_display_units(&AsciiString::from(units));
        }
    }
}

/// The objects of the operands: a new shape presentation for a shape, else the named one.
fn operands(
    session: &ViewerSession,
    shapes: &[Measured],
) -> Result<Vec<Handle<dyn AisInteractiveObject>>, Box<dyn Error>> {
    shapes
        .iter()
        .map(|measured| match measured {
            Measured::Shape(shape) => Ok(AisShape::new(shape).cast()),
            Measured::Object(name) => session
                .object(name)
                .cloned()
                .ok_or_else(|| format!("Error: shape with name '{name}' is not found.").into()),
        })
        .collect()
}

fn point_of(object: &Handle<dyn AisInteractiveObject>) -> Option<Point3> {
    object
        .cast::<AisPoint>()
        .get()
        .map(|p| p.component().get().map(|c| c.pnt()).unwrap_or_default())
}

fn shape_of(object: &Handle<dyn AisInteractiveObject>) -> Result<Shape, Failure> {
    if let Some(point) = object.cast::<AisPoint>().get() {
        return Ok(point.vertex()?.into_shape());
    }
    Ok(object
        .cast::<AisShape>()
        .get()
        .map(|s| s.shape())
        .unwrap_or_default())
}

type Built = Result<Handle<dyn PrsDimDimensionSource>, Box<dyn Error>>;

/// A length between two operands, or of one edge on the working plane; a vertex operand
/// moves the plane onto that vertex.
fn length(objects: &[Handle<dyn AisInteractiveObject>], plane: Option<Plane3>) -> Built {
    if let [edge] = objects {
        let shape = shape_of(edge)?;
        let plane = plane.ok_or("Error: can not build dimension without working plane")?;
        return Ok(PrsDimLengthDimension::from_edge(&topods::edge(&shape)?, &plane)?.cast());
    }
    let [first, second] = objects else {
        return Err("Error: wrong number of shapes to build dimension".into());
    };
    let (first, second) = (shape_of(first)?, shape_of(second)?);
    let dim = PrsDimLengthDimension::new();
    let value = dim.get().ok_or("a new length dimension")?;
    let is_vertex = |shape: &Shape| shape.shape_type().ok() == Some(ShapeEnum::Vertex);
    if let Some(mut plane) = plane {
        let vertex = [&first, &second].into_iter().find(|s| is_vertex(s));
        if let Some(vertex) = vertex {
            plane.set_location(BRepTool::pnt(&topods::vertex(vertex)?)?);
        }
        value.set_custom_plane(&plane);
    } else if is_vertex(&first) && is_vertex(&second) {
        return Err("Error: can not build dimension without working plane".into());
    }
    value.set_measured_shapes(&first, &second)?;
    Ok(dim.cast())
}

/// An angle of a cone face, between two edges, or at the middle of three points.
fn angle(objects: &[Handle<dyn AisInteractiveObject>]) -> Built {
    match objects {
        [face] => Ok(PrsDimAngleDimension::from_cone(&topods::face(&shape_of(face)?)?)?.cast()),
        [a, b] => Ok(PrsDimAngleDimension::from_edges(
            &topods::edge(&shape_of(a)?)?,
            &topods::edge(&shape_of(b)?)?,
        )?
        .cast()),
        [a, b, c] => {
            let points = [a, b, c].map(point_of);
            let [Some(a), Some(b), Some(c)] = points else {
                return Err("Error: wrong number of shapes to build dimension".into());
            };
            Ok(PrsDimAngleDimension::from_points(&a, &b, &c)?.cast())
        }
        _ => Err("Error: wrong number of shapes to build dimension".into()),
    }
}

/// A radius of a displayed circle (optionally anchored at a displayed point) or of a shape.
fn radius(objects: &[Handle<dyn AisInteractiveObject>]) -> Built {
    let anchor = objects.iter().find_map(point_of);
    let rest: Vec<_> = objects.iter().filter(|o| point_of(o).is_none()).collect();
    let [object] = rest.as_slice() else {
        return Err("Syntax error: wrong number of shapes to build dimension".into());
    };
    if let Some(circle) = object.cast::<AisCircle>().get() {
        let circ = circle.circle().get().ok_or("a circle")?.circ()?;
        let dim = match anchor {
            Some(anchor) => PrsDimRadiusDimension::from_circle_anchor(&circ, &anchor)?,
            None => PrsDimRadiusDimension::from_circle(&circ)?,
        };
        return Ok(dim.cast());
    }
    let shape = shape_of(object)?;
    let dim = PrsDimRadiusDimension::from_shape(&shape)?;
    if let (Some(anchor), Some(value)) = (anchor, dim.get()) {
        value.set_measured_shape(&shape, Some(anchor))?;
    }
    Ok(dim.cast())
}

/// A diameter of a displayed circle or of a shape.
fn diameter(objects: &[Handle<dyn AisInteractiveObject>]) -> Built {
    let [object] = objects else {
        return Err("Error: wrong number of shapes to build dimension".into());
    };
    if let Some(circle) = object.cast::<AisCircle>().get() {
        let circ = circle.circle().get().ok_or("a circle")?.circ()?;
        return Ok(PrsDimDiameterDimension::from_circle(&circ)?.cast());
    }
    Ok(PrsDimDiameterDimension::from_shape(&shape_of(object)?)?.cast())
}

/// Build the dimension of `request`, check it, and display it as `name`.
/// Example: `display_dimension(&mut session, "dim1", &request)?`.
pub(crate) fn display_dimension(
    session: &mut ViewerSession,
    name: &str,
    request: &DimensionRequest,
) -> Result<(), Box<dyn Error>> {
    let aspect = request.aspect()?;
    let objects = operands(session, &request.shapes)?;
    let dim = build_requested_dimension(request, &objects)?;
    let value = dim.get().ok_or("a built dimension")?;
    require_valid_dimension(value, request.kind)?;
    value.set_dimension_aspect(&Handle::new(aspect));
    request.apply_parameters(value);
    session.display_object(name, dim.cast())?;
    Ok(())
}

fn build_requested_dimension(
    request: &DimensionRequest,
    objects: &[Handle<dyn AisInteractiveObject>],
) -> Built {
    match request.kind {
        DimensionKind::Length => length(objects, request.working_plane()),
        DimensionKind::Angle => angle(objects),
        DimensionKind::Radius => radius(objects),
        DimensionKind::Diameter => diameter(objects),
    }
}

fn require_valid_dimension(
    value: &dyn PrsDimDimensionSource,
    kind: DimensionKind,
) -> Result<(), Box<dyn Error>> {
    if !value.is_valid() {
        return Err(format!(
            "Error: dimension geometry is invalid, {} dimension can't be built on input shapes.",
            kind.flag()
        )
        .into());
    }
    Ok(())
}

#[cfg(test)]
mod dimension_cpu_contract {
    include!("../support/dimension_request_contract.rs");
}

#[cfg(test)]
mod dimension_kind_command_contract {
    use super::DimensionKind;

    #[test]
    fn dimension_kind_flags_match_the_command_grammar() {
        let cases = [
            (DimensionKind::Length, "-length"),
            (DimensionKind::Angle, "-angle"),
            (DimensionKind::Radius, "-radius"),
            (DimensionKind::Diameter, "-diameter"),
        ];
        for (kind, flag) in cases {
            assert_eq!(kind.flag(), flag);
        }
    }
}
