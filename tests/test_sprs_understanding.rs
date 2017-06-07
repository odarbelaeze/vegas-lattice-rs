extern crate sprs;

use sprs::CsMat;


#[test]
fn test_csr_matrix_creation() {
    let mat = CsMat::new(
        (3, 3),
        vec![0, 2, 4, 5],
        vec![0, 1, 0, 2, 2],
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
    );
    assert!(*mat.get(0, 0).unwrap() - 1.0 < 1e-12);
    assert!(*mat.get(2, 2).unwrap() - 5.0 < 1e-12);
}


#[test]
fn test_csr_outer_matrix_iteration() {
    let mat = CsMat::new(
        (3, 3),
        vec![0, 2, 4, 5],
        vec![0, 1, 0, 2, 2],
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
    );
    let nums : Vec<_> = mat.outer_view(0).unwrap().iter().map(|(j, &val)| (j, val)).collect();
    assert_eq!(nums, vec![(0, 1.0), (1, 2.0)]);
}


#[test]
fn test_csr_matrix_multiplication_by_constant() {
    let mut mat = CsMat::new(
        (3, 3),
        vec![0, 2, 4, 5],
        vec![0, 1, 0, 2, 2],
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
    );
    mat.scale(3.0);
    let nums : Vec<_> = mat.outer_view(0).unwrap().iter().map(|(j, &val)| (j, val)).collect();
    assert_eq!(nums, vec![(0, 3.0), (1, 6.0)]);
}


#[test]
fn test_csr_matrix_multiplication_by_constant_again() {
    let mat = CsMat::new(
        (3, 3),
        vec![0, 2, 4, 5],
        vec![0, 1, 0, 2, 2],
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
    );
    let mat: CsMat<_, _, _, _> = &mat * 3.0;
    let nums : Vec<_> = mat.outer_view(0).unwrap().iter().map(|(j, &val)| (j, val)).collect();
    assert_eq!(nums, vec![(0, 3.0), (1, 6.0)]);
}