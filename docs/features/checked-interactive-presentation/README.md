# Checked Interactive Window Presentation Without Mandatory Readback (#501)

## Public Presentation Path

Interactive window views use `Graphic3dRenderView::try_present()` (or `try_redraw()`) with
`RenderGuiSurfacePolicy::CheckedV2`.

- **No per-frame CPU readback:** `VulkanRenderer::submit_prepared_gui_window` keeps ordinary
  interactive frames in flight via `FrameResources::readback_ticket()` and `launch_frame(...)`.
  It skips synchronous fence waits and CPU color/depth readback unless the frame explicitly
  requires host completion feedback (raytracing feedback, window depth status, or the
  `command-state-diagnostics` feature).
- **Checked GUI & surface validation preserved:** `CheckedV2` still validates surface extent,
  target revision, and GUI composition revision at acquire, pre-submit, present, and publication
  checkpoints (`WindowGuiContract`).
- **Explicit on-demand capture:**
  - `Graphic3dRenderView::capture_color()` reads only the color plane for screenshots without
    transferring the depth buffer.
  - `Graphic3dRenderView::capture()` reads both color and depth planes when picking or full
    diagnostics require depth.
  - `Graphic3dRenderView::dump_publication()` streams RGB rows directly from the device when
    applicable.
- **Resize and fault safety:** Failed presentations or out-of-date swapchains cancel assembly
  streaming, retire the failed presenter chain, and surface the primary `RenderError` with
  typed `RenderGuiSurfaceChange` details on `try_present()` / `try_redraw()`.
