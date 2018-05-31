use graph::Subsumed;
use std::iter;

pub mod bool_var;
pub mod domains;
pub mod int_var;
#[macro_use]
pub mod handlers;
#[macro_use]
pub mod macros;

/// Describes the state of a variable after its domain is updated.
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VariableId(usize);

/// Trait for types that represent decision variables.
/// A decision variable is variable along side with its domain of allowed values.
/// A variable has to be cloneable because the (tree based) searching process is based on cloning.
pub trait Variable: Clone {
    /// The underlying type holded by the `Variable`.
    type Type: Clone;
    /// Returns if the variable is affected.
    /// A variable is affected if and only if its a domain is a singleton.
    fn is_affected(&self) -> bool;
    /// Returns the value of the variable or `None` if the variable is not
    /// affected.
    fn value(&self) -> Option<Self::Type>;
    /// Returns the state of the variable without reinitialising it.
    /// The state of a variable describes if and how the domain of the variable has
    /// been updated.
    fn id(&self) -> VariableId;
    fn iter_ids(&self) -> Box<Iterator<Item = VariableId>> {
        Box::new(iter::once(self.id().clone()))
    }
}

/// This trait describes an array of variables. There is two types of array:
/// array of variables and array of references to variables. Both types are manipulated with the
/// same trait. When writting constraints over an array of variables, you should use the `Array`
/// trait instead of the specific types `ArrayOfVars` or `ArrayOfRefs`.
pub trait Array {
    type Variable: Variable;
    /// Returns a mutable reference to the variable at that position or None if out of bounds.
    fn get_mut(&mut self, position: usize) -> Option<&mut Self::Variable>;
    /// Returns a reference to the variable at that position or None if out of bounds.
    fn get(&self, position: usize) -> Option<&Self::Variable>;
    /// Returns a mutable reference to the variable at that position without doing bounds check.
    fn get_unchecked_mut(&mut self, position: usize) -> &mut Self::Variable;
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
    fn iter_ids<'a>(&'a self) -> Box<Iterator<Item = VariableId> + 'a> {
        Box::new(self.iter().map(|var| var.id()))
    }
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
