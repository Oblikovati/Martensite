//! Re-site a representative seam on the same analytic Circle→Extrude stock (#420).
use super::*;
use martensite_tkbrep::{
    brep::{
        BRepBuilder, BRepTool, EdgeCurve3D,
        access::{edge_tshape, face_tshape, vertex_tshape},
    },
    topexp,
    topods::{self, Builder, Edge, ShapeIterator, Vertex},
};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::{Geom2dCircle, Geom2dCurve, Geom2dLine};
use martensite_tkg3d::geom::{GeomCircle, GeomCylindricalSurface, GeomLine, GeomPlane};
use martensite_tkmath::gp::{
    Direction2, Direction3, Point2, Point3, StandardDirection2, StandardDirection3,
};
use std::f64::consts::TAU;

mod frozen;
mod inherited;
pub(super) use inherited::{UnchangedStock, certify_unchanged};
mod lift;
mod source;
#[cfg(test)]
mod tests;

const STAGE: CylinderFeatureStage = CylinderFeatureStage::Boolean;
const UP: [f64; 3] = [0., 0., 1.];
const X: [f64; 3] = [1., 0., 0.];
const Y: [f64; 3] = [0., 1., 0.];

pub(super) struct RecutStock {
    source: Shape,
    selected: Face,
    receipt: Vec<u8>,
    pub(super) solid: Shape,
    pub(super) band: Face,
    pub(super) sheet: [f64; 2],
    pub(super) clear_distance: f64,
    pub(super) spatial_upper: f64,
    pub(super) images: Vec<StockImage>,
}

impl RecutStock {
    pub(super) fn matches_source(&self, stock: &Shape, selected: &Face) -> bool {
        self.source.is_equal(stock) && self.selected.is_equal(selected)
    }
}

pub(super) struct StockImage {
    pub(super) source: CylinderBodyRef,
    pub(super) image: Shape,
    pub(super) relation: CylinderFeatureRelation,
}

pub(super) fn prepare(
    attempt: Attempt,
    stock: &Shape,
    side: &Face,
    native_u_hull: [f64; 2],
    sew: f64,
) -> FeatureChecked<RecutStock> {
    let attempt = RecutCheck(attempt);
    if !sew.is_finite() || sew <= 0. {
        return Err(attempt.refusal("finite positive sewing resolution"));
    }
    let source = source::Stock::capture(attempt, stock, side)?;
    let (generator_x, stock_error) = sum_with_residual(source.origin[0], -source.radius);
    let cut = lift::Cut::opposite(attempt, native_u_hull, source.radius, stock_error, sew)?;
    if cut.residual >= source.tolerance.min(sew) {
        return Err(attempt.refusal("replacement seam below the original stock and sewing limits"));
    }
    let installed = install(attempt, &source, &cut, generator_x)?;
    finish(attempt, stock, &source, installed, cut)
}

#[derive(Clone, Copy)]
struct RecutCheck(Attempt);

impl RecutCheck {
    fn native<T>(self, query: &'static str, result: Result<T, Failure>) -> FeatureChecked<T> {
        self.0.native(STAGE, query, result)
    }

    fn refusal(self, expected: &'static str) -> CylinderFeatureFailure {
        self.0
            .fail(STAGE, CylinderFeatureCause::DirectAttachment { expected })
    }
}

fn children(attempt: RecutCheck, parent: &Shape) -> FeatureChecked<Vec<Shape>> {
    let mut iter = ShapeIterator::new(parent, true, true);
    let mut output = Vec::new();
    while iter.more() {
        let child = attempt.native("native direct child", iter.value())?;
        output.push(child.to_shape());
        iter.next();
    }
    Ok(output)
}

fn sum_with_residual(a: f64, b: f64) -> (f64, f64) {
    let (large, small) = if a.abs() >= b.abs() { (a, b) } else { (b, a) };
    let sum = large + small;
    // FastTwoSum bounds the exact difference from the stored sum.
    (sum, (small - (sum - large)).abs())
}

fn line2(point: Point2, dir: StandardDirection2) -> Handle<dyn Geom2dCurve> {
    Handle::new(Geom2dLine::from_point(
        point,
        Direction2::from_standard(dir),
    ))
    .into_upcast()
}

