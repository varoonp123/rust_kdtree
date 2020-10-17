use std::cmp;
use std::iter;
/*
 * Currently, most of these structs are generic over T, the point type.
 *
 */
#[derive(Default, Debug)]
pub struct Node<T> {
    //T needs to be an iterable. Each element of T needs to impl PartialOrd and I need to be able
    //to add and square these (binary operations). A KDTree only makes sense for an affine space,
    //so I am fine making it only for Euclidean space.
    pub point: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub depth: usize,
}

struct KDTree {
    //Questions:
    //1. Should this be generic over f64?
    //2. Should this be generic over the point representation? If so, what trait must it satisfy
    //   (clearly it probably needs to be some (sized) sequence of float types.
    //3. Nightly Rust currently supports const generics, which would let me establish the size of
    //   each vector/the dimension of the kdtree at compile time, which would be nice.
    root: Option<Node<Vec<f64>>>,
    size: usize,
}

struct TreeIter<T> {
    stack: Vec<T>,
}

impl<T> TreeIter<T> {
    fn new(node: Node<T>) -> TreeIter<T> {
        let mut tree_iter = TreeIter { stack: vec![] };
        tree_iter._new_helper(node);

        tree_iter
    }

    fn _new_helper(&mut self, node: Node<T>) {
        if let Some(left_node) = node.left {
            self._new_helper(*left_node);
        }
        self.stack.push(node.point);
        if let Some(right_node) = node.right {
            self._new_helper(*right_node);
        }
    }
}
impl<T> Iterator for TreeIter<T> {
    //Placeholder implementation
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.stack.pop()
    }
}
struct TreeIter2<T> {
    stack: Vec<T>,
}

impl KDTree {
    fn new(points: &mut Vec<Vec<f64>>) -> KDTree {
        if points.len() == 0 {
            return KDTree {
                root: None,
                size: 0,
            };
        }

        KDTree {
            root: Some(KDTree::_new_helper(points, 0)),
            size: points.len(),
        }
    }

    fn _new_helper(points: &mut Vec<Vec<f64>>, depth: usize) -> Node<Vec<f64>> {
        // Currently this function assumes that each point in points has the same number fo
        // elements.
        if points.len() == 1 {
            return Node {
                point: points[0].clone(),
                left: None,
                right: None,
                depth: depth,
            };
        }
        let dimension = points[0].len();
        let axis = depth % dimension;
        let median_index: usize = points.len() / 2;
        // Sort in place, hence the mutable reference
        points.sort_by(|pt1, pt2| {
            pt1[axis]
                .partial_cmp(&pt2[axis])
                .unwrap_or(cmp::Ordering::Equal)
        });
        let left = &mut points[0..median_index].to_vec();
        let right = &mut points[median_index + 1..].to_vec();
        Node {
            point: points[median_index].clone(),
            left: Some(Box::new(KDTree::_new_helper(left, depth + 1))),
            right: Some(Box::new(KDTree::_new_helper(right, depth + 1))),
            depth: depth,
        }
    }
    fn iter(self) -> impl Iterator<Item = Vec<f64>> {
        match self.root {
            Some(_root) => TreeIter::new(_root),
            None => TreeIter { stack: vec![] },
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_kdtree_creation() {
        assert_eq!(1, 1);
    }
}
