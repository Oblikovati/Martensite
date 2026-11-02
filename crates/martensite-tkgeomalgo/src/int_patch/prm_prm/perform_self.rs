//! The self-intersection of a surface from the self-interference of its polyhedron:
//! the walkings start away from the diagonal (the points whose parameters on both
//! sides differ) and their ends on the diagonal are trimmed.
mod execution;
use super::IntPatchPrmPrmIntersection;
use super::helpers::section_point_to_parameters;
use super::perform_poly::IntPatchPrmPrmTolerances;
use super::start_params;
use super::walk_steps::{
    EndsCheck, WalkInputs, add_end_vertices, duplicated_line, keep_rejected, put_vertices,
    transitions,
};
use crate::int_patch::SelfSearchTermination;
use crate::int_patch::{
    IntPatchInterferencePolyhedron, IntPatchPoint, IntPatchPointLine, IntPatchPolyhedron,
    IntPatchWLType, IntPatchWLine,
};
use crate::int_start::TheTopolTool;
use crate::int_surf::{IntSurfLineOn2S, IntSurfPntOn2S, IntSurfTypeTrans};
use crate::int_walk::IntWalkPWalking;
use crate::intf::{SectionLine, SectionPoint};
use martensite_tkernel::message::ProgressRange;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::h_surface_tool::Adaptor3dSurfaceHandle;
use martensite_tkmath::gp::{Point2, Point3};

/// The walking of the self-intersection; `uv` keeps the parameters of the last section
/// point read, as the native locals do.
struct SelfWalk<'a> {
    inputs: WalkInputs<'a>,
    poly: &'a IntPatchPolyhedron,
    pw: IntWalkPWalking,
    uv: [f64; 4],
}

impl SelfWalk<'_> {
    fn parameters(&mut self, sp: &SectionPoint) -> Result<[f64; 4], Failure> {
        section_point_to_parameters(sp, self.poly, self.poly, &mut self.uv)?;
        Ok(self.uv)
    }
}

/// Whether the parameters `(pu, pv)` are in the parameter box of the segment
/// `[a, b]` on the first surface, enlarged by 1e-7.
fn in_segment_box((pu, pv): (f64, f64), a: (f64, f64), b: (f64, f64)) -> bool {
    let (mut au1, mut av1, mut bu1, mut bv1) = (a.0, a.1, b.0, b.1);
    if au1 > bu1 {
        std::mem::swap(&mut au1, &mut bu1);
    }
    if av1 > bv1 {
        std::mem::swap(&mut av1, &mut bv1);
    }
    au1 -= 1.0e-7;
    av1 -= 1.0e-7;
    bu1 += 1.0e-7;
    bv1 += 1.0e-7;
    pu >= au1 && pu <= bu1 && pv >= av1 && pv <= bv1
}

/// Whether the start point is far from the lines found: none of its parameter pairs
/// is in the parameter box of a segment of a line (the threshold test of the native
/// loops).
// The negated comparisons send the NaN operands where the native ones do.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn is_new_self_start(
    inputs: &WalkInputs<'_>,
    start: &IntSurfPntOn2S,
    lines: &[Handle<dyn crate::int_patch::IntPatchLine>],
) -> Result<bool, Failure> {
    let seuil: f64 = inputs.seuil();
    let mut dmini: f64 = seuil + seuil;
    let (pu1, pv1, pu2, pv2) = start.parameters();
    for line in lines {
        if !(dmini >= seuil) {
            break;
        }
        let testwline: Handle<IntPatchWLine> = line.cast::<IntPatchWLine>();
        if testwline.is_out_surf1_box(Point2::new(pu1, pv1))?
            || testwline.is_out_surf2_box(Point2::new(pu2, pv2))?
            || testwline.is_out_box(start.value())?
        {
            continue;
        }
        let nb: i32 = testwline.nb_pnts()?;
        let mut ll: i32 = 1;
        while ll < nb && dmini >= seuil {
            let a = testwline.point(ll)?.parameters_on_s1();
            let b = testwline.point(ll + 1)?.parameters_on_s1();
            if in_segment_box((pu1, pv1), a, b) || in_segment_box((pu2, pv2), a, b) {
                dmini = 0.0;
            }
            ll += 1;
        }
    }
    Ok(dmini > seuil)
}

