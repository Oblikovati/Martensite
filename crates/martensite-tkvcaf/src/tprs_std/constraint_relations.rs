//! The `PrsDimRelation` presentations of the geometric constraints: perpendicular,
//! parallel, symmetry, midpoint, tangent, concentric, equal distance, equal radius, fix
//! and coincident.
use super::constraint_tools::{
    ConstraintPresentation, TPrsStdConstraintTools, constraint_geometry, constraint_plane,
    geometry_shape, good_shape, live_constraint, no_presentation, one_shape, two_shapes,
};
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorCurve;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkcaf::tdata_xtd::TDataXtdConstraint;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, exceptions};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom::{GeomCircle, GeomPlane};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::CurveType;
use martensite_tkmath::gp::{Axis3, Point3};
use martensite_tkv3d::prs_dim::{
    PrsDimConcentricRelation, PrsDimEqualDistanceRelation, PrsDimEqualRadiusRelation,
    PrsDimFixRelation, PrsDimIdenticRelation, PrsDimMidPointRelation, PrsDimParallelRelation,
    PrsDimPerpendicularRelation, PrsDimRelationMembers, PrsDimSymmetricRelation,
    PrsDimTangentRelation,
};

/// The planar shapes of a two-geometry relation: at least two geometries, planar, both
/// shapes and a plane; `None` when the native code nullifies the object.
fn planar_pair(
    constraint: &Handle<TDataXtdConstraint>,
) -> Result<Option<(Shape, Shape, Handle<GeomPlane>)>, Failure> {
    let value = live_constraint(constraint)?;
    if value.nb_geometries() < 2 || !value.is_planar() {
        return Ok(None);
    }
    let (mut first, mut second) = two_shapes(constraint)?;
    let plane = constraint_plane(constraint)?;
    if first.is_null() || second.is_null() {
        return Ok(None);
    }
    good_shape(&mut first)?;
    good_shape(&mut second)?;
    if plane.is_null() {
        return Ok(None);
    }
    Ok(Some((first, second, plane)))
}

/// The three shapes (the third is the tool) and the plane of a three-geometry relation.
type PlanarTriple = ([Shape; 3], Handle<GeomPlane>);

/// The planar shapes of a three-geometry relation (the third is the tool).
fn planar_triple(constraint: &Handle<TDataXtdConstraint>) -> Result<Option<PlanarTriple>, Failure> {
    let value = live_constraint(constraint)?;
    if value.nb_geometries() < 3 || !value.is_planar() {
        return Ok(None);
    }
    let (first, second) = two_shapes(constraint)?;
    let mut shapes = [first, second, geometry_shape(constraint, 3)?];
    let plane = constraint_plane(constraint)?;
    if shapes.iter().any(Shape::is_null) {
        return Ok(None);
    }
    for shape in &mut shapes {
        good_shape(shape)?;
    }
    if plane.is_null() {
        return Ok(None);
    }
    Ok(Some((shapes, plane)))
}

