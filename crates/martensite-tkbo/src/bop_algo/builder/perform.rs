//! `BOPAlgoBuilder::Perform`, `PerformWithFiller`, `PerformInternal(1)`, `CheckData`,
//! `CheckFiller`, `Prepare`, `BuildResult` and `PostTreat` (BOPAlgoBuilder).
use super::{BOPAlgoBuilder, SameDomainShapes};
use crate::bop_algo::alerts::alert;
use crate::bop_algo::{AlertBuilderFailed, AlertTooFewArguments, BOPAlgoPaveFiller};
use crate::bop_ds::DataStructure;
use crate::bop_tools::make_container;
use martensite_tkbrep::brep::BRepBuilder;
use martensite_tkbrep::topods::Shape;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::{List, Map};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;

/// The virtual methods of `BOPAlgoBuilder` that the General Fuse pipeline calls and the
/// derived algorithms override (`CheckData`, `BuildResult`).
pub trait BuilderHooks {
    /// Run this actual receiver's whole ranged operation; `BuilderHooks::perform_with_progress(&mut builder, range)?`.
    /// Legacy-only receivers remain incomplete; raw completion still needs publication validation.
    fn perform_with_progress(
        &mut self,
        _range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        Ok(crate::bop_algo::ProgressCompletion::Incomplete {
            operation: crate::bop_algo::ProgressOperation::Builder,
            stage: crate::bop_algo::ProgressStage::Completion,
            class: crate::bop_algo::ProgressFailureClass::UnqualifiedStage,
            cause: None,
            context: None,
        })
    }

    /// The General Fuse state; `hooks.general_fuse().arguments()`.
    fn general_fuse(&mut self) -> &mut BOPAlgoBuilder;

    /// Checks the arguments (`CheckData`): at least two, then the filler report merged.
    fn check_data(&mut self, filler: &BOPAlgoPaveFiller) {
        self.general_fuse().check_data_default(filler);
    }

    /// Adds the images of the arguments of `kind` to the result (`BuildResult`).
    /// The base adds the images of `arguments()`; derived algorithms may read the DS.
    fn build_result(&mut self, kind: ShapeEnum, _structure: &DataStructure) -> Result<(), Failure> {
        self.general_fuse().build_result_default(kind)
    }

    /// Prepares the result after the check (`Prepare`): an empty compound; derived
    /// algorithms (BRepFeatBuilder) add their own preparation.
    fn prepare(&mut self) -> Result<(), Failure> {
        self.general_fuse().prepare();
        Ok(())
    }

    /// Classifies the face splits against the draft solids (`FillIn3DParts`); derived
    /// algorithms (BRepFeatBuilder) filter the IN parts afterwards.
    fn fill_in_3d_parts(
        &mut self,
        structure: &mut DataStructure,
        draft_solids: &mut SameDomainShapes,
    ) -> Result<(), Failure> {
        self.general_fuse()
            .fill_in_3d_parts(structure, draft_solids)
    }

    /// Classify through this actual ranged receiver; `hooks.fill_in_3d_parts_with_progress(ds, drafts, range)?`.
    /// Custom legacy-only receivers remain explicitly incomplete until they implement this hook.
    fn fill_in_3d_parts_with_progress(
        &mut self,
        _structure: &mut DataStructure,
        _draft_solids: &mut SameDomainShapes,
        _range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        Ok(crate::bop_algo::ProgressCompletion::Incomplete {
            operation: crate::bop_algo::ProgressOperation::Builder,
            stage: crate::bop_algo::ProgressStage::Images,
            class: crate::bop_algo::ProgressFailureClass::UnqualifiedStage,
            cause: None,
            context: None,
        })
    }

    /// Execute this receiver's ranged build program; `hooks.perform_internal1_with_progress(filler, range)?`.
    /// Unknown legacy-only programs must opt in rather than running a base substitute.
    fn perform_internal1_with_progress(
        &mut self,
        _filler: &mut BOPAlgoPaveFiller,
        _range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        Ok(crate::bop_algo::ProgressCompletion::Incomplete {
            operation: crate::bop_algo::ProgressOperation::Builder,
            stage: crate::bop_algo::ProgressStage::Completion,
            class: crate::bop_algo::ProgressFailureClass::UnqualifiedStage,
            cause: None,
            context: None,
        })
    }

    /// The pipeline run on the results of `filler` (`PerformInternal1`).
    fn perform_internal1(&mut self, filler: &mut BOPAlgoPaveFiller) -> Result<(), Failure>
    where
        Self: Sized,
    {
        perform_internal1(self, filler)
    }
}

impl BuilderHooks for BOPAlgoBuilder {
    fn perform_with_progress(
        &mut self,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
        let shapes = self.arguments.clone();
        super::entry_progress::perform_general_fuse_on_progress(self, shapes, range)
    }

