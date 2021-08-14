use crate::html::stats::Stats;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A common data format fed to Tera templates
#[derive(Serialize)]
pub(crate) struct HtmlData {
    /// System stats
    pub stats: Option<Value>,
    /// Raw ES response with dev idx docs
    pub devs: Option<Value>,
    /// List of related libraries, fully qualified  
    pub related: Option<Vec<RelatedKeywords>>,
    /// The raw search string as entered by the user
    pub raw_search: String,
    /// List of keywords extracted from the raw search
    pub keywords: Vec<String>,
    /// All search terms from the raw search with their counts from different fields in ES
    pub keywords_meta: Vec<KeywordMetadata>,
    /// A list of search terms matching known languages
    pub langs: Vec<String>,
    /// Same as `keywords` as a single string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords_str: Option<String>,
    /// A normalized version of the user login for dev profile page title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_str: Option<String>,
    /// Name of the HTML template to use. Defaults to 404
    pub template_name: String,
    /// Time to live for the HTTP response
    pub ttl: u32,
    /// HTTP response code
    pub http_resp_code: u32,
    /// Contents of HTML meta-tag for bots (nofollow, noindex), if any
    /// e.g. `<meta name="robots" content="noindex">` for `rust+actix` search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_robots: Option<String>,
    /// A container for job stats data populated for stats page only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_jobs: Option<Stats>,
}

/// A view of the keyword from ElasticSearch
#[derive(Serialize)]
pub(crate) struct KeywordMetadata {
    /// A normalized version of what the user searched for
    pub search_term: String,
    /// Number of developers using this keyword
    pub es_keyword_count: usize,
    /// Number of developers using this package
    pub es_package_count: usize,
    /// Number of developers using this language
    pub es_language_count: usize,
    /// True if the term got no matches at all. Needed to simplify the front-end logic.
    pub unknown: bool,
    /// True if the number of allowed search terms was exceeded. Needed to simplify the front end
    /// and keen the control of the number in one place.
    pub too_many: bool,
}

/// List of related keywords extracted from ES
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RelatedKeywords {
    pub k: String,
    pub c: usize,
}
