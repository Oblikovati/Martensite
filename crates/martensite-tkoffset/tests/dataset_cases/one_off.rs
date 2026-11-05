//! What one or a few scripts run and no table form covers: each function is that script's own block.
use crate::script::{OffsetScript, command_stopped};
use crate::support::{
    checkshape::checkshape_named,
    nbshapes::{Counting, NbCount, checknbshapes_named, nbshapes},
    properties::{Prop, props},
    section_check::checksection_count,
    tolerance_check::{checkreal, tolerance_report},
};
use crate::wire_end::exploded;
use martensite_tkbo::bop_algo::BOPAlgoArgumentAnalyzer;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkfeat::brep_feat::BRepFeatMakeCylindricalHole;
use martensite_tkg3d::{
    geom::{
        GeomCylindricalSurface, GeomOffsetSurface, GeomPlane, GeomRectangularTrimmedSurface,
        GeomSphericalSurface, GeomSurface,
    },
    top_abs::{Orientation, ShapeEnum},
};
use martensite_tkmath::{
    bnd::BoundingBox3 as Bounds,
    gp::{Axis1, Direction3, Point3},
};
use martensite_tktopalgo::{
    brep_bnd_lib::BRepBndLib, brep_extrema::BRepExtremaDistShapeShape,
    brep_lib::find_valid_range_of_edge,
};

/// The block every `bug745_*` repeats after its `mkoffset`.
pub fn wire_report(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let session = script.session();
    let counts = nbshapes(session, "result", Counting::Located);
    let (vertices, wires) = (counts[0], counts[2]);
    println!("Result shape result contains {vertices} vertexes");
    if vertices == 0 {
        println!("Faulty : Result shape is NULL shape");
        return Ok(());
    }
    if wires <= 1 {
        return Ok(());
    }
    let parts = exploded(script, "result", ShapeEnum::Wire);
    println!(
        "Faulty : Result shape result is COMPOUND and contains {} wires",
        parts.len()
    );
    for (name, _) in parts {
        checkshape_named(script.session(), &name);
        if checksection_count(script.session(), &name) != 0 {
            println!("Faulty : Result shape is UNclosed wire !!! ");
        } else {
            println!("OK: Result shape is CLOSED wire !!! ");
        }
    }
    Ok(())
}

/// `regexp {Mass: ...} [lprops r3] dummy len3`, then its limit.
pub fn punctual_section(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let session = script.session();
    let Some(shape) = session.shape("r3").cloned() else {
        return Err(command_stopped("r3 is not defined"));
    };
    if props(Prop::Length, &shape, None) > 1.0e-12 {
        println!("Error: the length of punctual section is not null");
    }
    Ok(())
}

/// `subshape b f 1`: the shape's first face, under `b_1`.
pub fn first_face(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    exploded(script, "b", ShapeEnum::Face);
    println!("b_1");
    Ok(())
}

/// `bounding name -save xmin yy zz xmax yy zz -dump`: the command's report, the two `x` kept.
fn bounds_report(script: &mut OffsetScript<'_>, name: &str, low: &'static str, high: &'static str) {
    let shape = script.shape(name);
    let mut bounds = Bounds::default();
    BRepBndLib::add(&shape, &mut bounds, true).expect("bounds of a solid");
    let [xmin, ymin, zmin, xmax, ymax, zmax] = bounds.get().expect("the solid has bounds");
    println!("Axes-aligned bounding box");
    println!("X-range: {xmin} {xmax}");
    println!("Y-range: {ymin} {ymax}");
    println!("Z-range: {zmin} {zmax}");
    script.values.keep(low, xmin.to_string());
    script.values.keep(high, xmax.to_string());
}

pub fn bounds_of_thick(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    bounds_report(script, "t", "xt1", "xt2");
    Ok(())
}

pub fn bounds_of_hole(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    bounds_report(script, "h", "xh1", "xh2");
    Ok(())
}

