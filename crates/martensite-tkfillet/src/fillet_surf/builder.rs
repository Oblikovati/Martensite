//! `FilletSurfBuilder`: the fillet surfaces of a contour of edges with a constant
//! radius, with their curves and pcurves on the support faces, without building the
//! solid.
use super::internal_builder::FilletSurfInternalBuilder;
use super::{ErrorTypeStatus, StatusDone, StatusType};
use crate::chfi3d::FilletShape;
use martensite_tkbrep::topods::{Face, Shape};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg3d::geom::{GeomCurve, GeomSurface, GeomTrimmedCurve};

/// The fillet surfaces of a contour (`FilletSurfBuilder`).
///
/// ```
/// use martensite_tkfillet::fillet_surf::{ErrorTypeStatus, FilletSurfBuilder, StatusDone};
/// use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
/// use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
/// let solid = BRepPrimAPIMakeBox::from_sizes(1., 1., 1.)?.shape()?.clone();
/// let b = FilletSurfBuilder::new(&solid, &[], 0.1, 1e-2, 1e-4, 1e-5)?;
/// assert_eq!(b.is_done(), StatusDone::IsNotOk);
/// assert_eq!(b.status_error(), ErrorTypeStatus::EmptyList);
/// # Ok::<(), martensite_tkernel::standard::Failure>(())
/// ```
pub struct FilletSurfBuilder {
    my_int_build: FilletSurfInternalBuilder,
    myisdone: StatusDone,
    myerrorstatus: ErrorTypeStatus,
}

/// StdFailNotDone of a member.
fn not_done(member: &str) -> Failure {
    Failure::with_message("StdFail_NotDone", member)
}

impl FilletSurfBuilder {
    /// `FilletSurfBuilder(S, E, R, Ta, Tapp3d, Tapp2d)` (the native defaults are 1e-2,
    /// 1e-4 and 1e-5): the contour of the edges `e` of the shape with the radius `r`,
    /// IsNotOk with the reason when the edges do not make a contour.
    pub fn new(
        s: &Shape,
        e: &[Shape],
        r: f64,
        ta: f64,
        tapp3d: f64,
        tapp2d: f64,
    ) -> Result<Self, Failure> {
        let mut my_int_build =
            FilletSurfInternalBuilder::new(s, FilletShape::Polynomial, ta, tapp3d, tapp2d)?;
        let add = my_int_build.add(e, r)?;
        let (myisdone, myerrorstatus) = match add {
            0 => (StatusDone::IsOk, ErrorTypeStatus::EmptyList),
            1 => (StatusDone::IsNotOk, ErrorTypeStatus::EmptyList),
            2 => (StatusDone::IsNotOk, ErrorTypeStatus::EdgeNotG1),
            3 => (StatusDone::IsNotOk, ErrorTypeStatus::FacesNotG1),
            4 => (StatusDone::IsNotOk, ErrorTypeStatus::EdgeNotOnShape),
            5 => (StatusDone::IsNotOk, ErrorTypeStatus::NotSharpEdge),
            _ => (StatusDone::IsNotOk, ErrorTypeStatus::EmptyList),
        };
        Ok(Self {
            my_int_build,
            myisdone,
            myerrorstatus,
        })
    }

    /// `Perform()`: the fillet surfaces; IsPartial when some are computed.
    pub fn perform(&mut self) -> Result<(), Failure> {
        if self.myisdone != StatusDone::IsOk {
            return Ok(());
        }
        self.my_int_build.perform()?;
        if self.my_int_build.done() {
            self.myisdone = StatusDone::IsOk;
        } else if self.my_int_build.nb_surface()? != 0 {
            (self.myisdone, self.myerrorstatus) =
                (StatusDone::IsPartial, ErrorTypeStatus::PbFilletCompute);
        } else {
            (self.myisdone, self.myerrorstatus) =
                (StatusDone::IsNotOk, ErrorTypeStatus::PbFilletCompute);
        }
        Ok(())
    }

    /// `IsDone()`.
    pub fn is_done(&self) -> StatusDone {
        self.myisdone
    }

    /// `StatusError()`.
    pub fn status_error(&self) -> ErrorTypeStatus {
        self.myerrorstatus
    }

    /// `NbSurface()`: StdFailNotDone when not computed.
    pub fn nb_surface(&self) -> Result<i32, Failure> {
        if self.is_done() != StatusDone::IsNotOk {
            return self.my_int_build.nb_surface();
        }
        Err(not_done("FilletSurfBuilder::NbSurface"))
    }

    /// StandardOutOfRange for a surface index out of `1..=NbSurface()`.
    fn check_index(&self, index: i32, member: &str) -> Result<(), Failure> {
        if index < 1 || index > self.nb_surface()? {
            return Err(Failure::out_of_range(member));
        }
        Ok(())
    }

    /// `SurfaceFillet(Index)`.
    pub fn surface_fillet(&self, index: i32) -> Result<Handle<dyn GeomSurface>, Failure> {
        self.check_index(index, "FilletSurfBuilder::SurfaceFillet")?;
        self.my_int_build.surface_fillet(index)
    }

