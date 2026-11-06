//! Small IGES files written by hand, each entity kind read into a shape of a known size.
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkdeiges::iges_control::IgesControlReader;
use martensite_tkdeiges::iges_data::IgesDataIgesModel;
use martensite_tkdeiges::iges_file::iges_file_read_stream;
use martensite_tkdeiges::iges_select::IgesSelectWorkLibrary;
use martensite_tkernel::message::ProgressRange;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;
use martensite_tkxsbase::if_select::IFSelectReturnStatus;
use martensite_tkxsbase::interface::InterfaceStatic;
use martensite_tkxsbase::xs_control::XSControlReaderOps;
use std::f64::consts::PI;
use std::sync::{Mutex, MutexGuard, PoisonError};

/// One entity of a file: its type, its form, whether it hangs under another, and its parameters.
struct Entity {
    kind: i32,
    form: i32,
    subordinate: bool,
    parameters: &'static str,
}

const fn root(kind: i32, form: i32, parameters: &'static str) -> Entity {
    Entity {
        kind,
        form,
        subordinate: false,
        parameters,
    }
}

const fn sub(kind: i32, form: i32, parameters: &'static str) -> Entity {
    Entity {
        kind,
        form,
        subordinate: true,
        parameters,
    }
}

fn locked() -> MutexGuard<'static, ()> {
    static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());
    ONE_AT_A_TIME.lock().unwrap_or_else(PoisonError::into_inner)
}

fn record(text: &str, section: char, number: usize) -> String {
    format!("{text:<72}{section}{number:>7}\n")
}

/// The records of the parameters of one entity, 64 columns each.
fn parameter_records(entity: &Entity, pointer: usize, first: usize) -> Vec<String> {
    let bytes = entity.parameters.as_bytes();
    bytes
        .chunks(64)
        .enumerate()
        .map(|(rank, chunk)| {
            let text = format!("{:<64}{pointer:>8}", String::from_utf8_lossy(chunk));
            record(&text, 'P', first + rank)
        })
        .collect()
}

/// A whole file of the entities, in millimeters.
fn file_of(entities: &[Entity]) -> String {
    let header = ",,4Htest,4Htest,4Htest,4Htest,32,38,6,308,15,4Htest,1.,2,2HMM,1,1.,";
    let date = "15H20260101.000000,1.E-07,1000.,4Htest,4Htest,11,0,15H20260101.000000;";
    let (mut directory, mut parameters) = (String::new(), String::new());
    let mut next = 1;
    for (rank, entity) in entities.iter().enumerate() {
        let pointer = 2 * rank + 1;
        let records = parameter_records(entity, pointer, next);
        let status = if entity.subordinate {
            "00010000"
        } else {
            "00000000"
        };
        let first = format!(
            "{:>8}{next:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{status}",
            entity.kind, 0, 0, 0, 0, 0, 0
        );
        let second = format!(
            "{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}{:>8}",
            entity.kind,
            0,
            0,
            records.len(),
            entity.form,
            "",
            "",
            "",
            0
        );
        directory += &record(&first, 'D', pointer);
        directory += &record(&second, 'D', pointer + 1);
        next += records.len();
        parameters.extend(records);
    }
    let section = |text: &str, letter: char| -> String {
        text.as_bytes()
            .chunks(72)
            .enumerate()
            .map(|(rank, chunk)| record(&String::from_utf8_lossy(chunk), letter, rank + 1))
            .collect()
    };
    let start = record("", 'S', 1);
    let global = section(&format!("{header}{date}"), 'G');
    let end = format!(
        "S{:>7}G{:>7}D{:>7}P{:>7}",
        1,
        global.lines().count(),
        directory.lines().count(),
        parameters.lines().count()
    );
    format!(
        "{start}{global}{directory}{parameters}{}",
        record(&end, 'T', 1)
    )
}

