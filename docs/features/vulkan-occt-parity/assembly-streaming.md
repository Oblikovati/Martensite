# Progressive assembly uploads

A mapped Vulkan view can show an assembly before every part is uploaded. Set
`Graphic3dRenderView::set_assembly_streaming` to
`RenderAssemblyStreaming::VisibleBudgetedV2` with a nonzero byte limit. After a
whole redraw, call `assembly_streaming_progress` and schedule another redraw if
the reported visible set is incomplete. A capture reads the image from that
redraw, including any partial result. `CompleteV1` is the default and restores
complete redraws on the next frame.

Each eligible redraw includes the visible parts whose vertex and index streams
are already resident. The renderer considers cold parts by distance from the
current screen center. Occurrences of the same stream use one share of the
budget. The selected draws keep their original layer and draw order. A camera
or scene change supplies a new visible set and new priorities. Current sources
stay alive through successive redraws, and submitted resources retire with
their frame fences.

The limit counts staged vertex and index bytes. It applies to ordinary opaque
triangle draws with stock raster geometry. Specialized passes, placement
batches, offscreen draws, immediate redraws, and a cold part larger than the
limit use a complete redraw. In those cases the progress query returns
`None`. The query waits for submitted work before reporting partial or complete
progress, so a deferred submission error is visible to the caller.
