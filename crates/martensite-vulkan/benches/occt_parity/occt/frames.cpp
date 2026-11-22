// SPDX-License-Identifier: GPL-2.0-only
// Twins of ../frames.rs: a static scene while the camera orbits one degree per frame.
#include "parity.hxx"

#include <Graphic3d_Group.hxx>

namespace parity
{
namespace
{
using Kept = std::vector<Handle(Graphic3d_Structure)>;

Measured orbit_case(const Bench& theBench, const Mode& theMode, const gp_Pnt& theCenter, double theRadius,
                    const std::function<Kept(const Stage&)>& theBuild)
{
  Stage    aStage(theBench, theCenter, theRadius);
  Kept     aKept     = theBuild(aStage);
  Measured aMeasured = measure_orbit(aStage, theMode);
  aKept.clear();
  aStage.close();
  return aMeasured;
}

double cell(int theIndex, int theCount)
{
  return 3.0 * theIndex - 1.5 * (theCount - 1);
}

Measured frame_empty(const Bench& theBench, const Mode& theMode)
{
  return orbit_case(theBench, theMode, gp_Pnt(0, 0, 0), 10.0, [](const Stage&) { return Kept(); });
}

Measured frame_mesh_1m(const Bench& theBench, const Mode& theMode)
{
  return orbit_case(theBench, theMode, gp_Pnt(0, 0, 0), 10.0, [](const Stage& theStage) {
    return Kept{display_array(theStage, uv_sphere(1000, 500, 10.0), lit_aspect(0.8, 0.5, 0.2, 0.0f))};
  });
}

Measured frame_structures_10k(const Bench& theBench, const Mode& theMode)
{
  return orbit_case(theBench, theMode, gp_Pnt(0, 0, 0), 213.0, [](const Stage& theStage) {
    Handle(Graphic3d_Aspects) anAspect = lit_aspect(0.2, 0.6, 0.9, 0.0f);
    Kept                      aKept;
    for (int anIndex = 0; anIndex < 10000; ++anIndex)
    {
      const gp_Pnt aCenter(cell(anIndex % 100, 100), cell(anIndex / 100, 100), 0.0);
      aKept.push_back(display_array(theStage, box_array(aCenter, 1.0), anAspect));
    }
    return aKept;
  });
}

Measured frame_segments_1m(const Bench& theBench, const Mode& theMode)
{
  return orbit_case(theBench, theMode, gp_Pnt(0, 0, 0), 708.0, [](const Stage& theStage) {
    return Kept{display_array(theStage, segment_field(1000000), line_aspect())};
  });
}

Measured frame_points_1m(const Bench& theBench, const Mode& theMode)
{
  return orbit_case(theBench, theMode, gp_Pnt(0, 0, 0), 708.0, [](const Stage& theStage) {
    return Kept{display_array(theStage, point_field(1000000), point_aspect())};
  });
}

Measured frame_text_1k(const Bench& theBench, const Mode& theMode)
{
  return orbit_case(theBench, theMode, gp_Pnt(0, 0, 0), 240.0, [](const Stage& theStage) {
    Handle(Graphic3d_Structure) aStructure = theStage.structure();
    Handle(Graphic3d_Group)     aGroup     = aStructure->NewGroup();
    aGroup->SetGroupPrimitivesAspect(text_aspect());
    for (int anIndex = 0; anIndex < 1000; ++anIndex)
    {
      aGroup->AddText(text_label(anIndex), true);
    }
    aStructure->Display();
    return Kept{aStructure};
  });
}

Measured frame_transparent_1k(const Bench& theBench, const Mode& theMode)
{
  return orbit_case(theBench, theMode, gp_Pnt(0, 0, 0), 27.0, [](const Stage& theStage) {
    Handle(Graphic3d_Aspects) anAspect = lit_aspect(0.9, 0.3, 0.3, 0.5f);
    Kept                      aKept;
    for (int anIndex = 0; anIndex < 1000; ++anIndex)
    {
      const gp_Pnt aCenter(cell(anIndex % 10, 10), cell(anIndex / 10 % 10, 10), cell(anIndex / 100, 10));
      aKept.push_back(display_array(theStage, box_array(aCenter, 1.0), anAspect));
    }
    return aKept;
  });
}

Measured frame_msaa4_100k(const Bench& theBench, const Mode& theMode)
{
  return orbit_case(theBench, theMode, gp_Pnt(0, 0, 0), 10.0, [](const Stage& theStage) {
    theStage.view->ChangeRenderingParams().NbMsaaSamples = 4;
    return Kept{display_array(theStage, uv_sphere(300, 170, 10.0), lit_aspect(0.8, 0.5, 0.2, 0.0f))};
  });
}
} // namespace

void frames(std::vector<Case>& theCases)
{
  theCases.insert(theCases.end(),
                  {{"frame_empty", frame_empty},
                   {"frame_mesh_1m", frame_mesh_1m},
                   {"frame_structures_10k", frame_structures_10k},
                   {"frame_segments_1m", frame_segments_1m},
                   {"frame_points_1m", frame_points_1m},
                   {"frame_text_1k", frame_text_1k},
                   {"frame_transparent_1k", frame_transparent_1k},
                   {"frame_msaa4_100k", frame_msaa4_100k}});
}
} // namespace parity
