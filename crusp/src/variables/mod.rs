use graph::Subsumed;
use enumflags2::{bitflags, make_bitflags};
use std::iter;

pub mod bool_var;
pub mod domains;
pub mod int_var;
#[macro_use]
pub mod handlers;
#[macro_use]
pub mod macros;

/// Describes the state of a variable after its domain is updated.
//#[repr(bitflags)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VariableState {
    /// If only the maximal bound of the variable has been updated.
    MaxBoundChange = 0b0000_0011,
    /// If only the minimal bound of the variable has been updated.
    MinBoundChange = 0b0000_0101,
    /// If both bounds of the variable has been updated.
    BoundsChange = 0b0000_0111,
    /// If the domain has been change but not its bounds.
    ValuesChange = 0b0000_1111,
    /// If no change occured.
    NoChange = 0b0000_0000,
    /// When the value has been changed by an universal brancher
    UniversalChange = 0b1110_0000,
    UniversalError = 0b1110_0001,
}

impl std::ops::BitOr for VariableState {
    type Output = Self;

     fn bitor(self, rhs: Self) -> Self::Output {
          unsafe {
              let lhs: u8 = std::mem::transmute(self);
              let rhs: u8 = std::mem::transmute(rhs);
              let univ: u8 = std::mem::transmute(VariableState::UniversalChange);
              let value: u8 = std::mem::transmute(VariableState::ValuesChange);
              let univ_bit = (lhs | rhs) & univ;
              let value_bit = (lhs | rhs) & value;
              let value_mask = (!univ_bit) >> 4;
              let res = univ_bit | (value_bit & value_mask);
              std::mem::transmute(res)
          }
      }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_op_or() {
        // no change is neutral
        assert_eq!(VariableState::NoChange | VariableState::MaxBoundChange, VariableState::MaxBoundChange);
        assert_eq!(VariableState::NoChange | VariableState::MinBoundChange, VariableState::MinBoundChange);
        assert_eq!(VariableState::NoChange | VariableState::BoundsChange, VariableState::BoundsChange);
        assert_eq!(VariableState::NoChange | VariableState::ValuesChange, VariableState::ValuesChange);
        assert_eq!(VariableState::NoChange | VariableState::NoChange, VariableState::NoChange);
        assert_eq!(VariableState::NoChange | VariableState::UniversalChange, VariableState::UniversalChange);
        assert_eq!(VariableState::NoChange | VariableState::UniversalError, VariableState::UniversalError);
        // max bounds
        assert_eq!(VariableState::MaxBoundChange | VariableState::MaxBoundChange, VariableState::MaxBoundChange);
        assert_eq!(VariableState::MaxBoundChange | VariableState::MinBoundChange, VariableState::BoundsChange);
        assert_eq!(VariableState::MaxBoundChange | VariableState::BoundsChange, VariableState::BoundsChange);
        assert_eq!(VariableState::MaxBoundChange | VariableState::ValuesChange, VariableState::ValuesChange);
        assert_eq!(VariableState::MaxBoundChange | VariableState::NoChange, VariableState::MaxBoundChange);
        assert_eq!(VariableState::MaxBoundChange | VariableState::UniversalChange, VariableState::UniversalError);
        assert_eq!(VariableState::MaxBoundChange | VariableState::UniversalError, VariableState::UniversalError);
        // min bounds
        assert_eq!(VariableState::MinBoundChange | VariableState::MaxBoundChange, VariableState::BoundsChange);
        assert_eq!(VariableState::MinBoundChange | VariableState::MinBoundChange, VariableState::MinBoundChange);
        assert_eq!(VariableState::MinBoundChange | VariableState::BoundsChange, VariableState::BoundsChange);
        assert_eq!(VariableState::MinBoundChange | VariableState::ValuesChange, VariableState::ValuesChange);
        assert_eq!(VariableState::MinBoundChange | VariableState::NoChange, VariableState::MinBoundChange);
        assert_eq!(VariableState::MinBoundChange | VariableState::UniversalChange, VariableState::UniversalError);
        assert_eq!(VariableState::MinBoundChange | VariableState::UniversalError, VariableState::UniversalError);
        // bounds
        assert_eq!(VariableState::BoundsChange | VariableState::MaxBoundChange, VariableState::BoundsChange);
        assert_eq!(VariableState::BoundsChange | VariableState::MinBoundChange, VariableState::BoundsChange);
        assert_eq!(VariableState::BoundsChange | VariableState::BoundsChange, VariableState::BoundsChange);
        assert_eq!(VariableState::BoundsChange | VariableState::ValuesChange, VariableState::ValuesChange);
        assert_eq!(VariableState::BoundsChange | VariableState::NoChange, VariableState::BoundsChange);
        assert_eq!(VariableState::BoundsChange | VariableState::UniversalChange, VariableState::UniversalError);
        assert_eq!(VariableState::BoundsChange | VariableState::UniversalError, VariableState::UniversalError);
        // values
        assert_eq!(VariableState::ValuesChange | VariableState::MaxBoundChange, VariableState::ValuesChange);
        assert_eq!(VariableState::ValuesChange | VariableState::MinBoundChange, VariableState::ValuesChange);
        assert_eq!(VariableState::ValuesChange | VariableState::BoundsChange, VariableState::ValuesChange);
        assert_eq!(VariableState::ValuesChange | VariableState::ValuesChange, VariableState::ValuesChange);
        assert_eq!(VariableState::ValuesChange | VariableState::NoChange, VariableState::ValuesChange);
        assert_eq!(VariableState::ValuesChange | VariableState::UniversalChange, VariableState::UniversalError);
        assert_eq!(VariableState::ValuesChange | VariableState::UniversalError, VariableState::UniversalError);
        // universal
        assert_eq!(VariableState::UniversalChange | VariableState::MaxBoundChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalChange | VariableState::MinBoundChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalChange | VariableState::BoundsChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalChange | VariableState::ValuesChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalChange | VariableState::NoChange, VariableState::UniversalChange);
        assert_eq!(VariableState::UniversalChange | VariableState::UniversalChange, VariableState::UniversalChange);
        assert_eq!(VariableState::UniversalChange | VariableState::UniversalError, VariableState::UniversalError);
        // universal error
        assert_eq!(VariableState::UniversalError | VariableState::MaxBoundChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalError | VariableState::MinBoundChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalError | VariableState::BoundsChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalError | VariableState::ValuesChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalError | VariableState::NoChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalError | VariableState::UniversalChange, VariableState::UniversalError);
        assert_eq!(VariableState::UniversalError | VariableState::UniversalError, VariableState::UniversalError);


    }
}


