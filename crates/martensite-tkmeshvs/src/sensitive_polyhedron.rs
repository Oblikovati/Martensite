// SPDX-License-Identifier: GPL-2.0-only
//! `MeshVSSensitivePolyhedron`: a volume detected through its face polygons.
//!
//! Quirks kept: the center sums the vertices of every face (a node shared by several
//! faces counts several times) but divides by the number of nodes, and the node array keeps
//! the origin for the nodes no face uses.
use crate::data_source::MeshVSTopologyHandle;
use martensite_tkernel::ncollection::{Array1, Vec3};
use martensite_tkernel::standard::{Failure, Handle, StandardTransient};
use martensite_tkmath::gp::{Point3, Xyz};
use martensite_tkv3d::select_basics::{SelectBasicsPickResult, SelectBasicsSelectingVolumeManager};
use martensite_tkv3d::select_mgr::SelectMgrEntityOwnerSource;
use martensite_tkv3d::select3d::{
    Select3DBndBox3d, Select3DSensitiveEntity, Select3DSensitiveEntityBase,
    Select3DSensitiveEntityMembers, Select3DTypeOfSensitivity,
};

/// A sensitive volume; `MeshVSSensitivePolyhedron::new(&owner, &nodes, 1, &topo)?`.
pub struct MeshVSSensitivePolyhedron {
    transient: StandardTransient,
    base: Select3DSensitiveEntityBase,
    topology: Vec<Vec<Point3>>,
    center: Xyz,
    bnd_box: Select3DBndBox3d,
    nodes: Vec<Point3>,
    nodes_lower: i32,
    topo: MeshVSTopologyHandle,
}
martensite_tkernel::impl_transient!(MeshVSSensitivePolyhedron, "MeshVSSensitivePolyhedron", dyn Select3DSensitiveEntity, transient,
    interfaces: [dyn Select3DSensitiveEntity]);

impl MeshVSSensitivePolyhedron {
    /// `nodes` is the array
    /// of lower bound `nodes_lower`; the face indices are zero-based into it. A face index
    /// outside the array is reported (undefined).
    pub fn new(
        owner: &Handle<dyn SelectMgrEntityOwnerSource>,
        nodes: &[Point3],
        nodes_lower: i32,
        topo: &MeshVSTopologyHandle,
    ) -> Result<Self, Failure> {
        let faces = topo.get().ok_or_else(|| {
            Failure::with_type(
                "Standard_NullObject",
                "MeshVSSensitivePolyhedron topology=null: expected faces",
                "",
            )
        })?;
        let mut used = vec![Point3::default(); nodes.len()];
        let (mut topology, mut center, mut bnd_box) = (
            Vec::new(),
            Xyz::new(0.0, 0.0, 0.0),
            Select3DBndBox3d::default(),
        );
        for face in faces.array1().iter() {
            let mut polygon = Vec::with_capacity(face.len());
            for &index in face.iter() {
                let vertex = *nodes
                    .get(index as usize)
                    .filter(|_| index >= 0)
                    .ok_or_else(|| out_of_range(index, nodes.len()))?;
                used[index as usize] = vertex;
                bnd_box.add(&Vec3::new(vertex.x(), vertex.y(), vertex.z()));
                center += vertex.xyz();
                polygon.push(vertex);
            }
            topology.push(polygon);
        }
        center /= nodes.len() as f64;
        Ok(Self {
            transient: StandardTransient::default(),
            base: Select3DSensitiveEntityBase::new(owner),
            topology,
            center,
            bnd_box,
            nodes: used,
            nodes_lower,
            topo: topo.clone(),
        })
    }
}

fn out_of_range(index: i32, len: usize) -> Failure {
    Failure::with_type(
        "Standard_OutOfRange",
        &format!("MeshVSSensitivePolyhedron node index={index}: expected 0..{len}"),
        "",
    )
}

impl Select3DSensitiveEntity for MeshVSSensitivePolyhedron {
    fn sensitive_base(&self) -> &Select3DSensitiveEntityBase {
        &self.base
    }
    /// `matches`: the nearest overlapped face polygon, folded into the caller's pick result.
    fn matches(
        &self,
        manager: &dyn SelectBasicsSelectingVolumeManager,
        pick: &mut SelectBasicsPickResult,
    ) -> Result<bool, Failure> {
        let mut face_pick = SelectBasicsPickResult::default();
        for polygon in &self.topology {
            let array = Array1::from_shared(polygon, 0);
            if manager.overlaps_polygon(
                &array,
                Select3DTypeOfSensitivity::Interior as i32,
                &mut face_pick,
            )? {
                *pick = *SelectBasicsPickResult::min(pick, &face_pick);
            }
        }
        if !pick.is_valid() {
            return Ok(false);
        }
        pick.set_dist_to_geom_center(manager.dist_to_geometry_center(&self.center_of_geometry())?);
        Ok(true)
    }
    /// `nb_sub_elements`: the number of nodes.
    fn nb_sub_elements(&self) -> i32 {
        self.nodes.len() as i32
    }
    fn bounding_box(&self) -> Select3DBndBox3d {
        self.bnd_box
    }
    fn center_of_geometry(&self) -> Point3 {
        Point3::from(self.center)
    }
    /// `get_connected`: a polyhedron of the stored nodes and topology.
    fn get_connected(&self) -> Result<Handle<dyn Select3DSensitiveEntity>, Failure> {
        Ok(Handle::new(Self::new(
            &self.owner_id(),
            &self.nodes,
            self.nodes_lower,
            &self.topo,
        )?)
        .cast())
    }
}
