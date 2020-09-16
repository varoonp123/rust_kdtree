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

struct KDTree<T> {
    root: Option<Box<Node<T>>>,
}

impl <T> Iterator for Node<T>{
    //Placeholder implementation
    type Item = T;
    fn next(&mut self) -> Option<T>{
        None
        
    }
}

impl KDTree<Vec<isize>> {
    fn new(points: &mut Vec<Vec<isize>>) -> KDTree<Vec<isize>> {
        if points.len() == 0 {
            return KDTree::<Vec<isize>> { root: None };
        }

        KDTree::<Vec<isize>> {
            root: Some(Box::new(KDTree::<Vec<isize>>::_new_helper(points, 0))),
        }
    }

    fn _new_helper(points: &mut Vec<Vec<isize>>, depth: usize) -> Node<Vec<isize>> {
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
        points.sort_by_key(|pt| pt[axis]);
        let left = &mut points[0..median_index].to_vec();
        let right = &mut points[median_index + 1..].to_vec();
        Node {
            point: points[median_index].clone(),
            left: Some(Box::new(KDTree::<Vec<isize>>::_new_helper(left, depth + 1))),
            right: Some(Box::new(KDTree::<Vec<isize>>::_new_helper(
                right,
                depth + 1,
            ))),
            depth: depth,
        }
    }
}
