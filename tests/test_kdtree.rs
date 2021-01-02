use ordered_float::{NotNan, OrderedFloat};
use rust_kdtree::kdtree::{distance_sq, KDTree, NDPoint};
use std::collections::BTreeSet;

use proptest::collection::vec;
use proptest::prelude::*;
use proptest::strategy::Strategy;

fn vec_of_vecs(size_of_each: usize) -> impl Strategy<Value = Vec<Vec<NotNan<f64>>>> {
    vec(
        vec(
            (-100.0..100.0f64).prop_map(|f| NotNan::new(f).unwrap()),
            size_of_each,
        ),
        0..1000,
    )
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
#[test]
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
#[test]
fn test_kdtree_nneighbors(points in vec_of_vecs(3), nneighbors in 1..10usize){
    if points.len() == 0{
        return Ok(());
    }
    let target = points[0].clone();
    let mut other_points = points[1..].to_vec();

    let mut dtree = KDTree::new(&mut vec![]);
    for point in &other_points{
        dtree.add(point.clone());
    }

    other_points.sort_by_key(|point| OrderedFloat(distance_sq(&target, &point)));

    let farthest_nneighbor = other_points.iter().take(nneighbors).last().unwrap();
    let nneighbors_ = other_points.iter().filter(|point| distance_sq(&target, &point) <= distance_sq(&target, &farthest_nneighbor)).map(|x| x.clone()).collect::<BTreeSet<NDPoint>>();

    let got = dtree.query(&target, nneighbors).into_iter().collect::<BTreeSet<NDPoint>>();
    //assert!(got <= nneighbors_);
    assert_eq!(got.difference(&nneighbors_).map(|x| x.clone()).collect::<Vec<NDPoint>>(), Vec::<NDPoint>::new(), "nneighbors: {}, target: {:?}, tree: {:?}", nneighbors, target, other_points);

    }
}
