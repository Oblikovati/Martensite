// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{self, Edge, Face, Shell, Solid, Wire},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkhlr::parting::{
    PartingEdgeBinding, PartingEdgeId, PartingFaceBinding, PartingFaceId, PartingScope,
    PartingSource,
};
use martensite_tkmath::gp::{Axis2, Point3, Vector3};
use martensite_tkprim::brep_prim_api::{
    BRepPrimAPIMakeBox, BRepPrimAPIMakeCone, BRepPrimAPIMakeCylinder, BRepPrimAPIMakePrism,
};
use martensite_tktopalgo::brep_builder_api::{
    BRepBuilderAPIMakeFace, BRepBuilderAPIMakePolygon, BRepBuilderAPIMakeShape,
};
use std::num::NonZeroU64;

pub(super) const SCOPE: PartingScope = PartingScope {
    document: [19; 16],
    solid: NonZeroU64::MIN,
    revision: 27,
};

pub(super) struct Model {
    pub solid: Solid,
    pub faces: Vec<Face>,
    pub edges: Vec<Edge>,
}

impl Model {
    pub fn new(solid: Solid) -> Self {
        let faces = Explorer::new(&solid, ShapeEnum::Face, ShapeEnum::Shape)
            .unwrap()
            .map(|shape| topods::face(&shape).unwrap())
            .collect();
        let mut edges = Vec::<Edge>::new();
        for shape in Explorer::new(&solid, ShapeEnum::Edge, ShapeEnum::Shape).unwrap() {
            if !edges.iter().any(|edge| edge.is_same(&shape)) {
                edges.push(topods::edge(&shape).unwrap());
            }
        }
        Self {
            solid,
            faces,
            edges,
        }
    }

    pub fn bindings(&self) -> (Vec<PartingFaceBinding<'_>>, Vec<PartingEdgeBinding<'_>>) {
        let next_id = |index: usize| NonZeroU64::new(index as u64 + 1).unwrap();
        let faces = self
            .faces
            .iter()
            .enumerate()
            .map(|(index, occurrence)| PartingFaceBinding {
                id: PartingFaceId(next_id(index)),
                occurrence,
            })
            .collect();
        let edges = self
            .edges
            .iter()
            .enumerate()
            .map(|(index, occurrence)| PartingEdgeBinding {
                id: PartingEdgeId(next_id(index)),
                occurrence,
            })
            .collect();
        (faces, edges)
    }

    pub fn source<'a>(
        &'a self,
        faces: &'a [PartingFaceBinding<'a>],
        edges: &'a [PartingEdgeBinding<'a>],
    ) -> PartingSource<'a> {
        PartingSource {
            scope: SCOPE,
            solid: &self.solid,
            faces,
            edges,
        }
    }
}

pub(super) fn block() -> Solid {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(60., 40., 10.).unwrap();
    topods::solid(maker.shape().unwrap()).unwrap()
}

pub(super) fn cylinder(radius: f64, height: f64) -> Solid {
    let mut maker = BRepPrimAPIMakeCylinder::new(radius, height);
    topods::solid(maker.shape().unwrap()).unwrap()
}

pub(super) fn incomplete_wall() -> Solid {
    let mut maker = BRepPrimAPIMakeCylinder::with_axes_angle(
        Axis2::default(),
        5.,
        10.,
        std::f64::consts::FRAC_PI_2,
    )
    .unwrap();
    topods::solid(maker.shape().unwrap()).unwrap()
}

pub(super) fn cone() -> Solid {
    let mut maker = BRepPrimAPIMakeCone::new(5., 4., 10.).unwrap();
    topods::solid(maker.shape().unwrap()).unwrap()
}

pub(super) fn cavity_box(min: Point3, max: Point3) -> Solid {
    let mut outer = BRepPrimAPIMakeBox::from_sizes(60., 40., 10.).unwrap();
    let mut cavity = BRepPrimAPIMakeBox::from_points(min, max).unwrap();
    let outer = outer.shell().unwrap();
    let cavity = Shell::unchecked(cavity.shell().unwrap().reversed());
    let builder = BRepBuilder::default();
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    builder.add(&mut solid, &outer).unwrap();
    builder.add(&mut solid, &cavity).unwrap();
    solid
}

pub(super) fn through_hole() -> Solid {
    let outer = square_wire([[0., 0.], [60., 0.], [60., 40.], [0., 40.]]);
    let hole = square_wire([[4., 3.], [4., 7.], [8., 7.], [8., 3.]]);
    let mut face = BRepBuilderAPIMakeFace::from_wire(&outer, true).unwrap();
    face.add(&hole).unwrap();
    let face = face.face().unwrap();
    let mut prism =
        BRepPrimAPIMakePrism::new(&face, Vector3::new(0., 0., 10.), false, true).unwrap();
    topods::solid(prism.shape().unwrap()).unwrap()
}

fn square_wire(corners: [[f64; 2]; 4]) -> Wire {
    let mut polygon = BRepBuilderAPIMakePolygon::new();
    for [x, y] in corners {
        polygon.add_point(Point3::new(x, y, 0.)).unwrap();
    }
    polygon.close().unwrap();
    polygon.wire().unwrap()
}
