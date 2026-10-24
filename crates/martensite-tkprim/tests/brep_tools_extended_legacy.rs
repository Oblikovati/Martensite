//! Exact stream, format-version, and parallel-read mirrors of `BRepTools_Test`.
#![allow(non_snake_case)]

use martensite_tkbrep::bintools::{self, WriteOptions as BinaryWriteOptions};
use martensite_tkbrep::brep::{BRepBuilder, BRepTool};
use martensite_tkbrep::brep_tools::{self, BRepWriteOptions};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::format_version as ascii_version;
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::storage::IoStream;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::poly::mesh_purpose;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert};
use martensite_tktopalgo::brep_gprop::BRepGProp;
use std::io::{Read, Seek, SeekFrom};
use std::sync::Arc;
use std::thread;

fn stream_bytes(stream: IoStream) -> Vec<u8> {
    let mut device = stream.into_device().unwrap();
    device.seek(SeekFrom::Start(0)).unwrap();
    let mut bytes = Vec::new();
    device.read_to_end(&mut bytes).unwrap();
    bytes
}

fn ascii_box_bytes() -> Vec<u8> {
    let mut maker =
        martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::from_sizes(10.0, 20.0, 30.0).unwrap();
    let shape = maker.shape().unwrap().clone();
    assert!(!shape.is_null());
    let mut stream = IoStream::from_bytes(Vec::new());
    brep_tools::write(&shape, &mut stream, &ProgressRange::default()).unwrap();
    assert!(stream.good());
    assert!(stream.tellp() > 0);
    stream_bytes(stream)
}

fn read_shape_buffer(bytes: &[u8]) -> Option<Shape> {
    let mut stream = IoStream::from_bytes(bytes.to_vec());
    let mut shape = Shape::default();
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .ok()?;
    if shape.is_null() {
        return None;
    }
    Some(shape)
}

fn volume_mass(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
    properties.mass().abs()
}

fn parallel_shapes(bytes: Arc<Vec<u8>>, count: usize) -> Vec<Option<Shape>> {
    let workers = (0..count)
        .map(|_| {
            let bytes = Arc::clone(&bytes);
            thread::spawn(move || read_shape_buffer(&bytes))
        })
        .collect::<Vec<_>>();
    workers
        .into_iter()
        .map(|worker| worker.join().unwrap_or(None))
        .collect()
}

#[test]
fn BRepTools_Test__ParallelRead_IdenticalMass() {
    let bytes = Arc::new(ascii_box_bytes());
    let reference_shape = read_shape_buffer(&bytes).expect("single-threaded read succeeds");
    let reference_mass = volume_mass(&reference_shape);
    assert!(reference_mass > 0.0);

    let shapes = parallel_shapes(bytes, 200);
    assert!(shapes.iter().all(Option::is_some));
    for (index, shape) in shapes.iter().enumerate() {
        let shape = shape.as_ref().expect("the parallel read produced a shape");
        assert!(!shape.is_null());
        let mass = volume_mass(shape);
        assert!(
            (mass - reference_mass).abs() <= 1.0e-6,
            "Shape {index} mass {mass} != reference {reference_mass}"
        );
    }
}

#[test]
fn BRepTools_Test__ParallelRead_RepeatedRuns() {
    let bytes = Arc::new(ascii_box_bytes());
    let reference_shape = read_shape_buffer(&bytes).expect("single-threaded read succeeds");
    let reference_mass = volume_mass(&reference_shape);
    let epsilon = 1.0e-6;

    for run in 0..5 {
        let shapes = parallel_shapes(Arc::clone(&bytes), 100);
        assert!(shapes.iter().all(Option::is_some), "Run {run}: read failed");
        for (index, shape) in shapes.iter().enumerate() {
            let shape = shape.as_ref().expect("the parallel read produced a shape");
            assert!(!shape.is_null());
            let mass = volume_mass(shape);
            assert!(
                (mass - reference_mass).abs() <= epsilon,
                "Run {run}, shape {index}, mass mismatch"
            );
        }
    }
}