impl TPrsStdConstraintTools {
    /// `ComputePerpendicular`: a `PrsDimPerpendicularRelation` (on the constraint's plane
    /// when it is planar).
    pub fn compute_perpendicular(
        constraint: &Handle<TDataXtdConstraint>,
    ) -> ConstraintPresentation {
        let value = live_constraint(constraint)?;
        if value.nb_geometries() < 2 {
            return no_presentation();
        }
        let planar = value.is_planar();
        let (mut first, mut second) = two_shapes(constraint)?;
        let geometry = if planar {
            constraint_geometry(constraint)?
        } else {
            Handle::default()
        };
        if first.is_null() || second.is_null() {
            return no_presentation();
        }
        good_shape(&mut first)?;
        good_shape(&mut second)?;
        let relation = PrsDimPerpendicularRelation::new(&first, &second, None);
        if planar {
            let plane = geometry.cast::<GeomPlane>();
            if plane.is_null() {
                return no_presentation();
            }
            relation.set_plane(&plane);
        }
        Ok(relation.cast())
    }
    /// `ComputeParallel`: a `PrsDimParallelRelation` on the constraint's plane.
    pub fn compute_parallel(constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        let Some((first, second, plane)) = planar_pair(constraint)? else {
            return no_presentation();
        };
        Ok(PrsDimParallelRelation::new(&first, &second, &plane).cast())
    }
    /// `ComputeSymmetry`: a `PrsDimSymmetricRelation` with the third geometry as tool.
    pub fn compute_symmetry(constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        let Some(([first, second, tool], plane)) = planar_triple(constraint)? else {
            return no_presentation();
        };
        Ok(PrsDimSymmetricRelation::new(&tool, &first, &second, &plane)?.cast())
    }
    /// `ComputeMidPoint`: a `PrsDimMidPointRelation` with the third geometry as tool.
    pub fn compute_mid_point(constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        let Some(([first, second, tool], plane)) = planar_triple(constraint)? else {
            return no_presentation();
        };
        Ok(PrsDimMidPointRelation::new(&tool, &first, &second, &plane)?.cast())
    }
    /// `ComputeTangent`: a `PrsDimTangentRelation` with the arrow size 10000000
    /// (jfa 9/10/2000).
    pub fn compute_tangent(constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        let Some((first, second, plane)) = planar_pair(constraint)? else {
            return no_presentation();
        };
        let relation = PrsDimTangentRelation::new(&first, &second, &plane, 0);
        relation.set_arrow_size(10_000_000.0);
        Ok(relation.cast())
    }
    /// `ComputeConcentric`: a `PrsDimConcentricRelation` when one shape is an edge;
    /// `StandardProgramError` with fewer than two geometries.
    pub fn compute_concentric(constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        let value = live_constraint(constraint)?;
        if value.nb_geometries() < 2 {
            return Err(exceptions::program_error(
                "TPrsStdConstraintTools::ComputeConcentric: at least 2 constraints are needed",
            ));
        }
        if !value.is_planar() {
            return no_presentation();
        }
        let (mut first, mut second) = two_shapes(constraint)?;
        let plane = constraint_plane(constraint)?;
        if first.is_null() || second.is_null() {
            return no_presentation();
        }
        good_shape(&mut first)?;
        good_shape(&mut second)?;
        let has_edge =
            first.shape_type()? == ShapeEnum::Edge || second.shape_type()? == ShapeEnum::Edge;
        if !has_edge || plane.is_null() {
            return no_presentation();
        }
        Ok(PrsDimConcentricRelation::new(&first, &second, &plane)?.cast())
    }
    /// `ComputeFix`: a `PrsDimFixRelation` of the first geometry on the plane.
    pub fn compute_fix(constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        let value = live_constraint(constraint)?;
        if value.nb_geometries() < 1 || !value.is_planar() {
            return no_presentation();
        }
        let mut shape = one_shape(constraint)?;
        if shape.is_null() {
            return no_presentation();
        }
        good_shape(&mut shape)?;
        let plane = constraint_plane(constraint)?;
        if plane.is_null() {
            return no_presentation();
        }
        Ok(PrsDimFixRelation::new(&shape, &plane, None).cast())
    }
    /// `ComputeCoincident`: a `PrsDimIdenticRelation` on the plane.
    pub fn compute_coincident(constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        let Some((first, second, plane)) = planar_pair(constraint)? else {
            return no_presentation();
        };
        Ok(PrsDimIdenticRelation::new(&first, &second, &plane).cast())
    }
}

/// Equal distance and equal radius.
impl TPrsStdConstraintTools {
    /// `ComputeEqualDistance`: a `PrsDimEqualDistanceRelation` of two correct pairs on the
    /// constraint's plane.
    pub fn compute_equal_distance(
        constraint: &Handle<TDataXtdConstraint>,
    ) -> ConstraintPresentation {
        if live_constraint(constraint)?.nb_geometries() < 4 {
            return no_presentation();
        }
        let (first, second) = two_shapes(constraint)?;
        let mut shapes = [
            first,
            second,
            geometry_shape(constraint, 3)?,
            geometry_shape(constraint, 4)?,
        ];
        let geometry = constraint_geometry(constraint)?;
        if shapes.iter().any(Shape::is_null) {
            return no_presentation();
        }
        for shape in &mut shapes {
            good_shape(shape)?;
        }
        if !check_shapes_pair(&shapes[0], &shapes[1])?
            || !check_shapes_pair(&shapes[2], &shapes[3])?
        {
            return no_presentation();
        }
        let plane = if live_constraint(constraint)?.is_planar() {
            geometry.cast::<GeomPlane>()
        } else {
            Handle::default()
        };
        if plane.is_null() {
            return no_presentation();
        }
        let [a, b, c, d] = &shapes;
        Ok(PrsDimEqualDistanceRelation::new(a, b, c, d, &plane).cast())
    }
    /// `ComputeEqualRadius`: a `PrsDimEqualRadiusRelation` of two edges, on the
    /// constraint's plane or else on the common plane of the two circles.
    pub fn compute_equal_radius(constraint: &Handle<TDataXtdConstraint>) -> ConstraintPresentation {
        let value = live_constraint(constraint)?;
        if value.nb_geometries() < 2 {
            return no_presentation();
        }
        let (mut first, mut second) = two_shapes(constraint)?;
        let geometry = constraint_geometry(constraint)?;
        if first.is_null() || second.is_null() {
            return no_presentation();
        }
        good_shape(&mut first)?;
        good_shape(&mut second)?;
        let (edge1, edge2) = (topods::edge(&first)?, topods::edge(&second)?);
        let mut plane = if value.is_planar() {
            geometry.cast::<GeomPlane>()
        } else {
            Handle::default()
        };
        if plane.is_null() {
            match common_circle_plane(&edge1, &edge2)? {
                Some(common) => plane = common,
                None => return no_presentation(),
            }
        }
        Ok(PrsDimEqualRadiusRelation::new(&edge1, &edge2, &plane).cast())
    }
}

