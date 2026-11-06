//! The blend and Boolean commands of the grids, with the command-host command's own defaults.
use crate::script::{OffsetScript, command_stopped};
use crate::steps::{Bool, Fillet};
use crate::support::common_operation::report_result_alerts;
use martensite_tkbo::{
    bop_algo::{BOPAlgoBOP, BOPAlgoPaveFiller, BuilderShape, Glue, Operation},
    brep_algo_api::{BRepAlgoAPICut, BRepAlgoAPIFuse, BRepAlgoAPISection},
};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::History,
    topexp::Explorer,
    topods::{self, Compound, Shape},
    toptools::MapOfShape,
};
use martensite_tkernel::{
    message::ProgressRange, precision, standard::Failure, storage::format_general,
};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{geom_abs::Shape as Continuity, gp::Point2};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

/// `tesp` of the command file: `tolblend` does not change it.
const TESP: f64 = 1.0e-4;
/// `tapp_angle` of the command file.
const TAPP_ANGLE: f64 = 1.0e-2;

/// The state the command-host fillet commands keep in file statics.
pub struct FilletCommands {
    /// The pave filler `bop` leaves for `bopfuse`.
    two_step: Option<BOPAlgoPaveFiller>,
    /// `ta`, `t3d`, `t2d`, `fl`, as `tolblend` sets them.
    tolerances: [f64; 4],
    /// `Rake` and the `name` of `mkevol`.
    evolved: Option<(BRepFilletAPIMakeFillet, &'static str)>,
}

impl FilletCommands {
    /// `t3d` of `tolblend`, which `brollingball` reads too; `commands.tolerance_3d()`.
    pub fn tolerance_3d(&self) -> f64 {
        self.tolerances[1]
    }
}

impl Default for FilletCommands {
    fn default() -> Self {
        Self {
            tolerances: [1.0e-2, 1.0e-4, 1.0e-5, 1.0e-3],
            evolved: None,
            two_step: None,
        }
    }
}

impl FilletCommands {
    /// `printtolblend`: what the fillet commands write first.
    fn tolerance_text(&self) -> String {
        let [ta, t3d, t2d, fl] = self.tolerances.map(|value| format_general(value, 17));
        format!(
            "tolerance ang : {ta}\ntolerance 3d  : {t3d}\ntolerance 2d  : {t2d}\nfleche        : {fl}\ntolblend {ta} {t3d} {t2d} {fl}"
        )
    }

    /// A fillet command returning 1: its result, the tolerances, ends the script.
    fn stopped(&self) -> Failure {
        command_stopped(&self.tolerance_text())
    }

    fn maker(&self, shape: &Shape) -> Result<BRepFilletAPIMakeFillet, Failure> {
        let [ta, t3d, t2d, fl] = self.tolerances;
        let mut maker = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational)?;
        maker.set_params(ta, TESP, t2d, t3d, t2d, fl);
        maker.set_continuity(Continuity::C1, TAPP_ANGLE);
        Ok(maker)
    }

    /// Run a fillet command; `commands.fillet(script, &Fillet::BuildEvolved)`.
    pub fn fillet(&mut self, script: &mut OffsetScript<'_>, step: &Fillet) -> Result<(), Failure> {
        match step {
            Fillet::Tolerances(values) => self.tolerances = *values,
            Fillet::Blend(result, shape, edges) => self.blend(script, result, shape, edges)?,
            Fillet::Plain(result, shape, edges) => plain_fillet(script, result, shape, edges)?,
            Fillet::BlendGeneratedEdges(result, shape, source, radius) => {
                self.blend_generated(script, (result, shape, source), *radius)?;
            }
            Fillet::MakeEvolved(result, shape) => {
                println!("{}", self.tolerance_text());
                self.evolved = Some((self.maker(&script.required(shape)?)?, result));
            }
            Fillet::UpdateEvolved(edge, pairs) => self.update_evolved(script, edge, pairs)?,
            Fillet::BuildEvolved => self.build_evolved(script)?,
        }
        Ok(())
    }

    fn blend(
        &self,
        script: &mut OffsetScript<'_>,
        result: &'static str,
        shape: &str,
        edges: &[(f64, &str)],
    ) -> Result<(), Failure> {
        let mut maker = self.maker(&script.required(shape)?)?;
        let added = add_defined_edges(script, &mut maker, edges)?;
        let outcome = if added == 0 {
            Err(self.stopped())
        } else {
            self.finish(script, maker, result)
        };
        println!("{}", self.tolerance_text());
        outcome
    }