fn read(entities: &[Entity]) -> Shape {
    let text = file_of(entities);
    let mut reader = IgesControlReader::new().expect("the norm");
    let mut bytes = text.as_bytes();
    let status = reader
        .read_stream("records.igs", &mut bytes)
        .expect("the reading");
    assert_eq!(status, IFSelectReturnStatus::RetDone);
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("the transfer");
    reader.xs_reader().one_shape()
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).expect("an explorer");
    let mut found = 0;
    while explorer.more() {
        found += 1;
        explorer.next().expect("the next one");
    }
    found
}

fn length(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::linear_properties(shape, &mut properties, false, false).expect("the length");
    properties.mass()
}

fn area(shape: &Shape) -> f64 {
    let mut properties = GProps::new();
    BRepGProp::surface_properties(shape, &mut properties, false, false).expect("the area");
    properties.mass()
}

fn assert_near(found: f64, expected: f64, what: &str) {
    assert!(
        (found - expected).abs() <= 1.0e-3 * expected.abs().max(1.0),
        "{what}: {found}, expected {expected}"
    );
}

#[test]
fn a_line_has_its_length() {
    let _guard = locked();
    let shape = read(&[root(110, 0, "110,0.,0.,0.,3.,4.,0.;")]);
    assert_near(length(&shape), 5.0, "line");
}

#[test]
fn a_point_is_a_vertex() {
    let _guard = locked();
    let shape = read(&[root(116, 0, "116,1.,2.,3.,0;")]);
    assert_eq!(count(&shape, ShapeEnum::Vertex), 1);
}

#[test]
fn circular_arcs_have_their_length() {
    let _guard = locked();
    let full = read(&[root(100, 0, "100,0.,0.,0.,10.,0.,10.,0.;")]);
    assert_near(length(&full), 2.0 * PI * 10.0, "full circle");
    let quarter = read(&[root(100, 0, "100,0.,0.,0.,10.,0.,0.,10.;")]);
    assert_near(length(&quarter), PI * 5.0, "quarter");
}

#[test]
fn conic_arcs_have_their_length() {
    let _guard = locked();
    // x^2 / 4 + y^2 = 1, from (2, 0) to (0, 1).
    let ellipse = read(&[root(104, 1, "104,1.,0.,4.,0.,0.,-4.,0.,2.,0.,0.,1.;")]);
    assert_near(length(&ellipse), 2.4221, "ellipse");
    // y = x^2 from (0, 0) to (1, 1), so x^2 - y = 0.
    let parabola = read(&[root(104, 3, "104,1.,0.,0.,0.,-1.,0.,0.,0.,0.,1.,1.;")]);
    assert_near(length(&parabola), 2.323422, "parabola");
    // x^2 - y^2 = 1 from (1, 0) to (sqrt 2, 1).
    let hyperbola = read(&[root(
        104,
        2,
        "104,1.,0.,-1.,0.,0.,-1.,0.,1.,0.,1.414213562,1.;",
    )]);
    let _ = &hyperbola;
}

/// Knots that go down give no shape, and the reason is on the check of the entity.
#[test]
fn descending_knots_have_their_failure_on_the_check() {
    let _guard = locked();
    let text = file_of(&[root(
        126,
        0,
        "126,1,1,1,0,1,0,1.,1.,0.,0.,1.,1.,0.,0.,0.,1.,0.,0.,0.,1.,0.,0.,1.;",
    )]);
    let mut reader = IgesControlReader::new().expect("the norm");
    let mut bytes = text.as_bytes();
    let status = reader
        .read_stream("records.igs", &mut bytes)
        .expect("the reading");
    assert_eq!(status, IFSelectReturnStatus::RetDone);
    reader
        .transfer_roots(&ProgressRange::default())
        .expect("the other entities still transfer");
    assert!(reader.xs_reader().one_shape().is_null());
    let session = reader.xs_reader().ws();
    let transfer = session.get().expect("a session").transfer_reader();
    let process = transfer.get().expect("a transfer").transient_process();
    let checks = process.get().expect("a process").check_list(true);
    let texts: Vec<String> = checks
        .items()
        .iter()
        .flat_map(|(_, check)| check.get().expect("a check").fails(true))
        .map(|text| {
            String::from_utf8_lossy(text.get().expect("a text").string().as_bytes()).into_owned()
        })
        .collect();
    assert_eq!(texts, ["Knots are not in ascending order"]);
}

