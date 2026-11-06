//! `RwMeshCoordinateSystemConverter`: positions, normals and transformations between two axis
//! conventions and two length units.
use super::RwMeshCoordinateSystem;
use martensite_tkernel::ncollection::{Mat4, Vec3, Vec4};
use martensite_tkmath::gp::{self, Axis3, Quaternion, Transform3, TransformForm, Vector3, Xyz};

/// Converts coordinates; without a defined input and output nothing changes.
///
/// ```
/// use martensite_tkmath::gp::Xyz;
/// use martensite_tkrwmesh::rw_mesh::{RwMeshCoordinateSystem as System, RwMeshCoordinateSystemConverter};
/// let mut converter = RwMeshCoordinateSystemConverter::new();
/// converter.set_input_coordinate_system_kind(System::Y_UP);
/// converter.set_output_coordinate_system_kind(System::Z_UP);
/// let mut point = Xyz::new(1.0, 2.0, 3.0);
/// converter.transform_position(&mut point);
/// assert!((point.z() - 2.0).abs() < 1e-12);
/// ```
#[derive(Clone, Debug)]
pub struct RwMeshCoordinateSystemConverter {
    input_ax3: Axis3,
    output_ax3: Axis3,
    input_length_unit: f64,
    output_length_unit: f64,
    has_input_ax3: bool,
    has_output_ax3: bool,
    trsf: Transform3,
    trsf_inv: Transform3,
    norm_trsf: Mat4<f32>,
    unit_factor: f64,
    has_scale: bool,
    is_empty: bool,
}

impl Default for RwMeshCoordinateSystemConverter {
    fn default() -> Self {
        Self {
            input_ax3: Axis3::default(),
            output_ax3: Axis3::default(),
            input_length_unit: -1.0,
            output_length_unit: -1.0,
            has_input_ax3: false,
            has_output_ax3: false,
            trsf: Transform3::default(),
            trsf_inv: Transform3::default(),
            norm_trsf: Mat4::default(),
            unit_factor: 1.0,
            has_scale: false,
            is_empty: true,
        }
    }
}

