//! Retain topology mesh bits and presentation identities across the diagnostic scope.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topexp::Explorer, topods};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::poly::{Triangulation, mesh_purpose};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
#[derive(Debug, PartialEq)]
pub(super) struct SphereIdentity {
    object: usize,
    camera: Vec<u64>,
    bindings: Vec<(i32, usize, usize)>,
    mesh: Vec<u64>,
    lights: Vec<(usize, usize)>,
    rendering: String,
}
impl SphereIdentity {
    /// Pin owned mesh bits and scene settings; `SphereIdentity::capture(session, role)?`.
    pub fn capture(session: &ViewerSession, role: &SphereAspectRole) -> io::Result<Self> {
        Ok(Self {
            object: role.object.object_address() as usize,
            camera: camera_bits(session),
            bindings: presentation_bindings(role),
            mesh: mesh_bits(&role.shape()).map_err(refused)?,
            lights: session
                .view()
                .active_lights()
                .iter()
                .map(|light| (light.object_address() as usize, light.revision()))
                .collect(),
            rendering: rendering_snapshot(session)?,
        })
    }
    /// Refuse any restored identity/settings mismatch; `before.require(session, role)?`.
    pub fn require(&self, session: &ViewerSession, role: &SphereAspectRole) -> io::Result<()> {
        let current_role = SphereAspectRole::resolve(session)?;
        let current = Self::capture(session, &current_role)?;
        if current_role.aspect != role.aspect {
            return Err(refused("restored aspect identity differs"));
        }
        if current != *self {
            return Err(refused(format!(
                "restored state={current:?}; expected {self:?}"
            )));
        }
        Ok(())
    }
}
fn mesh_bits(shape: &topods::Shape) -> Result<Vec<u64>, Failure> {
    let mut faces = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)?;
    let mut bits = Vec::new();
    while faces.more() {
        let face = topods::face(faces.current())?;
        let (mesh, _) = BRepTool::triangulation(&face, mesh_purpose::NONE)?;
        let actual = mesh.get().ok_or_else(|| {
            martensite_tkernel::standard::exceptions::program_error(
                "sphere face triangulation null; expected stored mesh",
            )
        })?;
        bits.push(mesh.object_address() as usize as u64);
        append_mesh(&mut bits, actual)?;
        faces.next()?;
    }
    Ok(bits)
}
fn append_mesh(bits: &mut Vec<u64>, mesh: &Triangulation) -> Result<(), Failure> {
    bits.extend([
        mesh.nb_nodes() as u64,
        mesh.nb_triangles() as u64,
        mesh.deflection().to_bits(),
        mesh.has_normals() as u64,
    ]);
    append_nodes(bits, mesh)?;
    append_indices(bits, mesh)
}
fn append_nodes(bits: &mut Vec<u64>, mesh: &Triangulation) -> Result<(), Failure> {
    for i in 1..=mesh.nb_nodes() {
        let point = mesh.node(i)?;
        bits.extend([
            point.x().to_bits(),
            point.y().to_bits(),
            point.z().to_bits(),
        ]);
        if mesh.has_normals() {
            let n = mesh.normal_vec3(i)?;
            bits.extend((0..3).map(|j| u64::from(n[j].to_bits())));
        }
    }
    Ok(())
}
fn append_indices(bits: &mut Vec<u64>, mesh: &Triangulation) -> Result<(), Failure> {
    for i in 1..=mesh.nb_triangles() {
        let tri = mesh.triangle(i)?;
        for j in 1..=3 {
            bits.push(tri.value(j)? as u64);
        }
    }
    Ok(())
}
fn camera_bits(session: &ViewerSession) -> Vec<u64> {
    let camera = session.view().camera();
    let mut bits = camera
        .orientation_matrix()
        .data()
        .map(f64::to_bits)
        .to_vec();
    bits.extend(camera.projection_matrix().data().map(f64::to_bits));
    bits
}
fn presentation_bindings(role: &SphereAspectRole) -> Vec<(i32, usize, usize)> {
    role.object
        .presentations()
        .into_iter()
        .flat_map(|p| {
            let mode = p.mode();
            p.groups().into_iter().map(move |g| {
                (
                    mode,
                    g.object_address() as usize,
                    g.aspects().object_address() as usize,
                )
            })
        })
        .collect()
}

fn rendering_snapshot(session: &ViewerSession) -> io::Result<String> {
    let mut stream = martensite_tkernel::standard::dump::DumpStream::default();
    session
        .view()
        .view_base()
        .rendering_params()
        .dump_json(&mut stream, -1)?;
    Ok(stream.text().to_owned())
}
