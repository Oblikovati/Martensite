//

//! The isos and triangulations of the faces of a drawable shape.
use super::TopologyRendering;
use super::plot::{Plot, draw_and_check};
use crate::edge_face::{FaceDrawRecord, FaceIso};
use martensite_raster_core::RasterDisplay;
use martensite_tkbrep::brep::BRepTool;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkbrep::topods::Face;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use martensite_tkg2d::geom2d::native::{std_max, std_min};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkmath::geom_abs::{IsoType, Shape as Continuity, SurfaceType};

#[path = "face_mesh.rs"]
mod mesh;
#[path = "face_surface.rs"]
mod sampling;
use mesh::draw_face_triangulation;
use sampling::{IntervalSteps, restriction, surface_steps};
type FaceDrawing<'a, 'd> = (&'a FaceDrawRecord, &'a mut RasterDisplay<'d>, &'a mut bool);
/// The walk along one iso: the current point parameters, the end parameters and the
/// steps (`U1`, `V1`, `U2`, `V2`, `stepU`, `stepV`).
#[derive(Debug, Clone, Copy, Default)]
struct IsoWalk {
    u1: f64,
    v1: f64,
    u2: f64,
    v2: f64,
    step_u: f64,
    step_v: f64,
}

/// The surface of a face as the iso drawing reads it.
struct IsoSurface {
    adaptor: BRepAdaptorSurface,
    kind: SurfaceType,
    /// The continuity interval bounds along U and along V.
    u_bounds: Array1<'static, f64>,
    v_bounds: Array1<'static, f64>,
}

impl IsoSurface {
    fn new(face: &Face) -> Result<Self, Failure> {
        let adaptor = BRepAdaptorSurface::with_face(face, restriction(face)?)?;
        Ok(Self {
            kind: adaptor.get_type()?,
            u_bounds: bounds(adaptor.nb_u_intervals(Continuity::CN)?, |b| {
                adaptor.u_intervals(b, Continuity::CN)
            })?,
            v_bounds: bounds(adaptor.nb_v_intervals(Continuity::CN)?, |b| {
                adaptor.v_intervals(b, Continuity::CN)
            })?,
            adaptor,
        })
    }
}

impl TopologyRendering {
    /// The face part of `DrawOn`: each face's isos in its color, and its
    /// triangulation when it has no surface or triangulations are shown.
    pub(super) fn draw_faces(
        &self,
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
    ) -> Result<(), Failure> {
        let faces = self.faces.borrow().clone();
        for face in &faces {
            if *halt {
                break;
            }
            self.draw_face_record(face, display, halt)?;
        }
        Ok(())
    }
    fn draw_face_record(
        &self,
        face: &FaceDrawRecord,
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
    ) -> Result<(), Failure> {
        display.set_color(face.color());
        let has_surface = BRepTool::surface_with_location(face.face())?.0.is_some();
        if has_surface {
            self.draw_face_isos(face, display, halt)?;
        }
        if !has_surface || self.triangulations.get() {
            draw_face_triangulation(face.face(), display)?;
        }
        Ok(())
    }

    /// The isos of one face on its (possibly restricted) adaptor.
    fn draw_face_isos(
        &self,
        face: &FaceDrawRecord,
        display: &mut RasterDisplay<'_>,
        halt: &mut bool,
    ) -> Result<(), Failure> {
        let surface = IsoSurface::new(face.face())?;
        for index in 1..=face.nb_isos() {
            let iso = face.iso(index)?;
            self.draw_iso(&surface, iso, (face, display, halt))?;
        }
        Ok(())
    }

    /// One iso: a move to its start, each continuity interval it crosses, then a line
    /// to its end.
    fn draw_iso(
        &self,
        surface: &IsoSurface,
        iso: FaceIso,
        (face, display, halt): FaceDrawing<'_, '_>,
    ) -> Result<(), Failure> {
        let along_v = iso.kind == IsoType::IsoU;
        let ti = iso_interval_bounds(surface, along_v);
        let mut walk = initial_iso_walk(ti, iso, along_v)?;
        display.move_to_3d(surface.adaptor.d0(walk.u1, walk.v1)?);
        self.walk_iso_intervals(
            (surface, iso),
            (ti, along_v),
            &mut walk,
            (face, &mut *display, &mut *halt),
        )?;
        self.finish_iso(surface, walk, (face, display, halt))
    }

