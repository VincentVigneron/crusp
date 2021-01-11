extern crate constraint_derive;

use std::ops::*;



use constraint_derive::{constraint};

//pub trait TestTrait {}
/*
#[var]
fn invoke1() {}

#[derive(Constraint)]
pub struct A {
    #[var] x: TestTrait,
    #[val] y: i32,
    #[state] z: i32,
}
*/
/*
trait MyTrait<T>
{}

pub trait Variable {}

pub trait VarView<Type> {}

pub trait VariableHandler<Type, View>
    where View: VarView<Type>,
        Type: Variable,
{
    fn get_variable_mut(&mut self, view: &View) -> &mut Type;
}

pub trait Constraint<Vars>
{
    fn init(&mut self, vars: &mut Vars) -> () {

    }
    fn propagate(&mut self, vars: &mut Vars) -> ();
}
*/

struct VariableId;
struct ConstraintId;

trait VariableChangeUpdater {
    fn push(&mut self, vid: VariableId, event: Result<VariableState, Error>) -> Result<VariableState, Error>;
    fn push_change(&mut self, vid: VariableId, event: VariableState) -> Result<VariableState, Error>;
    fn push_error(&mut self, vid: VariableId, event: Error) -> Result<VariableState, Error>;
}

trait ConstraintChangeUpdater {
    fn push(&mut self, cid: ConstraintId, vid: VariableId, event: Result<VariableState, Error>) -> Result<ConstraintState, Error>;
    fn push_change(&mut self, cid: ConstraintId, vid: VariableId, event: VariableState) -> Result<ConstraintState, Error>;
    fn push_error(&mut self, cid: ConstraintId, vid: VariableId, event: Error) -> Result<ConstraintState, Error>;
}

struct EventGraph {

}

impl ConstraintChangeUpdater for EvenGraph {
    fn push(&mut self, cid: ConstraintId, vid: VariableId, event: Result<VariableState, Error>) -> Result<ConstraintState, Error> {
        unimplemented!()
    }
    fn push_change(&mut self, cid: ConstraintId, vid: VariableId, event: VariableState) -> Result<ConstraintState, Error> {
        unimplemented!()
    }
    fn push_error(&mut self, cid: ConstraintId, vid: VariableId, event: Error) -> Result<ConstraintState, Error> {
        unimplemented!()
    }
}

struct ConstraintEventGraph<'a, Events> where Events:  ConstraintChangeUpdater {
    event_graph: &'a mut Events,
    cid: ConstraintId,
}

impl<'a, Events> VariableChangeUpdater for ConstraintEventGraphw<'a, Events>
    where Events:  ConstraintChangeUpdater
{
    fn push(&mut self, vid: VariableId, event: Result<VariableState, Error>) -> Result<ConstraintState, Error> {
        self.event_graph(self.cid, vid, event)
    }
    fn push_change(&mut self, vid: VariableId, event: VariableState) -> Result<ConstraintState, Error> {
        self.event_graph(self.cid, vid, event)
    }
    fn push_error(&mut self, vid: VariableId, event: Error) -> Result<ConstraintState, Error> {
        self.event_graph(self.cid, vid, event)
    }
}

impl<VarHandler, Type, T1, T2, V1, V2> Constraint<VarHandler>
    for MyConstraintsImpl<Type, T1, T2, V1, V2>
    where
    VarHandler: VariableHandler<T1, V1> + VariableHandler<T2,V2>,
    Type: Ord + Eq,
    T1: BoundedDomain<Type, T2>,
    T2: BoundedDomain<Type, T1>,
    V1: VarView<T1>,
    V2: VarView<T2>,
{
    fn propagate<Events>(&mut self, vars: &mut VarHandler, events: &mut Events) -> ()
        where Events: ConstraintChangeUpdater
    {
        let  vars = self.view.to_vars(vars);
        let mut events = ConstraintEventGraph {
            event_graph: events,
            cid: self.cid,
        };
        self.cst.propagate(vars.lhs, vars.rhs, &mut events);
    }
}


// maybe put the register on the variable
// Register: saves varaible changes during propagation
trait Constraint<Variables, Register> {
    // OK
    fn propagate(&mut self, vars: &mut Variables, register: &mut Register) -> ();
    // OK
    fn box_clone(&self) -> Box<dyn Constraint<Variables>>;
    // OK
    fn initialise(
        &mut self,
        variables_handler: &mut Variables
    ) -> ();
    // NOK: How to pass it to the constraint imolementor?????
    fn prepare(&mut self, states: dyn Iterator<Item = VariableId>);
    // NOK
    fn result(&mut self) -> Box<dyn Iterator<Item = (VariableId, VariableState)>>;
    fn result_visit(&mut self, func: dyn  FnMut(&(VariableId, VariableState)) -> ())  -> ();
    // ALMOST OK
    fn dependencies(
        &self,
        variables: &Variables,
    ) -> Box<dyn Iterator<Item = (VariableId, VariableState)>>;
}