/// `set ori [lindex [dtyp t] 2]; puts "Orientation of thick solid is $ori"`.
pub fn thick_orientation(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let word = match script.shape("t").orientation() {
        Orientation::Forward => "FORWARD",
        Orientation::Reversed => "REVERSED",
        Orientation::Internal => "INTERNAL",
        Orientation::External => "EXTERNAL",
    };
    println!("Orientation of thick solid is {word}");
    Ok(())
}

/// `hole h t 110 50 50 1 0 0 10`, which the script renames `result`.
pub fn cylindrical_hole(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let solid = script.required("t")?;
    let axis = Axis1::new(
        Point3::new(110., 50., 50.),
        Direction3::from_vector(martensite_tkmath::gp::Vector3::new(1., 0., 0.))?,
    );
    let mut hole = BRepFeatMakeCylindricalHole::new();
    hole.init(&solid, axis);
    hole.perform(10.)?;
    hole.build()?;
    let built = hole.shape().clone();
    script.set_shape("h", built);
    Ok(())
}

/// `set dx [dval xmax-xmin]` of both, and the script's comparison.
pub fn thickness_kept(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let read = |script: &OffsetScript<'_>, low, high| -> f64 {
        let value = |name| {
            script
                .values
                .text(name)
                .parse::<f64>()
                .expect("a kept number")
        };
        value(high) - value(low)
    };
    let (thick, holed) = (read(script, "xt1", "xt2"), read(script, "xh1", "xh2"));
    if (holed - thick).abs() > 1.0e-5 {
        println!("Thickness of solid = {thick}");
        println!("Thickness of solid with hole = {holed}");
        println!("Invalid result of making a hole");
        println!("Faulty 6334");
    } else {
        println!("OK 6334");
    }
    Ok(())
}

/// `set info [sprops result]`, the `regexp` over its centre of gravity and the script's check.
pub fn gravity_center(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let shape = script.required("result")?;
    props(Prop::Area, &shape, None);
    let center = crate::support::properties::center_of_area(&shape);
    let shown = |value: f64| crate::support::properties::general_six(value);
    println!("X={}", shown(center.x()));
    println!("Y={}", shown(center.y()));
    let z = shown(center.z());
    println!("Z={z}");
    checkreal("Z", &z, "94.9994", 0.1, 0.1);
    Ok(())
}

/// `regexp {Tolerance MAX=(...)} [tolerance result] full toler`, then `checkreal`.
pub fn agt007_tolerance(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let toler = tolerance_report(script.session(), "result");
    checkreal("MAXTOL", &toler, "3.0e-007", 0.0, 0.1);
    Ok(())
}

/// `validrange` of each edge of `result`, and the report of one without a range.
pub fn micro_edges(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    for (name, part) in exploded(script, "result", ShapeEnum::Edge) {
        let range = find_valid_range_of_edge(&topods::edge(&part)?)?;
        if range.is_valid {
            let (u1, u2) = (range.first.unwrap_or(0.), range.last.unwrap_or(0.));
            println!("{} {}", general_seventeen(u1), general_seventeen(u2));
            continue;
        }
        println!("edge has no valid range");
        let length = props(Prop::Length, &part, None);
        let tolerance = tolerance_report(script.session(), &name);
        println!(
            "ERROR: Micro edge is produced. Edge {name} (length={}, tolerance={tolerance}) is too small",
            crate::support::properties::general_six(length)
        );
    }
    Ok(())
}

fn general_seventeen(value: f64) -> String {
    martensite_tkernel::storage::format_general(value, 17)
}

