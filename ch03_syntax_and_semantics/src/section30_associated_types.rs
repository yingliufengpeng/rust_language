#[cfg(test)]
mod tests {
    use std::fmt;
    use std::fmt::Formatter;

    trait Graph2<N, E> {
        fn has_edge(&self, _: &N, _: &E) -> bool;

        fn edges(&self, _: &N) -> Vec<E>;
    }

    trait Graph {
        type N: fmt::Display;
        type E;

        fn has_edge(&self, _: &Self::N, _: &Self::E) -> bool;

        fn edges(&self, _: Self::N) -> Vec<Self::E>;
    }


    struct Node;

    struct Edge;

    struct MyGraph;

    impl fmt::Display for Node {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Node")
        }
    }

    impl Graph for MyGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self, _: &Self::N, _: &Self::E) -> bool {
            true
        }

        fn edges(&self, _: Self::N) -> Vec<Self::E> {
            vec![]
        }
    }

    fn distance<G: Graph + ?Sized>(graph: &G, start: &G::N, end: &G::E) -> u32 {
        0
    }

    #[test]
    fn test() {

        let graph = MyGraph;

        let obj = Box::new(graph) as Box<dyn Graph<N=Node, E=Edge>>;

        let distance = distance(&*obj, &Node , &Edge);

        println!("{}", distance);

    }
}
