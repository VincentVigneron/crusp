#![allow(clippy::single_match, clippy::match_same_arms, clippy::match_ref_pats,
         clippy::clone_on_ref_ptr, clippy::needless_pass_by_value,
         clippy::redundant_field_names, clippy::redundant_pattern)]
#![deny(clippy::wrong_pub_self_convention, clippy::used_underscore_binding,
        clippy::similar_names, clippy::pub_enum_variant_names,
        //clippy::missing_docs_in_private_items,
        clippy::non_ascii_literal, clippy::unicode_not_nfc,
        clippy::unwrap_used,
        clippy::option_map_or_none, clippy::map_unwrap_or,
        clippy::filter_map,
        clippy::shadow_unrelated, clippy::shadow_reuse, clippy::shadow_same,
        clippy::int_plus_one, clippy::string_add_assign, clippy::if_not_else,
        clippy::invalid_upcast_comparisons,
        clippy::cast_precision_loss, clippy::cast_lossless,
        clippy::cast_possible_wrap, clippy::cast_possible_truncation,
        clippy::mutex_integer, clippy::mut_mut, clippy::items_after_statements,
        clippy::print_stdout, clippy::mem_forget, clippy::maybe_infinite_iter)]

use priority_queue::PriorityQueue;

use std::rc::Rc;
use std::hash::Hash;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Debug;

// TODO MAY BR SPLIT EVENT HANDLER AND GRAPH CONSTRAINT LIST OF VARIABLES

// TODO(vincent): variables: active failure count: almost ok
// TODO(vincent): variables: actions: almost ok
// TODO(vincent): chb read more: ?????

// TODO(vincent): variables: last change
// TODO(vincent): disable constraints
// TODO(vincent): add constraint
// TODO(vincent): Add builder then create proc macro for graph auto generation
// TODO(vincent): rmv useless pub

// Schema:
// Add event of type A
// Push event in queau of event A graph
// Add event of type B
// Push event in queau of event B graph
// when peek
// Check all events of all queues and gather output events based on them
// return the out_event with the highest priority

// create default event ofr evetn taht only support one propagate function

pub trait Nullable {
    fn is_null(&self) -> bool;
    fn null() -> Self;
    fn nullify(&mut self) -> Self; // return previous value
}

pub trait GraphNode: Eq + Hash + std::cmp::PartialOrd + std::cmp::Ord + Copy + Debug{}
pub trait GraphEvent:  Mergeable + Subsumed + Nullable + Debug{}

pub trait Mergeable: Copy {
    fn merge(&self, rhs: Self) -> Self;
}

pub trait Subsumed {
    fn is_subsumed_under(&self, rhs: &Self) -> bool;
}

pub trait InputEventRegister<InNode, InEvent, Output> {
    // update pred out if already existing one
    //fn register<Pred>(&mut self, in_node: &InNode, in_event: &InEvent, out: &Output, filter) -> bool;
    //fn unregister<Pred>(&mut self, in_node: &InNode, in_event: &InEvent, out: &Output, filter: Pred)
    //    where Pred: Fn(&Output)-> bool;
}

pub trait InputEventHandler<InNode, InEvent> where
    InNode: GraphNode,
    InEvent: GraphEvent,
{
    /// Notify incoming event to the handler. Do not necessarly trigger the event.
    fn notify(&mut self, node: &InNode, event: &InEvent) -> bool;
    // Tells if any non null event occurs  for the node `node` since the last call to peek_change
    //fn peek_change(&mut self, node: &InNode) -> bool;
}

pub trait InOutEventHandlerBuilder<OutNode, OutEvent, InNode, InEvent> where
    OutNode: GraphNode,
    OutEvent: GraphEvent,
    InNode: GraphNode,
    InEvent: GraphEvent,
{
    fn add_event(&mut self, out_node: &OutNode, out_event: &OutEvent, in_node: &InNode, in_event: &InEvent, cost: i64);
}

pub trait OutputEventHandler<OutNode, OutEvent>  where
    OutNode: GraphNode,
    OutEvent: GraphEvent,
{
    fn collect_and_pop(&mut self) -> Option<(OutNode, OutEvent)>;
}

pub trait VisitMut<T> {
    fn visit_mut(&mut self, t: &T);
}


pub trait GraphBuilder<OutNode, InNode> where
    OutNode: GraphNode,
    InNode: GraphNode,
{
    fn add_node(&mut self, out_node: &OutNode, in_node: &InNode);
}

pub trait VisitOutputsNode<OutNode, InNode> where
    OutNode: GraphNode,
    InNode: GraphNode,
{
    fn visit_in_nodes<Visitor>(&self, out_node: &OutNode, visitor: &mut Visitor)
        where Visitor: VisitMut<InNode>;
}

