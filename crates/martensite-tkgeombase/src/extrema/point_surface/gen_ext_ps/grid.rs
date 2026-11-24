// of the implementation at .

//! The sampling grid of GenExtPS: the parameters (from the knots of spline
//! geometry, uniform otherwise), the grid nodes, and the points of the grid edges
//! and cells nearest to the point.
use super::GenExtPS;
use crate::extrema::ElementType;
use crate::extrema::pon::POnSurfParams;
use crate::native_failure::array_failure;
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::precision;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::geom::GeomCurve;
use martensite_tkg3d::geom_grid_eval::GeomGridEvalSurface;
use martensite_tkmath::geom_abs::{CurveType, SurfaceType};
use martensite_tkmath::gp::{Point3, Xyz};

struct GridDistance<'a> {
    surface: &'a dyn Adaptor3dSurface,
    point: Point3,
    diff_tol: f64,
}

/// A 1-based array of parameters for GeomGridEval.
pub(super) fn param_array(values: &[f64]) -> Result<Array1<'static, f64>, Failure> {
    let mut array: Array1<'static, f64> = Array1::with_bounds(1, values.len() as i32)
        .map_err(|error| array_failure(error, "Extrema_GenExtPS parameters"))?;
    for (index, value) in values.iter().enumerate() {
        array[index as i32 + 1] = *value;
    }
    Ok(array)
}

/// `n` parameters from `min + (max - min) / n / 200` by the step
/// `(max - min - (max - min) / n / 100) / (n - 1)`, accumulated as natively.
pub(super) fn uniform_params(min: f64, max: f64, n: i32) -> Vec<f64> {
    let mut step: f64 = max - min;
    let mut start: f64 = step / f64::from(n) / 100.;
    step = (step - start) / f64::from(n - 1);
    start = start / 2. + min;
    let mut params: Vec<f64> = Vec::with_capacity(n.max(0) as usize);
    let mut value: f64 = start;
    for _ in 1..=n {
        params.push(value);
        value += step;
    }
    params
}

// `fillParams`: the range ends and `degree` points in each knot span within the
// range (at least PConfusion apart), or None when there are fewer than `sample`.
fn fill_params(
    knots: &[f64],
    degree: i32,
    (par_min, par_max): (f64, f64),
    sample: i32,
) -> Option<Vec<f64>> {
    let conf: f64 = precision::p_confusion();
    let knot = |i: usize| knots[i - 1];
    let mut params: Vec<f64> = vec![par_min];
    let mut prev_par: f64 = par_min;
    let mut i: usize = 1;
    while i < knots.len() && knot(i) < (par_max - conf) {
        if knot(i + 1) < par_min + conf {
            i += 1;
            continue;
        }
        let step: f64 = (knot(i + 1) - knot(i)) / f64::from(degree.max(2));
        for k in 1..=degree {
            let par: f64 = knot(i) + f64::from(k) * step;
            if par > par_max - conf {
                break;
            }
            if par > prev_par + conf {
                params.push(par);
                prev_par = par;
            }
        }
        i += 1;
    }
    params.push(par_max);
    if (params.len() as i32) < sample {
        return None;
    }
    Some(params)
}

// The knots and degree of the basis curve of a swept surface, when it is a spline.
fn basis_curve_knots(surface: &dyn Adaptor3dSurface) -> Result<Option<(Vec<f64>, i32)>, Failure> {
    let curve = surface.basis_curve()?;
    let mut knots: Option<(Vec<f64>, i32)> = None;
    if curve.get_type()? == CurveType::BSplineCurve {
        let bspline = curve.bspline()?;
        knots = Some((bspline.knots().as_slice().to_vec(), bspline.degree()));
    }
    if curve.get_type()? == CurveType::BezierCurve {
        let bezier = curve.bezier()?;
        knots = Some((
            vec![bezier.first_parameter(), bezier.last_parameter()],
            bezier.degree(),
        ));
    }
    Ok(knots)
}