    fn walk_iso_intervals(
        &self,
        (surface, iso): (&IsoSurface, FaceIso),
        (ti, along_v): (&Array1<'_, f64>, bool),
        walk: &mut IsoWalk,
        (face, display, halt): FaceDrawing<'_, '_>,
    ) -> Result<(), Failure> {
        for interval in 1..=ti.len() as i32 - 1 {
            let Some(range) = clipped_iso_range(ti, interval, iso)? else {
                continue;
            };
            self.walk_iso_piece(
                (surface, iso.kind),
                (range, along_v),
                walk,
                (face, &mut *display, &mut *halt),
            )?;
        }
        Ok(())
    }

    fn walk_iso_piece(
        &self,
        (surface, kind): (&IsoSurface, IsoType),
        ((from, to), along_v): ((f64, f64), bool),
        walk: &mut IsoWalk,
        drawing: FaceDrawing<'_, '_>,
    ) -> Result<(), Failure> {
        let discret = f64::from(self.discret.get());
        if along_v {
            (walk.v1, walk.v2, walk.step_v) = (from, to, (to - from) / discret);
        } else {
            (walk.u1, walk.u2, walk.step_u) = (from, to, (to - from) / discret);
        }
        self.walk_interval(surface, kind, walk, drawing)
    }

    fn finish_iso(
        &self,
        surface: &IsoSurface,
        walk: IsoWalk,
        (face, display, halt): FaceDrawing<'_, '_>,
    ) -> Result<(), Failure> {
        let end = surface.adaptor.d0(walk.u2, walk.v2)?;
        let face_shape = face.face().as_ref();
        draw_and_check(
            display,
            end,
            (face_shape, walk.u2, walk.v2),
            halt,
            self.pick_sink.as_ref(),
        );
        Ok(())
    }

    /// The points of one interval, by surface type: none on a plane, `discret` steps on
    /// quadrics along circles, adaptive double steps on splines and swept curves.
    fn walk_interval(
        &self,
        surface: &IsoSurface,
        kind: IsoType,
        walk: &mut IsoWalk,
        drawing: FaceDrawing<'_, '_>,
    ) -> Result<(), Failure> {
        match surface_steps(surface, kind)? {
            IntervalSteps::None => Ok(()),
            IntervalSteps::Uniform => self.uniform_steps(surface, walk, drawing),
            IntervalSteps::Adaptive => self.adaptive_steps(surface, kind, walk, drawing),
        }
    }

    /// `discret - 1` points at equal steps.
    fn uniform_steps(
        &self,
        surface: &IsoSurface,
        walk: &mut IsoWalk,
        (face, display, halt): FaceDrawing<'_, '_>,
    ) -> Result<(), Failure> {
        for _ in 1..self.discret.get() {
            walk.u1 += walk.step_u;
            walk.v1 += walk.step_v;
            let point = surface.adaptor.d0(walk.u1, walk.v1)?;
            draw_and_check(
                display,
                point,
                (face.face().as_ref(), walk.u1, walk.v1),
                halt,
                self.pick_sink.as_ref(),
            );
        }
        Ok(())
    }

    /// `discret / 2` double steps, each plotted with `PlotIso`.
    fn adaptive_steps(
        &self,
        surface: &IsoSurface,
        kind: IsoType,
        walk: &mut IsoWalk,
        (face, display, halt): FaceDrawing<'_, '_>,
    ) -> Result<(), Failure> {
        let step = adaptive_iso_step(kind, walk);
        for _ in 1..=self.discret.get() / 2 {
            Plot::new(display, face.face().as_ref(), halt, self.pick_sink.as_ref()).iso(
                &surface.adaptor,
                kind,
                (walk.u1, walk.v1),
                step,
            )?;
            walk.u1 += walk.step_u * 2.;
            walk.v1 += walk.step_v * 2.;
        }
        Ok(())
    }
}

fn adaptive_iso_step(kind: IsoType, walk: &IsoWalk) -> f64 {
    if kind == IsoType::IsoV {
        return walk.step_u * 2.;
    }
    walk.step_v * 2.
}
fn iso_interval_bounds(surface: &IsoSurface, along_v: bool) -> &Array1<'static, f64> {
    if along_v {
        return &surface.v_bounds;
    }
    &surface.u_bounds
}
fn initial_iso_walk(ti: &Array1<'_, f64>, iso: FaceIso, along_v: bool) -> Result<IsoWalk, Failure> {
    let (start, end) = (
        std_max(iso.first, at(ti, 1)?),
        std_min(iso.last, at(ti, 2)?),
    );
    if along_v {
        return Ok(iso_v_walk(iso.parameter, (start, end)));
    }
    Ok(iso_u_walk(iso.parameter, (start, end)))
}
fn iso_v_walk(parameter: f64, (start, end): (f64, f64)) -> IsoWalk {
    IsoWalk {
        u1: parameter,
        v1: start,
        u2: parameter,
        v2: end,
        ..IsoWalk::default()
    }
}
fn iso_u_walk(parameter: f64, (start, end): (f64, f64)) -> IsoWalk {
    IsoWalk {
        u1: start,
        v1: parameter,
        u2: end,
        v2: parameter,
        ..IsoWalk::default()
    }
}
fn clipped_iso_range(
    ti: &Array1<'_, f64>,
    interval: i32,
    iso: FaceIso,
) -> Result<Option<(f64, f64)>, Failure> {
    let (t1, t2) = (at(ti, interval)?, at(ti, interval + 1)?);
    if (t1 <= iso.first && t2 <= iso.first) || (t1 >= iso.last && t2 >= iso.last) {
        return Ok(None);
    }
    Ok(Some((std_max(iso.first, t1), std_min(iso.last, t2))))
}

/// `NCollectionArray1<double> TI(1, nb + 1)` filled by `fill`.
fn bounds(
    nb: i32,
    fill: impl FnOnce(&mut Array1<'static, f64>) -> Result<(), Failure>,
) -> Result<Array1<'static, f64>, Failure> {
    martensite_raster_core::sampling::interval_bounds(nb, fill)
}

/// `TI(index)`.
fn at(bounds: &Array1<'_, f64>, index: i32) -> Result<f64, Failure> {
    martensite_raster_core::sampling::bound(bounds, index)
}