pub trait VisitAllOutputsNode<OutNode, Visitor> where
    OutNode: GraphNode,
{
    fn visit_all_in_nodes(&self, out_node: &OutNode, visitor: &mut Visitor);
}


struct EventLink<InEvent: GraphEvent, Output> {
    in_event: InEvent,
    out: Output,
}

pub struct LazyInputEventGraphBuilder<InNode, InEvent, Output> where
    InNode: GraphNode,
    InEvent: GraphEvent,
{
    in_nodes: HashMap<InNode, usize>,
    in_events: Vec<Vec<EventLink<InEvent, Output>>>,
}

pub struct LazyInputEventGraph<InNode, InEvent, Output> where
    InNode: GraphNode,
    InEvent: GraphEvent,
{
    in_nodes: HashMap<InNode, usize>,
    in_events: Vec<Vec<EventLink<InEvent, Output>>>,
}

impl <InNode, InEvent, Output> LazyInputEventGraphBuilder<InNode, InEvent, Output> where
    InNode: GraphNode,
    InEvent: GraphEvent,
{
    pub fn new() -> Self {
        LazyInputEventGraphBuilder {
            in_nodes: HashMap::new(),
            in_events: Vec::new(),
        }
    }

    #[allow(clippy::shadow_reuse)]
    pub fn add_event(&mut self, node: InNode, event: InEvent, out: Output) {
        let idx = self.in_nodes.len();
        let idx = *self.in_nodes.entry(node).or_insert(idx);
        let link = EventLink {
            in_event: event,
            out: out,
        };
        if idx >= self.in_events.len() {
            self.in_events.push(vec![link]);
        } else {
            self.in_events[idx].push(link);
        }
    }

    pub fn finalize(self) -> LazyInputEventGraph<InNode, InEvent, Output> {
        LazyInputEventGraph {
            in_nodes: self.in_nodes,
            in_events: self.in_events,
        }
    }
}

pub struct LazyInputEventHandler<InNode, InEvent, Output>  where
    InNode: GraphNode,
    InEvent: GraphEvent,
{
    graph: Rc<LazyInputEventGraph<InNode, InEvent, Output>>,
    events: Vec<(InNode, InEvent)>,
//    changes: HahshMap<InNode, bool>,
}

impl <InNode, InEvent, Output> InputEventHandler<InNode, InEvent> for LazyInputEventHandler<InNode, InEvent, Output> where
    InNode: GraphNode,
    InEvent: GraphEvent,
{
    fn notify(&mut self, node: &InNode, event: &InEvent) -> bool {
        if event.is_null() {
            return false;
        }
        match self.events.last_mut() {
            Some(&mut (l_node, ref mut l_evt)) if l_node == *node => {
                *l_evt = l_evt.merge(*event);
            }
            _ => {
                self.events.push((*node, *event));
            }
        }
        true
    }

    /*fn peek_change(&mut self, node: &InNode) -> bool {
        unimplemented!()
        match self.changes.get_mut(node) {
            Some(ref mut ch) => {
                let ret = *ch;
                *ch = true;
                ret
            },
            None => false
        }
    }*/
}

impl <InNode, InEvent, Output> LazyInputEventHandler<InNode, InEvent, Output> where
    InNode: GraphNode,
    InEvent: GraphEvent,
{
    pub fn builder() -> LazyInputEventGraphBuilder<InNode, InEvent, Output> {
        LazyInputEventGraphBuilder::new()
    }

    pub fn new(graph: LazyInputEventGraph<InNode, InEvent, Output>) -> Self {
        LazyInputEventHandler {
            graph: Rc::new(graph),
            events: Vec::new(),
        }
    }

    pub fn trigger_events<F>(&mut self, mut process: F) where
        F: FnMut(&Output)
    {
        if self.events.is_empty() {
            return;
        }
        self.events.sort_unstable_by(|lhs, rhs| lhs.0.partial_cmp(&rhs.0).expect("Comparable input nodes"));
        // consumes events here
        let events: Vec<_> = self.events.drain(..).collect();
        let mut events = events.into_iter();
        let (mut curr_node, mut curr_event) = events.next().expect("At least one element");
        for (in_node, in_event) in events {
            if curr_node == in_node {
                curr_event = curr_event.merge(in_event);
            } else {
                self.process_in_event(&curr_node, &curr_event, &mut process);
                curr_node = in_node;
                curr_event = in_event;
            }
        }
        self.process_in_event(&curr_node, &curr_event, &mut process);
    }

    #[allow(clippy::filter_map)]
    pub fn process_in_event<F>(&self, in_node: &InNode, in_event: &InEvent, process: &mut F) where
        F: FnMut(&Output)
    {
        // /self.changes.entry(in_node).or_insert(true);
        // TODO: rmv bound checks
        let in_idx = *self.graph.in_nodes.get(in_node).expect("Existing input node");
        self.graph.in_events[in_idx].iter()
            .filter(|&out_event| in_event.is_subsumed_under(&out_event.in_event.merge(*in_event)))
            .map(|link| &link.out)
            .for_each(|out| process(out));
    }
}


