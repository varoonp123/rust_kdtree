mod kdtree;
fn main() {
    let node: kdtree::Node = kdtree::Node {
        left:None,
        right:None,
        point: vec![1., 2., 3.],
        depth: 0,
    };
    println!("{:?}", &node);
}
