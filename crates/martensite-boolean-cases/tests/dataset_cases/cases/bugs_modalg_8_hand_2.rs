//! `bugs_modalg_8`: linear scripts in the commands of `support::bugs_commands`; each statement is one
//! line of its script.
use crate::support::{
    bugs_commands::beyond::child_count,
    bugs_commands::inspect::checkmaxtol_min,
    bugs_commands::repair::{sewing, ssolid},
    checkprops::{Expected, checkprops},
    checkshape::checkshape_named,
    checkview::{CheckView, checkview},
    commands::explode,
    construction::{edge, mkplane, vertex, wire},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::{bcommon, bfuse},
    properties::Prop,
    restore::restore,
    session::Session,
};

pub fn bug32942(s: &mut dyn Session) {
    println!("=========================================================================");
    println!(
        "OCC32942: Modeling Algorithms - Boolean operation Common has no result in specific example"
    );
    println!("=========================================================================");
    println!();
    restore(s, "bug32942.brep", "a");
    explode(s, "a", None);
    bcommon(s, "result", "a_1", "a_2");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkmaxtol_min(s, "result", 1.0e-5, "1.e-5");
    checkprops(s, "result", Prop::Area, Expected::Mass(21746.8));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "a", 2);
}

pub fn bug32943(s: &mut dyn Session) {
    println!("=========================================================================");
    println!(
        "OCC32943: Modeling Algorithms - Boolean operation Common has no result in specific example"
    );
    println!("=========================================================================");
    println!();
    restore(s, "bug32943.brep", "a");
    explode(s, "a", None);
    bcommon(s, "result", "a_1", "a_2");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkmaxtol_min(s, "result", 1.0e-5, "1.e-5");
    checkprops(s, "result", Prop::Area, Expected::Mass(17615.5));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "a", 2);
}

pub fn bug32944(s: &mut dyn Session) {
    println!("=========================================================================");
    println!(
        "OCC32944: Modeling Algorithms - Boolean operation Common has no result in specific example"
    );
    println!("=========================================================================");
    println!();
    restore(s, "bug32944.brep", "a");
    explode(s, "a", None);
    bcommon(s, "result", "a_1", "a_2");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkmaxtol_min(s, "result", 1.0e-5, "1.e-5");
    checkprops(s, "result", Prop::Area, Expected::Mass(36741.6));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "a", 2);
}

pub fn bug32945(s: &mut dyn Session) {
    println!("=========================================================================");
    println!(
        "OCC32945: Modeling Algorithms - Boolean operation Common has no result in specific example"
    );
    println!("=========================================================================");
    println!();
    restore(s, "bug32945.brep", "a");
    explode(s, "a", None);
    bcommon(s, "result", "a_1", "a_2");
    checkshape_named(s, "result");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(8),
            NbCount::Edges(12),
            NbCount::Wires(6),
            NbCount::Faces(6),
            NbCount::Shells(1),
            NbCount::Solids(1),
        ],
    );
    checkmaxtol_min(s, "result", 1.0e-5, "1.e-5");
    checkprops(s, "result", Prop::Area, Expected::Mass(83754.8));
    checkview(s, &CheckView::of("result"));
    // Beyond the script: checks an earlier test of this case made.
    child_count(s, "a", 2);
}