/// The two generated-history loops of `C4`.
pub fn c4_history(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    use crate::steps::{HistoryStep, More};
    for (kind, prefix, expected) in [
        (ShapeEnum::Edge, "ge_", NbCount::Faces(1)),
        (ShapeEnum::Vertex, "gv_", NbCount::Edges(1)),
    ] {
        for (name, _) in exploded(script, "s_7", kind) {
            let result: &'static str = Box::leak(format!("{prefix}{name}").into_boxed_str());
            let source: &'static str = Box::leak(name.into_boxed_str());
            let query = HistoryStep::Generated(result, "h", source);
            crate::more_steps::apply(script, &More::History(query))?;
            checknbshapes_named(
                script.session(),
                result,
                Counting::Located,
                &[expected],
                "History information",
            );
        }
    }
    Ok(())
}

/// `set info [dump s1]`: the dump names the surface, which is read here from the surface itself.
pub fn spherical_s1(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let surface = script
        .surface("s1")
        .ok_or_else(|| command_stopped("s1 is not a surface"))?;
    let trimmed = surface.cast::<GeomRectangularTrimmedSurface>();
    let basis = trimmed.get().map_or(surface.clone(), |t| t.basis_surface());
    if basis.cast::<GeomSphericalSurface>().is_null() {
        println!("Faulty: The basic surface is not spherical surface.");
    } else {
        println!("OK: The basic surface is spherical surface.");
    }
    Ok(())
}

/// `errorsweep`, and the script's reading of its number.
pub fn sweep_error(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let sweep = script.commands.sweep.as_ref();
    let error = sweep.map_or(0., |sweep| sweep.error_on_surface());
    println!("Tolerance on surfaces = {error}");
    if error != 0. {
        println!("Faulty: tolerance reached on resulting surfaces is wrong");
    }
    Ok(())
}

/// `[regexp {degenerated} [dump name]]`, `mkcurve` failing on it, for `r_2` and `r_4`.
pub fn degenerated_edges(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    for name in ["r_2", "r_4"] {
        let edge = topods::edge(&script.required(name)?)?;
        if BRepTool::degenerated(&edge)? {
            println!("OK: {name} is a degenerated edge.");
            if BRepTool::curve(&edge).is_ok_and(|(curve, _, _)| !curve.is_null()) {
                println!("Faulty: {name} has 3d curve");
            } else {
                println!("OK: {name} has no 3d curve.");
            }
        } else {
            println!("Faulty: {name} is not degenerated edge.");
        }
    }
    Ok(())
}

/// The loop over `result`'s faces, reading what `dump` names from each surface itself.
pub fn plane_or_cylinder(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let holds = |surface: &Handle<dyn GeomSurface>| -> (bool, bool) {
        let offset = !surface.cast::<GeomOffsetSurface>().is_null();
        let wanted = !surface.cast::<GeomPlane>().is_null()
            || !surface.cast::<GeomCylindricalSurface>().is_null();
        (offset, wanted)
    };
    for (index, (_, part)) in exploded(script, "result", ShapeEnum::Face)
        .into_iter()
        .enumerate()
    {
        let surface = BRepTool::surface(&topods::face(&part)?)?;
        let (offset, wanted) = holds(&surface);
        let i = index + 1;
        if offset {
            println!("Error: surface {i} is offset, should be plane or cylinder");
        }
        if !wanted {
            println!("Error: surface {i} is not plane or cylinder");
        }
    }
    Ok(())
}

/// `distmini dd ww_1_N rrN; regexp .. [dump dd_val] .. dist`, for the three vertices.
pub fn generated_edges_touch(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    for index in 1..=3 {
        let source = script.required(&format!("ww_1_{index}"))?;
        let edge = script.required(&format!("rr{index}"))?;
        let distance = BRepExtremaDistShapeShape::with_shapes(&source, &edge, Default::default())?;
        println!("\"distmini\" command returns:");
        println!("dd_val dd ");
        println!("Output is complete.");
        if distance.value()? > 1.0e-7 {
            println!("Error: wrong generated list of edges");
        }
    }
    Ok(())
}

