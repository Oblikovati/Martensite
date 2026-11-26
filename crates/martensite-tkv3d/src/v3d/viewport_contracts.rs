// SPDX-License-Identifier: GPL-2.0-only
//! Checked viewport projection, preview capture planning, and child-view composition
//! contracts (#308, #504, #505).
use super::v3d_bad_value;
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::Point3;
use martensite_tkservice::graphic3d::Graphic3dCamera;

/// Projected pixel coordinate and clip status for a single world point (#505).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectedPixel {
    /// Horizontal pixel coordinate from the left edge.
    pub x: f64,
    /// Vertical pixel coordinate from the top edge.
    pub y: f64,
    /// Normalized device depth coordinate.
    pub z_ndc: f64,
    /// True when the point projects inside the viewport frustum.
    pub in_view: bool,
}

/// Projects a batch of world-space points into pixel coordinates with validated viewport
/// dimensions (#505).
pub fn project_world_points_checked(
    camera: &Graphic3dCamera,
    viewport_width: u32,
    viewport_height: u32,
    points: &[Point3],
) -> Result<Vec<ProjectedPixel>, Failure> {
    if viewport_width == 0 || viewport_height == 0 {
        return Err(v3d_bad_value(
            "viewport dimensions must be non-zero for checked point projection",
        ));
    }
    let w = f64::from(viewport_width);
    let h = f64::from(viewport_height);
    let zero_to_one = camera.is_zero_to_one_depth();
    let mut projected = Vec::with_capacity(points.len());
    for point in points {
        let ndc = camera.project(point);
        let z_ok = if zero_to_one {
            ndc.z() >= 0.0 && ndc.z() <= 1.0
        } else {
            ndc.z() >= -1.0 && ndc.z() <= 1.0
        };
        let in_view = ndc.x().abs() <= 1.0 && ndc.y().abs() <= 1.0 && z_ok;
        projected.push(ProjectedPixel {
            x: (ndc.x() + 1.0) * 0.5 * w,
            y: h - 1.0 - (ndc.y() + 1.0) * 0.5 * h,
            z_ndc: ndc.z(),
            in_view,
        });
    }
    Ok(projected)
}

/// Rectangular pixel region inside a framebuffer (#308, #504).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PreviewRegion {
    /// Left offset in pixels.
    pub x: u32,
    /// Top offset in pixels.
    pub y: u32,
    /// Region width in pixels.
    pub width: u32,
    /// Region height in pixels.
    pub height: u32,
}

/// Validated sub-rectangle plan for small color preview capture without full-frame clones (#504).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreviewCapturePlan {
    region: PreviewRegion,
    byte_stride: usize,
    total_bytes: usize,
}

impl PreviewCapturePlan {
    /// Validates `region` against `(frame_width, frame_height)` and `bytes_per_pixel` (#504).
    pub fn validate(
        frame_width: u32,
        frame_height: u32,
        region: PreviewRegion,
        bytes_per_pixel: usize,
    ) -> Result<Self, Failure> {
        if region.width == 0 || region.height == 0 || bytes_per_pixel == 0 {
            return Err(v3d_bad_value(
                "preview region width, height, and bytes_per_pixel must be non-zero",
            ));
        }
        let right = region
            .x
            .checked_add(region.width)
            .ok_or_else(|| v3d_bad_value("preview region x + width overflowed u32"))?;
        let bottom = region
            .y
            .checked_add(region.height)
            .ok_or_else(|| v3d_bad_value("preview region y + height overflowed u32"))?;
        if right > frame_width || bottom > frame_height {
            return Err(v3d_bad_value(
                "preview region exceeds framebuffer dimensions",
            ));
        }
        let byte_stride = (region.width as usize)
            .checked_mul(bytes_per_pixel)
            .ok_or_else(|| v3d_bad_value("preview byte stride overflowed usize"))?;
        let total_bytes = byte_stride
            .checked_mul(region.height as usize)
            .ok_or_else(|| v3d_bad_value("preview buffer size overflowed usize"))?;
        Ok(Self {
            region,
            byte_stride,
            total_bytes,
        })
    }

    /// Validated preview region.
    pub fn region(&self) -> PreviewRegion {
        self.region
    }

    /// Row stride in bytes.
    pub fn byte_stride(&self) -> usize {
        self.byte_stride
    }

