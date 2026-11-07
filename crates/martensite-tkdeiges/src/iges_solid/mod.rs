//! Boundary representation and constructive solid IGES entities.
mod entities_01;
mod entities_02;
mod entities_03;
#[cfg(test)]
pub(crate) mod samples_01;
#[cfg(test)]
mod tests_01;
mod tools_01;
mod tools_02;
mod tools_03;
mod tools_04;
mod tools_05;
mod tools_06;
mod tools_07;
mod tools_08;
mod tools_09;
mod tools_10;
mod tools_11;
pub use entities_01::{
    IgesSolidBlock, IgesSolidBooleanTree, IgesSolidConeFrustum, IgesSolidConicalSurface,
    IgesSolidCylinder, IgesSolidCylindricalSurface, IgesSolidEdgeList, IgesSolidEllipsoid,
    IgesSolidFace,
};
pub use entities_02::{
    IgesSolidLoop, IgesSolidManifoldSolid, IgesSolidPlaneSurface, IgesSolidRightAngularWedge,
    IgesSolidSelectedComponent, IgesSolidShell, IgesSolidSolidAssembly,
};
pub use entities_03::{
    IgesSolidSolidInstance, IgesSolidSolidOfLinearExtrusion, IgesSolidSolidOfRevolution,
    IgesSolidSphere, IgesSolidSphericalSurface, IgesSolidToroidalSurface, IgesSolidTorus,
    IgesSolidVertexList,
};
pub use tools_01::{IgesSolidToolBlock, IgesSolidToolBooleanTree, IgesSolidToolConeFrustum};
pub use tools_02::{
    IgesSolidToolConicalSurface, IgesSolidToolCylinder, IgesSolidToolCylindricalSurface,
};
pub use tools_03::{IgesSolidToolEdgeList, IgesSolidToolEllipsoid};
pub use tools_04::{IgesSolidToolFace, IgesSolidToolLoop};
pub use tools_05::IgesSolidToolManifoldSolid;
pub use tools_06::{IgesSolidToolPlaneSurface, IgesSolidToolRightAngularWedge};
pub use tools_07::{IgesSolidToolSelectedComponent, IgesSolidToolShell};
pub use tools_08::{
    IgesSolidToolSolidAssembly, IgesSolidToolSolidInstance, IgesSolidToolSolidOfLinearExtrusion,
};
pub use tools_09::{
    IgesSolidToolSolidOfRevolution, IgesSolidToolSphere, IgesSolidToolSphericalSurface,
};
pub use tools_10::{IgesSolidToolToroidalSurface, IgesSolidToolTorus};
pub use tools_11::IgesSolidToolVertexList;
mod modules;
pub use modules::{
    IgesSolid, IgesSolidGeneralModule, IgesSolidProtocol, IgesSolidReadWriteModule,
    IgesSolidSpecificModule,
};
