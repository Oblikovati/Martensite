//! What the scripts mean to check and cannot, because of an input that is not public or a check of
//! theirs that stops early. These stand for no script.
use crate::session::commands::CafSession;
use crate::session::hooks::ImageCheck::Drawn;

fn displayed_box(s: &mut CafSession) {
    s.box_of_size("aBox1", 100, 200, 300);
    s.set_shape("0:2", "aBox1");
    s.ais_init_viewer();
    s.ais_set("0:2", "NS");
    s.ais_display("0:2");
}

caf_case!(
    selection_modes_through_undo_and_redo,
    "selection modes the presentation grid leaves unchecked",
    PRESENTATION,
    Drawn,
    |s| {
        s.new_command();
        displayed_box(s);
        s.new_command();
        assert_eq!(s.ais_sel_mode("0:2"), "");
        s.ais_set_sel_mode("0:2", &[2]);
        assert_eq!(s.ais_sel_mode("0:2"), "2");
        s.new_command();
        s.ais_set_sel_mode("0:2", &[2, 4]);
        assert_eq!(s.ais_sel_mode("0:2"), "2 4");
        s.new_command();
        s.undo();
        assert_eq!(s.ais_sel_mode("0:2"), "2");
        s.redo();
        assert_eq!(s.ais_sel_mode("0:2"), "2 4");
    }
);

// A document with this many sibling labels once overflowed the stack when it was released. The
// reference input is not public, so the shape of it is built here; this is not that script's input.
#[test]
fn a_document_with_80000_sublabels_is_released_without_a_stack_overflow() {
    let document = martensite_tklcaf::tdoc_std::TDocStdDocument::new(
        &martensite_tkernel::tcollection::ExtendedString::from_utf8("BinOcaf"),
    )
    .unwrap();
    let parent = document.get_data().root().find_child(2, true).unwrap();
    for tag in 1..=80_000 {
        parent.find_child(tag, true).unwrap();
    }
    assert_eq!(parent.nb_children().unwrap(), 80_000);
    drop(parent);
    drop(document);
}