/// The two `draft` calls under `catch`, each read for its own word.
pub fn draft_of_sphere(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    use crate::extra_steps::{Extra, apply};
    let first = apply(
        script,
        &Extra::Draft("r", "sh", [0., 0., 1.], 10., "shape", -1),
    );
    let first_text = first.as_ref().err().map_or(String::new(), |failure| {
        format!(
            "An exception was caught {}: {}",
            failure.exception_type(),
            String::from_utf8_lossy(failure.what())
        )
    });
    if !first_text.is_empty() {
        println!("{first_text}");
    }
    // `draft` writes which draft it makes before it builds.
    let second = apply(
        script,
        &Extra::Draft("result", "sh_1", [0., 0., 1.], 10., "shape", -1),
    );
    // Its stop word is neither a shape nor a surface nor a length: the command returns 1, silently.
    if let Err(failure) = &second
        && !failure.is_kind(crate::script::COMMAND_RETURNED_ONE)
    {
        println!(
            "An exception was caught {}: {}",
            failure.exception_type(),
            String::from_utf8_lossy(failure.what())
        );
    }
    if first_text.contains("Standard_ConstructionError:") {
        println!("Output is as expected");
    } else {
        println!("Error : not suitablle exception");
    }
    println!("Output is as expected");
    Ok(())
}

/// `set Log [bopargcheck Thickness001 #F]`: the continuity line of its report.
pub fn continuity_report(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let shape = script.required("Thickness001")?;
    let mut checker = BOPAlgoArgumentAnalyzer::new();
    checker.set_shape1(&shape);
    *checker.continuity_mode() = true;
    checker.perform();
    if checker.has_faulty() {
        println!("Error: MakeThickSolid creates C0 continuous edges");
    } else {
        println!("OK: MakeThickSolid don't creates C0 continuous edges");
    }
    Ok(())
}

/// `set info [intersect res curve surface]`, the script's three `regexp`s on the answer.
pub fn two_intersections(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    let curve = script
        .curve("r3")
        .ok_or_else(|| command_stopped("r3 is not a curve"))?;
    let surface = script
        .surface("ss")
        .ok_or_else(|| command_stopped("ss is not a surface"))?;
    let points = martensite_tkgeomalgo::geom_api::IntCS::with_curve(&curve, &surface)?;
    println!("   Points: ");
    let count = points.nb_points()?;
    let names: Vec<String> = (1..=count).map(|i| format!("res_{i}")).collect();
    println!("{} ", names.join(" "));
    if count != 2 {
        println!("Error : Wrong number of points of intersection. Should contain two points");
    } else {
        println!("OK: Number of points of intersection is valid");
    }
    Ok(())
}

/// `bug26077_2`'s checks of each argument and of the two fuses, in the script's words.
pub fn loft_ellipsoid_fuses(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    use crate::conditions::argument_check;
    use crate::support::common_operation::{performed_direct, report_result_alerts};
    use martensite_tkbo::bop_algo::Operation;
    const VALID: &str = "Shape(s) seem(s) to be valid for BOP.";
    let check = |script: &mut OffsetScript<'_>, name: &str| {
        if argument_check(&script.shape(name)) != VALID {
            println!("ERROR: Problematic shape in 26077: Shape {name} is not valid for BOP.");
        }
    };
    let fuse =
        |script: &mut OffsetScript<'_>, result: &str, a: &str, b: &str| -> Result<(), Failure> {
            let (first, second) = (script.required(a)?, script.required(b)?);
            let operation = performed_direct(&first, &second, Operation::Fuse, 1.0e-7);
            report_result_alerts(operation.options());
            let label = format!("Operation bfuse (bfuse {result} {a} {b})");
            if operation.options().has_errors() {
                println!("ERROR: 26077 is reproduced. {label} has been finished with KO status.");
            } else {
                println!("26077: {label} has been finished with OK status.");
                script.set_shape(result, operation.shape().clone());
            }
            Ok(())
        };
    check(script, "Loft");
    check(script, "Ellipsoid");
    pin_c0_cases(script, "Ellipsoid", (1, 1), 10205.7);
    fuse(script, "Fusion", "Loft", "Ellipsoid")?;
    check(script, "Extrude_Ellipse");
    check(script, "Fusion");
    pin_c0_cases(script, "Fusion", (1, 1), 10501.8);
    fuse(script, "Fusion002", "Extrude_Ellipse", "Fusion")
}

