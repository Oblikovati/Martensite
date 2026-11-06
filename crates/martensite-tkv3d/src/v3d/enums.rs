//! The V3d enumerations and `V3dImageDumpOptions`, with their native discriminants.
use martensite_tkservice::graphic3d::{
    GRAPHIC3D_ZLAYER_ID_BOT_OSD, Graphic3dBufferType, Graphic3dZLayerId,
};

/// `V3dTypeOfAxe`: the view axes.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum V3dTypeOfAxe {
    /// `V3dX`.
    X = 0,
    /// `V3dY`.
    Y = 1,
    /// `V3dZ`.
    Z = 2,
}

/// `V3dTypeOfView`: orthographic or perspective projection.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum V3dTypeOfView {
    /// `V3dORTHOGRAPHIC`.
    Orthographic = 0,
    /// `V3dPERSPECTIVE`.
    Perspective = 1,
}

/// `V3dTypeOfVisualization`: wireframe or Z-buffer.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum V3dTypeOfVisualization {
    /// `V3dWIREFRAME`.
    Wireframe = 0,
    /// `V3dZBUFFER`.
    Zbuffer = 1,
}

/// `V3dStereoDumpOptions`: which stereo projection a view dump renders.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum V3dStereoDumpOptions {
    /// `V3dSDO_MONO`: the monographic middle projection.
    Mono = 0,
    /// `V3dSDO_LEFT_EYE`.
    LeftEye = 1,
    /// `V3dSDO_RIGHT_EYE`.
    RightEye = 2,
    /// `V3dSDO_BLENDED`: the blended pair of the active stereo mode.
    Blended = 3,
}

/// `V3dTypeOfOrientation`: the predefined view directions.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum V3dTypeOfOrientation {
    Xpos = 0,
    Ypos = 1,
    Zpos = 2,
    Xneg = 3,
    Yneg = 4,
    Zneg = 5,
    XposYpos = 6,
    XposZpos = 7,
    YposZpos = 8,
    XnegYneg = 9,
    XnegYpos = 10,
    XnegZneg = 11,
    XnegZpos = 12,
    YnegZneg = 13,
    YnegZpos = 14,
    XposYneg = 15,
    XposZneg = 16,
    YposZneg = 17,
    XposYposZpos = 18,
    XposYnegZpos = 19,
    XposYposZneg = 20,
    XnegYposZpos = 21,
    XposYnegZneg = 22,
    XnegYposZneg = 23,
    XnegYnegZpos = 24,
    XnegYnegZneg = 25,
}

