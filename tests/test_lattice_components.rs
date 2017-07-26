/// Public API tests for lattice components: Site, Vertex, Lattice

extern crate vegas_lattice;

use vegas_lattice::{Site, Vertex, Lattice};
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
fn lattice_example() {
    let data = r#"
        {
            "sites": [
                {"id": 0, "kind": "Fe", "position": [0, 0, 0]}
            ],
            "vertices": [
                {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
            ]
        }
    "#;
    let site_result: Result<Lattice, _> = data.parse();
    assert!(site_result.is_ok());
}

#[test]
fn lattice_will_fail_for_inconsistent_vertices() {
    let data = r#"
        {
            "sites": [
                {"id": 0, "kind": "Fe", "position": [0, 0, 0]}
            ],
            "vertices": [
                {"source": 0, "target": 1, "delta": [0, 0, 1], "tags": ["core", "inner"]}
            ]
        }
    "#;
    let site_result: Result<Lattice, _> = data.parse();
    assert!(site_result.is_err());
}

#[test]
fn lattice_will_fail_for_duplicated_site_ids() {
    let data = r#"
        {
            "sites": [
                {"id": 0, "kind": "Fe", "position": [0, 0, 0]},
                {"id": 0, "kind": "Fe+", "position": [0, 0, 0]}
            ],
            "vertices": [
                {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
            ]
        }
    "#;
    let site_result: Result<Lattice, _> = data.parse();
    assert!(site_result.is_err());
}