/// The word "Faulties" names no fault, so the C0 cases the reference's `bopargcheck name #F` lists are held
/// too, as `(edges, faces)`, with its `vprops` volume. Anything else is an error the TODOs do not forgive.
fn pin_c0_cases(script: &mut OffsetScript<'_>, name: &str, expected: (usize, usize), volume: f64) {
    use martensite_tkbo::bop_algo::CheckStatus;
    let mut checker = BOPAlgoArgumentAnalyzer::new();
    checker.options_mut().set_fuzzy_value(0.0);
    let shape = script.shape(name);
    checker.set_shape1(&shape);
    *checker.argument_type_mode() = true;
    *checker.self_inter_mode() = true;
    *checker.small_edge_mode() = true;
    *checker.rebuild_face_mode() = true;
    *checker.continuity_mode() = true;
    *checker.curve_on_surface_mode() = true;
    checker.perform();
    let (mut edges, mut faces, mut others) = (0, 0, 0);
    for result in checker.check_result().iter() {
        let faulty = result.faulty_shapes1().iter().next();
        match (
            result.check_status(),
            faulty.map(|shape| shape.shape_type()),
        ) {
            (CheckStatus::GeomAbsC0, Some(Ok(ShapeEnum::Edge))) => edges += 1,
            (CheckStatus::GeomAbsC0, Some(Ok(ShapeEnum::Face))) => faces += 1,
            _ => others += 1,
        }
    }
    if (edges, faces) != expected || others != 0 {
        println!(
            "Error : {name} has {edges} C0 edges, {faces} C0 faces and {others} other faults; reference has {expected:?} and none"
        );
    }
    let mut properties = martensite_tkgeombase::gprop::GProps::default();
    let found = martensite_tktopalgo::brep_gprop::BRepGProp::volume_properties_eps(
        &shape,
        &mut properties,
        1.0,
        false,
        false,
    );
    found.expect("volume of a built solid");
    let measured = crate::support::properties::printed_mass(properties.mass());
    if measured != volume {
        println!("Error : {name} has volume {measured}; reference has {volume}");
    }
}

/// `bug77_2`'s five pipes at the reference's area and first inertia term. The script's TODO forgives any area, so
/// only this tells the reference's spine from one reversed by mistake: `reverse` leaves the wire REVERSED, as restored,
/// and flipping it gave 1.05641e+07 for the Frenet pipes.
pub fn pipe_modes_pin(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    use crate::support::properties::general_six;
    use martensite_tkgeombase::gprop::GProps;
    use martensite_tktopalgo::brep_gprop::BRepGProp;
    const REFERENCE: [(&str, &str, &str); 5] = [
        ("result_0", "1.12049e+07", "1.05079e+10"),
        ("result_1", "1.08e+07", "5.30447e+12"),
        ("result_1_a", "1.08e+07", "5.30447e+12"),
        ("result_2", "1.12049e+07", "1.05079e+10"),
        ("result_2_a", "1.12049e+07", "1.05079e+10"),
    ];
    for (name, area, inertia) in REFERENCE {
        let mut properties = GProps::default();
        BRepGProp::surface_properties_eps(&script.shape(name), &mut properties, 1.0e-4, false)?;
        let found = (
            general_six(properties.mass()),
            general_six(properties.matrix_of_inertia().value(1, 2)?),
        );
        if (found.0.as_str(), found.1.as_str()) != (area, inertia) {
            println!("Error : {name} has {found:?}; reference has {area} and {inertia}");
        }
    }
    Ok(())
}

