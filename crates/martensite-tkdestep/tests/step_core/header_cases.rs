//! The header section: what Martensite writes of itself, and the reading of a header.
use super::support::read_text;
use martensite_tkdestep::{
    api_header_section::APIHeaderSectionMakeHeader,
    header_section::{HeaderSection, HeaderSectionFileName, HeaderSectionFileSchema},
    rw_header_section::RWHeaderSection,
    step_data::{StepDataStepModel, StepDataStepWriter},
};
use martensite_tkernel::{
    standard::{Handle, RuntimeTyped},
    tcollection::HAsciiString,
};
use martensite_tkxsbase::interface::InterfaceModelEntities;

fn text_of(value: &Handle<HAsciiString>) -> String {
    let live = value.get().expect("a text");
    let bytes = live.string().as_bytes().to_vec();
    String::from_utf8_lossy(&bytes).into_owned()
}

#[test]
fn a_new_header_says_martensite_and_nothing_else() {
    RWHeaderSection::init();
    let header = APIHeaderSectionMakeHeader::new(4).unwrap();
    let model = StepDataStepModel::new();
    model.set_protocol(&HeaderSection::protocol().cast());
    header.apply(model.get().unwrap()).unwrap();
    let mut writer = StepDataStepWriter::new(&model);
    writer
        .send_model(&HeaderSection::protocol().cast(), true)
        .unwrap();
    let mut written = Vec::new();
    writer.print(&mut written).unwrap();
    let written = String::from_utf8_lossy(&written).into_owned();
    assert!(
        written.contains("FILE_NAME('Martensite Manifold Solid Brep Model'"),
        "{written}"
    );
    let version = env!("CARGO_PKG_VERSION");
    let processor = format!("'Martensite STEP processor {version}'");
    let system = format!("'Martensite {version}'");
    let flat = written.replace('\n', "");
    assert!(
        flat.replace(' ', "")
            .contains("('Martensite'),('Martensite')"),
        "{written}"
    );
    assert!(flat.contains(&processor), "{written}");
    assert!(flat.contains(&system), "{written}");
    assert!(flat.contains(",'Martensite');"), "{written}");
    assert!(
        written.contains("FILE_DESCRIPTION(('Martensite Model'),'2;1');"),
        "{written}"
    );
    assert!(
        written.contains("FILE_SCHEMA(('header_section'));"),
        "{written}"
    );
    for foreign in ["OCCT", "OCC", "Open CASCADE", "Unknown"] {
        assert!(!written.contains(foreign), "{foreign} in {written}");
    }
}

#[test]
fn header_values_are_read_and_old_values_stay_readable() {
    let read = read_text(
        b"ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('d'),'2;1');\n\
          FILE_NAME('n','2020-01-01T00:00:00',('a','b'),('o'),'Open CASCADE STEP processor 7.9',\n\
          'Open CASCADE 7.9','Unknown');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=A(1);\nENDSEC;\nEND-ISO-10303-21;\n",
    );
    assert_eq!(read.status, 0);
    assert!(read.fails().is_empty(), "{:?}", read.fails());
    let live = read.model.get().unwrap();
    let name = live
        .header_entity(HeaderSectionFileName::static_type())
        .unwrap();
    let name = name.cast::<HeaderSectionFileName>();
    assert_eq!(text_of(&name.name()), "n");
    assert_eq!(name.nb_author(), 2);
    assert_eq!(text_of(&name.author_value(2).unwrap()), "b");
    assert_eq!(text_of(&name.originating_system()), "Open CASCADE 7.9");
    let schema = live
        .header_entity(HeaderSectionFileSchema::static_type())
        .unwrap();
    let schema = schema.cast::<HeaderSectionFileSchema>();
    assert_eq!(
        text_of(&schema.schema_identifiers_value(1).unwrap()),
        "AUTOMOTIVE_DESIGN"
    );
    let header = APIHeaderSectionMakeHeader::from_model(live).unwrap();
    assert!(header.is_done());
}
