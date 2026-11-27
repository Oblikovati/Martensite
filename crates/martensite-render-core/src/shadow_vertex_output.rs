//! A derived shadow renders its parent's geometry without owning the parent's capture request.
use crate::PrimitiveDraw;

pub(super) fn retain_main_capture(draws: &mut [PrimitiveDraw], shadow: Option<[usize; 2]>) {
    let Some([rendered_structure, source_parent]) = shadow else {
        return;
    };
    for draw in draws {
        if let Some(token) = draw.vertex_output_target.take() {
            eprintln!(
                "{{\"kind\":\"vertex-output-derived-shadow\",\"source_token\":{token},\"rendered_structure\":{rendered_structure},\"source_parent\":{source_parent},\"capture_selected\":false,\"geometry_retained\":true}}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn selected_draw() -> PrimitiveDraw {
        PrimitiveDraw::new(
            vec![crate::RenderVertex::new([0.; 3], [1.; 4]); 3],
            vec![0, 1, 2],
            crate::identity_clip(),
        )
        .unwrap()
        .with_vertex_output_target(std::num::NonZeroU64::MIN)
    }
    #[test]
    fn shadow_geometry_material_indices_and_draw_count_are_retained() {
        let original = selected_draw();
        let mut draws = vec![original.clone()];
        retain_main_capture(&mut draws, Some([17, 19]));
        let mut expected = original;
        expected.vertex_output_target = None;
        assert_eq!(draws, vec![expected]);
    }
    #[test]
    fn a_shadow_without_its_main_presentation_cannot_complete_a_capture_request() {
        let request = crate::VertexOutputRequest::new(vec![0, 1, 2], 96)
            .unwrap()
            .for_source_token(std::num::NonZeroU64::MIN);
        let mut draws = vec![selected_draw()];
        retain_main_capture(&mut draws, Some([17, 19]));
        assert!(draws.iter().all(|draw| draw.vertex_output_target.is_none()));
        assert!(request.finish().is_err());
    }
    #[test]
    fn main_capture_ownership_is_unchanged_without_a_shadow_parent() {
        let original = selected_draw();
        let mut draws = vec![original.clone(), original.clone()];
        retain_main_capture(&mut draws, None);
        assert_eq!(draws, vec![original.clone(), original]);
    }
}
