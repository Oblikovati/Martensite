//! Geometric IGES entities: curves, surfaces, points, and transformation matrices.
mod entities_01;
mod entities_02;
mod entities_03;
mod entities_04;
mod entities_05;
mod entities_06;
mod entities_07;
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
mod tools_12;
mod tools_13;
mod tools_14;
pub use entities_01::{IgesGeomBSplineCurve, IgesGeomBSplineSurface, IgesGeomBoundary};
pub use entities_02::{IgesGeomBoundedSurface, IgesGeomCircularArc, IgesGeomCompositeCurve};
pub use entities_03::IgesGeomConicArc;
pub use entities_04::{
    IgesGeomCopiousData, IgesGeomCurveOnSurface, IgesGeomDirection, IgesGeomFlash, IgesGeomLine,
    IgesGeomOffsetCurve,
};
pub use entities_05::{IgesGeomOffsetSurface, IgesGeomPlane, IgesGeomPoint, IgesGeomRuledSurface};
pub use entities_06::{IgesGeomSplineCurve, IgesGeomSplineSurface};
pub use entities_07::{
    IgesGeomSurfaceOfRevolution, IgesGeomTabulatedCylinder, IgesGeomTransformationMatrix,
    IgesGeomTrimmedSurface,
};
pub use tools_01::{IgesGeomToolBSplineCurve, IgesGeomToolBSplineSurface};
pub use tools_03::IgesGeomToolBoundary;
pub use tools_04::{IgesGeomToolBoundedSurface, IgesGeomToolCircularArc};
pub use tools_05::{IgesGeomToolCompositeCurve, IgesGeomToolConicArc, IgesGeomToolCopiousData};
pub use tools_06::IgesGeomToolCurveOnSurface;
pub use tools_07::{IgesGeomToolDirection, IgesGeomToolFlash};
pub use tools_08::{IgesGeomToolLine, IgesGeomToolOffsetCurve};
pub use tools_09::{IgesGeomToolOffsetSurface, IgesGeomToolPlane, IgesGeomToolPoint};
pub use tools_10::{IgesGeomToolRuledSurface, IgesGeomToolSplineCurve};
pub use tools_11::IgesGeomToolSplineSurface;
pub use tools_13::{
    IgesGeomToolSurfaceOfRevolution, IgesGeomToolTabulatedCylinder,
    IgesGeomToolTransformationMatrix,
};
pub use tools_14::IgesGeomToolTrimmedSurface;
mod modules;
pub use modules::{
    IgesGeom, IgesGeomGeneralModule, IgesGeomProtocol, IgesGeomReadWriteModule,
    IgesGeomSpecificModule,
};