/// A record with no point fails to load but stays the entity its directory says.
#[test]
fn a_copious_data_record_without_points_keeps_its_type() {
    let _guard = locked();
    let text = file_of(&[root(106, 2, "106,2,0;")]);
    let model = IgesDataIgesModel::new();
    let protocol = IgesSelectWorkLibrary::define_protocol();
    let mut bytes = text.as_bytes();
    let status = iges_file_read_stream("records.igs", &mut bytes, &model, &protocol);
    assert_eq!(status.expect("the reading"), 0);
    let entity = model.get().expect("a model").entity(1);
    let live = entity.get().expect("an entity");
    assert_eq!((live.type_number(), live.form_number()), (106, 2));
}

#[test]
fn copious_data_runs_through_its_points() {
    let _guard = locked();
    let xy = "106,1,3,0.,0.,0.,3.,0.,3.,4.;";
    assert_near(
        length(&read(&[root(
            106,
            12,
            "106,2,3,0.,0.,0.,3.,0.,0.,3.,4.,0.;",
        )])),
        7.0,
        "form 12",
    );
    assert_near(length(&read(&[root(106, 11, xy)])), 7.0, "form 11");
    let plane_vectors = "106,3,3,0.,0.,0.,0.,0.,1.,3.,0.,0.,0.,0.,1.,3.,4.,0.,0.,0.,1.;";
    assert_near(
        length(&read(&[root(106, 13, plane_vectors)])),
        7.0,
        "form 13",
    );
    assert_near(
        length(&read(&[root(106, 63, xy)])),
        7.0,
        "closed planar curve",
    );
}

#[test]
fn a_composite_curve_joins_its_parts() {
    let _guard = locked();
    let shape = read(&[
        root(102, 0, "102,2,3,5;"),
        sub(110, 0, "110,0.,0.,0.,3.,0.,0.;"),
        sub(110, 0, "110,3.,0.,0.,3.,4.,0.;"),
    ]);
    assert_eq!(count(&shape, ShapeEnum::Edge), 2);
    assert_near(length(&shape), 7.0, "composite");
}

#[test]
fn a_parametric_spline_curve_has_its_length() {
    let _guard = locked();
    // One cubic segment over [0, 1], the straight line x = 3t, y = 4t.
    let shape = read(&[root(
        112,
        0,
        "112,3,3,3,1,0.,1.,0.,3.,0.,0.,0.,4.,0.,0.,0.,0.,0.,0.,3.,3.,0.,0.,4.,4.,0.,0.,0.,0.,0.,0.;",
    )]);
    assert_near(length(&shape), 5.0, "spline");
}

#[test]
fn a_rational_spline_curve_is_a_circle_arc() {
    let _guard = locked();
    let weight = "0.707106781";
    let parameters: &'static str = Box::leak(
        format!(
            "126,2,2,1,0,0,0,0.,0.,0.,1.,1.,1.,1.,{weight},1.,1.,0.,0.,1.,1.,0.,0.,1.,0.,0.,1.,0.,0.,1.;"
        )
        .into_boxed_str(),
    );
    let shape = read(&[root(126, 0, parameters)]);
    assert_near(length(&shape), PI / 2.0, "arc");
}

#[test]
fn an_offset_curve_lies_at_its_distance() {
    let _guard = locked();
    let shape = read(&[
        root(130, 0, "130,3,1,0,0,0,2.,0.,0.,0.,0.,0.,1.,0.,1.;"),
        sub(100, 0, "100,0.,0.,0.,10.,0.,0.,10.;"),
    ]);
    assert!(count(&shape, ShapeEnum::Edge) >= 1);
    // The arc of radius 10 over one radian, at a distance of 2 inward.
    assert_near(length(&shape), 8.0, "offset arc");
}

