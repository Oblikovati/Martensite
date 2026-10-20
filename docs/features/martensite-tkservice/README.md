# Visualization services

`martensite-tkservice` provides what a 3D viewer needs below its renderer: windows and
input events, pixel maps and image comparison, fonts and text layout, the presentation
data model (structures, groups, primitive arrays, aspects, materials, cameras, lights,
clip planes, textures and shader programs) and the graphic driver interfaces.

| Module | Capabilities |
| --- | --- |
| `aspect` | Window and display interfaces, grids, backgrounds, input events, XR actions and the space-mouse decoder. |
| `image` | Pixel maps and formats, pixel conversions, image comparison and the file-image interface. |
| `font` | Font manager and database, FreeType fonts, glyph outlines and the text formatter. |
| `graphic3d` | Presentation structures and groups, primitive arrays, aspects, materials, cameras, lights, clip planes, textures, shader programs and generators, frame statistics and the driver interfaces. |
| `media` | The video player classes, built without a codec backend. |
| `null_driver` | A graphic driver that keeps the scene data and draws nothing, for tests and headless use. |
| `xw` | The Xlib window (Linux); on other platforms an interface whose constructors raise. |
| `wnt`, `cocoa`, `wasm` | The Windows, macOS and canvas windows (see "Platforms"). |
| `platform` | The typed error of an interface without a backend on the current platform. |
| `json_dump` | The `DumpJson` steps the visualization classes and the viewer crates write. |

```rust
use martensite_tkservice::platform::is_platform_unsupported;
use martensite_tkservice::wnt::WntWindow;

// The Windows backend is not implemented yet, so the constructor raises a typed error.
let failure = WntWindow::new("Viewer", 0, 0, 640, 480).err().expect("unsupported");
assert!(is_platform_unsupported(&failure));
```

## Ownership and thread safety

Structures, groups, aspects, cameras, lights, clip planes and textures are shared
handles: cloning keeps identity, and edits are visible through every clone. The crate
does not publish immutable results. Display connections are leased: each window and
each renderer instance holds a lease, and the connection closes when the last lease is
released. A failed initialization returns a typed lifecycle
failure; the legacy `init` adapter keeps only its message.

## Platforms

Linux is complete. Every other platform interface exists on every platform with a
defined signature. Where its backend is missing it raises `Standard_NotImplemented`
naming the interface and the backend (`platform::is_platform_unsupported` recognizes
it), and its source carries a `TODO(platform)` comment. Future work:

- Windows: register the window class, create or wrap the HWND and implement the
  window queries (`wnt::WntWindow`); route the display connection; the WinCodec image
  codec for `image::ImageAlienPixMap`.
- macOS: drive NSWindow, NSView and NSAutoreleasePool for `cocoa::CocoaWindow` and
  `cocoa::CocoaLocalPool`; optionally Xlib through XQuartz for `xw`.
- Targets without `dlopen` or `LoadLibrary` (such as WebAssembly): link FreeType and
  fontconfig statically instead of loading them at run time.
- The media player and the video recorder need a codec backend; without it the
  recorder accepts `open` and fails every frame push.

## Run-time libraries and dependencies

The display and font libraries are loaded at run time, so the crate builds without
their development packages:

| Library | Loaded as | Used for |
| --- | --- | --- |
| Xlib | `libX11.so.6` through `x11-dl` (Linux) | Displays, windows, events and raster capture. |
| FreeType | `libfreetype.so.6` (Linux), `libfreetype.6.dylib` (macOS), `freetype.dll` and variants (Windows) | Font faces, glyph metrics, outlines and rasterization. |
| fontconfig | `libfontconfig.so.1` | The system font directory list. |

Without FreeType the font library is invalid and the font database holds only what it
can read without it. The bundled fallback font `src/font/dejavu_sans_latin.woff` (a
Latin subset of DejaVu Sans) is embedded in the library; its license (Bitstream Vera,
with DejaVu changes in the public domain) is `src/font/dejavu_sans_latin.LICENSE`.

Pinned dependencies:

| Crate | Version | Targets |
| --- | --- | --- |
| `x11-dl` | `=2.21.0` | Linux |
| `libc` | `=0.2.190` | Unix |
| `windows-sys` | `=0.61.2` (`Win32_Foundation`, `Win32_System_LibraryLoader`, `Win32_System_Registry`, `Win32_System_SystemInformation`) | Windows |

The FFI calls into these libraries and the raw pixel storage need unsafe code; the
crate's exception is recorded in AGENTS.md ("Unsafe code exception").

## Environment and resource names

These names are part of the current behavior and are kept:

- `CSF_ShadersDirectory`, else `CASROOT/src/Shaders`: the folder of the standard GLSL
  programs (`Graphic3dShaderProgram::shaders_folder`).
- `CSF_MDTVTexturesDirectory`, else `CASROOT/src/Textures`: the folder of the named 2D
  textures; the check looks for `2d_MatraDatavision.rgb`.
- The named texture files `2d_*.rgb` and the standard program ids `occt_<name>...`
  (for example `occt_font0000`, `occt_blit`), which the shader-manager corpus digests.
- `DISPLAY`: the default X display.

## Numerical behavior

Transcendental functions go through `martensite-cmath` (behavior version 2, #201).
Single-precision helpers compute in double precision and round once. Compared with the
platform single-precision functions, one value of the frozen corpus changed by one unit
in the last place; `crates/martensite-tkservice/tests/README.md` records it and [pbr-lut-attribution](pbr-lut-attribution/README.md) attributes it, with an MPFR certificate, to one binary32 sine the platform rounded incorrectly.

## Verification

Run `cargo test -p martensite-tkservice`. The tests need FreeType, the bundled
fallback font and one installed system font; `tests/README.md` lists the prerequisites,
the X11 integration feature and the fixtures. The frozen service corpus replays 3,142
requests in process, the shader-manager corpus checks the digest and length of every
generated program, and dedicated replays cover clip-plane sequences, light sets,
primitive buffers, XR actions and frame statistics.

The preparation ran on Linux x86-64. The Windows and macOS targets were checked with
Clippy only; the release candidate still needs the full supported platform matrix.
