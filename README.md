# rust_kdtree

This is an implementation of a KDTree created for the purposes of expanding my
understanding of Rust ownership, traits, and project structure. It should
absolutely not be used for anything important. 

## Features to Implement
1. n-dimensional range/brick queries
2. Nearest neighbor queries
3. Radius queries
4. Containment checking
5. Iteration
6. Property testing for all of this

## Ideas for Improvement
1. Const generics -- Right now, there are lots of places where index out of
   bounds errors can occur because the data associated to each node is a
   `Vec<f64>`. It would be better to define the dimension of the tree (the
   dimension of Euclidean space that is partitioned by the tree) at compile
   time and store the points internally as arrays rather than as vectors. This
   might look something like `let tree: KDTree<6> = KDTree<6>::new(vec![])`. I
   am not doing this now because const generics are currently on a feature in
   nightly Rust and I only want this project to use stable Rust features.
2. Generics over the float type. It is plausible that there could be different
   applications of a KDTree that require different notions of floats, for
   example, by implementing comparison in a different way. For example
   [CGAL](https://www.cgal.org/) uses its own KDTree and has to be precise
   about how it compares floats/doubles. The traits describing how this should
   work are not straight forward. In principle, something like a totally
   ordered ring would be nice (enough structure to calculate something like a
   Euclidean norm) but the Rust float types include infinities and NaN and
   consequently only implement `PartialOrd`.
   [Num](https://github.com/rust-num/num) might be useful for this.