struct Installed {
    band: Face,
    ends: [Face; 2],
    rims: [Edge; 2],
    anchors: [Vertex; 2],
    seam: Edge,
}

fn install(
    attempt: RecutCheck,
    stock: &source::Stock<'_>,
    cut: &lift::Cut,
    x: f64,
) -> FeatureChecked<Installed> {
    let band = empty_face(attempt, stock.side)?;
    let ends = [
        empty_face(attempt, &stock.ends[0].face)?,
        empty_face(attempt, &stock.ends[1].face)?,
    ];
    let a = install_rim(attempt, &stock.ends[0], &ends[0], &band, x, cut.range)?;
    let b = install_rim(attempt, &stock.ends[1], &ends[1], &band, x, cut.range)?;
    let anchors = [a.0, b.0];
    let rims = [a.1, b.1];
    let seam = install_seam(attempt, stock, &band, &anchors, x, cut.range)?;
    Ok(Installed {
        band,
        ends,
        rims,
        anchors,
        seam,
    })
}

fn empty_face(attempt: RecutCheck, source: &Face) -> FeatureChecked<Face> {
    let shape = attempt.native("preserve the original stock carrier", source.empty_copied())?;
    attempt.native("stock carrier face", topods::face(&shape))
}

fn install_rim(
    attempt: RecutCheck,
    source: &source::Rim,
    cap: &Face,
    band: &Face,
    x: f64,
    range: [f64; 2],
) -> FeatureChecked<(Vertex, Edge)> {
    let builder = BRepBuilder::default();
    let [_, y, z] = source.center;
    let mut anchor = Vertex::default();
    let mut rim = Edge::default();
    attempt.native(
        "shifted rim anchor",
        builder.make_vertex_at(&mut anchor, Point3::new(x, y, z), source.vertex_tol),
    )?;
    attempt.native(
        "retain literal circle",
        builder.make_edge_with_located_curve(
            &mut rim,
            &source.curve.curve,
            &source.curve.location,
            source.edge_tol,
        ),
    )?;
    let lateral = line2(Point2::new(0., source.v), StandardDirection2::X);
    attempt.native(
        "native lateral circle trim",
        builder.update_edge_pcurve_on_face(&rim, &lateral, band, source.edge_tol),
    )?;
    attempt.native(
        "retain literal disk circle trim",
        builder.update_edge_pcurve_on_face(&rim, &source.cap_pcurve, cap, source.edge_tol),
    )?;
    finish_edge(attempt, &rim, [&anchor, &anchor], range)?;
    Ok((anchor, rim))
}

fn install_seam(
    attempt: RecutCheck,
    stock: &source::Stock<'_>,
    band: &Face,
    anchors: &[Vertex; 2],
    x: f64,
    [lo, hi]: [f64; 2],
) -> FeatureChecked<Edge> {
    let [_, y, z] = stock.origin;
    let builder = BRepBuilder::default();
    let mut seam = Edge::default();
    let line = GeomLine::from_point(
        Point3::new(x, y, z),
        Direction3::from_standard(StandardDirection3::Z),
    );
    let curve = Handle::new(line).into_upcast();
    attempt.native(
        "antipodal generator",
        builder.make_edge_with_curve(&mut seam, &curve, stock.seam_tol),
    )?;
    let trims = [lo, hi].map(|angle| line2(Point2::new(angle, 0.), StandardDirection2::NegY));
    let [a, b] = stock.seam_order.map(|i| &trims[i]);
    attempt.native(
        "store both directed seam trims",
        builder.update_edge_pcurves_on_face(&seam, (a, b), band, stock.seam_tol),
    )?;
    finish_edge(
        attempt,
        &seam,
        [&anchors[stock.base], &anchors[1 - stock.base]],
        [0., stock.height],
    )?;
    Ok(seam)
}

fn finish_edge(
    attempt: RecutCheck,
    edge: &Edge,
    [first, last]: [&Vertex; 2],
    [lo, hi]: [f64; 2],
) -> FeatureChecked<()> {
    let mut edge_use = edge.to_edge();
    let builder = BRepBuilder::default();
    attempt.native(
        "edge starting vertex",
        Builder.add(&mut edge_use, &first.oriented(Orientation::Forward)),
    )?;
    attempt.native(
        "edge ending vertex",
        Builder.add(&mut edge_use, &last.oriented(Orientation::Reversed)),
    )?;
    attempt.native("common 3D and UV range", builder.range(edge, lo, hi, false))?;
    attempt.native("common edge parameter", builder.same_parameter(edge, true))?;
    attempt.native("common edge range", builder.same_range(edge, true))
}

