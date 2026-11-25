//! Source-bound conical attachment with a single immutable publication.
use super::*;
use crate::surface_sketch::WrappedHistoryRecord;
use martensite_tkbo::profile_extrusion::Resolution;
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools::outer_wire,
    topexp::Explorer,
    topods::{self, Builder, ShapeIterator},
};
use martensite_tkg3d::geom::{GeomConicalSurface, GeomPlane};

mod api;
mod assembly;
mod contact;
mod geometry;
mod publication;
mod result;
mod retained;
mod seams;
mod stock;
mod stock_binding;
mod stock_uv;
mod topology;

use assembly::{Frustum, Member, direct_children, occurrences, sort_stock_disks};
pub use result::{CertifiedConeAttachmentReport, CertifiedConeFeatureResult};
use stock::StockFrustumReceipt;
use stock_binding::{InstalledStock, stock_refusal};

const ASSEMBLY: CylinderFeatureStage = CylinderFeatureStage::Boolean;

struct Inputs<'a> {
    attempt: Attempt,
    stock: &'a Shape,
    cone: &'a Face,
    tool: &'a DetachedWrappedBodies,
    retained: &'a retained::Retained,
    records: &'a [WrappedHistoryRecord],
    regions: &'a [SurfaceRegionId],
    resolution: &'a Resolution,
}

fn refused(attempt: Attempt, expected: &'static str) -> CylinderFeatureFailure {
    attempt.fail(
        ASSEMBLY,
        CylinderFeatureCause::DirectAttachment { expected },
    )
}