impl RwMeshCoordinateSystemConverter {
    /// The frame of a convention, the default frame for an undefined one;
    /// `RwMeshCoordinateSystemConverter::standard_coordinate_system(System::Y_UP)`.
    pub fn standard_coordinate_system(system: RwMeshCoordinateSystem) -> Axis3 {
        let up = match system {
            RwMeshCoordinateSystem::POS_Y_FORWARD_POS_Z_UP => gp::dz(),
            RwMeshCoordinateSystem::NEG_Z_FORWARD_POS_Y_UP => gp::dy(),
            _ => return Axis3::default(),
        };
        Axis3::new(gp::origin(), up, gp::dx()).unwrap_or_default()
    }
    /// Nothing to convert; `RwMeshCoordinateSystemConverter::new()`.
    pub fn new() -> Self {
        Self::default()
    }
    /// True when target and current systems are the same; `converter.is_empty()`.
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
    /// Scale of the source unit to meters, -1 when undefined; `converter.input_length_unit()`.
    pub fn input_length_unit(&self) -> f64 {
        self.input_length_unit
    }
    /// Sets the source unit as a scale to meters; `converter.set_input_length_unit(0.001)`.
    pub fn set_input_length_unit(&mut self, scale: f64) {
        self.init(
            self.input_ax3,
            scale,
            self.output_ax3,
            self.output_length_unit,
        );
    }
    /// Scale of the target unit to meters, -1 when undefined; `converter.output_length_unit()`.
    pub fn output_length_unit(&self) -> f64 {
        self.output_length_unit
    }
    /// Sets the target unit as a scale to meters; `converter.set_output_length_unit(1.0)`.
    pub fn set_output_length_unit(&mut self, scale: f64) {
        self.init(
            self.input_ax3,
            self.input_length_unit,
            self.output_ax3,
            scale,
        );
    }
    /// True once a source system was set; `converter.has_input_coordinate_system()`.
    pub fn has_input_coordinate_system(&self) -> bool {
        self.has_input_ax3
    }
    /// The source system; `converter.input_coordinate_system()`.
    pub fn input_coordinate_system(&self) -> &Axis3 {
        &self.input_ax3
    }
    /// Sets the source frame; `converter.set_input_coordinate_system(frame)`.
    pub fn set_input_coordinate_system(&mut self, system: Axis3) {
        self.has_input_ax3 = true;
        self.init(
            system,
            self.input_length_unit,
            self.output_ax3,
            self.output_length_unit,
        );
    }
    /// Sets the source convention; `converter.set_input_coordinate_system_kind(System::Y_UP)`.
    pub fn set_input_coordinate_system_kind(&mut self, system: RwMeshCoordinateSystem) {
        self.has_input_ax3 = system != RwMeshCoordinateSystem::UNDEFINED;
        let frame = Self::standard_coordinate_system(system);
        self.init(
            frame,
            self.input_length_unit,
            self.output_ax3,
            self.output_length_unit,
        );
    }
    /// True once a target system was set; `converter.has_output_coordinate_system()`.
    pub fn has_output_coordinate_system(&self) -> bool {
        self.has_output_ax3
    }
    /// The target system; `converter.output_coordinate_system()`.
    pub fn output_coordinate_system(&self) -> &Axis3 {
        &self.output_ax3
    }
    /// Sets the target frame; `converter.set_output_coordinate_system(frame)`.
    pub fn set_output_coordinate_system(&mut self, system: Axis3) {
        self.has_output_ax3 = true;
        self.init(
            self.input_ax3,
            self.input_length_unit,
            system,
            self.output_length_unit,
        );
    }
    /// Sets the target convention; `converter.set_output_coordinate_system_kind(System::Z_UP)`.
    pub fn set_output_coordinate_system_kind(&mut self, system: RwMeshCoordinateSystem) {
        self.has_output_ax3 = system != RwMeshCoordinateSystem::UNDEFINED;
        let frame = Self::standard_coordinate_system(system);
        self.init(
            self.input_ax3,
            self.input_length_unit,
            frame,
            self.output_length_unit,
        );
    }
    /// Sets both systems and units and derives the conversion;
    /// `converter.init(from, 0.001, to, 1.0)`.
    pub fn init(&mut self, input: Axis3, input_unit: f64, output: Axis3, output_unit: f64) {
        self.input_length_unit = input_unit;
        self.output_length_unit = output_unit;
        self.input_ax3 = input;
        self.output_ax3 = output;
        self.init_unit_factor(input_unit, output_unit);
        let trsf = self.frame_change(input, output);
        self.trsf = trsf;
        // A frame change is a rigid motion, which always has an inverse.
        self.trsf_inv = trsf.inverted().unwrap_or_default();
        self.norm_trsf = Mat4::default();
        self.trsf.get_mat4(&mut self.norm_trsf);
        self.is_empty = !self.has_scale && self.trsf.form() == TransformForm::Identity;
    }
    fn init_unit_factor(&mut self, input_unit: f64, output_unit: f64) {
        if input_unit > 0.0 && output_unit > 0.0 {
            self.unit_factor = input_unit / output_unit;
            self.has_scale = (self.unit_factor - 1.0).abs() > gp::resolution();
        } else {
            self.unit_factor = 1.0;
            self.has_scale = false;
        }
    }
    fn frame_change(&self, input: Axis3, output: Axis3) -> Transform3 {
        if !(self.has_input_ax3 && self.has_output_ax3) {
            return Transform3::default();
        }
        let trsf = Transform3::transformation_between(output, input);
        let no_translation = trsf
            .translation_part()
            .is_equal(&Xyz::new(0.0, 0.0, 0.0), gp::resolution());
        if no_translation && trsf.rotation_quaternion().is_equal(&Quaternion::default()) {
            return Transform3::default();
        }
        trsf
    }
    /// Changes a transformation to the target system; `converter.transform_transformation(&mut trsf)`.
    pub fn transform_transformation(&self, trsf: &mut Transform3) {
        if self.has_scale {
            let part = trsf.translation_part() * self.unit_factor;
            trsf.set_translation_part(Vector3::new(part.x(), part.y(), part.z()));
        }
        if self.trsf.form() != TransformForm::Identity {
            *trsf = self.trsf.multiplied(*trsf).multiplied(self.trsf_inv);
        }
    }
    /// Changes a position to the target system; `converter.transform_position(&mut point)`.
    pub fn transform_position(&self, position: &mut Xyz) {
        if self.has_scale {
            *position *= self.unit_factor;
        }
        if self.trsf.form() != TransformForm::Identity {
            self.trsf.transforms(position);
        }
    }
    /// Changes a normal, ignoring translation and scale; `converter.transform_normal(&mut normal)`.
    pub fn transform_normal(&self, normal: &mut Vec3<f32>) {
        if self.trsf.form() != TransformForm::Identity {
            let turned = self.norm_trsf * Vec4::new(normal.x, normal.y, normal.z, 0.0);
            *normal = Vec3::new(turned.x, turned.y, turned.z);
        }
    }
}
