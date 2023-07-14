mod package;

#[cfg(test)]
mod tests {

    use crate::package::error::NuxError;
    use crate::package::request::{Request, Sort, Aggs, Query, AggsTerms, Type, MultiMatch, PackageAttrName};

    #[test]
    fn is_request_json_ok() {
        let request = Request{
            from: 0,
            size: 50,
            sort: vec![Sort::new("asc")],
            aggs: Aggs::new(""),
            query: Query::new(""),
        };

        let expected = Request::new("asc", "", "");
        assert_eq!(request, expected);
    }

    #[test]
    fn is_sort_ok() {
        let sort1 = Sort {
            score: None,
            package_attr_name: "desc".to_string(),
            package_pversion: "desc".to_string()
        };

        let sort2 = Sort {
            score: Some("desc".to_string()),
            package_attr_name: "desc".to_string(),
            package_pversion: "desc".to_string(),
        };

        let expected1 = Sort::new("desc");
        let expected2 = Sort::new("slave");

        assert_eq!(sort1, expected1);
        assert_eq!(sort2, expected2);
    }

    #[test]
    fn are_terms_ok() {
        let result = Ok(AggsTerms {
            field: "package_attr_set".to_string(),
            size: 20
        });

        let expected_result = AggsTerms::new("attr");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn are_terms_errors_ok() {
        let error = Err(NuxError::SpecificError(String::from("this is an invalid input:(")));

        let expected_error = AggsTerms::new("slave");
        assert_eq!(error, expected_error);
    }

    #[test]
    fn is_type_ok() {
        let result = Ok(Type{
            value: "package".to_string(),
            name: "filter_packages".to_string()
        });

        let error = Err(NuxError::SpecificError(String::from("this is an invalid input:(")));

        let expected_result = Type::new("package");
        let expected_error = Type::new("slave");

        assert_eq!(result, expected_result);
        assert_eq!(error, expected_error);
    }

    #[test]
    fn is_multi_match_ok() {
        let result = MultiMatch {
            type_field: "cross_fields".to_string(),
            query: "slave".to_string(),
            analyzer: "whitespace".to_string(),
            auto_generate_synonyms_phrase_query: false,
            operator: "and".to_string(),
            name: "multi_match_slave".to_string(),
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
        };

        let expected = MultiMatch::new("slave");

        assert_eq!(result, expected);
    }

    #[test]
    fn is_package_name_ok(){
        let result = PackageAttrName{
            value: "*slave*".to_string(),
            case_insensitive: true
        };

        let expected = PackageAttrName::new("slave");

        assert_eq!(result, expected);
    }
}