// could use transmute to

struct UniversalConstraint {
    // only triger when a value has been changed but not by a universal brancher
    // bound(value_change)
    // propagete => Failure
}

// trigerred when univeral error is set
struct UniversalFailure {

}

impl Subsumed for VariableState {
    /// # Subsomption relations
    /// * `MaxBoundChange` subsumed `BoundsChange`
    /// * `MinBoundChange` subsumed `BoundsChange`
    /// * `BoundsChange` subsumed `ValuesChange`
    /// * `ValuesChange` subsumed `NoChange`
    fn is_subsumed_under(&self, val: &Self) -> bool {
        // not correct yet
        // (make_bitflags!(self) & make_bitflags!(val)).contains(make_bitflags!(self))
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariableId(usize);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstraintId(usize);

pub trait VariableEvents {
    fn push(&mut self, vid: VariableId, event: Result<VariableState, VariableError>) -> Result<VariableState, VariableError>;
    fn push_change(&mut self, vid: VariableId, event: VariableState) -> Result<VariableState, VariableError>;
    fn push_error(&mut self, vid: VariableId, event: VariableError) -> Result<VariableState, VariableError>;
}

struct NoOpVariableEvents {}

impl VariableEvents for NoOpVariableEvents {
    fn push(&mut self, vid: VariableId, event: Result<VariableState, VariableError>) -> {
        event
    }
    fn push_change(&mut self, vid: VariableId, event: VariableState) -> Result<VariableState, VariableError> {
        Ok(event)
    }
    fn push_error(&mut self, vid: VariableId, event: VariableError) -> Result<VariableState, VariableError> {
        Error(event)
    }
}

pub trait ConstraintVariableEvents {
    fn push(&mut self, cid: ConstraintId, vid: VariableId, event: Result<VariableState, VariableError>) -> Result<VariableState, VariableError>;
    fn push_change(&mut self, cid: ConstraintId, vid: VariableId, event: VariableState) -> Result<VariableState, VariableError>;
    fn push_error(&mut self, cid: ConstraintId, vid: VariableId, event: VariableError) -> Result<VariableState, VariableError>;
}

/// Trait for types that represent decision variables.
/// A decision variable is variable along side with its domain of allowed values.
/// A variable has to be cloneable because the (tree based) searching process is based on cloning.
pub trait Variable<Type>: Clone
    where Type: Clone,
{
    /// Returns if the variable is affected.
    /// A variable is affected if and only if its a domain is a singleton.
    fn is_affected(&self) -> bool;
    /// Returns the value of the variable or `None` if the variable is not
    /// affected.
    fn value(&self) -> Option<Type>;
    /// Returns the state of the variable without reinitialising it.
    /// The state of a variable describes if and how the domain of the variable has
    /// been updated.
    fn id(&self) -> VariableId;
}

/// This trait describes an array of variables. There is two types of array:
/// array of variables and array of references to variables. Both types are manipulated with the
/// same trait. When writting constraints over an array of variables, you should use the `Array`
/// trait instead of the specific types `ArrayOfVars` or `ArrayOfRefs`.
pub trait ArrayOfVariables<Type>
    where Type: clone
{
    // Variable type of the array²
    type Variable: Variable<Type>;
    /// Returns a mutable reference to the variable at that position or None if out of bounds.
    fn get_mut(&mut self, position: usize) -> Option<&mut Self::Variable>;
    /// Returns a reference to the variable at that position or None if out of bounds.
    fn get(&self, position: usize) -> Option<&Self::Variable>;
    /// Returns a mutable reference to the variable at that position without doing bounds check.
    fn get_unchecked_mut(&mut self, position: usize) -> &Self::Variable;
    /// Returns a reference to the variable at that position without doing bounds check.
    fn get_unchecked(&self, position: usize) -> &Self::Variable;
    /// Returns an iterator over the variables.
    fn iter<'array>(&'array self) -> Box<Iterator<Item = &Self::Variable> + 'array>;
    /// Returns an iterator that allows modifying each variable.
    fn iter_mut<'array>(
        &'array mut self,
    ) -> Box<Iterator<Item = &mut Self::Variable> + 'array>;
    /// Returns the number of variables.
    fn len(&self) -> usize;
}
/*

/// Trait used to an array of variable;
pub trait ArrayBuilder: Sized {
    type Builder: VariableBuilder;
    fn into_iter(self) -> Box<Iterator<Item = Self::Builder>>;
}

/// Trait used to build a variable. `SpecificVariablesHandler` requires
/// to add VariableBuiler or ArrayBuilder in order to assign them one
/// unique id.
pub trait VariableBuilder: Clone {
    type Variable: Variable + Clone;
    fn finalize(self, id: usize) -> Self::Variable;
}

/// Represents an array of `Variable` builder.
#[derive(Debug, Clone)]
pub struct ArrayOfVarsBuilder<Builder: VariableBuilder> {
    /// The array of `Variable`.
    variables: Vec<Builder>,
}
impl<Builder: VariableBuilder> ArrayOfVarsBuilder<Builder> {
    /// Creates a new `ArrayOfVars` or None if the number of variables is null.
    ///
    /// # Arguments
    /// *`len` - The number of variables.
    /// *`var` - The prototype of variable used to fill the array.
    pub fn new(len: usize, var: Builder) -> Option<Self> {
        Some(ArrayOfVarsBuilder {
            variables: vec![var.clone(); len],
        })
    }
}

impl<Builder: VariableBuilder + 'static> ArrayBuilder for ArrayOfVarsBuilder<Builder> {
    type Builder = Builder;
    fn into_iter(self) -> Box<Iterator<Item = Self::Builder>> {
        Box::new(self.variables.into_iter())
    }
}

/// Represents an array of `Variable`.
#[derive(Debug, Clone)]
pub struct ArrayOfVars<Var: Variable> {
    /// The array of `Variable`.
    variables: Vec<Var>,
}

impl<Var: Variable> VariableContainer for ArrayOfVars<Var> {}

impl<Var: Variable> ArrayOfVars<Var> {
    /// Creates a new `ArrayOfVars` or None if the number of variables is null.
    ///
    /// # Arguments
    /// *`len` - The number of variables.
    /// *`var` - The prototype of variable used to fill the array.
    pub fn new(len: usize, var: Var) -> Option<Self> {
        Some(ArrayOfVars {
            variables: vec![var.clone(); len],
        })
    }
    ///
    /// # Arguments
    pub fn new_from_iter(var: impl IntoIterator<Item = Var>) -> Option<Self> {
        Some(ArrayOfVars {
            variables: var.into_iter().collect(),
        })
    }
}

impl<Var: Variable> Array for ArrayOfVars<Var> {
    type Variable = Var;
    fn get_mut(&mut self, position: usize) -> Option<&mut Var> {
        self.variables.get_mut(position)
    }

    fn get(&self, position: usize) -> Option<&Var> {
        self.variables.get(position)
    }

    fn get_unchecked_mut(&mut self, position: usize) -> &mut Var {
        unsafe { &mut *(self.variables.get_unchecked_mut(position) as *mut _) }
    }

    fn get_unchecked(&self, position: usize) -> &Var {
        unsafe { self.variables.get_unchecked(position) }
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

/// Represents an array of references to `Variable`.
#[derive(Debug, Clone)]
pub struct ArrayOfRefs<Var: Variable> {
    /// The array of references to `Variable`.
    variables: Vec<*mut Var>,
}

impl<Var: Variable> VariableContainer for ArrayOfRefs<Var> {}

// REF ARRAY BUILDER
impl<Var: Variable> ArrayOfRefs<Var> {
    /// Creates a new `ArrayOfVars` or None if the number of variables is null.
    ///
    /// # Argument
    /// *`variables` - Vector of references to variables.
    fn new(variables: Vec<*mut Var>) -> Option<Self> {
        Some(ArrayOfRefs {
            variables: variables,
        })
    }
}

impl<Var: Variable> Array for ArrayOfRefs<Var> {
    type Variable = Var;
    fn get_mut(&mut self, position: usize) -> Option<&mut Var> {
        unsafe { self.variables.get_mut(position).map(|var| &mut (**var)) }
    }

    fn get(&self, position: usize) -> Option<&Var> {
        unsafe { self.variables.get(position).map(|var| &(**var)) }
    }

    fn get_unchecked_mut(&mut self, position: usize) -> &mut Var {
        unsafe { &mut (**self.variables.get_unchecked_mut(position)) }
    }

    fn get_unchecked(&self, position: usize) -> &Var {
        unsafe { &(**self.variables.get_unchecked(position)) }
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


*/
