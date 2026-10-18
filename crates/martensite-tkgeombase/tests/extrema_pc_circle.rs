//! Extrema pc circle contracts.
#![allow(non_snake_case)]
mod common;
#[allow(unused_imports)]
use common::*;
#[allow(unused_imports)]
use prelude::*;

// Test fixture for extrema_pc::Circle tests.
const THE_TOL: f64 = 1.0e-10;
const THE_2PI: f64 = 2.0 * PI;
const THE_PI: f64 = PI;
const THE_PI_2: f64 = PI / 2.0;

//==================================================================================================
// Basic tests - point outside circle
//==================================================================================================

#[test]
fn extrema_pc_circle_point_outside_on_x_axis() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(20.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::with_domain(aCircle, Domain1D::new(0.0, THE_2PI));
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    // Find min and max
    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };
    let aMaxIdx: usize = 1 - aMinIdx;

    // Minimum at (10, 0, 0), distance = 10
    EXPECT_NEAR((aResult[aMinIdx].square_distance).sqrt(), 10.0, THE_TOL);
    assert!(aResult[aMinIdx].is_minimum);
    EXPECT_NEAR(aResult[aMinIdx].parameter, 0.0, THE_TOL);

    // Maximum at (-10, 0, 0), distance = 30
    EXPECT_NEAR((aResult[aMaxIdx].square_distance).sqrt(), 30.0, THE_TOL);
    assert!(!(aResult[aMaxIdx].is_minimum));
    EXPECT_NEAR(aResult[aMaxIdx].parameter, THE_PI, THE_TOL);
}

#[test]
fn extrema_pc_circle_shifted_full_period_unique_representatives() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(20.0, 0.0, 0.0);

    let aUMin: f64 = 7.0;
    let mut anEval = ExtremaPC::Circle::with_domain(aCircle, Domain1D::new(aUMin, aUMin + THE_2PI));
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);
    for anIndex in 0..aResult.nb_ext() {
        EXPECT_GE(aResult[anIndex].parameter, aUMin);
        EXPECT_LE(aResult[anIndex].parameter, aUMin + THE_2PI);
    }
}

