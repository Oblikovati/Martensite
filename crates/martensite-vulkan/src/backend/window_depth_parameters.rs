//! Geometry-stage specialization owns viewport values without overlapping vertex push bytes.
use crate::{
    RenderViewportGridPrecision, RenderWindowDepthBehavior, RenderWindowDepthFailure as Failure,
};
use ash::vk;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WindowDepthPipelineParameters {
    viewport: [u32; 6],
    fragment_depth: bool,
    float64: bool,
    clipped_fan: bool,
    shared_incidence: bool,
    reference: RenderViewportGridPrecision,
    raster: RenderViewportGridPrecision,
    negative_one_to_one: bool,
}

impl WindowDepthPipelineParameters {
    pub(super) fn selected(
        behavior: RenderWindowDepthBehavior,
        viewport: vk::Viewport,
        negative_one_to_one: bool,
    ) -> Result<Option<Self>, Failure> {
        let Some(grids) = selected_grids(behavior)? else {
            return Ok(None);
        };
        require_viewport(viewport)?;
        require_grids(behavior, grids)?;
        Ok(Some(Self::operation(
            behavior,
            viewport,
            grids,
            negative_one_to_one,
        )))
    }
    fn operation(
        behavior: RenderWindowDepthBehavior,
        viewport: vk::Viewport,
        grids: [RenderViewportGridPrecision; 2],
        negative_one_to_one: bool,
    ) -> Self {
        Self {
            viewport: viewport_words(viewport),
            fragment_depth: super::window_depth_device::fragment_selected(behavior),
            float64: super::window_depth_device::float64_selected(behavior),
            clipped_fan: super::clipped_depth_device::selected(behavior),
            shared_incidence: matches!(
                behavior,
                RenderWindowDepthBehavior::ClippedIncidenceFramebufferV6 { .. }
            ),
            reference: grids[0],
            raster: grids[1],
            negative_one_to_one,
        }
    }

    pub(super) fn cache_words(self) -> [u32; 10] {
        let [x, y, width, height, minimum, maximum] = self.viewport;
        [
            x,
            y,
            width,
            height,
            minimum,
            maximum,
            self.reference.fractional_bits(),
            self.raster.fractional_bits(),
            u32::from(self.negative_one_to_one),
            if self.float64 {
                5
            } else {
                u32::from(self.fragment_depth)
                    + u32::from(self.clipped_fan)
                    + u32::from(self.shared_incidence)
            },
        ]
    }

    pub(super) fn specialization_words(self) -> [u32; 7] {
        let [x, y, width, height, _, _] = self.viewport;
        [
            x,
            y,
            width,
            height,
            self.reference.fractional_bits(),
            self.raster.fractional_bits(),
            u32::from(self.negative_one_to_one),
        ]
    }

    pub(super) fn fragment_depth(self) -> bool {
        self.fragment_depth
    }

    pub(super) fn float64(self) -> bool {
        self.float64
    }

    pub(super) fn clipped_fan(self) -> bool {
        self.clipped_fan
    }

    pub(super) fn shared_incidence(self) -> bool {
        self.shared_incidence
    }

    pub(super) fn viewport(self) -> vk::Viewport {
        let [x, y, width, height, min_depth, max_depth] = self.viewport.map(f32::from_bits);
        vk::Viewport {
            x,
            y,
            width,
            height,
            min_depth,
            max_depth,
        }
    }

    pub(super) fn grids(self) -> [RenderViewportGridPrecision; 2] {
        [self.reference, self.raster]
    }

    pub(super) fn negative_one_to_one(self) -> bool {
        self.negative_one_to_one
    }

    pub(super) fn require_actual_viewport(self, actual: vk::Viewport) -> Result<(), Failure> {
        if viewport_words(actual) == self.viewport {
            return Ok(());
        }
        Err(Failure::InvalidViewport {
            words: viewport_words(actual),
        })
    }
}

fn selected_grids(
    behavior: RenderWindowDepthBehavior,
) -> Result<Option<[RenderViewportGridPrecision; 2]>, Failure> {
    if behavior == RenderWindowDepthBehavior::HardwareV1 {
        return Ok(None);
    }
    let grids = ordinary_grids(behavior)
        .or_else(|| fan_grids(behavior))
        .ok_or_else(|| Failure::UnsupportedInterface {
            actual: format!("window-depth selection {behavior:?}; expected registered version"),
        })?;
    Ok(Some(grids))
}
fn ordinary_grids(behavior: RenderWindowDepthBehavior) -> Option<[RenderViewportGridPrecision; 2]> {
    match behavior {
        RenderWindowDepthBehavior::UnsnappedWindowDepthV2 {
            reference_grid,
            raster_grid,
        }
        | RenderWindowDepthBehavior::Float64ReferenceWindowDepthV5 {
            reference_grid,
            raster_grid,
        }
        | RenderWindowDepthBehavior::ReferenceWindowDepthV3 {
            reference_grid,
            raster_grid,
        } => Some([reference_grid, raster_grid]),
        _ => None,
    }
}
fn fan_grids(behavior: RenderWindowDepthBehavior) -> Option<[RenderViewportGridPrecision; 2]> {
    match behavior {
        RenderWindowDepthBehavior::ClippedFanFramebufferV4 {
            fan_grid,
            raster_grid,
        }
        | RenderWindowDepthBehavior::ClippedIncidenceFramebufferV6 {
            fan_grid,
            raster_grid,
        } => Some([fan_grid, raster_grid]),
        _ => None,
    }
}
const REFERENCE_GRIDS: [RenderViewportGridPrecision; 2] = [
    RenderViewportGridPrecision::Fractional12,
    RenderViewportGridPrecision::Fractional8,
];
fn require_grids(
    behavior: RenderWindowDepthBehavior,
    grids: [RenderViewportGridPrecision; 2],
) -> Result<(), Failure> {
    if matches!(
        behavior,
        RenderWindowDepthBehavior::ClippedFanFramebufferV4 { .. }
    ) || grids == REFERENCE_GRIDS
    {
        return Ok(());
    }
    Err(Failure::UnsupportedInterface {
        actual: format!(
            "version{} grid pair {grids:?}; expected explicit reference12/raster8",
            behavior.behavior_version()
        ),
    })
}

fn require_viewport(viewport: vk::Viewport) -> Result<(), Failure> {
    let words = viewport_words(viewport);
    if words
        .map(f32::from_bits)
        .iter()
        .all(|value| value.is_finite())
        && viewport.width != 0.
        && viewport.height != 0.
        && viewport.min_depth == 0.
        && viewport.max_depth == 1.
    {
        return Ok(());
    }
    Err(Failure::InvalidViewport { words })
}

fn viewport_words(viewport: vk::Viewport) -> [u32; 6] {
    [
        viewport.x,
        viewport.y,
        viewport.width,
        viewport.height,
        viewport.min_depth,
        viewport.max_depth,
    ]
    .map(f32::to_bits)
}

#[cfg(test)]
#[path = "window_depth_parameters_tests.rs"]
mod tests;
