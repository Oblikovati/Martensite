//! A Redraw override edits the caller's options while ToPixMap reads them, as native allows.
//! Rows equal the recorded image_dump_options_live capture (three native processes).
#![allow(unsafe_code)] // The light name borrows the anchored callback storage.
use super::light_name_callback_fixture::{CallbackNameStorage, CallbackObservedView};
use super::{
    V3dImageDumpOptions, V3dImageDumpOptionsSource, V3dLightName, V3dTypeOfView, V3dView,
    V3dViewSource, V3dViewer,
};
use martensite_tkernel::standard::{Handle, IntrusiveReferenceSlot};
use martensite_tkservice::{
    aspect::AspectNeutralWindow, graphic3d::Graphic3dBufferType, image::ImagePixMap,
    null_driver::NullGraphicDriver,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// The edit the Redraw override applies to the caller's options.
#[derive(Clone, Copy)]
enum LiveEdit {
    None,
    RebindName,
    SwitchBuffer,
}

/// Caller-owned options shared with the view's Redraw override; `options.current_options()`.
struct LiveOptions {
    control: Arc<CallbackNameStorage>,
    edit: LiveEdit,
    edited: Arc<AtomicBool>,
}
impl<'anchor> V3dImageDumpOptionsSource<'anchor> for LiveOptions {
    fn current_options(&self) -> V3dImageDumpOptions<'anchor> {
        // The anchored storage outlives every read of this test transaction.
        let mut options = V3dImageDumpOptions {
            width: 8,
            height: 8,
            buffer_type: Graphic3dBufferType::ShadowMap,
            // SAFETY: the pointer names a live NUL-terminated buffer that outlives this view.
            light_name: unsafe { V3dLightName::from_ptr(self.control.pointer()) },
            ..Default::default()
        };
        if !self.edited.load(Ordering::SeqCst) {
            return options;
        }
        match self.edit {
            LiveEdit::None => {}
            LiveEdit::RebindName => options.light_name = V3dLightName::from_c_str(c"rebound"),
            LiveEdit::SwitchBuffer => options.buffer_type = Graphic3dBufferType::Rgba,
        }
        options
    }
}

/// A view whose Redraw override marks the options edited before the base redraw.
struct LiveOptionsView {
    base: V3dView,
    edited: Arc<AtomicBool>,
}
martensite_tkernel::impl_transient!(LiveOptionsView,"Martensite_LiveOptionsView",V3dView,base,
    interfaces:[dyn V3dViewSource]);
impl V3dViewSource for LiveOptionsView {
    fn view_base(&self) -> &V3dView {
        &self.base
    }
    fn redraw(&self) {
        self.edited.store(true, Ordering::SeqCst);
        super::view_source_redraw::v3d_view_redraw(&self.base);
    }
}

fn live_view(
    viewer: &Handle<V3dViewer>,
    control: &Arc<CallbackNameStorage>,
    edited: &Arc<AtomicBool>,
) -> Handle<dyn V3dViewSource> {
    let mut base = V3dView::new_embedded(viewer, V3dTypeOfView::Orthographic).unwrap();
    base.driver_view_slot = IntrusiveReferenceSlot::from_owned(CallbackObservedView::create(
        &viewer.structure_manager(),
        control,
    ));
    let view = Handle::new(LiveOptionsView {
        base,
        edited: edited.clone(),
    })
    .cast();
    V3dView::bind_source(&view).unwrap();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_size(8, 8);
    view.set_window(&window.cast(), 0).unwrap();
    view
}

fn run_case(label: &str, edit: LiveEdit) -> String {
    let control = CallbackNameStorage::new();
    control.observe_buffer_dumps();
    let edited = Arc::new(AtomicBool::new(false));
    let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
    let view = live_view(&viewer, &control, &edited);
    // SetWindow redraws; native assigns the options only after it, so earlier edits are lost.
    edited.store(false, Ordering::SeqCst);
    control.overwrite(*b"sun\0\0\0\0\0");
    let options = LiveOptions {
        control: control.clone(),
        edit,
        edited,
    };
    let image = ImagePixMap::default();
    let done = view.to_pix_map(&image, &options).unwrap();
    let after = options.current_options();
    let name = after.light_name.to_ascii_string().unwrap();
    let mut rows = vec![format!("case {label}")];
    rows.extend(control.rows.read().iter().cloned());
    let mut result = format!(
        "result {} format {} buffer {} after {}",
        i32::from(done),
        image.format() as i32,
        after.buffer_type as i32,
        name.len()
    );
    for byte in name.as_bytes() {
        result.push_str(&format!(" {byte}"));
    }
    rows.push(result);
    rows.join("\n")
}

#[test]
fn redraw_edits_of_the_caller_options_reach_the_later_reads() {
    let actual = [
        run_case("unchanged", LiveEdit::None),
        run_case("rebind-name", LiveEdit::RebindName),
        run_case("switch-buffer", LiveEdit::SwitchBuffer),
    ]
    .join("\n")
        + "\n";
    assert_eq!(
        actual,
        include_str!("../../tests/fixtures/image-dump-live-options-native.txt")
    );
}