// TODO(checking):
// - double lifetime, etc.etc.etc.

#[constraint]
pub struct ConstraintState<T,LHS,RHS>
    where T: Ord+Eq,
    LHS: BoundedDomain<T, RHS>,
    RHS: BoundedDomain<T, LHS>,
{
    #[var(value)] x: LHS, //MyTrait<T>,
    #[var(bound)] y: RHS, //MyTrait<T>,
    //#[var] a: i32,
    #[val] u: T,
    #[state] v: T,
    #[vararray(value)]: i32,
}


/*
impl<T,LHS,RHS> ConstraintState<T,LHS,RHS>
where T: Ord+Eq,
LHS: BoundedDomain<T, RHS>,
RHS: BoundedDomain<T, LHS>,
 {
     fn new() -> () {

     }
         pub fn propagate<T1,T2, Register>(&mut self, lhs: &mut T1, rhs: &mut T2, reg: &mut Register) -> ()
             where T1: TestTrait, T2: TestTrait
         {
             register!(reg: lhs.lower_or_equal_than(rhs));
             // =>
             match lhs.lower_or_equal_than(rhs) {
                 (Bound, No) => {
                     reg.push_bound(lhs.id)
                 }
                 (Bound, Bound) => {
                     reg.push_bound(lhs.id)
                     reg.push_bound(rhs.id)
                 }
                 (Val, Val) => {
                     reg.push_val(lhs.id)
                     reg.push_val(rhs.id)
                 }
             }
         }

             pub fn propagate_no_register<T1,T2>(&mut self, lhs: &mut T1, rhs: &mut T2) -> ()
                 where T1: TestTrait, T2: TestTrait
             {
                let mut reg = NoReg{};
                self.propagate(lhs, rhs, &mut reg);
             }
 }
*/

/*
impl ConstraintState {
    pub fn propagate<T1,T2>(&mut self, lhs: &mut T1, rhs: &mut T2) -> ()
        where T1: TestTrait, T2: TestTrait
    {

    }
}
*/


pub trait Subsumed {
    fn is_subsumed_under(&self, val: &Self) -> bool;
}
// Describes the state of a variable after its domain is updated.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VariableState {
    /// If only the maximal bound of the variable has been updated.
    MaxBoundChange,
    /// If only the minimal bound of the variable has been updated.
    MinBoundChange,
    /// If both bounds of the variable has been updated.
    BoundsChange,
    /// If the domain has been change but not its bounds.
    ValuesChange,
    /// If no change occured.
    NoChange,
}

impl Subsumed for VariableState {
    /// # Subsomption relations
    /// * `MaxBoundChange` subsumed `BoundsChange`
    /// * `MinBoundChange` subsumed `BoundsChange`
    /// * `BoundsChange` subsumed `ValuesChange`
    /// * `ValuesChange` subsumed `NoChange`
    fn is_subsumed_under(&self, val: &Self) -> bool {
        match *self {
            VariableState::MaxBoundChange => *val == VariableState::MaxBoundChange,
            VariableState::MinBoundChange => *val == VariableState::MinBoundChange,
            VariableState::BoundsChange => {
                *val != VariableState::ValuesChange && *val != VariableState::NoChange
            }
            VariableState::ValuesChange => *val != VariableState::NoChange,
            VariableState::NoChange => true,
        }
    }
}

/// Represents an error that occured during variable domain update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableError {
    /// The domain of the variable is empty.
    DomainWipeout,
}

pub struct VariableId {}

pub trait Variable {}

pub trait Variable_<T> {
    fn id(&self) -> VariableId;
    fn is_affected(&self) -> bool;
    fn value(&self) -> Option<&T>;
}
pub trait ArrayOfVariable<T> {
    type Variable: Variable_<T>;
}

pub trait FiniteDomain<Type>: Variable {
    fn size(&self) -> usize;
}

