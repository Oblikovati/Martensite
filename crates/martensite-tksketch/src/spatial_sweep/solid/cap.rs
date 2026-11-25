// SPDX-License-Identifier: GPL-2.0-only
//! Bind actual closing caps and their affine edges to the authored terminal section.
use super::{
    Failure, SolidSweepHistory, SolidSweepImage, Stage, SweepHistory, SweepOptions, members,
    native, poll,
};
use crate::{
    spatial_prism::volume::{
        affine_curve,
        interval::{Point, Span, l1_difference},
    },
    spatial_sweep::{
        SweepImage,
        admission::Admission,
        geometry::{meridian_error, require_bound, trim_basis, world},
    },
};
use martensite_tkbrep::{
    brep::BRepTool,
    topods::{self, Edge, Face, Shape, Solid},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::{geom::GeomPlane, top_abs::ShapeEnum};
use martensite_tkmath::gp::Axis3;
use martensite_tksketch3d::wire::{DetachedWire, DirectedLineUse};

pub(super) fn bind(
    solid: &Solid,
    sources: [Shape; 2],
    lateral: Vec<SweepImage>,
    profile: &DetachedWire,
    admission: &Admission,
    options: SweepOptions,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<(Vec<SolidSweepImage>, f64), Failure> {
    let final_members = [
        members(solid, ShapeEnum::Face)?,
        members(solid, ShapeEnum::Edge)?,
        members(solid, ShapeEnum::Vertex)?,
    ];
    let [opening, closing] = sources;
    let profile_lines: [DirectedLineUse; 4] = profile
        .edges
        .iter()
        .map(|e| e.source)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| Failure::SolidClosure)?;
    let context = CapContext {
        lateral: &lateral,
        profile: profile_lines,
        admission,
        limit: options.source_trace_limit,
    };
    let (low, low_bound) = context.binding(&opening, &final_members[0], 0)?;
    let (high, high_bound) = context.binding(&closing, &final_members[0], 3)?;
    let mut images = vec![low, high];
    for image in lateral {
        poll(cancelled, Stage::Binding)?;
        images.push(bind_lateral(image, &final_members)?);
    }
    let images_count: usize = final_members.iter().map(IndexedMapOfShape::extent).sum();
    if images.len() != images_count || images[0].shape.is_same(&images[1].shape) {
        return Err(Failure::Binding("a complete final solid member census"));
    }
    Ok((images, low_bound.max(high_bound)))
}

struct CapContext<'a> {
    lateral: &'a [SweepImage],
    profile: [DirectedLineUse; 4],
    admission: &'a Admission,
    limit: f64,
}

impl CapContext<'_> {
    fn binding(
        &self,
        source: &Shape,
        final_faces: &IndexedMapOfShape,
        station: usize,
    ) -> Result<(SolidSweepImage, f64), Failure> {
        let image = retained(source, final_faces)?;
        let native = native(topods::face(&image), Stage::Binding)?;
        let bound = self.verify(&native, station)?;
        let cap = SolidSweepImage {
            shape: image,
            history: SolidSweepHistory::Cap {
                station: self.admission.stations[station].point,
                profile: self.profile,
            },
        };
        Ok((cap, bound))
    }

    fn verify(&self, face: &Face, station: usize) -> Result<f64, Failure> {
        let boundary = members(face, ShapeEnum::Edge)?;
        if boundary.extent() != 4 || members(face, ShapeEnum::Wire)?.extent() != 1 {
            return Err(Failure::SolidClosure);
        }
        let plane = plane(face)?;
        let mut seen = [false; 4];
        let mut maximum = 0.0_f64;
        for i in 1..=boundary.extent() {
            let shape = native(boundary.find_key(i), Stage::Binding)?;
            let source = self.side_source(shape, station)?;
            if std::mem::replace(&mut seen[source], true) {
                return Err(Failure::Binding(
                    "each cap edge on a distinct terminal profile side",
                ));
            }
            let edge = native(topods::edge(shape), Stage::Binding)?;
            let expected = self.endpoints(source, station);
            maximum = maximum.max(check_edge(&edge, face, plane, expected, self.limit)?);
        }
        Ok(maximum)
    }

    fn side_source(&self, shape: &Shape, station: usize) -> Result<usize, Failure> {
        let side = self
            .lateral
            .iter()
            .find(|image| image.shape.is_same(shape))
            .and_then(|image| match &image.history {
                SweepHistory::Edge(h)
                    if h.spine_station == Some(self.admission.stations[station].point) =>
                {
                    h.faces.first()
                }
                _ => None,
            });
        let source =
            side.and_then(|side| self.profile.iter().position(|line| *line == side.profile));
        source.ok_or(Failure::Binding(
            "actual cap edge on its identified terminal ring",
        ))
    }

    fn endpoints(&self, source: usize, station: usize) -> [Point; 2] {
        let frame = if station == 0 { 0 } else { 2 };
        [source, (source + 1) % self.profile.len()].map(|i| {
            world(
                self.admission,
                self.admission.profile.points[i].1,
                frame,
                station,
            )
        })
    }
}

