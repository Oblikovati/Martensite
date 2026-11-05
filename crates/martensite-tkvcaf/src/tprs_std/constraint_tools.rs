//! `TPrsStdConstraintTools`: the `PrsDim` presentation of each `TDataXtdConstraint` type.
//!
//! Every `ComputeX` returns a new object or a null handle (the native value-returning
//! overloads start from a null object). The deprecated overloads that update an existing
//! object (`ComputeX(aConst, anAIS)`) are excluded by the design record, so the branches of the native
//! `computeXImpl` that only run on a non-null input object are not implemented.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Edge, Face, Shape};
use martensite_tkcaf::tdata_xtd::{TDataXtdConstraint, TDataXtdConstraintEnum, TDataXtdGeometry};
use martensite_tkcaf::tnaming::{TNamingNamedShape, TNamingTool};
use martensite_tkernel::standard::c_string::{FormatArg, sprintf_into};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkernel::tcollection::ExtendedString;
use martensite_tkg3d::geom::{GeomCartesianPoint, GeomGeometry, GeomLine, GeomPlane};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString, TDataStdName};
use martensite_tklcaf::tdf::{TdfAttributeState, TdfLabel, TdfReference};
use martensite_tkmath::gp::{Line3, Plane3, Point3};
use martensite_tkv3d::ais::AisInteractiveObject;
use martensite_tkv3d::prs_dim::{PrsDimRelationMembers, PrsDimRelationSource, prs_dim_units_api};

/// `TPrsStdConstraintTools`: a class of static members only.
pub struct TPrsStdConstraintTools;

/// The result of every `ComputeX`: the presentation, or null.
pub type ConstraintPresentation = Result<Handle<dyn AisInteractiveObject>, Failure>;

/// The null presentation of a constraint that cannot be shown (`NullifyAIS`).
pub(super) fn no_presentation() -> ConstraintPresentation {
    Ok(Handle::default())
}

/// The live constraint behind a handle.
pub(super) fn live_constraint(
    constraint: &Handle<TDataXtdConstraint>,
) -> Result<&TDataXtdConstraint, Failure> {
    martensite_tklcaf::live(constraint, "TPrsStdConstraintTools")
}

impl TPrsStdConstraintTools {
    /// `ComputeTextAndValue(aConst, aValue, aText, anIsAngle)`: the value of the constraint
    /// and its text (`%g` of the value in the current units, prefixed `name=` or
    /// `father.name=` when the value is captured by a named label).
    pub fn compute_text_and_value(
        constraint: &Handle<TDataXtdConstraint>,
        value: &mut f64,
        text: &mut ExtendedString,
        is_angle: bool,
    ) -> Result<(), Failure> {
        let real = live_constraint(constraint)?.get_value();
        let real = martensite_tklcaf::live(&real, "TPrsStdConstraintTools::ComputeTextAndValue")?;
        *value = real.get();
        let units = prs_dim_units_api();
        let shown = if is_angle {
            units.write().current_from_ls(value.abs(), b"PLANE ANGLE")?
        } else {
            units.write().current_from_ls(*value, b"LENGTH")?
        };
        let mut bytes = Vec::new();
        sprintf_into(&mut bytes, b"%g", &[FormatArg::Real(shown)]).map_err(io_failure)?;
        bytes.pop();
        *text = ExtendedString::from_c_string(&bytes, false);
        if real.is_captured()? {
            *text = captured_text(&real.label(), text)?;
        }
        Ok(())
    }
    /// `UpdateOnlyValue(aConst, anAIS)`: the new text of a dimension constraint's relation
    /// (nothing for a null object, a non-dimension constraint or an object that is not a
    /// `PrsDimRelation`).
    pub fn update_only_value(
        constraint: &Handle<TDataXtdConstraint>,
        ais: &Handle<dyn AisInteractiveObject>,
    ) -> Result<(), Failure> {
        if ais.is_null() || !live_constraint(constraint)?.is_dimension() {
            return Ok(());
        }
        let (mut value, mut text) = (0.0, ExtendedString::default());
        let is_angle = live_constraint(constraint)?.get_type() == TDataXtdConstraintEnum::Angle;
        Self::compute_text_and_value(constraint, &mut value, &mut text, is_angle)?;
        if let Some(relation) = ais.cast::<dyn PrsDimRelationSource>().get() {
            relation.set_text(&text);
        }
        Ok(())
    }
    /// `ComputeOthers`: no presentation.
    pub fn compute_others(_constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        no_presentation()
    }
}

/// A formatting failure as a `StandardFailure`.
fn io_failure(error: std::io::Error) -> Failure {
    martensite_tkernel::standard::exceptions::program_error(&format!(
        "TPrsStdConstraintTools::ComputeTextAndValue: Sprintf failed: {error}"
    ))
}

