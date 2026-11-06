//! The `bugs/modalg_2` scripts that branch on what a command answers.
use crate::support::{
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    construction::copy,
    feat_script::FeatScript,
    properties::Prop,
    restore::restore,
    splitshape::splitshape,
};
use martensite_tkg3d::top_abs::ShapeEnum;

pub fn bug22986(s: &mut FeatScript<'_>) {
    println!("===========");
    println!("22986");
    println!("===========");
    println!();
    restore(s, "CFE900_cts21gbs.rle", "base");
    restore(s, "cts21322_face.rle", "wire");
    explode(s, "base", Some(ShapeEnum::Face));
    copy(s, "base_6", "fsketch");
    splitshape(s, "result", "fsketch", &["fsketch", "wire"], &[]);
    if explode(s, "result", Some(ShapeEnum::Face)).len() != 3 {
        println!("Faulty 22986 : The result of splitshape command is incorrect.");
    } else {
        println!("22986  OK");
    }
    checkprops(s, "result", Prop::Area, Expected::Mass(414.));
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}
