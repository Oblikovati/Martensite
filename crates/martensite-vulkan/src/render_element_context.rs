//! Accessors of `RenderElementContext`, mirroring the `GlWorkspace` getters.
use crate::primitive::PrimitiveSubmission;
use crate::render_element::{
    RenderElementContext, RenderElementFilter, RenderElementGeometry, TextPlacement,
};
use martensite_tkernel::{quantity::ColorRgba, standard::Handle};
use martensite_tkservice::graphic3d::*;

impl RenderElementContext<'_> {
    /// The render context the element draws with (`GlWorkspace::GetGlContext`): device
    /// limits, viewport, resolution, matrix stacks and the per-draw state of the current
    /// aspects; `context.context_state.viewport[2]`.
    pub fn context_state(&self) -> &crate::RenderContextState {
        &self.gl_state
    }
    /// The view window width (`GlWorkspace::Width`); `context.width`.
    pub fn width(&self) -> i32 {
        self.gl_state.window_size()[0]
    }
    /// The view window height (`GlWorkspace::Height`); `context.height`.
    pub fn height(&self) -> i32 {
        self.gl_state.window_size()[1]
    }
    /// Currently set aspects (`Workspace::Aspects`); `context.aspects()`.
    pub fn aspects(&self) -> &Handle<dyn Graphic3dAspectsSource> {
        self.aspects
    }
    /// Replace the aspects for later elements of the group and return the previous ones
    /// (`Workspace::SetAspects`); `let previous = context.set_aspects(style);`.
    pub fn set_aspects(
        &mut self,
        aspects: Handle<dyn Graphic3dAspectsSource>,
    ) -> Handle<dyn Graphic3dAspectsSource> {
        std::mem::replace(self.aspects, aspects)
    }
    /// The structure highlight style, null when not highlighted (`HighlightStyle`).
    pub fn highlight_style(&self) -> &Handle<dyn Graphic3dPresentationAttributesSource> {
        &self.highlight
    }
    /// Whether highlight colors apply (`ToHighlight`); `context.to_highlight()`.
    pub fn to_highlight(&self) -> bool {
        !self.highlight.is_null()
    }
    /// Back-face culling allowed: a closed group without clipping (`ToAllowFaceCulling`).
    pub fn to_allow_face_culling(&self) -> bool {
        self.state.allow_face_culling
    }
    /// The active render filter (`RenderFilter`); `context.render_filter()`.
    pub fn render_filter(&self) -> RenderElementFilter {
        self.state.filter
    }
    /// Depth testing is used (`UseZBuffer`); `context.use_z_buffer()`.
    pub fn use_z_buffer(&self) -> bool {
        self.state.use_z_buffer
    }
    /// Depth writes are enabled for the layer (`UseDepthWrite`); `context.use_depth_write()`.
    pub fn use_depth_write(&self) -> bool {
        self.state.use_depth_write
    }
    /// Transparent filling elements skipped so far by the layer's opaque pass
    /// (`NbSkippedTransparentElements`); `context.nb_skipped_transparent_elements()`.
    pub fn nb_skipped_transparent_elements(&self) -> usize {
        self.state.skipped_transparent
    }
    /// Edge color, or the highlight color (`EdgeColor`); `context.edge_color()`.
    pub fn edge_color(&self) -> ColorRgba {
        self.highlighted_or(|aspects| aspects.edge_color_rgba())
    }
    /// Interior color, or the highlight color (`InteriorColor`); `context.interior_color()`.
    pub fn interior_color(&self) -> ColorRgba {
        self.highlighted_or(|aspects| aspects.interior_color_rgba())
    }
    /// Back interior color when distinguished, else the interior color (`BackInteriorColor`).
    pub fn back_interior_color(&self) -> ColorRgba {
        self.highlighted_or(|aspects| match aspects.distinguish() {
            true => aspects.back_interior_color_rgba(),
            false => aspects.interior_color_rgba(),
        })
    }
    /// Text color, the aspect interior color, or the highlight color (`TextColor`).
    pub fn text_color(&self) -> ColorRgba {
        self.interior_color()
    }
    /// Text subtitle color, the aspect edge color, or the highlight color (`TextSubtitleColor`).
    pub fn text_subtitle_color(&self) -> ColorRgba {
        self.edge_color()
    }
    /// Draw a primitive array with the current aspects, as the group's own arrays are drawn.
    /// Example: `context.add_primitive_array(Graphic3dTypeOfPrimitiveArray::Triangles, &indices, &attributes, &bounds)`.
    pub fn add_primitive_array(
        &mut self,
        kind: Graphic3dTypeOfPrimitiveArray,
        indices: &Handle<dyn Graphic3dIndexBufferSource>,
        attributes: &Handle<dyn Graphic3dBufferSource>,
        bounds: &Handle<Graphic3dBoundBuffer>,
    ) {
        self.add_primitive_submission(PrimitiveSubmission {
            kind,
            indices: indices.clone(),
            attributes: attributes.clone(),
            bounds: bounds.clone(),
            upload: Default::default(),
            uid: 0,
        });
    }
    /// Draw a retained submission whose residency slot records the frames that draw it.
    pub(crate) fn add_primitive_submission(&mut self, primitive: PrimitiveSubmission) {
        self.emit(RenderElementGeometry::Primitives(primitive));
    }
    /// Draw a text with the current aspects; `context.add_text(&text)`.
    pub fn add_text(&mut self, text: &Handle<Graphic3dText>) {
        self.add_retained_text(text, Default::default(), TextPlacement::Scene);
    }
    /// Draw a text whose residency slot records the glyph buffers of the frames that draw it.
    pub(crate) fn add_retained_text(
        &mut self,
        text: &Handle<Graphic3dText>,
        upload: crate::element_upload::ElementUploadSlot,
        placement: TextPlacement,
    ) {
        if !text.is_null() {
            self.emit(RenderElementGeometry::Text(text.clone(), upload, placement));
        }
    }
    fn emit(&mut self, geometry: RenderElementGeometry) {
        self.emitted.push((self.aspects.clone(), geometry));
    }
    fn highlighted_or(
        &self,
        color: impl Fn(&dyn Graphic3dAspectsSource) -> ColorRgba,
    ) -> ColorRgba {
        if let Some(style) = self.highlight.get() {
            return style.color_rgba();
        }
        self.aspects.get().map_or_else(ColorRgba::default, color)
    }
}

