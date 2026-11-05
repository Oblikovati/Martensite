//! Procedural counterparts of the frozen native census; seeded images are raw stage inputs.
use super::{BOPAlgoBuilder, DataStructure, Face, Shape};
use crate::{bop_ds::InterfFF, int_tools::Context};
use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    topexp::Explorer,
};
use martensite_tkernel::{ncollection::List, standard::Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Axis3, Cylinder3, Plane3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SameDomainFixtureKind {
    NoIntersections,
    Filtered,
    Planar,
    Cylinder,
    MissingSurface,
    SameParent,
    MultiPair,
}

pub(super) struct SameDomainFixture {
    pub builder: BOPAlgoBuilder,
    pub structure: DataStructure,
    pub sources: Vec<Face>,
    pub pieces: Vec<Face>,
}

/// Retain actual construction identities; `same_domain_fixture(SameDomainFixtureKind::Cylinder)`.
pub(super) fn same_domain_fixture(kind: SameDomainFixtureKind) -> SameDomainFixture {
    let (sources, arguments) = source_faces(kind);
    let mut structure = structure_from_arguments(arguments);
    initialize_intersections(kind, &mut structure, &sources);
    let mut builder = configured_builder(kind);
    let pieces = seeded_pieces(kind, &sources);
    for (source, piece) in sources.iter().zip(&pieces) {
        bind_image(&mut builder, source, piece);
    }
    SameDomainFixture {
        builder,
        structure,
        sources,
        pieces,
    }
}

fn source_faces(kind: SameDomainFixtureKind) -> (Vec<Face>, Vec<Shape>) {
    if kind == SameDomainFixtureKind::SameParent {
        return box_source_faces();
    }
    let first = same_domain_seed(kind);
    let mut sources = vec![first.clone(), surface_twin(&first, false)];
    if kind == SameDomainFixtureKind::MultiPair {
        sources.push(surface_twin(&first, false));
    }
    let arguments = sources.iter().cloned().map(Face::into_shape).collect();
    (sources, arguments)
}

fn box_source_faces() -> (Vec<Face>, Vec<Shape>) {
    let mut maker = BRepPrimAPIMakeBox::from_sizes(2., 3., 4.).unwrap();
    let solid = maker.shape().unwrap().clone();
    let sources = vec![maker.bottom_face().unwrap(), maker.top_face().unwrap()];
    (sources, vec![solid])
}

fn planar_face() -> Face {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (0., 2., 0., 3.))
        .unwrap()
        .face()
        .unwrap()
        .clone()
}

fn cylindrical_face() -> Face {
    let cylinder = Cylinder3::new(Axis3::default(), 2.).unwrap();
    BRepBuilderAPIMakeFace::from_cylinder_bounds(cylinder, (0., 1., 0., 3.))
        .unwrap()
        .face()
        .unwrap()
        .clone()
}

/// Share actual wire and surface identity; `surface_twin(&face, false)` is a distinct Face TShape.
pub(super) fn surface_twin(source: &Face, missing_surface: bool) -> Face {
    let builder = BRepBuilder::default();
    let mut twin = Face::default();
    if missing_surface {
        builder.make_face(&mut twin);
    } else {
        builder
            .make_face_on_surface(&mut twin, &BRepTool::surface(source).unwrap(), 1.0e-7)
            .unwrap();
    }
    let mut shape = twin.into_shape();
    for wire in Explorer::new(source, ShapeEnum::Wire, ShapeEnum::Shape).unwrap() {
        builder.add(&mut shape, &wire).unwrap();
    }
    Face::unchecked(shape)
}

fn structure_from_arguments(arguments: Vec<Shape>) -> DataStructure {
    let mut structure = DataStructure::default();
    structure.set_arguments(&arguments);
    structure.init(1.0e-7).unwrap();
    structure
}

fn add_intersection(structure: &mut DataStructure, first: &Face, second: &Face, face_info: bool) {
    let first = structure.index(first);
    let second = structure.index(second);
    let mut record = InterfFF::default();
    record.set_indices(first, second);
    structure.interf_ff().append(record);
    if !face_info {
        return;
    }
    structure.change_face_info(first).unwrap();
    structure.change_face_info(second).unwrap();
}

fn seeded_pieces(kind: SameDomainFixtureKind, sources: &[Face]) -> Vec<Face> {
    if kind == SameDomainFixtureKind::MissingSurface {
        return vec![
            surface_twin(&sources[0], true),
            surface_twin(&sources[1], false),
        ];
    }
    if kind == SameDomainFixtureKind::SameParent {
        let first = planar_face();
        return vec![first.clone(), surface_twin(&first, false)];
    }
    Vec::new()
}

/// Seed only raw intermediate relations; `bind_image(builder, source, source)` is not publication.
pub(super) fn bind_image(builder: &mut BOPAlgoBuilder, source: &Face, piece: &Face) {
    let mut images = List::default();
    images.append(piece.clone().into_shape());
    builder.images.bound(source.clone().into_shape(), images);
}

fn initialize_intersections(
    kind: SameDomainFixtureKind,
    structure: &mut DataStructure,
    sources: &[Face],
) {
    if kind != SameDomainFixtureKind::NoIntersections {
        add_intersection(
            structure,
            &sources[0],
            &sources[1],
            kind != SameDomainFixtureKind::Filtered,
        );
    }
    if sources.len() == 3 {
        add_intersection(structure, &sources[1], &sources[2], true);
    }
}

fn configured_builder(kind: SameDomainFixtureKind) -> BOPAlgoBuilder {
    let mut builder = BOPAlgoBuilder::new();
    if !matches!(
        kind,
        SameDomainFixtureKind::NoIntersections | SameDomainFixtureKind::Filtered
    ) {
        builder.context = Handle::new(Context::default());
    }
    builder
}

fn same_domain_seed(kind: SameDomainFixtureKind) -> Face {
    if matches!(
        kind,
        SameDomainFixtureKind::NoIntersections
            | SameDomainFixtureKind::Filtered
            | SameDomainFixtureKind::Planar
    ) {
        return planar_face();
    }
    cylindrical_face()
}
