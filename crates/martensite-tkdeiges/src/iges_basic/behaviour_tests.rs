//! What the sample round can't see: derived values against closed forms, and the record of a file.
use crate::iges_basic::{IgesBasicGroup, IgesBasicToolGroup};
use crate::iges_data::{IgesDataIgesEntity, iges_new_array1, iges_new_array2};
use crate::iges_geom::{
    IgesGeomBSplineSurface, IgesGeomCircularArc, IgesGeomConicArc, IgesGeomCopiousData,
    IgesGeomLine, IgesGeomPlane, IgesGeomToolConicArc, IgesGeomToolTransformationMatrix,
    IgesGeomTransformationMatrix,
};
use crate::iges_solid::{IgesSolidBlock, IgesSolidBooleanTree, IgesSolidShell};
use crate::iges_test_samples::{iges_round_read, iges_written_file};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::{Point3, Xy, Xyz};
use martensite_tkxsbase::interface::InterfaceModelEntities;

const CLOSE: f64 = 1e-12;

fn bspline_surface(bounds: [f64; 4]) -> Result<Handle<IgesGeomBSplineSurface>, Failure> {
    let weights = iges_new_array2::<f64>(0, 1, 0, 1)?;
    crate::iges_basic::entity_shape::iges_array2_fill(&weights, 1.0, "bspline_surface")?;
    let surface = Handle::new(IgesGeomBSplineSurface::new());
    surface.init(
        1,
        1,
        1,
        1,
        false,
        false,
        true,
        false,
        false,
        &iges_new_array1::<f64>(-1, 2)?,
        &iges_new_array1::<f64>(-1, 2)?,
        &weights,
        &iges_new_array2::<Xyz>(0, 1, 0, 1)?,
        bounds[0],
        bounds[1],
        bounds[2],
        bounds[3],
    )?;
    Ok(surface)
}

#[test]
fn a_surface_read_then_written_keeps_its_four_bounds() -> Result<(), Failure> {
    let surface = bspline_surface([0.25, 0.75, 2.5, 8.5])?;
    assert!(String::from_utf8_lossy(&iges_written_file(&surface)?).contains("0.25,0.75,2.5,8.5;"));
    let model = iges_round_read(&surface)?;
    let read = model.value(1)?.cast::<IgesGeomBSplineSurface>();
    let live = read.get().expect("the model holds the surface");
    assert_eq!(
        (live.u_min(), live.u_max(), live.v_min(), live.v_max()),
        (0.25, 0.75, 2.5, 8.5)
    );
    assert!(String::from_utf8_lossy(&iges_written_file(&read)?).contains("0.25,0.75,2.5,8.5;"));
    Ok(())
}

#[test]
fn a_circular_arc_gives_its_radius_and_its_angle() -> Result<(), Failure> {
    let arc = IgesGeomCircularArc::new();
    arc.init(1.0, Xy::new(1.0, 1.0), Xy::new(3.0, 1.0), Xy::new(1.0, 3.0));
    assert!((arc.radius() - 2.0).abs() < CLOSE);
    assert!((arc.angle()? - std::f64::consts::FRAC_PI_2).abs() < CLOSE);
    assert!(!arc.is_closed());
    let center = arc.transformed_center()?;
    assert_eq!((center.x(), center.y(), center.z()), (1.0, 1.0, 1.0));
    Ok(())
}

#[test]
fn a_whole_circle_has_a_full_turn_of_angle() -> Result<(), Failure> {
    let arc = IgesGeomCircularArc::new();
    arc.init(0.0, Xy::new(0.0, 0.0), Xy::new(1.0, 0.0), Xy::new(1.0, 0.0));
    assert!(arc.is_closed());
    assert!((arc.angle()? - 2.0 * std::f64::consts::PI).abs() < CLOSE);
    Ok(())
}

