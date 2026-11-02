//! Dispatch observations at the calls that actually discover and visit candidates.
use super::*;
use martensite_tkernel::ncollection::Array1;

impl IntPatchPrmPrmIntersection {
    /// Search a surface with the existing tolerances; `inter.perform_self(&s,&domain,tol)?`.
    pub fn perform_self(
        &mut self,
        surface: &Adaptor3dSurfaceHandle,
        domain: &(dyn TheTopolTool + 'static),
        tolerances: IntPatchPrmPrmTolerances,
    ) -> Result<(), Failure> {
        self.perform_self_with_progress(surface, domain, tolerances, &ProgressRange::default())
    }
    /// Observe a cancellable self-search; `inter.perform_self_with_progress(&s,&domain,tol,&range)?`.
    pub fn perform_self_with_progress(
        &mut self,
        surface: &Adaptor3dSurfaceHandle,
        domain: &(dyn TheTopolTool + 'static),
        tolerances: IntPatchPrmPrmTolerances,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.self_search.begin();
        let result = self.search_surface(surface, domain, tolerances, progress);
        self.self_search.finish(&result);
        if self.self_search.termination == SelfSearchTermination::Cancelled {
            self.done = false;
        }
        result
    }
    fn search_surface(
        &mut self,
        surface: &Adaptor3dSurfaceHandle,
        domain: &(dyn TheTopolTool + 'static),
        tolerances: IntPatchPrmPrmTolerances,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.self_search.check_cancelled(progress)?;
        let poly = IntPatchPolyhedron::with_subdivisions(
            surface,
            domain.nb_samples_u()?,
            domain.nb_samples_v()?,
        )?;
        self.search_self_polyhedron((surface, &poly, domain), tolerances, progress)
    }
    /// Search a supplied sampled polyhedron; `inter.perform_self_polyhedron((&s,&poly,&domain),tol)?`.
    pub fn perform_self_polyhedron(
        &mut self,
        inputs: (
            &Adaptor3dSurfaceHandle,
            &IntPatchPolyhedron,
            &(dyn TheTopolTool + 'static),
        ),
        tolerances: IntPatchPrmPrmTolerances,
    ) -> Result<(), Failure> {
        self.self_search.begin();
        let result = self.search_self_polyhedron(inputs, tolerances, &ProgressRange::default());
        self.self_search.finish(&result);
        result
    }
    fn search_self_polyhedron(
        &mut self,
        (surface, poly, domain): (
            &Adaptor3dSurfaceHandle,
            &IntPatchPolyhedron,
            &(dyn TheTopolTool + 'static),
        ),
        tolerances: IntPatchPrmPrmTolerances,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.self_search.check_cancelled(progress)?;
        let interference = self.discover_self_candidates(poly)?;
        self.empt = true;
        self.done = true;
        self.slin.clear();
        let mut walk = self_walk(surface, poly, domain, tolerances)?;
        self.search_sections(&mut walk, &interference, progress)?;
        self.search_zones(&mut walk, &interference, progress)
    }
    fn discover_self_candidates(
        &mut self,
        poly: &IntPatchPolyhedron,
    ) -> Result<IntPatchInterferencePolyhedron, Failure> {
        self.self_search.discovery_started = true;
        let interference = IntPatchInterferencePolyhedron::of_self(poly)?;
        self.self_search.discovered(
            interference.nb_section_lines(),
            interference.nb_tangent_zones(),
        );
        Ok(interference)
    }
    fn search_sections(
        &mut self,
        walk: &mut SelfWalk<'_>,
        interference: &IntPatchInterferencePolyhedron,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        for index in 1..=interference.nb_section_lines() {
            self.self_search.check_cancelled(progress)?;
            self.self_search.sections.visited += 1;
            let result = interference
                .line_value(index)
                .and_then(|line| self.walk_self_section(walk, line, progress));
            let cancelled = self.self_search.termination == SelfSearchTermination::Cancelled;
            self.self_search.sections.returned(&result, cancelled);
            result?;
        }
        Ok(())
    }
    fn search_zones(
        &mut self,
        walk: &mut SelfWalk<'_>,
        interference: &IntPatchInterferencePolyhedron,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        for index in 1..=interference.nb_tangent_zones() {
            self.self_search.check_cancelled(progress)?;
            self.self_search.tangent_zones.visited += 1;
            let result = self.search_zone(walk, interference, index, progress);
            let cancelled = self.self_search.termination == SelfSearchTermination::Cancelled;
            self.self_search.tangent_zones.returned(&result, cancelled);
            result?;
        }
        Ok(())
    }
    fn search_zone(
        &mut self,
        walk: &mut SelfWalk<'_>,
        interference: &IntPatchInterferencePolyhedron,
        index: i32,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        let zone = interference.zone_value(index)?;
        self.self_search.starts.scheduled += zone.number_of_points() as u64;
        for point in 1..=zone.number_of_points() {
            let uv = walk.parameters(zone.get_point(point)?)?;
            self.observed_self_start(walk, uv, progress)?;
        }
        Ok(())
    }
    pub(super) fn observed_self_start(
        &mut self,
        walk: &mut SelfWalk<'_>,
        uv: [f64; 4],
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.self_search.check_cancelled(progress)?;
        self.self_search.starts.visited += 1;
        let pruned = self.self_search.starts.pruned;
        let result = self.walk_self_start(walk, uv, progress);
        if self.self_search.starts.pruned == pruned {
            self.self_search.starts.returned(
                &result,
                self.self_search.termination == SelfSearchTermination::Cancelled,
            );
        }
        result
    }
    pub(super) fn record_refinement(
        &mut self,
        result: &Result<Option<IntSurfPntOn2S>, Failure>,
        walker: &IntWalkPWalking,
    ) {
        if result.is_err() {
            self.self_search.refinements.failed += 1;
        } else {
            self.self_search.refinements.finished += 1;
        }
        self.self_search
            .numerical
            .accumulate(walker.numerical_search_counts());
        self.self_search.observe_walking(walker.search_observation);
    }
    pub(super) fn observed_self_march(
        &mut self,
        walk: &mut SelfWalk<'_>,
        start: &Array1<'_, f64>,
        progress: &ProgressRange<'_>,
    ) -> Result<(), Failure> {
        self.self_search.walks.entered();
        let result = walk.pw.perform_with_progress(start, progress);
        self.self_search
            .numerical
            .accumulate(walk.pw.numerical_search_counts());
        self.self_search.observe_walking(walk.pw.search_observation);
        self.self_search
            .walks
            .returned(&result, walk.pw.search_observation.cancelled);
        result
    }
}
fn self_walk<'a>(
    surface: &'a Adaptor3dSurfaceHandle,
    poly: &'a IntPatchPolyhedron,
    domain: &'a (dyn TheTopolTool + 'static),
    t: IntPatchPrmPrmTolerances,
) -> Result<SelfWalk<'a>, Failure> {
    let pw = self_marching_algorithm(surface, t)?;
    let inputs = self_walking_inputs(surface, domain, t);
    Ok(SelfWalk {
        inputs,
        poly,
        pw,
        uv: [0.; 4],
    })
}
fn self_marching_algorithm(
    surface: &Adaptor3dSurfaceHandle,
    t: IntPatchPrmPrmTolerances,
) -> Result<IntWalkPWalking, Failure> {
    IntWalkPWalking::new(
        surface,
        surface,
        t.tol_tangency,
        t.epsilon,
        t.deflection,
        t.increment,
    )
}
fn self_walking_inputs<'a>(
    surface: &'a Adaptor3dSurfaceHandle,
    domain: &'a (dyn TheTopolTool + 'static),
    t: IntPatchPrmPrmTolerances,
) -> WalkInputs<'a> {
    WalkInputs {
        surf1: surface,
        d1: domain,
        surf2: surface,
        d2: None,
        tol_tangency: t.tol_tangency,
        epsilon: t.epsilon,
        deflection: t.deflection,
        increment: t.increment,
    }
}