/// The text of a captured value: the referenced label's `name=` (`father.name=` when the
/// father is named too) before the value text; the plain text when the referenced label
/// has no name. A captured value without a `TdfReference` dereferences null natively.
fn captured_text(value_label: &TdfLabel, text: &ExtendedString) -> Result<ExtendedString, Failure> {
    let mut reference: Handle<TdfReference> = Handle::default();
    value_label.find_attribute_as(&TdfReference::get_id(), &mut reference)?;
    let referenced =
        martensite_tklcaf::live(&reference, "TPrsStdConstraintTools::ComputeTextAndValue")?.get();
    let mut name: Handle<TDataStdName> = Handle::default();
    if !referenced.find_attribute_as(&TDataStdName::get_id(), &mut name)? {
        return Ok(text.clone());
    }
    let own = martensite_tklcaf::live(&name, "TPrsStdConstraintTools")?.get();
    let mut father: Handle<TDataStdName> = Handle::default();
    let full = if referenced
        .father()
        .find_attribute_as(&TDataStdName::get_id(), &mut father)?
    {
        let father = martensite_tklcaf::live(&father, "TPrsStdConstraintTools")?.get();
        father.cat(ExtendedString::from_utf8(".")).cat(own)
    } else {
        own
    };
    Ok(full.cat(ExtendedString::from_utf8("=")).cat(text.clone()))
}

/// `GetOneShape`: the current shape of the first geometry (null without it).
pub(super) fn one_shape(constraint: &Handle<TDataXtdConstraint>) -> Result<Shape, Failure> {
    geometry_shape(constraint, 1)
}

/// The current shape of the geometry `index` (null when that geometry is null).
pub(super) fn geometry_shape(
    constraint: &Handle<TDataXtdConstraint>,
    index: i32,
) -> Result<Shape, Failure> {
    let named = live_constraint(constraint)?.get_geometry(index)?;
    if named.is_null() {
        return Ok(Shape::default());
    }
    TNamingTool::current_shape(&named)
}

/// `GetTwoShapes`.
pub(super) fn two_shapes(
    constraint: &Handle<TDataXtdConstraint>,
) -> Result<(Shape, Shape), Failure> {
    Ok((
        geometry_shape(constraint, 1)?,
        geometry_shape(constraint, 2)?,
    ))
}

/// `GetGeom`: the plane, line or point of the constraint's plane attribute (null when the
/// constraint has no plane, its named shape is empty or its label holds none of them).
pub(super) fn constraint_geometry(
    constraint: &Handle<TDataXtdConstraint>,
) -> Result<Handle<dyn GeomGeometry>, Failure> {
    let plane = live_constraint(constraint)?.get_plane();
    let Some(plane) = plane.get() else {
        return Ok(Handle::default());
    };
    let label = plane.label();
    let mut named: Handle<TNamingNamedShape> = Handle::default();
    if label.find_attribute_as(&TNamingNamedShape::get_id(), &mut named)?
        && TNamingTool::get_shape(&named)?.is_null()
    {
        return Ok(Handle::default());
    }
    let mut pln = Plane3::default();
    if TDataXtdGeometry::plane_of_label(&label, &mut pln)? {
        return Ok(Handle::new(GeomPlane::from_pln(pln)).cast());
    }
    let mut line = Line3::default();
    if TDataXtdGeometry::line_of_label(&label, &mut line)? {
        return Ok(Handle::new(GeomLine::from_lin(line)).cast());
    }
    let mut point = Point3::default();
    if TDataXtdGeometry::point_of_label(&label, &mut point)? {
        return Ok(Handle::new(GeomCartesianPoint::from_pnt(point)).cast());
    }
    Ok(Handle::default())
}

/// `down_cast<GeomPlane>(GetGeom(...))`.
pub(super) fn constraint_plane(
    constraint: &Handle<TDataXtdConstraint>,
) -> Result<Handle<GeomPlane>, Failure> {
    Ok(constraint_geometry(constraint)?.cast::<GeomPlane>())
}

/// `GetGoodShape`: an edge or a vertex stays; another shape becomes its first edge, else its
/// first vertex, else stays.
pub(super) fn good_shape(shape: &mut Shape) -> Result<(), Failure> {
    if matches!(shape.shape_type()?, ShapeEnum::Edge | ShapeEnum::Vertex) {
        return Ok(());
    }
    let edges = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    if edges.more() {
        *shape = edges.current().clone();
        return Ok(());
    }
    let vertices = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)?;
    if vertices.more() {
        *shape = vertices.current().clone();
    }
    Ok(())
}

/// `IsFace`: the shape has a face.
pub(super) fn is_face(shape: &Shape) -> Result<bool, Failure> {
    Ok(Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?.more())
}

/// `GetFace`: the first face of the shape (null without one).
pub(super) fn first_face(shape: &Shape) -> Result<Face, Failure> {
    let faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    if !faces.more() {
        return Ok(Face::default());
    }
    topods::face(faces.current())
}

/// `GetEdge`: the first edge of the shape (null without one).
pub(super) fn first_edge(shape: &Shape) -> Result<Edge, Failure> {
    let edges = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    if !edges.more() {
        return Ok(Edge::default());
    }
    topods::edge(edges.current())
}

/// `FindExternalShape`: 1 when the first geometry is not below the constraint's label, else
/// 2 when the second is not, else 0. Natively a null geometry is dereferenced.
pub(super) fn external_shape(constraint: &Handle<TDataXtdConstraint>) -> Result<i32, Failure> {
    let value = live_constraint(constraint)?;
    let label = value.label();
    for index in 1..=2 {
        let named = value.get_geometry(index)?;
        let named = martensite_tklcaf::live(&named, "TPrsStdConstraintTools::FindExternalShape")?;
        if !named.label().is_descendant(&label) {
            return Ok(index);
        }
    }
    Ok(0)
}