fn finish(
    attempt: RecutCheck,
    stock: &Shape,
    source: &source::Stock<'_>,
    mut built: Installed,
    cut: lift::Cut,
) -> FeatureChecked<RecutStock> {
    let swaps = [
        (&source.ends[0].edge, &built.rims[0]),
        (&source.ends[1].edge, &built.rims[1]),
        (&source.seam, &built.seam),
    ];
    attach_wire(attempt, &source.side_wire, &mut built.band, &swaps)?;
    for (end, target) in source.ends.iter().zip(&mut built.ends) {
        attach_wire(attempt, &end.wire, target, &swaps)?;
    }
    let solid = close_stock(attempt, &source.shell, &built)?;
    let images = bind_images(attempt, stock, source, &built)?;
    let receipt = frozen::stock_receipt(
        attempt.0,
        &built.band,
        &built.seam,
        non_band_images(&built.band, &images),
    )?;
    Ok(RecutStock {
        source: stock.clone(),
        selected: source.side.clone(),
        receipt,
        solid,
        band: built.band,
        sheet: cut.range,
        clear_distance: cut.clear_distance,
        spatial_upper: cut.residual,
        images,
    })
}

fn attach_wire(
    attempt: RecutCheck,
    source: &Shape,
    face: &mut Face,
    swaps: &[(&Edge, &Edge); 3],
) -> FeatureChecked<()> {
    let mut wire = attempt.native("copy stock wire occurrence", source.empty_copied())?;
    for edge in children(attempt, source)? {
        let (_, image) = swaps
            .iter()
            .find(|(old, _)| edge.is_same(old))
            .ok_or_else(|| attempt.refusal("known original rim or periodic seam occurrence"))?;
        let occurrence = image.oriented(edge.orientation());
        attempt.native(
            "preserve directed replaced stock coedge",
            Builder.add(&mut wire, &occurrence),
        )?;
    }
    attempt.native("closed stock trim", wire.set_closed(true))?;
    attempt.native("install stock trim", Builder.add(face, &wire))
}

fn close_stock(
    attempt: RecutCheck,
    source: &Shape,
    installed: &Installed,
) -> FeatureChecked<Shape> {
    let mut shell = attempt.native("preserve source shell orientation", source.empty_copied())?;
    for face in std::iter::once(&installed.band).chain(&installed.ends) {
        attempt.native("stock face occurrence", Builder.add(&mut shell, face))?;
    }
    attempt.native("closed stock shell", shell.set_closed(true))?;
    let mut solid = topods::Solid::default();
    Builder.make_solid(&mut solid);
    attempt.native("stock solid shell", Builder.add(&mut solid, &shell))?;
    Ok(solid.to_shape())
}

fn bind_images(
    attempt: RecutCheck,
    stock: &Shape,
    source: &source::Stock<'_>,
    built: &Installed,
) -> FeatureChecked<Vec<StockImage>> {
    use CylinderFeatureRelation::{Generated, Modified};
    let index = body::Index::new(attempt.0, stock, STAGE)?;
    let mut images = Vec::new();
    let mut bind = |old: &Shape, image: &Shape, relation| {
        let source = index
            .reference(old, attempt.0)?
            .ok_or_else(|| attempt.refusal("original stock occurrence-to-image binding"))?;
        images.push(StockImage {
            source,
            image: image.to_shape(),
            relation,
        });
        Ok(())
    };
    bind(source.side, &built.band, Modified)?;
    bind(&source.seam, &built.seam, Generated)?;
    for (i, end) in source.ends.iter().enumerate() {
        bind(&end.face, &built.ends[i], Modified)?;
        bind(&end.edge, &built.rims[i], Modified)?;
        bind(&end.anchor, &built.anchors[i], Generated)?;
    }
    Ok(images)
}

fn non_band_images<'a>(
    band: &'a Face,
    images: &'a [StockImage],
) -> impl Iterator<Item = &'a Shape> {
    images
        .iter()
        .filter(|link| !link.image.is_same(band))
        .map(|link| &link.image)
}