impl GenExtPS<'_> {
    // `fillParams` into the U or the V parameters (kept when there are too few).
    fn fill_direction(&mut self, knots: &[f64], degree: i32, is_u: bool) {
        let filled = if is_u {
            fill_params(knots, degree, (self.umin, self.usup), self.usample)
        } else {
            fill_params(knots, degree, (self.vmin, self.vsup), self.vsample)
        };
        let Some(params) = filled else {
            return;
        };
        if is_u {
            self.u_params = Some(params);
        } else {
            self.v_params = Some(params);
        }
    }

    // `GetGridPoints(theSurf)`: the parameters of spline geometry from its knots and
    // degrees, then the sample counts of the parameters that exist.
    fn get_grid_points(&mut self, surface: &dyn Adaptor3dSurface) -> Result<(), Failure> {
        if !self.fill_spline_params(surface)? {
            return Ok(());
        }
        if let Some(params) = &self.u_params {
            self.usample = params.len() as i32;
        }
        if let Some(params) = &self.v_params {
            self.vsample = params.len() as i32;
        }
        Ok(())
    }

    // The type dispatch of GetGridPoints; false where the retained code returns early.
    fn fill_spline_params(&mut self, surface: &dyn Adaptor3dSurface) -> Result<bool, Failure> {
        match surface.get_type()? {
            SurfaceType::OffsetSurface => {
                let basis = surface.basis_surface()?;
                self.get_grid_points(&*basis)?;
            }
            SurfaceType::BSplineSurface => {
                let bspline = surface.bspline()?;
                self.fill_direction(bspline.u_knots().as_slice(), bspline.u_degree(), true);
                self.fill_direction(bspline.v_knots().as_slice(), bspline.v_degree(), false);
            }
            SurfaceType::BezierSurface => {
                let bezier = surface.bezier()?;
                let (u1, u2, v1, v2) = martensite_tkg3d::geom::GeomSurface::bounds(&*bezier);
                self.fill_direction(&[u1, u2], bezier.u_degree(), true);
                self.fill_direction(&[v1, v2], bezier.v_degree(), false);
            }
            kind @ (SurfaceType::SurfaceOfRevolution | SurfaceType::SurfaceOfExtrusion) => {
                let Some((knots, degree)) = basis_curve_knots(surface)? else {
                    return Ok(false);
                };
                self.fill_direction(&knots, degree, kind != SurfaceType::SurfaceOfRevolution);
            }
            _ => {}
        }
        Ok(true)
    }

    // The first part of BuildGrid: the parameters, the nodes and the array bounds,
    // once until the next Initialize or change of algorithm.
    fn init_grid(&mut self) -> Result<(), Failure> {
        let surface_ref = self.surface_ref()?;
        let surface: &dyn Adaptor3dSurface = surface_ref.get();
        self.get_grid_points(surface)?;
        if self.u_params.is_none() {
            self.u_params = Some(uniform_params(self.umin, self.usup, self.usample));
        }
        if self.v_params.is_none() {
            self.v_params = Some(uniform_params(self.vmin, self.vsup, self.vsample));
        }
        let (us, vs) = (self.usample, self.vsample);
        self.points.resize((0, us + 1), (0, vs + 1))?;
        let (u_params, v_params) = (
            self.u_params.clone().unwrap_or_default(),
            self.v_params.clone().unwrap_or_default(),
        );
        let grid_points: Array2<'static, Point3> = GeomGridEvalSurface::from_adaptor(surface)?
            .evaluate_grid(&param_array(&u_params)?, &param_array(&v_params)?)?;
        if grid_points.is_empty() {
            return Ok(());
        }
        for no_u in 1..=us {
            for no_v in 1..=vs {
                let node: Point3 = *grid_points
                    .value(no_u, no_v)
                    .map_err(|error| array_failure(error, "NCollection_Array2::Value"))?;
                let mut param = POnSurfParams::new(
                    u_params[(no_u - 1) as usize],
                    v_params[(no_v - 1) as usize],
                    node,
                );
                param.set_element_type(ElementType::Node);
                param.set_indices(no_u, no_v);
                self.points.set_value(no_u, no_v, param)?;
            }
        }
        self.face_pnt_params.resize((0, us), (0, vs))?;
        self.u_edge_pnt_params.resize((1, us - 1), (1, vs))?;
        self.v_edge_pnt_params.resize((1, us), (1, vs - 1))?;
        // The boundary has a negative square distance, for the search of maxima.
        for no_v in 0..=vs + 1 {
            self.points.change_value(0, no_v)?.set_sqr_distance(-1.);
            self.points
                .change_value(us + 1, no_v)?
                .set_sqr_distance(-1.);
        }
        for no_u in 1..=us {
            self.points.change_value(no_u, 0)?.set_sqr_distance(-1.);
            self.points
                .change_value(no_u, vs + 1)?
                .set_sqr_distance(-1.);
        }
        self.init = true;
        Ok(())
    }

    /// `BuildGrid(thePoint)`: the grid, then the distances to its nodes and, for the
    /// minima, the nearest points of its edges and cells.
    pub(super) fn build_grid(&mut self, point: Point3) -> Result<(), Failure> {
        if !self.init {
            self.init_grid()?;
        }
        let complete_layout = self.init && self.usample >= 2 && self.vsample >= 2;
        if complete_layout {
            self.row_node_distances(point)?;
        } else {
            self.scalar_node_distances(point)?;
        }
        if matches!(
            self.flag,
            crate::extrema::ExtFlag::Min | crate::extrema::ExtFlag::MinMax
        ) {
            self.minimum_grid(point, complete_layout)?;
        }
        Ok(())
    }

    fn minimum_grid(&mut self, point: Point3, complete_layout: bool) -> Result<(), Failure> {
        let surface = self.surface_ref()?;
        let distance = GridDistance {
            surface: surface.get(),
            point,
            diff_tol: self.tolu + self.tolv,
        };
        if complete_layout {
            self.row_edge_distances(&distance)?;
            self.row_face_distances(&distance)?;
        } else {
            // An empty evaluator grid or a refused reinitialization can leave old rows.
            self.scalar_edge_distances(&distance)?;
            self.scalar_face_distances(&distance)?;
        }
        self.reset_face_distances()
    }

    fn row_node_distances(&mut self, point: Point3) -> Result<(), Failure> {
        for no_u in 1..=self.usample {
            let nodes = self.points.row_mut(no_u)?;
            for param in &mut nodes[1..=self.vsample as usize] {
                let distance = point.square_distance(param.value());
                param.set_sqr_distance(distance);
            }
        }
        Ok(())
    }

    fn scalar_node_distances(&mut self, point: Point3) -> Result<(), Failure> {
        for no_u in 1..=self.usample {
            for no_v in 1..=self.vsample {
                let param = self.points.change_value(no_u, no_v)?;
                let distance = point.square_distance(param.value());
                param.set_sqr_distance(distance);
            }
        }
        Ok(())
    }

    fn row_edge_distances(&mut self, distance: &GridDistance<'_>) -> Result<(), Failure> {
        for no_u in 1..=self.usample {
            let nodes = self.points.row(no_u)?;
            let following = self.points.row(no_u + 1)?;
            let u_edges = if no_u < self.usample {
                self.u_edge_pnt_params.row_mut(no_u)?
            } else {
                &mut []
            };
            let v_edges = self.v_edge_pnt_params.row_mut(no_u)?;
            compute_edge_row(distance, nodes, following, u_edges, v_edges)?;
        }
        Ok(())
    }

    // Step 2: `UEdge(i, j) = {Point(i, j); Point(i + 1, j)}`,
    // `VEdge(i, j) = {Point(i, j); Point(i, j + 1)}`.
    fn scalar_edge_distances(&mut self, distance: &GridDistance<'_>) -> Result<(), Failure> {
        for no_u in 1..=self.usample {
            for no_v in 1..=self.vsample {
                self.scalar_edge_cell(distance, no_u, no_v)?;
            }
        }
        Ok(())
    }

    fn scalar_edge_cell(
        &mut self,
        distance: &GridDistance<'_>,
        no_u: i32,
        no_v: i32,
    ) -> Result<(), Failure> {
        let param0 = self.points.value(no_u, no_v)?;
        if no_u < self.usample {
            let param1 = self.points.value(no_u + 1, no_v)?;
            let edge = distance.compute_edge_parameters(true, (param0, param1))?;
            self.u_edge_pnt_params.set_value(no_u, no_v, edge)?;
        }
        if no_v < self.vsample {
            let param1 = self.points.value(no_u, no_v + 1)?;
            let edge = distance.compute_edge_parameters(false, (param0, param1))?;
            self.v_edge_pnt_params.set_value(no_u, no_v, edge)?;
        }
        Ok(())
    }

    fn row_face_distances(&mut self, distance: &GridDistance<'_>) -> Result<(), Failure> {
        for no_u in 1..self.usample {
            let u_edges = self.u_edge_pnt_params.row(no_u)?;
            let v_edges = self.v_edge_pnt_params.row(no_u)?;
            let next_v_edges = self.v_edge_pnt_params.row(no_u + 1)?;
            let faces = self.face_pnt_params.row_mut(no_u)?;
            let interior = &mut faces[1..self.vsample as usize];
            for (col, face) in interior.iter_mut().enumerate() {
                let edges = [
                    u_edges[col],
                    u_edges[col + 1],
                    v_edges[col],
                    next_v_edges[col],
                ];
                *face = distance.face_point([no_u, col as i32 + 1], edges)?;
            }
        }
        Ok(())
    }

    // Step 3: the nearest point of each cell `{UEdge(i, j); VEdge(i + 1, j);
    // UEdge(i, j + 1); VEdge(i, j)}`.
    fn scalar_face_distances(&mut self, distance: &GridDistance<'_>) -> Result<(), Failure> {
        for no_u in 1..self.usample {
            for no_v in 1..self.vsample {
                let edges = [
                    *self.u_edge_pnt_params.value(no_u, no_v)?,
                    *self.u_edge_pnt_params.value(no_u, no_v + 1)?,
                    *self.v_edge_pnt_params.value(no_u, no_v)?,
                    *self.v_edge_pnt_params.value(no_u + 1, no_v)?,
                ];
                let face = distance.face_point([no_u, no_v], edges)?;
                self.face_pnt_params.set_value(no_u, no_v, face)?;
            }
        }
        Ok(())
    }

    fn reset_face_distances(&mut self) -> Result<(), Failure> {
        let (us, vs) = (self.usample, self.vsample);
        for no_v in 0..=vs {
            self.face_pnt_params
                .change_value(0, no_v)?
                .set_sqr_distance(f64::MAX);
            self.face_pnt_params
                .change_value(us, no_v)?
                .set_sqr_distance(f64::MAX);
        }
        for no_u in 1..us {
            self.face_pnt_params
                .change_value(no_u, 0)?
                .set_sqr_distance(f64::MAX);
            self.face_pnt_params
                .change_value(no_u, vs)?
                .set_sqr_distance(f64::MAX);
        }
        Ok(())
    }
}

