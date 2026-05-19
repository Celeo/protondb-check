use anyhow::{Result, bail};
use indoc::formatdoc;
use serde::Deserialize;
use std::{fmt, sync::LazyLock};
use ureq::Agent;

pub const ALGOLIA_LIMIT: u8 = 10;
const ALGOLIA_SEARCH_URL: &str = "https://94he6yatei-dsn.algolia.net/1/indexes/steamdb/query?x-algolia-agent=Algolia%20for%20JavaScript%20(4.24.0);%20Browser";
const PROTONDB_SEARCH_URL: &str = "https://www.protondb.com/api/v1/reports/summaries/";

static AGENT: LazyLock<Agent> = LazyLock::new(|| {
    Agent::config_builder()
        .http_status_as_error(false)
        .user_agent("github.com/celeo/protondb-check")
        .build()
        .into()
});

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ReleaseYear {
    Number(f64),
    Text(String),
}

impl fmt::Display for ReleaseYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReleaseYear::Number(n) => write!(f, "{}", *n as u32),
            ReleaseYear::Text(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AlgoliaEntry {
    pub name: String,
    #[serde(default, rename = "oslist")]
    pub os_list: Vec<String>,
    #[serde(rename = "releaseYear")]
    pub release_year: Option<ReleaseYear>,
    #[serde(rename = "objectID")]
    pub object_id: String,
}

#[derive(Debug, Deserialize)]
struct AlgoliaResponse {
    hits: Vec<AlgoliaEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ProtonResponse {
    pub confidence: String,
    pub tier: String,
}

/// Query the Algolia instance for matching games.
pub fn query_algolia(game: &str) -> Result<Vec<AlgoliaEntry>> {
    let mut result = AGENT
        .post(ALGOLIA_SEARCH_URL)
        // access parameters retrieved from the ProtonDB site
        .header("x-algolia-api-key", "9ba0e69fb2974316cdaec8f5f257088f")
        .header("x-algolia-application-id", "94HE6YATEI")
        .header("Referer", "https://www.protondb.com/")
        .send(formatdoc!(
            r#"
            {{
                "query": "{game}",
                "attributesToHighlight": [],
                "attributesToSnippet": [],
                "facets": ["tags"],
                "facetFilters": [["appType:Game"]],
                "hitsPerPage": {ALGOLIA_LIMIT},
                "attributesToRetrieve": [
                    "name",
                    "oslist",
                    "releaseYear",
                    "objectID"
                ],
                "page": 0
            }}"#
        ))?;

    let status = result.status();
    if !status.is_success() {
        let body = match result.body_mut().read_to_string() {
            Ok(s) => format!(": {s}"),
            Err(e) => {
                eprintln!("Could not read response body: {e}");
                String::new()
            }
        };
        bail!(
            "Received status code {} ({}) from Algolia{body}",
            status.as_u16(),
            status.canonical_reason().unwrap_or("unknown reason"),
        );
    }

    let content = result.body_mut().read_json::<AlgoliaResponse>()?;
    Ok(content.hits)
}

/// Query the ProtonDB API for site-specific info for the game.
pub fn query_protondb(game_id: &str) -> Result<ProtonResponse> {
    let mut result = AGENT
        .get(format!("{PROTONDB_SEARCH_URL}{game_id}.json"))
        .call()?;

    let status = result.status();
    if !status.is_success() {
        bail!(
            "Received status code {} ({}) from ProtonDB",
            status.as_u16(),
            status.canonical_reason().unwrap_or("unknown reason"),
        );
    }

    let content = result.body_mut().read_json::<ProtonResponse>()?;
    Ok(content)
}
