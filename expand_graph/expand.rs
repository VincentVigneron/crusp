#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use crusp_derive_graph::crusp_lazy_graph;
use crusp_graph::*;
pub struct OutNode {
    idx: usize,
}
impl ::core::marker::StructuralPartialEq for OutNode {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for OutNode {
    #[inline]
    fn eq(&self, other: &OutNode) -> bool {
        match *other {
            OutNode {
                idx: ref __self_1_0,
            } => match *self {
                OutNode {
                    idx: ref __self_0_0,
                } => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &OutNode) -> bool {
        match *other {
            OutNode {
                idx: ref __self_1_0,
            } => match *self {
                OutNode {
                    idx: ref __self_0_0,
                } => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl ::core::marker::StructuralEq for OutNode {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for OutNode {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<usize>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::hash::Hash for OutNode {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            OutNode {
                idx: ref __self_0_0,
            } => ::core::hash::Hash::hash(&(*__self_0_0), state),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialOrd for OutNode {
    #[inline]
    fn partial_cmp(&self, other: &OutNode) -> ::core::option::Option<::core::cmp::Ordering> {
        match *other {
            OutNode {
                idx: ref __self_1_0,
            } => match *self {
                OutNode {
                    idx: ref __self_0_0,
                } => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &OutNode) -> bool {
        match *other {
            OutNode {
                idx: ref __self_1_0,
            } => match *self {
                OutNode {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Greater,
                    ) == ::core::cmp::Ordering::Less
                }
            },
        }
    }
    #[inline]
    fn le(&self, other: &OutNode) -> bool {
        match *other {
            OutNode {
                idx: ref __self_1_0,
            } => match *self {
                OutNode {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Greater,
                    ) != ::core::cmp::Ordering::Greater
                }
            },
        }
    }
    #[inline]
    fn gt(&self, other: &OutNode) -> bool {
        match *other {
            OutNode {
                idx: ref __self_1_0,
            } => match *self {
                OutNode {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Less,
                    ) == ::core::cmp::Ordering::Greater
                }
            },
        }
    }
    #[inline]
    fn ge(&self, other: &OutNode) -> bool {
        match *other {
            OutNode {
                idx: ref __self_1_0,
            } => match *self {
                OutNode {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Less,
                    ) != ::core::cmp::Ordering::Less
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Ord for OutNode {
    #[inline]
    fn cmp(&self, other: &OutNode) -> ::core::cmp::Ordering {
        match *other {
            OutNode {
                idx: ref __self_1_0,
            } => match *self {
                OutNode {
                    idx: ref __self_0_0,
                } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for OutNode {
    #[inline]
    fn clone(&self) -> OutNode {
        {
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for OutNode {}
impl GraphNode for OutNode {}
pub struct OutEvent {
    val: i32,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for OutEvent {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for OutEvent {
    #[inline]
    fn clone(&self) -> OutEvent {
        {
            let _: ::core::clone::AssertParamIsClone<i32>;
            *self
        }
    }
}
impl Nullable for OutEvent {
    fn is_null(&self) -> bool {
        self.val == 0
    }
    fn null() -> Self {
        OutEvent { val: 0 }
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
        OutEvent { val: ret }
    }
}
impl Subsumed for OutEvent {
    fn is_subsumed_under(&self, rhs: &Self) -> bool {
        true
    }
}
impl GraphEvent for OutEvent {}
pub struct InNode1 {
    idx: usize,
}
impl ::core::marker::StructuralPartialEq for InNode1 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for InNode1 {
    #[inline]
    fn eq(&self, other: &InNode1) -> bool {
        match *other {
            InNode1 {
                idx: ref __self_1_0,
            } => match *self {
                InNode1 {
                    idx: ref __self_0_0,
                } => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &InNode1) -> bool {
        match *other {
            InNode1 {
                idx: ref __self_1_0,
            } => match *self {
                InNode1 {
                    idx: ref __self_0_0,
                } => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl ::core::marker::StructuralEq for InNode1 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for InNode1 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<usize>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::hash::Hash for InNode1 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            InNode1 {
                idx: ref __self_0_0,
            } => ::core::hash::Hash::hash(&(*__self_0_0), state),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialOrd for InNode1 {
    #[inline]
    fn partial_cmp(&self, other: &InNode1) -> ::core::option::Option<::core::cmp::Ordering> {
        match *other {
            InNode1 {
                idx: ref __self_1_0,
            } => match *self {
                InNode1 {
                    idx: ref __self_0_0,
                } => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &InNode1) -> bool {
        match *other {
            InNode1 {
                idx: ref __self_1_0,
            } => match *self {
                InNode1 {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Greater,
                    ) == ::core::cmp::Ordering::Less
                }
            },
        }
    }
    #[inline]
    fn le(&self, other: &InNode1) -> bool {
        match *other {
            InNode1 {
                idx: ref __self_1_0,
            } => match *self {
                InNode1 {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Greater,
                    ) != ::core::cmp::Ordering::Greater
                }
            },
        }
    }
    #[inline]
    fn gt(&self, other: &InNode1) -> bool {
        match *other {
            InNode1 {
                idx: ref __self_1_0,
            } => match *self {
                InNode1 {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Less,
                    ) == ::core::cmp::Ordering::Greater
                }
            },
        }
    }
    #[inline]
    fn ge(&self, other: &InNode1) -> bool {
        match *other {
            InNode1 {
                idx: ref __self_1_0,
            } => match *self {
                InNode1 {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Less,
                    ) != ::core::cmp::Ordering::Less
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Ord for InNode1 {
    #[inline]
    fn cmp(&self, other: &InNode1) -> ::core::cmp::Ordering {
        match *other {
            InNode1 {
                idx: ref __self_1_0,
            } => match *self {
                InNode1 {
                    idx: ref __self_0_0,
                } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for InNode1 {
    #[inline]
    fn clone(&self) -> InNode1 {
        {
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for InNode1 {}
impl GraphNode for InNode1 {}
pub struct InEvent1 {
    val: i32,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for InEvent1 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for InEvent1 {
    #[inline]
    fn clone(&self) -> InEvent1 {
        {
            let _: ::core::clone::AssertParamIsClone<i32>;
            *self
        }
    }
}
impl Nullable for InEvent1 {
    fn is_null(&self) -> bool {
        self.val == 0
    }
    fn null() -> Self {
        InEvent1 { val: 0 }
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
        InEvent1 { val: ret }
    }
}
impl Subsumed for InEvent1 {
    fn is_subsumed_under(&self, rhs: &Self) -> bool {
        true
    }
}
impl GraphEvent for InEvent1 {}
pub struct InNode2 {
    idx: usize,
}
impl ::core::marker::StructuralPartialEq for InNode2 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for InNode2 {
    #[inline]
    fn eq(&self, other: &InNode2) -> bool {
        match *other {
            InNode2 {
                idx: ref __self_1_0,
            } => match *self {
                InNode2 {
                    idx: ref __self_0_0,
                } => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &InNode2) -> bool {
        match *other {
            InNode2 {
                idx: ref __self_1_0,
            } => match *self {
                InNode2 {
                    idx: ref __self_0_0,
                } => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl ::core::marker::StructuralEq for InNode2 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for InNode2 {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<usize>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::hash::Hash for InNode2 {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            InNode2 {
                idx: ref __self_0_0,
            } => ::core::hash::Hash::hash(&(*__self_0_0), state),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialOrd for InNode2 {
    #[inline]
    fn partial_cmp(&self, other: &InNode2) -> ::core::option::Option<::core::cmp::Ordering> {
        match *other {
            InNode2 {
                idx: ref __self_1_0,
            } => match *self {
                InNode2 {
                    idx: ref __self_0_0,
                } => match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                    }
                    cmp => cmp,
                },
            },
        }
    }
    #[inline]
    fn lt(&self, other: &InNode2) -> bool {
        match *other {
            InNode2 {
                idx: ref __self_1_0,
            } => match *self {
                InNode2 {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Greater,
                    ) == ::core::cmp::Ordering::Less
                }
            },
        }
    }
    #[inline]
    fn le(&self, other: &InNode2) -> bool {
        match *other {
            InNode2 {
                idx: ref __self_1_0,
            } => match *self {
                InNode2 {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Greater,
                    ) != ::core::cmp::Ordering::Greater
                }
            },
        }
    }
    #[inline]
    fn gt(&self, other: &InNode2) -> bool {
        match *other {
            InNode2 {
                idx: ref __self_1_0,
            } => match *self {
                InNode2 {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Less,
                    ) == ::core::cmp::Ordering::Greater
                }
            },
        }
    }
    #[inline]
    fn ge(&self, other: &InNode2) -> bool {
        match *other {
            InNode2 {
                idx: ref __self_1_0,
            } => match *self {
                InNode2 {
                    idx: ref __self_0_0,
                } => {
                    ::core::option::Option::unwrap_or(
                        ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0), &(*__self_1_0)),
                        ::core::cmp::Ordering::Less,
                    ) != ::core::cmp::Ordering::Less
                }
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Ord for InNode2 {
    #[inline]
    fn cmp(&self, other: &InNode2) -> ::core::cmp::Ordering {
        match *other {
            InNode2 {
                idx: ref __self_1_0,
            } => match *self {
                InNode2 {
                    idx: ref __self_0_0,
                } => match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                    ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                    cmp => cmp,
                },
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for InNode2 {
    #[inline]
    fn clone(&self) -> InNode2 {
        {
            let _: ::core::clone::AssertParamIsClone<usize>;
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for InNode2 {}
impl GraphNode for InNode2 {}
pub struct InEvent2 {
    val: i32,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for InEvent2 {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for InEvent2 {
    #[inline]
    fn clone(&self) -> InEvent2 {
        {
            let _: ::core::clone::AssertParamIsClone<i32>;
            *self
        }
    }
}
impl Nullable for InEvent2 {
    fn is_null(&self) -> bool {
        self.val == 0
    }
    fn null() -> Self {
        InEvent2 { val: 0 }
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
        InEvent2 { val: ret }
    }
}
impl Subsumed for InEvent2 {
    fn is_subsumed_under(&self, rhs: &Self) -> bool {
        true
    }
}
impl GraphEvent for InEvent2 {}
struct GraphNameBuilder {
    out: ::crusp_graph::HandlerOutputBuilder<OutNode, OutEvent>,
    in1: ::crusp_graph::LazyInputEventGraphBuilder<
        InNode1,
        InEvent1,
        ::crusp_graph::OutCostEventLink<OutEvent>,
    >,
    in2: ::crusp_graph::LazyInputEventGraphBuilder<
        InNode2,
        InEvent2,
        ::crusp_graph::OutCostEventLink<OutEvent>,
    >,
    __crusp__rev_in1: ::crusp_graph::AdjacentListGraphBuilder<OutNode, InNode1>,
    __crusp__rev_in2: ::crusp_graph::AdjacentListGraphBuilder<OutNode, InNode2>,
}
struct GraphName {
    out: ::crusp_graph::HandlerOutput<OutNode, OutEvent>,
    in1: ::crusp_graph::LazyInputEventHandler<
        InNode1,
        InEvent1,
        ::crusp_graph::OutCostEventLink<OutEvent>,
    >,
    in2: ::crusp_graph::LazyInputEventHandler<
        InNode2,
        InEvent2,
        ::crusp_graph::OutCostEventLink<OutEvent>,
    >,
    __crusp__rev_in1: ::std::rc::Rc<::crusp_graph::AdjacentListGraph<OutNode, InNode1>>,
    __crusp__rev_in2: ::std::rc::Rc<::crusp_graph::AdjacentListGraph<OutNode, InNode2>>,
}
impl ::crusp_graph::VisitOutputsNode<OutNode, InNode1> for GraphName {
    fn visit_in_nodes<Visitor>(&self, out_node: &OutNode, visitor: &mut Visitor)
    where
        Visitor: VisitMut<InNode1>,
    {
        self.__crusp__rev_in1.visit_in_nodes(out_node, visitor);
    }
}
impl ::crusp_graph::VisitOutputsNode<OutNode, InNode2> for GraphName {
    fn visit_in_nodes<Visitor>(&self, out_node: &OutNode, visitor: &mut Visitor)
    where
        Visitor: VisitMut<InNode2>,
    {
        self.__crusp__rev_in2.visit_in_nodes(out_node, visitor);
    }
}
impl<Visitor> ::crusp_graph::VisitAllOutputsNode<OutNode, Visitor> for GraphName
where
    Visitor: VisitMut<InNode1>,
    Visitor: VisitMut<InNode2>,
{
    fn visit_all_in_nodes(&self, out_node: &OutNode, visitor: &mut Visitor) {
        self.__crusp__rev_in1.visit_in_nodes(out_node, visitor);
        self.__crusp__rev_in2.visit_in_nodes(out_node, visitor);
    }
}
impl ::crusp_graph::InputEventHandler<InNode1, InEvent1> for GraphName {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn notify(&mut self, in_node: &InNode1, in_event: &InEvent1) -> bool {
        if self.in1.notify(in_node, in_event) {
            true
        } else {
            false
        }
    }
}
impl ::crusp_graph::InputEventHandler<InNode2, InEvent2> for GraphName {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn notify(&mut self, in_node: &InNode2, in_event: &InEvent2) -> bool {
        if self.in2.notify(in_node, in_event) {
            true
        } else {
            false
        }
    }
}
impl ::crusp_graph::InOutEventHandlerBuilder<OutNode, OutEvent, InNode1, InEvent1>
    for GraphNameBuilder
{
    fn add_event(
        &mut self,
        out_node: &OutNode,
        out_event: &OutEvent,
        in_node: &InNode1,
        in_event: &InEvent1,
        cost: i64,
    ) {
        let idx = self.out.add_node(*out_node);
        let out = <::crusp_graph::OutCostEventLink<OutEvent>>::new(idx, *out_event, cost);
        self.in1.add_event(*in_node, *in_event, out);
        self.__crusp__rev_in1.add_node(out_node, in_node);
    }
}
impl ::crusp_graph::InOutEventHandlerBuilder<OutNode, OutEvent, InNode2, InEvent2>
    for GraphNameBuilder
{
    fn add_event(
        &mut self,
        out_node: &OutNode,
        out_event: &OutEvent,
        in_node: &InNode2,
        in_event: &InEvent2,
        cost: i64,
    ) {
        let idx = self.out.add_node(*out_node);
        let out = <::crusp_graph::OutCostEventLink<OutEvent>>::new(idx, *out_event, cost);
        self.in2.add_event(*in_node, *in_event, out);
        self.__crusp__rev_in2.add_node(out_node, in_node);
    }
}
impl GraphNameBuilder {
    pub fn new() -> Self {
        GraphNameBuilder {
            in1: <::crusp_graph::LazyInputEventHandler<
                InNode1,
                InEvent1,
                ::crusp_graph::OutCostEventLink<OutEvent>,
            >>::builder(),
            in2: <::crusp_graph::LazyInputEventHandler<
                InNode2,
                InEvent2,
                ::crusp_graph::OutCostEventLink<OutEvent>,
            >>::builder(),
            __crusp__rev_in1: <::crusp_graph::AdjacentListGraph<OutNode, InNode1>>::builder(),
            __crusp__rev_in2: <::crusp_graph::AdjacentListGraph<OutNode, InNode2>>::builder(),
            out: <::crusp_graph::HandlerOutput<OutNode, OutEvent>>::builder(),
        }
    }
    pub fn finalize(self) -> GraphName {
        GraphName {
            in1: <::crusp_graph::LazyInputEventHandler<
                InNode1,
                InEvent1,
                ::crusp_graph::OutCostEventLink<OutEvent>,
            >>::new(self.in1.finalize()),
            in2: <::crusp_graph::LazyInputEventHandler<
                InNode2,
                InEvent2,
                ::crusp_graph::OutCostEventLink<OutEvent>,
            >>::new(self.in2.finalize()),
            __crusp__rev_in1: ::std::rc::Rc::new(self.__crusp__rev_in1.finalize()),
            __crusp__rev_in2: ::std::rc::Rc::new(self.__crusp__rev_in2.finalize()),
            out: self.out.finalize(),
        }
    }
}
impl GraphName {
    pub fn builder() -> GraphNameBuilder {
        <GraphNameBuilder>::new()
    }
    #[allow(clippy::type_complexity)]
    #[inline]
    pub fn split_in_out(
        &mut self,
    ) -> (
        &mut ::crusp_graph::HandlerOutput<OutNode, OutEvent>,
        &mut ::crusp_graph::LazyInputEventHandler<
            InNode1,
            InEvent1,
            ::crusp_graph::OutCostEventLink<OutEvent>,
        >,
        &mut ::crusp_graph::LazyInputEventHandler<
            InNode2,
            InEvent2,
            ::crusp_graph::OutCostEventLink<OutEvent>,
        >,
    ) {
        (
            unsafe { &mut *((&mut self.out) as *mut _) },
            unsafe { &mut *((&mut self.in1) as *mut _) },
            unsafe { &mut *((&mut self.in2) as *mut _) },
        )
    }
}
impl ::crusp_graph::OutputEventHandler<OutNode, OutEvent> for GraphName {
    fn collect_and_pop(&mut self) -> Option<(OutNode, OutEvent)> {
        let (__crusp__outs, in1, in2) = self.split_in_out();
        in1.trigger_events(|__crusp__out| __crusp__outs.collect_out_event(__crusp__out));
        in2.trigger_events(|__crusp__out| __crusp__outs.collect_out_event(__crusp__out));
        self.out.pop()
    }
}
pub fn main() {}