fn round_trip(shape: &Shape, binary: bool, triangles: bool, version: i32) -> Shape {
    let mut stream = IoStream::from_bytes(Vec::new());
    if binary {
        bintools::write(
            shape,
            &mut stream,
            BinaryWriteOptions {
                with_triangles: triangles,
                with_normals: false,
                version,
            },
            &ProgressRange::default(),
        )
        .unwrap();
    } else {
        brep_tools::write_with_options(
            shape,
            &mut stream,
            BRepWriteOptions {
                with_triangles: triangles,
                with_normals: false,
                version,
            },
            &ProgressRange::default(),
        )
        .unwrap();
    }
    assert!(stream.good());
    let bytes = stream_bytes(stream);
    assert!(!bytes.is_empty());
    let mut stream = IoStream::from_bytes(bytes);
    let mut restored = Shape::default();
    if binary {
        bintools::read(&mut restored, &mut stream, &ProgressRange::default()).unwrap();
    } else {
        brep_tools::read(
            &mut restored,
            &mut stream,
            &BRepBuilder::default(),
            &ProgressRange::default(),
        )
        .unwrap();
    }
    assert!(!restored.is_null());
    restored
}

fn subshape_count(shape: &Shape, kind: ShapeEnum) -> usize {
    Explorer::new(shape, kind, ShapeEnum::Shape)
        .unwrap()
        .count()
}

fn triangle_count(shape: &Shape) -> i32 {
    Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .map(|face| {
            let face = martensite_tkbrep::topods::face(&face).unwrap();
            BRepTool::triangulation(&face, mesh_purpose::NONE)
                .unwrap()
                .0
                .get()
                .map_or(0, |triangulation| triangulation.nb_triangles())
        })
        .sum()
}

fn expect_equivalent_shape(expected: &Shape, actual: &Shape, triangles: i32) {
    assert!(!expected.is_null());
    assert!(!actual.is_null());
    assert_eq!(actual.shape_type().unwrap(), expected.shape_type().unwrap());
    for kind in [
        ShapeEnum::Solid,
        ShapeEnum::Shell,
        ShapeEnum::Face,
        ShapeEnum::Edge,
        ShapeEnum::Vertex,
    ] {
        assert_eq!(subshape_count(actual, kind), subshape_count(expected, kind));
    }
    assert_eq!(triangle_count(actual), triangles);
    let mut expected_volume = GProps::default();
    let mut actual_volume = GProps::default();
    BRepGProp::volume_properties(expected, &mut expected_volume, false, false, false).unwrap();
    BRepGProp::volume_properties(actual, &mut actual_volume, false, false, false).unwrap();
    assert!((actual_volume.mass() - expected_volume.mass()).abs() <= 1.0e-7);
    let mut expected_surface = GProps::default();
    let mut actual_surface = GProps::default();
    BRepGProp::surface_properties(expected, &mut expected_surface, false, false).unwrap();
    BRepGProp::surface_properties(actual, &mut actual_surface, false, false).unwrap();
    assert!((actual_surface.mass() - expected_surface.mass()).abs() <= 1.0e-7);
}

#[test]
fn BRepTools_Test__ModDataBug_29723_FormatVersions() {
    let mut maker =
        martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::from_sizes(10.0, 20.0, 30.0).unwrap();
    let source = maker.shape().unwrap().clone();
    let mut converter = BRepBuilderAPINurbsConvert::with_shape(&source, false).unwrap();
    let nurbs: Shape = converter.shape().unwrap().clone();
    assert!(!nurbs.is_null());
    let versions = [
        (false, ascii_version::VERSION_1),
        (false, ascii_version::VERSION_2),
        (true, 1),
        (true, 2),
        (true, 3),
    ];
    for (binary, version) in versions {
        let restored = round_trip(&nurbs, binary, true, version);
        expect_equivalent_shape(&nurbs, &restored, 0);
    }
}

