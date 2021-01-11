//extern crate constraint_derive;

use crusp_derive_graph::crusp_lazy_graph;
use crusp_graph::*;
use std::fmt::Debug;

/*
struct NoAction {}
struct NoAfc {}
imp^l ...
...
...
*/
/*

trait VarStats<VarId> {
    fn actions() -> ActionHandlers<VarId>;
    fn afc() -> AfcHandler<VarId>;
    fn chb() -> ChbHHandler<VarId>;
}

// Let the branch implementors defined the bounds of varhanxler and var stats
// if no branch use action AcitonHandler is probably useless bounds...
// Or maybe not because used during constraint propagation
trait Brancher<VarHandler, VarStats = NoStats> {
    fn branch(&mut self, variables: &VarHandler, stats: &VarStats)... -> Box<dyn Patch<Handler>>;
}

// problem is thant Space might have multiple type of vars so multiple type of VarHandlmer
trait Space {
    fn variables() -> VariableHandler<??????>;
    fn actions() -> ActionHandlers<??????>;
    fn afc() -> AfcHandler<?????>;
}

// Implies different id type by variable type
pub trait VariableContainerHandler<Var>
where
    Var: VariableContainer,
{
    type View: VariableContainerView;

    fn get_mut(&mut self, view: &Self::View) -> &mut Var;
    fn get(&self, view: &Self::View) -> &Var;

    /*fn action(&self, view: &Self::View) -> f32;
    fn action_mut(&self, view: &Self::View) -> &mut f32;
    fn afc(&self, view: &Self::View) -> &f32;
    fn afc_mut(&self, view: &Self::View) -> &mut f32;
    fn increment_afc(&self, view: &Self::View)  -> () {
        .....
    }
    fn update_actions(&mut self, view: &Self::view, decay: f32, recorder: EventRecorder<Var::IdType>) -> () {
        let var_id = self.get(&view).id();
        let mut action = self.action_mut(view):
        if recorder.peek_change(var_id) {
            *action += 1;
        } else {
            *action = decay * *action;
        }
    }*/
}
trait ActionHandlerBuilder<VarId> {
    fn add_action(VarId, decay);
}

trait ActionHandler {
    fn update_actions(&mut self, view: &Self::view, decay: f32, recorder: EventRecorder<Var::IdType>) -> ();
    fn action(var_id) -> f32;
}

// Can create array of size nvariables and do not use map if faster
struct MultipleActionHandler<VarIdType> {
ActionHandler...
....
...
}

struct ActionHandler<VarIdType> {
    actions_variables: Rc<Vec<VID, Atomic<usize>>>>;
}
// AFC needs to save failmure of constraints too but not too difficult
trait AFCHandler<VarId> {
    // only on failure
    update_afc(VarId: id);
}

impl CstFn<VarId> for AFCHandler<VarId> where VarId: Variable...  {

}

struct Graph {

}

impl Graph {
    visit_all_ins_for_out<Visitor>(&self, out: OutNode, f: Visitor)
        where Visitor: Visit<IN1> + Visit<IN2> +
    {}
}
trait CstFn<T> {
    fn apply(&mut self, t: T);
}

fn cst_var<F:CstFn<VT1>+CstFn<VT2>+...>(&self, f: &mut F) {
    self.in1.cst_var(f);
}

fn afc_updater() {
    let cid = ...;
    let cst = csts.get(cid);
    match cst.propagate(....) {
        CstState::Success(...) => {

        },
        CstState::Failure(...) => {
            // won't work because vid might have multiple types
            // one type per variable type
            // event if update_afc can work on multiple type
            // the type of lambda can not change during cst_var call
            graph.cst_var(cid, afc_handler);
        }
    }
}

struct Space {
    //action_handleres: acthandler;
    //afc_hanlders.......
}
*/
#[derive(PartialEq, Eq, std::hash::Hash, std::cmp::PartialOrd, std::cmp::Ord,Clone,  Copy, Debug)]
pub struct OutNode {
    idx: usize,
}
impl GraphNode for OutNode {}

#[derive(Copy, Clone, Debug)]
pub struct OutEvent {
    val: i32,
}
impl Nullable for OutEvent {
    fn is_null(&self) -> bool {
        self.val == 0
    }
    fn null() -> Self {
        OutEvent {
            val: 0
        }
    }
    fn nullify(&mut self) -> Self {
        let prev = *self;
        *self = Self::null();
        prev
    }
}
impl Mergeable for OutEvent {
    fn merge(&self, rhs: Self) -> Self {
        let ret = self.val | rhs.val;
        OutEvent {
            val: ret
        }
    }
}
impl Subsumed for OutEvent {
    fn is_subsumed_under(&self, rhs: &Self) -> bool{
        true
    }
}
impl GraphEvent for OutEvent {}

