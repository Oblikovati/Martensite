//! Camera values read from the current view at the same dump observation as submitted draws.
use super::Graphic3dRenderView;
use martensite_tkservice::graphic3d::Graphic3dDataStructureManager;
use std::io;

#[derive(Debug)]
pub(super) struct CameraStateSnapshot {
    pub eye_f64: [u64; 3],
    pub direction_f64: [u64; 3],
    pub up_f64: [u64; 3],
    pub axial_scale_f64: [u64; 3],
    pub scale_f64: u64,
    pub near_f64: u64,
    pub far_f64: u64,
    pub orientation_f32: [[u32; 4]; 4],
    pub projection_f32: [[u32; 4]; 4],
}

pub(super) fn snapshot(view: &Graphic3dRenderView) -> io::Result<CameraStateSnapshot> {
    let camera_handle = view.camera();
    let camera = camera_handle
        .get()
        .ok_or_else(|| io::Error::other("frame camera; expected a live camera handle"))?;
    let eye = camera.eye();
    let direction = camera.direction();
    let up = camera.up();
    let axial = camera.axial_scale();
    Ok(CameraStateSnapshot {
        eye_f64: [eye.x().to_bits(), eye.y().to_bits(), eye.z().to_bits()],
        direction_f64: [
            direction.x().to_bits(),
            direction.y().to_bits(),
            direction.z().to_bits(),
        ],
        up_f64: [up.x().to_bits(), up.y().to_bits(), up.z().to_bits()],
        axial_scale_f64: [
            axial.x().to_bits(),
            axial.y().to_bits(),
            axial.z().to_bits(),
        ],
        scale_f64: camera.scale().to_bits(),
        near_f64: camera.z_near().to_bits(),
        far_f64: camera.z_far().to_bits(),
        orientation_f32: matrix_bits(camera.orientation_matrix_f()),
        projection_f32: matrix_bits(camera.projection_matrix_f()),
    })
}

fn matrix_bits(matrix: martensite_tkernel::ncollection::Mat4<f32>) -> [[u32; 4]; 4] {
    std::array::from_fn(|row| std::array::from_fn(|column| matrix.get_value(row, column).to_bits()))
}

pub(super) fn encode(snapshot: &CameraStateSnapshot) -> Vec<u8> {
    format!(
        "schema=martensite-original-camera-bits-v1\nfloat64_words=ieee754-hex\nfloat32_words=ieee754-hex,row-major\neye={:016x?}\ndirection={:016x?}\nup={:016x?}\naxial_scale={:016x?}\nscale={:016x}\nnear={:016x}\nfar={:016x}\norientation_f32={:08x?}\nprojection_f32={:08x?}\n",
        snapshot.eye_f64,
        snapshot.direction_f64,
        snapshot.up_f64,
        snapshot.axial_scale_f64,
        snapshot.scale_f64,
        snapshot.near_f64,
        snapshot.far_f64,
        snapshot.orientation_f32,
        snapshot.projection_f32,
    )
    .into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_snapshot_encoder_preserves_raw_ieee_words_and_matrix_order() {
        let value = CameraStateSnapshot {
            eye_f64: [1.0_f64.to_bits(), (-0.0_f64).to_bits(), 3.0_f64.to_bits()],
            direction_f64: [4.0_f64.to_bits(), 5.0_f64.to_bits(), 6.0_f64.to_bits()],
            up_f64: [7.0_f64.to_bits(), 8.0_f64.to_bits(), 9.0_f64.to_bits()],
            axial_scale_f64: [10.0_f64.to_bits(), 11.0_f64.to_bits(), 12.0_f64.to_bits()],
            scale_f64: 13.0_f64.to_bits(),
            near_f64: 14.0_f64.to_bits(),
            far_f64: 15.0_f64.to_bits(),
            orientation_f32: [
                [1, 2, 3, 4],
                [5, 6, 7, 8],
                [9, 10, 11, 12],
                [13, 14, 15, 16],
            ],
            projection_f32: [[16; 4]; 4],
        };
        let encoded = String::from_utf8(encode(&value)).unwrap();
        assert!(encoded.contains("float64_words=ieee754-hex"));
        assert!(encoded.contains("eye=[3ff0000000000000, 8000000000000000, 4008000000000000]"));
        assert!(encoded.contains("orientation_f32=[[00000001, 00000002, 00000003, 00000004]"));
        assert!(encoded.contains("projection_f32=[[00000010, 00000010, 00000010, 00000010]"));
    }
}