fn plane(face: &Face) -> Result<Axis3, Failure> {
    let (surface, location) = native(BRepTool::surface_with_location(face), Stage::Binding)?;
    let plane = surface
        .downcast_ref::<GeomPlane>()
        .filter(|_| location.is_identity())
        .ok_or(Failure::Geometry("an unlocated planar terminal solid cap"))?;
    Ok(plane.pln().position())
}

fn check_edge(
    edge: &Edge,
    face: &Face,
    plane: Axis3,
    expected: [Point; 2],
    limit: f64,
) -> Result<f64, Failure> {
    let representation = "literal affine native cap boundary mapped to the authored rectangle";
    let spatial = native(BRepTool::curve_with_location(edge), Stage::Binding)?;
    let spatial = affine_curve::spatial(&spatial, false, true, [0.; 3])
        .ok_or(Failure::Geometry(representation))?;
    let trim = native(BRepTool::curve_on_face(edge, face), Stage::Binding)?;
    let trim = trim_basis(trim).ok_or(Failure::Geometry(representation))?;
    let uv = affine_curve::planar(&trim, false, true).ok_or(Failure::Geometry(representation))?;
    let traced = uv.map(|p| point(plane, p));
    let geometry = meridian_error(spatial, expected).max(meridian_error(traced, expected));
    let same_representation = [0, 1]
        .map(|i| l1_difference(traced[i], spatial[i]))
        .into_iter()
        .fold(0., f64::max);
    require_bound(geometry.max(same_representation), limit, representation)
}

pub(super) fn point(plane: Axis3, [u, v]: [Span; 2]) -> Point {
    let o = plane.location();
    let [a, b] = [plane.x_direction(), plane.y_direction()];
    std::array::from_fn(|i| {
        Span::at(o.coordinates()[i])
            + Span::at(a.coordinates()[i]) * u
            + Span::at(b.coordinates()[i]) * v
    })
}

fn bind_lateral(
    image: SweepImage,
    final_members: &[IndexedMapOfShape; 3],
) -> Result<SolidSweepImage, Failure> {
    let index = match image.history {
        SweepHistory::Face(_) => 0,
        SweepHistory::Edge(_) => 1,
        SweepHistory::Vertex(_) => 2,
    };
    Ok(SolidSweepImage {
        shape: retained(&image.shape, &final_members[index])?,
        history: SolidSweepHistory::Lateral(image.history),
    })
}

fn retained(original: &Shape, actual: &IndexedMapOfShape) -> Result<Shape, Failure> {
    let index = actual.find_index(original);
    if index == 0 {
        return Err(Failure::Binding(
            "all original sweep and cap identities retained in the final solid",
        ));
    }
    native(actual.find_key(index), Stage::Binding).cloned()
}
