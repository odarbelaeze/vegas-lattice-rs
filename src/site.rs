use crate::util::Tagged;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use std::str::FromStr;

/// Represetns a site in a lattice.
///
/// The `kind` field is the type of the site, for example `Fe` for iron or `Cu` for copper.
/// The `position` field is a tuple of the x, y, and z coordinates of the site within the
/// lattice.
///
/// # Exalples
///
/// Here is an example of how to create a site and access its fields:
///
/// ```rust
/// use vegas_lattice::Site;
///
/// let site = Site::new("Fe");
///
/// assert_eq!(site.kind(), "Fe");
/// assert_eq!(site.position(), (0.0, 0.0, 0.0));
/// ```
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
    fn tags(&self) -> Option<Vec<&str>> {
        self.tags
            .as_ref()
            .map(|tags| tags.iter().map(|tag| tag.as_ref()).collect())
    }
}

impl Site {
    /// Create a new site with a given kind located at the origin
    pub fn new(kind: &str) -> Self {
        Site {
            kind: kind.to_string(),
            position: (0.0, 0.0, 0.0),
            tags: None,
        }
    }

    /// Return the position of the site
    pub fn position(&self) -> (f64, f64, f64) {
        self.position
    }

    /// Return the kind of the site
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Move along the x axis
    pub fn move_x(mut self, distance: f64) -> Self {
        self.position.0 += distance;
        self
    }

    /// Move along the y axis
    pub fn move_y(mut self, distance: f64) -> Self {
        self.position.1 += distance;
        self
    }

    /// Move along the z axis
    pub fn move_z(mut self, distance: f64) -> Self {
        self.position.2 += distance;
        self
    }

    /// Changes the kind of the site
    pub fn with_kind(mut self, kind: &str) -> Self {
        self.kind = kind.to_string();
        self
    }

    /// Changes the position of the site
    pub fn with_position(mut self, position: (f64, f64, f64)) -> Self {
        self.position = position;
        self
    }

    /// Adds tags to the site
    pub fn with_tags(mut self, tags: Vec<&str>) -> Self {
        self.tags = Some(tags.iter().map(|s| s.to_string()).collect());
        self
    }
}

#[cfg(test)]
mod test {
    use super::Site;
    use std::str::FromStr;

    #[test]
    fn site_can_be_created() {
        let site = Site::new("Fe");
        assert_eq!(site.kind, "Fe");
        assert_eq!(site.position, (0.0, 0.0, 0.0));
    }

    #[test]
    fn site_can_be_moved() {
        let site = Site::new("Fe").move_x(1.0);
        assert_eq!(site.position, (1.0, 0.0, 0.0));
    }

    #[test]
    fn site_can_be_changed() {
        let site = Site::new("Fe").with_kind("Cu");
        assert_eq!(site.kind, "Cu");
    }

    #[test]
    fn site_can_be_positioned() {
        let site = Site::new("Fe").with_position((1.0, 1.0, 1.0));
        assert_eq!(site.position, (1.0, 1.0, 1.0));
    }

    #[test]
    fn site_can_be_tagged() {
        let site = Site::new("Fe").with_tags(vec!["core", "inner"]);
        assert_eq!(
            site.tags,
            Some(vec!["core".to_string(), "inner".to_string()])
        );
    }

    #[test]
    fn site_can_be_read_from_string() {
        let data = r#"
            {"kind": "Fe", "position": [0, 0, 0]}
        "#;
        let site_result = Site::from_str(data);
        assert!(site_result.is_ok());
    }

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
}
