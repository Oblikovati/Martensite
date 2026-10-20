//! Font_FontMgr, Font_FTFont and Font_TextFormatter.
use super::common::{Reader, end, flag, int, real, text, xy};
use super::print::Output;
use super::print::show;
use super::tried;
use martensite_tkernel::message::Message;
use martensite_tkernel::ncollection::String8;
use martensite_tkernel::tcollection::AsciiString;
use martensite_tkservice::font::{
    FontFTFont, FontFTFontParams, FontFontAspect, FontFontMgr, FontGlyphOutline, FontHinting,
    FontRect, FontStrictLevel, FontTextFormatter, FontTextFormatterIterationFilter,
    FontTextFormatterIterator, FontUnicodeSubset,
};
use martensite_tkservice::graphic3d::{
    Graphic3dHorizontalTextAlignment, Graphic3dVerticalTextAlignment,
};
use martensite_tkservice::image::ImagePixMap;

const NAMES: [&str; 12] = [
    "sans-serif",
    "monospace",
    "serif",
    "DejaVu Sans",
    "Courier",
    "cjk",
    "korean",
    "no-such-font",
    "Times-Bold",
    "arial",
    "",
    "",
];

fn quiet() {
    Message::default_messenger().printers_mut().clear();
}

fn aspect(value: i32) -> FontFontAspect {
    use FontFontAspect as A;
    [A::Undefined, A::Regular, A::Bold, A::Italic, A::BoldItalic][(value + 1) as usize]
}
fn strict(value: i32) -> FontStrictLevel {
    [
        FontStrictLevel::Strict,
        FontStrictLevel::Aliases,
        FontStrictLevel::Any,
    ][value as usize]
}
fn align_x(value: i32) -> Graphic3dHorizontalTextAlignment {
    use Graphic3dHorizontalTextAlignment as H;
    [H::Left, H::Center, H::Right][value as usize]
}
fn align_y(value: i32) -> Graphic3dVerticalTextAlignment {
    use Graphic3dVerticalTextAlignment as V;
    [V::Bottom, V::Center, V::Top, V::Topfirstline][value as usize]
}

/// `N (C)*`: the UTF-32 code points as a UTF-8 string.
fn read_text(r: &mut Reader<'_>) -> (String8, Vec<u32>) {
    let chars: Vec<u32> = (0..r.integer()).map(|_| r.integer() as u32).collect();
    let mut units = chars.clone();
    units.push(0);
    (String8::from_unicode_units(&units, -1), chars)
}

fn rect(output: &Output, b: &FontRect) {
    [b.left, b.right, b.top, b.bottom]
        .map(f64::from)
        .into_iter()
        .for_each(|value| real(output, value));
}

/// The size, format and FNV-1a hash of the image pixels, as `sv_font_image`.
fn image(output: &Output, img: &ImagePixMap) {
    let (w, h) = (img.width(), img.height());
    int(output, w as i32);
    int(output, h as i32);
    int(output, img.format() as i32);
    flag(output, img.is_top_down());
    let row_bytes = w * img.size_pixel_bytes();
    let data = img.data();
    let mut hash: u32 = 2_166_136_261;
    for y in 0..h {
        for &b in data.data.row(y).map_or(&[][..], |row| &row[..row_bytes]) {
            hash = (hash ^ u32::from(b)).wrapping_mul(16_777_619);
        }
    }
    int(output, hash as i32);
}

fn find_fonts(output: &Output, manager: &FontFontMgr) {
    for name in NAMES {
        let name = AsciiString::from(name);
        for level in 0..3 {
            for a in -1..4 {
                let mut found_aspect = aspect(a);
                let font = manager.find_font(&name, strict(level), &mut found_aspect, true);
                flag(output, font.is_some());
                if let Some(font) = font.get() {
                    text(output, font.font_name().as_bytes());
                }
                int(output, found_aspect as i32);
            }
        }
        for subset in [
            FontUnicodeSubset::Western,
            FontUnicodeSubset::Korean,
            FontUnicodeSubset::Cjk,
            FontUnicodeSubset::Arabic,
        ] {
            let font = manager.find_fallback_font(subset, FontFontAspect::Regular);
            flag(output, font.is_some());
            if let Some(font) = font.get() {
                text(output, font.font_name().as_bytes());
            }
        }
    }
}

