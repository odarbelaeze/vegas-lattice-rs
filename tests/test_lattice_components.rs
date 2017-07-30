/// Public API tests for lattice components: Site, Vertex, Lattice

extern crate vegas_lattice;

use vegas_lattice::Lattice;



#[test]
fn lattice_example() {
    let data = r#"
        {
            "size": [1, 1, 1],
            "sites": [
                {"kind": "Fe", "position": [0, 0, 0]}
            ],
            "vertices": [
                {"source": 0, "target": 0, "delta": [0, 0, 1], "tags": ["core", "inner"]}
            ]
        }
    "#;
    let lattice_result: Result<Lattice, _> = data.parse();
    assert!(lattice_result.is_ok());
}

#[test]
fn lattice_will_fail_for_inconsistent_vertices() {
    let data = r#"
        {
            "size": [1, 1, 1],
            "sites": [
                {"kind": "Fe", "position": [0, 0, 0]}
            ],
            "vertices": [
                {"source": 0, "target": 1, "delta": [0, 0, 1], "tags": ["core", "inner"]}
            ]
        }
    "#;
    let lattice_result: Result<Lattice, _> = data.parse();
    assert!(lattice_result.is_err());
}

#[test]
fn lattice_will_fail_for_inconsistent_size() {
    let data = r#"
        {
            "size": [1, 1, -1],
            "sites": [
                {"kind": "Fe", "position": [0, 0, 0]}
            ],
            "vertices": []
        }
    "#;
    let lattice_result: Result<Lattice, _> = data.parse();
    assert!(lattice_result.is_err());
}