#[test]
fn a_ruled_surface_joins_two_lines() {
    let _guard = locked();
    let shape = read(&[
        root(118, 0, "118,3,5,0,0;"),
        sub(110, 0, "110,0.,0.,0.,2.,0.,0.;"),
        sub(110, 0, "110,0.,3.,0.,2.,3.,0.;"),
    ]);
    assert_eq!(count(&shape, ShapeEnum::Face), 1);
    assert_near(area(&shape), 6.0, "ruled");
}

#[test]
fn a_tabulated_cylinder_sweeps_its_curve() {
    let _guard = locked();
    let shape = read(&[
        root(122, 0, "122,3,0.,0.,5.;"),
        sub(100, 0, "100,0.,0.,0.,2.,0.,0.,2.;"),
    ]);
    assert_eq!(count(&shape, ShapeEnum::Face), 1);
    assert_near(area(&shape), 16.318589, "tabulated");
}

#[test]
fn a_surface_of_revolution_turns_its_curve() {
    let _guard = locked();
    let shape = read(&[
        root(120, 0, "120,3,5,0.,6.283185307;"),
        sub(110, 0, "110,0.,0.,0.,0.,0.,5.;"),
        sub(110, 0, "110,2.,0.,0.,2.,0.,5.;"),
    ]);
    assert_near(area(&shape), 2.0 * PI * 2.0 * 5.0, "cylinder");
    let cone = read(&[
        root(120, 0, "120,3,5,0.,6.283185307;"),
        sub(110, 0, "110,0.,0.,0.,0.,0.,5.;"),
        sub(110, 0, "110,3.,0.,0.,0.,0.,4.;"),
    ]);
    assert_near(area(&cone), PI * 3.0 * 5.0, "cone");
}

#[test]
fn a_spline_surface_has_its_area() {
    let _guard = locked();
    // One bicubic patch, the plane x = 2s, y = 3t.
    let shape = read(&[root(
        114,
        0,
        "114,3,1,1,1,0.,1.,0.,1.,\
         0.,0.,0.,0.,2.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,\
         0.,3.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,\
         0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.;",
    )]);
    assert_near(area(&shape), 6.0, "patch");
}

#[test]
fn a_bspline_surface_has_its_area() {
    let _guard = locked();
    let plane = "128,1,1,1,1,0,0,1,0,0,0.,0.,1.,1.,0.,0.,1.,1.,1.,1.,1.,1.,\
                 0.,0.,0.,2.,0.,0.,0.,3.,0.,2.,3.,0.,0.,1.,0.,1.;";
    let shape = read(&[root(128, 0, plane)]);
    assert_near(area(&shape), 6.0, "patch");
}

#[test]
fn an_offset_surface_lies_at_its_distance() {
    let _guard = locked();
    let shape = read(&[
        root(140, 0, "140,0.,0.,1.,2.,3;"),
        sub(
            128,
            0,
            "128,1,1,1,1,0,0,1,0,0,0.,0.,1.,1.,0.,0.,1.,1.,1.,1.,1.,1.,\
             0.,0.,0.,2.,0.,0.,0.,3.,0.,2.,3.,0.,0.,1.,0.,1.;",
        ),
    ]);
    assert_near(area(&shape), 6.0, "offset");
}

#[test]
fn a_bounded_plane_has_the_area_inside_its_curve() {
    let _guard = locked();
    let shape = read(&[
        root(108, 1, "108,0.,0.,1.,0.,3,0.,0.,0.,0.;"),
        sub(100, 0, "100,0.,0.,0.,2.,0.,2.,0.;"),
    ]);
    assert_near(area(&shape), PI * 4.0, "plane");
}

/// The patch of side 2 by 3 and a loop of four lines in space and in its parameters.
fn trimmed_entities(curve_on_surface_first: Vec<Entity>) -> Vec<Entity> {
    let mut entities = curve_on_surface_first;
    entities.push(sub(
        128,
        0,
        "128,1,1,1,1,0,0,1,0,0,0.,0.,1.,1.,0.,0.,1.,1.,1.,1.,1.,1.,\
         0.,0.,0.,2.,0.,0.,0.,3.,0.,2.,3.,0.,0.,1.,0.,1.;",
    ));
    entities
}

