//! `StepConstructStyles`: reads and writes shape styles such as colour: a list of styles, taken from
//! the STEP model (reading) or filled by calls to `add_style` (writing).
use super::{StepConstruct, StepConstructRenderingProperties, StepConstructTool};
use crate::api_header_section::APIHeaderSectionMakeHeader;
use crate::step_basic::{StepBasicMeasureValueMember, StepBasicSizeSelect};
use crate::step_data::{StepDataSelectMember, StepDataSelectType, StepDataStepModel};
use crate::step_entity::{HArray1, HSequence, step_array1_set, step_live, step_new_array1};
use crate::step_geom::StepGeomGeometricRepresentationItem;
use crate::step_repr::{
    StepReprProductDefinitionShape, StepReprRepresentation, StepReprRepresentationContext,
    StepReprRepresentationItem, StepReprRepresentationRelationshipWithTransformation,
    StepReprRepresentedDefinition,
};
use crate::step_shape::{
    StepShapeContextDependentShapeRepresentation, StepShapeShapeDefinitionRepresentation,
    StepShapeShapeRepresentation,
};
use crate::step_visual::{
    StepVisualColour, StepVisualCurveStyle, StepVisualCurveStyleFontSelect,
    StepVisualDraughtingModel, StepVisualDraughtingPreDefinedCurveFont, StepVisualFillAreaStyle,
    StepVisualFillAreaStyleColour, StepVisualFillStyleSelect, StepVisualInvisibility,
    StepVisualMechanicalDesignGeometricPresentationRepresentation, StepVisualOverRidingStyledItem,
    StepVisualPresentationStyleAssignment, StepVisualPresentationStyleByContext,
    StepVisualPresentationStyleSelect, StepVisualStyleContextSelect, StepVisualStyledItem,
    StepVisualSurfaceSide, StepVisualSurfaceSideStyle, StepVisualSurfaceStyleElementSelect,
    StepVisualSurfaceStyleFillArea, StepVisualSurfaceStyleRenderingWithProperties,
    StepVisualSurfaceStyleUsage,
};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::ncollection::IndexedMap;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, StandardTransient};
use martensite_tkernel::tcollection::HAsciiString;
use martensite_tkxsbase::interface::InterfaceModelEntities;
use martensite_tkxsbase::transfer_brep::TransferBRep;
use martensite_tkxsbase::xs_control::XSControlWorkSession;
use std::ops::Deref;

mod colours;
#[allow(unused_imports, reason = "what the sibling modules use")]
pub use colours::*;

const WHERE: &str = "StepConstructStyles";

fn text(value: &str) -> Handle<HAsciiString> {
    Handle::new(HAsciiString::new(value))
}

/// An array of one value, from 1.
fn single<T>(value: T) -> Result<Handle<HArray1<T>>, Failure>
where
    T: Default + Clone + Send + Sync + 'static,
{
    let array = step_new_array1::<T>(1, 1)?;
    step_array1_set(&array, 1, value, WHERE)?;
    Ok(array)
}

/// The exact type test of the `dynamic_type() == STANDARD_TYPE(T)`.
fn is_exactly<T: RuntimeTyped>(entity: &Handle<StandardTransient>) -> bool {
    entity
        .dynamic_type()
        .is_some_and(|kind| std::ptr::eq(kind, T::static_type()))
}

/// `process_as_surface_style_rendering`: true when the select is a SurfaceStyleRendering, even if its
/// colour and transparency couldn't be extracted.
fn process_as_surface_style_rendering(
    select: &StepVisualSurfaceStyleElementSelect,
    rendering_props: &mut StepConstructRenderingProperties,
) -> Result<bool, Failure> {
    let rendering = select.surface_style_rendering();
    if rendering.is_null() {
        return Ok(false);
    }
    let with_properties = rendering.cast::<StepVisualSurfaceStyleRenderingWithProperties>();
    if with_properties.is_null() {
        return Ok(true);
    }
    rendering_props.init_entity(&with_properties)?;
    Ok(rendering_props.is_defined())
}

/// `process_as_surface_style_boundary`: true when the select is a SurfaceStyleBoundary, even if its
/// colour couldn't be extracted.
fn process_as_surface_style_boundary(
    select: &StepVisualSurfaceStyleElementSelect,
    boundary_colour: &mut Handle<StepVisualColour>,
) -> bool {
    let boundary = select.surface_style_boundary();
    let Some(boundary) = boundary.get() else {
        return false;
    };
    if let Some(style) = boundary.style_of_boundary().get() {
        *boundary_colour = style.curve_colour();
    }
    true
}

