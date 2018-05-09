use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub trait Subsumed {
    fn is_subsumed_under(&self, val: &Self) -> bool;
}

//struct NodeIdx1(usize);
//struct NodeIdx2(usize);

// Maybe externalise event to avoid unecessary clone
// of the graph
#[derive(Clone)]
pub struct TwoNodesGraph<Node1, Node2, Edge>
where
    Node1: Eq + Hash + Copy,
    Node2: Eq + Hash + Copy,
    Edge: Eq + Subsumed,
{
    //nodes1: HashSet<Node1>,
    //nodes2: HashSet<Node2>,
    edges1: HashMap<Node1, Vec<(Edge, Vec<Node2>)>>,
    edges2: HashMap<Node2, HashSet<Node1>>,

    events: Vec<Event<Node1, Node2, Edge>>,
}

#[derive(Clone)]
pub struct Event<Node1, Node2, Cause>
where
    Node1: Eq + Hash + Copy,
    Node2: Eq + Hash + Copy,
    Cause: Eq + Subsumed,
{
    src: Node1,
    from: Node2,
    event: Cause,
}
impl<Node1, Node2, Cause> Event<Node1, Node2, Cause>
where
    Node1: Eq + Hash + Copy,
    Node2: Eq + Hash + Copy,
    Cause: Eq + Subsumed,
{
    pub fn new(src: Node1, from: Node2, event: Cause) -> Self {
        Event {
            src: src,
            from: from,
            event: event,
        }
    }
}

impl<Node1, Node2, Cause> Subsumed for Event<Node1, Node2, Cause>
where
    Node1: Eq + Hash + Copy,
    Node2: Eq + Hash + Copy,
    Cause: Eq + Subsumed,
{
    fn is_subsumed_under(&self, val: &Event<Node1, Node2, Cause>) -> bool {
        self.src == val.src && self.event.is_subsumed_under(&val.event)
    }
}

impl<Node1, Node2, Edge> TwoNodesGraph<Node1, Node2, Edge>
where
    Node1: Eq + Hash + Copy,
    Node2: Eq + Hash + Copy,
    Edge: Eq + Subsumed,
{
    pub fn new() -> Self {
        TwoNodesGraph {
            //nodes1: HashSet::new(),
            //nodes2: HashSet::new(),
            edges1: HashMap::new(),
            edges2: HashMap::new(),
            events: Vec::new(),
        }
    }

    // add node 2 to node 2
    pub fn insert_node1_to_node2(&mut self, src: Node1, label: Edge, dst: Node2) {
        let edge = self.edges1.entry(src).or_insert(vec![]);
        let position = edge.iter().position(|&(ref key, _)| *key == label);
        let position = match position {
            Some(position) => position,
            None => {
                edge.push((label, vec![]));
                edge.len() - 1
            }
        };
        let &mut (_, ref mut nodes) = unsafe { edge.get_unchecked_mut(position) };
        if !nodes.contains(&dst) {
            nodes.push(dst);
            self.edges2.entry(dst).or_insert(HashSet::new()).insert(src);
        }
    }

    pub fn add_event(&mut self, src: Node1, from: Node2, cause: Edge) {
        let event = Event::new(src, from, cause);
        let position = self.events
            .iter()
            .position(|ev| ev.is_subsumed_under(&event));
        if let Some(position) = position {
            self.events[position] = event;
        } else {
            self.events.push(event);
        }
    }

    // Better naming
    // Replace vector by iterator (at least for Node1) it will allow for a constraints
    // to skip unecessary comutation if it wants to propagate all at each
    // change. It requires to split events and graph to please the borrows checker. The lifetime of
    // an event should be lesser than the lifetime of the graph.
    pub fn events(&mut self) -> Option<Vec<(Node2, Vec<Node1>)>> {
        let mut nodes = HashMap::new();
        for event in self.events.drain(0..) {
            let Event { src, from, event } = event;
            let succs = self.edges1.get(&src);
            if succs.is_none() {
                continue;
            }
            let succs = succs
                .unwrap()
                .iter()
                .find(|&entry| {
                    let (ref label, _) = *entry;
                    (*label).is_subsumed_under(&event)
                })
                .map(|&(_, ref succs)| {
                    succs
                        .iter()
                        .filter(|&succ| *succ != from)
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .unwrap_or(Vec::new());
            for succ in succs.into_iter() {
                nodes.entry(succ).or_insert(HashSet::new()).insert(src);
            }
        }
        if nodes.is_empty() {
            None
        } else {
            Some(
                nodes
                    .into_iter()
                    .map(|(node2, nodes1)| (node2, nodes1.into_iter().collect()))
                    .collect(),
            )
        }
    }
}
