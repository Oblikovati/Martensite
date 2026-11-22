// SPDX-License-Identifier: GPL-2.0-only
// OCCT twins of the Rust cases in ../ (renderer parity with TKOpenGl). Each case builds the
// same scene through V3d on TKOpenGl as the Rust case of the same name does on Vulkan, and
// reports the same checksum: the primitive counts of the renderer's frame statistics.
#ifndef MARTENSITE_VULKAN_OCCT_PARITY_HXX
#define MARTENSITE_VULKAN_OCCT_PARITY_HXX

#include <Aspect_DisplayConnection.hxx>
#include <Graphic3d_ArrayOfPrimitives.hxx>
#include <Graphic3d_Aspects.hxx>
#include <Graphic3d_Structure.hxx>
#include <Graphic3d_Text.hxx>
#include <OpenGl_GraphicDriver.hxx>
#include <V3d_View.hxx>
#include <V3d_Viewer.hxx>
#include <Xw_Window.hxx>

#include <chrono>
#include <functional>
#include <string>
#include <vector>

namespace parity
{
//! The sizes of one program run: `--bench` measures, otherwise a two-frame smoke test.
struct Mode
{
  int         frames  = 2;
  int         repeats = 1;
  int         warmup  = 1;
  std::string filter;

  //! The operations of one timed run, scaled from the frame count.
  int scaled(int theDivisor) const { return frames / theDivisor > 0 ? frames / theDivisor : 1; }
};

//! One case's timed runs, in nanoseconds per operation, and its work checksum.
struct Measured
{
  std::vector<double> nanos;
  long long           checksum;
};

//! The shared driver every case draws with; startup cases make their own.
struct Bench
{
  Handle(OpenGl_GraphicDriver) driver;
};

//! A named case; `run` builds its scene once and times `mode.repeats` runs.
struct Case
{
  const char* name;
  Measured (*run)(const Bench& theBench, const Mode& theMode);
};

//! Times `repeats` runs of `ops` calls of `op`, then waits for the GPU (`finish`), so a
//! run ends only when its frames are rendered.
std::vector<double> timed_runs(const Mode&                    theMode,
                               int                            theOps,
                               const std::function<void(int)>& theOp,
                               const std::function<void()>&    theFinish);

//! Times `repeats` single operations `op`, each on a new state that `build` makes before the
//! timed window; one untimed build and operation run first, as the warm-up. Each window ends
//! when the GPU has finished (`finish`).
std::vector<double> timed_rebuilds(const Mode&                  theMode,
                                   const std::function<void()>& theBuild,
                                   const std::function<void()>& theOp,
                                   const std::function<void()>& theFinish);

//! Folds counters into one checksum, as the Rust `fold_counts` does.
long long fold(const std::vector<long long>& theCounts);

//! The vertical-sync-free OpenGL driver on `theDisplay`.
Handle(OpenGl_GraphicDriver) new_driver(const Handle(Aspect_DisplayConnection)& theDisplay);

//! A mapped, unmanaged window, as the Rust `NativeWindow` makes one.
Handle(Xw_Window) new_window(const Handle(Aspect_DisplayConnection)& theDisplay, int theWidth, int theHeight);

//! Turns vertical sync off for the GL drawable of `theWindow` (see stage.cpp).
void disable_vertical_sync(const Handle(Xw_Window)& theWindow);

//! Resizes the X window and lets Xw_Window read its new size.
void resize_window(const Handle(Xw_Window)& theWindow, int theWidth, int theHeight);

//! Blocks until the GL context of `theView` has executed every command.
void finish(const Handle(V3d_View)& theView);

const int WIDTH  = 1280;
const int HEIGHT = 720;

//! The stage every case draws on, twin of ../stage.rs.
class Stage
{
public:
  Stage(const Bench& theBench, const gp_Pnt& theCenter, double theRadius);

  void orbit(int theFrame) const;
  void redraw_orbit(int theFrame) const;
  Handle(Graphic3d_Structure) structure() const;
  long long counted_frame() const;
  void finish() const { parity::finish(view); }
  void close();

  Handle(Xw_Window)  window;
  Handle(V3d_Viewer) viewer;
  Handle(V3d_View)   view;

private:
  void set_counting(Graphic3d_RenderingParams::PerfCounters theCounters, float theInterval) const;

  gp_Pnt myCenter;
  double myRadius;
};

//! Warms up, then times `mode.frames` orbit frames per run and counts the start frame.
Measured measure_orbit(const Stage& theStage, const Mode& theMode);

//! `measure_orbit` with a tenth of the frames per run, for the scenes whose Vulkan frame takes
//! seconds, so that a round stays within minutes.
Measured measure_heavy_orbit(const Stage& theStage, const Mode& theMode);

//! The structures a case keeps displayed until it ends.
using Kept = std::vector<Handle(Graphic3d_Structure)>;

//! The columns and rows of the box grid of `frame_structures_10k` and `close_10k`.
const int GRID_10K_COLUMNS = 100;
const int GRID_10K_ROWS    = 100;

//! Displays `theColumns` x `theRows` boxes of half size 1 three units apart, centred on the
//! origin, one structure each (twin of the Rust `box_grid`).
Kept box_grid(const Stage& theStage, int theColumns, int theRows);

//! The radius that frames `box_grid`: the half diagonal of its cells, rounded up.
double box_grid_radius(int theColumns, int theRows);

//! Displays one structure whose one group draws `theArray` with `theAspect`.
Handle(Graphic3d_Structure) display_array(const Stage&                              theStage,
                                          const Handle(Graphic3d_ArrayOfPrimitives)& theArray,
                                          const Handle(Graphic3d_Aspects)&           theAspect);

// Procedural scenes, twins of ../geometry.rs.
Handle(Graphic3d_ArrayOfPrimitives) uv_sphere(int theLon, int theLat, double theRadius);
Handle(Graphic3d_ArrayOfPrimitives) box_array(const gp_Pnt& theCenter, double theHalf);
Handle(Graphic3d_ArrayOfPrimitives) segment_field(int theCount);
Handle(Graphic3d_ArrayOfPrimitives) point_field(int theCount);
Handle(Graphic3d_Aspects)           lit_aspect(double theR, double theG, double theB, float theTransparency);
Handle(Graphic3d_Aspects)           line_aspect();
Handle(Graphic3d_Aspects)           point_aspect();
Handle(Graphic3d_Aspects)           text_aspect();
Handle(Graphic3d_Text)              text_label(int theIndex);

void startup(std::vector<Case>& theCases);
void frames(std::vector<Case>& theCases);
void ais(std::vector<Case>& theCases);
void transfer(std::vector<Case>& theCases);
} // namespace parity

#endif