pub trait OrderedDomain<Type>: FiniteDomain<Type>
where
    Type: Ord + Eq
{
    fn min(&self) -> Option<Type>;
    fn max(&self) -> Option<Type>;
    fn unchecked_min(&self) -> Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.min().expect(&error)
    }
    fn unchecked_max(&self) -> Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.max().expect(&error)
    }
    fn strict_upperbound<Register=NoOpRegister>(
        &mut self,
        ub: Type
    ) -> Result<VariableState, VariableError>;
    fn strict_upperbound_ref<Register=NoOpRegister>(
        &mut self,
        ub: Type,
        register: &mut Register
    ) -> Result<VariableState, VariableError>
    where Register: FnMut(&VariableId, VariableState) -> ()
    {
        let state = self.strict_upperbound(ub)?;
        // if state has cgange
//        reg.push(self.id(), state)
        Ok(state)
    }
    fn weak_upperbound(&mut self, ub: Type
    register: &mut Register)
        -> Result<VariableState, VariableError>;
    fn strict_lowerbound(
        &mut self,
        lb: Type,
    ) -> Result<VariableState, VariableError>;
    fn weak_lowerbound(&mut self, lb: Type)
        -> Result<VariableState, VariableError>;
}

// will require proper support for specialization
/*
 impl<Type, LHS, RHS> BoundedDomain<Type, RHS> for LHS where
    Type: Ord + Eq,
    LHS: OrderedDomain<Type>,
    RHS: OrderedDomain<Type>
{
    default fn less_than(
        &mut self,
        value: &mut RHS,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_upperbound(value.unchecked_max())?;
        let state_value = value.strict_lowerbound(self.unchecked_min())?;

        Ok((state_self, state_value))
    }
    default fn less_or_equal_than(
        &mut self,
        value: &mut RHS,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.weak_upperbound(value.unchecked_max())?;
        let state_value = value.weak_lowerbound(self.unchecked_min())?;

        Ok((state_self, state_value))
    }
    default fn greater_than(
        &mut self,
        value: &mut RHS,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_lowerbound(value.unchecked_min())?;
        let state_value = value.strict_upperbound(self.unchecked_max())?;

        Ok((state_self, state_value))
    }
    default fn greater_or_equal_than(
        &mut self,
        value: &mut RHS,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.weak_lowerbound(value.unchecked_min())?;
        let state_value = value.weak_upperbound(self.unchecked_max())?;

        Ok((state_self, state_value))
    }

    default fn equal_bounds(
        &mut self,
        value: &mut RHS,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let mut x = VariableState::NoChange;
        let mut y = VariableState::NoChange;
        loop {
            let x1 = self.less_or_equal_than(value)?.0;
            let y1 = self.greater_or_equal_than(value)?.1;
            if (x1 == VariableState::NoChange) && (y1 == VariableState::NoChange) {
                break;
            }
            if x1.is_subsumed_under(&x) {
                x = x1;
            }
            if y1.is_subsumed_under(&y) {
                y = y1;
            }
        }
        Ok((x,y))
    }
}
*/

pub trait BoundedDomain2<Type, T1, T2>:
    OrderedDomain<Type>
    + BoundedDomain<Type, T1>
    + BoundedDomain<Type, T2>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
{}

pub trait BoundedDomain3<Type, T1, T2, T3>:
    OrderedDomain<Type>
    + BoundedDomain<Type, T1>
    + BoundedDomain<Type, T2>
    + BoundedDomain<Type, T3>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
    T3: OrderedDomain<Type>,
{}

pub trait BoundedDomain4<Type, T1, T2, T3, T4>:
    OrderedDomain<Type>
    + BoundedDomain<Type, T1>
    + BoundedDomain<Type, T2>
    + BoundedDomain<Type, T3>
    + BoundedDomain<Type, T4>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
    T3: OrderedDomain<Type>,
    T4: OrderedDomain<Type>,
{}

pub trait BoundedDomain5<Type, T1, T2, T3, T4, T5>:
    OrderedDomain<Type>
    + BoundedDomain<Type, T1>
    + BoundedDomain<Type, T2>
    + BoundedDomain<Type, T3>
    + BoundedDomain<Type, T4>
    + BoundedDomain<Type, T5>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
    T3: OrderedDomain<Type>,
    T4: OrderedDomain<Type>,
    T5: OrderedDomain<Type>,
{}

pub trait BoundedDomain6<Type, T1, T2, T3, T4, T5, T6>:
    OrderedDomain<Type>
    + BoundedDomain<Type, T1>
    + BoundedDomain<Type, T2>
    + BoundedDomain<Type, T3>
    + BoundedDomain<Type, T4>
    + BoundedDomain<Type, T5>
    + BoundedDomain<Type, T6>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
    T3: OrderedDomain<Type>,
    T4: OrderedDomain<Type>,
    T5: OrderedDomain<Type>,
    T6: OrderedDomain<Type>,
{}

