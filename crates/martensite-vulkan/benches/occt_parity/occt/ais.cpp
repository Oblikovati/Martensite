// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../ais.rs: the 20 x 20 shaded sphere grid of perf/vis/bug24623_1, a selection change
// in a grid of 10,000 boxes and an assembly of 30,000 connected instances.
#include "parity.hxx"

#include <AIS_ConnectedInteractive.hxx>
#include <AIS_InteractiveContext.hxx>
#include <AIS_Shape.hxx>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeCone.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <TopLoc_Location.hxx>
#include <gp_Trsf.hxx>

#include <cmath>
#include <cstdio>
#include <memory>

namespace parity
{
namespace
{
//! A square grid of `side` x `side` cells `pitch` apart, for a shape that lies inside the
//! sphere of radius `reach` around its origin.
struct GridLayout
{
  int    side;
  double pitch;
  double reach;
};

//! The spheres of radius 100 of bug24623_1.
const GridLayout SPHERES = {20, 110.0, 100.0};
//! Boxes of half size 1; 2 bounds their half diagonal of 1.73.
const GridLayout BOXES = {100, 3.0, 2.0};

//! The instances of frame_connected_30k: 30 parts on a 200 x 150 grid, 10 units apart.
const int    PARTS            = 30;
const int    INSTANCE_COLUMNS = 200;
const int    INSTANCE_ROWS    = 150;
const double INSTANCE_PITCH   = 10.0;
//! Bounds the distance of a part from its origin; the tallest cylinder reaches 6.3.
const double PART_REACH = 8.0;

TopoDS_Shape placed(const TopoDS_Shape& theShape, int theIndex, const GridLayout& theLayout)
{
  gp_Trsf anOffset;
  anOffset.SetTranslation(gp_Vec(double(theIndex / theLayout.side) * theLayout.pitch,
                                 -double(theIndex % theLayout.side) * theLayout.pitch, 0.0));
  return theShape.Moved(TopLoc_Location(anOffset));
}

//! A stage, its context and the displayed objects of a shape grid.
struct ShapeGrid
{
  std::unique_ptr<Stage>                       stage;
  Handle(AIS_InteractiveContext)               context;
  std::vector<Handle(AIS_InteractiveObject)> objects;