#[test]
fn BRepTools_Test__DemoBinPersist_1_BinaryStreamRoundTrip() {
    let mut maker = martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox::at_point(
        martensite_tkmath::gp::Point3::new(10.0, 20.0, 30.0),
        100.0,
        200.0,
        300.0,
    )
    .unwrap();
    let source: Shape = maker.shape().unwrap().clone();
    assert!(!source.is_null());

    let mut stream = IoStream::from_bytes(Vec::new());
    bintools::write(
        &source,
        &mut stream,
        BinaryWriteOptions::default(),
        &ProgressRange::default(),
    )
    .unwrap();
    assert!(stream.good());
    let bytes = stream_bytes(stream);
    assert!(!bytes.is_empty());
    let mut stream = IoStream::from_bytes(bytes);
    let mut restored = Shape::default();
    bintools::read(&mut restored, &mut stream, &ProgressRange::default()).unwrap();
    assert!(!restored.is_null());

    let mut source_bounds = martensite_tkmath::bnd::BoundingBox3::default();
    let mut restored_bounds = martensite_tkmath::bnd::BoundingBox3::default();
    martensite_tktopalgo::brep_bnd_lib::BRepBndLib::add(&source, &mut source_bounds, true).unwrap();
    martensite_tktopalgo::brep_bnd_lib::BRepBndLib::add(&restored, &mut restored_bounds, true)
        .unwrap();
    let restored_bounds = restored_bounds.get().unwrap();
    let source_bounds = source_bounds.get().unwrap();
    for index in 0..6 {
        assert!((restored_bounds[index] - source_bounds[index]).abs() <= 1.0e-9);
    }
    assert!(
        martensite_tktopalgo::brep_check::BRepCheckAnalyzer::new(&restored, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    for kind in [
        ShapeEnum::Solid,
        ShapeEnum::Shell,
        ShapeEnum::Face,
        ShapeEnum::Edge,
        ShapeEnum::Vertex,
    ] {
        assert_eq!(
            subshape_count(&restored, kind),
            subshape_count(&source, kind)
        );
    }
    for (kind, expected) in [(0, 4800.0_f64), (1, 220000.0), (2, 6000000.0)] {
        let mut original = GProps::default();
        let mut copy = GProps::default();
        match kind {
            0 => {
                BRepGProp::linear_properties(&source, &mut original, false, false).unwrap();
                BRepGProp::linear_properties(&restored, &mut copy, false, false).unwrap();
            }
            1 => {
                BRepGProp::surface_properties(&source, &mut original, false, false).unwrap();
                BRepGProp::surface_properties(&restored, &mut copy, false, false).unwrap();
            }
            _ => {
                BRepGProp::volume_properties(&source, &mut original, false, false, false).unwrap();
                BRepGProp::volume_properties(&restored, &mut copy, false, false, false).unwrap();
            }
        }
        assert!((copy.mass() - original.mass()).abs() <= 1.0e-7);
        // #247: a relative epsilon bound permits more than four representable steps.
        crate::floating::expect_double_eq(original.mass(), expected);
    }
}

#[cfg(test)]
mod persistence_mass_rounding_tests {
    use crate::floating::expect_double_eq;

    #[test]
    fn closed_form_masses_accept_four_steps_in_both_directions() {
        for expected in [4800.0_f64, 220000.0, 6000000.0] {
            for bits in [expected.to_bits() - 4, expected.to_bits() + 4] {
                expect_double_eq(f64::from_bits(bits), expected);
            }
        }
    }

    #[test]
    fn closed_form_masses_reject_five_steps_in_both_directions() {
        for expected in [4800.0_f64, 220000.0, 6000000.0] {
            for bits in [expected.to_bits() - 5, expected.to_bits() + 5] {
                let comparison = || expect_double_eq(f64::from_bits(bits), expected);
                assert!(std::panic::catch_unwind(comparison).is_err());
            }
        }
    }

    #[test]
    fn closed_form_masses_reject_nonfinite_results() {
        for expected in [4800.0_f64, 220000.0, 6000000.0] {
            for actual in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let comparison = || expect_double_eq(actual, expected);
                assert!(std::panic::catch_unwind(comparison).is_err());
            }
        }
    }
}
