//! Show a document's shape labels in a view and read back the saved view.
use super::document_view_capture;
use super::support::document_viewer::{ViewerLifecycle, register_vulkan_factory};
use martensite_tkernel::quantity::Color;
use martensite_tkernel::standard::Handle;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tklcaf::tdata_std::{TDataStdGenericExtString as _, TDataStdName};
use martensite_tklcaf::tdf::{TdfChildIterator, TdfLabel, TdfTool};
use martensite_tkmath::toploc::Location;
use martensite_tkv3d::ais::AisInteractiveObject;
use martensite_tkv3d::prs_mgr::{
    PrsMgrPresentableObject as _, PrsMgrPresentableObjectMembers as _,
};
use martensite_tkxcaf::xcaf_doc::XCAFDocShapeTool;
use martensite_tkxcaf::xcaf_prs::XCAFPrsAISObject;
use std::collections::HashSet;

const SHADED: i32 = 1;
const PROBE_TOLERANCE: u8 = 8;
const PROBES: [(usize, usize); 11] = [
    (204, 204),
    (100, 100),
    (300, 100),
    (100, 300),
    (300, 300),
    (204, 100),
    (204, 300),
    (100, 204),
    (300, 204),
    (50, 50),
    (360, 360),
];

/// How one document is walked: whether assemblies open into their components.
pub struct DocumentDisplay {
    explore: bool,
    names: HashSet<String>,
    shown: Vec<String>,
}

impl DocumentDisplay {
    pub fn new(explore: bool) -> Self {
        Self {
            explore,
            names: HashSet::new(),
            shown: Vec::new(),
        }
    }

    /// Display every free shape shaded; the names of the displayed objects.
    /// Example: `DocumentDisplay::new(true).show_free_shapes(&lifecycle, "D", &tool)`.
    pub fn show_free_shapes(
        mut self,
        lifecycle: &ViewerLifecycle,
        document: &str,
        tool: &XCAFDocShapeTool,
    ) -> Vec<String> {
        for label in tool.get_free_shapes().unwrap() {
            self.show_label(
                lifecycle,
                &label,
                &format!("{document}:"),
                &Location::default(),
            );
        }
        self.shown
    }

    fn show_label(
        &mut self,
        lifecycle: &ViewerLifecycle,
        label: &TdfLabel,
        prefix: &str,
        parent: &Location,
    ) {
        let name = format!("{prefix}{}", self.unique_name(label));
        let mut referred = label.clone();
        XCAFDocShapeTool::get_referred_shape(label, &mut referred).unwrap();
        if self.explore && XCAFDocShapeTool::is_assembly(&referred).unwrap() {
            let location = parent
                .multiplied(&XCAFDocShapeTool::get_location(label).unwrap())
                .unwrap();
            let mut children = TdfChildIterator::new(&referred, false).unwrap();
            while children.more() {
                self.show_label(lifecycle, &children.value(), &format!("{name}/"), &location);
                children.next();
            }
            return;
        }
        display_leaf(lifecycle, label, parent, &name);
        self.shown.push(name);
    }

    /// The label's name or its definition's, else its entry, numbered on a collision.
    fn unique_name(&mut self, label: &TdfLabel) -> String {
        let base = label_name(label);
        let mut name = base.clone();
        let mut index = 1;
        while !self.names.insert(name.clone()) {
            name = format!("{base}_{index}");
            index += 1;
        }
        name
    }
}

fn label_name(label: &TdfLabel) -> String {
    if let Some(own) = stored_name(label) {
        return own;
    }
    let mut referred = TdfLabel::default();
    if XCAFDocShapeTool::get_referred_shape(label, &mut referred).unwrap()
        && let Some(definition) = stored_name(&referred)
    {
        return definition;
    }
    let mut entry = AsciiString::default();
    TdfTool::entry(label, &mut entry);
    String::from_utf8_lossy(entry.as_bytes()).into_owned()
}

fn stored_name(label: &TdfLabel) -> Option<String> {
    let mut name = Handle::<TDataStdName>::default();
    let found = label
        .find_attribute_as(&TDataStdName::get_id(), &mut name)
        .unwrap();
    let text = found.then(|| name.get().unwrap().get().to_string())?;
    (!text.is_empty()).then_some(text)
}