/// The plane of the (untransformed) circle of an edge: `GeomPlane(gpAx3(circle.Position()))`.
/// Natively an edge that is not on a circle dereferences null.
fn circle_plane(edge: &topods::Edge) -> Result<Handle<GeomPlane>, Failure> {
    let adaptor = BRepAdaptorCurve::with_edge(edge)?;
    let curve = adaptor.transformed_curve().curve();
    let curve =
        martensite_tklcaf::live(&curve, "TPrsStdConstraintTools::ComputeEqualRadius")?.curve();
    let circle = curve.cast::<GeomCircle>();
    let circle =
        martensite_tklcaf::live(&circle, "TPrsStdConstraintTools::ComputeEqualRadius")?.circ()?;
    Ok(Handle::new(GeomPlane::new(Axis3::from_axis2(
        circle.position(),
    ))))
}

/// The common plane of the circles of two edges: the second circle's plane when both
/// planes have the same distance and parallel normals.
fn common_circle_plane(
    first: &topods::Edge,
    second: &topods::Edge,
) -> Result<Option<Handle<GeomPlane>>, Failure> {
    let (plane1, plane2) = (circle_plane(first)?, circle_plane(second)?);
    let (p1, p2) = (
        martensite_tklcaf::live(&plane1, "TPrsStdConstraintTools")?,
        martensite_tklcaf::live(&plane2, "TPrsStdConstraintTools")?,
    );
    let (d1, d2) = (p1.coefficients()?[3], p2.coefficients()?[3]);
    let (dir1, dir2) = (
        p1.pln().position().direction(),
        p2.pln().position().direction(),
    );
    if (d1 - d2).abs() < precision::confusion() && dir1.is_parallel(dir2, precision::confusion()) {
        return Ok(Some(plane2));
    }
    Ok(None)
}

/// `CheckShapesPair`: two parallel lines, two concentric circles, or a vertex at the
/// center of a circle (a vertex with another curve passes; two vertices pass).
fn check_shapes_pair(first: &Shape, second: &Shape) -> Result<bool, Failure> {
    let (type1, type2) = (first.shape_type()?, second.shape_type()?);
    if type1 == ShapeEnum::Edge && type2 == ShapeEnum::Edge {
        return check_edge_pair(first, second);
    }
    if type1 == ShapeEnum::Vertex && type2 == ShapeEnum::Vertex {
        return Ok(true);
    }
    let (point, edge) = if type1 == ShapeEnum::Vertex {
        (
            BRepTool::pnt(&topods::vertex(first)?)?,
            topods::edge(second)?,
        )
    } else {
        (
            BRepTool::pnt(&topods::vertex(second)?)?,
            topods::edge(first)?,
        )
    };
    let curve = BRepAdaptorCurve::with_edge(&edge)?;
    if curve.get_type()? == CurveType::Circle {
        let center: Point3 = curve.circle()?.location();
        return Ok(center.is_equal(&point, precision::confusion()));
    }
    Ok(true)
}

/// The edge-edge case of `CheckShapesPair`.
fn check_edge_pair(first: &Shape, second: &Shape) -> Result<bool, Failure> {
    let curve1 = BRepAdaptorCurve::with_edge(&topods::edge(first)?)?;
    let curve2 = BRepAdaptorCurve::with_edge(&topods::edge(second)?)?;
    let (kind1, kind2) = (curve1.get_type()?, curve2.get_type()?);
    if kind1 == CurveType::Line && kind2 == CurveType::Line {
        let (dir1, dir2) = (curve1.line()?.direction(), curve2.line()?.direction());
        return Ok(dir1.is_parallel(dir2, precision::confusion()));
    }
    if kind1 == CurveType::Circle && kind2 == CurveType::Circle {
        let (c1, c2) = (curve1.circle()?.location(), curve2.circle()?.location());
        return Ok(c1.is_equal(&c2, precision::confusion()));
    }
    Ok(false)
}