    /// `TolApp3d(Index)`.
    pub fn tol_app3d(&self, index: i32) -> Result<f64, Failure> {
        self.check_index(index, "FilletSurfBuilder::TolApp3d")?;
        self.my_int_build.tol_app3d(index)
    }

    /// `SupportFace1(Index)`.
    pub fn support_face1(&self, index: i32) -> Result<Face, Failure> {
        self.check_index(index, "FilletSurfBuilder::SupportFace1")?;
        self.my_int_build.support_face1(index)
    }

    /// `SupportFace2(Index)`.
    pub fn support_face2(&self, index: i32) -> Result<Face, Failure> {
        self.check_index(index, "FilletSurfBuilder::SupportFace2")?;
        self.my_int_build.support_face2(index)
    }

    /// `CurveOnFace1(Index)`.
    pub fn curve_on_face1(&self, index: i32) -> Result<Handle<dyn GeomCurve>, Failure> {
        self.check_index(index, "FilletSurfBuilder::CurveOnFace1")?;
        self.my_int_build.curve_on_face1(index)
    }

    /// `CurveOnFace2(Index)`.
    pub fn curve_on_face2(&self, index: i32) -> Result<Handle<dyn GeomCurve>, Failure> {
        self.check_index(index, "FilletSurfBuilder::CurveOnFace2")?;
        self.my_int_build.curve_on_face2(index)
    }

    /// `PCurveOnFace1(Index)`.
    pub fn pcurve_on_face1(&self, index: i32) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        self.check_index(index, "FilletSurfBuilder::PCurveOnFace1")?;
        self.my_int_build.pcurve_on_face1(index)
    }

    /// `PCurve1OnFillet(Index)`.
    pub fn pcurve1_on_fillet(&self, index: i32) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        self.check_index(index, "FilletSurfBuilder::PCurve1OnFillet")?;
        self.my_int_build.pcurve1_on_fillet(index)
    }

    /// `PCurveOnFace2(Index)`.
    pub fn pcurve_on_face2(&self, index: i32) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        self.check_index(index, "FilletSurfBuilder::PCurveOnFace2")?;
        self.my_int_build.pcurve_on_face2(index)
    }

    /// `PCurve2OnFillet(Index)`.
    pub fn pcurve2_on_fillet(&self, index: i32) -> Result<Handle<dyn Geom2dCurve>, Failure> {
        self.check_index(index, "FilletSurfBuilder::PCurve2OnFillet")?;
        self.my_int_build.pcurve2_on_fillet(index)
    }

    /// StdFailNotDone when not computed.
    fn check_done(&self, member: &str) -> Result<(), Failure> {
        if self.is_done() == StatusDone::IsNotOk {
            return Err(not_done(member));
        }
        Ok(())
    }

    /// `FirstParameter()`.
    pub fn first_parameter(&self) -> Result<f64, Failure> {
        self.check_done("FilletSurfBuilder::FirstParameter")?;
        self.my_int_build.first_parameter()
    }

    /// `LastParameter()`.
    pub fn last_parameter(&self) -> Result<f64, Failure> {
        self.check_done("FilletSurfBuilder::LastParameter")?;
        self.my_int_build.last_parameter()
    }

    /// `StartSectionStatus()`.
    pub fn start_section_status(&self) -> Result<StatusType, Failure> {
        self.check_done("FilletSurfBuilder::StartSectionStatus")?;
        self.my_int_build.start_section_status()
    }

    /// `EndSectionStatus()` (its native message names StartSectionStatus).
    pub fn end_section_status(&self) -> Result<StatusType, Failure> {
        self.check_done("FilletSurfBuilder::StartSectionStatus")?;
        self.my_int_build.end_section_status()
    }

    /// `Simulate()`: only the sections; IsNotOk when the simulation fails.
    pub fn simulate(&mut self) -> Result<(), Failure> {
        if self.myisdone != StatusDone::IsOk {
            return Ok(());
        }
        self.my_int_build.simulate()?;
        if self.my_int_build.done() {
            self.myisdone = StatusDone::IsOk;
        } else {
            (self.myisdone, self.myerrorstatus) =
                (StatusDone::IsNotOk, ErrorTypeStatus::PbFilletCompute);
        }
        Ok(())
    }

    /// `NbSection(IndexSurf)`.
    pub fn nb_section(&self, index_surf: i32) -> Result<i32, Failure> {
        self.check_done("FilletSurfBuilder::NbSection)")?;
        self.check_index(index_surf, "FilletSurfBuilder::NbSection")?;
        self.my_int_build.nb_section(index_surf)
    }

    /// `Section(IndexSurf, IndexSec)`: the arc of circle of the section.
    pub fn section(
        &self,
        index_surf: i32,
        index_sec: i32,
    ) -> Result<Handle<GeomTrimmedCurve>, Failure> {
        self.check_index(index_surf, "FilletSurfBuilder::Section NbSurface")?;
        if index_sec < 1 || index_sec > self.nb_section(index_surf)? {
            return Err(Failure::out_of_range(
                "FilletSurfBuilder::Section NbSection",
            ));
        }
        self.my_int_build.section(index_surf, index_sec)
    }
}
