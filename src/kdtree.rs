#![allow(dead_code)]
use std::cmp;

type NDPoint = Vec<f64>;
#[derive(Default, Debug, Clone)]
pub struct Node {
    //T needs to be an iterable. Each element of T needs to impl PartialOrd and I need to be able
    //to add and square these (binary operations). A KDTree only makes sense for an affine space,
    //so I am fine making it only for Euclidean space.
    pub point: NDPoint,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub depth: usize,
}

pub struct KDTree {
    //Questions:
    //1. Should this be generic over f64?
    //2. Should this be generic over the point representation? If so, what trait must it satisfy
    //   (clearly it probably needs to be some (sized) sequence of float types.
    //3. Nightly Rust currently supports const generics, which would let me establish the size of
    //   each vector/the dimension of the kdtree at compile time, which would be nice.
    root: Option<Node>,
    pub size: usize,
}

pub struct TreeIter {
    stack: Vec<Node>,
}

impl TreeIter {
    fn new(node: Node) -> TreeIter {
        TreeIter { stack: vec![node] }
    }
}
impl Iterator for TreeIter {
    //Placeholder implementation
    type Item = NDPoint;
    fn next(&mut self) -> Option<NDPoint> {
        if self.stack.is_empty() {
            return None;
        }
        let result = self.stack.pop().unwrap();
        if let Some(left_node) = result.left {
            self.stack.push(*left_node);
        }
        if let Some(right_node) = result.right {
            self.stack.push(*right_node);
        }
        Some(result.point)
    }
}

impl IntoIterator for Node {
    type Item = NDPoint;
    type IntoIter = TreeIter;
    fn into_iter(self) -> Self::IntoIter {
        TreeIter::new(self)
    }
}

impl Node {
    fn from_point(target: NDPoint, depth: usize) -> Node {
        Node {
            point: target,
            left: None,
            right: None,
            depth: depth,
        }
    }
    fn contains_helper(&self, target: &NDPoint) -> bool {
        let axis = self.depth % self.point.len();
        match &self.point[axis].partial_cmp(&target[axis]) {
            Some(cmp::Ordering::Less) => self
                .right
                .as_ref()
                .map_or(false, |x| x.contains_helper(target)),
            Some(cmp::Ordering::Greater) => self
                .left
                .as_ref()
                .map_or(false, |x| x.contains_helper(target)),
            _ => {
                (&self.point == target)
                    || self
                        .left
                        .as_ref()
                        .map_or(false, |x| x.contains_helper(target))
                    || self
                        .right
                        .as_ref()
                        .map_or(false, |x| x.contains_helper(target))
            }
        }
    }

    fn add_helper(&mut self, target: NDPoint) {
        let axis = self.depth % self.point.len();
        if self.point[axis].ge(&target[axis]) {
            match &mut self.left {
                Some(node) => node.add_helper(target),
                None => self.left = Some(Box::new(Node::from_point(target, self.depth + 1))),
            }
        } else {
            match &mut self.right {
                Some(node) => node.add_helper(target),
                None => self.right = Some(Box::new(Node::from_point(target, self.depth + 1))),
            }
        }
    }
}
impl KDTree {
    pub fn new(points: &mut Vec<NDPoint>) -> KDTree {
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

    fn _new_helper(points: &mut Vec<NDPoint>, depth: usize) -> Node {
        // Currently this function assumes that each point in points has the same number fo
        // elements.
        if points.len() == 0 {
            return Node {
                point: points[0].clone(),
                left: None,
                right: None,
                depth: depth,
            };
        }
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
        let right = &mut points[median_index..].to_vec();
        Node {
            point: points[median_index].clone(),
            left: Some(Box::new(KDTree::_new_helper(left, depth + 1))),
            right: Some(Box::new(KDTree::_new_helper(right, depth + 1))),
            depth: depth,
        }
    }
    pub fn iter(self) -> impl Iterator<Item = NDPoint> {
        match self.root {
            Some(_root) => TreeIter::new(_root),
            None => TreeIter { stack: vec![] },
        }
    }

    pub fn contains(&self, target: &NDPoint) -> bool {
        match &self.root {
            Some(r) => r.contains_helper(target),
            None => false,
        }
    }
    pub fn add(&mut self, target: NDPoint) {
        self.size += 1;
        match &mut self.root {
            Some(node) => node.add_helper(target),
            None => {
                self.root = Some(Node::from_point(target, 0));
            }
        }
    }
}
impl IntoIterator for KDTree {
    type Item = NDPoint;
    type IntoIter = TreeIter;
    fn into_iter(self) -> Self::IntoIter {
        if let Some(node) = self.root {
            return TreeIter::new(node);
        }
        TreeIter { stack: vec![] }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kdtree_contains() {
        let points = vec![vec![1., 2., 3.], vec![2., 3., 4.], vec![3., 4., 5.]];
        let kdtree = KDTree::new(&mut points.clone());
        assert_eq!(kdtree.size, 3);
        for point in &points {
            assert_eq!(kdtree.contains(&point), true);
        }

        for point in kdtree {
            assert_eq!(points.contains(&point), true);
        }
    }
}
