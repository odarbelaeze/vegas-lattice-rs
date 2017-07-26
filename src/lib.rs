extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use serde_json::Error;


#[derive(Debug, Serialize, Deserialize)]
pub struct Site {
    id: u64,
    kind: String,
    position: (f64, f64, f64),
    tags: Option<Vec<String>>,
}


impl std::str::FromStr for Site {
    type Err = Error;
    fn from_str(source: &str) -> Result<Site, Self::Err> {
        serde_json::from_str(source)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Vertex {
    source: u64,
    target: u64,
    delta: (i32, i32, i32),
    tags: Option<Vec<String>>,
}


impl std::str::FromStr for Vertex {
    type Err = Error;
    fn from_str(source: &str) -> Result<Vertex, Self::Err> {
        serde_json::from_str(source)
    }
}


#[cfg(test)]
mod tests {
    use super::{Site, Vertex};
    use std::str::FromStr;

    #[test]
    fn vertex_site_can_be_read_from_str() {
        let data = r#"
            {"id": 0, "kind": "Fe", "position": [0, 0, 0]}
        "#;
        let site_result = Site::from_str(data);
        assert!(site_result.is_ok());
    }

    #[test]
    fn site_will_take_optional_tags() {
        let data = r#"
            {"id": 0, "kind": "Fe", "position": [0, 0, 0], "tags": ["core", "inner"]}
        "#;
        let site_result: Result<Site, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(site_result.unwrap().tags, Some(vec!["core".to_string(), "inner".to_string()]));
    }

    #[test]
    fn site_will_parse_id() {
        let data = r#"
            {"id": 0, "kind": "Fe", "position": [0, 0, 0]}
        "#;
        let site_result: Result<Site, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(site_result.unwrap().id, 0);
    }

    #[test]
    fn vertex_will_take_optional_tags() {
        let data = r#"
            {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
        "#;
        let site_result: Result<Vertex, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(site_result.unwrap().tags, Some(vec!["core".to_string(), "inner".to_string()]));
    }
}
