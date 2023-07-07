mod package;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::package::request::{Request, Sort, Aggs, Query};

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

}