/// Whether the parameters of the point on both sides are equal within 1e-6.
fn on_diagonal(p: &IntSurfPntOn2S) -> bool {
    let (u1, v1, u2, v2) = p.parameters();
    (u1 - u2).abs() <= 1e-6 && (v1 - v2).abs() <= 1e-6
}

/// The range of the points of the walking without its ends on the diagonal (one
/// diagonal point kept at each end).
fn trimmed_range(line: &IntSurfLineOn2S) -> Result<(i32, i32), Failure> {
    let nbpw: i32 = line.nb_points();
    let mut i: i32 = 0;
    let mut imin: i32;
    loop {
        i += 1;
        imin = i;
        if !(i < nbpw && on_diagonal(&line.value(i)?)) {
            break;
        }
    }
    if imin > 2 {
        imin -= 1;
    }
    i = nbpw + 1;
    let mut imax: i32;
    loop {
        i -= 1;
        imax = i;
        if !(i > 2 && on_diagonal(&line.value(i)?)) {
            break;
        }
    }
    if imax < nbpw {
        imax += 1;
    }
    Ok((imin, imax))
}

impl IntPatchPrmPrmIntersection {
    /// The marching from the start points of one section line: the point farthest from
    /// the diagonal, then the first (or middle) one, then the last one.
    fn walk_self_section(
        &mut self,
        walk: &mut SelfWalk<'_>,
        line: &SectionLine,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let nbp: i32 = line.number_of_points();
        self.self_search.starts.scheduled += if nbp > 5 { 3 } else { 1 };
        let mut nbps2: i32 = if nbp > 3 { nbp / 2 } else { 1 };
        let (mut n, mut first, mut second) = (0, 0, 0);
        loop {
            n += 1;
            if n == 1 {
                nbps2 = farthest_from_diagonal(walk, line, nbps2)?;
                if nbp < 3 {
                    n = 3;
                }
                first = nbps2;
            } else if n == 2 {
                nbps2 = if first == 1 { nbp / 2 } else { 1 };
                second = nbps2;
            } else {
                nbps2 = if first == nbp {
                    (first + second) / 2
                } else {
                    nbp
                };
            }
            let uv: [f64; 4] = walk.parameters(line.get_point(nbps2)?)?;
            self.observed_self_start(walk, uv, progress)?;
            if !(nbp > 5 && n < 3) {
                return Ok(());
            }
        }
    }

    /// The walking from the start parameters `uv`, when its first point is off the
    /// diagonal.
    fn walk_self_start(
        &mut self,
        walk: &mut SelfWalk<'_>,
        uv: [f64; 4],
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let (start, point) = self.refine_self_seed(walk, uv)?;
        let Some(point) = point else {
            return self.prune_self_start();
        };
        if !self.eligible_self_seed(walk, &point)? {
            return self.prune_self_start();
        }
        let previous_lines = self.slin.len() as i32;
        self.observed_self_march(walk, &start, progress)?;
        self.finish_self_march(walk, previous_lines)
    }
    fn refine_self_seed(
        &mut self,
        walk: &mut SelfWalk<'_>,
        uv: [f64; 4],
    ) -> Result<(Array1<'static, f64>, Option<IntSurfPntOn2S>), Failure> {
        let start = start_params(uv)?;
        self.self_search.refinements.entered();
        let seed = walk.pw.perform_first_point(&start);
        self.record_refinement(&seed, &walk.pw);
        Ok((start, seed?))
    }
    fn eligible_self_seed(
        &self,
        walk: &SelfWalk<'_>,
        point: &IntSurfPntOn2S,
    ) -> Result<bool, Failure> {
        let (u1, v1, u2, v2) = point.parameters();
        if !((u1 - u2).abs() > 1e-7 || (v1 - v2).abs() > 1e-7) {
            return Ok(false);
        }
        is_new_self_start(&walk.inputs, point, &self.slin)
    }
    fn prune_self_start(&mut self) -> Result<(), Failure> {
        self.self_search.starts.pruned += 1;
        Ok(())
    }
    fn finish_self_march(
        &mut self,
        walk: &SelfWalk<'_>,
        previous_lines: i32,
    ) -> Result<(), Failure> {
        let pw = &walk.pw;
        if !pw.is_done() || pw.nb_points()? <= 2 {
            return self.prune_self_start();
        }
        let ends = self_march_endpoints(pw)?;
        let duplicated = duplicated_line(
            &walk.inputs,
            pw,
            &self.slin,
            (previous_lines, false, EndsCheck::Either),
        )?;
        if keep_rejected(pw, duplicated, &mut self.slin)? {
            return self.prune_self_start();
        }
        self.accept_self_walking(&walk.inputs, pw, ends)
    }