    /// `build`, and the result under `name` unless it is not done.
    fn finish(
        &self,
        script: &mut OffsetScript<'_>,
        mut maker: BRepFilletAPIMakeFillet,
        name: &str,
    ) -> Result<(), Failure> {
        maker.build(&ProgressRange::default())?;
        if !maker.is_done() {
            return Err(self.stopped());
        }
        script.set_shape(name, maker.shape()?.clone());
        Ok(())
    }

    /// `boolean/history/A7`: the edges each face of its source generated, every one a contour.
    fn blend_generated(
        &self,
        script: &mut OffsetScript<'_>,
        (result, shape, source): (&'static str, &str, &str),
        radius: f64,
    ) -> Result<(), Failure> {
        let edges = generated_edges(script, source)?;
        let outcome = self.blend_edges(script, (result, shape), &edges, radius);
        println!("{}", self.tolerance_text());
        outcome
    }

    fn blend_edges(
        &self,
        script: &mut OffsetScript<'_>,
        (result, shape): (&'static str, &str),
        edges: &[Shape],
        radius: f64,
    ) -> Result<(), Failure> {
        let mut maker = self.maker(&script.required(shape)?)?;
        for edge in edges {
            maker.add_with_radius(radius, &topods::edge(edge)?)?;
        }
        self.finish(script, maker, result)
    }

    fn update_evolved(
        &mut self,
        script: &OffsetScript<'_>,
        edge: &str,
        pairs: &[[f64; 2]],
    ) -> Result<(), Failure> {
        let edge = topods::edge(&script.required(edge)?)?;
        let radii: Vec<Point2> = pairs.iter().map(|p| Point2::new(p[0], p[1])).collect();
        let (maker, _) = self
            .evolved
            .as_mut()
            .ok_or_else(|| command_stopped("MakeFillet not initialized"))?;
        maker.add_with_pairs(&radii, &edge)
    }

    fn build_evolved(&mut self, script: &mut OffsetScript<'_>) -> Result<(), Failure> {
        let (maker, name) = self
            .evolved
            .take()
            .ok_or_else(|| command_stopped("MakeFillet not initialized"))?;
        self.finish(script, maker, name)
    }

    /// Run a blending boolean; `commands.boolean(script, &step)`.
    pub fn boolean(&mut self, script: &mut OffsetScript<'_>, step: &Bool) -> Result<(), Failure> {
        match step {
            Bool::CutBlend(result, a, b, radius) => self.cut_blend(script, result, (a, b), *radius),
            Bool::FuseBlend(result, a, b, radius) => {
                self.fuse_blend(script, result, (a, b), *radius)
            }
            _ => unreachable!("boolean_steps runs the others"),
        }
    }

    /// `bfuseblend`: blends every section edge on every solid of the fuse. A solid whose
    /// blend is not done keeps its fused form, and the command says so as an error.
    fn fuse_blend(
        &self,
        script: &mut OffsetScript<'_>,
        result: &'static str,
        (a, b): (&str, &str),
        radius: f64,
    ) -> Result<(), Failure> {
        println!("{}", self.tolerance_text());
        let (s1, s2) = (script.required(a)?, script.required(b)?);
        let filler = performed_filler(&s1, &s2)?;
        let mut fuse = BRepAlgoAPIFuse::from_shapes_with_filler(&s1, &s2, filler)?;
        if !fuse.is_done() {
            return Err(command_stopped("boolean operation not done"));
        }
        let fused = fuse.shape().clone();
        let section = BRepAlgoAPISection::from_shapes(&s1, &s2, fuse.take_ds_filler(), true)?;
        let compound = self.blended_solids(&fused, section.shape(), radius)?;
        script.set_shape(result, compound);
        Ok(())
    }

    /// `bcutblend`: as `bfuseblend`, with the cut in place of the fuse.
    fn cut_blend(
        &self,
        script: &mut OffsetScript<'_>,
        result: &'static str,
        (a, b): (&str, &str),
        radius: f64,
    ) -> Result<(), Failure> {
        println!("{}", self.tolerance_text());
        let (s1, s2) = (script.required(a)?, script.required(b)?);
        let filler = performed_filler(&s1, &s2)?;
        let mut cut = BRepAlgoAPICut::from_shapes_with_filler(&s1, &s2, filler, true)?;
        if !cut.is_done() {
            return Err(command_stopped("boolean operation not done"));
        }
        let kept = cut.shape().clone();
        let section = BRepAlgoAPISection::from_shapes(&s1, &s2, cut.take_ds_filler(), true)?;
        let compound = self.blended_solids(&kept, section.shape(), radius)?;
        script.set_shape(result, compound);
        Ok(())
    }

    /// `bop a b`.
    fn prepare_two_step(
        &mut self,
        script: &OffsetScript<'_>,
        a: &str,
        b: &str,
    ) -> Result<(), Failure> {
        let (s1, s2) = (script.required(a)?, script.required(b)?);
        let mut filler = BOPAlgoPaveFiller::new();
        filler.set_arguments(&[s1, s2].into_iter().collect());
        filler.options_mut().set_fuzzy_value(precision::confusion());
        filler.options_mut().set_run_parallel(false);
        filler.set_non_destructive(false);
        filler.set_glue(Glue::Off);
        filler.options_mut().set_use_obb(false);
        filler.perform();
        report_result_alerts(filler.options());
        self.two_step = Some(filler);
        Ok(())
    }

    /// `bopfuse result`, on the filler `bop` left.
    fn fuse_two_step(
        &mut self,
        script: &mut OffsetScript<'_>,
        result: &'static str,
    ) -> Result<(), Failure> {
        let mut filler = self
            .two_step
            .take()
            .ok_or_else(|| command_stopped("prepare PaveFiller first"))?;
        let arguments: Vec<Shape> = filler.arguments().iter().cloned().collect();
        let mut bop = configured_bop(&arguments[0], &arguments[1], Operation::Fuse);
        bop.perform_with_filler(&mut filler);
        report_result_alerts(bop.options());
        if !bop.options().has_errors() {
            script.set_shape(result, bop.shape().clone());
        }
        Ok(())
    }

    fn blended_solids(
        &self,
        fused: &Shape,
        section: &Shape,
        radius: f64,
    ) -> Result<Shape, Failure> {
        let builder = BRepBuilder::default();
        let mut result = Compound::default();
        builder.make_compound(&mut result);
        let mut solids = Explorer::new(fused, ShapeEnum::Solid, ShapeEnum::Shape)?;
        while solids.more() {
            let blended = self.blended_solid(solids.current(), section, radius)?;
            builder.add(&mut result, &blended)?;
            solids.next()?;
        }
        Ok(result.into_shape())
    }

    /// One solid with the section's edges blended, or as it was when that is not done.
    fn blended_solid(&self, solid: &Shape, section: &Shape, radius: f64) -> Result<Shape, Failure> {
        let mut blender = self.maker(solid)?;
        let mut edges = Explorer::new(section, ShapeEnum::Edge, ShapeEnum::Shape)?;
        while edges.more() {
            blender.add_with_radius(radius, &topods::edge(edges.current())?)?;
            edges.next()?;
        }
        blender.build(&ProgressRange::default())?;
        if blender.is_done() {
            return Ok(blender.shape()?.clone());
        }
        println!(
            "Error: Cannot find the result of BLEND-operation. The result of BOP operation will be returned."
        );
        Ok(solid.clone())
    }
}

/// `fillet`: with the defaults of the command's own file, whatever `tolblend` set. A radius of 0
/// skips its name; a command returning 1 writes nothing.
fn plain_fillet(
    script: &mut OffsetScript<'_>,
    result: &'static str,
    shape: &str,
    edges: &[(f64, &str)],
) -> Result<(), Failure> {
    let mut maker = FilletCommands::default().maker(&script.required(shape)?)?;
    if add_every_edge(script, &mut maker, edges)? == 0 {
        return Err(command_stopped(""));
    }
    maker.build(&ProgressRange::default())?;
    if !maker.is_done() {
        return Err(command_stopped(""));
    }
    script.set_shape(result, maker.shape()?.clone());
    Ok(())
}

fn add_every_edge(
    script: &OffsetScript<'_>,
    maker: &mut BRepFilletAPIMakeFillet,
    edges: &[(f64, &str)],
) -> Result<usize, Failure> {
    let mut added = 0;
    for (radius, name) in edges.iter().filter(|(radius, _)| *radius != 0.0) {
        let mut found = Explorer::new(&script.shape(name), ShapeEnum::Edge, ShapeEnum::Shape)?;
        while found.more() {
            maker.add_with_radius(*radius, &topods::edge(found.current())?)?;
            added += 1;
            found.next()?;
        }
    }
    Ok(added)
}

/// `blend`'s edges: a name that holds no edge is skipped; how many were added.
fn add_defined_edges(
    script: &OffsetScript<'_>,
    maker: &mut BRepFilletAPIMakeFillet,
    edges: &[(f64, &str)],
) -> Result<usize, Failure> {
    let mut added = 0;
    for (radius, name) in edges {
        let edge = script.shape(name);
        if !edge.is_null() && edge.shape_type()? == ShapeEnum::Edge {
            maker.add_with_radius(*radius, &topods::edge(&edge)?)?;
            added += 1;
        }
    }
    Ok(added)
}

/// The distinct edges the saved history generated from the faces of `source`.
fn generated_edges(script: &OffsetScript<'_>, source: &str) -> Result<Vec<Shape>, Failure> {
    let history = script
        .saved_history
        .clone()
        .ok_or_else(|| command_stopped("no saved history"))?;
    let live = history
        .get()
        .ok_or_else(|| command_stopped("null history"))?;
    let (mut seen, mut edges) = (MapOfShape::default(), Vec::new());
    let mut faces = Explorer::new(&script.required(source)?, ShapeEnum::Face, ShapeEnum::Shape)?;
    while faces.more() {
        add_generated_edges(live, faces.current(), &mut seen, &mut edges)?;
        faces.next()?;
    }
    Ok(edges)
}

/// `generated`, which says so when a shape generated nothing.
fn add_generated_edges(
    history: &History,
    face: &Shape,
    seen: &mut MapOfShape,
    edges: &mut Vec<Shape>,
) -> Result<(), Failure> {
    let generated = history.generated(face)?;
    if generated.is_empty() {
        println!("No shapes were generated from the shape.");
    }
    for shape in &generated {
        add_new_edges(shape, seen, edges)?;
    }
    Ok(())
}

fn add_new_edges(
    shape: &Shape,
    seen: &mut MapOfShape,
    edges: &mut Vec<Shape>,
) -> Result<(), Failure> {
    let mut found = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape)?;
    while found.more() {
        if seen.add(found.current().clone()) {
            edges.push(found.current().clone());
        }
        found.next()?;
    }
    Ok(())
}

/// The pave filler of `bfuseblend` and `bcutblend`, or the failure that stops the script.
fn performed_filler(s1: &Shape, s2: &Shape) -> Result<BOPAlgoPaveFiller, Failure> {
    let mut filler = BOPAlgoPaveFiller::default();
    filler.set_arguments(&[s1.clone(), s2.clone()].into_iter().collect());
    filler.perform();
    if filler.options().has_errors() {
        return Err(command_stopped("Check types of the arguments, please"));
    }
    Ok(filler)
}

/// A Boolean operation with the default `BooleanObjects` options.
fn configured_bop(a: &Shape, b: &Shape, operation: Operation) -> BOPAlgoBOP {
    let mut bop = BOPAlgoBOP::new();
    bop.add_argument(a);
    bop.add_tool(b);
    bop.set_operation(operation);
    bop.set_glue(Glue::Off);
    bop.options_mut().set_fuzzy_value(precision::confusion());
    bop.set_non_destructive(false);
    bop.options_mut().set_run_parallel(false);
    bop.options_mut().set_use_obb(false);
    bop.set_check_inverted(true);
    bop.set_to_fill_history(true);
    bop
}

/// `bfuse` and `bcut`: `bsmt` with the default `BooleanObjects` options. Errors are reported
/// as alerts and leave the result unset, so they stop nothing by themselves.
pub fn bsmt(
    script: &mut OffsetScript<'_>,
    result: &'static str,
    (a, b): (&str, &str),
    operation: Operation,
) -> Result<(), Failure> {
    let mut bop = configured_bop(&script.required(a)?, &script.required(b)?, operation);
    bop.perform();
    report_result_alerts(bop.options());
    let filled: &mut BuilderShape = &mut bop;
    script.last_history = filled.history();
    if !bop.options().has_errors() {
        script.set_shape(result, bop.shape().clone());
    }
    Ok(())
}
