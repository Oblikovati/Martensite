// Independent OCCT control; build command is recorded in README.md.
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepGProp.hxx>
#include <GProp_GProps.hxx>
#include <Geom_Ellipse.hxx>
#include <Geom_SurfaceOfLinearExtrusion.hxx>
#include <Standard_Version.hxx>
#include <TopoDS_Face.hxx>
#include <gp_Ax2.hxx>
#include <cmath>
#include <iomanip>
#include <iostream>
void emit_measurement(const Handle(Geom_Surface)& surface, double pi, double requested) {
    const TopoDS_Face face = BRepBuilderAPI_MakeFace(surface, pi/2., 3.*pi/2., 0., 2., 1e-9).Face();
    GProp_GProps props;
    const double achieved = BRepGProp::SurfaceProperties(face, props, requested, false);
    std::cout << "whole," << requested << ',' << achieved << ',' << props.Mass() << ',' << props.CentreOfMass().X() << '\n';
    GProp_GProps partitioned;
    for (int index = 0; index < 32; ++index) {
        const double first = pi/2. + pi*index/32.;
        const double last = pi/2. + pi*(index+1)/32.;
        const TopoDS_Face piece = BRepBuilderAPI_MakeFace(surface, first, last, 0., 2., 1e-9).Face();
        GProp_GProps local;
        BRepGProp::SurfaceProperties(piece, local, requested, false);
        partitioned.Add(local);
    }
    std::cout << "partitioned," << requested << ",," << partitioned.Mass() << ',' << partitioned.CentreOfMass().X() << '\n';
}
int main() {
    const gp_Ax2 axis(gp_Pnt(4., 1.5, 0.), gp_Dir(0., 0., 1.));
    Handle(Geom_Curve) curve = new Geom_Ellipse(axis, 1., 0.6);
    Handle(Geom_Surface) surface = new Geom_SurfaceOfLinearExtrusion(curve, gp_Dir(0., 0., 1.));
    std::cout << "OCCT " << OCC_VERSION_COMPLETE << '\n' << std::setprecision(17);
    for (const double requested : {1e-5, 1e-9, 1e-12}) {
        emit_measurement(surface, std::acos(-1.), requested);
    }
}
