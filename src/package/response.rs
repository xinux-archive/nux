use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub took: i64,
    #[serde(rename = "timed_out")]
    pub timed_out: bool,
    #[serde(rename = "_shards")]
    pub shards: Shards,
    pub hits: Hits,
    pub aggregations: Aggregations,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Shards {
    pub total: i64,
    pub successful: i64,
    pub skipped: i64,
    pub failed: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hits {
    pub total: Total,
    #[serde(rename = "max_score")]
    pub max_score: Value,
    pub hits: Vec<Hit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Total {
    pub value: i64,
    pub relation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hit {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub type_field: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_score")]
    pub score: f64,
    #[serde(rename = "_source")]
    pub source: Source,
    pub sort: (f64, String, String),
    #[serde(rename = "matched_queries")]
    pub matched_queries: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "package_attr_name")]
    pub package_attr_name: String,
    #[serde(rename = "package_attr_set")]
    pub package_attr_set: String,
    #[serde(rename = "package_pname")]
    pub package_pname: String,
    #[serde(rename = "package_pversion")]
    pub package_pversion: String,
    #[serde(rename = "package_platforms")]
    pub package_platforms: Vec<String>,
    #[serde(rename = "package_outputs")]
    pub package_outputs: Vec<String>,
    #[serde(rename = "package_default_output")]
    pub package_default_output: String,
    #[serde(rename = "package_programs")]
    pub package_programs: Vec<String>,
    #[serde(rename = "package_license")]
    pub package_license: Vec<PackageLicense>,
    #[serde(rename = "package_license_set")]
    pub package_license_set: Vec<String>,
    #[serde(rename = "package_maintainers")]
    pub package_maintainers: Vec<PackageMaintainer>,
    #[serde(rename = "package_maintainers_set")]
    pub package_maintainers_set: Vec<String>,
    #[serde(rename = "package_description")]
    pub package_description: String,
    #[serde(rename = "package_longDescription")]
    pub package_long_description: Option<String>,
    #[serde(rename = "package_hydra")]
    pub package_hydra: Value,
    #[serde(rename = "package_system")]
    pub package_system: String,
    #[serde(rename = "package_homepage")]
    pub package_homepage: Vec<String>,
    #[serde(rename = "package_position")]
    pub package_position: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicense {
    pub url: String,
    pub full_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainer {
    pub name: String,
    pub github: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregations {
    pub all: All,
    #[serde(rename = "package_attr_set")]
    pub package_attr_set: PackageAttrSet2,
    #[serde(rename = "package_maintainers_set")]
    pub package_maintainers_set: PackageMaintainersSet2,
    #[serde(rename = "package_platforms")]
    pub package_platforms: PackagePlatforms2,
    #[serde(rename = "package_license_set")]
    pub package_license_set: PackageLicenseSet2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct All {
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
    #[serde(rename = "package_attr_set")]
    pub package_attr_set: PackageAttrSet,
    #[serde(rename = "package_maintainers_set")]
    pub package_maintainers_set: PackageMaintainersSet,
    #[serde(rename = "package_platforms")]
    pub package_platforms: PackagePlatforms,
    #[serde(rename = "package_license_set")]
    pub package_license_set: PackageLicenseSet,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrSet {
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    pub buckets: Vec<Bucket>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    pub key: String,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainersSet {
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    pub buckets: Vec<Bucket2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket2 {
    pub key: String,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePlatforms {
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    pub buckets: Vec<Bucket3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket3 {
    pub key: String,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicenseSet {
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    pub buckets: Vec<Bucket4>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket4 {
    pub key: String,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrSet2 {
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    pub buckets: Vec<Bucket5>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket5 {
    pub key: String,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainersSet2 {
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    pub buckets: Vec<Bucket6>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket6 {
    pub key: String,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePlatforms2 {
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    pub buckets: Vec<Bucket7>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket7 {
    pub key: String,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicenseSet2 {
    #[serde(rename = "doc_count_error_upper_bound")]
    pub doc_count_error_upper_bound: i64,
    #[serde(rename = "sum_other_doc_count")]
    pub sum_other_doc_count: i64,
    pub buckets: Vec<Bucket8>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket8 {
    pub key: String,
    #[serde(rename = "doc_count")]
    pub doc_count: i64,
}
