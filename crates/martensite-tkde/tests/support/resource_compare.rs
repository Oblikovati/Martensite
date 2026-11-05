//! The resource helpers of the wrapper configuration fixtures: parse a resource, compare two by
//! keys and values whatever their order, and replace every provider value.
use martensite_tkde::de::DeConfigurationContext;
use martensite_tkernel::tcollection::AsciiString;

pub fn context(resource: &str) -> DeConfigurationContext {
    let context = DeConfigurationContext::new();
    assert!(context.load(&AsciiString::new(resource)).unwrap());
    context
}

pub fn same_configuration(first: &str, second: &str) -> bool {
    let (first, second) = (context(first), context(second));
    let (first, second) = (first.get_internal_map(), second.get_internal_map());
    first.extent() == second.extent()
        && first
            .items()
            .all(|(key, value)| second.seek(key) == Some(value))
}

pub fn change_provider_values(configuration: &str, value: &str) -> String {
    let mut result = String::new();
    for line in configuration.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("provider.") {
            if let Some(colon) = line.find(':') {
                result.push_str(&format!("{} {value} \t\n", &line[..=colon]));
            }
        } else if trimmed.starts_with("global.") {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

/// The text of a resource; a saved resource is always ASCII.
pub fn resource_text(resource: &AsciiString) -> &str {
    std::str::from_utf8(resource.as_bytes()).expect("an ASCII resource")
}
