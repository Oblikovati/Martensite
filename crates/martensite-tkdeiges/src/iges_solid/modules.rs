use super::{
    IgesSolidBlock, IgesSolidBooleanTree, IgesSolidConeFrustum, IgesSolidConicalSurface,
    IgesSolidCylinder, IgesSolidCylindricalSurface, IgesSolidEdgeList, IgesSolidEllipsoid,
    IgesSolidFace, IgesSolidLoop, IgesSolidManifoldSolid, IgesSolidPlaneSurface,
    IgesSolidRightAngularWedge, IgesSolidSelectedComponent, IgesSolidShell, IgesSolidSolidAssembly,
    IgesSolidSolidInstance, IgesSolidSolidOfLinearExtrusion, IgesSolidSolidOfRevolution,
    IgesSolidSphere, IgesSolidSphericalSurface, IgesSolidToolBlock, IgesSolidToolBooleanTree,
    IgesSolidToolConeFrustum, IgesSolidToolConicalSurface, IgesSolidToolCylinder,
    IgesSolidToolCylindricalSurface, IgesSolidToolEdgeList, IgesSolidToolEllipsoid,
    IgesSolidToolFace, IgesSolidToolLoop, IgesSolidToolManifoldSolid, IgesSolidToolPlaneSurface,
    IgesSolidToolRightAngularWedge, IgesSolidToolSelectedComponent, IgesSolidToolShell,
    IgesSolidToolSolidAssembly, IgesSolidToolSolidInstance, IgesSolidToolSolidOfLinearExtrusion,
    IgesSolidToolSolidOfRevolution, IgesSolidToolSphere, IgesSolidToolSphericalSurface,
    IgesSolidToolToroidalSurface, IgesSolidToolTorus, IgesSolidToolVertexList,
    IgesSolidToroidalSurface, IgesSolidTorus, IgesSolidVertexList,
};
use crate::iges_geom::IgesGeom;
use crate::iges_package_tables;
use martensite_tkernel::standard::{Handle, StandardTransient};
use martensite_tkxsbase::interface::InterfaceCategory;

/// The case of a type and form, 0 when not recognised.
fn iges_solid_case_iges(typenum: i32, formnum: i32) -> i32 {
    match (typenum, formnum) {
        (150, _) => 1,
        (152, _) => 13,
        (154, _) => 5,
        (156, _) => 3,
        (158, _) => 20,
        (160, _) => 23,
        (162, _) => 19,
        (164, _) => 18,
        (168, _) => 8,
        (180, _) => 2,
        (182, _) => 14,
        (184, _) => 16,
        (186, _) => 11,
        (190, _) => 12,
        (192, _) => 6,
        (194, _) => 4,
        (196, _) => 21,
        (198, _) => 22,
        (430, _) => 17,
        (502, _) => 24,
        (504, _) => 7,
        (508, _) => 10,
        (510, _) => 9,
        (514, _) => 15,
        _ => 0,
    }
}

/// The category of a case.
fn iges_solid_category(_cn: i32, _ent: &Handle<StandardTransient>) -> i32 {
    InterfaceCategory::number("Shape")
}

iges_package_tables! {
package: IgesSolid,
protocol: IgesSolidProtocol "IgesSolidProtocol",
read_write: IgesSolidReadWriteModule "IgesSolidReadWriteModule",
general: IgesSolidGeneralModule "IgesSolidGeneralModule",
specific: IgesSolidSpecificModule "IgesSolidSpecificModule",
resources: [1 => IgesGeom::protocol()],
prior_init: [IgesGeom::try_init()],
case_iges: iges_solid_case_iges,
category: iges_solid_category,
cases: [
1 => IgesSolidBlock, IgesSolidToolBlock;
2 => IgesSolidBooleanTree, IgesSolidToolBooleanTree;
3 => IgesSolidConeFrustum, IgesSolidToolConeFrustum;
4 => IgesSolidConicalSurface, IgesSolidToolConicalSurface;
5 => IgesSolidCylinder, IgesSolidToolCylinder;
6 => IgesSolidCylindricalSurface, IgesSolidToolCylindricalSurface;
7 => IgesSolidEdgeList, IgesSolidToolEdgeList;
8 => IgesSolidEllipsoid, IgesSolidToolEllipsoid;
9 => IgesSolidFace, IgesSolidToolFace;
10 => IgesSolidLoop, IgesSolidToolLoop;
11 => IgesSolidManifoldSolid, IgesSolidToolManifoldSolid;
12 => IgesSolidPlaneSurface, IgesSolidToolPlaneSurface;
13 => IgesSolidRightAngularWedge, IgesSolidToolRightAngularWedge;
14 => IgesSolidSelectedComponent, IgesSolidToolSelectedComponent;
15 => IgesSolidShell, IgesSolidToolShell;
16 => IgesSolidSolidAssembly, IgesSolidToolSolidAssembly;
17 => IgesSolidSolidInstance, IgesSolidToolSolidInstance;
18 => IgesSolidSolidOfLinearExtrusion, IgesSolidToolSolidOfLinearExtrusion;
19 => IgesSolidSolidOfRevolution, IgesSolidToolSolidOfRevolution;
20 => IgesSolidSphere, IgesSolidToolSphere;
21 => IgesSolidSphericalSurface, IgesSolidToolSphericalSurface;
22 => IgesSolidToroidalSurface, IgesSolidToolToroidalSurface;
23 => IgesSolidTorus, IgesSolidToolTorus;
24 => IgesSolidVertexList, IgesSolidToolVertexList;
],
corrected: [
],
new_void: [
1 => Handle::new(IgesSolidBlock::new()).cast();
2 => Handle::new(IgesSolidBooleanTree::new()).cast();
3 => Handle::new(IgesSolidConeFrustum::new()).cast();
4 => Handle::new(IgesSolidConicalSurface::new()).cast();
5 => Handle::new(IgesSolidCylinder::new()).cast();
6 => Handle::new(IgesSolidCylindricalSurface::new()).cast();
7 => Handle::new(IgesSolidEdgeList::new()).cast();
8 => Handle::new(IgesSolidEllipsoid::new()).cast();
9 => Handle::new(IgesSolidFace::new()).cast();
10 => Handle::new(IgesSolidLoop::new()).cast();
11 => Handle::new(IgesSolidManifoldSolid::new()).cast();
12 => Handle::new(IgesSolidPlaneSurface::new()).cast();
13 => Handle::new(IgesSolidRightAngularWedge::new()).cast();
14 => Handle::new(IgesSolidSelectedComponent::new()).cast();
15 => Handle::new(IgesSolidShell::new()).cast();
16 => Handle::new(IgesSolidSolidAssembly::new()).cast();
17 => Handle::new(IgesSolidSolidInstance::new()).cast();
18 => Handle::new(IgesSolidSolidOfLinearExtrusion::new()).cast();
19 => Handle::new(IgesSolidSolidOfRevolution::new()).cast();
20 => Handle::new(IgesSolidSphere::new()).cast();
21 => Handle::new(IgesSolidSphericalSurface::new()).cast();
22 => Handle::new(IgesSolidToroidalSurface::new()).cast();
23 => Handle::new(IgesSolidTorus::new()).cast();
24 => Handle::new(IgesSolidVertexList::new()).cast();
],
general_extra: {}
}