#[test]
fn extrema_pc_circle_large_tolerance_preserves_opposite_extrema() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        1.0,
    )
    .unwrap();
    let mut anEval = ExtremaPC::Circle::with_domain(aCircle, Domain1D::new(0.0, THE_2PI));

    let aResult = anEval.perform(Point3::new(2.0, 0.0, 0.0), 4.0, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);
    EXPECT_NEAR(aResult.min_square_distance(), 1.0, THE_TOL);
    EXPECT_NEAR(aResult.max_square_distance(), 9.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_more_than_one_period_retains_constrained_endpoint() {
    let aDomainEnd: f64 = THE_2PI + 0.1;
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let mut anEval = ExtremaPC::Circle::with_domain(aCircle, Domain1D::new(0.0, aDomainEnd));
    let aPoint = elclib::value(aDomainEnd, &aCircle);

    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    let mut hasUpperEndpoint: bool = false;
    for anIndex in 0..aResult.nb_ext() {
        hasUpperEndpoint =
            hasUpperEndpoint || (aResult[anIndex].parameter - aDomainEnd).abs() <= THE_TOL;
    }
    assert!(hasUpperEndpoint);
}

#[test]
fn extrema_pc_circle_point_outside_on_y_axis() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(0.0, 25.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };
    let aMaxIdx: usize = 1 - aMinIdx;

    // Minimum at (0, 10, 0), distance = 15
    EXPECT_NEAR((aResult[aMinIdx].square_distance).sqrt(), 15.0, THE_TOL);
    EXPECT_NEAR(aResult[aMinIdx].parameter, THE_PI_2, THE_TOL);

    // Maximum at (0, -10, 0), distance = 35
    EXPECT_NEAR((aResult[aMaxIdx].square_distance).sqrt(), 35.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_point_outside_diagonal() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(20.0, 20.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Distance from (20,20) to circle center = sqrt(800) = 28.28...
    // Minimum distance = 28.28 - 10 = 18.28...
    let aDistToCenter: f64 = (800.0_f64).sqrt();
    EXPECT_NEAR(
        (aResult[aMinIdx].square_distance).sqrt(),
        aDistToCenter - 10.0,
        THE_TOL,
    );
}

//==================================================================================================
// Point inside circle tests
//==================================================================================================

#[test]
fn extrema_pc_circle_point_inside_on_x_axis() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(3.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };
    let aMaxIdx: usize = 1 - aMinIdx;

    // Minimum distance = 10 - 3 = 7
    EXPECT_NEAR((aResult[aMinIdx].square_distance).sqrt(), 7.0, THE_TOL);

    // Maximum distance = 10 + 3 = 13
    EXPECT_NEAR((aResult[aMaxIdx].square_distance).sqrt(), 13.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_point_inside_on_y_axis() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(0.0, 5.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };
    let aMaxIdx: usize = 1 - aMinIdx;

    // Minimum distance = 10 - 5 = 5
    EXPECT_NEAR((aResult[aMinIdx].square_distance).sqrt(), 5.0, THE_TOL);

    // Maximum distance = 10 + 5 = 15
    EXPECT_NEAR((aResult[aMaxIdx].square_distance).sqrt(), 15.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_point_inside_near_center() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(0.1, 0.1, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aDistToCenter: f64 = (0.02_f64).sqrt();
    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Minimum distance ~= 10 - 0.14
    EXPECT_NEAR(
        (aResult[aMinIdx].square_distance).sqrt(),
        10.0 - aDistToCenter,
        THE_TOL,
    );
}

//==================================================================================================
// Degenerate case - point at center
//==================================================================================================

#[test]
fn extrema_pc_circle_point_at_center_degenerate() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(0.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    // Should return infinite solutions
    assert!(aResult.is_infinite());
    EXPECT_NEAR((aResult.infinite_square_distance).sqrt(), 10.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_point_near_center_not_degenerate() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(1e-8, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, 1e-12, MM).unwrap();

    // Should still find 2 extrema (not degenerate with smaller tolerance)
    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    // Point is essentially at center, so min and max distances are nearly equal to radius
    let aOffset: f64 = 1e-8;
    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };
    let aMaxIdx: usize = 1 - aMinIdx;
    let aMinExpected: f64 = 10.0 - aOffset;
    let aMaxExpected: f64 = 10.0 + aOffset;

    EXPECT_NEAR(
        (aResult[aMinIdx].square_distance).sqrt(),
        aMinExpected,
        1e-6,
    );
    EXPECT_NEAR(
        (aResult[aMaxIdx].square_distance).sqrt(),
        aMaxExpected,
        1e-6,
    );

    // Verify min is near 0 and max is near PI
    EXPECT_NEAR(aResult[aMinIdx].parameter, 0.0, THE_TOL);
    EXPECT_NEAR(aResult[aMaxIdx].parameter, THE_PI, THE_TOL);
}

//==================================================================================================
// Point out of plane tests
//==================================================================================================

#[test]
fn extrema_pc_circle_point_above_plane() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(20.0, 0.0, 5.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Minimum: closest point on circle is (10,0,0)
    // Distance = sqrt(10^2 + 5^2) = sqrt(125)
    EXPECT_NEAR(aResult[aMinIdx].square_distance, 125.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_point_below_plane() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(0.0, 15.0, -7.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Minimum: closest point on circle is (0,10,0)
    // Distance = sqrt(5^2 + 7^2) = sqrt(74)
    EXPECT_NEAR(aResult[aMinIdx].square_distance, 74.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_point_on_axis_degenerate() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(0.0, 0.0, 15.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    // Point on axis = infinite solutions (all points equidistant)
    assert!(aResult.is_infinite());
    // Distance = sqrt(R^2 + h^2) = sqrt(100 + 225) = sqrt(325)
    EXPECT_NEAR(aResult.infinite_square_distance, 325.0, THE_TOL);
}

//==================================================================================================
// Partial arc tests
//==================================================================================================

#[test]
fn extrema_pc_circle_partial_arc_first_quadrant() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(20.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Minimum should be at u=0 (closest point on arc)
    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR((aMinSqDist).sqrt(), 10.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_partial_arc_second_quadrant() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(-20.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    EXPECT_GE(aResult.nb_ext(), 1);

    // Minimum should be at u=PI (closest point on arc)
    let aMinSqDist: f64 = aResult.min_square_distance();
    EXPECT_NEAR((aMinSqDist).sqrt(), 10.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_partial_arc_extrema_outside_range() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(20.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    // Arc from PI/2 to 3*PI/2 (left half of circle)
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());

    // The natural extrema (at 0 and PI) - 0 is outside, PI is inside
    // Should find extremum at PI and possibly endpoints
    let mut aFoundPi: bool = false;
    for i in 0..aResult.nb_ext() {
        if (aResult[i].parameter - THE_PI).abs() < 0.1 {
            aFoundPi = true;
            // Maximum distance at PI
            EXPECT_NEAR((aResult[i].square_distance).sqrt(), 30.0, THE_TOL);
        }
    }
    assert!(aFoundPi);
}

//==================================================================================================
// Different circle configurations
//==================================================================================================

#[test]
fn extrema_pc_circle_circle_with_offset() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(100.0, 200.0, 50.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        15.0,
    )
    .unwrap();
    let aPoint = Point3::new(130.0, 200.0, 50.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Distance from point to center = 30
    // Minimum distance = 30 - 15 = 15
    EXPECT_NEAR((aResult[aMinIdx].square_distance).sqrt(), 15.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_circle_in_yz_plane() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(1.0, 0.0, 0.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(5.0, 20.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Closest point on circle is (0, 10, 0)
    // Distance = sqrt(5^2 + 10^2) = sqrt(125)
    EXPECT_NEAR(aResult[aMinIdx].square_distance, 125.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_circle_in_xz_plane() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 1.0, 0.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(20.0, 3.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Closest point on circle is (10, 0, 0)
    // Distance = sqrt(10^2 + 3^2) = sqrt(109)
    EXPECT_NEAR(aResult[aMinIdx].square_distance, 109.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_small_radius() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        0.001,
    )
    .unwrap();
    let aPoint = Point3::new(0.01, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Minimum distance = 0.01 - 0.001 = 0.009
    EXPECT_NEAR((aResult[aMinIdx].square_distance).sqrt(), 0.009, THE_TOL);
}

#[test]
fn extrema_pc_circle_large_radius() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        1000.0,
    )
    .unwrap();
    let aPoint = Point3::new(1500.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Minimum distance = 1500 - 1000 = 500
    EXPECT_NEAR((aResult[aMinIdx].square_distance).sqrt(), 500.0, THE_TOL);
}

//==================================================================================================
// Point on circle tests
//==================================================================================================

#[test]
fn extrema_pc_circle_point_on_circle() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(10.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Minimum distance = 0 (point is on circle)
    EXPECT_NEAR(aResult[aMinIdx].square_distance, 0.0, THE_TOL);
    EXPECT_NEAR(aResult[aMinIdx].parameter, 0.0, THE_TOL);
}

#[test]
fn extrema_pc_circle_point_on_circle_at_pi() {
    let aCircle = Circle3::new(
        Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
        10.0,
    )
    .unwrap();
    let aPoint = Point3::new(-10.0, 0.0, 0.0);

    let mut anEval = ExtremaPC::Circle::new(aCircle);
    let aResult = anEval.perform_with_endpoints(aPoint, THE_TOL, MM).unwrap();

    assert!(aResult.is_done());
    assert_eq!(aResult.nb_ext(), 2);

    let aMinIdx: usize = if aResult[0].square_distance < aResult[1].square_distance {
        0
    } else {
        1
    };

    // Minimum distance = 0 (point is on circle)
    EXPECT_NEAR(aResult[aMinIdx].square_distance, 0.0, THE_TOL);
    EXPECT_NEAR(aResult[aMinIdx].parameter, THE_PI, THE_TOL);
}

#[test]
fn extrema_pc_circle_non_finite_query_is_rejected() {
    let mut anEvaluator = ExtremaPC::Circle::new(
        Circle3::new(
            Axis2::new(Point3::default(), dir3(0.0, 0.0, 1.0)).unwrap(),
            10.0,
        )
        .unwrap(),
    );
    let aQuery = Point3::new(precision::infinite(), 0.0, 0.0);
    assert_eq!(
        anEvaluator.perform(aQuery, THE_TOL, MM).unwrap().status,
        Status::InvalidInput
    );
    assert_eq!(
        anEvaluator
            .perform_with_endpoints(aQuery, THE_TOL, MM)
            .unwrap()
            .status,
        Status::InvalidInput
    );
}

//=================================================================================================

#[test]
fn extrema_pc_circle_planar_delegation_preserves_parameter_and_height() {
    let mut anEvaluator3d = ExtremaPC::Circle::new(
        Circle3::new(
            Axis2::new(Point3::new(0.0, 0.0, 0.0), dir3(0.0, 0.0, 1.0)).unwrap(),
            4.0,
        )
        .unwrap(),
    );
    let mut anEvaluator2d = ExtremaPC2d::Circle::new(
        Circle2::new(
            Axis22d::new(Point2::new(0.0, 0.0), dir2(1.0, 0.0), true).unwrap(),
            4.0,
        )
        .unwrap(),
    );
    let aResult3d = anEvaluator3d
        .perform(Point3::new(8.0, 3.0, 6.0), THE_TOL, MM)
        .unwrap();
    let aResult2d = anEvaluator2d.perform(Point2::new(8.0, 3.0), THE_TOL, MM);
    assert_eq!(aResult3d.nb_ext(), aResult2d.nb_ext());
    for anIndex in 0..aResult3d.nb_ext() {
        EXPECT_NEAR(
            aResult3d[anIndex].parameter,
            aResult2d[anIndex].parameter,
            THE_TOL,
        );
        EXPECT_NEAR(
            aResult3d[anIndex].square_distance,
            aResult2d[anIndex].square_distance + 36.0,
            THE_TOL,
        );
    }
}
