use crate::{PrimitiveDraw, RenderError, RenderVertex};

pub(super) fn world_vertex(draw: &PrimitiveDraw, vertex: RenderVertex) -> [f64; 4] {
    let position = [
        vertex.position[0],
        vertex.position[1],
        vertex.position[2],
        vertex.weight,
    ];
    let Some(clipping) = &draw.clipping else {
        return position.map(f64::from);
    };
    std::array::from_fn(|row| {
        f64::from(
            ((clipping.world_from_object()[4 + row] * position[1]
                + clipping.world_from_object()[row] * position[0])
                + clipping.world_from_object()[8 + row] * position[2])
                + clipping.world_from_object()[12 + row] * position[3],
        )
    })
}

pub(super) fn accepts(draw: &PrimitiveDraw, world: [f64; 4]) -> Result<bool, RenderError> {
    let Some(clipping) = &draw.clipping else {
        return Ok(true);
    };
    if world[3] == 0. || !world.iter().all(|value| value.is_finite()) {
        return Err(crate::error::rejected(
            world,
            "finite clipped world position with nonzero W",
        ));
    }
    Ok(clipping
        .chains()
        .iter()
        .all(|chain| chain.iter().any(|plane| positive_halfspace(*plane, world))))
}

fn positive_halfspace(plane: [f32; 4], world: [f64; 4]) -> bool {
    let homogeneous = world.map(|value| value as f32);
    let point = homogeneous.map(|value| value / homogeneous[3]);
    let distance = (plane[1] * point[1] + plane[2] * point[2]) + plane[0] * point[0];
    distance + plane[3] >= 0.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RenderClipping, identity_clip};

    #[test]
    fn world_transform_and_plane_boundary_match_homogeneous_fragment_input() {
        let mut model = identity_clip();
        model[12] = 2.;
        let clipping = RenderClipping::new(vec![vec![[1., 0., 0., -2.]]], model).unwrap();
        let draw = PrimitiveDraw::segments(
            vec![
                RenderVertex::homogeneous([0., 0., 0., 2.], [1.; 4]),
                RenderVertex::homogeneous([1., 0., 0., 2.], [1.; 4]),
            ],
            vec![0, 1],
            identity_clip(),
            crate::LineStyle::new(1., 0xffff, 1).unwrap(),
        )
        .unwrap()
        .with_clipping(clipping);
        let boundary = world_vertex(&draw, draw.vertices[0]);
        assert_eq!(boundary, [4., 0., 0., 2.]);
        assert!(accepts(&draw, boundary).unwrap());
        assert!(accepts(&draw, [6., 0., 0., 2.]).unwrap());
        assert!(!accepts(&draw, [2., 0., 0., 2.]).unwrap());
    }

    #[test]
    fn plane_boundary_divides_homogeneous_position_before_native_dot_order() {
        let plane = [0.100057654, -0.7322584, -0.2176599, 0.24223319];
        let clipping = RenderClipping::new(vec![vec![plane]], identity_clip()).unwrap();
        let draw = PrimitiveDraw::segments(
            vec![
                RenderVertex::new([0.; 3], [1.; 4]),
                RenderVertex::new([1.; 3], [1.; 4]),
            ],
            vec![0, 1],
            identity_clip(),
            crate::LineStyle::new(1., 0xffff, 1).unwrap(),
        )
        .unwrap()
        .with_clipping(clipping);
        assert!(accepts(&draw, [-0.5355978, 0.4122862, -0.06631203, 1.4079705]).unwrap());
    }
}