pub trait BoundedDomain7<Type, T1, T2, T3, T4, T5, T6, T7>:
    OrderedDomain<Type>
    + BoundedDomain<Type, T1>
    + BoundedDomain<Type, T2>
    + BoundedDomain<Type, T3>
    + BoundedDomain<Type, T4>
    + BoundedDomain<Type, T5>
    + BoundedDomain<Type, T6>
    + BoundedDomain<Type, T7>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
    T3: OrderedDomain<Type>,
    T4: OrderedDomain<Type>,
    T5: OrderedDomain<Type>,
    T6: OrderedDomain<Type>,
    T7: OrderedDomain<Type>,
{}

pub trait BoundedDomain8<Type, T1, T2, T3, T4, T5, T6, T7, T8>:
    OrderedDomain<Type>
    + BoundedDomain<Type, T1>
    + BoundedDomain<Type, T2>
    + BoundedDomain<Type, T3>
    + BoundedDomain<Type, T4>
    + BoundedDomain<Type, T5>
    + BoundedDomain<Type, T6>
    + BoundedDomain<Type, T7>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
    T3: OrderedDomain<Type>,
    T4: OrderedDomain<Type>,
    T5: OrderedDomain<Type>,
    T6: OrderedDomain<Type>,
    T7: OrderedDomain<Type>,
    T8: OrderedDomain<Type>,
{}

pub trait BoundedDomain9<Type, T1, T2, T3, T4, T5, T6, T7, T8, T9>:
    OrderedDomain<Type>
    + BoundedDomain<Type, T1>
    + BoundedDomain<Type, T2>
    + BoundedDomain<Type, T3>
    + BoundedDomain<Type, T4>
    + BoundedDomain<Type, T5>
    + BoundedDomain<Type, T6>
    + BoundedDomain<Type, T7>
    + BoundedDomain<Type, T8>
    + BoundedDomain<Type, T9>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
    T3: OrderedDomain<Type>,
    T4: OrderedDomain<Type>,
    T5: OrderedDomain<Type>,
    T6: OrderedDomain<Type>,
    T7: OrderedDomain<Type>,
    T8: OrderedDomain<Type>,
    T9: OrderedDomain<Type>,
{}

pub trait BoundedDomain10<Type, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>:
    OrderedDomain<Type>
    + BoundedDomain9<Type, T1, T2, T3, T4, T5, T6, T7, T8, T9>
    + BoundedDomain<Type, T10>
    where
    Type: Ord+Eq,
    T1: OrderedDomain<Type>,
    T2: OrderedDomain<Type>,
    T3: OrderedDomain<Type>,
    T4: OrderedDomain<Type>,
    T5: OrderedDomain<Type>,
    T6: OrderedDomain<Type>,
    T7: OrderedDomain<Type>,
    T8: OrderedDomain<Type>,
    T9: OrderedDomain<Type>,
    T10: OrderedDomain<Type>,
{}

pub trait BoundedDomain<Type, Other=Self>:  OrderedDomain<Type>
where
    Type: Ord + Eq,
    Other: OrderedDomain<Type>
{
    fn less_than(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_upperbound(value.unchecked_max())?;
        let state_value = value.strict_lowerbound(self.unchecked_min())?;

        Ok((state_self, state_value))
    }
    fn less_or_equal_than(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.weak_upperbound(value.unchecked_max())?;
        let state_value = value.weak_lowerbound(self.unchecked_min())?;

        Ok((state_self, state_value))
    }
    fn greater_than(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_lowerbound(value.unchecked_min())?;
        let state_value = value.strict_upperbound(self.unchecked_max())?;

        Ok((state_self, state_value))
    }
    fn greater_or_equal_than(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.weak_lowerbound(value.unchecked_min())?;
        let state_value = value.weak_upperbound(self.unchecked_max())?;

        Ok((state_self, state_value))
    }

    fn equal_bounds(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let mut x = VariableState::NoChange;
        let mut y = VariableState::NoChange;
        loop {
            let x1 = self.less_or_equal_than(value)?.0;
            let y1 = self.greater_or_equal_than(value)?.1;
            if (x1 == VariableState::NoChange) && (y1 == VariableState::NoChange) {
                break;
            }
            if x1.is_subsumed_under(&x) {
                x = x1;
            }
            if y1.is_subsumed_under(&y) {
                y = y1;
            }
        }
        Ok((x,y))
    }
}

