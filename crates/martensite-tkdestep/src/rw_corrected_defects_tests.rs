//! Values the readers and writers keep, each on a small hand-written STEP record: the value reads
//! as written, comes out of the entity, writes back the same and reads the same again.
use crate::rw_ap214_tests::{FOOTER, HEADER, data_part, failures, model_of, written};
use crate::step_ap214::{
    StepAP214ExternallyDefinedClass, StepAP214ExternallyDefinedGeneralProperty,
    StepAP214RepItemGroup,
};
use crate::step_basic::{StepBasicExternallyDefinedItem, StepBasicSourceItem};
use crate::step_data::{
    StepDataSelectMember, StepDataSelectType, StepDataStepModel, StepDataUndefinedEntity,
};
use crate::step_dim_tol::StepDimTolCommonDatum;
use crate::step_element::{
    StepElementSurface3dElementDescriptor, StepElementVolume3dElementDescriptor,
};
use crate::step_entity::*;
use crate::step_kinematics::*;
use crate::step_visual::*;
use martensite_tkernel::standard::Transient;
use martensite_tkxsbase::interface::InterfaceModelEntities;

/// The records a text holds, without the whitespace the writer lays them out with.
fn records_of(text: &str) -> Vec<String> {
    let compact: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    compact
        .split(';')
        .filter(|record| record.starts_with('#'))
        .map(str::to_string)
        .collect()
}

/// Reads `body` with no failure; what is written is `body` again, and so is what is read back.
fn model_keeping(body: &str) -> Handle<StepDataStepModel> {
    let model = model_of(&format!("{HEADER}{body}{FOOTER}"));
    assert_eq!(failures(&model), Vec::<String>::new(), "reading {body}");
    let first = written(&model);
    assert_eq!(records_of(data_part(&first)), records_of(body));
    let again = model_of(&first);
    assert_eq!(failures(&again), Vec::<String>::new(), "reading {first}");
    assert_eq!(records_of(data_part(&written(&again))), records_of(body));
    model
}

/// The entity `num` of a model as `T`, which it must be.
fn entity_of<T: RuntimeTyped + Transient>(
    model: &Handle<StepDataStepModel>,
    num: i32,
) -> Handle<T> {
    let entity = model.get().unwrap().value(num).unwrap();
    assert!(
        !entity.is_kind(StepDataUndefinedEntity::static_type()),
        "#{num} is not recognized"
    );
    entity.cast::<T>()
}

fn string_of(text: &Handle<HAsciiString>) -> String {
    String::from_utf8_lossy(text.get().unwrap().string().as_bytes()).into_owned()
}

const SHADINGS: [(&str, StepVisualShadingSurfaceMethod); 4] = [
    (
        ".CONSTANT_SHADING.",
        StepVisualShadingSurfaceMethod::SsmConstantShading,
    ),
    (
        ".COLOUR_SHADING.",
        StepVisualShadingSurfaceMethod::SsmColourShading,
    ),
    (
        ".DOT_SHADING.",
        StepVisualShadingSurfaceMethod::SsmDotShading,
    ),
    (
        ".NORMAL_SHADING.",
        StepVisualShadingSurfaceMethod::SsmNormalShading,
    ),
];

#[test]
fn surface_style_rendering_keeps_each_shading_method() {
    for (text, method) in SHADINGS {
        let body = format!(
            "#1=DRAUGHTING_PRE_DEFINED_COLOUR('red');\n#2=SURFACE_STYLE_RENDERING({text},#1);\n"
        );
        let model = model_keeping(&body);
        let entity = entity_of::<StepVisualSurfaceStyleRendering>(&model, 2);
        assert_eq!(entity.rendering_method(), method, "{text}");
    }
}

