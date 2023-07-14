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
    pub fn new(sort: &str, aggs: &str, aggs_size: Option<u64>, query: &str) -> Request {
        Request {
            from: 0,
            size: 50,
            sort: vec![Sort::new(sort)],
            aggs: Aggs::new(aggs, aggs_size),
            query: Query::new(query),
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
    pub fn new(aggs: &str, size: Option<u64>) -> Aggs {
        Aggs {
            package_attr_set: PackageAttrSet::new(aggs, size),
            package_license_set: PackageLicenseSet::new(aggs, size),
            package_maintainers_set: PackageMaintainersSet::new(aggs, size),
            package_platforms: PackagePlatforms::new(aggs, size),
            all: All::new(aggs, size)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrSet {
    pub terms: AggsTerms,
}

impl PackageAttrSet {
    pub fn new(attr: &str, size: Option<u64>) -> PackageAttrSet {
        PackageAttrSet {
            terms: AggsTerms::new(attr, size).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicenseSet {
    pub terms: AggsTerms,
}

impl PackageLicenseSet {
    pub fn new(license: &str, size: Option<u64>) -> PackageLicenseSet {
        PackageLicenseSet {
            terms: AggsTerms::new(license, size).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainersSet {
    pub terms: AggsTerms,
}

impl PackageMaintainersSet {
    pub fn new(maintainers: &str, size: Option<u64>) -> PackageMaintainersSet {
        PackageMaintainersSet {
            terms: AggsTerms::new(maintainers, size).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePlatforms {
    pub terms: AggsTerms,
}

impl PackagePlatforms {
    pub fn new(platforms: &str, size: Option<u64>) -> PackagePlatforms {
        PackagePlatforms {
            terms: AggsTerms::new(platforms, size).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggsTerms {
    pub field: String,
    pub size: u64,
}

impl AggsTerms {
    pub fn new(field: &str, size: Option<u64>) -> Result<AggsTerms, NuxError> {

        let result = match field {
            "attr" => AggsTerms {
                field: "package_attr_set".to_string(),
                size: size.unwrap_or(20)
            },
            "license" => AggsTerms {
                field: "package_license_set".to_string(),
                size: size.unwrap_or(20)
            },
            "maintainers" => AggsTerms {
                field: "package_maintainers_set".to_string(),
                size: size.unwrap_or(20)
            },
            "platforms" => AggsTerms {
                field: "package_platforms".to_string(),
                size: size.unwrap_or(20)
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
    pub fn new(all: &str, size: Option<u64>) -> All {
        All {
            global: Global::default(),
            aggregations: Aggregations::new(all, size)
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
    pub fn new(aggregations: &str, size: Option<u64>) -> Aggregations {
        Aggregations {
            package_attr_set: PackageAttrSet2::new(aggregations, size),
            package_license_set: PackageLicenseSet2::new(aggregations, size),
            package_maintainers_set: PackageMaintainersSet2::new(aggregations, size),
            package_platforms: PackagePlatforms2::new(aggregations, size)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageAttrSet2 {
    pub terms: AggsTerms,
}

impl PackageAttrSet2 {
    pub fn new(attr: &str, size: Option<u64>) -> PackageAttrSet2 {
        PackageAttrSet2 {
            terms: AggsTerms::new(attr, size).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageLicenseSet2 {
    pub terms: AggsTerms,
}

impl PackageLicenseSet2 {
    pub fn new(license: &str, size: Option<u64>) -> PackageLicenseSet2 {
        PackageLicenseSet2 {
            terms: AggsTerms::new(license, size).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageMaintainersSet2 {
    pub terms: AggsTerms,
}

impl PackageMaintainersSet2 {
    pub fn new(maintainers: &str, size: Option<u64>) -> PackageMaintainersSet2 {
        PackageMaintainersSet2 {
            terms: AggsTerms::new(maintainers, size).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagePlatforms2 {
    pub terms: AggsTerms,
}

impl PackagePlatforms2 {
    pub fn new(platforms: &str, size: Option<u64>) -> PackagePlatforms2 {
        PackagePlatforms2 {
            terms: AggsTerms::new(platforms, size).unwrap()
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub bool: Bool,
}

impl Query {
    pub fn new(query: &str) -> Query {
        Query {
            bool: Bool::new(query)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bool {
    pub filter: Vec<Filter>,
    pub must: Vec<Must2>,
}

impl Bool {
    pub fn new(bool: &str) -> Bool {
        Bool {
            filter: vec![Filter::new(bool)],
            must: vec![Must2::new(bool)]
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub term: Option<Term>,
    pub bool: Option<Bool2>,
}

impl Filter {
    pub fn new(filter: &str) -> Filter {
        Filter {
            term: Some(Term::new(filter)),
            bool: Some(Bool2::default())
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    #[serde(rename = "type")]
    pub type_field: Type,
}

impl Term {
    pub fn new(term: &str) -> Term {
        Term {
            type_field: Type::new(term).unwrap()
        }
    }
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

impl Must2 {
    pub fn new(must: &str) -> Must2 {
        Must2 {
            dis_max: DisMax::new(must)
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisMax {
    #[serde(rename = "tie_breaker")]
    pub tie_breaker: f64,
    pub queries: Vec<Query2>,
}

impl DisMax {
    pub fn new(dismax: &str) -> DisMax {
        DisMax {
            tie_breaker: 0.7,
            queries: vec![Query2::new(dismax)]
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query2 {
    #[serde(rename = "multi_match")]
    pub multi_match: Option<MultiMatch>,
    pub wildcard: Option<Wildcard>,
}

impl Query2 {
    pub fn new(query: &str) -> Query2 {
        Query2 {
            multi_match: Some(MultiMatch::new(query)),
            wildcard: Some(Wildcard::new(query))
        }
    }
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

impl Wildcard {
    pub fn new(wildcard: &str) -> Wildcard {
        Wildcard {
            package_attr_name: PackageAttrName::new(wildcard)
        }
    }
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
