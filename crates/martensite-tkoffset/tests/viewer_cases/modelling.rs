//! The healing, loft, fillet, offset, two-step boolean and text commands of the sample scripts.
use crate::geometry::Curve;
use crate::geometry::Curve2d;
use crate::variables::Variables;
use martensite_tkbo::bop_algo::{BOPAlgoPaveFiller, Operation};
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::String8;
use martensite_tkfillet::brep_fillet_api::BRepFilletAPIMakeFillet;
use martensite_tkfillet::chfi3d::FilletShape;
use martensite_tkgeomalgo::geom_api;
use martensite_tkmath::geom_abs::{JoinType, Shape as Continuity};
use martensite_tkmath::gp::{Axis3, Direction3, Plane3, Point3};
use martensite_tkoffset::brep_offset::{BRepOffsetMakeOffset, BRepOffsetMode, MakeOffsetOptions};
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIThruSections;
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment, Graphic3dVerticalTextAlignment,
};
use martensite_tkshhealing::shape_fix::ShapeFixShape;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape as _;
use martensite_tktopalgo::brep_extrema::BRepExtremaDistShapeShape;
use martensite_tktopalgo::brep_lib;
use martensite_tkv3d::std_prs::{StdPrsBRepFont, StdPrsBRepFontTextOptions};

/// `fixshape result shape`; `fixed(&shape)`.
pub fn fixed(shape: &Shape) -> Shape {
    let healer = ShapeFixShape::default();
    healer.init(shape.clone());
    healer
        .perform_with_progress(&ProgressRange::default())
        .unwrap();
    healer.shape()
}

/// `to3d name curve2d plane`; `to_3d(&line, &plane)`.
pub fn to_3d(curve: &Curve2d, plane: &Plane3) -> Curve {
    geom_api::to_3d(curve, plane).unwrap().cast()
}

/// `thrusections [-N] result solid ruled wire wire`; `-N` turns the compatibility check off.
/// Example: `loft(&[wire_a, wire_b], true, true, true)`.
pub fn loft(sections: &[Shape], solid: bool, ruled: bool, check: bool) -> Shape {
    let mut generator = BRepOffsetAPIThruSections::new(solid, ruled, 1.0e-06);
    for section in sections {
        generator.add_wire(&topods::wire(section).unwrap());
    }
    generator.set_mutable_input(true);
    generator.check_compatibility(check);
    generator.build(&ProgressRange::default()).unwrap();
    assert!(
        generator.is_done(),
        "thrusections of {} sections",
        sections.len()
    );
    generator.shape().unwrap().clone()
}

/// `blend result shape radius edge ...`, with the command's default tolerances.
/// Example: `blended(&solid, 2.5, &edges)`.
pub fn blended(shape: &Shape, radius: f64, edges: &[Shape]) -> Shape {
    let mut maker = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational).unwrap();
    maker.set_params(1.0e-2, 1.0e-4, 1.0e-5, 1.0e-4, 1.0e-5, 1.0e-3);
    maker.set_continuity(Continuity::C1, 1.0e-2);
    for edge in edges {
        maker
            .add_with_radius(radius, &topods::edge(edge).unwrap())
            .unwrap();
    }
    maker.build(&ProgressRange::default()).unwrap();
    assert!(maker.is_done(), "blend of {} edges: not done", edges.len());
    maker.shape().unwrap().clone()
}

/// `offsetshape result shape value tolerance cap`: a hollow solid with one open face.
/// Example: `hollowed(&solid, -0.6, 1.0e-3, &face)`.
pub fn hollowed(shape: &Shape, value: f64, tolerance: f64, cap: &Shape) -> Shape {
    let options = MakeOffsetOptions {
        mode: BRepOffsetMode::Skin,
        intersection: false,
        self_inter: false,
        join: JoinType::Arc,
        thickening: false,
        remove_int_edges: false,
    };
    let mut builder = BRepOffsetMakeOffset::new();
    builder.initialize(shape, value, tolerance, options);
    builder.add_face(&topods::face(cap).unwrap());
    builder.make_thick_solid(&ProgressRange::default()).unwrap();
    assert!(!builder.shape().is_null(), "offsetshape: null result");
    builder.shape().clone()
}

