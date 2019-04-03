#[macro_use]
extern crate itertools;

use itertools::{cons_tuples, Itertools};

#[test]
fn itertools_over_a_two_by_two_lattice() {
    assert_eq!(
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        iproduct!(0..2, 0..2).collect::<Vec<_>>()
    );
}

#[test]
fn cartesian_products_by_hand() {
    assert_eq!(
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
        (0..2).cartesian_product(0..2).collect::<Vec<_>>()
    );
}

#[test]
fn triple_cartesian_products_by_hand() {
    assert_eq!(
        iproduct!(0..2, 0..2, 0..2).collect::<Vec<_>>(),
        cons_tuples((0..2).cartesian_product(0..2).cartesian_product(0..2)).collect::<Vec<_>>()
    );
}
