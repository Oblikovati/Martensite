use super::super::circle;
use super::*;
use martensite_tkbrep::topexp::Explorer;
use martensite_tkg3d::top_abs::ShapeEnum;

#[test]
fn actual_circle_edge_and_unshown_wire_preserve_shared_input_topology() {
    let inputs = circle::inputs().unwrap();
    let center = inputs.circle.location();
    assert_eq!([center.x(), center.y(), center.z()], [150., 0., 200.]);
    assert_eq!(inputs.circle.radius(), 100.);
    let direction = inputs.circle.axis().direction();
    assert_eq!(*direction.coordinates(), [1., 0., 0.]);
    let wire = inputs.wire.clone().into_shape();
    let mut edges = Explorer::new(&wire, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    assert!(edges.more());
    assert!(edges.current().is_same(&inputs.edge.clone().into_shape()));
    edges.next().unwrap();
    assert!(!edges.more());
}

#[test]
fn actual_free_edge_source_camera_picks_real_point_and_miss_stays_fail_capable() {
    let mut session = cpu_session();
    let inputs = circle::inputs().unwrap();
    circle::scene(&mut session, &inputs).unwrap();
    session.move_cursor(275, 175).unwrap();
    assert_eq!(highlighted(&session), hover_object(&session, "e").unwrap());
    let point = session.detected_point().unwrap();
    assert_checkpoint_point(point);
    observe(&session, "free_edge");
    session.move_cursor(0, 0).unwrap();
    assert!(!session.context().has_detected());
    assert!(
        session
            .detected_point()
            .unwrap()
            .iter()
            .any(|v| (*v - 150.).abs() > 1.)
    );
    assert!(!session.context().is_displayed(&Handle::default()));
}

fn assert_checkpoint_point(point: [f64; 3]) {
    assert!(point.iter().all(|v| v.is_finite()));
    for (actual, expected) in point.into_iter().zip([150., 31., 294.]) {
        assert!(
            (actual - expected).abs() <= 1.,
            "source pick={point:?}; expected component tolerance1"
        );
    }
}