/// `sv_fontmgr`: the font database, the aliases and the font lookups.
pub fn fontmgr_case(output: &Output, _values: &[f64]) {
    quiet();
    let manager = FontFontMgr::get_instance();
    let fonts = manager.get_available_fonts();
    int(output, fonts.len() as i32);
    for font in &fonts {
        text(output, font.to_string_desc().as_bytes());
        flag(output, font.is_single_stroke_font());
    }
    let aliases = manager.get_all_aliases();
    int(output, aliases.len() as i32);
    for alias in &aliases {
        text(output, alias.string().as_bytes());
        let names = manager.get_font_aliases(&alias.string());
        int(output, names.len() as i32);
        names
            .iter()
            .for_each(|n| text(output, n.string().as_bytes()));
    }
    find_fonts(output, &manager);
    end(output);
}

fn metrics(output: &Output, f: &FontFTFont) {
    flag(output, f.is_valid());
    [f.ascender(), f.descender(), f.line_spacing()]
        .map(f64::from)
        .into_iter()
        .for_each(|value| real(output, value));
    int(output, f.glyph_max_size_x(false) as i32);
    int(output, f.glyph_max_size_y(false) as i32);
    int(output, f.glyphs_number(false));
    int(output, f.point_size() as i32);
    flag(output, f.to_synthesize_italic());
    flag(output, f.is_single_stroke_font());
    real(output, f64::from(f.width_scaling()));
}

fn outline(output: &Output, o: &Option<FontGlyphOutline>) {
    flag(output, o.is_some());
    let Some(o) = o else { return };
    real(output, o.units_per_em());
    int(output, o.rule() as i32);
    flag(output, o.has_possible_overlaps());
    flag(output, o.is_single_stroke());
    int(output, o.segments().len() as i32);
    int(output, o.contours().len() as i32);
    for s in o.segments() {
        int(output, s.kind() as i32);
        [s.start(), s.control1(), s.control2(), s.end()]
            .into_iter()
            .for_each(|value| xy(output, value));
    }
    for c in o.contours() {
        int(output, c.first_segment() as i32);
        int(output, c.nb_segments() as i32);
    }
}

fn character(output: &Output, f: &FontFTFont, c: u32, next: u32) {
    flag(output, f.has_symbol(c));
    flag(output, f.render_glyph(c));
    let mut bounds = FontRect::default();
    f.glyph_rect(&mut bounds);
    rect(output, &bounds);
    image(output, f.glyph_image());
    real(output, f64::from(f.advance_x_of(c, next)));
    real(output, f64::from(f.advance_y_of(c, next)));
    let glyph = f.resolve_glyph(c);
    flag(output, glyph.is_some());
    if let Some(glyph) = &glyph {
        int(output, glyph.face() as i32);
        int(output, glyph.index() as i32);
        flag(output, f.is_glyph_valid(glyph));
    }
    outline(output, &f.load_glyph_outline_char(c));
}

fn read_params(r: &mut Reader<'_>) -> FontFTFontParams {
    let (point_size, resolution, hinting) = (
        r.integer() as u32,
        r.integer() as u32,
        FontHinting(r.integer()),
    );
    let (to_synthesize_italic, is_single_stroke_font) = (r.flag(), r.flag());
    FontFTFontParams {
        point_size,
        resolution,
        font_hinting: hinting,
        to_synthesize_italic,
        is_single_stroke_font,
    }
}

/// The embedded fallback font, or null.
fn embedded_font(params: &FontFTFontParams) -> martensite_tkernel::standard::Handle<FontFTFont> {
    let font = FontFTFont::default();
    let name = AsciiString::from("Embedded fallback");
    if font.init_from(&FontFontMgr::embed_fallback_font(), &name, params, 0) {
        martensite_tkernel::standard::Handle::new(font)
    } else {
        martensite_tkernel::standard::Handle::default()
    }
}

