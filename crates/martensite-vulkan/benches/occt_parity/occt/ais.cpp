// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../ais.rs: the 20 x 20 shaded sphere grid of perf/vis/bug24623_1.
#include "parity.hxx"

#include <AIS_InteractiveContext.hxx>
#include <AIS_Shape.hxx>
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
const int    SIDE   = 20;
const double RADIUS = 100.0;
const double PITCH  = 110.0;

TopoDS_Shape placed(const TopoDS_Shape& theSphere, int theIndex)
{
  gp_Trsf anOffset;
  anOffset.SetTranslation(gp_Vec(double(theIndex / SIDE) * PITCH, -double(theIndex % SIDE) * PITCH, 0.0));
  return theSphere.Moved(TopLoc_Location(anOffset));
}

struct SphereGrid
{
  std::unique_ptr<Stage>         stage;
  Handle(AIS_InteractiveContext) context;
};

SphereGrid sphere_grid(const Bench& theBench)
{
  const double aHalf   = PITCH * (SIDE - 1) / 2.0;
  const double aRadius = aHalf * std::sqrt(2.0) + RADIUS;
  SphereGrid   aGrid;
  aGrid.stage   = std::make_unique<Stage>(theBench, gp_Pnt(aHalf, -aHalf, 0.0), aRadius);
  aGrid.context = new AIS_InteractiveContext(aGrid.stage->viewer);
  const TopoDS_Shape aSphere = BRepPrimAPI_MakeSphere(RADIUS).Shape();
  for (int anIndex = 0; anIndex < SIDE * SIDE; ++anIndex)
  {
    Handle(AIS_Shape) anObject = new AIS_Shape(placed(aSphere, anIndex));
    aGrid.context->Display(anObject, AIS_Shaded, 0, false);
  }
  return aGrid;
}

Measured frame_ais_spheres_400(const Bench& theBench, const Mode& theMode)
{
  SphereGrid aGrid     = sphere_grid(theBench);
  Measured   aMeasured = measure_orbit(*aGrid.stage, theMode);
  aGrid.context.Nullify();
  aGrid.stage->close();
  return aMeasured;
}

Measured highlight_ais_spheres_400(const Bench& theBench, const Mode& theMode)
{
  SphereGrid aGrid = sphere_grid(theBench);
  const Stage& aStage = *aGrid.stage;
  aStage.redraw_orbit(0);
  long long  aDetections[2] = {0, 0};
  auto       aHover         = [&](int) {
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
  aGrid.context.Nullify();
  aGrid.stage->close();
  return aMeasured;
}
} // namespace

void ais(std::vector<Case>& theCases)
{
  theCases.insert(theCases.end(),
                  {{"frame_ais_spheres_400", frame_ais_spheres_400},
                   {"highlight_ais_spheres_400", highlight_ais_spheres_400}});
}
} // namespace parity