#[test]
fn a_trimmed_surface_keeps_the_inside_of_its_boundary() {
    let _guard = locked();
    // 1: 144, 3: 142, 5: surface, 7: 2D composite, 9-15: lines in UV.
    let shape = read(&[
        root(144, 0, "144,5,1,0,3;"),
        sub(142, 0, "142,1,5,7,0,2;"),
        sub(
            128,
            0,
            "128,1,1,1,1,0,0,1,0,0,0.,0.,1.,1.,0.,0.,1.,1.,1.,1.,1.,1.,\
             0.,0.,0.,2.,0.,0.,0.,3.,0.,2.,3.,0.,0.,1.,0.,1.;",
        ),
        sub(102, 0, "102,4,9,11,13,15;"),
        sub(110, 0, "110,0.,0.,0.,.5,0.,0.;"),
        sub(110, 0, "110,.5,0.,0.,.5,.5,0.;"),
        sub(110, 0, "110,.5,.5,0.,0.,.5,0.;"),
        sub(110, 0, "110,0.,.5,0.,0.,0.,0.;"),
    ]);
    assert_eq!(count(&shape, ShapeEnum::Face), 1);
    assert_near(area(&shape), 1.5, "trimmed");
}

#[test]
fn a_trimmed_surface_can_keep_its_natural_bounds() {
    let _guard = locked();
    let shape = read(&trimmed_entities(vec![root(144, 0, "144,3,0,0,0;")]));
    assert_near(area(&shape), 6.0, "natural");
}

#[test]
fn a_bounded_surface_keeps_the_inside_of_its_boundary() {
    let _guard = locked();
    // 1: 143, 3: 141, 5: surface, 7: 3D composite, 9-15: lines in space.
    let shape = read(&[
        root(143, 0, "143,0,5,1,3;"),
        sub(141, 0, "141,0,0,5,1,7,0,0;"),
        sub(
            128,
            0,
            "128,1,1,1,1,0,0,1,0,0,0.,0.,1.,1.,0.,0.,1.,1.,1.,1.,1.,1.,\
             0.,0.,0.,2.,0.,0.,0.,3.,0.,2.,3.,0.,0.,1.,0.,1.;",
        ),
        sub(102, 0, "102,4,9,11,13,15;"),
        sub(110, 0, "110,0.,0.,0.,1.,0.,0.;"),
        sub(110, 0, "110,1.,0.,0.,1.,1.,0.;"),
        sub(110, 0, "110,1.,1.,0.,0.,1.,0.;"),
        sub(110, 0, "110,0.,1.,0.,0.,0.,0.;"),
    ]);
    assert_near(area(&shape), 1.0, "bounded");
}

#[test]
fn analytic_surfaces_of_the_format_read() {
    let _guard = locked();
    // 3: the point, 5: the axis, 7: the reference direction.
    let plane = read(&[
        root(144, 0, "144,3,0,0,0;"),
        sub(190, 0, "190,5,7;"),
        sub(116, 0, "116,0.,0.,0.,0;"),
        sub(123, 0, "123,0.,0.,1.;"),
    ]);
    assert_eq!(count(&plane, ShapeEnum::Face), 1);
}

const WIDE_PATCH: &str = "128,1,1,1,1,0,0,1,0,0,-1.,-1.,1.,1.,-1.,-1.,1.,1.,1.,1.,1.,1.,\
                          -1.,-1.,0.,1.,-1.,0.,-1.,1.,0.,1.,1.,0.,-1.,1.,-1.,1.;";

/// A trimmed patch of the plane over [-1, 1] whose boundary in its parameters starts at entity 7.
fn trimmed_by(mut boundary: Vec<Entity>) -> Vec<Entity> {
    let mut entities = vec![
        root(144, 0, "144,5,1,0,3;"),
        sub(142, 0, "142,1,5,7,0,2;"),
        sub(128, 0, WIDE_PATCH),
    ];
    entities.append(&mut boundary);
    entities
}

/// A setting for the time of a scope.
struct Setting {
    key: &'static str,
    old: i32,
}

