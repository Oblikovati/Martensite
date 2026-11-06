//! Pagination and HTTP failures never become an empty publication inventory.
use super::invalid;
use crate::{context::BuildContext, error::BuildResult, process::CommandSpec};
use serde::de::DeserializeOwned;

pub(super) fn pages<T: DeserializeOwned>(
    context: &BuildContext<'_>,
    endpoint: &str,
) -> BuildResult<Vec<T>> {
    let spec =
        CommandSpec::new("gh", context.root()).args(["api", "--paginate", "--slurp", endpoint]);
    let text = context.runner.capture(&spec)?;
    decode_pages(&text, endpoint)
}

fn decode_pages<T: DeserializeOwned>(text: &str, endpoint: &str) -> BuildResult<Vec<T>> {
    let pages: Vec<Vec<T>> = serde_json::from_str(text)
        .map_err(|cause| invalid(&format!("{endpoint}: {cause}"), "complete JSON page arrays"))?;
    if pages.is_empty() {
        return Err(invalid(endpoint, "at least one completed API page"));
    }
    for (index, page) in pages.iter().enumerate() {
        if page.len() > 100 || index + 1 < pages.len() && page.len() != 100 {
            return Err(invalid(
                endpoint,
                "full intermediate pages and at most 100 records per page",
            ));
        }
    }
    Ok(pages.into_iter().flatten().collect())
}

/// One release job reads every published asset, so one dropped request must not end a long run.
const ASSET_ATTEMPTS: u32 = 3;

pub(super) fn asset(context: &BuildContext<'_>, id: u64) -> BuildResult<Vec<u8>> {
    let endpoint = format!("repos/{{owner}}/{{repo}}/releases/assets/{id}");
    let spec = CommandSpec::new("gh", context.root()).args([
        "api",
        &endpoint,
        "--header",
        "Accept: application/octet-stream",
    ]);
    let mut output = context.runner.capture_output(&spec)?;
    for attempt in 1..ASSET_ATTEMPTS {
        if output.code == 0 {
            break;
        }
        std::thread::sleep(retry_delay(attempt));
        output = context.runner.capture_output(&spec)?;
    }
    if output.code != 0 {
        return Err(invalid(
            &format!(
                "asset {id}, HTTP command exit {} after {ASSET_ATTEMPTS} attempts: {}",
                output.code,
                String::from_utf8_lossy(&output.stderr).trim()
            ),
            "successful asset retrieval",
        ));
    }
    Ok(output.stdout)
}

/// The wait before the next attempt. The caller still checks each asset's size and digest.
fn retry_delay(attempt: u32) -> std::time::Duration {
    if cfg!(test) {
        std::time::Duration::ZERO
    } else {
        std::time::Duration::from_secs(u64::from(attempt) * 5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incomplete_page_shapes_cannot_justify_absence() {
        for text in ["", "[]", "[{}]", "[[1],[2]]", "[[1],", "[[],[]]"] {
            assert!(decode_pages::<u64>(text, "fixture").is_err(), "{text}");
        }
        assert!(decode_pages::<u64>("[[]]", "fixture").unwrap().is_empty());
        let pages = serde_json::json!([vec![1; 100], vec![2]]).to_string();
        assert_eq!(decode_pages::<u64>(&pages, "fixture").unwrap().len(), 101);
    }
}
