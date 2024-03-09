use super::list::List;

trait Graph<V> {
    fn new() -> Self;

    fn union(&self, other: Self) -> Self;

    fn neighbors(&self, v: V) -> List<(V, V)>;

    fn single_node_search(&self, v: V) -> bool;

    fn double_node_search(&self, start: V, found: fn(V) -> bool) -> Option<List<V>>;

    fn top_sort(&self);
}

#[derive(Default, Clone, PartialEq)]
pub struct AssociationListGraph<V> {
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

impl<V> Graph<V> for AssociationListGraph<V> {
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

impl<V> std::fmt::Debug for AssociationListGraph<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Graph")
            .field("vertices", &self.vertices)
            .field("edges", &self.edges)
            .finish()
    }
}

impl<V> IntoIterator for AssociationListGraph<V>
where
    V: Ord,
{
    type Item;

    type IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
