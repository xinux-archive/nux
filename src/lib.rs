mod package;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::package::request::{Request, Sort, Aggs, Query};
    use super::*;

    #[test]
    fn is_request_json_ok() {

        let request = Request{
            from: 0,
            size: 50,
            sort: vec![Sort::new(sort)],
            aggs: Aggs::default(),
            query: Query::default(),
        };

        let expected = Request::new("asc");

        assert_eq!(request, expected)
    }
}
