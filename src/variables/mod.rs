use graph::Subsumed;
use snowflake::ProcessUniqueId;

pub mod int_var;
pub mod handlers;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableState {
    BoundsChange,
    ValuesChange,
    MaxBoundChange,
    MinBoundChange,
    NoChange,
}

impl Subsumed for VariableState {
    fn is_subsumed_under(&self, val: &Self) -> bool {
        match *self {
            VariableState::MaxBoundChange => {
                *val != VariableState::NoChange && *val != VariableState::MinBoundChange
            }
            VariableState::MinBoundChange => {
                *val != VariableState::NoChange && *val != VariableState::MaxBoundChange
            }
            VariableState::BoundsChange => {
                *val == VariableState::BoundsChange || *val == VariableState::ValuesChange
            }
            VariableState::ValuesChange => *val == VariableState::ValuesChange,
            VariableState::NoChange => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableError {
    DomainWipeout,
}

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum IndexType {
    FromVar(usize),
    FromArray(usize),
    FromArrayVar(usize, usize),
}

pub trait VariableView: Copy {}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ViewIndex {
    id: ProcessUniqueId,
    index_type: IndexType,
}

impl ViewIndex {
    pub fn new_from_var(id: ProcessUniqueId, x: usize) -> ViewIndex {
        ViewIndex {
            id: id,
            index_type: IndexType::FromVar(x),
        }
    }

    pub fn new_from_array(id: ProcessUniqueId, x: usize) -> ViewIndex {
        ViewIndex {
            id: id,
            index_type: IndexType::FromArray(x),
        }
    }

    pub fn new_from_array_var(id: ProcessUniqueId, x: usize, y: usize) -> ViewIndex {
        ViewIndex {
            id: id,
            index_type: IndexType::FromArrayVar(x, y),
        }
    }
    // x sub_view_of x
    // x sub_view_of_y && y sub_view_of x => x == y
    pub fn is_subview_of(&self, idx: &ViewIndex) -> bool {
        if self.id != idx.id {
            return false;
        }
        match self.index_type {
            IndexType::FromArrayVar(x, y) => match idx.index_type {
                IndexType::FromArray(x_) => x == x_,
                IndexType::FromArrayVar(x_, y_) => x == x_ && y == y_,
                _ => false,
            },
            IndexType::FromArray(x) => match idx.index_type {
                IndexType::FromArray(x_) => x == x_,
                _ => false,
            },
            IndexType::FromVar(x) => match idx.index_type {
                IndexType::FromVar(x_) => x == x_,
                _ => false,
            },
        }
    }

    pub fn get_id(&self) -> ProcessUniqueId {
        self.id.clone()
    }

    pub fn get_type(&self) -> IndexType {
        self.index_type.clone()
    }
}

pub trait AllDisjoint: Iterator<Item = ViewIndex> {
    fn all_disjoint(self) -> Result<(), (ViewIndex, ViewIndex)>
    where
        Self: Sized;
}

// More precise result for all_disjoint (i.e. which views are equal and ,which view is a
// subview of an array)
impl<I> AllDisjoint for I
where
    I: Iterator<Item = ViewIndex>,
{
    fn all_disjoint(self) -> Result<(), (ViewIndex, ViewIndex)>
    where
        Self: Sized,
    {
        use std::iter;
        let views: Vec<_> = self.collect();
        let incompatibles = views
            .iter()
            .enumerate()
            .map(|(i, view)| (view, views.iter().skip(i + 1)))
            .flat_map(|(left, rights)| iter::repeat(left).zip(rights))
            .find(|&(ref left, ref right)| {
                left.is_subview_of(right) || right.is_subview_of(left)
            });
        match incompatibles {
            None => Ok(()),
            Some((left, right)) => Err((left.clone(), right.clone())),
        }
    }
}

pub trait Variable: Clone {
    fn is_fixed(&self) -> bool;
    fn get_state(&self) -> &VariableState;
    fn retrieve_state(&mut self) -> VariableState;
}

pub trait List<Var: Variable>: Variable {
    fn get_mut(&mut self, idx: usize) -> &mut Var;
    fn get(&self, idx: usize) -> &Var;
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Var> + 'a>;
    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &mut Var> + 'a>;
    fn len(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Array<Var: Variable> {
    pub variables: Vec<Var>,
    state: VariableState,
    //states: Vec<VariableState>,
}

impl<Var: Variable> Array<Var> {
    pub fn new(len: usize, var: Var) -> Option<Self> {
        Some(Array {
            variables: vec![var.clone(); len],
            state: VariableState::NoChange,
            //states: vec![VariableState::NoChange; len],
        })
    }
}

impl<Var: Variable> List<Var> for Array<Var> {
    fn get_mut(&mut self, idx: usize) -> &mut Var {
        unsafe { &mut *(self.variables.get_unchecked_mut(idx) as *mut _) }
    }

    fn get(&self, idx: usize) -> &Var {
        unsafe { self.variables.get_unchecked(idx) }
    }

    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Var> + 'a> {
        Box::new(self.variables.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &mut Var> + 'a> {
        Box::new(self.variables.iter_mut())
    }

    fn len(&self) -> usize {
        self.variables.len()
    }
}
impl<Var: Variable> Variable for Array<Var> {
    fn is_fixed(&self) -> bool {
        unimplemented!()
    }
    fn get_state(&self) -> &VariableState {
        &self.state
    }
    fn retrieve_state(&mut self) -> VariableState {
        self.variables
            .iter()
            .map(|var| var.get_state())
            .scan(VariableState::NoChange, |acc, state| {
                if *acc == VariableState::BoundsChange {
                    return None;
                }
                *acc = if *acc == VariableState::NoChange {
                    state.clone()
                } else if *state == VariableState::BoundsChange {
                    VariableState::BoundsChange
                } else {
                    acc.clone()
                };

                Some(acc.clone())
            })
            .last()
            .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct RefArray<Var: Variable> {
    pub variables: Vec<*mut Var>,
    state: VariableState,
    //states: Vec<VariableState>,
}

// REF ARRAY BUILDER
impl<Var: Variable> RefArray<Var> {
    pub fn new(variables: Vec<*mut Var>) -> Option<Self> {
        Some(RefArray {
            variables: variables,
            state: VariableState::NoChange,
            //states: vec![VariableState::NoChange; len],
        })
    }
}

impl<Var: Variable> List<Var> for RefArray<Var> {
    fn get_mut(&mut self, idx: usize) -> &mut Var {
        unsafe { &mut (**self.variables.get_unchecked_mut(idx)) }
    }

    fn get(&self, idx: usize) -> &Var {
        unsafe { &(**self.variables.get_unchecked(idx)) }
    }

    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Var> + 'a> {
        unsafe { Box::new(self.variables.iter().map(|&var| &*var)) }
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &mut Var> + 'a> {
        unsafe { Box::new(self.variables.iter_mut().map(|&mut var| &mut *var)) }
    }

    fn len(&self) -> usize {
        self.variables.len()
    }
}

impl<Var: Variable> Variable for RefArray<Var> {
    fn is_fixed(&self) -> bool {
        unimplemented!()
    }
    fn get_state(&self) -> &VariableState {
        &self.state
    }
    fn retrieve_state(&mut self) -> VariableState {
        self.iter()
            .map(|var| var.get_state())
            .scan(VariableState::NoChange, |acc, state| {
                if *acc == VariableState::BoundsChange {
                    return None;
                }
                *acc = if *acc == VariableState::NoChange {
                    state.clone()
                } else if *state == VariableState::BoundsChange {
                    VariableState::BoundsChange
                } else {
                    acc.clone()
                };

                Some(acc.clone())
            })
            .last()
            .unwrap()
    }
}

#[macro_use]
pub mod macros;