/// `process_as_surface_style_fill_area`: true when the select is a SurfaceStyleFillArea, even if its colour
/// couldn't be extracted. A colour of the negative side is used only while there is no colour.
fn process_as_surface_style_fill_area(
    select: &StepVisualSurfaceStyleElementSelect,
    side: StepVisualSurfaceSide,
    surface_colour: &mut Handle<StepVisualColour>,
) -> Result<bool, Failure> {
    let fill_area = select.surface_style_fill_area();
    let Some(fill_area) = fill_area.get() else {
        return Ok(false);
    };
    let style = fill_area.fill_area();
    let Some(style) = style.get() else {
        return Ok(true);
    };
    for index in 1..=style.nb_fill_styles() {
        let colour = style.fill_styles_value(index)?.fill_area_style_colour();
        if let Some(colour) = colour.get()
            // abv 30 Mar 00: trj3_s1-pe.stp
            && (surface_colour.is_null() || side != StepVisualSurfaceSide::SsNegative)
        {
            *surface_colour = colour.fill_colour();
        }
    }
    Ok(true)
}

/// `process_as_surface_style_usage`: true when the select is a SurfaceStyleUsage.
fn process_as_surface_style_usage(
    select: &StepVisualPresentationStyleSelect,
    surface_colour: &mut Handle<StepVisualColour>,
    boundary_colour: &mut Handle<StepVisualColour>,
    rendering_props: &mut StepConstructRenderingProperties,
) -> Result<bool, Failure> {
    let usage = select.surface_style_usage();
    let Some(usage) = usage.get() else {
        return Ok(false);
    };
    let side_style = usage.style();
    let side_style = step_live(&side_style, WHERE)?;
    for index in 1..=side_style.nb_styles()? {
        let element = side_style.styles_value(index)?;
        // An element is of only one of these types, so stop as soon as its type is processed.
        let _processed =
            process_as_surface_style_fill_area(&element, usage.side(), surface_colour)?
                || process_as_surface_style_boundary(&element, boundary_colour)
                || process_as_surface_style_rendering(&element, rendering_props)?;
    }
    Ok(true)
}

/// `process_as_curve_style`: true when the select is a CurveStyle.
fn process_as_curve_style(
    select: &StepVisualPresentationStyleSelect,
    curve_colour: &mut Handle<StepVisualColour>,
) -> bool {
    let style = select.curve_style();
    let Some(style) = style.get() else {
        return false;
    };
    *curve_colour = style.curve_colour();
    true
}

/// ```
/// use martensite_tkdestep::step_construct::StepConstructStyles;
/// let mut styles = StepConstructStyles::new();
/// assert_eq!((styles.nb_styles(), styles.nb_root_styles()), (0, 0));
/// assert!(styles.style(1).is_err());
/// styles.clear_styles();
/// ```
pub struct StepConstructStyles {
    tool: StepConstructTool,
    my_map_of_styles: Vec<(
        Handle<StandardTransient>,
        Handle<StepVisualPresentationStyleAssignment>,
    )>,
    my_styles: IndexedMap<Handle<StandardTransient>>,
    my_root_styles: IndexedMap<Handle<StandardTransient>>,
    my_psa: Vec<Handle<StepVisualPresentationStyleAssignment>>,
}

impl Default for StepConstructStyles {
    fn default() -> Self {
        Self {
            tool: StepConstructTool::default(),
            my_map_of_styles: Vec::new(),
            my_styles: IndexedMap::new(1),
            my_root_styles: IndexedMap::new(1),
            my_psa: Vec::new(),
        }
    }
}

impl Deref for StepConstructStyles {
    type Target = StepConstructTool;
    fn deref(&self) -> &StepConstructTool {
        &self.tool
    }
}

