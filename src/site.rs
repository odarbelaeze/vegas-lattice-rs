use std::str::FromStr;

use super::util::{Axis, Tagged};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;

/// Represetns a site in a lattice
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    kind: String,
    position: (f64, f64, f64),
    tags: Option<Vec<String>>,
}

impl FromStr for Site {
    type Err = SerdeError;
    fn from_str(source: &str) -> Result<Site, Self::Err> {
        serde_json::from_str(source)
    }
}

impl Tagged for Site {
    fn tags(&self) -> Option<&Vec<String>> {
        self.tags.as_ref()
    }
}

impl Site {
    /// Return the position of the site
    pub fn position(&self) -> (f64, f64, f64) {
        self.position
    }

    /// Return the kind of the site
    pub fn kind(&self) -> String {
        self.kind.clone()
    }

    /// Returns a new site moved a given disntance along a given axis
    pub fn move_along(&self, axis: Axis, distance: f64) -> Self {
        let mut site = self.clone();
        match axis {
            Axis::X => site.position.0 += distance,
            Axis::Y => site.position.1 += distance,
            Axis::Z => site.position.2 += distance,
        };
        site
    }

    /// Returns a new site with the same properties but a different kind
    pub fn with_kind(&self, kind: String) -> Self {
        let mut site = self.clone();
        site.kind = kind;
        site
    }
}

#[cfg(test)]
mod test {
    use super::Site;
    use std::str::FromStr;

    #[test]
    fn site_will_take_optional_tags() {
        let data = r#"
            {"kind": "Fe", "position": [0, 0, 0], "tags": ["core", "inner"]}
        "#;
        let site_result: Result<Site, _> = data.parse();
        assert!(site_result.is_ok());
        assert_eq!(
            site_result.unwrap().tags,
            Some(vec!["core".to_string(), "inner".to_string()])
        );
    }

    #[test]
    fn vertex_site_can_be_read_from_str() {
        let data = r#"
            {"kind": "Fe", "position": [0, 0, 0]}
        "#;
        let site_result = Site::from_str(data);
        assert!(site_result.is_ok());
    }
}
