//! Standard_Dump legacy assertions against real geometry types.
//!
//! These cases dump gp_Pnt, gp_Ax3 and Bnd_Box; the cases on the stream alone run with
//! the foundation crate.
#![allow(non_snake_case)]
use martensite_tkernel::standard::dump::{DumpStream, add_values_separator};
use martensite_tkmath::{
    bnd::BoundingBox3,
    gp::{Axis3, Direction3, Point3},
};
#[test]
fn Standard_DumpTest__gp_Pnt_DumpAndInit() {
    let point = Point3::new(1.5, 2.5, 3.5);
    let mut stream = DumpStream::default();
    point.dump_json(&mut stream).unwrap();
    assert!(stream.text().contains("\"gp_Pnt\""));
    assert!(stream.text().contains("[1.5, 2.5, 3.5]"));
    let mut restored = Point3::default();
    let mut position = 1;
    assert!(
        restored
            .init_from_json(stream.text(), &mut position)
            .unwrap()
    );
    assert_eq!(point.x(), restored.x());
    assert_eq!(point.y(), restored.y());
    assert_eq!(point.z(), restored.z());
}
#[test]
fn Standard_DumpTest__gp_Ax3_DumpAndInit_MultipleSeparators() {
    let axis = Axis3::new(Point3::new(1.0, 2.0, 3.0), Direction3::Z, Direction3::X).unwrap();
    let mut stream = DumpStream::default();
    axis.dump_json(&mut stream).unwrap();
    for name in ["Direction", "XDirection", "YDirection"] {
        assert!(stream.text().contains(&format!(", \"{name}\"")));
        assert!(!stream.text().contains(&format!("]\"{name}\"")));
    }
    let mut restored = Axis3::default();
    let mut position = 1;
    assert!(
        restored
            .init_from_json(stream.text(), &mut position)
            .unwrap()
    );
    assert!(axis.location().is_equal(&restored.location(), 1e-10));
    assert!(axis.direction().is_equal(&restored.direction(), 1e-10));
}
#[test]
fn Standard_DumpTest__Bnd_Box_ComplexDump() {
    let mut bounds = BoundingBox3::new(
        Point3::new(-5.0, -10.0, -15.0),
        Point3::new(20.0, 30.0, 40.0),
    );
    bounds.set_gap(1.5);
    let mut stream = DumpStream::default();
    bounds.dump_json(&mut stream).unwrap();
    for name in ["CornerMax", "Gap", "Flags"] {
        assert!(stream.text().contains(&format!(", \"{name}\"")));
    }
    let mut restored = BoundingBox3::default();
    let mut position = 1;
    assert!(
        restored
            .init_from_json(stream.text(), &mut position)
            .unwrap()
    );
    let original = bounds.get().unwrap();
    let copy = restored.get().unwrap();
    for index in 0..6 {
        assert_eq!(original[index], copy[index]);
    }
    assert_eq!(bounds.gap(), restored.gap());
}
#[test]
fn Standard_DumpTest__ConsecutiveDumps() {
    let mut stream = DumpStream::default();
    Point3::new(1.0, 2.0, 3.0).dump_json(&mut stream).unwrap();
    add_values_separator(&mut stream).unwrap();
    Point3::new(4.0, 5.0, 6.0).dump_json(&mut stream).unwrap();
    assert!(stream.text().contains(", \"gp_Pnt\""));
}
#[test]
fn Standard_DumpTest__VoidBoxSerialization() {
    let bounds = BoundingBox3::default();
    let mut stream = DumpStream::default();
    bounds.dump_json(&mut stream).unwrap();
    assert!(!stream.text().is_empty());
    let mut restored = BoundingBox3::default();
    let mut position = 1;
    assert!(
        restored
            .init_from_json(stream.text(), &mut position)
            .unwrap()
    );
    assert!(restored.is_void());
}
