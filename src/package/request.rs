use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub from: i64,
    pub size: i64,
    pub sort: Vec<Sort>,
    pub aggs: Aggs,
    pub query: Query,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    #[serde(rename = "_score")]
    pub score: String,
    #[serde(rename = "package_attr_name")]
    pub package_attr_name: String,
    #[serde(rename = "package_pversion")]
    pub package_pversion: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggs {
    #[serde(rename = "package_attr_set")]
    pub package_attr_set: PackageAttrSet,
    #[serde(rename = "package_license_set")]
    pub package_license_set: PackageLicenseSet,
    #[serde(rename = "package_maintainers_set")]
    pub package_maintainers_set: PackageMaintainersSet,
    #[serde(rename = "package_platforms")]
    pub package_platforms: PackagePlatforms,
    pub all: All,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrSet {
    pub terms: Terms,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms {
    pub field: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicenseSet {
    pub terms: Terms2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms2 {
    pub field: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainersSet {
    pub terms: Terms3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms3 {
    pub field: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePlatforms {
    pub terms: Terms4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms4 {
    pub field: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct All {
    pub global: Global,
    pub aggregations: Aggregations,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Global {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregations {
    #[serde(rename = "package_attr_set")]
    pub package_attr_set: PackageAttrSet2,
    #[serde(rename = "package_license_set")]
    pub package_license_set: PackageLicenseSet2,
    #[serde(rename = "package_maintainers_set")]
    pub package_maintainers_set: PackageMaintainersSet2,
    #[serde(rename = "package_platforms")]
    pub package_platforms: PackagePlatforms2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrSet2 {
    pub terms: Terms5,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms5 {
    pub field: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicenseSet2 {
    pub terms: Terms6,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms6 {
    pub field: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainersSet2 {
    pub terms: Terms7,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms7 {
    pub field: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePlatforms2 {
    pub terms: Terms8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms8 {
    pub field: String,
    pub size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub bool: Bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bool {
    pub filter: Vec<Filter>,
    pub must: Vec<Must2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub term: Option<Term>,
    pub bool: Option<Bool2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    #[serde(rename = "type")]
    pub type_field: Type,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub value: String,
    #[serde(rename = "_name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bool2 {
    pub must: Vec<Must>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Must {
    pub bool: Bool3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bool3 {
    pub should: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Must2 {
    #[serde(rename = "dis_max")]
    pub dis_max: DisMax,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisMax {
    #[serde(rename = "tie_breaker")]
    pub tie_breaker: f64,
    pub queries: Vec<Query2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query2 {
    #[serde(rename = "multi_match")]
    pub multi_match: Option<MultiMatch>,
    pub wildcard: Option<Wildcard>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiMatch {
    #[serde(rename = "type")]
    pub type_field: String,
    pub query: String,
    pub analyzer: String,
    #[serde(rename = "auto_generate_synonyms_phrase_query")]
    pub auto_generate_synonyms_phrase_query: bool,
    pub operator: String,
    #[serde(rename = "_name")]
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wildcard {
    #[serde(rename = "package_attr_name")]
    pub package_attr_name: PackageAttrName,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrName {
    pub value: String,
    #[serde(rename = "case_insensitive")]
    pub case_insensitive: bool,
}
