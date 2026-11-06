//! Actual concrete AisTexturedShape feeds retained PrsMgr presentations into Vulkan.
#[allow(dead_code)]
#[path = "../viewer_fixtures/support/textured_face_fixture.rs"]
mod mesh_fixture;
#[path = "ais_textured_shape_modes.rs"]
#[allow(dead_code)]
// Individual capture runs also cover dependency diagnostics outside the benchmark cohort.
pub mod modes;
#[path = "ais_textured_shape_settings.rs"]
mod settings;
use super::scene::ViewerScene;
use martensite_tkbrep::{brep::BRepBuilder, topods::Face};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkmath::gp::Plane3;
use martensite_tkmesh::brep_mesh::BRepMeshIncrementalMesh;
use martensite_tkservice::graphic3d::*;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;
use martensite_tkv3d::{ais::*, prs_mgr::*};
use martensite_vulkan::Graphic3dRenderDriver;
pub struct AisTexturedShapeScene {
    pub scene: ViewerScene,
    pub object: Handle<AisTexturedShape>,
    pub manager: Handle<PrsMgrPresentationManager>,
}
impl AisTexturedShapeScene {
    /// Retain a real concrete class and its manager; `AisTexturedShapeScene::with_mode(&driver,"basic")?`.
    pub fn with_mode(driver: &Handle<Graphic3dRenderDriver>, mode: &str) -> Result<Self, Failure> {
        let scene = ViewerScene::new(driver);
        scene.structure.erase();
        let face = textured_capture_face(mode)?;
        let object = AisTexturedShape::new(&face);
        settings::configure_textured_capture(&object, mode)?;
        let manager = PrsMgrPresentationManager::new(&scene.viewer.structure_manager());
        manager.display(&object.cast(), textured_display_mode(mode))?;
        Ok(Self {
            scene,
            object,
            manager,
        })
    }
    /// Exercise the native live appearance/source overrides; `fixture.mutate("updated-pixels")?`.
    pub fn mutate(&self, mode: &str) -> Result<(), Failure> {
        settings::mutate_textured_capture(&self.object, mode)?;
        if mode == "updated-pixels" {
            self.manager.update(&self.object.cast(), 3)?;
        }
        Ok(())
    }
}
fn textured_display_mode(mode: &str) -> i32 {
    // Native display mode selection is kept separate from texture filename/source policies.
    match mode {
        "wireframe" => 0,
        "shaded" => 1,
        "bbox" => 2,
        _ => 3,
    }
}
fn textured_capture_face(mode: &str) -> Result<Face, Failure> {
    let face =
        BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 4., -3., 5.))?.face()?;
    if mode == "face-boundaries-meshed" {
        BRepMeshIncrementalMesh::with_shape(&face, 0.1, false, 0.5, false)?;
        return Ok(face);
    }
    BRepBuilder::default().update_face_triangulation(
        &face,
        &mesh_fixture::textured_face_mesh(1)?,
        true,
    )?;
    Ok(face)
}