#[test]
fn surface_style_rendering_with_properties_keeps_each_shading_method() {
    for (text, method) in SHADINGS {
        let body = format!(
            "#1=DRAUGHTING_PRE_DEFINED_COLOUR('red');\n#2=SURFACE_STYLE_TRANSPARENT(0.5);\n\
             #3=SURFACE_STYLE_RENDERING_WITH_PROPERTIES({text},#1,(#2));\n"
        );
        let model = model_keeping(&body);
        let entity = entity_of::<StepVisualSurfaceStyleRenderingWithProperties>(&model, 3);
        assert_eq!(entity.rendering_method(), method, "{text}");
    }
}

/// With the test inverted every text matched something, so the fail could not happen.
#[test]
fn a_shading_method_that_does_not_exist_is_a_failure() {
    let body =
        "#1=DRAUGHTING_PRE_DEFINED_COLOUR('red');\n#2=SURFACE_STYLE_RENDERING(.NO_SHADING.,#1);\n";
    let found = failures(&model_of(&format!("{HEADER}{body}{FOOTER}")));
    assert_eq!(
        found,
        ["#2: Parameter #1 (rendering_method) has not allowed value"]
    );
}

/// What the kinematic pairs refer to, as records 1 to 5.
const JOINT: &str = "#1=CARTESIAN_POINT('a',(0.,0.,0.));\n#2=CARTESIAN_POINT('b',(1.,0.,0.));\n\
                     #3=VERTEX_POINT('v',#1);\n#4=VERTEX_POINT('w',#2);\n#5=KINEMATIC_JOINT('j',#3,#4);\n";
/// Each direction, one absent: t_x, t_y, t_z, r_x, r_y and r_z.
const DIRECTIONS: &str =
    ".BIDIRECTIONAL.,.POSITIVE_ONLY.,.NEGATIVE_ONLY.,.NOT_ACTUATED.,$,.BIDIRECTIONAL.";

fn assert_directions(pair: &StepKinematicsActuatedKinematicPair) {
    use StepKinematicsActuatedDirection as Direction;
    assert_eq!(pair.tx(), Direction::AdBidirectional);
    assert_eq!(pair.ty(), Direction::AdPositiveOnly);
    assert_eq!(pair.tz(), Direction::AdNegativeOnly);
    assert_eq!(pair.rx(), Direction::AdNotActuated);
    assert!(!pair.has_ry());
    assert_eq!(pair.rz(), Direction::AdBidirectional);
}

#[test]
fn actuated_kinematic_pair_keeps_each_direction() {
    let body = format!("{JOINT}#6=ACTUATED_KINEMATIC_PAIR('r','t','d',#1,#2,#5,{DIRECTIONS});\n");
    let model = model_keeping(&body);
    assert_directions(
        entity_of::<StepKinematicsActuatedKinematicPair>(&model, 6)
            .get()
            .unwrap(),
    );
}

#[test]
fn actuated_pair_in_a_complex_record_keeps_each_direction() {
    let body = format!(
        "{JOINT}#6=(ACTUATED_KINEMATIC_PAIR({DIRECTIONS})GEOMETRIC_REPRESENTATION_ITEM()\
         ITEM_DEFINED_TRANSFORMATION('t','d',#1,#2)KINEMATIC_PAIR(#5)\
         LOW_ORDER_KINEMATIC_PAIR(.T.,.F.,.F.,.F.,.F.,.F.)PRISMATIC_PAIR()\
         PRISMATIC_PAIR_WITH_RANGE(0.,1.)REPRESENTATION_ITEM('r'));\n"
    );
    let model = model_keeping(&body);
    let entity = entity_of::<StepKinematicsActuatedKinPairAndOrderKinPair>(&model, 6);
    assert_directions(entity.get_actuated_kinematic_pair().get().unwrap());
}