#[derive(PartialEq, Eq, std::hash::Hash, std::cmp::PartialOrd, std::cmp::Ord, Clone, Copy, Debug)]
pub struct InNode1 {
    idx: usize,
}
impl GraphNode for InNode1 {}


#[derive(Copy, Clone, Debug)]
pub struct InEvent1 {
    val: i32,
}
impl Nullable for InEvent1 {
    fn is_null(&self) -> bool {
        self.val == 0
    }
    fn null() -> Self {
        InEvent1 {
            val: 0
        }
    }
    fn nullify(&mut self) -> Self {
        let prev = *self;
        *self = Self::null();
        prev
    }
}
impl Mergeable for InEvent1 {
    fn merge(&self, rhs: Self) -> Self {
        let ret = self.val | rhs.val;
        InEvent1 {
            val: ret
        }
    }
}
impl Subsumed for InEvent1 {
    fn is_subsumed_under(&self, rhs: &Self) -> bool{
        true
    }
}
impl GraphEvent for InEvent1 {}

#[derive(PartialEq, Eq, std::hash::Hash, std::cmp::PartialOrd, std::cmp::Ord, Clone, Copy, Debug)]
pub struct InNode2 {
    idx: usize,
}
impl GraphNode for InNode2 {}


#[derive(Copy, Clone, Debug)]
pub struct InEvent2 {
    val: i32,
}
impl Nullable for InEvent2 {
    fn is_null(&self) -> bool {
        self.val == 0
    }
    fn null() -> Self {
        InEvent2 {
            val: 0
        }
    }
    fn nullify(&mut self) -> Self {
        let prev = *self;
        *self = Self::null();
        prev
    }
}
impl Mergeable for InEvent2 {
    fn merge(&self, rhs: Self) -> Self {
        let ret = self.val | rhs.val;
        InEvent2 {
            val: ret
        }
    }
}
impl Subsumed for InEvent2 {
    fn is_subsumed_under(&self, rhs: &Self) -> bool{
        true
    }
}
impl GraphEvent for InEvent2 {}

//on failure mode
// add a statistics generics ot the graph
// save failures
//#[crusp_lazy_graph(statistics)]
#[crusp_lazy_graph]
struct GraphName<Statist>
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
    in2: (InNode2, InEvent2),
    /*#[input]
    in2: (u32, f64),
    #[input]
    in3: (i64, bool),*/
}

// save actions & inactions

#[derive(Debug)]
struct MyVisitor {
    n1: usize,
    n2: usize,
}

impl MyVisitor {
    fn new() -> MyVisitor {
        MyVisitor {
            n1: 0,
            n2: 0,
        }
    }
}

impl VisitMut<InNode1> for MyVisitor {
    fn visit_mut(&mut self, _t: &InNode1) {
        self.n1 += 1;
    }
}

impl VisitMut<InNode2> for MyVisitor {
    fn visit_mut(&mut self, _t: &InNode2) {
        self.n2 += 1;
    }
}

pub fn main() {
    let mut visitor = MyVisitor::new();
    let oe0 = OutEvent{val: 0};
    let oe1 = OutEvent{val: 1};
    let oe2 = OutEvent{val: 2};
    let on0 = OutNode{idx: 0};
    let on1 = OutNode{idx: 1};
    let on2 = OutNode{idx: 2};
    let in1 = InNode1{idx: 0};
    let ie1 = InEvent1{val: 1};
    let in12 = InNode1{idx: 1};
    let ie12 = InEvent1{val: 2};
    let in2 = InNode2{idx: 0};
    // WARN: null events are ignored
    let ie2 = InEvent2{val: 1};
    let mut graph = GraphName::builder();
    graph.add_event(&on0, &oe0, &in1, &ie1, 0i64);
    graph.add_event(&on1, &oe1, &in12, &ie12, 2i64);
    graph.add_event(&on2, &oe2, &in2, &ie2, 1i64);
    let mut graph = graph.finalize();
    println!("{:?}", visitor);
    graph.visit_all_in_nodes(&on0, &mut visitor);
    println!("{:?}", visitor);
    graph.notify(&in1, &ie1);
    let event = graph.collect_and_pop();
    println!("{:?}", event);
    // 1 - 2 -1 - 0
    graph.notify(&in1, &ie1);
    graph.notify(&in12, &ie12);
    graph.notify(&in2, &ie2);
    let event = graph.collect_and_pop();
    println!("{:?}", event);
    let event = graph.collect_and_pop();
    println!("{:?}", event);
    graph.notify(&in12, &ie12);
    let event = graph.collect_and_pop();
    println!("{:?}", event);
    let event = graph.collect_and_pop();
    println!("{:?}", event);
//    println!("{:?}", event);
//    println!("{:?}", event);

    println!("****");
}
