use std::io::ErrorKind;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::package::error::NuxError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub from: i64,
    pub size: i64,
    pub sort: Vec<Sort>,
    pub aggs: Aggs,
    pub query: Query,
}

impl Request {
    pub fn new(sort: &str, aggs: &str) -> Request {
        Request {
            from: 0,
            size: 50,
            sort: vec![Sort::new(sort)],
            aggs: Aggs::new(aggs),
            query: Query::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    #[serde(rename = "_score")]
    pub score: Option<String>,
    #[serde(rename = "package_attr_name")]
    pub package_attr_name: String,
    #[serde(rename = "package_pversion")]
    pub package_pversion: String,
}

impl Sort {
    pub fn new(by: &str) -> Self {
        match by {
            "asc" | "alphabetically ascending" => Sort {
                score: None,
                package_attr_name: "asc".to_string(),
                package_pversion: "asc".to_string(),
            },
            "desc" | "alphabetically descending" => Sort {
                score: None,
                package_attr_name: "desc".to_string(),
                package_pversion: "desc".to_string(),
            },
            _ => Sort::default()
        }
    }
}

impl Default for Sort {
    fn default() -> Self {
        Sort {
            score: Some("desc".to_string()),
            package_attr_name: "desc".to_string(),
            package_pversion: "desc".to_string(),
        }
    }
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

impl Aggs {
    pub fn new(aggs: &str) -> Aggs {
        Aggs {
            package_attr_set: PackageAttrSet::new(aggs),
            package_license_set: PackageLicenseSet::new(aggs),
            package_maintainers_set: PackageMaintainersSet::new(aggs),
            package_platforms: PackagePlatforms::new(aggs),
            all: All::new(aggs)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrSet {
    pub terms: AggsTerms,
}

impl PackageAttrSet {
    pub fn new(attr: &str) -> PackageAttrSet {
        PackageAttrSet {
            terms: AggsTerms::new(attr).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicenseSet {
    pub terms: AggsTerms,
}

impl PackageLicenseSet {
    pub fn new(license: &str) -> PackageLicenseSet {
        PackageLicenseSet {
            terms: AggsTerms::new(license).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainersSet {
    pub terms: AggsTerms,
}

impl PackageMaintainersSet {
    pub fn new(maintainers: &str) -> PackageMaintainersSet {
        PackageMaintainersSet {
            terms: AggsTerms::new(maintainers).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePlatforms {
    pub terms: AggsTerms,
}

impl PackagePlatforms {
    pub fn new(platforms: &str) -> PackagePlatforms {
        PackagePlatforms {
            terms: AggsTerms::new(platforms).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggsTerms {
    pub field: String,
    pub size: i64,
}

impl AggsTerms {
    pub fn new(field: &str) -> Result<AggsTerms, NuxError> {
        let result = match field {
            "attr" => AggsTerms {
                field: "package_attr_set".to_string(),
                size: 20
            },
            "license" => AggsTerms {
                field: "package_license_set".to_string(),
                size: 20
            },
            "maintainers" => AggsTerms {
                field: "package_maintainers_set".to_string(),
                size: 20
            },
            "platforms" => AggsTerms {
                field: "package_platforms".to_string(),
                size: 20
            },
            _ => return Err(NuxError::SpecificError(String::from("this is an invalid input:(")))
        };
        Ok(result)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct All {
    pub global: Global,
    pub aggregations: Aggregations,
}

impl All {
    pub fn new(all: &str) -> All {
        All {
            global: Global::default(),
            aggregations: Aggregations::new(all)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Global {}

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

impl Aggregations {
    pub fn new(aggregations: &str) -> Aggregations {
        Aggregations {
            package_attr_set: PackageAttrSet2::new(aggregations),
            package_license_set: PackageLicenseSet2::new(aggregations),
            package_maintainers_set: PackageMaintainersSet2::new(aggregations),
            package_platforms: PackagePlatforms2::new(aggregations)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrSet2 {
    pub terms: AggsTerms,
}

impl PackageAttrSet2 {
    pub fn new(attr: &str) -> PackageAttrSet2 {
        PackageAttrSet2 {
            terms: AggsTerms::new(attr).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicenseSet2 {
    pub terms: AggsTerms,
}

impl PackageLicenseSet2 {
    pub fn new(license: &str) -> PackageLicenseSet2 {
        PackageLicenseSet2 {
            terms: AggsTerms::new(license).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainersSet2 {
    pub terms: AggsTerms,
}

impl PackageMaintainersSet2 {
    pub fn new(maintainers: &str) -> PackageMaintainersSet2 {
        PackageMaintainersSet2 {
            terms: AggsTerms::new(maintainers).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePlatforms2 {
    pub terms: AggsTerms,
}

impl PackagePlatforms2 {
    pub fn new(platforms: &str) -> PackagePlatforms2 {
        PackagePlatforms2 {
            terms: AggsTerms::new(platforms).unwrap()
        }
    }
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

impl Type {
    pub fn new(value: &str) -> Result<Type, NuxError> {
        let result = match value {
            "package" => Type {
                value: "package".to_string(),
                name: "filter_packages".to_string()
            },
            "option" => Type {
                value: "option".to_string(),
                name: "filter_packages".to_string()
            },
            _ => return Err(NuxError::SpecificError(String::from("this is an invalid input:(")))
        };
        Ok(result)
    }
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

impl MultiMatch {
    pub fn new(name: &str) -> MultiMatch {
        MultiMatch {
            type_field: "cross_fields".to_string(),
            query: String::from(name),
            analyzer: "whitespace".to_string(),
            auto_generate_synonyms_phrase_query: false,
            operator: "and".to_string(),
            name: "multi_match_".to_owned() + &String::from(name),
            fields: vec![
                "package_attr_name^9".to_string(),
                "package_attr_name.*^5.3999999999999995".to_string(),
                "package_programs^9".to_string(),
                "package_programs.*^5.3999999999999995".to_string(),
                "package_pname^6".to_string(),
                "package_pname.*^3.5999999999999996".to_string(),
                "package_description^1.3".to_string(),
                "package_description.*^0.78".to_string(),
                "package_longDescription^1".to_string(),
                "package_longDescription.*^0.6".to_string(),
                "flake_name^0.5".to_string(),
                "flake_name.*^0.3".to_string()
            ]
        }
    }
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

impl PackageAttrName {
    pub fn new(name: &str) -> PackageAttrName {
        PackageAttrName {
            value: "*".to_owned() + &String::from(name) + "*",
            case_insensitive: true
        }
    }
}