pub fn bug33361(s: &mut dyn Session) {
    println!("============");
    println!("0033361: Modeling Algorithm - Fuse operation generates incomplete result");
    println!("============");
    println!();
    vertex(
        s,
        "v1",
        [-13.1319809553115, 86.7822037750006, -0.00566973476865229],
    );
    vertex(
        s,
        "v2",
        [-128.558636709231, 86.5345485791916, -0.00570258843333704],
    );
    vertex(
        s,
        "v3",
        [-100.69915263562, -4.39311321186025, -0.00566084285433277],
    );
    vertex(
        s,
        "v4",
        [-79.6175928285065, -4.39310160958954, -0.0056548256445943],
    );
    vertex(
        s,
        "v5",
        [-14.129603767474, 86.2679145189996, -19.5413698283793],
    );
    vertex(
        s,
        "v6",
        [-127.672333074639, 86.0243014066245, -19.5414021458256],
    );
    vertex(
        s,
        "v7",
        [-99.9691824161271, -4.39312007067491, -19.5413606345018],
    );
    vertex(
        s,
        "v8",
        [-80.240198192502, -4.39310921279638, -19.5413550033513],
    );
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    edge(s, "e5", "v1", "v5");
    edge(s, "e6", "v2", "v6");
    edge(s, "e7", "v3", "v7");
    edge(s, "e8", "v4", "v8");
    edge(s, "e9", "v5", "v6");
    edge(s, "e10", "v6", "v7");
    edge(s, "e11", "v7", "v8");
    edge(s, "e12", "v8", "v5");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    wire(s, "w2", &["e1", "e6", "e9", "e5"]);
    wire(s, "w3", &["e2", "e7", "e10", "e6"]);
    wire(s, "w4", &["e3", "e8", "e11", "e7"]);
    wire(s, "w5", &["e4", "e5", "e12", "e8"]);
    wire(s, "w6", &["e9", "e10", "e11", "e12"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    mkplane(s, "f4", "w4");
    mkplane(s, "f5", "w5");
    mkplane(s, "f6", "w6");
    sewing(s, "s1", 1.0e-6, &["f1", "f2", "f3", "f4", "f5", "f6"]);
    ssolid(s, "s1", "s1");
    vertex(
        s,
        "v1",
        [127.874808332641, 87.0848242748098, -0.00573631640736139],
    );
    vertex(
        s,
        "v2",
        [4.13755363440343, 86.8193381629493, -0.00577153550167253],
    );
    vertex(
        s,
        "v3",
        [87.7714933163724, -4.39312459253253, -0.00571376462476536],
    );
    vertex(
        s,
        "v4",
        [108.964655532952, -4.39311292884125, -0.00570771556087379],
    );
    vertex(
        s,
        "v5",
        [127.250964034433, 86.5713396058731, -19.5413363036311],
    );
    vertex(
        s,
        "v6",
        [5.24619819968578, 86.3095706585265, -19.5413710296105],
    );
    vertex(
        s,
        "v7",
        [88.4127315018899, -4.39313150014382, -19.5413135815986],
    );
    vertex(
        s,
        "v8",
        [108.446956291808, -4.39312047427561, -19.5413078633248],
    );
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    edge(s, "e5", "v1", "v5");
    edge(s, "e6", "v2", "v6");
    edge(s, "e7", "v3", "v7");
    edge(s, "e8", "v4", "v8");
    edge(s, "e9", "v5", "v6");
    edge(s, "e10", "v6", "v7");
    edge(s, "e11", "v7", "v8");
    edge(s, "e12", "v8", "v5");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    wire(s, "w2", &["e1", "e6", "e9", "e5"]);
    wire(s, "w3", &["e2", "e7", "e10", "e6"]);
    wire(s, "w4", &["e3", "e8", "e11", "e7"]);
    wire(s, "w5", &["e4", "e5", "e12", "e8"]);
    wire(s, "w6", &["e9", "e10", "e11", "e12"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    mkplane(s, "f4", "w4");
    mkplane(s, "f5", "w5");
    mkplane(s, "f6", "w6");
    sewing(s, "s2", 1.0e-6, &["f1", "f2", "f3", "f4", "f5", "f6"]);
    ssolid(s, "s2", "s2");
    vertex(
        s,
        "v1",
        [108.933394491783, -4.52270499989385, -0.0678854291688111],
    );
    vertex(
        s,
        "v2",
        [71.6392583557696, -4.82783796975667, 0.111191412258037],
    );
    vertex(
        s,
        "v3",
        [71.3588792617103, 151.745015502843, 0.0366204967163988],
    );
    vertex(
        s,
        "v4",
        [127.845048470054, 86.9672826319557, -0.202979251077327],
    );
    vertex(
        s,
        "v5",
        [109.955308180154, -5.58542764710532, 46.9881961078522],
    );
    vertex(
        s,
        "v6",
        [70.0688762581537, -5.90713971196433, 46.9762651891041],
    );
    vertex(
        s,
        "v7",
        [69.7567011034265, 151.061981157248, 47.828071244823],
    );
    vertex(
        s,
        "v8",
        [128.441716305279, 83.7679782847199, 47.4778456953173],
    );
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    edge(s, "e5", "v1", "v5");
    edge(s, "e6", "v2", "v6");
    edge(s, "e7", "v3", "v7");
    edge(s, "e8", "v4", "v8");
    edge(s, "e9", "v5", "v6");
    edge(s, "e10", "v6", "v7");
    edge(s, "e11", "v7", "v8");
    edge(s, "e12", "v8", "v5");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    wire(s, "w2", &["e1", "e6", "e9", "e5"]);
    wire(s, "w3", &["e2", "e7", "e10", "e6"]);
    wire(s, "w4", &["e3", "e8", "e11", "e7"]);
    wire(s, "w5", &["e4", "e5", "e12", "e8"]);
    wire(s, "w6", &["e9", "e10", "e11", "e12"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    mkplane(s, "f4", "w4");
    mkplane(s, "f5", "w5");
    mkplane(s, "f6", "w6");
    sewing(s, "s3", 1.0e-6, &["f1", "f2", "f3", "f4", "f5", "f6"]);
    ssolid(s, "s3", "s3");
    vertex(
        s,
        "v1",
        [-122.825413548196, 67.8222420317801, -0.00583425335077337],
    );
    vertex(
        s,
        "v2",
        [121.414994525437, 55.8353479065558, -0.00563439528369258],
    );
    vertex(
        s,
        "v3",
        [127.874867553719, 87.0848594871004, -0.00564196037409204],
    );
    vertex(
        s,
        "v4",
        [-128.558663169796, 86.5344601820726, -0.00584644818335889],
    );
    vertex(
        s,
        "v5",
        [-122.343446023918, 68.6316879811904, -19.5414411166127],
    );
    vertex(
        s,
        "v6",
        [121.07295954832, 56.6852348873057, -19.5412518461508],
    );
    vertex(
        s,
        "v7",
        [127.250997928568, 86.5713737315812, -19.5412353309277],
    );
    vertex(
        s,
        "v8",
        [-127.67235532236, 86.0242158147956, -19.5414386144761],
    );
    edge(s, "e1", "v1", "v2");
    edge(s, "e2", "v2", "v3");
    edge(s, "e3", "v3", "v4");
    edge(s, "e4", "v4", "v1");
    edge(s, "e5", "v1", "v5");
    edge(s, "e6", "v2", "v6");
    edge(s, "e7", "v3", "v7");
    edge(s, "e8", "v4", "v8");
    edge(s, "e9", "v5", "v6");
    edge(s, "e10", "v6", "v7");
    edge(s, "e11", "v7", "v8");
    edge(s, "e12", "v8", "v5");
    wire(s, "w1", &["e1", "e2", "e3", "e4"]);
    wire(s, "w2", &["e1", "e6", "e9", "e5"]);
    wire(s, "w3", &["e2", "e7", "e10", "e6"]);
    wire(s, "w4", &["e3", "e8", "e11", "e7"]);
    wire(s, "w5", &["e4", "e5", "e12", "e8"]);
    wire(s, "w6", &["e9", "e10", "e11", "e12"]);
    mkplane(s, "f1", "w1");
    mkplane(s, "f2", "w2");
    mkplane(s, "f3", "w3");
    mkplane(s, "f4", "w4");
    mkplane(s, "f5", "w5");
    mkplane(s, "f6", "w6");
    sewing(s, "s4", 1.0e-6, &["f1", "f2", "f3", "f4", "f5", "f6"]);
    ssolid(s, "s4", "s4");
    bfuse(s, "result", "s1", "s2");
    bfuse(s, "result", "result", "s3");
    bfuse(s, "result", "result", "s4");
    checkshape_named(s, "result");
    checkview(s, &CheckView::of("result"));
}
