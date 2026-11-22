// SPDX-License-Identifier: GPL-2.0-only
// Twin of ../stage.rs and the timing helpers of ../linux.rs.
#include "parity.hxx"

#include <OpenGl_Context.hxx>
#include <OpenGl_FrameStats.hxx>
#include <OpenGl_GlCore11Fwd.hxx>
#include <OpenGl_View.hxx>
#include <OpenGl_Window.hxx>
#include <Graphic3d_Camera.hxx>

#include <GL/glx.h>
#include <X11/Xlib.h>

#include <algorithm>
#include <cmath>
#include <cstdio>
#include <stdexcept>

namespace parity
{
namespace
{
const double ORBIT_START     = 30.0;
const double ORBIT_STEP      = 1.0;
const double ORBIT_ELEVATION = 35.264;

double radians(double theDegrees)
{
  return theDegrees * M_PI / 180.0;
}

const Graphic3d_FrameStatsCounter COUNTERS[] = {Graphic3d_FrameStatsCounter_NbStructsNotCulled,
                                                Graphic3d_FrameStatsCounter_NbElemsFillNotCulled,
                                                Graphic3d_FrameStatsCounter_NbElemsLineNotCulled,
                                                Graphic3d_FrameStatsCounter_NbElemsPointNotCulled,
                                                Graphic3d_FrameStatsCounter_NbElemsTextNotCulled,
                                                Graphic3d_FrameStatsCounter_NbTrianglesNotCulled,
                                                Graphic3d_FrameStatsCounter_NbLinesNotCulled,
                                                Graphic3d_FrameStatsCounter_NbPointsNotCulled};

const Graphic3d_RenderingParams::PerfCounters COUNTED = Graphic3d_RenderingParams::PerfCounters(
  Graphic3d_RenderingParams::PerfCounters_Structures | Graphic3d_RenderingParams::PerfCounters_Groups
  | Graphic3d_RenderingParams::PerfCounters_GroupArrays | Graphic3d_RenderingParams::PerfCounters_Triangles
  | Graphic3d_RenderingParams::PerfCounters_Lines | Graphic3d_RenderingParams::PerfCounters_Points);

Handle(OpenGl_Context) view_context(const Handle(V3d_View)& theView)
{
  Handle(OpenGl_View) aGlView = Handle(OpenGl_View)::DownCast(theView->View());
  if (aGlView.IsNull() || aGlView->GlWindow().IsNull())
  {
    throw std::runtime_error("view without an OpenGL window; expected a mapped OpenGl_View");
  }
  return aGlView->GlWindow()->GetGlContext();
}
} // namespace

std::vector<double> timed_runs(const Mode&                     theMode,
                               int                             theOps,
                               const std::function<void(int)>& theOp,
                               const std::function<void()>&    theFinish)
{
  std::vector<double> aNanos;
  for (int aRepeat = 0; aRepeat < theMode.repeats; ++aRepeat)
  {
    const auto aStart = std::chrono::steady_clock::now();
    for (int anIndex = 0; anIndex < theOps; ++anIndex)
    {
      theOp(aRepeat * theOps + anIndex);
    }
    theFinish();
    const auto anElapsed = std::chrono::duration<double, std::nano>(std::chrono::steady_clock::now() - aStart);
    aNanos.push_back(anElapsed.count() / theOps);
  }
  return aNanos;
}

std::vector<double> timed_rebuilds(const Mode&                  theMode,
                                   const std::function<void()>& theBuild,
                                   const std::function<void()>& theOp,
                                   const std::function<void()>& theFinish)
{
  for (int aWarmup = 0; aWarmup < std::min(theMode.warmup, 1); ++aWarmup)
  {
    theBuild();
    theOp();
    theFinish();
  }
  Mode aSingle    = theMode;
  aSingle.repeats = 1;
  std::vector<double> aNanos;
  for (int aRepeat = 0; aRepeat < theMode.repeats; ++aRepeat)
  {
    theBuild();
    // The build's own GPU work must not end inside the timed window.
    theFinish();
    aNanos.push_back(timed_runs(aSingle, 1, [&](int) { theOp(); }, theFinish).front());
  }
  return aNanos;
}

long long fold(const std::vector<long long>& theCounts)
{
  unsigned long long aSum = 0;
  for (long long aCount : theCounts)
  {
    aSum = aSum * 1000003ULL + (unsigned long long)aCount;
  }
  return (long long)aSum;
}

Handle(OpenGl_GraphicDriver) new_driver(const Handle(Aspect_DisplayConnection)& theDisplay)
{
  Handle(OpenGl_GraphicDriver) aDriver = new OpenGl_GraphicDriver(theDisplay, false);
  aDriver->ChangeOptions().swapInterval = 0;
  if (!aDriver->InitContext())
  {
    throw std::runtime_error("OpenGL driver initialization failed; expected a GLX context");
  }
  return aDriver;
}

Handle(Xw_Window) new_window(const Handle(Aspect_DisplayConnection)& theDisplay, int theWidth, int theHeight)
{
  Handle(Xw_Window) aWindow =
    new Xw_Window(theDisplay, "Martensite Vulkan presentation", 100, 100, theWidth, theHeight);
  Display*             aDisplay = (Display*)theDisplay->GetDisplayAspect();
  XSetWindowAttributes anAttributes{};
  // Test geometry must not race the desktop window manager's placement, as in the Rust fixture.
  anAttributes.override_redirect = True;
  XChangeWindowAttributes(aDisplay, (Window)aWindow->NativeHandle(), CWOverrideRedirect, &anAttributes);
  aWindow->Map();
  XSync(aDisplay, False);
  return aWindow;
}

void disable_vertical_sync(const Handle(Xw_Window)& theWindow)
{
  // OpenGl_Context::SetSwapInterval passes 0 to glXSwapIntervalSGI, which rejects it, so
  // OpenGl_Caps::swapInterval = 0 leaves vertical sync on under GLX. Set it as OCCT intends.
  typedef void (*SwapIntervalExt)(Display*, GLXDrawable, int);
  const SwapIntervalExt aSwap =
    (SwapIntervalExt)glXGetProcAddressARB((const GLubyte*)"glXSwapIntervalEXT");
  if (aSwap == nullptr)
  {
    throw std::runtime_error("no glXSwapIntervalEXT; expected GLX_EXT_swap_control");
  }
  Display* aDisplay = (Display*)theWindow->DisplayConnection()->GetDisplayAspect();
  aSwap(aDisplay, (GLXDrawable)theWindow->NativeHandle(), 0);
}

void resize_window(const Handle(Xw_Window)& theWindow, int theWidth, int theHeight)
{
  Display* aDisplay = (Display*)theWindow->DisplayConnection()->GetDisplayAspect();
  XResizeWindow(aDisplay, (Window)theWindow->NativeHandle(), (unsigned)theWidth, (unsigned)theHeight);
  XSync(aDisplay, False);
  theWindow->DoResize();
}

void finish(const Handle(V3d_View)& theView)
{
  view_context(theView)->core11fwd->glFinish();
}

Stage::Stage(const Bench& theBench, const gp_Pnt& theCenter, double theRadius)
    : myCenter(theCenter),
      myRadius(theRadius)
{
  window = new_window(theBench.driver->GetDisplayConnection(), WIDTH, HEIGHT);
  viewer = new V3d_Viewer(theBench.driver);
  viewer->SetDefaultLights();
  viewer->SetLightOn();
  view = viewer->CreateView();
  view->SetImmediateUpdate(false);
  view->SetWindow(window);
  disable_vertical_sync(window);
  view->SetBackgroundColor(Quantity_Color(0.0, 0.0, 0.0, Quantity_TOC_RGB));
  const Handle(Graphic3d_Camera)& aCamera = view->Camera();
  aCamera->SetCenter(myCenter);
  aCamera->SetProjectionType(Graphic3d_Camera::Projection_Orthographic);
  aCamera->SetScale(2.2 * myRadius);
  orbit(0);
}

void Stage::orbit(int theFrame) const
{
  const double anAngle  = radians(ORBIT_START + ORBIT_STEP * theFrame);
  const double aRise    = std::sin(radians(ORBIT_ELEVATION));
  const double aRun     = std::cos(radians(ORBIT_ELEVATION));
  const double aDistance = 4.0 * myRadius;
  const gp_Pnt anEye(myCenter.X() + aDistance * std::cos(anAngle) * aRun,
                     myCenter.Y() + aDistance * std::sin(anAngle) * aRun,
                     myCenter.Z() + aDistance * aRise);
  view->Camera()->SetEye(anEye);
  view->Camera()->SetUp(gp_Dir(0.0, 0.0, 1.0));
}

void Stage::redraw_orbit(int theFrame) const
{
  orbit(theFrame);
  view->Redraw();
}

Handle(Graphic3d_Structure) Stage::structure() const
{
  return new Graphic3d_Structure(viewer->StructureManager());
}

long long Stage::counted_frame() const
{
  orbit(0);
  set_counting(COUNTED, 0.0f);
  view->Redraw();
  finish();
  const Handle(OpenGl_FrameStats)& aStats = view_context(view)->FrameStats();
  std::vector<long long>           aCounts;
  for (Graphic3d_FrameStatsCounter aCounter : COUNTERS)
  {
    aCounts.push_back((long long)aStats->CounterValue(aCounter));
  }
  set_counting(Graphic3d_RenderingParams::PerfCounters_Basic, 1.0f);
  std::fprintf(stderr, "counters [");
  for (long long aCount : aCounts)
  {
    std::fprintf(stderr, " %lld", aCount);
  }
  std::fprintf(stderr, " ]\n");
  return fold(aCounts);
}

void Stage::set_counting(Graphic3d_RenderingParams::PerfCounters theCounters, float theInterval) const
{
  view->ChangeRenderingParams().CollectedStats      = theCounters;
  view->ChangeRenderingParams().StatsUpdateInterval = theInterval;
}

void Stage::close()
{
  view->Remove();
  view.Nullify();
  viewer.Nullify();
  window.Nullify();
}

namespace
{
//! Warms up, then times `theFrames` orbit frames per run and counts the start frame.
Measured measure_orbit_frames(const Stage& theStage, const Mode& theMode, int theFrames)
{
  for (int aFrame = 0; aFrame < std::min(theMode.warmup, theFrames); ++aFrame)
  {
    theStage.redraw_orbit(aFrame);
  }
  Measured aMeasured;
  aMeasured.nanos = timed_runs(
    theMode, theFrames, [&](int theFrame) { theStage.redraw_orbit(theFrame); },
    [&]() { theStage.finish(); });
  aMeasured.checksum = theStage.counted_frame();
  return aMeasured;
}
} // namespace

Measured measure_orbit(const Stage& theStage, const Mode& theMode)
{
  return measure_orbit_frames(theStage, theMode, theMode.frames);
}

Measured measure_heavy_orbit(const Stage& theStage, const Mode& theMode)
{
  return measure_orbit_frames(theStage, theMode, theMode.scaled(10));
}

Handle(Graphic3d_Structure) display_array(const Stage&                               theStage,
                                          const Handle(Graphic3d_ArrayOfPrimitives)& theArray,
                                          const Handle(Graphic3d_Aspects)&           theAspect)
{
  Handle(Graphic3d_Structure) aStructure = theStage.structure();
  Handle(Graphic3d_Group)     aGroup     = aStructure->NewGroup();
  aGroup->SetGroupPrimitivesAspect(theAspect);
  aGroup->AddPrimitiveArray(theArray, true);
  aStructure->Display();
  return aStructure;
}
} // namespace parity
