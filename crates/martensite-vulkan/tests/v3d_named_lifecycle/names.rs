//! This private protocol covers the nonempty one/two/three-part source names.
use super::*;

pub(super) struct ResolvedNames {
    pub driver: AsciiString,
    pub context: AsciiString,
    pub view: AsciiString,
}

impl<F: SourceDriverFactory> NamedViewerScene<F> {
    pub(super) fn resolve(&self, input: &str) -> Outcome<ResolvedNames> {
        let parts: Vec<_> = input.split('/').collect();
        if !input.is_ascii() || parts.iter().any(|part| part.is_empty()) || parts.len() > 3 {
            return Err(refused(
                input,
                "a nonempty ASCII source view, viewer/view or driver/viewer/view",
            ));
        }
        let (driver, context, view) = if parts.len() == 1 {
            self.simple_names(input)?
        } else {
            self.path_names(input, &parts)?
        };
        Ok(ResolvedNames {
            driver: AsciiString::from(driver.as_str()),
            context: AsciiString::from(context.as_str()),
            view: AsciiString::from(view.as_str()),
        })
    }

    fn path_names(&self, input: &str, parts: &[&str]) -> Outcome<(String, String, String)> {
        if let [driver, context, _] = parts {
            return Ok((
                driver.to_string(),
                format!("{driver}/{context}"),
                input.to_string(),
            ));
        }
        let driver = self.current_driver_name()?;
        Ok((
            driver.clone(),
            format!("{driver}/{}", parts[0]),
            format!("{driver}/{input}"),
        ))
    }

    fn simple_names(&self, input: &str) -> Outcome<(String, String, String)> {
        let driver = self.current_driver_name()?;
        let context = match self.current_context()? {
            Some(context) => text(self.contexts.find2(&context)?),
            None => free_name(&self.contexts, &format!("{driver}/Viewer")),
        };
        Ok((driver, context.clone(), format!("{context}/{input}")))
    }

    fn current_driver_name(&self) -> Outcome<String> {
        match self.current_context()? {
            Some(context) => Ok(text(
                self.drivers
                    .find2(&context.current_viewer().clone().driver())?,
            )),
            None => Ok(free_name(&self.drivers, "Driver")),
        }
    }

    pub(super) fn current_context(&self) -> Outcome<Option<Handle<AisInteractiveContext>>> {
        if self.current.is_null() {
            return Ok(None);
        }
        Ok(Some(self.context_for_view(&self.current)?))
    }
}

pub(super) fn text(value: &AsciiString) -> String {
    String::from_utf8_lossy(value.as_bytes()).into_owned()
}

pub(super) fn free_name<T: std::fmt::Debug>(map: &DoubleMap<AsciiString, T>, prefix: &str) -> String
where
    martensite_tkernel::ncollection::DefaultHashPolicy:
        martensite_tkernel::ncollection::HashPolicy<T>,
{
    let mut index = 1;
    while map.is_bound1(&AsciiString::from(format!("{prefix}{index}").as_str())) {
        index += 1;
    }
    format!("{prefix}{index}")
}