    /// The line of an accepted self walking: the walking without its diagonal ends, a
    /// closing vertex when it is closed, the vertices of the restrictions and the end
    /// vertices; appended to the lines.
    fn accept_self_walking(
        &mut self,
        inputs: &WalkInputs<'_>,
        pw: &IntWalkPWalking,
        ends: (martensite_tkmath::gp::Point3, martensite_tkmath::gp::Point3),
    ) -> Result<(), Failure> {
        let (trans1, trans2): (IntSurfTypeTrans, IntSurfTypeTrans) =
            transitions(inputs, pw, false)?;
        let (imin, imax) = trimmed_range(pw.line()?)?;
        if imin >= imax {
            self.self_search.starts.pruned += 1;
            return Ok(());
        }
        let pw_line: Handle<IntSurfLineOn2S> = Handle::new(IntSurfLineOn2S::new(None));
        for i in imin..=imax {
            pw_line.add(&pw.line()?.value(i)?);
        }
        let tol_tang: f64 = inputs.tol_tangency;
        let wline: Handle<IntPatchWLine> =
            Handle::new(IntPatchWLine::new(&pw_line, false, trans1, trans2));
        wline.set_creating_way_info(IntPatchWLType::PrmPrm);
        close_self_line(&wline, inputs.tol_tangency, tol_tang)?;
        // The method PutVertexOnLine can reduce the number of points in <wline>.
        if !put_vertices(inputs, &wline, tol_tang)? {
            self.self_search.starts.pruned += 1;
            return Ok(());
        }
        add_end_vertices(pw, &wline, ends, tol_tang)?;
        self.slin.push(wline.cast());
        self.empt = false;
        Ok(())
    }
}

/// The point of the section line farthest from the diagonal (the largest difference of
/// the parameters on both sides), from the middle toward the first point, then toward
/// the last one.
fn farthest_from_diagonal(
    walk: &mut SelfWalk<'_>,
    line: &SectionLine,
    nbps2: i32,
) -> Result<i32, Failure> {
    let nbp: i32 = line.number_of_points();
    let mut best: i32 = nbps2;
    let mut incidence: f64 = 0.0;
    let ranks = (1..=nbp / 2).rev().chain(nbp / 2..=nbp);
    for nsp1 in ranks {
        let [u1, v1, u2, v2] = walk.parameters(line.get_point(nsp1)?)?;
        let current: f64 = (u1 - u2).abs() + (v1 - v2).abs();
        if current > incidence {
            best = nsp1;
            incidence = current;
        }
    }
    Ok(best)
}

/// A closed walking line (its end points within the tangency tolerance) gets its last
/// point replaced by its first point, with the parameters of both sides exchanged.
// The negated comparison sends the NaN operands where the native one does.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn close_self_line(wline: &IntPatchWLine, tol_tangency: f64, tol_tang: f64) -> Result<(), Failure> {
    let nb: i32 = wline.nb_pnts()?;
    let (deb, fin) = (wline.point(1)?, wline.point(nb)?);
    if !(deb.value().distance(&fin.value()) <= tol_tangency) {
        return Ok(());
    }
    let (u1t, v1t, u2t, v2t) = deb.parameters();
    let mut vtx = IntPatchPoint::new();
    vtx.set_value_tangent(deb.value(), tol_tang, false);
    vtx.set_parameters(u2t, v2t, u1t, v1t);
    vtx.set_parameter(f64::from(nb));
    wline.set_point(nb, &vtx)
}

fn self_march_endpoints(pw: &IntWalkPWalking) -> Result<(Point3, Point3), Failure> {
    Ok((pw.value(1)?.value(), pw.value(pw.nb_points()?)?.value()))
}