/// `bug32447_4`'s generated face as the reference builds it. The script's 1% cannot tell which spine it swept:
/// the last word of `convert c1 c1` selects `RationalC1`, and the default gave 690.469 here. The surface's
/// degrees, poles and knots, and the reported area and error, are held at the reference's values.
pub fn swept_face_pin(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    use martensite_tkg3d::geom::GeomBSplineSurface;
    use martensite_tkgeombase::gprop::GProps;
    use martensite_tktopalgo::brep_gprop::BRepGProp;
    let shape = script.shape("r1");
    let surface = BRepTool::surface(&topods::face(&shape)?)?;
    let spline = Handle::<GeomBSplineSurface>::down_cast(&surface);
    let spline = spline
        .get()
        .ok_or_else(|| command_stopped("r1's surface is not a B-spline"))?;
    let shape_of = (
        (spline.u_degree(), spline.v_degree()),
        (spline.nb_u_poles(), spline.nb_v_poles()),
        (spline.nb_u_knots(), spline.nb_v_knots()),
    );
    if shape_of != ((6, 4), (6, 13), (2, 4)) {
        println!("Error : r1's surface is {shape_of:?}; reference has ((6, 4), (6, 13), (2, 4))");
    }
    let mut properties = GProps::default();
    let error = BRepGProp::surface_properties_eps(&shape, &mut properties, 1.0e-4, false)?;
    let area = crate::support::properties::general_six(properties.mass());
    let error = crate::support::properties::general_six(error);
    if (area.as_str(), error.as_str()) != ("691.348", "5.3394e-05") {
        println!(
            "Error : r1 has area {area} and error {error}; reference has 691.348 and 5.3394e-05"
        );
    }
    Ok(())
}

/// The tail of `bug27090`: the sixth solid's shells, and the point's class when it has two.
pub fn sixth_solid_shells(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    use martensite_tkbrep::topods::ShapeIterator;
    let solid = script.shape("v_6");
    let mut count = 0;
    let mut children = ShapeIterator::new(&solid, true, true);
    let mut first = None;
    while children.more() {
        first.get_or_insert(children.value()?.clone());
        count += 1;
        children.next();
    }
    if count <= 1 {
        println!("Info: test is not actual for the current version");
        return Ok(());
    }
    let shell = first.expect("more than one child");
    let mut one = martensite_tkbrep::topods::Solid::default();
    let builder = martensite_tkbrep::brep::BRepBuilder::default();
    builder.make_solid(&mut one);
    let mut one = one.into_shape();
    builder.add(&mut one, &shell)?;
    let at = Point3::new(
        -131.81074615766306,
        -874.99999999999977,
        -2443.8619433375443,
    );
    let mut classifier =
        martensite_tktopalgo::brep_class3d::BRepClass3dSolidClassifier::with_shape(&one)?;
    classifier.perform(&at, 1.0e-7)?;
    if classifier.state() == martensite_tkg3d::top_abs::State::Out {
        println!("Info: point is classified OUT, it is OK");
    } else {
        println!(
            "Error: point is classified as {:?}, but must be OUT",
            classifier.state()
        );
    }
    Ok(())
}

/// `baddcompound a`, then a hundred `bfillds`: a run reporting anything stops and is the bug.
pub fn repeated_fill(script: &mut OffsetScript<'_>) -> Result<(), Failure> {
    use crate::steps::{General, More};
    use martensite_tkbrep::topods::ShapeIterator;
    let compound = script.required("a")?;
    let mut children = ShapeIterator::new(&compound, true, true);
    while children.more() {
        script.general_objects.push(children.value()?.clone());
        children.next();
    }
    for pass in 1..=100 {
        println!("pass {pass}");
        crate::more_steps::apply(script, &More::General(General::Fill))?;
        let options = script
            .general_filler
            .as_ref()
            .expect("just filled")
            .options();
        if options.has_errors() || options.has_warnings() {
            println!("Error: the bug with failure building 2D curve of edge on face is reproduced");
            break;
        }
    }
    Ok(())
}
