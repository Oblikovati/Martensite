//! Name conveniences retain UTF-8 spans; exact byte names have a separate stream interface.
use martensite_tkbool::top_ope_brep_build::{
    TopOpeBRepBuildPlainShapeSet, TopOpeBRepBuildShapeSet,
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

#[test]
fn utf8_name_conveniences_keep_full_spans_including_embedded_nul() -> Result<(), Failure> {
    let mut set = TopOpeBRepBuildPlainShapeSet::new(ShapeEnum::Vertex, false)?;
    set.set_deb_number(-7);
    for name in ["", "plain", "é", "left\0right"] {
        set.set_deb_name(name);
        assert_eq!(set.deb_name(), name);
        let mut output = Vec::new();
        set.dump_name(&mut output, "P\0Q")
            .expect("Vec writes cannot fail");
        assert_eq!(output, format!("P\0Q({name},-7)").into_bytes());
    }
    Ok(())
}

#[path = "shape_set_name_bytes/exact.rs"]
mod exact;
