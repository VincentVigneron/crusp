use snowflake::ProcessUniqueId;
use variables::{VariableView, ViewIndex};

// move Var and ArrayView inside macro => find how to handle extern crate ProcessUniqeId

// TODO two views of the same index of the array must have the same type
// TODO add trait Array: ?Var and trait Var: ? Array ... ?

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VarViewType {
    FromVar(usize),
    FromArray(usize, usize),
}

#[derive(Clone, Debug)]
pub struct VarView {
    id: ProcessUniqueId,
    view: VarViewType,
}

impl VarView {
    pub fn new(id: ProcessUniqueId, x: usize) -> VarView {
        VarView {
            id: id,
            view: VarViewType::FromVar(x),
        }
    }

    pub fn new_from_array(id: ProcessUniqueId, x: usize, y: usize) -> VarView {
        VarView {
            id: id,
            view: VarViewType::FromArray(x, y),
        }
    }

    pub fn get_idx(&self) -> &VarViewType {
        &self.view
    }
}

impl VariableView for VarView {
    fn get_id(&self) -> ViewIndex {
        match self.view {
            VarViewType::FromVar(x) => ViewIndex::new_from_var(self.id, x),
            VarViewType::FromArray(x, y) => ViewIndex::new_from_array(self.id, x, y),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ArrayView {
    id: ProcessUniqueId,
    x: usize,
}

impl ArrayView {
    pub fn new(id: ProcessUniqueId, x: usize) -> ArrayView {
        ArrayView { id: id, x: x }
    }

    // Change id type to implement partialeq
    pub fn get(&self, y: usize) -> VarView {
        VarView {
            id: self.id,
            view: VarViewType::FromArray(self.x, y),
        }
    }

    pub fn get_idx(&self) -> usize {
        self.x
    }
}

impl VariableView for ArrayView {
    fn get_id(&self) -> ViewIndex {
        ViewIndex::new_from_var(self.id, self.x)
    }
}

// OTHER SYNTAX
// variables_handler_build!(
//      IntVar,
//      Array of IntVar,
//      BoolVar,
//  )
//  Other impl
//  One vector for each type
//  the vector 0 is reserved for single var ?

#[macro_export]
macro_rules! variables_handler_build {
    ($($type: ident),+) => {
        use $crate::variables::Variable;
        use $crate::variables::VariableView;
        use $crate::variables::VariableState;
        use $crate::variables::Array;
        use $crate::variables::handlers::macros::{VarView, ArrayView, VarViewType};
        use $crate::variables::handlers::{
            VariablesHandlerBuilder,
            SpecificVariablesHandler,
            SpecificVariablesHandlerBuilder};
        use snowflake::ProcessUniqueId;

        #[derive(Debug,Clone)]
        struct SpecificTypeHandler<Var: Variable> {
            id: ProcessUniqueId,
            variables: Vec<Var>,
            variables_array: Vec<Array<Var>>,
        }

        impl<Var: Variable> SpecificTypeHandler<Var> {
            fn new() -> Self {
                SpecificTypeHandler {
                    id: ProcessUniqueId::new(),
                    variables: Vec::new(),
                    variables_array: Vec::new(),
                }
            }
        }

        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub struct Builder {
            $(
                $type: SpecificTypeHandler<$type>
             ),+
        }

        #[derive(Debug,Clone)]
        #[allow(non_snake_case)]
        pub struct Handler {
            $(
                $type: SpecificTypeHandler<$type>
             ),+
        }

        impl Builder {
            pub fn new() -> Builder {
                Builder {
                    $(
                        $type: SpecificTypeHandler::new()
                     ),+
                }
            }
        }

        impl VariablesHandlerBuilder<Handler> for Builder {
            fn finalize(self) -> Handler {
                Handler {
                    $(
                        $type: self.$type
                     ),+
                }
            }
        }

        // TODO call retrieve_all_changed_states from specifichandlers
        impl $crate::variables::handlers::VariablesHandler for Handler {
            fn retrieve_all_changed_states(
                &mut self,
            ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>> {
                let changed_states = vec![$({
                    let id = self.$type.id.clone();
                    let var_states: Vec<(Box<VariableView>, _)> = self.$type
                        .variables_array
                        .iter_mut()
                        .enumerate()
                        .flat_map(|(x,val)| {
                              val.iter_mut()
                                  .enumerate()
                                  .map(move |(y,val)| (x,y,val.retrieve_state()))
                        })
                        .filter(|&(_,_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,y,state)| {
                            let view: Box<VariableView> =
                                Box::new(VarView::new_from_array(id, x, y));
                            (view, state)
                        })
                        .collect();
                    let array_states: Vec<(Box<VariableView>, _)> = self.$type
                        .variables_array
                        .iter_mut()
                        .enumerate()
                        .map(|(x,val)| (x, val.retrieve_state()))
                        .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,state)| {
                            let view: Box<VariableView> = Box::new(VarView::new(id, x));
                            (view, state)
                        })
                        .collect();
                    let id = self.$type.id.clone();
                    let views: Vec<(Box<VariableView>, _)> = self.$type.variables
                        .iter_mut()
                        .enumerate()
                        .map(|(x,val)| (x, val.retrieve_state()))
                        .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,state)| {
                            let view: Box<VariableView> = Box::new(VarView::new(id, x));
                            (view, state)
                        })
                        .collect();
                    Box::new(
                        var_states.into_iter()
                            .chain(array_states.into_iter())
                            .chain(views.into_iter()))
                }),+];
                Box::new(
                    changed_states.into_iter().flat_map(|changes| changes)
                )
            }
        }

        $(
            impl SpecificVariablesHandlerBuilder<$type, VarView, Handler>
            for Builder {
                fn add(&mut self, x: $type) -> VarView {
                    let view = VarView::new(self.$type.id, self.$type.variables.len());
                    self.$type.variables.push(x);
                    view
                }
            }

            impl SpecificVariablesHandlerBuilder<Array<$type>, ArrayView, Handler>
            for Builder {
                fn add(&mut self, x: Array<$type>) -> ArrayView {
                    let view = ArrayView::new(self.$type.id, self.$type.variables_array.len());
                    self.$type.variables_array.push(x);
                    view
                }
            }

            impl SpecificVariablesHandler<$type, VarView> for Handler {
                fn get_mut(&mut self, view: &VarView) -> &mut $type {
                    match *view.get_idx() {
                        VarViewType::FromVar(x) => {
                            unsafe { self.$type.variables.get_unchecked_mut(x) }
                        }
                        VarViewType::FromArray(x,y) => {
                            unsafe {
                                self.$type.variables_array
                                    .get_unchecked_mut(x)
                                    .variables
                                    .get_unchecked_mut(y)
                            }
                        }
                    }
                }
                fn get(&self, view: &VarView) -> &$type {
                    match *view.get_idx() {
                        VarViewType::FromVar(x) => {
                            unsafe { self.$type.variables.get_unchecked(x) }
                        }
                        VarViewType::FromArray(x,y) => {
                            unsafe {
                                self.$type.variables_array
                                    .get_unchecked(x)
                                    .variables
                                    .get_unchecked(y)
                            }
                        }
                    }
                }

                fn retrieve_state(&mut self, view: &VarView) -> VariableState {
                    self.get_mut(view).retrieve_state()
                }

                fn retrieve_states<'a, Views>(
                    &mut self,
                    views: Views,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>>
                    where
                        Views: Iterator<Item = &'a VarView>,
                {
                    let mut states: Vec<(Box<VariableView>, _)> = Vec::new();
                    for view in views {
                        let state = self.get_mut(view).retrieve_state();
                        let view = Box::new(view.clone());
                        states.push((view,state));
                    }
                    Box::new(states.into_iter())
                }
                fn retrieve_all_changed_states(
                    &mut self,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>> {
                    let id = self.$type.id.clone();
                    let views: Vec<(Box<VariableView>, _)> = self.$type.variables
                        .iter_mut()
                        .enumerate()
                        .map(|(x,val)| (x, val.retrieve_state()))
                        .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,state)| {
                            let view: Box<VariableView> = Box::new(VarView::new(id, x));
                            (view, state)
                        })
                        .collect();
                    Box::new(views.into_iter())
                }
            }

            impl SpecificVariablesHandler<Array<$type>, ArrayView> for Handler {
                fn get_mut(&mut self, view: &ArrayView) -> &mut Array<$type> {
                    unsafe {
                        self.$type.variables_array.get_unchecked_mut(view.get_idx())
                    }
                }
                fn get(&self, view: &ArrayView) -> & Array<$type> {
                    unsafe {
                        self.$type.variables_array.get_unchecked(view.get_idx())
                    }
                }

                fn retrieve_state(&mut self, view: &ArrayView) -> VariableState {
                    self.get_mut(view).retrieve_state()
                }

                // Optimize by accessing variabl directly in the data structure
                fn retrieve_states<'a, Views>(
                    &mut self,
                    views: Views,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>>
                    where
                        Views: Iterator<Item = &'a ArrayView>,
                {
                    let mut states: Vec<(Box<VariableView>, _)> = Vec::new();
                    for view in views {
                        {
                            let state = self.get_mut(view).retrieve_state();
                            let view = Box::new(view.clone());
                            states.push((view,state));
                        }
                        for i in 0..(self.get(view).variables.len()) {
                            let view = view.get(i);
                            let state = self.get_mut(&view).retrieve_state();
                            let view = Box::new(view.clone());
                            states.push((view,state));
                        }
                    }
                    Box::new(states.into_iter())
                }
                fn retrieve_all_changed_states(
                    &mut self,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>> {
                    let id = self.$type.id.clone();
                    let var_states: Vec<(Box<VariableView>, _)> = self.$type
                        .variables_array
                        .iter_mut()
                        .enumerate()
                        .flat_map(|(x,val)| {
                              val.iter_mut()
                                  .enumerate()
                                  .map(move |(y,val)| (x,y,val.retrieve_state()))
                        })
                        .filter(|&(_,_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,y,state)| {
                            let view: Box<VariableView> =
                                Box::new(VarView::new_from_array(id, x, y));
                            (view, state)
                        })
                        .collect();
                    let array_states: Vec<(Box<VariableView>, _)> = self.$type
                        .variables_array
                        .iter_mut()
                        .enumerate()
                        .map(|(x,val)| (x, val.retrieve_state()))
                        .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,state)| {
                            let view: Box<VariableView> = Box::new(VarView::new(id, x));
                            (view, state)
                        })
                        .collect();
                    Box::new(var_states.into_iter().chain(array_states.into_iter()))
                }
            }
        )+
    };
}
