//! The `bfuse_complex` case bodies, part 5; each statement is one line of its script.
use super::*;

pub fn S2(s: &mut dyn Session) {
    println!("hkg60159");
    println!("Fuse");
    println!();
    restore(s, "CFE900_hkg60gcr.rle", "sh");
    restore(s, "hkg60159_face.rle", "ct");
    prism(s, "pr", "ct", [0., -30., 0.]);
    bcut(s, "res", "pr", "sh");
    explode(s, "res", None);
    bfuse(s, "res2", "sh", "res_3");
    bfuse(s, "result", "sh", "res_2");
    checkprops(s, "result", Prop::Area, Expected::Mass(3734.83));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["sh", "res_2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "pr", Prop::Area, "906");
    kind_is(s, "pr", ShapeEnum::Solid);
    printed_mass(s, "sh", Prop::Area, "3535.42");
    kind_is(s, "sh", ShapeEnum::Solid);
    printed_mass(s, "res_3", Prop::Area, "363.308");
    kind_is(s, "res_3", ShapeEnum::Solid);
    printed_mass(s, "res_2", Prop::Area, "339.918");
    kind_is(s, "res_2", ShapeEnum::Solid);
    part_count(s, "res", None, 3);
    view_mass_is_nonzero(s, "result");
}

pub fn S3(s: &mut dyn Session) {
    println!("PRO15203");
    println!("Fuse");
    println!();
    restore(s, "pro15203b.rle", "c1");
    restore(s, "pro15203c.rle", "c2");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    bfuse(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "c1", Prop::Area, "285.023");
    kind_is(s, "c1", ShapeEnum::Face);
    printed_mass(s, "c2", Prop::Area, "285.023");
    kind_is(s, "c2", ShapeEnum::Face);
    part_count(s, "c1", Some(ShapeEnum::Face), 0);
    part_count(s, "c2", Some(ShapeEnum::Face), 0);
    view_mass_is_nonzero(s, "result");
}

pub fn S4(s: &mut dyn Session) {
    println!("PRO15203");
    println!("Fuse");
    println!();
    restore(s, "pro15203b.rle", "c1");
    restore(s, "pro15203d.rle", "c2");
    explode(s, "c1", Some(ShapeEnum::Face));
    copy(s, "c1_1", "c1");
    explode(s, "c2", Some(ShapeEnum::Face));
    copy(s, "c2_1", "c2");
    bfuse(s, "result", "c1", "c2");
    checkprops(s, "result", Prop::Area, Expected::Mass(570.047));
    checkview(
        s,
        &CheckView::of("result").otherwise(&["c1", "c2"], Prop::Area),
    );
    // Beyond the script: checks an earlier test of this case made.
    printed_mass(s, "c1", Prop::Area, "285.023");
    kind_is(s, "c1", ShapeEnum::Face);
    printed_mass(s, "c2", Prop::Area, "285.023");
    kind_is(s, "c2", ShapeEnum::Face);
    part_count(s, "c1", Some(ShapeEnum::Face), 0);
    part_count(s, "c2", Some(ShapeEnum::Face), 0);
    view_mass_is_nonzero(s, "result");
}
