//! `IgesToBRepCurveAndSurface`: the settings, the model and the process of a transfer of geometry.
use crate::iges_data::{IgesDataIgesEntity, IgesDataIgesModel, iges_live};
use martensite_tkbrep::topods::Shape;
use martensite_tkernel::message::Msg;
use martensite_tkernel::precision;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::geom::GeomSurface;
use martensite_tkg3d::geom_adaptor::GeomAdaptorSurface;
use martensite_tkxsbase::interface::{InterfaceInterfaceModel, InterfaceStatic};
use martensite_tkxsbase::transfer::TransferBinder;
use martensite_tkxsbase::transfer::TransferTransientProcess;
use martensite_tkxsbase::transfer_brep::{TransferBRepShapeBinder, TransferBRepShapeListBinder};

const WHERE: &str = "IgesToBRepCurveAndSurface";

/// The settings of a transfer, shared by the curve and surface translators.
///
/// ```
/// use martensite_tkdeiges::iges_to_brep::IgesToBRepCurveAndSurface;
/// let tool = IgesToBRepCurveAndSurface::new();
/// assert_eq!(tool.get_eps_geom(), 1.0e-4);
/// assert!(tool.get_mode_transfer());
/// ```
#[derive(Clone)]
pub struct IgesToBRepCurveAndSurface {
    eps: f64,
    eps_coeff: f64,
    eps_geom: f64,
    min_tol: f64,
    max_tol: f64,
    mode_is_topo: bool,
    mode_approx: bool,
    cont_is_opti: bool,
    unit_factor: f64,
    surface_curve: i32,
    continuity: i32,
    surface: Handle<dyn GeomSurface>,
    uv_resolution: f64,
    is_resol_com: bool,
    model: Handle<IgesDataIgesModel>,
    tp: Handle<TransferTransientProcess>,
}

impl Default for IgesToBRepCurveAndSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl IgesToBRepCurveAndSurface {
    /// A tool with epsilons at 1e-4 (1e-6 for coefficients), in topology mode, without optimization;
    /// `IgesToBRepCurveAndSurface::new()`.
    pub fn new() -> Self {
        Self::with_settings(1.0e-4, 1.0e-6, 1.0e-4, true, false, false)
    }
    /// A tool with the given settings;
    /// `IgesToBRepCurveAndSurface::with_settings(1e-4, 1e-6, 1e-4, true, false, false)`.
    pub fn with_settings(
        eps: f64,
        eps_coeff: f64,
        eps_geom: f64,
        mode: bool,
        mode_approx: bool,
        optimized: bool,
    ) -> Self {
        let mut tool = Self {
            eps,
            eps_coeff,
            eps_geom,
            min_tol: -1.0,
            max_tol: -1.0,
            mode_is_topo: mode,
            mode_approx,
            cont_is_opti: optimized,
            unit_factor: 1.0,
            surface_curve: 0,
            continuity: 0,
            surface: Handle::default(),
            uv_resolution: 0.0,
            is_resol_com: false,
            model: Handle::default(),
            tp: TransferTransientProcess::new(10000),
        };
        tool.update_min_max_tol();
        tool
    }
    /// Back to the default settings; `tool.init()`.
    pub fn init(&mut self) {
        let model = self.model.clone();
        *self = Self::new();
        // The model stays: only the settings are defaults.
        self.model = model;
    }
    /// Sets the general epsilon; `tool.set_epsilon(1e-4)`.
    pub fn set_epsilon(&mut self, eps: f64) {
        self.eps = eps;
    }
    /// The general epsilon; `tool.get_epsilon()`.
    pub fn get_epsilon(&self) -> f64 {
        self.eps
    }
    /// Sets the epsilon of coefficients; `tool.set_eps_coeff(1e-6)`.
    pub fn set_eps_coeff(&mut self, eps: f64) {
        self.eps_coeff = eps;
    }
    /// The epsilon of coefficients; `tool.get_eps_coeff()`.
    pub fn get_eps_coeff(&self) -> f64 {
        self.eps_coeff
    }
    /// Sets the epsilon of geometry, which updates the tolerance limits; `tool.set_eps_geom(1e-4)`.
    pub fn set_eps_geom(&mut self, eps: f64) {
        self.eps_geom = eps;
        self.update_min_max_tol();
    }
    /// The epsilon of geometry; `tool.get_eps_geom()`.
    pub fn get_eps_geom(&self) -> f64 {
        self.eps_geom
    }
    /// Sets the minimal tolerance; `tool.set_min_tol(1e-7)`.
    pub fn set_min_tol(&mut self, tolerance: f64) {
        self.min_tol = tolerance;
    }
    /// Sets the maximal tolerance; `tool.set_max_tol(1.0)`.
    pub fn set_max_tol(&mut self, tolerance: f64) {
        self.max_tol = tolerance;
    }
    /// The maximal tolerance from the `read.maxprecision.val` setting and the epsilon of geometry in
    /// the file unit, the minimal one at the confusion; runs on each change of those;
    /// `tool.update_min_max_tol()`.
    pub fn update_min_max_tol(&mut self) {
        // The maximum follows the static parameter.
        self.max_tol =
            InterfaceStatic::r_val("read.maxprecision.val").max(self.eps_geom * self.unit_factor);
        self.min_tol = precision::confusion();
    }
    /// The minimal tolerance; `tool.get_min_tol()`.
    pub fn get_min_tol(&self) -> f64 {
        self.min_tol
    }
    /// The maximal tolerance; `tool.get_max_tol()`.
    pub fn get_max_tol(&self) -> f64 {
        self.max_tol
    }
    /// Sets the approximation mode; `tool.set_mode_approx(true)`.
    pub fn set_mode_approx(&mut self, mode: bool) {
        self.mode_approx = mode;
    }
    /// The approximation mode; `tool.get_mode_approx()`.
    pub fn get_mode_approx(&self) -> bool {
        self.mode_approx
    }
    /// Sets the topology mode; `tool.set_mode_transfer(false)`.
    pub fn set_mode_transfer(&mut self, mode: bool) {
        self.mode_is_topo = mode;
    }
    /// The topology mode; `tool.get_mode_transfer()`.
    pub fn get_mode_transfer(&self) -> bool {
        self.mode_is_topo
    }
    /// Sets the optimization of the continuity; `tool.set_optimized(true)`.
    pub fn set_optimized(&mut self, optimized: bool) {
        self.cont_is_opti = optimized;
    }
    /// The optimization of the continuity; `tool.get_optimized()`.
    pub fn get_optimized(&self) -> bool {
        self.cont_is_opti
    }
    /// The file unit; `tool.get_unit_factor()`.
    pub fn get_unit_factor(&self) -> f64 {
        self.unit_factor
    }
    /// Sets the priority of curves on surfaces; `tool.set_surface_curve(2)`.
    pub fn set_surface_curve(&mut self, value: i32) {
        self.surface_curve = value;
    }
    /// The priority of curves on surfaces: 0 the file, 2 keep 2D, 3 keep 3D;
    /// `tool.get_surface_curve()`.
    pub fn get_surface_curve(&self) -> i32 {
        self.surface_curve
    }
    /// Sets the model; a unit other than 1 in its global section becomes the file unit;
    /// `tool.set_model(&model)?`.
    ///
    /// # Errors
    /// A null object failure for a null model.
    pub fn set_model(&mut self, model: &Handle<IgesDataIgesModel>) -> Result<(), Failure> {
        let unit = iges_live(model, WHERE)?.global_section().unit_value();
        self.model = model.clone();
        if unit != 1.0 {
            self.report_unit(unit);
            self.unit_factor = unit;
        }
        self.update_min_max_tol();
        Ok(())
    }
    fn report_unit(&self, unit: f64) {
        if let Some(process) = self.tp.get()
            && process.trace_level() > 2
        {
            use std::fmt::Write;
            let _ = writeln!(process.messenger().send_info(), "UnitFactor = {unit}");
        }
    }
    /// The model; `tool.get_model()`.
    pub fn get_model(&self) -> Handle<IgesDataIgesModel> {
        self.model.clone()
    }
    /// Sets the continuity to try: 0 nothing, 1 C1, 2 C2; `tool.set_continuity(1)`.
    pub fn set_continuity(&mut self, continuity: i32) {
        self.continuity = continuity;
    }
    /// The continuity to try; `tool.get_continuity()`.
    pub fn get_continuity(&self) -> i32 {
        self.continuity
    }
    /// Sets the process that holds messages and results; `tool.set_transfer_process(&process)`.
    pub fn set_transfer_process(&mut self, process: &Handle<TransferTransientProcess>) {
        self.tp = process.clone();
    }
    /// The process; `tool.get_transfer_process()`.
    pub fn get_transfer_process(&self) -> Handle<TransferTransientProcess> {
        self.tp.clone()
    }
    /// Records a fail on an entity; `tool.send_fail(&entity, &message)`.
    pub fn send_fail<T: ?Sized + RuntimeTyped>(&self, start: &Handle<T>, message: &Msg) {
        if let Some(process) = self.tp.get() {
            process.send_fail(&start.cast(), message);
        }
    }
    /// Records a failure of an entity's transfer on its check and gives the empty result that
    /// stands for it; `tool.no_result(&entity, &failure)`.
    pub(super) fn no_result<S, R>(&self, start: &Handle<S>, failure: &Failure) -> R
    where
        S: ?Sized + RuntimeTyped,
        R: Default,
    {
        if let Some(process) = self.tp.get() {
            process.add_fail_text(&start.cast(), &failure.to_string(), "");
        }
        R::default()
    }
    /// Records a warning on an entity; `tool.send_warning(&entity, &message)`.
    pub fn send_warning<T: ?Sized + RuntimeTyped>(&self, start: &Handle<T>, message: &Msg) {
        if let Some(process) = self.tp.get() {
            process.send_warning(&start.cast(), message);
        }
    }
    /// Records an information on an entity; `tool.send_msg(&entity, &message)`.
    pub fn send_msg<T: ?Sized + RuntimeTyped>(&self, start: &Handle<T>, message: &Msg) {
        if let Some(process) = self.tp.get() {
            process.send_msg(&start.cast(), message);
        }
    }
    /// Records the fail of a catalog key.
    pub(super) fn fail_key<T: ?Sized + RuntimeTyped>(&self, start: &Handle<T>, key: &str) {
        self.send_fail(start, &Msg::from_key(&key.into()));
    }
    /// Records the warning of a catalog key.
    pub(super) fn warn_key<T: ?Sized + RuntimeTyped>(&self, start: &Handle<T>, key: &str) {
        self.send_warning(start, &Msg::from_key(&key.into()));
    }
    /// Records the warning of a catalog key with its arguments.
    pub(super) fn warn_with_args<T: ?Sized + RuntimeTyped>(
        &self,
        start: &Handle<T>,
        key: &str,
        arguments: &[&[u8]],
    ) -> Result<(), Failure> {
        let mut message = Msg::from_key(&key.into());
        for argument in arguments {
            message.arg_text(argument)?;
        }
        self.send_warning(start, &message);
        Ok(())
    }
    /// The message that an entity gave no shape, with its label.
    pub(super) fn label_message<T: ?Sized + RuntimeTyped>(
        &self,
        entity: &Handle<T>,
        what: &[u8],
    ) -> Result<Msg, Failure> {
        let label = iges_live(&self.model, WHERE)?.string_label(&entity.cast());
        let mut message = Msg::from_key(&"IGES_1156".into());
        message.arg_text(what)?;
        message.arg_h_ascii(iges_live(&label, WHERE)?)?;
        Ok(message)
    }
    fn process(&self) -> Result<&TransferTransientProcess, Failure> {
        iges_live(&self.tp, WHERE)
    }
    fn bound_shape_binder(
        &self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<TransferBRepShapeBinder>, Failure> {
        Ok(self.process()?.find(&start.cast()).cast())
    }
    fn bound_list_binder(
        &self,
        start: &Handle<IgesDataIgesEntity>,
    ) -> Result<Handle<TransferBRepShapeListBinder>, Failure> {
        Ok(self.process()?.find(&start.cast()).cast())
    }
    /// Whether an entity already has a result; `tool.has_shape_result(&entity)?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn has_shape_result(&self, start: &Handle<IgesDataIgesEntity>) -> Result<bool, Failure> {
        Ok(self
            .bound_shape_binder(start)?
            .get()
            .is_some_and(|binder| binder.has_result()))
    }
    /// The result of an entity, a null shape without one; `tool.get_shape_result(&entity)?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn get_shape_result(&self, start: &Handle<IgesDataIgesEntity>) -> Result<Shape, Failure> {
        let binder = self.bound_shape_binder(start)?;
        Ok(match binder.get() {
            Some(live) if live.has_result() => live.result(),
            _ => Shape::default(),
        })
    }
    /// Records the result of an entity; `tool.set_shape_result(&entity, &shape)?`.
    ///
    /// # Errors
    /// A failure of the binding of the result.
    pub fn set_shape_result(
        &self,
        start: &Handle<IgesDataIgesEntity>,
        result: &Shape,
    ) -> Result<(), Failure> {
        let binder = Handle::new(TransferBRepShapeBinder::new());
        self.process()?.bind(&start.cast(), &binder.cast())?;
        iges_live(&binder, WHERE)?.set_result(result)
    }
    /// The count of results of a vertex or an edge list; `tool.nb_shape_result(&entity)?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn nb_shape_result(&self, start: &Handle<IgesDataIgesEntity>) -> Result<i32, Failure> {
        match self.bound_list_binder(start)?.get() {
            Some(live) => live.nb_shapes(),
            None => Ok(0),
        }
    }
    /// A result of a vertex or an edge list by rank, a null shape past the end;
    /// `tool.get_shape_result_at(&entity, 1)?`.
    ///
    /// # Errors
    /// A null object failure for a null process.
    pub fn get_shape_result_at(
        &self,
        start: &Handle<IgesDataIgesEntity>,
        num: i32,
    ) -> Result<Shape, Failure> {
        let binder = self.bound_list_binder(start)?;
        let Some(live) = binder.get() else {
            return Ok(Shape::default());
        };
        if num <= live.nb_shapes()? {
            return live.shape(num);
        }
        Ok(Shape::default())
    }
    /// Adds a result of a vertex or an edge list; `tool.add_shape_result(&entity, &shape)?`.
    ///
    /// # Errors
    /// A failure of the binding of the result.
    pub fn add_shape_result(
        &self,
        start: &Handle<IgesDataIgesEntity>,
        result: &Shape,
    ) -> Result<(), Failure> {
        let mut binder = self.bound_list_binder(start)?;
        if binder.is_null() {
            binder = Handle::new(TransferBRepShapeListBinder::new());
            self.process()?.bind(&start.cast(), &binder.cast())?;
        }
        iges_live(&binder, WHERE)?.add_result(result)
    }
    /// Sets the surface of curves on surface, forgetting its resolution when it changes;
    /// `tool.set_surface(&surface)`.
    pub fn set_surface(&mut self, surface: &Handle<dyn GeomSurface>) {
        if self.surface.object_address() != surface.object_address() {
            self.surface = surface.clone();
            self.is_resol_com = false;
            self.uv_resolution = 0.0;
        }
    }
    /// The surface of curves on surface; `tool.surface()`.
    pub fn surface(&self) -> Handle<dyn GeomSurface> {
        self.surface.clone()
    }
    /// The parametric length of a unit on the surface, computed once;
    /// `tool.get_uv_resolution()?`.
    ///
    /// # Errors
    /// A failure of the adaptor of the surface.
    pub fn get_uv_resolution(&mut self) -> Result<f64, Failure> {
        if !self.is_resol_com && self.surface.is_some() {
            self.is_resol_com = true;
            let adaptor = GeomAdaptorSurface::with_surface(&self.surface)?;
            self.uv_resolution = adaptor.u_resolution(1.0)?.min(adaptor.v_resolution(1.0)?);
        }
        Ok(self.uv_resolution)
    }
}

