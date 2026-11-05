// SPDX-License-Identifier: GPL-2.0-only
//! The mesh presentation cases of the volume-element test model, each on a real view: the viewer
//! and the Vulkan 1.3 driver draw every image, on a virtual window, so no display is needed.
//! Each case checks its images and the entities it hid, and ends with a clean validation report.
//! Set `MARTENSITE_MESH_VIEW_DIR` to keep the images as PNG files.
#![cfg(target_os = "linux")]

#[path = "mesh_view_cases/cases.rs"]
mod cases;
#[path = "mesh_view_cases/color_commands.rs"]
mod color_commands;
#[path = "mesh_view_cases/element_model.rs"]
mod element_model;
#[path = "mesh_view_cases/mesh_commands.rs"]
mod mesh_commands;
#[path = "mesh_view_cases/view_host.rs"]
mod view_host;
