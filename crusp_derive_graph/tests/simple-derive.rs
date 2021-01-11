//extern crate constraint_derive;

use crusp_derive_graph::crusp_lazy_graph;

#[derive(Eq + std::hash::Hash + std::cmp::PartialOrd + Copy)]
struct OutNode {
    idx: usize,
}
impl GraphNode for OutNode {}

struct OutEvent {

}

#[derive(Eq + std::hash::Hash + std::cmp::PartialOrd + Copy)]
struct InNode1 {
    idx: usize,
}
impl GraphNode for InNode1 {}

struct InEvent1 {

}


#[crusp_lazy_graph]
struct GraphName
// not necessary
//    where
//        OutNode: GraphNode,
//        OutEvent: GraphEvent,
{
    #[output]
    out: (OutNode, OutEvent),
    #[input]
    in1: (InNode1, InEvent1),
    #[input]
    in2: (u32, f64),
    #[input]
    in3: (i64, bool),
}

pub fn main() {

}