    /// Total pixel buffer size in bytes.
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }
}

/// Positioned child V3d view slot inside a composed root viewport (#308).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildViewSlot {
    /// Stable slot identifier.
    pub id: String,
    /// Pixel bounds inside the parent view.
    pub viewport: PreviewRegion,
    /// Deterministic stacking order (higher draws on top).
    pub z_order: i32,
}

/// Deterministic composition of child V3d views inside a root GUI view (#308).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChildViewComposition {
    root_width: u32,
    root_height: u32,
    slots: Vec<ChildViewSlot>,
}

impl ChildViewComposition {
    /// Creates an empty composition for a root viewport of `(root_width, root_height)`.
    pub fn new(root_width: u32, root_height: u32) -> Result<Self, Failure> {
        if root_width == 0 || root_height == 0 {
            return Err(v3d_bad_value("root viewport dimensions must be non-zero"));
        }
        Ok(Self {
            root_width,
            root_height,
            slots: Vec::new(),
        })
    }

    /// Adds a validated child view slot and keeps slots sorted by `(z_order, id)` (#308).
    pub fn add_slot(&mut self, slot: ChildViewSlot) -> Result<(), Failure> {
        PreviewCapturePlan::validate(self.root_width, self.root_height, slot.viewport, 4)?;
        if self.slots.iter().any(|existing| existing.id == slot.id) {
            return Err(v3d_bad_value("duplicate child view slot id"));
        }
        self.slots.push(slot);
        self.slots
            .sort_by(|a, b| a.z_order.cmp(&b.z_order).then_with(|| a.id.cmp(&b.id)));
        Ok(())
    }

    /// Returns the front-most child slot containing pixel `(px, py)`, if any (#308).
    pub fn hit_test(&self, px: u32, py: u32) -> Option<&ChildViewSlot> {
        self.slots.iter().rev().find(|slot| {
            let r = slot.viewport;
            px >= r.x && px < r.x + r.width && py >= r.y && py < r.y + r.height
        })
    }

    /// Child slots in deterministic paint order (back-to-front).
    pub fn slots(&self) -> &[ChildViewSlot] {
        &self.slots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_projection_projects_origin_near_viewport_center() {
        let camera = Graphic3dCamera::default();
        let projected = project_world_points_checked(
            &camera,
            800,
            600,
            &[Point3::new(0.0, 0.0, 0.0), Point3::new(1000.0, 0.0, 0.0)],
        )
        .expect("projection");
        assert_eq!(projected.len(), 2);
        assert!(projected[0].in_view);
        assert!((projected[0].x - 400.0).abs() < 1.0);
        assert!((projected[0].y - 299.0).abs() <= 1.5);
        assert!(project_world_points_checked(&camera, 0, 600, &[]).is_err());
    }

    #[test]
    fn preview_capture_plan_validates_bounds_and_computes_exact_bytes() {
        let plan = PreviewCapturePlan::validate(
            1920,
            1080,
            PreviewRegion {
                x: 100,
                y: 50,
                width: 128,
                height: 64,
            },
            4,
        )
        .expect("valid plan");
        assert_eq!(plan.byte_stride(), 512);
        assert_eq!(plan.total_bytes(), 32768);
        assert!(
            PreviewCapturePlan::validate(
                100,
                100,
                PreviewRegion {
                    x: 80,
                    y: 0,
                    width: 30,
                    height: 10
                },
                4
            )
            .is_err()
        );
    }

    #[test]
    fn child_view_composition_orders_by_z_and_hit_tests_frontmost() {
        let mut comp = ChildViewComposition::new(1000, 800).expect("root");
        comp.add_slot(ChildViewSlot {
            id: "main".to_string(),
            viewport: PreviewRegion {
                x: 0,
                y: 0,
                width: 1000,
                height: 800,
            },
            z_order: 0,
        })
        .expect("main");
        comp.add_slot(ChildViewSlot {
            id: "pip".to_string(),
            viewport: PreviewRegion {
                x: 700,
                y: 500,
                width: 250,
                height: 250,
            },
            z_order: 10,
        })
        .expect("pip");
        assert_eq!(comp.hit_test(750, 550).map(|s| s.id.as_str()), Some("pip"));
        assert_eq!(comp.hit_test(100, 100).map(|s| s.id.as_str()), Some("main"));
    }
}