fn conic(coefficients: [f64; 6]) -> IgesGeomConicArc {
    let [a, b, c, d, e, f] = coefficients;
    let arc = IgesGeomConicArc::new();
    arc.init(a, b, c, d, e, f, 0.0, Xy::new(1.0, 0.0), Xy::new(0.0, 1.0));
    arc
}

#[test]
fn the_form_of_a_conic_follows_its_equation() {
    assert_eq!(
        conic([1.0, 0.0, 1.0, 0.0, 0.0, -1.0]).computed_form_number(),
        1
    );
    assert_eq!(
        conic([1.0, 0.0, -1.0, 0.0, 0.0, -1.0]).computed_form_number(),
        2
    );
    assert_eq!(
        conic([0.0, 0.0, 1.0, -1.0, 0.0, 0.0]).computed_form_number(),
        3
    );
    assert_eq!(
        conic([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]).computed_form_number(),
        0
    );
}

#[test]
fn an_ellipse_gives_its_center_axis_and_radii() -> Result<(), Failure> {
    // x^2/4 + y^2 = 1, written with the factor that keeps the coefficients plain.
    let arc = conic([1.0, 0.0, 4.0, 0.0, 0.0, -4.0]);
    assert!(arc.is_from_ellipse());
    let (mut xcen, mut ycen, mut xax, mut yax, mut rmin, mut rmax) = (9.0, 9.0, 9.0, 9.0, 9.0, 9.0);
    arc.computed_definition(
        &mut xcen, &mut ycen, &mut xax, &mut yax, &mut rmin, &mut rmax,
    );
    assert!(xcen.abs() < CLOSE && ycen.abs() < CLOSE);
    assert!((rmin.min(rmax) - 1.0).abs() < CLOSE, "{rmin} {rmax}");
    assert!((rmin.max(rmax) - 2.0).abs() < CLOSE, "{rmin} {rmax}");
    Ok(())
}

#[test]
fn correcting_a_conic_sets_the_form_its_equation_has() -> Result<(), Failure> {
    let arc = Handle::new(conic([1.0, 0.0, 1.0, 0.0, 0.0, -1.0]));
    arc.init_type_and_form(104, 3);
    assert!(IgesGeomToolConicArc::new().own_correct(&arc)?);
    assert_eq!(arc.form_number(), 1);
    assert!(!IgesGeomToolConicArc::new().own_correct(&arc)?);
    Ok(())
}

#[test]
fn copious_data_counts_tuples_by_its_data_type() -> Result<(), Failure> {
    let data = iges_new_array1::<f64>(1, 12)?;
    for rank in 1..=12 {
        crate::iges_data::iges_array1_set(&data, rank, f64::from(rank), "copious")?;
    }
    let pairs = IgesGeomCopiousData::new();
    pairs.init(1, 7.0, &data)?;
    assert_eq!(pairs.nb_points()?, 6);
    assert_eq!(pairs.point(2)?, Point3::new(3.0, 4.0, 7.0));
    let triples = IgesGeomCopiousData::new();
    triples.init(2, 0.0, &data)?;
    assert_eq!(triples.nb_points()?, 4);
    assert_eq!(triples.point(2)?, Point3::new(4.0, 5.0, 6.0));
    let sextuples = IgesGeomCopiousData::new();
    sextuples.init(3, 0.0, &data)?;
    assert_eq!(sextuples.nb_points()?, 2);
    let vector = sextuples.vector(2)?;
    assert_eq!((vector.x(), vector.y(), vector.z()), (10.0, 11.0, 12.0));
    assert_eq!(sextuples.data(2, 3)?, 9.0);
    assert!(sextuples.point(3).is_err());
    Ok(())
}

#[test]
fn copious_data_refuses_null_data_and_a_shifted_list() -> Result<(), Failure> {
    assert!(
        IgesGeomCopiousData::new()
            .init(1, 0.0, &Handle::default())
            .is_err()
    );
    let shifted = iges_new_array1::<f64>(0, 3)?;
    assert!(IgesGeomCopiousData::new().init(1, 0.0, &shifted).is_err());
    Ok(())
}