/// Declares a translator: its settings by `deref`, its constructors from settings.
macro_rules! curve_and_surface_tool {
    ($(#[$meta:meta])* $tool:ident, $initial:expr) => {
        curve_and_surface_tool!($(#[$meta])* $tool, $initial, |_| {});
    };
    ($(#[$meta:meta])* $tool:ident, $initial:expr, $always:expr) => {
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $tool {
            base: $crate::iges_to_brep::IgesToBRepCurveAndSurface,
        }
        impl std::ops::Deref for $tool {
            type Target = $crate::iges_to_brep::IgesToBRepCurveAndSurface;
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }
        impl std::ops::DerefMut for $tool {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
        impl Default for $tool {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $tool {
            #[doc = concat!("A `", stringify!($tool), "` at its initial settings.")]
            pub fn new() -> Self {
                let mut base = $crate::iges_to_brep::IgesToBRepCurveAndSurface::new();
                let initial: fn(&mut $crate::iges_to_brep::IgesToBRepCurveAndSurface) = $initial;
                initial(&mut base);
                Self::with_owned_base(base)
            }
            fn with_owned_base(mut base: $crate::iges_to_brep::IgesToBRepCurveAndSurface) -> Self {
                let always: fn(&mut $crate::iges_to_brep::IgesToBRepCurveAndSurface) = $always;
                always(&mut base);
                Self { base }
            }
            /// A tool with the settings of `base`.
            pub fn with_base(base: &$crate::iges_to_brep::IgesToBRepCurveAndSurface) -> Self {
                Self::with_owned_base(base.clone())
            }
            /// A tool with the given settings.
            pub fn with_settings(
                eps: f64,
                eps_coeff: f64,
                eps_geom: f64,
                mode: bool,
                mode_approx: bool,
                optimized: bool,
            ) -> Self {
                Self::with_owned_base($crate::iges_to_brep::IgesToBRepCurveAndSurface::with_settings(
                    eps, eps_coeff, eps_geom, mode, mode_approx, optimized,
                ))
            }
            /// The settings as the plain tool.
            pub fn base(&self) -> &$crate::iges_to_brep::IgesToBRepCurveAndSurface {
                &self.base
            }
        }
    };
}
pub(super) use curve_and_surface_tool;

#[cfg(test)]
mod no_result_tests {
    use super::IgesToBRepCurveAndSurface;
    use crate::iges_data::IgesDataIgesEntity;
    use martensite_tkernel::standard::{Failure, Handle};

    #[test]
    fn a_failed_transfer_gives_an_empty_result_and_its_reason_on_the_check() {
        let tool = IgesToBRepCurveAndSurface::new();
        let entity = Handle::new(IgesDataIgesEntity::default());
        let failure = Failure::new("the geometry refused it");
        let result: Handle<IgesDataIgesEntity> = tool.no_result(&entity, &failure);
        assert!(result.is_null());
        let process = tool.get_transfer_process();
        let check = process.get().unwrap().check(&entity.cast());
        let fails = check.get().unwrap().fails(true);
        let text = fails[0].get().unwrap().string();
        assert!(String::from_utf8_lossy(text.as_bytes()).contains("the geometry refused it"));
    }
}
