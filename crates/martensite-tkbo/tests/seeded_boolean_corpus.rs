// SPDX-License-Identifier: GPL-2.0-only
//! Seeded Boolean regression corpus (seed 20261014) with independent per-face
//! checks (validity, surface type, face count, and volume inclusion-exclusion) (#3, #40).
use martensite_tkbo::brep_algo_api::{BRepAlgoAPICommon, BRepAlgoAPICut, BRepAlgoAPIFuse};
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkbrep::topods::{self, Shape};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::SurfaceType;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;
use martensite_tktopalgo::brep_gprop::BRepGProp;

const CORPUS_SEED: u64 = 20261014;

struct XorShift64(u64);

impl XorShift64 {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn next_unit(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }

    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_unit()
    }
}

fn make_box(min: [f64; 3], size: [f64; 3]) -> Shape {
    BRepPrimAPIMakeBox::from_points(
        Point3::new(min[0], min[1], min[2]),
        Point3::new(min[0] + size[0], min[1] + size[1], min[2] + size[2]),
    )
    .expect("make box")
    .shape()
    .expect("box shape")
    .clone()
}

fn volume_of(shape: &Shape) -> f64 {
    let mut props = GProps::default();
    BRepGProp::volume_properties(shape, &mut props, true, false, false).expect("volume properties");
    props.mass()
}

fn verify_planar_solid_faces(shape: &Shape, label: &str) {
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false).expect("analyzer");
    assert!(
        analyzer.is_valid().expect("is_valid"),
        "{label}: BRepCheckAnalyzer failed"
    );

    let mut face_count = 0usize;
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape).expect("explorer");
    while explorer.more() {
        let face = topods::face(explorer.current()).expect("face");
        let adaptor = BRepAdaptorSurface::with_face(&face, true).expect("surface adaptor");
        assert_eq!(
            adaptor.get_type().expect("surface type"),
            SurfaceType::Plane,
            "{label}: expected planar face"
        );
        let mut props = GProps::default();
        BRepGProp::surface_properties(&face, &mut props, false, false)
            .expect("face surface properties");
        assert!(
            props.mass() > 1e-9,
            "{label}: face {face_count} has non-positive area {}",
            props.mass()
        );
        face_count += 1;
        explorer.next().expect("next face");
    }
    assert!(
        face_count >= 6,
        "{label}: expected >= 6 faces, got {face_count}"
    );
}

#[test]
fn seeded_boolean_corpus_satisfies_per_face_and_volume_invariants() {
    let mut rng = XorShift64(CORPUS_SEED);

    for case in 0..8 {
        let a_size = [
            rng.range(1.5, 3.0),
            rng.range(1.5, 3.0),
            rng.range(1.5, 3.0),
        ];
        let b_min = [
            rng.range(0.3, 1.0),
            rng.range(0.3, 1.0),
            rng.range(0.3, 1.0),
        ];
        let b_size = [
            rng.range(1.2, 2.5),
            rng.range(1.2, 2.5),
            rng.range(1.2, 2.5),
        ];

        let a = make_box([0.0, 0.0, 0.0], a_size);
        let b = make_box(b_min, b_size);

        let vol_a = a_size[0] * a_size[1] * a_size[2];
        let vol_b = b_size[0] * b_size[1] * b_size[2];

        let fuse = BRepAlgoAPIFuse::from_shapes(&a, &b).expect("fuse");
        assert!(fuse.is_done(), "case {case}: fuse failed");
        verify_planar_solid_faces(fuse.shape(), &format!("case {case} fuse"));

        let common = BRepAlgoAPICommon::from_shapes(&a, &b).expect("common");
        assert!(common.is_done(), "case {case}: common failed");
        verify_planar_solid_faces(common.shape(), &format!("case {case} common"));

        let cut = BRepAlgoAPICut::from_shapes(&a, &b).expect("cut");
        assert!(cut.is_done(), "case {case}: cut failed");
        verify_planar_solid_faces(cut.shape(), &format!("case {case} cut"));

        let vol_fuse = volume_of(fuse.shape());
        let vol_common = volume_of(common.shape());
        let vol_cut = volume_of(cut.shape());

        let inc_exc_err = ((vol_fuse + vol_common) - (vol_a + vol_b)).abs();
        assert!(
            inc_exc_err < 1e-6,
            "case {case}: inclusion-exclusion residual {inc_exc_err} >= 1e-6"
        );
        let cut_err = ((vol_cut + vol_common) - vol_a).abs();
        assert!(
            cut_err < 1e-6,
            "case {case}: cut residual {cut_err} >= 1e-6"
        );
    }
}