    fn perform_internal1_with_progress(
        &mut self,
        filler: &mut crate::bop_algo::BOPAlgoPaveFiller,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, martensite_tkernel::standard::Failure> {
        crate::bop_algo::builder::operation_progress::run_general_fuse(
            self,
            filler,
            range,
            crate::bop_algo::builder::operation_progress::BuildMode::Ranged,
        )
    }

    fn fill_in_3d_parts_with_progress(
        &mut self,
        structure: &mut crate::bop_ds::DataStructure,
        drafts: &mut crate::bop_algo::builder::SameDomainShapes,
        range: &martensite_tkernel::message::ProgressRange<'_>,
    ) -> Result<crate::bop_algo::ProgressCompletion, martensite_tkernel::standard::Failure> {
        self.general_fuse()
            .fill_in_3d_parts_with_progress(structure, drafts, range)
    }

    fn general_fuse(&mut self) -> &mut BOPAlgoBuilder {
        self
    }
}

impl BOPAlgoBuilder {
    /// Intersects the arguments with an own pave filler, then builds the splits (`Perform`).
    /// `builder.perform();`.
    pub fn perform(&mut self) {
        let shapes = self.arguments.clone();
        perform_on(self, shapes);
    }

    /// Builds the splits from the results of `filler`, taking its options
    /// (`PerformWithFiller`); `builder.perform_with_filler(&mut filler);`.
    pub fn perform_with_filler(&mut self, filler: &mut BOPAlgoPaveFiller) {
        perform_with(self, filler);
    }

    /// A pave filler on `arguments` with the options of this builder.
    pub(crate) fn new_filler(&self, arguments: List<Shape>) -> BOPAlgoPaveFiller {
        let mut filler = BOPAlgoPaveFiller::new();
        filler.set_arguments(&arguments);
        let options = self.options();
        filler
            .options_mut()
            .set_run_parallel(options.run_parallel());
        filler.options_mut().set_fuzzy_value(options.fuzzy_value());
        filler.set_non_destructive(self.non_destructive);
        filler.set_glue(self.glue);
        filler.options_mut().set_use_obb(options.use_obb());
        filler
    }

    /// At least two arguments, then the filler report merged (`CheckData`).
    pub(crate) fn check_data_default(&mut self, filler: &BOPAlgoPaveFiller) {
        if self.arguments.len() < 2 {
            self.options().add_error(&alert(AlertTooFewArguments));
            return;
        }
        self.check_filler(filler);
    }

    /// Merges the report of the filler (`CheckFiller`).
    pub(crate) fn check_filler(&mut self, filler: &BOPAlgoPaveFiller) {
        self.options().report().merge(filler.options().report());
    }

    /// The result starts as an empty compound (`Prepare`); `builder.prepare()`.
    pub fn prepare(&mut self) {
        make_container(ShapeEnum::Compound, &mut self.shape);
    }

    /// Adds the images of the arguments of `kind`, or the arguments themselves, to the
    /// result once each (`BuildResult`).
    pub(crate) fn build_result_default(&mut self, kind: ShapeEnum) -> Result<(), Failure> {
        let arguments: Vec<Shape> = self.arguments.iter().cloned().collect();
        self.add_argument_images(&arguments, kind)
    }