  void close()
  {
    objects.clear();
    context.Nullify();
    stage->close();
  }
};

//! Copies of `theShape` on `theLayout`, displayed shaded with selection mode 0, which
//! SetSelected needs.
ShapeGrid shape_grid(const Bench& theBench, const TopoDS_Shape& theShape, const GridLayout& theLayout)
{
  const double aHalf   = theLayout.pitch * (theLayout.side - 1) / 2.0;
  const double aRadius = aHalf * std::sqrt(2.0) + theLayout.reach;
  ShapeGrid    aGrid;
  aGrid.stage   = std::make_unique<Stage>(theBench, gp_Pnt(aHalf, -aHalf, 0.0), aRadius);
  aGrid.context = new AIS_InteractiveContext(aGrid.stage->viewer);
  for (int anIndex = 0; anIndex < theLayout.side * theLayout.side; ++anIndex)
  {
    Handle(AIS_Shape) anObject = new AIS_Shape(placed(theShape, anIndex, theLayout));
    aGrid.context->Display(anObject, AIS_Shaded, 0, false);
    aGrid.objects.push_back(anObject);
  }
  return aGrid;
}

ShapeGrid sphere_grid(const Bench& theBench)
{
  return shape_grid(theBench, BRepPrimAPI_MakeSphere(SPHERES.reach).Shape(), SPHERES);
}

Measured frame_ais_spheres_400(const Bench& theBench, const Mode& theMode)
{
  ShapeGrid aGrid     = sphere_grid(theBench);
  Measured  aMeasured = measure_orbit(*aGrid.stage, theMode);
  aGrid.close();
  return aMeasured;
}

Measured highlight_ais_spheres_400(const Bench& theBench, const Mode& theMode)
{
  ShapeGrid    aGrid  = sphere_grid(theBench);
  const Stage& aStage = *aGrid.stage;
  aStage.redraw_orbit(0);
  long long aDetections[2] = {0, 0};
  auto      aHover         = [&](int) {
    const int aPoints[2][2] = {{WIDTH / 2, HEIGHT / 2}, {2, 2}};
    for (int i = 0; i < 2; ++i)
    {
      aGrid.context->MoveTo(aPoints[i][0], aPoints[i][1], aStage.view, true);
      aDetections[i] += aGrid.context->HasDetected() ? 1 : 0;
    }
  };
  for (int anIndex = 0; anIndex < theMode.warmup; ++anIndex)
  {
    aHover(anIndex);
  }
  Measured aMeasured;
  aMeasured.nanos = timed_runs(theMode, theMode.frames, aHover, [&]() { aStage.finish(); });
  std::fprintf(stderr, "detections [%lld, %lld]\n", aDetections[0], aDetections[1]);
  aMeasured.checksum = fold({aDetections[0], aDetections[1]});
  aGrid.close();
  return aMeasured;
}

Measured select_10k(const Bench& theBench, const Mode& theMode)
{
  const TopoDS_Shape aBox  = BRepPrimAPI_MakeBox(gp_Pnt(-1.0, -1.0, -1.0), gp_Pnt(1.0, 1.0, 1.0)).Shape();
  ShapeGrid          aGrid = shape_grid(theBench, aBox, BOXES);
  const Stage&       aStage = *aGrid.stage;
  aStage.redraw_orbit(0);
  const Handle(AIS_InteractiveObject) aPicks[2] = {aGrid.objects[0], aGrid.objects[5050]};
  int                                  aChanges  = 0;
  auto                                 aSelect   = [&](int) {
    ++aChanges;
    aGrid.context->SetSelected(aPicks[aChanges % 2], true);
  };
  for (int anIndex = 0; anIndex < theMode.warmup; ++anIndex)
  {
    aSelect(anIndex);
  }
  Measured aMeasured;
  aMeasured.nanos    = timed_runs(theMode, theMode.frames, aSelect, [&]() { aStage.finish(); });
  aMeasured.checksum = fold({(long long)aGrid.context->NbSelected(), aStage.counted_frame()});
  aGrid.close();
  return aMeasured;
}

//! A box standing on the origin, centred on the z axis, that grows with `theGrow` (0 to 9).
TopoDS_Shape box_part(double theGrow)
{
  const double aHalfWidth = 1.0 + 0.15 * theGrow;
  const double aHalfDepth = 1.5 + 0.1 * theGrow;
  const double aHeight    = 1.0 + 0.4 * theGrow;
  return BRepPrimAPI_MakeBox(gp_Pnt(-aHalfWidth, -aHalfDepth, 0.0), gp_Pnt(aHalfWidth, aHalfDepth, aHeight))
    .Shape();
}

//! Part `theIndex` of 30: ten boxes, ten cylinders and ten cones that grow with the index.
TopoDS_Shape part(int theIndex)
{
  const double aGrow = double(theIndex % 10);
  switch (theIndex / 10)
  {
    case 0:
      return box_part(aGrow);
    case 1:
      return BRepPrimAPI_MakeCylinder(1.0 + 0.2 * aGrow, 2.0 + 0.4 * aGrow).Shape();
    default:
      return BRepPrimAPI_MakeCone(1.5 + 0.2 * aGrow, 0.5, 2.0 + 0.3 * aGrow).Shape();
  }
}

//! Displays instance `theIndex` of `thePart` at its grid cell, shaded, without selection.
void display_instance(const Handle(AIS_InteractiveContext)& theContext,
                      const Handle(AIS_InteractiveObject)&  thePart,
                      int                                   theIndex)
{
  gp_Trsf anOffset;
  anOffset.SetTranslation(gp_Vec(double(theIndex % INSTANCE_COLUMNS) * INSTANCE_PITCH,
                                 double(theIndex / INSTANCE_COLUMNS) * INSTANCE_PITCH, 0.0));
  Handle(AIS_ConnectedInteractive) anInstance = new AIS_ConnectedInteractive();
  anInstance->Connect(thePart, anOffset);
  theContext->Display(anInstance, AIS_Shaded, -1, false);
}

Measured frame_connected_30k(const Bench& theBench, const Mode& theMode)
{
  const double aHalfWidth = INSTANCE_PITCH * (INSTANCE_COLUMNS - 1) / 2.0;
  const double aHalfDepth = INSTANCE_PITCH * (INSTANCE_ROWS - 1) / 2.0;
  const double aRadius    = std::sqrt(aHalfWidth * aHalfWidth + aHalfDepth * aHalfDepth) + PART_REACH;
  Stage        aStage(theBench, gp_Pnt(aHalfWidth, aHalfDepth, 0.0), aRadius);
  Handle(AIS_InteractiveContext)             aContext = new AIS_InteractiveContext(aStage.viewer);
  std::vector<Handle(AIS_InteractiveObject)> aParts;
  for (int anIndex = 0; anIndex < PARTS; ++anIndex)
  {
    aParts.push_back(new AIS_Shape(part(anIndex)));
  }
  for (int anIndex = 0; anIndex < INSTANCE_COLUMNS * INSTANCE_ROWS; ++anIndex)
  {
    display_instance(aContext, aParts[anIndex % PARTS], anIndex);
  }
  Measured aMeasured = measure_heavy_orbit(aStage, theMode);
  aContext.Nullify();
  aStage.close();
  return aMeasured;
}
} // namespace

void ais(std::vector<Case>& theCases)
{
  theCases.insert(theCases.end(),
                  {{"frame_ais_spheres_400", frame_ais_spheres_400},
                   {"highlight_ais_spheres_400", highlight_ais_spheres_400},
                   {"select_10k", select_10k},
                   {"frame_connected_30k", frame_connected_30k}});
}
} // namespace parity
