use std::collections::HashMap;
use std::collections::HashSet;

struct NodeIdx1(usize);
struct NodeIdx2(usize);
pub struct TwoNodesGraph<Node1, Node2, Edge1, Edge2> {
    nodes1: HashMap<Node1, NodeIdx1>,
    nodes2: HashMap<Node2, NodeIdx2>,

    edges1: Vec<(Edge1, Node2)>,
    edges2: Vec<(Edge2, Node1)>,
}