/// `GlWorkspace::DumpJson` of the state an element sees.
impl RenderElementContext<'_> {
    /// `GlWorkspace::DumpJson`: depth test and write, the workspace's culling and default
    /// aspects (as native `GlWorkspace` builds them), the skipped transparent elements, the
    /// render filter, the current aspects (also the applied ones: the frame applies the
    /// current aspects to each element), face culling and the highlight aspects (the defaults
    /// until a highlighted face copies the current aspects); `context.dump_json(&mut s, -1)?`.
    pub fn dump_json(
        &self,
        stream: &mut martensite_tkernel::standard::dump::DumpStream,
        depth: i32,
    ) -> std::io::Result<()> {
        use crate::driver_dump::{address_of, aspects_element, dumped_handle};
        use martensite_tkservice::json_dump::{class_begin, field_dumped, field_integer};
        let none_culling = workspace_aspects(Some(Graphic3dTypeOfBackfacingModel::DoubleSided));
        let front_culling = workspace_aspects(Some(Graphic3dTypeOfBackfacingModel::BackCulled));
        let defaults = workspace_aspects(None);
        let highlight = workspace_aspects(None);
        let aspects_field =
            |stream: &mut martensite_tkernel::standard::dump::DumpStream,
             field: &str,
             aspect: &Handle<dyn Graphic3dAspectsSource>| {
                let address = aspect.object_address() as usize;
                field_dumped(stream, depth, field, &|o, d| {
                    aspects_element(o, d, address, aspect)
                })
            };
        class_begin(stream, "GlWorkspace")?;
        field_integer(stream, "myUseZBuffer", i32::from(self.state.use_z_buffer))?;
        field_integer(
            stream,
            "myUseDepthWrite",
            i32::from(self.state.use_depth_write),
        )?;
        aspects_field(stream, "&myNoneCulling", &none_culling)?;
        aspects_field(stream, "&myFrontCulling", &front_culling)?;
        let skipped = self.state.skipped_transparent as i32;
        field_integer(stream, "myNbSkippedTranspElems", skipped)?;
        field_integer(stream, "myRenderFilter", self.state.filter.bits() as i32)?;
        aspects_field(stream, "&myDefaultAspects", &defaults)?;
        let current = self.aspects.clone();
        let address = address_of(self.aspects);
        field_dumped(stream, depth, "myAspectsSet", &|o, d| {
            aspects_element(o, d, address, &current)
        })?;
        dumped_handle(
            stream,
            depth,
            "myAspectsApplied.get()",
            &current,
            &|a, o, d| a.dump_json(o, d),
        )?;
        let culling = i32::from(self.state.allow_face_culling);
        field_integer(stream, "myToAllowFaceCulling", culling)?;
        aspects_field(stream, "&myAspectFaceHl", &highlight)
    }
}

/// The aspects of a default-constructed native `GlAspects` (solid, white, unlit), with
/// the culling setup of `myNoneCulling` / `myFrontCulling` when `culling` is given.
fn workspace_aspects(
    culling: Option<Graphic3dTypeOfBackfacingModel>,
) -> Handle<dyn Graphic3dAspectsSource> {
    use martensite_tkernel::quantity::{Color, NamedColor};
    let aspect = Handle::new(Graphic3dAspects::default());
    aspect.set_interior_style(martensite_tkservice::aspect::AspectInteriorStyle::Solid);
    aspect.set_interior_color(Color::named(NamedColor::WHITE));
    aspect.set_edge_color(Color::named(NamedColor::WHITE));
    aspect.set_shading_model(Graphic3dTypeOfShadingModel::Unlit);
    if let Some(culling) = culling {
        aspect.set_face_culling(culling);
        aspect.set_draw_edges(false);
        aspect.set_alpha_mode(Graphic3dAlphaMode::Opaque, 0.5);
    }
    aspect.cast()
}