impl StepConstructStyles {
    /// `StepConstructStyles()`: an empty tool.
    pub fn new() -> Self {
        Self::default()
    }
    /// `StepConstructStyles(WS)`: a tool loaded with a session.
    pub fn with_ws(ws: &Handle<XSControlWorkSession>) -> Result<Self, Failure> {
        Ok(Self {
            tool: StepConstructTool::with_ws(ws)?,
            ..Self::default()
        })
    }
    /// `init`: true on success. The root styles stay.
    pub fn init(&mut self, ws: &Handle<XSControlWorkSession>) -> Result<bool, Failure> {
        self.my_map_of_styles.clear();
        self.my_styles.clear(true);
        self.my_psa.clear();
        self.tool.set_ws(ws)
    }
    /// `nb_styles`.
    pub fn nb_styles(&self) -> i32 {
        self.my_styles.extent() as i32
    }
    /// `style`.
    pub fn style(&self, i: i32) -> Result<Handle<StepVisualStyledItem>, Failure> {
        Ok(self.my_styles.find_key(i as usize)?.cast())
    }
    /// `nb_root_styles`: the number of override styles.
    pub fn nb_root_styles(&self) -> i32 {
        self.my_root_styles.extent() as i32
    }
    /// `root_style`.
    pub fn root_style(&self, i: i32) -> Result<Handle<StepVisualStyledItem>, Failure> {
        Ok(self.my_root_styles.find_key(i as usize)?.cast())
    }
    /// `clear_styles`: all defined styles and the PSA sequence.
    pub fn clear_styles(&mut self) {
        self.my_styles.clear(true);
        self.my_psa.clear();
        self.my_root_styles.clear(true);
    }
    /// `add_style(style)`.
    pub fn add_style(&mut self, style: &Handle<StepVisualStyledItem>) {
        self.my_styles.add(style.cast());
    }
    /// `add_style(item, PSA, Override)`: a style linking the PSA to the item, stored and returned; of
    /// the subtype OverridingStyledItem when `override_style` is not null.
    pub fn add_style_for_item(
        &mut self,
        item: &Handle<StepReprRepresentationItem>,
        psa: &Handle<StepVisualPresentationStyleAssignment>,
        override_style: &Handle<StepVisualStyledItem>,
    ) -> Result<Handle<StepVisualStyledItem>, Failure> {
        let styles = single(psa.clone())?;
        let style: Handle<StepVisualStyledItem> = if override_style.is_null() {
            let style = Handle::new(StepVisualStyledItem::new());
            style.init(&text("color"), &styles, &item.cast());
            style
        } else {
            let style = Handle::new(StepVisualOverRidingStyledItem::new());
            style.init(
                &text("overriding color"),
                &styles,
                &item.cast(),
                override_style,
            );
            style.cast()
        };
        self.my_styles.add(style.cast());
        // for future using
        self.my_psa.push(psa.clone());
        Ok(style)
    }
    /// `add_style(Shape, PSA, Override)`: the same on the entity of a shape, found by
    /// `STEPConstruct::find_entity`; null without an entity.
    pub fn add_style_for_shape(
        &mut self,
        shape: &Shape,
        psa: &Handle<StepVisualPresentationStyleAssignment>,
        override_style: &Handle<StepVisualStyledItem>,
    ) -> Result<Handle<StepVisualStyledItem>, Failure> {
        let item = StepConstruct::find_entity(self.finder_process(), shape)?;
        if item.is_null() {
            return Ok(Handle::default());
        }
        self.add_style_for_item(&item, psa, override_style)
    }
    /// `create_mdgpr`: an MDGPR filled with all the styles previously defined; the caller adds it to
    /// the model.
    pub fn create_mdgpr(
        &self,
        context: &Handle<StepReprRepresentationContext>,
        repr: &mut Handle<StepVisualMechanicalDesignGeometricPresentationRepresentation>,
        step_model: &Handle<StepDataStepModel>,
    ) -> Result<bool, Failure> {
        let count = self.my_styles.extent() as i32;
        if count < 1 {
            return Ok(false);
        }
        // create MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION
        let elems = step_new_array1::<Handle<StepReprRepresentationItem>>(1, count)?;
        for i in 1..=count {
            step_array1_set(
                &elems,
                i,
                self.my_styles.find_key(i as usize)?.cast(),
                WHERE,
            )?;
        }
        // create new MDGPR
        let created =
            Handle::new(StepVisualMechanicalDesignGeometricPresentationRepresentation::new());
        created.init(&text(""), &elems, context);
        *repr = created;
        // for AP203, add subschema name
        if step_live(step_model, WHERE)?
            .internal_parameters()
            .write_schema
            .as_raw()
            == 3
        {
            let model = self.tool.model()?.cast::<StepDataStepModel>();
            let mut header = APIHeaderSectionMakeHeader::from_model(step_live(&model, WHERE)?)?;
            header.add_schema_identifier(&text("SHAPE_APPEARANCE_LAYER_MIM"))?;
        }
        Ok(true)
    }
}