pub struct AdjacentListGraphBuilder<SrcNode, DstNode>
    where SrcNode: GraphNode,
        DstNode: GraphNode
{
    map_to_idx: HashMap<SrcNode, usize>,
    ins: Vec<Vec<DstNode>>,
}

impl <SrcNode, DstNode> AdjacentListGraphBuilder<SrcNode, DstNode>
    where SrcNode: GraphNode,
        DstNode: GraphNode
{
    fn new() -> Self {
        AdjacentListGraphBuilder {
            map_to_idx: HashMap::new(),
            ins: Vec::new(),
        }
    }

    pub fn finalize(mut self) -> AdjacentListGraph<SrcNode, DstNode> {
        &mut self.ins[..].sort();
        self.ins.dedup();
        AdjacentListGraph {
            map_to_idx: self.map_to_idx,
            ins: self.ins,
        }
    }
}

impl <OutNode, InNode> GraphBuilder<OutNode, InNode> for AdjacentListGraphBuilder<OutNode, InNode> where
    OutNode: GraphNode,
    InNode: GraphNode,
{
    fn add_node(&mut self, out_node: &OutNode, in_node: &InNode) {
        let len = self.ins.len();
        let idx = *self.map_to_idx.entry(*out_node).or_insert(len);
        if idx == len {
            self.ins.push(Vec::new());
        }
        self.ins[idx].push(*in_node);
    }
}

pub struct AdjacentListGraph<SrcNode, DstNode>
    where SrcNode: GraphNode,
        DstNode: GraphNode
{
    map_to_idx: HashMap<SrcNode, usize>,
    ins: Vec<Vec<DstNode>>,
}

impl <SrcNode, DstNode> AdjacentListGraph<SrcNode, DstNode> where
    SrcNode: GraphNode,
    DstNode: GraphNode,
{
    pub fn builder() -> AdjacentListGraphBuilder<SrcNode, DstNode> {
        AdjacentListGraphBuilder::new()
    }
}

impl <SrcNode, DstNode> VisitOutputsNode<SrcNode, DstNode> for AdjacentListGraph<SrcNode, DstNode> where
    SrcNode: GraphNode,
    DstNode: GraphNode,
{
    /*fn visit_all_in_nodes<Visitor>(&self, visitor: &mut Visitor)
        where Visitor: VisitMut<DstNode>
    {
        if let Some(idx) = self.map_to_idx.get(out_node) {
            let ins = self.ins[*idx].iter();
            for v in ins {
                visitor.visit_mut(&v);
            }
        }
    }*/

    fn visit_in_nodes<Visitor>(&self, out_node: &SrcNode, visitor: &mut Visitor)
        where Visitor: VisitMut<DstNode>
    {
        if let Some(idx) = self.map_to_idx.get(out_node) {
            let ins = self.ins[*idx].iter();
            for v in ins {
                visitor.visit_mut(&v);
            }
        }
    }
}


pub struct OutCostEventLink<OutEvent: GraphEvent> {
    idx: usize,
    event: OutEvent,
    cost: i64,
}

impl <OutEvent: GraphEvent> OutCostEventLink<OutEvent> {
    pub fn new(idx: usize, event: OutEvent, cost: i64) -> Self {
        OutCostEventLink {
            idx: idx,
            event: event,
            cost: cost,
        }
    }
}


pub struct HandlerOutputBuilder<OutNode, OutEvent> where
    OutNode: GraphNode,
    OutEvent: GraphEvent,
{
    outs: Vec<OutNode>,
    out_map: HashMap<OutNode, usize>,
    _event: std::marker::PhantomData<OutEvent>,
}

impl <OutNode, OutEvent> HandlerOutputBuilder<OutNode, OutEvent> where
    OutNode: GraphNode,
    OutEvent: GraphEvent,
{
    pub fn new() -> Self {
        HandlerOutputBuilder {
            outs: Vec::new(),
            out_map: HashMap::new(),
            _event: std::marker::PhantomData,
        }
    }

    pub fn add_node(&mut self, node: OutNode) -> usize {
        let idx = match self.out_map.entry(node) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let idx = *e.insert(self.outs.len());
                self.outs.push(node);
                idx
            }
        };
        idx
    }

    pub fn finalize(self) -> HandlerOutput<OutNode, OutEvent> {
        let len = self.outs.len();
        HandlerOutput {
            mode: vec![OutEvent::null(); len],
            outs: self.outs,
            queue: PriorityQueue::new(),
        }
    }
}

