//! Walking segments are replaced only when they follow a complete domain restriction.
mod build;
mod groups;
mod merge;
pub(super) use groups::convert_groups;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkgeomalgo::{
    int_patch::{IntPatchPoint, IntPatchPointLine, IntPatchWLine},
    int_start::TheArc,
};
use martensite_tkmath::gp::Point2;
use martensite_tktopalgo::brep_top_adaptor::BRepTopAdaptorTopolTool;
pub(super) use merge::merge_restriction_segments;

#[derive(Clone, Copy)]
pub(super) struct Side<'a> {
    pub(super) surface: &'a Handle<BRepAdaptorSurface>,
    pub(super) domain: &'a BRepTopAdaptorTopolTool,
    pub(super) first: bool,
    pub(super) tolerance: f64,
}

impl Side<'_> {
    fn parameters(&self, point: &IntPatchPoint) -> (f64, f64) {
        if self.first {
            point.parameters_on_s1()
        } else {
            point.parameters_on_s2()
        }
    }

    fn is_vertex(&self, point: &IntPatchPoint) -> bool {
        if self.first {
            point.is_vertex_on_s1()
        } else {
            point.is_vertex_on_s2()
        }
    }

    fn on_restriction(&self, line: &IntPatchWLine) -> Result<bool, Failure> {
        let mut along = 0;
        for index in 1..=line.nb_pnts()? {
            let (u, v) = line.point(index)?.parameters_on_surface(self.first);
            self.surface.d1(u, v)?;
            if self
                .domain
                .is_the_point_on(Point2::new(u, v), self.tolerance, true)?
            {
                along += 1;
            }
        }
        Ok(along == line.nb_pnts()?)
    }

    fn appendable(&self, line: &IntPatchWLine) -> Result<bool, Failure> {
        let has_arc = if self.first {
            line.has_arc_on_s1()
        } else {
            line.has_arc_on_s2()
        };
        Ok(has_arc || self.on_restriction(line)?)
    }

    fn project(&self, arc: &TheArc, point: &IntPatchPoint) -> Result<f64, Failure> {
        let (u, v) = self.parameters(point);
        martensite_tkgeomalgo::geom2d_int::TheProjPCurOfGInter::find_parameter(
            &**arc,
            Point2::new(u, v),
            1e-7,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use martensite_tkgeomalgo::int_surf::IntSurfPntOn2S;
    use martensite_tkgeomalgo::{
        int_patch::{IntPatchLine, IntPatchRLine},
        int_surf::IntSurfLineOn2S,
    };
    use martensite_tkmath::gp::Plane3;
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeFace;

    fn context() -> (Handle<BRepAdaptorSurface>, BRepTopAdaptorTopolTool) {
        let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-1., 1., -1., 1.))
            .unwrap()
            .face()
            .unwrap();
        let surface = Handle::new(BRepAdaptorSurface::with_face(&face, true).unwrap());
        let domain = BRepTopAdaptorTopolTool::with_surface(&surface.cast()).unwrap();
        (surface, domain)
    }

    fn walking(side: Side<'_>, reverse: bool) -> Handle<dyn IntPatchLine> {
        side.domain.init();
        let arc = side.domain.value().unwrap();
        side.domain.initialize_curve(&arc).unwrap();
        side.domain.init_vertex_iterator().unwrap();
        let first = side.domain.vertex().unwrap();
        side.domain.next_vertex().unwrap();
        let last = side.domain.vertex().unwrap();
        let (first, last) = if reverse {
            (last, first)
        } else {
            (first, last)
        };
        let params = (
            first.parameter(&arc).unwrap(),
            last.parameter(&arc).unwrap(),
        );
        let points = Handle::new(IntSurfLineOn2S::new(None));
        for parameter in [params.0, (params.0 + params.1) / 2., params.1] {
            let uv = arc.value(parameter).unwrap();
            let mut point = IntSurfPntOn2S::default();
            point.set_value_all(
                side.surface.value(uv.x(), uv.y()).unwrap(),
                uv.x(),
                uv.y(),
                uv.x(),
                uv.y(),
            );
            points.add(&point);
        }
        let line = Handle::new(IntPatchWLine::new_undecided(&points, false));
        for (index, vertex) in [(1, first), (3, last)] {
            let point = points.value(index).unwrap();
            let mut endpoint = IntPatchPoint::new();
            endpoint.set_value_tangent(point.value(), 1e-7, false);
            let (u1, v1, u2, v2) = point.parameters();
            endpoint.set_parameters(u1, v1, u2, v2);
            endpoint.set_parameter(f64::from(index));
            endpoint.set_vertex(side.first, &vertex);
            line.add_vertex(&endpoint, false);
        }
        line.cast()
    }

    #[test]
    fn whole_boundary_walking_line_becomes_a_restriction_in_either_direction() {
        for reverse in [false, true] {
            let (surface, domain) = context();
            let side = Side {
                surface: &surface,
                domain: &domain,
                first: true,
                tolerance: 1e-7,
            };
            let line = walking(side, reverse);
            assert!(side.on_restriction(&line.cast::<IntPatchWLine>()).unwrap());
            let mut lines = vec![line];
            convert_groups(
                &mut lines,
                [
                    side,
                    Side {
                        first: false,
                        ..side
                    },
                ],
            )
            .unwrap();
            assert_eq!(lines.len(), 1);
            let restriction = lines[0].cast::<IntPatchRLine>();
            assert!(!restriction.is_null());
            assert!(restriction.is_arc_on_s1());
            assert_eq!(restriction.nb_pnts().unwrap(), 3);
            assert_eq!(restriction.nb_vertex(), 2);
            let first = restriction.vertex(1).unwrap().parameter_on_line();
            let last = restriction.vertex(2).unwrap().parameter_on_line();
            assert!(first < last);
        }
    }
    #[test]
    fn disjoint_boundary_segments_merge_through_gap_samples_and_preserve_hundred_point_source_polygon()
     {
        let (surface, domain) = context();
        let side = Side {
            surface: &surface,
            domain: &domain,
            first: true,
            tolerance: 1e-7,
        };
        let mut segments = separated_segments(side);
        merge_restriction_segments(
            &mut segments,
            [
                side,
                Side {
                    first: false,
                    ..side
                },
            ],
        )
        .unwrap();
        assert_eq!(segments.len(), 1);
        let merged = segments[0].cast::<IntPatchWLine>();
        assert_eq!(merged.nb_pnts().unwrap(), 100);
        assert_eq!(merged.nb_vertex(), 2);
        assert!(
            (merged
                .point(1)
                .unwrap()
                .value()
                .distance(&merged.point(100).unwrap().value())
                - 2.)
                .abs()
                < 1e-12
        );
    }
    fn separated_segments(side: Side<'_>) -> Vec<Handle<dyn IntPatchLine>> {
        let source = walking(side, false).cast::<IntPatchWLine>();
        let mut segments = Vec::new();
        for (first, last) in [(0., 0.4), (0.6, 1.)] {
            let polygon = Handle::new(IntSurfLineOn2S::new(None));
            let a = source.point(1).unwrap();
            let b = source.point(3).unwrap();
            for fraction in [first, (first + last) / 2., last] {
                let coords = a.value().xyz() * (1. - fraction) + b.value().xyz() * fraction;
                let pa = a.parameters();
                let pb = b.parameters();
                let mut point = IntSurfPntOn2S::default();
                point.set_value_all(
                    martensite_tkmath::gp::Point3::new(coords.x(), coords.y(), coords.z()),
                    pa.0 * (1. - fraction) + pb.0 * fraction,
                    pa.1 * (1. - fraction) + pb.1 * fraction,
                    pa.2 * (1. - fraction) + pb.2 * fraction,
                    pa.3 * (1. - fraction) + pb.3 * fraction,
                );
                polygon.add(&point);
            }
            let line = Handle::new(IntPatchWLine::new_undecided(&polygon, false));
            for index in [1, 3] {
                let point = polygon.value(index).unwrap();
                let mut vertex = IntPatchPoint::new();
                vertex.set_value_tangent(point.value(), 1e-7, false);
                let (u1, v1, u2, v2) = point.parameters();
                vertex.set_parameters(u1, v1, u2, v2);
                vertex.set_parameter(f64::from(index));
                line.add_vertex(&vertex, false);
            }
            segments.push(line.cast::<dyn IntPatchLine>());
        }
        segments
    }

    #[test]
    fn a_gap_off_the_other_surface_prevents_merging() {
        use martensite_tkmath::gp::{Direction3, Point3};
        let (surface, domain) = context();
        let side = Side {
            surface: &surface,
            domain: &domain,
            first: true,
            tolerance: 1e-7,
        };
        let plane = Plane3::new(
            Point3::new(0., 0., 1.),
            Direction3::new(0., 0., 1.).unwrap(),
        )
        .unwrap();
        let other_face = BRepBuilderAPIMakeFace::from_plane_bounds(plane, (-1., 1., -1., 1.))
            .unwrap()
            .face()
            .unwrap();
        let other_surface = Handle::new(BRepAdaptorSurface::with_face(&other_face, true).unwrap());
        let other_domain = BRepTopAdaptorTopolTool::with_surface(&other_surface.cast()).unwrap();
        let other = Side {
            surface: &other_surface,
            domain: &other_domain,
            first: false,
            tolerance: 1e-7,
        };
        let mut segments = separated_segments(side);
        let originals = segments.clone();
        merge_restriction_segments(&mut segments, [side, other]).unwrap();
        assert!(segments == originals);
    }
}
