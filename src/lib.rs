mod package;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use crate::package::error::NuxError;
    use crate::package::request::{Request, Sort, Aggs, Query, AggsTerms, Type};

    #[test]
    fn is_request_json_ok() {
        let request = Request{
            from: 0,
            size: 50,
            sort: vec![Sort::new("asc")],
            aggs: Aggs::default(),
            query: Query::default(),
        };

        let expected = Request::new("asc");
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

        let error = Err(NuxError::CustomError(ErrorKind::InvalidInput));

        let expected_result = AggsTerms::new("attr");
        let expected_error = AggsTerms::new("slave");

        assert_eq!(result, expected_result);
        assert_eq!(error, expected_error);
    }

    #[test]
    fn is_type_ok() {
        let result = Ok(Type{
            value: "option".to_string(),
            name: "filter_packages".to_string()
        });

        let error = Err(NuxError::CustomError(ErrorKind::InvalidInput));

        let expected_result = Type::new("option");
        let expected_error = Type::new("slave");

        assert_eq!(result, expected_result);
        assert_eq!(error, expected_error);
    }
}
