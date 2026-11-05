//! `bugs_modalg_8`: each statement is one line of its script.
use crate::support::{
    checkview::{CheckView, checkview},
    commands::{pcylinder, psphere, ttranslate},
    nbshapes::{Counting, NbCount, checknbshapes},
    one_step::bcut,
    session::Session,
};

pub use super::bugs_modalg_8_hand_2::*;
pub use super::bugs_modalg_8_hand_9::*;
pub use super::bugs_modalg_8_hand_10::*;

pub fn bug33541(s: &mut dyn Session) {
    println!("================================");
    println!("0033541: Modeling Algorithms - Simple sphere cut from cylinder fails");
    println!("================================");
    println!();
    pcylinder(s, "c", 1., 1.);
    psphere(s, "s", 1.);
    ttranslate(s, "s", [1., 1., 1.]);
    bcut(s, "result", "c", "s");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(3),
            NbCount::Edges(5),
            NbCount::Wires(4),
            NbCount::Faces(4),
        ],
    );
    checkview(s, &CheckView::of("result"));
}

pub fn bug33570(s: &mut dyn Session) {
    println!("================================");
    println!("0033570: Modeling Algorithms - Incorrect result of cutting spheres");
    println!("================================");
    println!();
    psphere(s, "s1", 4.);
    psphere(s, "s2", 1.);
    ttranslate(s, "s1", [0., 5.5, 18.]);
    ttranslate(s, "s2", [0., 4.5, 14.]);
    bcut(s, "result", "s1", "s2");
    checknbshapes(
        s,
        "result",
        Counting::Located,
        &[
            NbCount::Vertices(4),
            NbCount::Edges(7),
            NbCount::Wires(2),
            NbCount::Faces(2),
        ],
    );
    checkview(s, &CheckView::of("result"));
}