#[test]
fn volume_purposes_keep_the_name_they_were_read_under() {
    let body = "#1=VOLUME_3D_ELEMENT_DESCRIPTOR(.LINEAR.,'d',\
                (APPLICATION_DEFINED_ELEMENT_PURPOSE('Unknown'),\
                ENUMERATED_VOLUME_ELEMENT_PURPOSE(.STRESS_DISPLACEMENT.)),.TETRAHEDRON.);\n";
    let model = model_keeping(body);
    let entity = entity_of::<StepElementVolume3dElementDescriptor>(&model, 1);
    let purposes = entity.purpose();
    let purposes = purposes.get().unwrap().array1();
    let member = |rank: i32| purposes.value(rank).unwrap().get().unwrap().name();
    assert_eq!(member(1), "APPLICATION_DEFINED_ELEMENT_PURPOSE");
    assert_eq!(member(2), "ENUMERATED_VOLUME_ELEMENT_PURPOSE");
}

#[test]
fn surface_purposes_keep_the_name_they_were_read_under() {
    let body = "#1=SURFACE_3D_ELEMENT_DESCRIPTOR(.LINEAR.,'d',\
                ((APPLICATION_DEFINED_ELEMENT_PURPOSE('Unknown'),\
                ENUMERATED_SURFACE_ELEMENT_PURPOSE(.MEMBRANE_DIRECT.))),.QUADRILATERAL.);\n";
    let model = model_keeping(body);
    let entity = entity_of::<StepElementSurface3dElementDescriptor>(&model, 1);
    let purposes = entity.purpose();
    let inner = purposes.get().unwrap().array1().value(1).unwrap().clone();
    let names = inner.get().unwrap().sequence(|items| {
        items
            .iter()
            .map(|member| member.get().unwrap().name())
            .collect::<Vec<String>>()
    });
    assert_eq!(
        names,
        [
            "APPLICATION_DEFINED_ELEMENT_PURPOSE",
            "ENUMERATED_SURFACE_ELEMENT_PURPOSE"
        ]
    );
}

const SOURCE: &str = "#1=EXTERNAL_SOURCE(IDENTIFIER('library'));\n";

/// The text of an `IDENTIFIER(...)`, which a source item holds as a named member.
fn identifier_of(item: &StepBasicSourceItem) -> String {
    let member = item.member();
    let member = member.get().expect("the member of the source item");
    assert_eq!(member.name(), "IDENTIFIER");
    String::from_utf8_lossy(member.string().as_bytes()).into_owned()
}

/// The item an externally defined record reads into its member, by its identifier and source.
fn assert_external_item(item: &Handle<StepBasicExternallyDefinedItem>) {
    let item = item.get().expect("the member `Init` fills");
    assert_eq!(identifier_of(&item.item_id()), "item");
    let source = item.source();
    assert_eq!(identifier_of(&source.get().unwrap().source_id()), "library");
}

#[test]
fn externally_defined_class_keeps_its_item() {
    let body = format!("{SOURCE}#2=EXTERNALLY_DEFINED_CLASS('n','d',IDENTIFIER('item'),#1);\n");
    let model = model_keeping(&body);
    let entity = entity_of::<StepAP214ExternallyDefinedClass>(&model, 2);
    assert_external_item(&entity.externally_defined_item());
}

#[test]
fn externally_defined_general_property_keeps_its_item() {
    let body = format!(
        "{SOURCE}#2=EXTERNALLY_DEFINED_GENERAL_PROPERTY('i','n','d',IDENTIFIER('item'),#1);\n"
    );
    let model = model_keeping(&body);
    let entity = entity_of::<StepAP214ExternallyDefinedGeneralProperty>(&model, 2);
    assert_external_item(&entity.externally_defined_item());
}

#[test]
fn rep_item_group_keeps_its_representation_item() {
    let model = model_keeping("#1=REP_ITEM_GROUP('g','d','item');\n");
    let entity = entity_of::<StepAP214RepItemGroup>(&model, 1);
    let item = entity.representation_item();
    assert_eq!(
        string_of(&item.get().expect("the member `Init` fills").name()),
        "item"
    );
}