#[test]
fn a_form_number_out_of_its_range_is_refused() -> Result<(), Failure> {
    assert!(
        IgesGeomTransformationMatrix::new()
            .set_form_number(5)
            .is_err()
    );
    assert!(
        bspline_surface([0.0, 1.0, 0.0, 1.0])?
            .set_form_number(10)
            .is_err()
    );
    assert!(
        bspline_surface([0.0, 1.0, 0.0, 1.0])?
            .set_form_number(9)
            .is_ok()
    );
    Ok(())
}

fn matrix(rows: [[f64; 4]; 3]) -> Result<Handle<IgesGeomTransformationMatrix>, Failure> {
    let values = iges_new_array2::<f64>(1, 3, 1, 4)?;
    for (row, cells) in (1..).zip(rows) {
        for (column, cell) in (1..).zip(cells) {
            crate::iges_data::iges_array2_set(&values, row, column, cell, "matrix")?;
        }
    }
    let result = Handle::new(IgesGeomTransformationMatrix::new());
    result.init(&values)?;
    Ok(result)
}

const SHIFT_AND_MIRROR: [[f64; 4]; 3] = [
    [-1.0, 0.0, 0.0, 10.0],
    [0.0, 1.0, 0.0, 20.0],
    [0.0, 0.0, 1.0, 30.0],
];

#[test]
fn a_matrix_refuses_other_dimensions() -> Result<(), Failure> {
    let wrong = iges_new_array2::<f64>(1, 3, 1, 3)?;
    assert!(IgesGeomTransformationMatrix::new().init(&wrong).is_err());
    assert!(
        IgesGeomTransformationMatrix::new()
            .init(&Handle::default())
            .is_err()
    );
    Ok(())
}

#[test]
fn correcting_a_matrix_sets_the_form_of_its_handedness() -> Result<(), Failure> {
    let negative = matrix(SHIFT_AND_MIRROR)?;
    assert_eq!(negative.form_number(), 0);
    assert!(IgesGeomToolTransformationMatrix::new().own_correct(&negative)?);
    assert_eq!(negative.form_number(), 1);
    Ok(())
}

#[test]
fn located_members_go_through_the_matrix_of_the_entity() -> Result<(), Failure> {
    let line = IgesGeomLine::new();
    line.init(Xyz::new(1.0, 2.0, 3.0), Xyz::new(4.0, 5.0, 6.0));
    assert_eq!(line.transformed_start_point()?, Point3::new(1.0, 2.0, 3.0));
    let transform: Handle<dyn crate::iges_data::IgesDataTransfEntity> =
        matrix(SHIFT_AND_MIRROR)?.cast();
    line.init_transf(&transform);
    assert_eq!(
        line.transformed_start_point()?,
        Point3::new(9.0, 22.0, 33.0)
    );
    assert_eq!(line.start_point(), Point3::new(1.0, 2.0, 3.0));

    let block = IgesSolidBlock::new();
    block.init(
        Xyz::new(1.0, 2.0, 3.0),
        Xyz::new(0.0, 0.0, 0.0),
        Xyz::new(1.0, 0.0, 0.0),
        Xyz::new(0.0, 0.0, 1.0),
    );
    assert_eq!(
        (block.x_length(), block.y_length(), block.z_length()),
        (1.0, 2.0, 3.0)
    );
    block.init_transf(&transform);
    let axis = Xyz::from(block.transformed_x_axis()?);
    assert_eq!((axis.x(), axis.y(), axis.z()), (-1.0, 0.0, 0.0));
    let corner = block.transformed_corner()?;
    assert_eq!((corner.x(), corner.y(), corner.z()), (10.0, 20.0, 30.0));
    // The second axis is normal to the two stored ones.
    let normal = Xyz::from(block.y_axis()?);
    assert_eq!((normal.x(), normal.y(), normal.z()), (0.0, -1.0, 0.0));
    Ok(())
}

