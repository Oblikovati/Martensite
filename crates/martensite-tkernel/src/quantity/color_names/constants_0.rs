// Named-color table data: entry order is the `NamedColor` index, so keep entries in place.

impl NamedColor {
    /// Named color BLACK; `NamedColor::BLACK.name()` identifies its canonical entry.
    pub const BLACK: Self = Self(0);
    /// Named color MATRABLUE; `NamedColor::MATRABLUE.name()` identifies its canonical entry.
    pub const MATRABLUE: Self = Self(1);
    /// Named color MATRAGRAY; `NamedColor::MATRAGRAY.name()` identifies its canonical entry.
    pub const MATRAGRAY: Self = Self(2);
    /// Named color AQUAMARINE1; `NamedColor::AQUAMARINE1.name()` identifies its canonical entry.
    pub const AQUAMARINE1: Self = Self(9);
    /// Named color AZURE; `NamedColor::AZURE.name()` identifies its canonical entry.
    pub const AZURE: Self = Self(12);
    /// Named color BLUE; `NamedColor::BLUE.name()` identifies its canonical entry.
    pub const BLUE: Self = Self(22);
    /// Named color BLUE3; `NamedColor::BLUE3.name()` identifies its canonical entry.
    pub const BLUE3: Self = Self(24);
    /// Named color BLUE4; `NamedColor::BLUE4.name()` identifies its canonical entry.
    pub const BLUE4: Self = Self(25);
    /// Named color BROWN; `NamedColor::BROWN.name()` identifies its canonical entry.
    pub const BROWN: Self = Self(27);
    /// Named color BROWN3; `NamedColor::BROWN3.name()` identifies its canonical entry.
    pub const BROWN3: Self = Self(30);
    /// Named color CHARTREUSE; `NamedColor::CHARTREUSE.name()` identifies its canonical entry.
    pub const CHARTREUSE: Self = Self(42);
    /// Named color CHARTREUSE2; `NamedColor::CHARTREUSE2.name()` identifies its canonical entry.
    pub const CHARTREUSE2: Self = Self(43);
    /// Named color CYAN; `NamedColor::CYAN.name()` identifies its canonical entry.
    pub const CYAN: Self = Self(61);
    /// Named color CYAN3; `NamedColor::CYAN3.name()` identifies its canonical entry.
    pub const CYAN3: Self = Self(63);
    /// Named color DARKGOLDENROD; `NamedColor::DARKGOLDENROD.name()` identifies its canonical entry.
    pub const DARKGOLDENROD: Self = Self(65);
    /// Named color DARKGREEN; `NamedColor::DARKGREEN.name()` identifies its canonical entry.
    pub const DARKGREEN: Self = Self(70);
    /// Named color DARKOLIVEGREEN; `NamedColor::DARKOLIVEGREEN.name()` identifies its canonical entry.
    pub const DARKOLIVEGREEN: Self = Self(72);
}