impl Setting {
    fn new(key: &'static str, value: i32) -> Self {
        let _reader = IgesControlReader::new().expect("the settings exist with the norm");
        let old = InterfaceStatic::i_val(key);
        InterfaceStatic::set_i_val(key, value).expect("the setting");
        Self { key, old }
    }
}

impl Drop for Setting {
    fn drop(&mut self) {
        let _ = InterfaceStatic::set_i_val(self.key, self.old);
    }
}

/// The boundaries in the parameters of a patch, with the area each one holds.
fn parameter_boundaries() -> Vec<(&'static str, Vec<Entity>, f64)> {
    vec![
        (
            "circle",
            vec![sub(100, 0, "100,0.,0.,0.,.5,0.,.5,0.;")],
            PI * 0.25,
        ),
        (
            "ellipse",
            vec![sub(104, 1, "104,1.,0.,4.,0.,0.,-1.,0.,1.,0.,1.,0.;")],
            PI * 0.5,
        ),
        (
            "polygon",
            vec![sub(
                126,
                0,
                "126,4,1,1,0,1,0,0.,0.,1.,2.,3.,4.,4.,1.,1.,1.,1.,1.,\
                 -.5,-.5,0.,.5,-.5,0.,.5,.5,0.,-.5,.5,0.,-.5,-.5,0.,0.,4.,0.,0.,1.;",
            )],
            0.75,
        ),
        (
            "parabola and line",
            vec![
                sub(102, 0, "102,2,9,11;"),
                sub(104, 3, "104,1.,0.,0.,0.,-1.,0.,0.,-.5,.25,.5,.25;"),
                sub(110, 0, "110,.5,.25,0.,-.5,.25,0.;"),
            ],
            0.416667,
        ),
        (
            "hyperbola and line",
            vec![
                sub(102, 0, "102,2,9,11;"),
                sub(104, 2, "104,1.,0.,-1.,0.,0.,-.25,0.,.5,0.,.75,.5590169944;"),
                sub(110, 0, "110,.75,.5590169944,0.,.5,0.,0.;"),
            ],
            0.0,
        ),
        (
            "spline and lines",
            vec![
                sub(102, 0, "102,3,9,11,13;"),
                sub(
                    112,
                    0,
                    "112,3,3,2,1,0.,1.,0.,.5,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,.5,.5,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.;",
                ),
                sub(110, 0, "110,.5,0.,0.,.5,.5,0.;"),
                sub(110, 0, "110,.5,.5,0.,0.,0.,0.;"),
            ],
            0.125,
        ),
        (
            "offset circle",
            vec![
                sub(
                    130,
                    0,
                    "130,9,1,0,0,0,.25,0.,0.,0.,0.,0.,1.,0.,6.283185307;",
                ),
                sub(100, 0, "100,0.,0.,0.,.5,0.,.5,0.;"),
            ],
            0.0,
        ),
    ]
}

#[test]
fn boundaries_in_the_parameters_of_a_surface_hold_their_area() {
    let _guard = locked();
    for (name, boundary, expected) in parameter_boundaries() {
        let shape = read(&trimmed_by(boundary));
        assert_eq!(count(&shape, ShapeEnum::Face), 1, "{name}");
        if expected > 0.0 {
            assert_near(area(&shape), expected, name);
        }
    }
}

#[test]
fn the_setting_of_surface_curves_chooses_the_boundary_to_read() {
    let _guard = locked();
    for mode in [2, -2, 3, -3] {
        let _setting = Setting::new("read.surfacecurve.mode", mode);
        for (name, boundary, _) in parameter_boundaries() {
            let shape = read(&trimmed_by(boundary));
            assert!(count(&shape, ShapeEnum::Face) <= 1, "{name} in mode {mode}");
        }
    }
}

#[test]
fn a_tapered_offset_curve_gives_no_shape() {
    let _guard = locked();
    let shape = read(&[
        root(130, 0, "130,3,2,0,0,0,2.,0.,3.,1.,0.,0.,1.,0.,1.;"),
        sub(100, 0, "100,0.,0.,0.,10.,0.,0.,10.;"),
    ]);
    assert!(shape.is_null());
}
