//! Point, rectangle and polygon selection of cylinders, cones and spheres, with the selection
//! state of every object checked after each select command.
use super::prelude::*;
use martensite_tkbrep::topods::Shape;

/// `c1` a cylinder moved to (25, 25), `c2` a cone moved to (-25, 25), `c3` a truncated cone.
fn cones_and_cylinder() -> Result<[(&'static str, Shape); 3], Box<dyn Error>> {
    Ok([
        ("c1", translated(&cylinder(10., 20.)?, [25., 25., 0.])?),
        ("c2", translated(&cone(10., 0., 20.)?, [-25., 25., 0.])?),
        ("c3", cone(10., 5., 10.)?),
    ])
}

/// Four unit spheres centred at (+-2, +-2, 0): s1 (2, 2), s2 (2, -2), s3 (-2, 2), s4 (-2, -2).
fn four_spheres() -> Result<[(&'static str, Shape); 4], Box<dyn Error>> {
    let at = |offset| -> Result<Shape, Box<dyn Error>> { Ok(translated(&sphere(1.)?, offset)?) };
    Ok([
        ("s1", at([2., 2., 0.])?),
        ("s2", at([2., -2., 0.])?),
        ("s3", at([-2., 2., 0.])?),
        ("s4", at([-2., -2., 0.])?),
    ])
}

/// `single_click_selection_cone`: a cone of radius 10 and height 20, shaded.
pub(crate) fn cone_single_clicks() -> Outcome {
    banner(&[
        "=================================",
        "0032281: Visualization - add Select3DSensitiveCylinder",
        "Tests selection of Select3DSensitiveCylinder",
        "=================================",
    ]);
    let mut run = QuadricRun::shaded(&[("cone", cone(10., 0., 20.)?)])?;
    run.picks(&[
        Pick {
            pixels: &[(300, 200)],
            overlap: false,
            expected: "+",
            message: "Error: cone should be selected",
        },
        Pick {
            pixels: &[(300, 101)],
            overlap: false,
            expected: "+",
            message: "Error: cone should be selected",
        },
        Pick {
            pixels: &[(300, 300)],
            overlap: false,
            expected: "+",
            message: "Error: cone should be selected",
        },
        Pick {
            pixels: &[(357, 182)],
            overlap: false,
            expected: "-",
            message: "Error: cone should be unselected",
        },
        Pick {
            pixels: &[(242, 182)],
            overlap: false,
            expected: "-",
            message: "Error: cone should be unselected",
        },
        Pick {
            pixels: &[(310, 101)],
            overlap: false,
            expected: "-",
            message: "Error: cone should be unselected",
        },
    ])?;
    run.run.finish_without_image()
}

/// `single_click_selection_cylinder`: a cylinder of radius 10 and height 20, shaded.
pub(crate) fn cylinder_single_clicks() -> Outcome {
    banner(&[
        "=================================",
        "0032281: Visualization - add Select3DSensitiveCylinder",
        "Tests selection of Select3DSensitiveCylinder",
        "=================================",
    ]);
    let mut run = QuadricRun::shaded(&[("cyl", cylinder(10., 20.)?)])?;
    run.picks(&[
        Pick {
            pixels: &[(300, 200)],
            overlap: false,
            expected: "+",
            message: "Error: cylinder should be selected",
        },
        Pick {
            pixels: &[(300, 100)],
            overlap: false,
            expected: "+",
            message: "Error: cylinder should be selected",
        },
        Pick {
            pixels: &[(300, 300)],
            overlap: false,
            expected: "+",
            message: "Error: cylinder should be selected",
        },
        Pick {
            pixels: &[(300, 25)],
            overlap: false,
            expected: "-",
            message: "Error: cylinder should be unselected",
        },
        Pick {
            pixels: &[(388, 50)],
            overlap: false,
            expected: "-",
            message: "Error: cylinder should be unselected",
        },
        Pick {
            pixels: &[(424, 200)],
            overlap: false,
            expected: "-",
            message: "Error: cylinder should be unselected",
        },
        Pick {
            pixels: &[(300, 35)],
            overlap: false,
            expected: "+",
            message: "Error: cylinder should be selected",
        },
        Pick {
            pixels: &[(420, 95)],
            overlap: false,
            expected: "+",
            message: "Error: cylinder should be selected",
        },
        Pick {
            pixels: &[(420, 200)],
            overlap: false,
            expected: "+",
            message: "Error: cylinder should be selected",
        },
    ])?;
    run.view(&[ViewStep::Project(Orientation::ZUP_TOP), ViewStep::Fit])?;
    run.picks(&[
        Pick {
            pixels: &[(300, 200)],
            overlap: false,
            expected: "+",
            message: "Error: cylinder should be selected",
        },
        Pick {
            pixels: &[(440, 64)],
            overlap: false,
            expected: "+",
            message: "Error: cylinder should be selected",
        },
        Pick {
            pixels: &[(446, 60)],
            overlap: false,
            expected: "-",
            message: "Error: cylinder should be unselected",
        },
    ])?;
    run.run.finish_without_image()
}

/// `single_click_selection_trunc_cone`: a cone from radius 10 to 5, height 10, shaded.
pub(crate) fn truncated_cone_single_clicks() -> Outcome {
    banner(&[
        "=================================",
        "0032281: Visualization - add Select3DSensitiveCylinder",
        "Tests selection of Select3DSensitiveCylinder",
        "=================================",
    ]);
    let mut run = QuadricRun::shaded(&[("tr_cone", cone(10., 5., 10.)?)])?;
    run.picks(&[
        Pick {
            pixels: &[(300, 200)],
            overlap: false,
            expected: "+",
            message: "Error: truncated cone should be selected",
        },
        Pick {
            pixels: &[(300, 88)],
            overlap: false,
            expected: "+",
            message: "Error: truncated cone  should be selected",
        },
        Pick {
            pixels: &[(421, 187)],
            overlap: false,
            expected: "+",
            message: "Error: truncated cone  should be selected",
        },
        Pick {
            pixels: &[(300, 86)],
            overlap: false,
            expected: "-",
            message: "Error: truncated cone  should be unselected",
        },
        Pick {
            pixels: &[(378, 120)],
            overlap: false,
            expected: "-",
            message: "Error: truncated cone  should be unselected",
        },
        Pick {
            pixels: &[(423, 187)],
            overlap: false,
            expected: "-",
            message: "Error: truncated cone  should be unselected",
        },
    ])?;
    run.run.finish_without_image()
}

/// `polygon_selection`: a cylinder and two cones, polygon picks.
pub(crate) fn cone_cylinder_polygons() -> Outcome {
    banner(&[
        "=================================",
        "0032281: Visualization - add Select3DSensitiveCylinder",
        "Tests polygon selection of Select3DSensitiveCylinder",
        "=================================",
    ]);
    let mut run = QuadricRun::shaded(&cones_and_cylinder()?)?;
    run.picks(&[
        Pick {
            pixels: &[
                (124, 93),
                (234, 24),
                (394, 85),
                (539, 125),
                (542, 346),
                (329, 351),
                (123, 335),
            ],
            overlap: false,
            expected: "+++",
            message: "Error: all shapes should be selected",
        },
        Pick {
            pixels: &[
                (124, 93),
                (234, 24),
                (394, 85),
                (539, 125),
                (542, 346),
                (329, 351),
                (123, 335),
            ],
            overlap: true,
            expected: "+++",
            message: "Error: all shapes should be selected",
        },
        Pick {
            pixels: &[
                (397, 223),
                (401, 161),
                (431, 142),
                (468, 135),
                (504, 145),
                (531, 164),
                (533, 191),
                (531, 267),
                (533, 301),
                (509, 314),
                (475, 327),
                (442, 323),
                (406, 313),
                (396, 288),
            ],
            overlap: false,
            expected: "+--",
            message: "Error: only cylinder c1 should be selected",
        },
        Pick {
            pixels: &[
                (176, 143),
                (242, 39),
                (316, 147),
                (308, 171),
                (299, 180),
                (284, 187),
                (266, 196),
                (245, 196),
                (222, 196),
                (200, 187),
                (188, 182),
                (179, 171),
                (172, 154),
            ],
            overlap: false,
            expected: "-+-",
            message: "Error: only cone c2 should be selected",
        },
        Pick {
            pixels: &[
                (179, 272),
                (217, 214),
                (242, 207),
                (271, 216),
                (279, 223),
                (282, 233),
                (311, 274),
                (311, 287),
                (303, 304),
                (283, 315),
                (247, 323),
                (221, 321),
                (196, 314),
                (184, 304),
                (177, 292),
                (172, 279),
            ],
            overlap: false,
            expected: "--+",
            message: "Error: only cone c3 should be selected",
        },
    ])?;
    run.view(&[ViewStep::Project(Orientation::ZUP_TOP), ViewStep::Fit])?;
    run.picks(&[
        Pick {
            pixels: &[(183, 124), (281, 46), (423, 115), (295, 207)],
            overlap: true,
            expected: "---",
            message: "Error: all shapes should be unselected",
        },
        Pick {
            pixels: &[
                (7, 10),
                (87, 2),
                (145, 16),
                (189, 60),
                (380, 257),
                (396, 317),
                (381, 356),
                (360, 390),
                (301, 396),
                (241, 383),
                (195, 338),
                (2, 133),
                (2, 103),
                (3, 57),
            ],
            overlap: false,
            expected: "-++",
            message: "Error: only cones c2 and c3 should be selected",
        },
        Pick {
            pixels: &[(410, 142), (465, 149), (445, 205)],
            overlap: false,
            expected: "---",
            message: "Error: all shapes should be unselected",
        },
        Pick {
            pixels: &[(410, 142), (465, 149), (445, 205)],
            overlap: true,
            expected: "+--",
            message: "Error: only cylinder c1 should be selected",
        },
        Pick {
            pixels: &[(261, 306), (297, 275), (325, 320)],
            overlap: true,
            expected: "--+",
            message: "Error: only cone c3 should be selected",
        },
    ])?;
    run.view(&[ViewStep::Project(Orientation::ZUP_RIGHT), ViewStep::Fit])?;
    run.picks(&[
        Pick {
            pixels: &[
                (0, 332),
                (65, 198),
                (201, 198),
                (269, 331),
                (260, 358),
                (15, 350),
            ],
            overlap: false,
            expected: "--+",
            message: "Error: only cone c3 should be selected",
        },
        Pick {
            pixels: &[(50, 376), (85, 156), (163, 155), (208, 381)],
            overlap: true,
            expected: "--+",
            message: "Error: only cone c3 should be selected",
        },
        Pick {
            pixels: &[(318, 172), (599, 191), (599, 236), (322, 242)],
            overlap: true,
            expected: "++-",
            message: "Error: cylinder c1 and cone c2 should be selected",
        },
    ])?;
    run.run.finish_without_image()
}

/// `rectangle_selection`: a cylinder and two cones, rectangle picks.
pub(crate) fn cone_cylinder_rectangles() -> Outcome {
    banner(&[
        "=================================",
        "0032281: Visualization - add Select3DSensitiveCylinder",
        "Tests rectangular selection of Select3DSensitiveCylinder",
        "=================================",
    ]);
    let mut run = QuadricRun::shaded(&cones_and_cylinder()?)?;
    run.picks(&[
        Pick {
            pixels: &[(15, 15), (585, 385)],
            overlap: false,
            expected: "+++",
            message: "Error: all shapes should be selected",
        },
        Pick {
            pixels: &[(15, 15), (585, 385)],
            overlap: true,
            expected: "+++",
            message: "Error: all shapes should be selected",
        },
        Pick {
            pixels: &[(314, 52), (565, 347)],
            overlap: false,
            expected: "+--",
            message: "Error: only cylinder c1 should be selected",
        },
        Pick {
            pixels: &[(314, 52), (565, 347)],
            overlap: true,
            expected: "+--",
            message: "Error: only cylinder c1 should be selected",
        },
        Pick {
            pixels: &[(304, 52), (565, 347)],
            overlap: false,
            expected: "+--",
            message: "Error: only cylinder c1 should be selected",
        },
        Pick {
            pixels: &[(304, 52), (565, 347)],
            overlap: true,
            expected: "+++",
            message: "Error: all shapes should be selected",
        },
        Pick {
            pixels: &[(274, 77), (282, 92)],
            overlap: true,
            expected: "---",
            message: "Error: all shapes should be unselected",
        },
    ])?;
    run.view(&[ViewStep::Project(Orientation::ZUP_TOP), ViewStep::Fit])?;
    run.picks(&[
        Pick {
            pixels: &[(90, 90), (510, 310)],
            overlap: false,
            expected: "---",
            message: "Error: all shapes should be unselected",
        },
        Pick {
            pixels: &[(90, 90), (510, 310)],
            overlap: true,
            expected: "+++",
            message: "Error: all shapes should be selected",
        },
        Pick {
            pixels: &[(480, 60), (540, 120)],
            overlap: true,
            expected: "+--",
            message: "Error: only cylinder c1 should be selected",
        },
        Pick {
            pixels: &[(60, 60), (120, 120)],
            overlap: true,
            expected: "-+-",
            message: "Error: only cone c2 should be selected",
        },
        Pick {
            pixels: &[(270, 270), (330, 330)],
            overlap: true,
            expected: "--+",
            message: "Error: only cone c3 should be selected",
        },
        Pick {
            pixels: &[(146, 158), (162, 169)],
            overlap: true,
            expected: "---",
            message: "Error: all shapes should be unselected",
        },
    ])?;
    run.view(&[ViewStep::Project(Orientation::ZUP_RIGHT), ViewStep::Fit])?;
    run.picks(&[
        Pick {
            pixels: &[(0, 0), (600, 400)],
            overlap: false,
            expected: "+++",
            message: "Error: all shapes should be selected",
        },
        Pick {
            pixels: &[(0, 175), (290, 350)],
            overlap: false,
            expected: "--+",
            message: "Error: only cone c3 should be selected",
        },
        Pick {
            pixels: &[(310, 20), (600, 360)],
            overlap: false,
            expected: "++-",
            message: "Error: cylinder c1 and cone c2 should be selected",
        },
        Pick {
            pixels: &[(0, 230), (600, 250)],
            overlap: true,
            expected: "+++",
            message: "Error: all shapes should be selected",
        },
        Pick {
            pixels: &[(85, 185), (205, 355)],
            overlap: true,
            expected: "--+",
            message: "Error: only cone c3 should be selected",
        },
        Pick {
            pixels: &[(400, 40), (500, 370)],
            overlap: true,
            expected: "++-",
            message: "Error: cylinder c1 and cone c2 should be selected",
        },
        Pick {
            pixels: &[(425, 160), (500, 250)],
            overlap: true,
            expected: "++-",
            message: "Error: cylinder c1 and cone c2 should be selected",
        },
    ])?;
    run.run.finish_without_image()
}

/// `single_click_selection`: a unit sphere, shaded.
pub(crate) fn sphere_single_clicks() -> Outcome {
    banner(&[
        "=================================",
        "0032182: Visualization - add Select3DSensitiveSphere",
        "Tests selection of Select3DSenstiveSphere",
        "=================================",
    ]);
    let mut run = QuadricRun::shaded(&[("s", sphere(1.)?)])?;
    run.picks(&[
        Pick {
            pixels: &[(300, 200)],
            overlap: false,
            expected: "+",
            message: "Error: sphere should be selected",
        },
        Pick {
            pixels: &[(195, 110)],
            overlap: false,
            expected: "-",
            message: "Error: sphere should be unselected",
        },
        Pick {
            pixels: &[(390, 110)],
            overlap: false,
            expected: "-",
            message: "Error: sphere should be unselected",
        },
        Pick {
            pixels: &[(390, 282)],
            overlap: false,
            expected: "-",
            message: "Error: sphere should be unselected",
        },
        Pick {
            pixels: &[(213, 291)],
            overlap: false,
            expected: "-",
            message: "Error: sphere should be unselected",
        },
        Pick {
            pixels: &[(300, 200)],
            overlap: true,
            expected: "+",
            message: "Error: sphere should be selected",
        },
        Pick {
            pixels: &[(195, 110)],
            overlap: true,
            expected: "-",
            message: "Error: sphere should be unselected",
        },
    ])?;
    run.run.finish_without_image()
}

/// `polygon_selection`: four unit spheres, polygon picks.
pub(crate) fn sphere_polygons() -> Outcome {
    banner(&[
        "=================================",
        "0032182: Visualization - add Select3DSensitiveSphere",
        "Tests polygon selection of Select3DSenstiveSphere",
        "=================================",
    ]);
    let mut run = QuadricRun::shaded(&four_spheres()?)?;
    run.picks(&[Pick {
        pixels: &[(15, 200), (300, 15), (585, 200), (300, 385)],
        overlap: false,
        expected: "++++",
        message: "Error: all spheres should be selected",
    }])?;
    run.view(&[ViewStep::Project(Orientation::ZUP_TOP)])?;
    run.picks(&[
        Pick {
            pixels: &[(170, 200), (300, 80), (430, 200), (300, 320)],
            overlap: false,
            expected: "----",
            message: "Error: all spheres should be unselected",
        },
        Pick {
            pixels: &[(177, 78), (422, 78), (422, 322), (177, 322)],
            overlap: true,
            expected: "++++",
            message: "Error: all spheres should be selected",
        },
    ])?;
    run.view(&[ViewStep::Project(Orientation::ZUP_RIGHT)])?;
    run.picks(&[
        Pick {
            pixels: &[(181, 323), (289, 196), (147, 85), (59, 206)],
            overlap: false,
            expected: "-+-+",
            message: "Error: spheres s1 and s3 should be unselected",
        },
        Pick {
            pixels: &[(131, 197), (177, 156), (219, 198), (179, 247)],
            overlap: false,
            expected: "----",
            message: "Error: all spheres should be unselected",
        },
        Pick {
            pixels: &[(131, 197), (177, 156), (219, 198), (179, 247)],
            overlap: true,
            expected: "-+-+",
            message: "Error: spheres s1 and s3 should be unselected",
        },
        Pick {
            pixels: &[(227, 147), (282, 268), (367, 151)],
            overlap: true,
            expected: "----",
            message: "Error: all spheres should be unselected",
        },
    ])?;
    run.run.finish_without_image()
}

/// `rectangle_selection`: four unit spheres, rectangle picks.
pub(crate) fn sphere_rectangles() -> Outcome {
    banner(&[
        "=================================",
        "0032182: Visualization - add Select3DSensitiveSphere",
        "Tests rectangular selection of Select3DSenstiveSphere",
        "=================================",
    ]);
    let mut run = QuadricRun::shaded(&four_spheres()?)?;
    run.picks(&[
        Pick {
            pixels: &[(15, 15), (585, 385)],
            overlap: false,
            expected: "++++",
            message: "Error: all spheres should be selected",
        },
        Pick {
            pixels: &[(15, 15), (585, 385)],
            overlap: true,
            expected: "++++",
            message: "Error: all spheres should be selected",
        },
        Pick {
            pixels: &[(50, 135), (300, 400)],
            overlap: false,
            expected: "---+",
            message: "Error: only sphere s4 should be selected",
        },
        Pick {
            pixels: &[(0, 0), (300, 400)],
            overlap: true,
            expected: "-+++",
            message: "Error: spheres s2, s3 and s4 should be selected",
        },
    ])?;
    run.view(&[ViewStep::Project(Orientation::ZUP_RIGHT)])?;
    run.picks(&[
        Pick {
            pixels: &[(80, 100), (300, 300)],
            overlap: false,
            expected: "-+-+",
            message: "Error: spheres s1 and s3 should be unselected",
        },
        Pick {
            pixels: &[(380, 170), (451, 241)],
            overlap: true,
            expected: "+-+-",
            message: "Error: spheres s1 and s3 should be selected",
        },
        Pick {
            pixels: &[(0, 400), (600, 200)],
            overlap: false,
            expected: "----",
            message: "Error: all spheres should be unselected",
        },
        Pick {
            pixels: &[(0, 400), (600, 200)],
            overlap: true,
            expected: "++++",
            message: "Error: all spheres should be selected",
        },
    ])?;
    run.run.finish_without_image()
}
