//! `RasterColorKind`, `RasterColor` and `RasterMarkerShape`.

/// The fifteen colors of the command host's palette (`RasterColorKind`, French names as reference).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(i32)]
pub enum PaletteKind {
    /// `Rasterblanc`: white (index 0).
    #[default]
    Blanc,
    /// `Rasterrouge`: red.
    Rouge,
    /// `Rastervert`: green.
    Vert,
    /// `Rasterbleu`: blue.
    Bleu,
    /// `Rastercyan`: cyan.
    Cyan,
    /// `Rasteror`: gold.
    Or,
    /// `Rastermagenta`: magenta.
    Magenta,
    /// `Rastermarron`: maroon.
    Marron,
    /// `Rasterorange`: orange.
    Orange,
    /// `Rasterrose`: pink.
    Rose,
    /// `Rastersaumon`: salmon.
    Saumon,
    /// `Rasterviolet`: violet.
    Violet,
    /// `Rasterjaune`: yellow.
    Jaune,
    /// `Rasterkaki`: khaki.
    Kaki,
    /// `Rastercorail`: coral (index 14).
    Corail,
}

pub use crate::{COLOR_NAMES, MAXCOLOR};

impl PaletteKind {
    /// Every kind, in index order.
    pub const ALL: [Self; MAXCOLOR] = [
        Self::Blanc,
        Self::Rouge,
        Self::Vert,
        Self::Bleu,
        Self::Cyan,
        Self::Or,
        Self::Magenta,
        Self::Marron,
        Self::Orange,
        Self::Rose,
        Self::Saumon,
        Self::Violet,
        Self::Jaune,
        Self::Kaki,
        Self::Corail,
    ];

    /// The palette index (the C++ enum value).
    pub fn index(self) -> usize {
        self as usize
    }

    /// The kind with palette index `index`; `None` outside `0..MAXCOLOR`.
    ///
    /// ```
    /// use martensite_raster_core::PaletteKind;
    /// assert_eq!(PaletteKind::from_index(3), Some(PaletteKind::Bleu));
    /// assert_eq!(PaletteKind::from_index(15), None);
    /// ```
    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }
}

/// `RasterColor`: a palette color (white by default).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PaletteColor {
    kind: PaletteKind,
}

impl PaletteColor {
    /// `RasterColor(RasterColorKind)`.
    ///
    /// ```
    /// use martensite_raster_core::{PaletteColor, PaletteKind};
    /// assert_eq!(PaletteColor::new(PaletteKind::Vert).id(), PaletteKind::Vert);
    /// assert_eq!(PaletteColor::default().id(), PaletteKind::Blanc);
    /// ```
    pub fn new(kind: PaletteKind) -> Self {
        Self { kind }
    }

    /// `ID`: the palette entry.
    pub fn id(self) -> PaletteKind {
        self.kind
    }
}

/// `RasterMarkerShape`. `Circle` keeps its screen size under zoom; `CircleZoom` scales.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkerShape {
    /// `RasterSquare`.
    Square,
    /// `RasterLosange`: a diamond.
    Losange,
    /// `RasterX`.
    X,
    /// `RasterPlus`.
    Plus,
    /// `RasterCircle`.
    Circle,
    /// `RasterCircleZoom`.
    CircleZoom,
}