/// `sv_ftfont`: a font found by name (-1: the embedded fallback font), its metrics, characters and bounding boxes.
pub fn ftfont_case(output: &Output, values: &[f64]) {
    quiet();
    let mut r = Reader::new(values);
    let (name, a, level) = (r.integer(), r.integer(), r.integer());
    let params = read_params(&mut r);
    let (width, fallback) = (r.real() as f32, r.flag());
    let (text_value, chars) = read_text(&mut r);
    let font = if name < 0 {
        embedded_font(&params)
    } else {
        FontFTFont::find_and_create(
            &AsciiString::from(NAMES[name as usize]),
            aspect(a),
            &params,
            strict(level),
        )
    };
    flag(output, font.is_some());
    let Some(f) = font.get() else {
        return end(output);
    };
    flag(output, f.set_width_scaling(width));
    f.set_use_unicode_subset_fallback(fallback);
    metrics(output, f);
    for (i, &c) in chars.iter().enumerate() {
        character(output, f, c, chars.get(i + 1).copied().unwrap_or(0));
    }
    int(output, f.glyph_max_size_x(true) as i32);
    int(output, f.glyph_max_size_y(true) as i32);
    int(output, f.glyphs_number(true));
    for x in 0..3 {
        for y in 0..4 {
            show(
                output,
                f.bounding_box(&text_value, align_x(x), align_y(y)),
                |b| rect(output, &b),
            );
        }
    }
    end(output);
}

fn formatter_queries(output: &Output, fmt: &FontTextFormatter, n: i32) {
    let lines = fmt.new_lines();
    for i in -1..=(lines.len() as i32 + 1) {
        real(output, f64::from(fmt.line_width(i)));
    }
    for i in -1..=n {
        let mut g = FontRect::default();
        let ok = fmt.glyph_bounding_box(i, &mut g).unwrap_or(false);
        flag(output, ok);
        if ok {
            rect(output, &g);
        }
    }
    for i in 0..n {
        show(output, fmt.is_lf_symbol(i), |value| flag(output, value));
        show(output, fmt.line_position_index(i), |value| {
            int(output, value)
        });
        show(output, fmt.line_index(i), |value| int(output, value));
    }
    let mut it =
        FontTextFormatterIterator::new(fmt, FontTextFormatterIterationFilter::ExcludeInvisible);
    while it.more() {
        int(output, it.symbol() as i32);
        int(output, it.symbol_position());
        flag(output, it.has_next());
        int(output, it.symbol_next() as i32);
        it.next();
    }
}

fn formatter_results(output: &Output, fmt: &FontTextFormatter) -> i32 {
    let corners = fmt.corners();
    int(output, corners.len() as i32);
    corners.iter().for_each(|c| {
        real(output, f64::from(c.x));
        real(output, f64::from(c.y));
    });
    let lines = fmt.new_lines();
    int(output, lines.len() as i32);
    lines.iter().for_each(|l| real(output, f64::from(*l)));
    [
        fmt.result_width(),
        fmt.result_height(),
        fmt.maximum_symbol_width(),
        fmt.first_position(),
    ]
    .map(f64::from)
    .into_iter()
    .for_each(|value| real(output, value));
    let mut b = FontRect::default();
    fmt.bnd_box(&mut b);
    rect(output, &b);
    real(output, f64::from(fmt.line_height(0)));
    real(output, f64::from(fmt.line_height(1)));
    corners.len() as i32
}

/// `sv_textfmt`: a formatted text, its corners, lines and glyph cells.
pub fn textfmt_case(output: &Output, values: &[f64]) {
    quiet();
    let mut r = Reader::new(values);
    let (name, x, y) = (r.integer(), r.integer(), r.integer());
    let (wrapping, word) = (r.real() as f32, r.flag());
    let params = FontFTFontParams {
        point_size: r.integer() as u32,
        ..Default::default()
    };
    let (text_value, _) = read_text(&mut r);
    let font = FontFTFont::find_and_create(
        &AsciiString::from(NAMES[name as usize]),
        FontFontAspect::Regular,
        &params,
        FontStrictLevel::Any,
    );
    flag(output, font.is_some());
    let Some(f) = font.get() else {
        return end(output);
    };
    let fmt = FontTextFormatter::default();
    fmt.setup_alignment(align_x(x), align_y(y));
    fmt.set_wrapping(wrapping);
    fmt.set_word_wrapping(word);
    tried(output, fmt.append(&text_value, f));
    tried(output, fmt.format());
    let n = formatter_results(output, &fmt);
    formatter_queries(output, &fmt, n);
    end(output);
}
