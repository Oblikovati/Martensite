// SPDX-License-Identifier: GPL-2.0-only
// Twin of ../geometry.rs: the same vertices, edges and aspects.
#include "parity.hxx"

#include <Graphic3d_ArrayOfPoints.hxx>
#include <Graphic3d_ArrayOfSegments.hxx>
#include <Graphic3d_ArrayOfTriangles.hxx>
#include <Graphic3d_AspectFillArea3d.hxx>
#include <Graphic3d_AspectLine3d.hxx>
#include <Graphic3d_AspectMarker3d.hxx>
#include <Graphic3d_AspectText3d.hxx>

#include <cmath>
#include <cstdio>

namespace parity
{
namespace
{
void add_sphere_vertex(const Handle(Graphic3d_ArrayOfPrimitives)& theArray,
                       int theLon, int theLat, double theRadius, int theRow, int theColumn)
{
  const double aPolar   = M_PI * theRow / theLat;
  const double anAzimuth = 2.0 * M_PI * theColumn / theLon;
  const double aNormal[3] = {std::sin(aPolar) * std::cos(anAzimuth),
                             std::sin(aPolar) * std::sin(anAzimuth),
                             std::cos(aPolar)};
  theArray->AddVertex(theRadius * aNormal[0], theRadius * aNormal[1], theRadius * aNormal[2],
                      aNormal[0], aNormal[1], aNormal[2]);
}

void add_sphere_quad(const Handle(Graphic3d_ArrayOfPrimitives)& theArray, int theLon, int theRow, int theColumn)
{
  const int aFirst = theRow * (theLon + 1) + theColumn + 1;
  const int aNext  = aFirst + 1;
  const int aBelow = aFirst + theLon + 1;
  theArray->AddEdges(aFirst, aBelow, aNext);
  theArray->AddEdges(aNext, aBelow, aBelow + 1);
}

void add_box_face(const Handle(Graphic3d_ArrayOfPrimitives)& theArray,
                  const gp_Pnt& theCenter, double theHalf, int theAxis, double theSign)
{
  double aNormal[3] = {0.0, 0.0, 0.0}, anU[3] = {0.0, 0.0, 0.0}, aV[3] = {0.0, 0.0, 0.0};
  aNormal[theAxis % 3]  = theSign;
  anU[(theAxis + 1) % 3] = 1.0;
  aV[(theAxis + 2) % 3]  = 1.0;
  double aCorners[4][2] = {{-1.0, -1.0}, {1.0, -1.0}, {1.0, 1.0}, {-1.0, 1.0}};
  if (theSign < 0.0)
  {
    std::swap(aCorners[0], aCorners[3]);
    std::swap(aCorners[1], aCorners[2]);
  }
  const double aCenter[3] = {theCenter.X(), theCenter.Y(), theCenter.Z()};
  int          anIndices[4];
  for (int aCorner = 0; aCorner < 4; ++aCorner)
  {
    double aPoint[3];
    for (int i = 0; i < 3; ++i)
    {
      aPoint[i] = aCenter[i] + theHalf * (aNormal[i] + aCorners[aCorner][0] * anU[i] + aCorners[aCorner][1] * aV[i]);
    }
    anIndices[aCorner] = theArray->AddVertex(aPoint[0], aPoint[1], aPoint[2], aNormal[0], aNormal[1], aNormal[2]);
  }
  theArray->AddEdges(anIndices[0], anIndices[1], anIndices[2]);
  theArray->AddEdges(anIndices[0], anIndices[2], anIndices[3]);
}

void grid_cell(int theIndex, double& theX, double& theY)
{
  theX = double(theIndex % 1000) - 500.0;
  theY = double(theIndex / 1000) - 500.0;
}
} // namespace

Handle(Graphic3d_ArrayOfPrimitives) uv_sphere(int theLon, int theLat, double theRadius)
{
  Handle(Graphic3d_ArrayOfPrimitives) anArray = new Graphic3d_ArrayOfTriangles(
    (theLon + 1) * (theLat + 1), 6 * theLon * theLat, Graphic3d_ArrayFlags_VertexNormal);
  for (int aRow = 0; aRow <= theLat; ++aRow)
  {
    for (int aColumn = 0; aColumn <= theLon; ++aColumn)
    {
      add_sphere_vertex(anArray, theLon, theLat, theRadius, aRow, aColumn);
    }
  }
  for (int aRow = 0; aRow < theLat; ++aRow)
  {
    for (int aColumn = 0; aColumn < theLon; ++aColumn)
    {
      add_sphere_quad(anArray, theLon, aRow, aColumn);
    }
  }
  return anArray;
}

Handle(Graphic3d_ArrayOfPrimitives) box_array(const gp_Pnt& theCenter, double theHalf)
{
  Handle(Graphic3d_ArrayOfPrimitives) anArray =
    new Graphic3d_ArrayOfTriangles(24, 36, Graphic3d_ArrayFlags_VertexNormal);
  for (int anAxis = 0; anAxis < 3; ++anAxis)
  {
    add_box_face(anArray, theCenter, theHalf, anAxis, -1.0);
    add_box_face(anArray, theCenter, theHalf, anAxis, 1.0);
  }
  return anArray;
}

Handle(Graphic3d_ArrayOfPrimitives) segment_field(int theCount)
{
  Handle(Graphic3d_ArrayOfPrimitives) anArray = new Graphic3d_ArrayOfSegments(2 * theCount);
  for (int anIndex = 0; anIndex < theCount; ++anIndex)
  {
    double aX, aY;
    grid_cell(anIndex, aX, aY);
    const double aTop = 1.0 + double(anIndex % 7) * 0.5;
    anArray->AddVertex(aX, aY, 0.0);
    anArray->AddVertex(aX, aY, aTop);
  }
  return anArray;
}

Handle(Graphic3d_ArrayOfPrimitives) point_field(int theCount)
{
  Handle(Graphic3d_ArrayOfPrimitives) anArray =
    new Graphic3d_ArrayOfPoints(theCount, Graphic3d_ArrayFlags_VertexColor);
  for (int anIndex = 0; anIndex < theCount; ++anIndex)
  {
    double aX, aY;
    grid_cell(anIndex, aX, aY);
    const double aZ     = 20.0 * std::sin(aX / 50.0) * std::cos(aY / 50.0);
    const double aLevel = (aZ + 20.0) / 40.0;
    anArray->AddVertex(gp_Pnt(aX, aY, aZ), Quantity_Color(aLevel, 0.5, 1.0 - aLevel, Quantity_TOC_RGB));
  }
  return anArray;
}

Handle(Graphic3d_Aspects) lit_aspect(double theR, double theG, double theB, float theTransparency)
{
  Graphic3d_MaterialAspect aMaterial(Graphic3d_NameOfMaterial_Plastified);
  aMaterial.SetTransparency(theTransparency);
  return new Graphic3d_AspectFillArea3d(Aspect_IS_SOLID, Quantity_Color(theR, theG, theB, Quantity_TOC_RGB),
                                        Quantity_NOC_BLACK, Aspect_TOL_SOLID, 1.0, aMaterial, aMaterial);
}

Handle(Graphic3d_Aspects) line_aspect()
{
  return new Graphic3d_AspectLine3d(Quantity_NOC_WHITE, Aspect_TOL_SOLID, 1.0);
}

Handle(Graphic3d_Aspects) point_aspect()
{
  return new Graphic3d_AspectMarker3d(Aspect_TOM_POINT, Quantity_NOC_WHITE, 1.0);
}

Handle(Graphic3d_Aspects) text_aspect()
{
  return new Graphic3d_AspectText3d(Quantity_NOC_WHITE, "DejaVu Sans", 1.0, 0.0, Aspect_TOST_NORMAL,
                                    Aspect_TODT_NORMAL);
}

Handle(Graphic3d_Text) text_label(int theIndex)
{
  Handle(Graphic3d_Text) aText = new Graphic3d_Text(16.0f);
  char                   aLabel[16];
  std::snprintf(aLabel, sizeof(aLabel), "L%03d", theIndex);
  aText->SetText(aLabel);
  aText->SetPosition(gp_Pnt(double(theIndex % 40) * 10.0 - 200.0, double(theIndex / 40) * 10.0 - 125.0, 0.0));
  return aText;
}
} // namespace parity
