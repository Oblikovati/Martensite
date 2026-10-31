# Image file codecs

`ImageAlienPixMap` reads and writes PNG, JPEG, BMP and GIF files through the
[`image`](https://crates.io/crates/image) crate (issue #330), and reads SGI files
with its own decoder (issue #332). Only `src/image/alien_codec.rs` uses the
crate; `alien_pixels.rs` converts pixels, `alien_palette.rs` reads the palette
of indexed files and `alien_sgi.rs` decodes SGI files.

## Dependency

`image` 0.25.6 with default features off and only `png`, `jpeg`, `bmp` and `gif`
on. Later 0.25 releases always pull a colour-management crate licensed
BSD-3-Clause OR Apache-2.0, which `deny.toml` does not allow. The new crates in
the lock file are `image`, `png`, `fdeflate`, `miniz_oxide`, `gif`, `weezl`,
`color_quant`, `zune-jpeg`, `zune-core`, `byteorder-lite` and `bitflags` 1.x;
`cargo deny --all-features check` passes.

## Loading

The format comes from the leading bytes, else from the text after the last dot
of the name (or the whole name when it has no dot). The pixmap is bottom-up,
each row padded to a multiple of four bytes:

| File pixels | Pixmap format |
| --- | --- |
| 8-bit gray | `Gray` |
| 16-bit gray | `Gray16` (native byte order) |
| 8-bit RGB | `Bgr` |
| RGBA, gray with alpha, GIF frames | `Bgra`; `Bgr32` when every pixel is opaque |
| 16-bit RGB, RGBA or gray with alpha | `Bgr`, high byte of each channel, alpha dropped |
| Indexed, palette a gray ramp | `Gray`, the index |
| Indexed, any other palette | `Bgr`, palette transparency dropped |

| SGI gray | `Gray` |
| SGI gray and alpha | `Bgra` (gray repeated in each colour channel); `Bgr32` when opaque |
| SGI RGB | `Bgr` |
| SGI RGBA | `Bgra`; `Bgr32` when every pixel is opaque |

A gray-ramp palette is black then white at one bit per pixel; at other depths,
every listed entry `i` is `(i, i, i)`. GIF files load their first frame on the
logical screen.

Failures leave the pixmap empty and send a fail message:
"Error: image '<name>' has unsupported file format",
"Error: image file '<name>' is missing or invalid" or
"Error: image '<name>' has unsupported pixel format" (with "image stream" for
`load_stream`).

## SGI files

An SGI file starts with the magic number 474 (bytes `01 da`); without a known
signature, the extensions ".rgb", ".rgba", ".bw" and ".sgi" select it. The
512-byte big-endian header is followed by one plane per channel, each plane's
rows bottom row first. Storage 0 keeps rows verbatim; storage 1 lists each row's
start offset after the header (all rows of plane 0, then plane 1, and so on) and
encodes each row in runs: a count byte with the high bit set is followed by that
many literal bytes, otherwise by one byte repeated that many times, and a zero
count ends the row. The row-length table is not needed and is not read.

The dimension field limits the image: dimension 1 has one row, dimension 2 one
channel, whatever the stored height and depth say. One-byte channels with one to
four channels load. Two-byte channels, colour-map files (colour map other than
0) and more than four channels fail with the unsupported pixel format message;
a truncated header, a storage, channel size or dimension outside the format,
rows beyond the end of the file and runs that end early or overflow the row fail
with the missing or invalid message. SGI files are only read.

PNG, BMP, GIF and SGI decoding is exact. JPEG decoding uses `zune-jpeg`; its inverse
DCT and chroma upsampling differ from other decoders by a few levels per channel
(up to 13 on a 2048x2048 photograph, most pixels within 2).

## Saving

The extension picks the format: ".ppm" keeps the binary PPM writer; PNG, JPEG,
BMP and GIF go through the encoder. Other extensions and empty images return
false. A buffer smaller than the encoded file fails with
"Error: memory buffer too small for storing image".

Pixels are written top row first in red-green-blue order. `Gray`, `Alpha` and
colour formats keep their 8-bit values; 16-bit and float gray scale linearly from
their minimum and maximum onto 0..255 (a constant image maps through [0, 255]);
float colour clamps to [0, 1]. JPEG and GIF take three channels. JPEG uses
quality 75; GIF quantizes to 256 colours with NeuQuant, exact when the image has
at most 256 colours. Half-float and two-channel float formats have no file form.

PNG uses explicit `Default` compression with `Adaptive` row filtering (#347).
The previous `Fast` compression produced large files for repetitive RGB images.
File, stream and buffer saves share this policy. Pixel samples stay lossless,
but compressed bytes and lengths change. Encoded lengths depend on the pixels
and dependency versions. To replay the previous policy, build Martensite commit
`0c4f89ef050d4508c8ed4f6ae563496057ea6074` with its locked dependencies.

## Not covered

Gamma adjustment, SGI writing, SGI files with two-byte channels or a colour map,
other file formats (TGA, TIFF, HDR, EXR, DDS through this path) and multi-frame GIF playback beyond the first frame.