fn compute_edge_row(
    distance: &GridDistance<'_>,
    nodes: &[POnSurfParams],
    following: &[POnSurfParams],
    u_edges: &mut [POnSurfParams],
    v_edges: &mut [POnSurfParams],
) -> Result<(), Failure> {
    let mut u_edges = u_edges.iter_mut();
    let mut v_edges = v_edges.iter_mut();
    for (col, node) in nodes[1..nodes.len() - 1].iter().enumerate() {
        if let Some(edge) = u_edges.next() {
            *edge = distance.compute_edge_parameters(true, (node, &following[col + 1]))?;
        }
        if let Some(edge) = v_edges.next() {
            *edge = distance.compute_edge_parameters(false, (node, &nodes[col + 2]))?;
        }
    }
    Ok(())
}

impl GridDistance<'_> {
    // `ComputeEdgeParameters`: the point of the edge `param0`-`param1` nearest to
    // `point`: an end when the ends are confused or the nearest is an end,
    // otherwise the projection by the ratio of the chord.
    fn compute_edge_parameters(
        &self,
        is_u_edge: bool,
        (param0, param1): (&POnSurfParams, &POnSurfParams),
    ) -> Result<POnSurfParams, Failure> {
        let sqr_dist01: f64 = param0.value().square_distance(param1.value());
        if sqr_dist01 <= self.diff_tol {
            return Ok(*param0);
        }
        let diff_dist: f64 = (param0.get_sqr_distance() - param1.get_sqr_distance()).abs();
        if diff_dist >= sqr_dist01 - self.diff_tol {
            if param0.get_sqr_distance() > param1.get_sqr_distance() {
                return Ok(*param1);
            }
            return Ok(*param0);
        }
        let po_p: Xyz = self.point.xyz() - param0.value().xyz();
        let po_p1: Xyz = param1.value().xyz() - param0.value().xyz();
        let ratio: f64 = po_p.dot(po_p1) / sqr_dist01;
        let ((u0, v0), (u1, v1)) = (param0.parameter(), param1.parameter());
        let (mut u_par, mut v_par) = (u0, v0);
        if is_u_edge {
            u_par += ratio * (u1 - u0);
        } else {
            v_par += ratio * (v1 - v0);
        }
        let mut grid_param = POnSurfParams::new(u_par, v_par, self.surface.value(u_par, v_par)?);
        let (index_u, index_v) = param0.get_indices();
        grid_param.set_element_type(if is_u_edge {
            ElementType::UIsoEdge
        } else {
            ElementType::VIsoEdge
        });
        grid_param.set_sqr_distance(self.point.square_distance(grid_param.value()));
        grid_param.set_indices(index_u, index_v);
        Ok(grid_param)
    }

    // The nearest point of the cell `(no_u, no_v)`: of its nearest edge when the
    // projection is outside the cell, otherwise at the middle parameters.
    fn face_point(
        &self,
        [no_u, no_v]: [i32; 2],
        [ue0, ue1, ve0, ve1]: [POnSurfParams; 4],
    ) -> Result<POnSurfParams, Failure> {
        let outside = |e0: &POnSurfParams, e1: &POnSurfParams| {
            let sqr_dist01: f64 = e0.value().square_distance(e1.value());
            (e0.get_sqr_distance() - e1.get_sqr_distance()).abs() >= sqr_dist01 - self.diff_tol
        };
        if outside(&ue0, &ue1) || outside(&ve0, &ve1) {
            let nearer = |a: POnSurfParams, b: POnSurfParams| {
                if a.get_sqr_distance() < b.get_sqr_distance() {
                    a
                } else {
                    b
                }
            };
            return Ok(nearer(nearer(ue0, ue1), nearer(ve0, ve1)));
        }
        let u_par: f64 = 0.5 * (ue0.parameter().0 + ue1.parameter().0);
        let v_par: f64 = 0.5 * (ve0.parameter().1 + ve1.parameter().1);
        let mut param = POnSurfParams::new(u_par, v_par, self.surface.value(u_par, v_par)?);
        param.set_element_type(ElementType::Face);
        param.set_sqr_distance(self.point.square_distance(param.value()));
        param.set_indices(no_u, no_v);
        Ok(param)
    }
}

#[cfg(test)]
#[path = "row_traversal_tests.rs"]
mod row_traversal_tests;