pub struct HandlerOutput<OutNode, OutEvent> where
    OutNode: GraphNode,
    OutEvent: GraphEvent,
{
    mode: Vec<OutEvent>,
    outs: Vec<OutNode>,
    queue: PriorityQueue<usize, i64>,
}

impl <OutNode, OutEvent> HandlerOutput<OutNode, OutEvent> where
    OutNode: GraphNode,
    OutEvent: GraphEvent,
{
    pub fn builder() -> HandlerOutputBuilder<OutNode, OutEvent> {
        HandlerOutputBuilder::new()
    }

    pub fn collect_out_event(&mut self, out: &OutCostEventLink<OutEvent>) {
        unsafe {
            let out_idx = out.idx;
            self.queue.push(out_idx, out.cost);
            let curr_state = self.mode.get_unchecked_mut(out_idx);
            *curr_state = curr_state.merge(out.event);
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<(OutNode, OutEvent)> {
        let (out_idx, _cost) = self.queue.pop()?;
        let event = self.mode[out_idx].nullify();
        Some((self.outs[out_idx], event))
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::{Debug};
    use std::ops::*;

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    struct VariableId {
        id: usize,
    }

    #[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
    struct ConstraintId {
        id: usize,
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug)]
    struct InputEvent {
        event: u8,
    }

    impl BitAnd for InputEvent {
        type Output = InputEvent;

        fn bitand(self, rhs: Self) -> Self::Output {
            InputEvent{
                event: self.event & rhs.event
            }
        }
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug)]
    struct OutputEvent {
        event: u8,
    }

    impl BitOr for OutputEvent {
        type Output = OutputEvent;

        fn bitor(self, rhs: Self) -> Self::Output {
            OutputEvent{
                event: self.event | rhs.event
            }
        }
    }


    impl Nullable for OutputEvent {
        fn null() -> Self {
            OutputEvent{ event: 0 }
        }
    }

    #[test]
    fn test_graph() {
        let v0 = VariableId{id: 0};
        let v1 = VariableId{id: 1};
        let c0 = ConstraintId{id: 0};
        let c1 = ConstraintId{id: 1};
        let in1 = InputEvent{event: 0b0001};
        let in2 = InputEvent{event: 0b0010};
        let in3 = InputEvent{event: 0b0011};
        let out1 = OutputEvent{event: 0b0001};
        let out2 = OutputEvent{event: 0b0010};
        let out3 = OutputEvent{event: 0b0011};
        let mut builder = PounderedEventGraph::<VariableId, ConstraintId, InputEvent, OutputEvent>::builder();
        builder.add_node(v0, c0,
            InNodeEvents{
                in_event: in1,
                out_event: out1,
                cost: 10i64,
            }
        );
        builder.add_node(v0, c0,
            InNodeEvents{
                in_event: in2,
                out_event: out2,
                cost: 5i64,
            }
        );
        builder.add_node(v1, c1,
            InNodeEvents{
                in_event: in2,
                out_event: out2,
                cost: 7i64,
            }
        );
        let mut graph_events = builder.finalize();

        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v0, &in1);
        assert_eq!(graph_events.pop(), Some((c0, out1)));
        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v0, &in2);
        assert_eq!(graph_events.pop(), Some((c0, out2)));
        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v0, &in1);
        graph_events.input_event(&v0, &in2);
        assert_eq!(graph_events.pop(), Some((c0, out3)));
        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v0, &in3);
        assert_eq!(graph_events.pop(), Some((c0, out3)));
        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v1, &in2);
        graph_events.input_event(&v0, &in1);
        assert_eq!(graph_events.pop(), Some((c0, out1)));
        assert_eq!(graph_events.pop(), Some((c1, out2)));
        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v0, &in1);
        graph_events.input_event(&v1, &in2);
        assert_eq!(graph_events.pop(), Some((c0, out1)));
        assert_eq!(graph_events.pop(), Some((c1, out2)));
        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v0, &in2);
        graph_events.input_event(&v1, &in2);
        assert_eq!(graph_events.pop(), Some((c1, out2)));
        assert_eq!(graph_events.pop(), Some((c0, out2)));
        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v1, &in2);
        graph_events.input_event(&v0, &in2);
        assert_eq!(graph_events.pop(), Some((c1, out2)));
        assert_eq!(graph_events.pop(), Some((c0, out2)));
        assert_eq!(graph_events.pop(), None);

        graph_events.input_event(&v0, &in2);
        graph_events.input_event(&v1, &in2);
        graph_events.input_event(&v0, &in1);
        assert_eq!(graph_events.pop(), Some((c0, out3)));
        assert_eq!(graph_events.pop(), Some((c1, out2)));
        assert_eq!(graph_events.pop(), None);
    }
}
*/