#[test]
fn common_datum_keeps_its_datum() {
    let body = "#1=APPLICATION_CONTEXT('a');\n#2=PRODUCT_CONTEXT('c',#1,'m');\n\
                #3=PRODUCT('p','n','d',(#2));\n#4=PRODUCT_DEFINITION_FORMATION('f','d',#3);\n\
                #5=PRODUCT_DEFINITION_CONTEXT('c',#1,'s');\n#6=PRODUCT_DEFINITION('i','d',#4,#5);\n\
                #7=PRODUCT_DEFINITION_SHAPE('s','d',#6);\n\
                #8=COMMON_DATUM('n','d',#7,.F.,'dn','dd',#7,.T.,'A');\n";
    let model = model_keeping(body);
    let entity = entity_of::<StepDimTolCommonDatum>(&model, 8);
    let datum = entity.datum();
    let datum = datum.get().expect("the member `Init` fills");
    assert_eq!(string_of(&datum.identification()), "A");
    assert_eq!(string_of(&datum.parent.name()), "dn");
}

/// What the triangulated faces refer to, as record 1.
const COORDINATES: &str = "#1=COORDINATES_LIST('c',3,((0.,0.,0.),(1.,0.,0.),(0.,1.,0.)));\n";

fn rows_of<T: Copy + Send + Sync + 'static>(array: &Handle<HArray2<T>>) -> Vec<Vec<T>> {
    let array = array.get().unwrap().array2();
    (1..=array.col_length())
        .map(|row| {
            (1..=array.row_length())
                .map(|column| *array.value(row, column).unwrap())
                .collect()
        })
        .collect()
}

fn normals_kept_by_a_complex_face(normals: &str) -> Vec<Vec<f64>> {
    let body = format!(
        "{COORDINATES}#2=COMPLEX_TRIANGULATED_FACE('f',#1,3,{normals},$,(1,2,3),((1,2,3)),());\n"
    );
    let model = model_keeping(&body);
    rows_of(&entity_of::<StepVisualComplexTriangulatedFace>(&model, 2).normals())
}

/// The list of normals may be empty: only the bounds of an array with content were swapped.
#[test]
fn complex_face_writes_no_normals() {
    let body =
        format!("{COORDINATES}#2=COMPLEX_TRIANGULATED_FACE('f',#1,3,(),$,(1,2,3),((1,2,3)),());\n");
    model_keeping(&body);
}

#[test]
fn complex_face_writes_one_normal() {
    assert_eq!(
        normals_kept_by_a_complex_face("((0.,-1.,0.5))"),
        [[0., -1., 0.5]]
    );
}

#[test]
fn complex_face_writes_more_normals_than_they_have_components() {
    assert_eq!(
        normals_kept_by_a_complex_face("((1.,0.,0.),(0.,1.,0.),(0.,0.,1.),(0.5,0.5,0.))"),
        [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.], [0.5, 0.5, 0.]]
    );
}

/// A square array keeps its order: only its bounds were swapped, and those are the same.
#[test]
fn complex_face_keeps_a_square_array_in_order() {
    assert_eq!(
        normals_kept_by_a_complex_face("((1.,2.,3.),(4.,5.,6.),(7.,8.,9.))"),
        [[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]
    );
}

#[test]
fn cubic_bezier_face_writes_both_its_arrays() {
    let body = format!(
        "{COORDINATES}#2=CUBIC_BEZIER_TRIANGULATED_FACE('f',#1,3,((0.,0.,1.)),$,\
         ((1,2,3,1,2,3,1,2,3,1),(3,2,1,3,2,1,3,2,1,3)));\n"
    );
    let model = model_keeping(&body);
    let entity = entity_of::<StepVisualCubicBezierTriangulatedFace>(&model, 2);
    assert_eq!(rows_of(&entity.normals()), [[0., 0., 1.]]);
    assert_eq!(
        rows_of(&entity.ctriangles()),
        [
            [1, 2, 3, 1, 2, 3, 1, 2, 3, 1],
            [3, 2, 1, 3, 2, 1, 3, 2, 1, 3]
        ]
    );
}