#[test]
fn the_group_modes_move_between_their_forms() {
    let group = IgesBasicGroup::new();
    assert_eq!(group.form_number(), 1);
    assert!(!group.is_ordered() && !group.is_without_back_p());
    group.set_ordered(true);
    assert_eq!(group.form_number(), 14);
    group.set_without_back_p(true);
    assert_eq!(group.form_number(), 15);
    assert!(group.is_ordered() && group.is_without_back_p());
    group.set_ordered(false);
    assert_eq!(group.form_number(), 7);
    group.set_without_back_p(false);
    assert_eq!(group.form_number(), 1);
    assert!(group.set_user(5000, 0).is_err());
    assert!(group.set_user(5001, 0).is_ok());
    assert_eq!((group.type_number(), group.form_number()), (5001, 0));
}

#[test]
fn a_group_keeps_its_entities_when_it_grows_or_shrinks() -> Result<(), Failure> {
    let group = IgesBasicGroup::with_nb(2)?;
    let line: Handle<IgesDataIgesEntity> = Handle::new(IgesGeomLine::new()).cast();
    group.set_value(1, &line)?;
    group.set_nb(4)?;
    assert_eq!(group.nb_entities(), 4);
    assert!(group.entity(1)? == line);
    group.set_nb(1)?;
    assert_eq!(group.nb_entities(), 1);
    assert!(group.entity(1)? == line);
    group.set_nb(0)?;
    assert_eq!(group.nb_entities(), 1);
    Ok(())
}

#[test]
fn correcting_a_group_drops_its_null_entities() -> Result<(), Failure> {
    let group = Handle::new(IgesBasicGroup::with_nb(3)?);
    let line = Handle::new(IgesGeomLine::new());
    line.init(Xyz::new(0.0, 0.0, 0.0), Xyz::new(1.0, 0.0, 0.0));
    group.set_value(2, &line.cast())?;
    assert!(IgesBasicToolGroup::new().own_correct(&group)?);
    assert_eq!(group.nb_entities(), 1);
    assert!(!IgesBasicToolGroup::new().own_correct(&group)?);
    Ok(())
}

#[test]
fn a_tree_tells_operands_from_operations() -> Result<(), Failure> {
    let operands = iges_new_array1::<Handle<IgesDataIgesEntity>>(1, 3)?;
    let line: Handle<IgesDataIgesEntity> = Handle::new(IgesGeomLine::new()).cast();
    crate::iges_data::iges_array1_set(&operands, 1, line.clone(), "tree")?;
    crate::iges_data::iges_array1_set(&operands, 2, line, "tree")?;
    let operations = iges_new_array1::<i32>(1, 3)?;
    crate::iges_data::iges_array1_set(&operations, 3, 2, "tree")?;
    let tree = IgesSolidBooleanTree::new();
    tree.init(&operands, &operations)?;
    assert!(tree.is_operand(1)? && !tree.is_operand(3)?);
    assert_eq!((tree.operation(1)?, tree.operation(3)?), (0, 2));
    let shorter = iges_new_array1::<i32>(1, 2)?;
    assert!(
        IgesSolidBooleanTree::new()
            .init(&operands, &shorter)
            .is_err()
    );
    Ok(())
}

#[test]
fn a_shell_form_follows_its_closure() {
    let shell = IgesSolidShell::new();
    shell.set_closed(true);
    assert!(shell.is_closed());
    shell.set_closed(false);
    assert_eq!(shell.form_number(), 2);
}

#[test]
fn a_plane_with_a_negative_form_is_a_hole_only_with_a_curve() {
    let plane = IgesGeomPlane::new();
    plane.set_form_number(-5);
    assert_eq!(plane.form_number(), -1);
    assert!(!plane.has_bounding_curve_hole());
    plane.set_form_number(5);
    assert_eq!(plane.form_number(), 1);
}
