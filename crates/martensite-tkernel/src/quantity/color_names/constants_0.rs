// Named-color table data: entry order is the `NamedColor` index, so keep entries in place.

impl NamedColor {
    /// Named color BLACK; `NamedColor::BLACK.name()` identifies its canonical entry.
    pub const BLACK: Self = Self(0);
    /// Named color MATRABLUE; `NamedColor::MATRABLUE.name()` identifies its canonical entry.
    pub const MATRABLUE: Self = Self(1);
    /// Named color MATRAGRAY; `NamedColor::MATRAGRAY.name()` identifies its canonical entry.
    pub const MATRAGRAY: Self = Self(2);
    /// Named color ALICEBLUE; `NamedColor::ALICEBLUE.name()` identifies its canonical entry.
    pub const ALICEBLUE: Self = Self(3);
    /// Named color ANTIQUEWHITE; `NamedColor::ANTIQUEWHITE.name()` identifies its canonical entry.
    pub const ANTIQUEWHITE: Self = Self(4);
    /// Named color ANTIQUEWHITE1; `NamedColor::ANTIQUEWHITE1.name()` identifies its canonical entry.
    pub const ANTIQUEWHITE1: Self = Self(5);
    /// Named color ANTIQUEWHITE2; `NamedColor::ANTIQUEWHITE2.name()` identifies its canonical entry.
    pub const ANTIQUEWHITE2: Self = Self(6);
    /// Named color ANTIQUEWHITE3; `NamedColor::ANTIQUEWHITE3.name()` identifies its canonical entry.
    pub const ANTIQUEWHITE3: Self = Self(7);
    /// Named color ANTIQUEWHITE4; `NamedColor::ANTIQUEWHITE4.name()` identifies its canonical entry.
    pub const ANTIQUEWHITE4: Self = Self(8);
    /// Named color AQUAMARINE1; `NamedColor::AQUAMARINE1.name()` identifies its canonical entry.
    pub const AQUAMARINE1: Self = Self(9);
    /// Named color AQUAMARINE2; `NamedColor::AQUAMARINE2.name()` identifies its canonical entry.
    pub const AQUAMARINE2: Self = Self(10);
    /// Named color AQUAMARINE4; `NamedColor::AQUAMARINE4.name()` identifies its canonical entry.
    pub const AQUAMARINE4: Self = Self(11);
    /// Named color AZURE; `NamedColor::AZURE.name()` identifies its canonical entry.
    pub const AZURE: Self = Self(12);
    /// Named color AZURE2; `NamedColor::AZURE2.name()` identifies its canonical entry.
    pub const AZURE2: Self = Self(13);
    /// Named color AZURE3; `NamedColor::AZURE3.name()` identifies its canonical entry.
    pub const AZURE3: Self = Self(14);
    /// Named color AZURE4; `NamedColor::AZURE4.name()` identifies its canonical entry.
    pub const AZURE4: Self = Self(15);
    /// Named color BEIGE; `NamedColor::BEIGE.name()` identifies its canonical entry.
    pub const BEIGE: Self = Self(16);
    /// Named color BISQUE; `NamedColor::BISQUE.name()` identifies its canonical entry.
    pub const BISQUE: Self = Self(17);
    /// Named color BISQUE2; `NamedColor::BISQUE2.name()` identifies its canonical entry.
    pub const BISQUE2: Self = Self(18);
    /// Named color BISQUE3; `NamedColor::BISQUE3.name()` identifies its canonical entry.
    pub const BISQUE3: Self = Self(19);
    /// Named color BISQUE4; `NamedColor::BISQUE4.name()` identifies its canonical entry.
    pub const BISQUE4: Self = Self(20);
    /// Named color BLANCHEDALMOND; `NamedColor::BLANCHEDALMOND.name()` identifies its canonical entry.
    pub const BLANCHEDALMOND: Self = Self(21);
    /// Named color BLUE; `NamedColor::BLUE.name()` identifies its canonical entry.
    pub const BLUE: Self = Self(22);
    /// Named color BLUE2; `NamedColor::BLUE2.name()` identifies its canonical entry.
    pub const BLUE2: Self = Self(23);
    /// Named color BLUE3; `NamedColor::BLUE3.name()` identifies its canonical entry.
    pub const BLUE3: Self = Self(24);
    /// Named color BLUE4; `NamedColor::BLUE4.name()` identifies its canonical entry.
    pub const BLUE4: Self = Self(25);
    /// Named color BLUEVIOLET; `NamedColor::BLUEVIOLET.name()` identifies its canonical entry.
    pub const BLUEVIOLET: Self = Self(26);
    /// Named color BROWN; `NamedColor::BROWN.name()` identifies its canonical entry.
    pub const BROWN: Self = Self(27);
    /// Named color BROWN1; `NamedColor::BROWN1.name()` identifies its canonical entry.
    pub const BROWN1: Self = Self(28);
    /// Named color BROWN2; `NamedColor::BROWN2.name()` identifies its canonical entry.
    pub const BROWN2: Self = Self(29);
    /// Named color BROWN3; `NamedColor::BROWN3.name()` identifies its canonical entry.
    pub const BROWN3: Self = Self(30);
    /// Named color BROWN4; `NamedColor::BROWN4.name()` identifies its canonical entry.
    pub const BROWN4: Self = Self(31);
    /// Named color BURLYWOOD; `NamedColor::BURLYWOOD.name()` identifies its canonical entry.
    pub const BURLYWOOD: Self = Self(32);
    /// Named color BURLYWOOD1; `NamedColor::BURLYWOOD1.name()` identifies its canonical entry.
    pub const BURLYWOOD1: Self = Self(33);
    /// Named color BURLYWOOD2; `NamedColor::BURLYWOOD2.name()` identifies its canonical entry.
    pub const BURLYWOOD2: Self = Self(34);
    /// Named color BURLYWOOD3; `NamedColor::BURLYWOOD3.name()` identifies its canonical entry.
    pub const BURLYWOOD3: Self = Self(35);
    /// Named color BURLYWOOD4; `NamedColor::BURLYWOOD4.name()` identifies its canonical entry.
    pub const BURLYWOOD4: Self = Self(36);
    /// Named color CADETBLUE; `NamedColor::CADETBLUE.name()` identifies its canonical entry.
    pub const CADETBLUE: Self = Self(37);
    /// Named color CADETBLUE1; `NamedColor::CADETBLUE1.name()` identifies its canonical entry.
    pub const CADETBLUE1: Self = Self(38);
    /// Named color CADETBLUE2; `NamedColor::CADETBLUE2.name()` identifies its canonical entry.
    pub const CADETBLUE2: Self = Self(39);
    /// Named color CADETBLUE3; `NamedColor::CADETBLUE3.name()` identifies its canonical entry.
    pub const CADETBLUE3: Self = Self(40);
    /// Named color CADETBLUE4; `NamedColor::CADETBLUE4.name()` identifies its canonical entry.
    pub const CADETBLUE4: Self = Self(41);
    /// Named color CHARTREUSE; `NamedColor::CHARTREUSE.name()` identifies its canonical entry.
    pub const CHARTREUSE: Self = Self(42);
    /// Named color CHARTREUSE2; `NamedColor::CHARTREUSE2.name()` identifies its canonical entry.
    pub const CHARTREUSE2: Self = Self(43);
    /// Named color CHARTREUSE3; `NamedColor::CHARTREUSE3.name()` identifies its canonical entry.
    pub const CHARTREUSE3: Self = Self(44);
    /// Named color CHARTREUSE4; `NamedColor::CHARTREUSE4.name()` identifies its canonical entry.
    pub const CHARTREUSE4: Self = Self(45);
    /// Named color CHOCOLATE; `NamedColor::CHOCOLATE.name()` identifies its canonical entry.
    pub const CHOCOLATE: Self = Self(46);
    /// Named color CHOCOLATE1; `NamedColor::CHOCOLATE1.name()` identifies its canonical entry.
    pub const CHOCOLATE1: Self = Self(47);
    /// Named color CHOCOLATE2; `NamedColor::CHOCOLATE2.name()` identifies its canonical entry.
    pub const CHOCOLATE2: Self = Self(48);
    /// Named color CHOCOLATE3; `NamedColor::CHOCOLATE3.name()` identifies its canonical entry.
    pub const CHOCOLATE3: Self = Self(49);
    /// Named color CHOCOLATE4; `NamedColor::CHOCOLATE4.name()` identifies its canonical entry.
    pub const CHOCOLATE4: Self = Self(50);
    /// Named color CORAL; `NamedColor::CORAL.name()` identifies its canonical entry.
    pub const CORAL: Self = Self(51);
    /// Named color CORAL1; `NamedColor::CORAL1.name()` identifies its canonical entry.
    pub const CORAL1: Self = Self(52);
    /// Named color CORAL2; `NamedColor::CORAL2.name()` identifies its canonical entry.
    pub const CORAL2: Self = Self(53);
    /// Named color CORAL3; `NamedColor::CORAL3.name()` identifies its canonical entry.
    pub const CORAL3: Self = Self(54);
    /// Named color CORAL4; `NamedColor::CORAL4.name()` identifies its canonical entry.
    pub const CORAL4: Self = Self(55);
    /// Named color CORNFLOWERBLUE; `NamedColor::CORNFLOWERBLUE.name()` identifies its canonical entry.
    pub const CORNFLOWERBLUE: Self = Self(56);
    /// Named color CORNSILK1; `NamedColor::CORNSILK1.name()` identifies its canonical entry.
    pub const CORNSILK1: Self = Self(57);
    /// Named color CORNSILK2; `NamedColor::CORNSILK2.name()` identifies its canonical entry.
    pub const CORNSILK2: Self = Self(58);
    /// Named color CORNSILK3; `NamedColor::CORNSILK3.name()` identifies its canonical entry.
    pub const CORNSILK3: Self = Self(59);
    /// Named color CORNSILK4; `NamedColor::CORNSILK4.name()` identifies its canonical entry.
    pub const CORNSILK4: Self = Self(60);
    /// Named color CYAN; `NamedColor::CYAN.name()` identifies its canonical entry.
    pub const CYAN: Self = Self(61);
    /// Named color CYAN2; `NamedColor::CYAN2.name()` identifies its canonical entry.
    pub const CYAN2: Self = Self(62);
    /// Named color CYAN3; `NamedColor::CYAN3.name()` identifies its canonical entry.
    pub const CYAN3: Self = Self(63);
    /// Named color CYAN4; `NamedColor::CYAN4.name()` identifies its canonical entry.
    pub const CYAN4: Self = Self(64);
    /// Named color DARKGOLDENROD; `NamedColor::DARKGOLDENROD.name()` identifies its canonical entry.
    pub const DARKGOLDENROD: Self = Self(65);
    /// Named color DARKGOLDENROD1; `NamedColor::DARKGOLDENROD1.name()` identifies its canonical entry.
    pub const DARKGOLDENROD1: Self = Self(66);
    /// Named color DARKGOLDENROD2; `NamedColor::DARKGOLDENROD2.name()` identifies its canonical entry.
    pub const DARKGOLDENROD2: Self = Self(67);
    /// Named color DARKGOLDENROD3; `NamedColor::DARKGOLDENROD3.name()` identifies its canonical entry.
    pub const DARKGOLDENROD3: Self = Self(68);
    /// Named color DARKGOLDENROD4; `NamedColor::DARKGOLDENROD4.name()` identifies its canonical entry.
    pub const DARKGOLDENROD4: Self = Self(69);
    /// Named color DARKGREEN; `NamedColor::DARKGREEN.name()` identifies its canonical entry.
    pub const DARKGREEN: Self = Self(70);
    /// Named color DARKKHAKI; `NamedColor::DARKKHAKI.name()` identifies its canonical entry.
    pub const DARKKHAKI: Self = Self(71);
    /// Named color DARKOLIVEGREEN; `NamedColor::DARKOLIVEGREEN.name()` identifies its canonical entry.
    pub const DARKOLIVEGREEN: Self = Self(72);
    /// Named color DARKOLIVEGREEN1; `NamedColor::DARKOLIVEGREEN1.name()` identifies its canonical entry.
    pub const DARKOLIVEGREEN1: Self = Self(73);
    /// Named color DARKOLIVEGREEN2; `NamedColor::DARKOLIVEGREEN2.name()` identifies its canonical entry.
    pub const DARKOLIVEGREEN2: Self = Self(74);
    /// Named color DARKOLIVEGREEN3; `NamedColor::DARKOLIVEGREEN3.name()` identifies its canonical entry.
    pub const DARKOLIVEGREEN3: Self = Self(75);
    /// Named color DARKOLIVEGREEN4; `NamedColor::DARKOLIVEGREEN4.name()` identifies its canonical entry.
    pub const DARKOLIVEGREEN4: Self = Self(76);
    /// Named color DARKORANGE; `NamedColor::DARKORANGE.name()` identifies its canonical entry.
    pub const DARKORANGE: Self = Self(77);
    /// Named color DARKORANGE1; `NamedColor::DARKORANGE1.name()` identifies its canonical entry.
    pub const DARKORANGE1: Self = Self(78);
    /// Named color DARKORANGE2; `NamedColor::DARKORANGE2.name()` identifies its canonical entry.
    pub const DARKORANGE2: Self = Self(79);
    /// Named color DARKORANGE3; `NamedColor::DARKORANGE3.name()` identifies its canonical entry.
    pub const DARKORANGE3: Self = Self(80);
    /// Named color DARKORANGE4; `NamedColor::DARKORANGE4.name()` identifies its canonical entry.
    pub const DARKORANGE4: Self = Self(81);
    /// Named color DARKORCHID; `NamedColor::DARKORCHID.name()` identifies its canonical entry.
    pub const DARKORCHID: Self = Self(82);
    /// Named color DARKORCHID1; `NamedColor::DARKORCHID1.name()` identifies its canonical entry.
    pub const DARKORCHID1: Self = Self(83);
    /// Named color DARKORCHID2; `NamedColor::DARKORCHID2.name()` identifies its canonical entry.
    pub const DARKORCHID2: Self = Self(84);
    /// Named color DARKORCHID3; `NamedColor::DARKORCHID3.name()` identifies its canonical entry.
    pub const DARKORCHID3: Self = Self(85);
    /// Named color DARKORCHID4; `NamedColor::DARKORCHID4.name()` identifies its canonical entry.
    pub const DARKORCHID4: Self = Self(86);
    /// Named color DARKSALMON; `NamedColor::DARKSALMON.name()` identifies its canonical entry.
    pub const DARKSALMON: Self = Self(87);
    /// Named color DARKSEAGREEN; `NamedColor::DARKSEAGREEN.name()` identifies its canonical entry.
    pub const DARKSEAGREEN: Self = Self(88);
    /// Named color DARKSEAGREEN1; `NamedColor::DARKSEAGREEN1.name()` identifies its canonical entry.
    pub const DARKSEAGREEN1: Self = Self(89);
    /// Named color DARKSEAGREEN2; `NamedColor::DARKSEAGREEN2.name()` identifies its canonical entry.
    pub const DARKSEAGREEN2: Self = Self(90);
    /// Named color DARKSEAGREEN3; `NamedColor::DARKSEAGREEN3.name()` identifies its canonical entry.
    pub const DARKSEAGREEN3: Self = Self(91);
    /// Named color DARKSEAGREEN4; `NamedColor::DARKSEAGREEN4.name()` identifies its canonical entry.
    pub const DARKSEAGREEN4: Self = Self(92);
    /// Named color DARKSLATEBLUE; `NamedColor::DARKSLATEBLUE.name()` identifies its canonical entry.
    pub const DARKSLATEBLUE: Self = Self(93);
    /// Named color DARKSLATEGRAY1; `NamedColor::DARKSLATEGRAY1.name()` identifies its canonical entry.
    pub const DARKSLATEGRAY1: Self = Self(94);
    /// Named color DARKSLATEGRAY2; `NamedColor::DARKSLATEGRAY2.name()` identifies its canonical entry.
    pub const DARKSLATEGRAY2: Self = Self(95);
    /// Named color DARKSLATEGRAY3; `NamedColor::DARKSLATEGRAY3.name()` identifies its canonical entry.
    pub const DARKSLATEGRAY3: Self = Self(96);
    /// Named color DARKSLATEGRAY4; `NamedColor::DARKSLATEGRAY4.name()` identifies its canonical entry.
    pub const DARKSLATEGRAY4: Self = Self(97);
    /// Named color DARKSLATEGRAY; `NamedColor::DARKSLATEGRAY.name()` identifies its canonical entry.
    pub const DARKSLATEGRAY: Self = Self(98);
    /// Named color DARKTURQUOISE; `NamedColor::DARKTURQUOISE.name()` identifies its canonical entry.
    pub const DARKTURQUOISE: Self = Self(99);
}
