use snowflake::ProcessUniqueId;

pub mod int_var;
pub mod handlers;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableState {
    BoundChange,
    ValuesChange,
    NoChange,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableError {
    DomainWipeout,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum IndexType {
    FromVar(usize),
    FromArray(usize, usize),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn new_from_array(id: ProcessUniqueId, x: usize, y: usize) -> ViewIndex {
        ViewIndex {
            id: id,
            index_type: IndexType::FromArray(x, y),
        }
    }
    // x sub_view_of x
    // x sub_view_of_y && y sub_view_of x => x == y
    pub fn is_subview_of(&self, idx: &ViewIndex) -> bool {
        if self.id != idx.id {
            return false;
        }
        match self.index_type {
            IndexType::FromArray(x, y) => match idx.index_type {
                IndexType::FromVar(x_) => x == x_,
                IndexType::FromArray(x_, y_) => x == x_ && y == y_,
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

    pub fn get_mut(&mut self, idx: usize) -> &mut Var {
        unsafe { &mut *(self.variables.get_unchecked_mut(idx) as *mut _) }
    }

    pub fn get(&self, idx: usize) -> &Var {
        unsafe { self.variables.get_unchecked(idx) }
    }

    pub fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &mut Var> + 'a> {
        Box::new(self.variables.iter_mut())
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
                if *acc == VariableState::BoundChange {
                    return None;
                }
                *acc = if *acc == VariableState::NoChange {
                    state.clone()
                } else if *state == VariableState::BoundChange {
                    VariableState::BoundChange
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
