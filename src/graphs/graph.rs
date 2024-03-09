use super::list::List;

pub struct Graph<V> {
    pub vertices: List<V>,
    pub edges: List<(V, V)>,
}

// constructor:
// - new
// - union

// queries:
//  - neighbors
//  - search (single node): bfs/dfs
//  - search (double node): dijkstras, bellman-ford, johnsons

// sort
//  - topological sort

impl<V> Graph<V> {
    fn new() -> Self {
        todo!()
    }

    fn union(&self, other: Self) -> Self {
        todo!()
    }

    fn neighbors(&self, v: V) -> List<(V, V)> {
        todo!()
    }

    fn single_node_search(&self, v: V) -> bool {
        todo!()
    }

    fn double_node_search(&self, start: V, found: fn(V) -> bool) -> Option<List<V>> {
        todo!()
    }

    fn top_sort(&self) {
        todo!()
    }
}
