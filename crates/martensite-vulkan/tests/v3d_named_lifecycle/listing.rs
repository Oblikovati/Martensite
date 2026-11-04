//! The tree is formatted from the live kernel-backed maps in their native bucket order.
use super::*;
use names::text;

impl<F: SourceDriverFactory> NamedViewerScene<F> {
    pub(super) fn view_list(&self) -> Outcome<String> {
        self.audit()?;
        if self.contexts.is_empty() {
            return Ok(String::new());
        }
        let mut output = String::from("vviewlist:\n");
        for (driver, _) in self.drivers.items() {
            let driver = text(driver);
            output.push_str(&format!("{driver}:\n"));
            self.list_contexts(&driver, &mut output);
        }
        Ok(output)
    }

    fn list_contexts(&self, driver: &str, output: &mut String) {
        for (context, _) in self
            .contexts
            .items()
            .filter(|(name, _)| text(name).contains(driver))
        {
            let context = text(context);
            output.push_str(&format!(" {}:\n", &context[driver.len() + 1..]));
            self.list_views(&context, output);
        }
    }

    fn list_views(&self, context: &str, output: &mut String) {
        for (name, view) in self
            .views
            .items()
            .filter(|(name, _)| text(name).contains(context))
        {
            let name = text(name);
            let current = if view == &self.current { "(*)" } else { "" };
            output.push_str(&format!("  {}{current}\n", &name[context.len() + 1..]));
        }
    }
}
