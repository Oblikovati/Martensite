# Image file codecs

`ImageAlienPixMap` reads and writes PNG, JPEG, BMP and GIF files through the
[`image`](https://crates.io/crates/image) crate (issue #330). Only
`src/image/alien_codec.rs` uses the crate; `alien_pixels.rs` converts pixels and
`alien_palette.rs` reads the palette of indexed files.

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

A gray-ramp palette is black then white at one bit per pixel; at other depths,
every listed entry `i` is `(i, i, i)`. GIF files load their first frame on the
logical screen.

Failures leave the pixmap empty and send a fail message:
"Error: image '<name>' has unsupported file format",
"Error: image file '<name>' is missing or invalid" or
"Error: image '<name>' has unsupported pixel format" (with "image stream" for
`load_stream`).

PNG, BMP and GIF decoding is exact. JPEG decoding uses `zune-jpeg`; its inverse
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

## Not covered

Gamma adjustment, other file formats (TGA, TIFF, SGI, HDR, EXR, DDS through
this path) and multi-frame GIF playback beyond the first frame.
