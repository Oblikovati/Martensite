//! This caller owns the multi-dump extension; old single-dump hosts need no extra method.
use super::*;

impl<V: SourceViewOutput> SourceCompletion<V> {
    /// Keep every source dump alive until the same END; `completion.capture_additional("second.png")?`.
    pub(crate) fn capture_additional(
        &mut self,
        basename: &'static str,
    ) -> Result<(), Box<dyn Error>> {
        let screenshot = source_dump(&self.scene, basename)?;
        self.additional.push((screenshot, basename));
        Ok(())
    }
}
