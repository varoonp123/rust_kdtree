use rust_kdtree::kdtree::KDTree;

use proptest::collection::vec;
use proptest::prelude::*;

fn vec_of_vecs(size_of_each: usize) -> impl Strategy<Value = Vec<Vec<f64>>> {
    vec(vec(-100.0..100.0f64, size_of_each), 0..1000)
}
proptest! {
#[test]
fn test_kdtree_contains_prop(points in vec_of_vecs(3)){

        let dtree = KDTree::new(&mut points.clone());
        assert_eq!(dtree.size, points.len());
        for point in &points {
            assert_eq!(dtree.contains(&point), true);
        }

        for point in dtree {
            assert_eq!(points.contains(&point), true);
        }
    }
fn test_kdtree_add(points in vec_of_vecs(3)){

        let mut dtree = KDTree::new(&mut vec![]);
        for point in &points{
            dtree.add(point.clone());
        }
        assert_eq!(dtree.size, points.len());
        for point in &points {
            assert_eq!(dtree.contains(&point), true);
        }

        for point in dtree {
            assert_eq!(points.contains(&point), true);
        }
    }
}