impl BoundedDomain<i32> for IntVarBound<i32>
{
    fn less_than(
        &mut self,
        value: &mut IntVarBound<i32>,
    ) -> Result<(VariableState, VariableState), VariableError> {
        unimplemented!()
    }
    fn less_or_equal_than(
        &mut self,
        value: &mut IntVarBound<i32>,
    ) -> Result<(VariableState, VariableState), VariableError> {
        unimplemented!()
    }
    fn greater_than(
        &mut self,
        value: &mut IntVarBound<i32>,
    ) -> Result<(VariableState, VariableState), VariableError> {
        unimplemented!()
    }
    fn greater_or_equal_than(
        &mut self,
        value: &mut IntVarBound<i32>,
    ) -> Result<(VariableState, VariableState), VariableError> {
        unimplemented!()
    }
    fn equal_bounds(
        &mut self,
        value: &mut IntVarBound<i32>,
    ) -> Result<(VariableState, VariableState), VariableError> {
        unimplemented!()
    }
}


pub fn my_lower<A,B,C>(a: &mut A, b: &mut B, c: &mut C) -> ()
where A: BoundedDomain3<i32, A, B, C>,
    B: BoundedDomain3<i32, A, B, C>,
    C: BoundedDomain3<i32, A, B, C>,
{
    let _ = a.less_than(b);
    let _ = a.less_than(c);
    let _ = b.less_than(c);
    let _ = b.greater_or_equal_than(a);
    let _ = c.greater_or_equal_than(a);
    let _ = c.greater_or_equal_than(b);
}


impl<Type> Variable for IntVarBound<Type>
    where Type: Ord + Eq
{}

impl FiniteDomain<i32> for IntVarBound<i32>
{
    fn size(&self) -> usize {
        (self.max - self.min) as usize + 1
    }
}

pub trait IterableDomain<T>: FiniteDomain<T> {
    /// Returns an `Iterator` over the elements of the domain.
    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a>;
}

pub trait AssignableDomain<T>: Variable where
    T: Eq
{
    fn equals_value(&mut self, value: T) -> Result<VariableState, VariableError>;
}

pub trait PrunableDomain<T>: AssignableDomain<T> where
    T: Eq
{
    fn in_values<Values>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = T>;
    fn remove_value(&mut self, value: T)
        -> Result<VariableState, VariableError>;
    fn remove_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&T) -> bool;
    fn retains_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&T) -> bool;
}

pub trait EqualityDomains<T, Other=Self>: PrunableDomain<T> where
    T: Eq,
    Other: PrunableDomain<T>
{
    fn equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError>;
    fn not_equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError>;
}

pub trait PrunableOrderedDomain<T>: OrderedDomain<T>+PrunableDomain<T> where
    T: Ord+Eq
{
    fn in_sorted_values<Values: Iterator<Item = T>>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = T>;
}

impl OrderedDomain<i32> for IntVarBound<i32>
{
    fn min(&self) -> Option<i32> {
        if self.min <= self.max {
            Some(self.min)
        } else {
            None
        }
    }

    fn max(&self) -> Option<i32> {
        if self.min <= self.max {
            Some(self.max)
        } else {
            None
        }
    }

    fn strict_upperbound(&mut self, ub: i32)
        -> Result<VariableState, VariableError> {
        if self.unchecked_max() < ub {
            Ok(VariableState::NoChange)
        } else if self.unchecked_min() >= ub {
            Err(VariableError::DomainWipeout)
        } else {
            self.max = ub-1;
            Ok(VariableState::BoundsChange)
        }
    }

    fn weak_upperbound(&mut self, ub: i32)
        -> Result<VariableState, VariableError> {
        if self.unchecked_max() <= ub {
            Ok(VariableState::NoChange)
        } else if self.unchecked_min() > ub {
            Err(VariableError::DomainWipeout)
        } else {
            self.max = ub;
            Ok(VariableState::BoundsChange)
        }
    }

    fn strict_lowerbound(
        &mut self,
        lb: i32,
    ) -> Result<VariableState, VariableError> {
        if self.unchecked_max() > lb {
            Ok(VariableState::NoChange)
        } else if self.unchecked_max() <= lb {
            Err(VariableError::DomainWipeout)
        } else {
            self.min = lb+1;
            Ok(VariableState::BoundsChange)
        }
    }

    fn weak_lowerbound(&mut self, lb: i32)
        -> Result<VariableState, VariableError> {
        if self.unchecked_max() >= lb {
            Ok(VariableState::NoChange)
        } else if self.unchecked_max() < lb {
            Err(VariableError::DomainWipeout)
        } else {
            self.min = lb;
            Ok(VariableState::BoundsChange)
        }
    }
}

struct IntVarBound<Type> where Type:Ord+Eq {
    min: Type,
    max: Type,
}
