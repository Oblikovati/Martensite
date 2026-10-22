//! Native object modification regression for the complete OCC31452 case body.
use crate::tobj_model::{ObjectFixture, TObjTestObject};
use martensite_process_policy::case_runner::{CaseRunner, CaseSpec, FactorProfile};
use martensite_tkernel::{ncollection::HArray1, standard::Handle, tcollection::ExtendedString};
use martensite_tklcaf::tdata_std::TDataStdName;
use martensite_tktobj::tobj::{TObjModel, TObjModelState, TObjObjectState};

#[test]
fn bug31452_child_and_array_changes_mark_both_parent_objects_modified() {
    let artifacts = std::env::temp_dir().join("martensite-tobj-original-case-receipts");
    let spec = CaseSpec::new(
        "tobj_modifications_legacy::bug31452_child_and_array_changes_mark_both_parent_objects_modified",
        1000,
        FactorProfile::RequireUnset,
    );
    let receipt = CaseRunner::new(&artifacts, None)
        .run(spec, || {
            original_modification_changes();
            Ok(())
        })
        .unwrap_or_else(|failure| panic!("{failure}"));
    println!("{{\"case_receipt\":\"{}\"}}", receipt.directory.display());
}

fn original_modification_changes() {
    let fixture = named_document();
    let objects = named_objects(&fixture);
    let children = named_children(&objects);
    objects[0].set_ref(&objects[1].cast());
    children[0].set_int(200);
    children[1].set_int(110);
    objects[1].set_real_arr(&real_values(0.123));
    fixture.my_model().open_command().unwrap();
    children[0].set_int(150);
    objects[1].set_real_arr(&real_values(0.150));
    assert!(objects[0].has_modifications().unwrap());
    assert!(objects[1].has_modifications().unwrap());
    fixture.my_model().close().unwrap();
}

fn named_document() -> ObjectFixture {
    let fixture = ObjectFixture::set_up();
    let document = fixture.my_model().get_document().unwrap();
    let root = document.get_data().root();
    TDataStdName::set(&root, &ExtendedString::new("TD1")).unwrap();
    document.set_undo_limit(10).unwrap();
    fixture
}

fn named_objects(fixture: &ObjectFixture) -> [Handle<TObjTestObject>; 3] {
    ["obj1", "obj2", "obj3"].map(|name| {
        let object = fixture.create_object();
        object.set_name_c_str(name).unwrap();
        object
    })
}

fn named_children(objects: &[Handle<TObjTestObject>; 3]) -> [Handle<TObjTestObject>; 3] {
    [(0, "ch11"), (1, "ch21"), (1, "ch22")].map(|(parent, name)| {
        let child = objects[parent].add_child();
        child.set_name_c_str(name).unwrap();
        child
    })
}

#[allow(clippy::approx_constant)] // OCC31452 uses the decimal 3.14, not the mathematical constant.
fn real_values(last: f64) -> Handle<HArray1<f64>> {
    let array = Handle::new(HArray1::with_value(1, 3, 0.).unwrap());
    for (index, value) in [(1, 3.14), (2, 2.78), (3, last)] {
        array.change_array1().set_value(index, value).unwrap();
    }
    array
}