    /// Adds the images of the `arguments` of `kind`, or the arguments themselves, to the
    /// result once each.
    pub(crate) fn add_argument_images(
        &mut self,
        arguments: &[Shape],
        kind: ShapeEnum,
    ) -> Result<(), Failure> {
        let mut fence: Map<Shape, ShapeMapHasher> = Map::with_hasher(ShapeMapHasher, 1);
        let builder = BRepBuilder::default();
        for argument in arguments {
            if argument.shape_type()? != kind {
                continue;
            }
            let images: Vec<Shape> = match self.images.seek(argument) {
                Some(images) => images.iter().cloned().collect(),
                None => vec![argument.clone()],
            };
            for image in images {
                if fence.add(image.clone()) {
                    builder.add(&mut self.base.shape, &image)?;
                }
            }
        }
        Ok(())
    }
}

/// Intersects `shapes` with a pave filler made with the builder options, runs the pipeline
/// and keeps the filler (`Perform` of the builders, entry point 1); a derived algorithm
/// runs its own hooks through it; `perform_on(&mut algo, shapes)`.
pub fn perform_on<H: BuilderHooks>(hooks: &mut H, shapes: List<Shape>) {
    let _ = perform_on_using(
        hooks,
        shapes,
        BOPAlgoPaveFiller::perform,
        |actual, filler, ()| actual.perform_internal1(filler),
    );
}

/// Retain the actual receiver while a caller borrows operation-local progress.
/// Example: `perform_on_using(hooks, shapes, |pf| pf.perform(), |h, pf, ()| h.perform_internal1(pf));`.
pub(crate) fn perform_on_using<H: BuilderHooks, I, R>(
    hooks: &mut H,
    shapes: List<Shape>,
    intersect: impl FnOnce(&mut BOPAlgoPaveFiller) -> I,
    build: impl FnOnce(&mut H, &mut BOPAlgoPaveFiller, I) -> Result<R, Failure>,
) -> Result<R, Failure> {
    hooks.general_fuse().options().clear();
    let mut filler = hooks.general_fuse().new_filler(shapes);
    let prepared = intersect(&mut filler);
    let outcome = perform_internal_using(hooks, &mut filler, |h, pf| build(h, pf, prepared));
    hooks.general_fuse().filler = Some(filler);
    outcome
}

/// Takes the options of `filler` and runs the pipeline on it (`PerformWithFiller`, entry
/// point 0: the filler stays with the caller).
pub(crate) fn perform_with<H: BuilderHooks>(hooks: &mut H, filler: &mut BOPAlgoPaveFiller) {
    adopt_supplied_options(hooks, filler);
    perform_internal(hooks, filler);
}

/// Take supplied filler options without selecting a receiver program.
fn adopt_supplied_options<H: BuilderHooks>(hooks: &mut H, filler: &BOPAlgoPaveFiller) {
    let builder = hooks.general_fuse();
    builder.options().clear();
    builder.filler = None;
    builder.non_destructive = filler.non_destructive();
    let (fuzzy, use_obb) = (filler.options().fuzzy_value(), filler.options().use_obb());
    builder.options_mut().set_fuzzy_value(fuzzy);
    builder.glue = filler.glue();
    builder.options_mut().set_use_obb(use_obb);
}

/// Build from a prepared filler through the selected ranged receiver.
/// Example: `perform_with_filler_progress(&mut builder, &mut filler, &indicator.start())?`.
/// Execution completion still requires independent geometry validation before publication.
pub fn perform_with_filler_progress<H: BuilderHooks>(
    hooks: &mut H,
    filler: &mut BOPAlgoPaveFiller,
    range: &martensite_tkernel::message::ProgressRange<'_>,
) -> Result<crate::bop_algo::ProgressCompletion, Failure> {
    adopt_supplied_options(hooks, filler);
    let completion = perform_internal_using(hooks, filler, |actual, prepared| {
        actual.perform_internal1_with_progress(prepared, range)
    })?;
    if matches!(
        completion,
        crate::bop_algo::ProgressCompletion::Failed { cause: Some(_), .. }
    ) {
        hooks
            .general_fuse()
            .options()
            .add_error(&alert(AlertBuilderFailed));
    }
    Ok(completion)
}

/// Runs the pipeline; a failure becomes an `AlertBuilderFailed` error, as the native catch
/// of StandardFailure does (`PerformInternal`).
pub(crate) fn perform_internal<H: BuilderHooks>(hooks: &mut H, filler: &mut BOPAlgoPaveFiller) {
    let _ = perform_internal_using(hooks, filler, |actual, prepared| {
        actual.perform_internal1(prepared)
    });
}

/// Keep the existing failure boundary around the selected actual-receiver callback.
/// Example: `perform_internal_using(hooks, filler, |h, pf| h.perform_internal1(pf));`.
pub(crate) fn perform_internal_using<H: BuilderHooks, R>(
    hooks: &mut H,
    filler: &mut BOPAlgoPaveFiller,
    build: impl FnOnce(&mut H, &mut BOPAlgoPaveFiller) -> Result<R, Failure>,
) -> Result<R, Failure> {
    hooks.general_fuse().options().clear();
    let outcome = build(hooks, filler);
    if outcome.is_err() {
        hooks
            .general_fuse()
            .options()
            .add_error(&alert(AlertBuilderFailed));
    }
    outcome
}

/// The General Fuse pipeline (`PerformInternal1`): images of vertices, edges, wires,
/// faces, shells, solids, compsolids and compounds, each followed by `BuildResult`, then
/// the history and the post treatment.
pub(crate) fn perform_internal1<H: BuilderHooks>(
    hooks: &mut H,
    filler: &mut BOPAlgoPaveFiller,
) -> Result<(), Failure> {
    let completion = super::operation_progress::run_general_fuse(
        hooks,
        filler,
        &martensite_tkernel::message::ProgressRange::default(),
        super::operation_progress::BuildMode::Unbound,
    )?;
    crate::bop_algo::builder_area::progress::legacy_unbound_area_result(completion)
}

/// Run the existing image/result hooks on their shared stage dispatcher.
/// Example: `fill_images_with_results(hooks, structure)?` preserves the native report decisions.
pub(crate) fn fill_images_with_results<H: BuilderHooks>(
    hooks: &mut H,
    structure: &mut DataStructure,
) -> Result<bool, Failure> {
    super::operation_progress::run_unbound_images(hooks, structure)?;
    Ok(!hooks.general_fuse().options().has_errors())
}

/// Takes the context and the options of the filler (the first lines of `PerformInternal1`).
pub(crate) fn adopt_filler(builder: &mut BOPAlgoBuilder, filler: &BOPAlgoPaveFiller) {
    builder.context = filler.context_handle();
    let fuzzy = filler.options().fuzzy_value();
    builder.options_mut().set_fuzzy_value(fuzzy);
    builder.non_destructive = filler.non_destructive();
}

/// The failure of a pipeline run on a filler that built no data structure.
pub(crate) fn no_structure() -> Failure {
    Failure::with_type(
        "Standard_NullObject",
        "BOPAlgoBuilder: the pave filler has no data structure; expected a performed filler",
        "",
    )
}

#[cfg(test)]
#[path = "driver_contract.rs"]
mod driver_contract;