/// `bop first second; bopfuse result`, the fuse from a prepared pave filler.
/// Example: `two_step_fuse(&mut variables, "bottle", "bodytext", "thread")`.
pub fn two_step_fuse(variables: &mut Variables, result: &str, first: &str, second: &str) {
    use crate::support::common_operation::{configured_intersections, performed_with_filler};
    let (first, second) = (variables.get(first), variables.get(second));
    let mut filler: BOPAlgoPaveFiller = configured_intersections(&first, &second);
    filler.perform();
    let operation = performed_with_filler(&first, &second, Operation::Fuse, &mut filler);
    assert!(
        !operation.options().has_errors(),
        "bopfuse {result}: failed"
    );
    variables.set(result, operation.shape().clone());
}

/// `distmini name first second; dval name_val`; `distance_between(&vertex, &face)`.
pub fn distance_between(first: &Shape, second: &Shape) -> f64 {
    let mut distance = BRepExtremaDistShapeShape::new();
    distance.load_s1(first).unwrap();
    distance.load_s2(second).unwrap();
    distance.perform(&ProgressRange::default()).unwrap();
    assert!(distance.is_done(), "distmini: no distance is found");
    distance.value().unwrap()
}

/// The font files of the text samples: FreeSerif and its bold face for the serif names, the bold
/// oblique FreeSans and the oblique FreeMono for the others.
macro_rules! font_file {
    ($name:literal) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../external/martensite-test-data/fonts/freefont-20211204/",
            $name
        )
    };
}
pub const SERIF: &str = font_file!("FreeSerif.ttf");
pub const SERIF_BOLD: &str = font_file!("FreeSerifBold.ttf");
pub const SANS_BOLD_OBLIQUE: &str = font_file!("FreeSansBoldOblique.ttf");
pub const MONO_OBLIQUE: &str = font_file!("FreeMonoOblique.ttf");

/// A `text2brep` request, with the command's defaults for what a script leaves out.
#[derive(Clone, Copy)]
pub struct TextRequest<'a> {
    pub font: &'a str,
    pub height: f64,
    pub position: [f64; 3],
    pub vertical: Graphic3dVerticalTextAlignment,
}

impl Default for TextRequest<'_> {
    fn default() -> Self {
        Self {
            font: MONO_OBLIQUE,
            height: 16.,
            position: [0.; 3],
            vertical: Graphic3dVerticalTextAlignment::Bottom,
        }
    }
}

/// `text2brep name text ...`, the contours separate; `text_shape("Ice", &request)`.
pub fn text_shape(text: &str, request: &TextRequest<'_>) -> Shape {
    let font = StdPrsBRepFont::new();
    let path = String8::from_unicode_units(request.font.as_bytes(), -1);
    let found = font.init(&path, request.height, 0);
    assert!(
        found,
        "font {:?}; expected the file in the test data",
        request.font
    );
    let [x, y, z] = request.position;
    let z_axis = Direction3::new(0., 0., 1.).unwrap();
    let x_axis = Direction3::new(1., 0., 0.).unwrap();
    let options = StdPrsBRepFontTextOptions {
        pen: Axis3::new(Point3::new(x, y, z), z_axis, x_axis).unwrap(),
        horizontal_alignment: Graphic3dHorizontalTextAlignment::Left,
        vertical_alignment: request.vertical,
        to_concatenate_contours: false,
    };
    let units = String8::from_unicode_units(text.as_bytes(), -1);
    let shape = font.render_text_with(&units, &options);
    assert!(!shape.is_null(), "text {text:?}; expected contours");
    shape
}

/// `mkedgecurve wire tolerance`: the 3D curves of the wire's edges; `build_3d_curves(&wire, 1.0e-5)`.
pub fn build_3d_curves(shape: &Shape, tolerance: f64) {
    brep_lib::build_curves3d(shape, tolerance, Continuity::C1, 14, 0).unwrap();
}
