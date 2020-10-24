use rust_kdtree::kdtree::{KDTree};

use proptest::prelude::*;
use proptest::collection::vec;

fn vec_of_vecs(size_of_each: usize) -> impl Strategy<Value = Vec<Vec<f64>>>{
    vec(vec(-100.0..100.0f64, size_of_each), 0..1000)
    
}
proptest!{
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
}
