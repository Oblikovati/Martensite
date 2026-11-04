//! Completion is mandatory for the source-selected operation, not inferred from an optional buffer.
use crate::{RenderError, RenderWindowDepthFailure as Failure};
pub(super) trait DepthDrawCompletion {
    fn validate_draw(&self) -> Result<(), RenderError>;
    #[cfg(feature = "command-state-diagnostics")]
    fn diagnostic_failure(&self, _draw: usize) {}
}
pub(super) struct SubmittedDepthCompletion<'a> {
    pub draw: usize,
    pub required: bool,
    pub actual_pipeline: bool,
    pub status: Option<&'a dyn DepthDrawCompletion>,
}
pub(super) fn validate<'a>(
    draws: impl Iterator<Item = SubmittedDepthCompletion<'a>>,
) -> Result<(), RenderError> {
    for draw in draws {
        let result = validate_one(&draw);
        if let Err(error) = result {
            let Some(cause) = error.window_depth_failure() else {
                return Err(error);
            };
            return Err(Failure::GpuDraw {
                draw: draw.draw,
                cause: Box::new(cause.clone()),
            }
            .into());
        }
    }
    Ok(())
}
fn validate_one(draw: &SubmittedDepthCompletion<'_>) -> Result<(), RenderError> {
    if draw.required != draw.actual_pipeline || draw.required != draw.status.is_some() {
        return Err(Failure::GpuBinding {
            required: draw.required,
            actual_pipeline: draw.actual_pipeline,
            present: draw.status.is_some(),
        }
        .into());
    }
    match draw.status {
        Some(status) => validate_status(status, draw.draw),
        None => Ok(()),
    }
}
fn validate_status(status: &dyn DepthDrawCompletion, draw: usize) -> Result<(), RenderError> {
    let result = status.validate_draw();
    #[cfg(feature = "command-state-diagnostics")]
    if result.is_err() {
        status.diagnostic_failure(draw);
    }
    #[cfg(not(feature = "command-state-diagnostics"))]
    let _ = draw;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    struct RecordedDrawCompletion {
        calls: Cell<usize>,
        failure: Option<Failure>,
    }
    impl DepthDrawCompletion for RecordedDrawCompletion {
        fn validate_draw(&self) -> Result<(), RenderError> {
            self.calls.set(self.calls.get() + 1);
            self.failure.clone().map_or(Ok(()), |e| Err(e.into()))
        }
    }
    #[test]
    fn default_without_v3_is_transparent_and_performs_no_status_read() {
        assert!(
            validate(
                [SubmittedDepthCompletion {
                    draw: 0,
                    required: false,
                    actual_pipeline: false,
                    status: None
                }]
                .into_iter()
            )
            .is_ok()
        );
    }
    #[test]
    fn absent_foreign_or_dropped_pipeline_bindings_refuse_before_any_read() {
        let reader = RecordedDrawCompletion {
            calls: Cell::new(0),
            failure: None,
        };
        for (required, pipeline, present) in [
            (true, true, false),
            (true, false, true),
            (false, true, false),
            (false, false, true),
        ] {
            assert!(
                validate(
                    [SubmittedDepthCompletion {
                        draw: 7,
                        required,
                        actual_pipeline: pipeline,
                        status: present.then_some(&reader as &dyn DepthDrawCompletion)
                    }]
                    .into_iter()
                )
                .is_err()
            );
        }
        assert_eq!(reader.calls.get(), 0);
    }
    #[test]
    fn typed_draw_triangle_and_cause_are_retained_and_later_publication_work_is_not_run() {
        let failure = Failure::GpuTriangle {
            triangle: 2,
            vertex_indices: [8, 9, 10],
            flags: 0,
        };
        let first = RecordedDrawCompletion {
            calls: Cell::new(0),
            failure: Some(failure.clone()),
        };
        let later = RecordedDrawCompletion {
            calls: Cell::new(0),
            failure: None,
        };
        let outcome = validate(
            [
                SubmittedDepthCompletion {
                    draw: 3,
                    required: true,
                    actual_pipeline: true,
                    status: Some(&first),
                },
                SubmittedDepthCompletion {
                    draw: 4,
                    required: true,
                    actual_pipeline: true,
                    status: Some(&later),
                },
            ]
            .into_iter(),
        );
        assert_eq!(
            outcome.unwrap_err().window_depth_failure(),
            Some(&Failure::GpuDraw {
                draw: 3,
                cause: Box::new(failure)
            })
        );
        assert_eq!(first.calls.get(), 1);
        assert_eq!(later.calls.get(), 0);
    }
}
