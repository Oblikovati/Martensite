// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../transfer.rs: start-up, upload, readback and resize.
#include "parity.hxx"

#include <Graphic3d_Group.hxx>
#include <Image_PixMap.hxx>

namespace parity
{
namespace
{
Measured startup_first_frame(const Bench&, const Mode& theMode)
{
  long long aStarted = 0;
  Measured  aMeasured;
  aMeasured.nanos = timed_runs(
    theMode, 1,
    [&](int) {
      Handle(Aspect_DisplayConnection) aDisplay = new Aspect_DisplayConnection();
      Handle(OpenGl_GraphicDriver)     aDriver  = new_driver(aDisplay);
      Handle(Xw_Window)                aWindow  = new_window(aDisplay, WIDTH, HEIGHT);
      Handle(V3d_Viewer)               aViewer  = new V3d_Viewer(aDriver);
      aViewer->SetDefaultLights();
      aViewer->SetLightOn();
      Handle(V3d_View) aView = aViewer->CreateView();
      aView->SetWindow(aWindow);
      disable_vertical_sync(aWindow);
      aView->Redraw();
      finish(aView);
      aStarted += aView->IsInvalidated() ? 0 : 1;
      aView->Remove();
    },
    []() {});
  aMeasured.checksum = aStarted;
  return aMeasured;
}

Measured upload_mesh_100k(const Bench& theBench, const Mode& theMode)
{
  Stage                               aStage(theBench, gp_Pnt(0, 0, 0), 10.0);
  Handle(Graphic3d_ArrayOfPrimitives) anArray  = uv_sphere(300, 170, 10.0);
  Handle(Graphic3d_Aspects)           anAspect = lit_aspect(0.8, 0.5, 0.2, 0.0f);
  Measured                            aMeasured;
  aMeasured.nanos = timed_runs(
    theMode, theMode.scaled(4),
    [&](int theFrame) {
      Handle(Graphic3d_Structure) aStructure = display_array(aStage, anArray, anAspect);
      aStage.redraw_orbit(theFrame);
      aStructure->Erase();
      aStructure->Remove();
    },
    [&]() { aStage.finish(); });
  Handle(Graphic3d_Structure) aKept = display_array(aStage, anArray, anAspect);
  aMeasured.checksum                = aStage.counted_frame();
  aKept.Nullify();
  aStage.close();
  return aMeasured;
}

Measured dump_1080p(const Bench& theBench, const Mode& theMode)
{
  Stage                       aStage(theBench, gp_Pnt(0, 0, 0), 10.0);
  Handle(Graphic3d_Structure) aKept =
    display_array(aStage, uv_sphere(300, 170, 10.0), lit_aspect(0.8, 0.5, 0.2, 0.0f));
  aStage.redraw_orbit(0);
  Image_PixMap anImage;
  long long    aDumped = 0;
  Measured     aMeasured;
  aMeasured.nanos = timed_runs(
    theMode, theMode.scaled(4),
    [&](int) { aDumped += aStage.view->ToPixMap(anImage, 1920, 1080, Graphic3d_BT_RGB, true) ? 1 : 0; },
    []() {});
  const long long aSize = (long long)(anImage.SizeX() * anImage.SizeY());
  aMeasured.checksum    = fold({aDumped, aSize, aStage.counted_frame()});
  aKept.Nullify();
  aStage.close();
  return aMeasured;
}

Measured resize_redraw(const Bench& theBench, const Mode& theMode)
{
  Stage                       aStage(theBench, gp_Pnt(0, 0, 0), 10.0);
  Handle(Graphic3d_Structure) aKept =
    display_array(aStage, uv_sphere(300, 170, 10.0), lit_aspect(0.8, 0.5, 0.2, 0.0f));
  aStage.redraw_orbit(0);
  Measured aMeasured;
  aMeasured.nanos = timed_runs(
    theMode, theMode.scaled(2),
    [&](int theIndex) {
      const bool isSmall = theIndex % 2 == 0;
      resize_window(aStage.window, isSmall ? 1024 : WIDTH, isSmall ? 576 : HEIGHT);
      aStage.view->MustBeResized();
      aStage.view->Redraw();
    },
    [&]() { aStage.finish(); });
  resize_window(aStage.window, WIDTH, HEIGHT);
  aStage.view->MustBeResized();
  aMeasured.checksum = aStage.counted_frame();
  aKept.Nullify();
  aStage.close();
  return aMeasured;
}
} // namespace

void startup(std::vector<Case>& theCases)
{
  theCases.push_back({"startup_first_frame", startup_first_frame});
}

void transfer(std::vector<Case>& theCases)
{
  theCases.insert(theCases.end(),
                  {{"upload_mesh_100k", upload_mesh_100k},
                   {"dump_1080p", dump_1080p},
                   {"resize_redraw", resize_redraw}});
}
} // namespace parity