impl V3dTypeOfOrientation {
    /// Every orientation in native order.
    pub const ALL: [Self; 26] = [
        Self::Xpos,
        Self::Ypos,
        Self::Zpos,
        Self::Xneg,
        Self::Yneg,
        Self::Zneg,
        Self::XposYpos,
        Self::XposZpos,
        Self::YposZpos,
        Self::XnegYneg,
        Self::XnegYpos,
        Self::XnegZneg,
        Self::XnegZpos,
        Self::YnegZneg,
        Self::YnegZpos,
        Self::XposYneg,
        Self::XposZneg,
        Self::YposZneg,
        Self::XposYposZpos,
        Self::XposYnegZpos,
        Self::XposYposZneg,
        Self::XnegYposZpos,
        Self::XposYnegZneg,
        Self::XnegYposZneg,
        Self::XnegYnegZpos,
        Self::XnegYnegZneg,
    ];
    /// `V3dTypeOfOrientation_Zup_AxoLeft`.
    pub const ZUP_AXO_LEFT: Self = Self::XnegYnegZpos;
    /// `V3dTypeOfOrientation_Zup_AxoRight`.
    pub const ZUP_AXO_RIGHT: Self = Self::XposYnegZpos;
    /// `V3dTypeOfOrientation_Zup_Front`.
    pub const ZUP_FRONT: Self = Self::Yneg;
    /// `V3dTypeOfOrientation_Zup_Back`.
    pub const ZUP_BACK: Self = Self::Ypos;
    /// `V3dTypeOfOrientation_Zup_Top`.
    pub const ZUP_TOP: Self = Self::Zpos;
    /// `V3dTypeOfOrientation_Zup_Bottom`.
    pub const ZUP_BOTTOM: Self = Self::Zneg;
    /// `V3dTypeOfOrientation_Zup_Left`.
    pub const ZUP_LEFT: Self = Self::Xneg;
    /// `V3dTypeOfOrientation_Zup_Right`.
    pub const ZUP_RIGHT: Self = Self::Xpos;
    /// `V3dTypeOfOrientation_Yup_AxoLeft`.
    pub const YUP_AXO_LEFT: Self = Self::XnegYposZpos;
    /// `V3dTypeOfOrientation_Yup_AxoRight`.
    pub const YUP_AXO_RIGHT: Self = Self::XposYposZpos;
    /// `V3dTypeOfOrientation_Yup_Front`.
    pub const YUP_FRONT: Self = Self::Zpos;
    /// `V3dTypeOfOrientation_Yup_Back`.
    pub const YUP_BACK: Self = Self::Zneg;
    /// `V3dTypeOfOrientation_Yup_Top`.
    pub const YUP_TOP: Self = Self::Ypos;
    /// `V3dTypeOfOrientation_Yup_Bottom`.
    pub const YUP_BOTTOM: Self = Self::Yneg;
    /// `V3dTypeOfOrientation_Yup_Left`.
    pub const YUP_LEFT: Self = Self::Xpos;
    /// `V3dTypeOfOrientation_Yup_Right`.
    pub const YUP_RIGHT: Self = Self::Xneg;
}

/// `V3dImageDumpOptions`: the parameters of `V3dView::ToPixMap`.
///
/// Example: `let options = V3dImageDumpOptions { width: 64, height: 32, ..Default::default };`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)] // Native public aggregate field order includes padding before the pointer.
pub struct V3dImageDumpOptions<'anchor> {
    /// `Width`: 0 means an already allocated image.
    pub width: i32,
    /// `Height`: 0 means an already allocated image.
    pub height: i32,
    /// `BufferType`.
    pub buffer_type: Graphic3dBufferType,
    /// `StereoOptions`.
    pub stereo_options: V3dStereoDumpOptions,
    /// `TileSize`: 0 for automatic tiling.
    pub tile_size: i32,
    /// `ToAdjustAspect`.
    pub to_adjust_aspect: bool,
    /// `TargetZLayerId`.
    pub target_z_layer_id: Graphic3dZLayerId,
    /// `IsSingleLayer`.
    pub is_single_layer: bool,
    /// `LightName`: the light whose shadow map is dumped.
    pub light_name: super::V3dLightName<'anchor>,
}

impl Default for V3dImageDumpOptions<'_> {
    /// RGB, mono, automatic tiling, aspect adjusted, up to the bottom OSD layer.
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            buffer_type: Graphic3dBufferType::Rgb,
            stereo_options: V3dStereoDumpOptions::Mono,
            tile_size: 0,
            to_adjust_aspect: true,
            target_z_layer_id: GRAPHIC3D_ZLAYER_ID_BOT_OSD,
            is_single_layer: false,
            light_name: super::V3dLightName::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminants_and_aliases() {
        assert_eq!(V3dTypeOfOrientation::XnegYnegZneg as i32, 25);
        assert_eq!(
            V3dTypeOfOrientation::ALL[19],
            V3dTypeOfOrientation::XposYnegZpos
        );
        assert_eq!(V3dTypeOfOrientation::YUP_RIGHT, V3dTypeOfOrientation::Xneg);
        assert_eq!(V3dStereoDumpOptions::Blended as i32, 3);
        let options = V3dImageDumpOptions::default();
        assert!(options.to_adjust_aspect && !options.is_single_layer);
    }
}