fn display_leaf(lifecycle: &ViewerLifecycle, label: &TdfLabel, location: &Location, name: &str) {
    let presentation = XCAFPrsAISObject::new(label).unwrap();
    let object = presentation.get().unwrap();
    if !location.is_identity() {
        object
            .set_local_transformation(&location.transformation())
            .unwrap();
    }
    assert!(object.accept_display_mode(SHADED));
    object.set_display_mode(SHADED);
    let shown: Handle<dyn AisInteractiveObject> = presentation.cast();
    let context = lifecycle.interactive_context();
    context.get().unwrap().display(&shown, false).unwrap();
    lifecycle
        .state()
        .map_of_ais
        .borrow_mut()
        .bind(shown, AsciiString::from(name))
        .unwrap();
}

/// A first view for one document, on a registered renderer; `let lifecycle = opened_view();`.
pub fn opened_view() -> ViewerLifecycle {
    register_vulkan_factory();
    let lifecycle = ViewerLifecycle::new(true);
    assert_eq!(
        lifecycle.initialize("Driver1/Viewer1/View1").unwrap(),
        "Driver1/Viewer1/View1"
    );
    lifecycle
}

/// Show what was displayed without updating; `redraw_document(&lifecycle);`.
pub fn redraw_document(lifecycle: &ViewerLifecycle) {
    lifecycle
        .interactive_context()
        .get()
        .unwrap()
        .update_current_viewer();
}

/// Fit the view to everything shown; `fit_view(&lifecycle);`.
pub fn fit_view(lifecycle: &ViewerLifecycle) {
    let view = lifecycle.current_view();
    let view_ref = view.get().unwrap();
    view_ref.fit_all(0.01, false).unwrap();
    view_ref.redraw();
}

/// Save the view and read the file back as one 409 by 409 PPM with something drawn.
/// Example: `let image = save_view(&lifecycle, "pencil");`.
pub fn save_view(lifecycle: &ViewerLifecycle, case: &str) -> SavedView {
    let bytes = document_view_capture::read_case_saved_view(&lifecycle.current_view(), case);
    SavedView::read(bytes, case)
}

/// The saved image; the probes read a pixel in the capture's top-down coordinates.
pub struct SavedView {
    bytes: Vec<u8>,
}

const HEADER: &[u8] = b"P6\n409 409\n255\n";
const SIDE: usize = 409;

impl SavedView {
    fn read(bytes: Vec<u8>, case: &str) -> Self {
        assert_eq!(&bytes[..HEADER.len()], HEADER, "{case}: PPM header");
        assert_eq!(
            bytes.len(),
            HEADER.len() + 3 * SIDE * SIDE,
            "{case}: PPM size"
        );
        let first = &bytes[HEADER.len()..HEADER.len() + 3];
        assert!(
            bytes[HEADER.len()..]
                .as_chunks::<3>()
                .0
                .iter()
                .any(|pixel| pixel.as_slice() != first),
            "{case}: the view holds a single color"
        );
        preserve_requested_artifact(&bytes, case);
        Self { bytes }
    }

    /// The sRGB bytes of a pixel, top row first; `self.pixel(204, 204)`.
    fn pixel(&self, x: usize, y: usize) -> [u8; 3] {
        let start = HEADER.len() + 3 * (y * SIDE + x);
        [
            self.bytes[start],
            self.bytes[start + 1],
            self.bytes[start + 2],
        ]
    }

    /// Assert one color per probe, within `PROBE_TOLERANCE` per channel for a hardware driver's
    /// rounding. Every probe is well inside a face or the background, away from antialiased edges.
    /// Example: `image.assert_probes([[0, 0, 0]; 11])`.
    pub fn assert_probes(&self, expected: [[u8; 3]; PROBES.len()]) {
        for ((x, y), color) in PROBES.into_iter().zip(expected) {
            let actual = self.pixel(x, y);
            let close = actual
                .iter()
                .zip(&color)
                .all(|(left, right)| left.abs_diff(*right) <= PROBE_TOLERANCE);
            assert!(close, "pixel {x},{y}: {actual:?} against {color:?}");
        }
    }
}

fn preserve_requested_artifact(bytes: &[u8], case: &str) {
    let Some(directory) = std::env::var_os("MARTENSITE_DOCUMENT_ARTIFACT_DIR") else {
        return;
    };
    std::fs::create_dir_all(&directory).unwrap();
    let file = std::path::Path::new(&directory).join(format!("{case}.ppm"));
    std::fs::write(file, bytes).unwrap();
}

/// The name of the color nearest to a stored one; `named(tool.get_color(&label)?)`.
pub fn named(color: Option<Color>) -> Option<&'static str> {
    color.map(|value| Color::string_name(value.name()))
}
