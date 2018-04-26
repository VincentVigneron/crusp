use snowflake::ProcessUniqueId;
use variables::VariableView;

// move Var and ArrayView inside macro => find how to handle extern crate ProcessUniqeId

// TODO remove array view index and use VarView
// TODO Add two fields to VarView
// TODO remove unecessary code
// TODO add trait Array: ?Var and trait Var: ? Array ... ?
#[derive(Clone, Debug)]
pub struct VarView {
    id: ProcessUniqueId,
    idx: usize,
}

impl VarView {
    pub fn new(idx: usize) -> VarView {
        VarView {
            id: ProcessUniqueId::new(),
            idx: idx,
        }
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }
}

impl VariableView for VarView {
    fn get_id(&self) -> ProcessUniqueId {
        self.id
    }
}

#[derive(Clone, Debug)]
pub enum ArrayViewType {
    Variable(usize, usize),
    Array(usize),
}

// TODO remove enum and use ArrayViewVar
// TODO two views of the same index of the array must have the same type
#[derive(Clone, Debug)]
pub struct ArrayView {
    id: ProcessUniqueId,
    view: ArrayViewType,
    //views: Vec<Some<ArrayView>>,
}

// TODO index
impl ArrayView {
    pub fn new(x: usize) -> ArrayView {
        ArrayView {
            id: ProcessUniqueId::new(),
            view: ArrayViewType::Array(x),
        }
    }

    pub fn get(&self, y: usize) -> ArrayViewIndex {
        match self.view {
            ArrayViewType::Array(x) => ArrayViewIndex {
                id: ProcessUniqueId::new(),
                view: ArrayViewType::Variable(x, y),
            },
            _ => panic!("Can't index not array view"),
        }
    }

    pub fn get_view(&self) -> &ArrayViewType {
        &self.view
    }
}

impl VariableView for ArrayView {
    fn get_id(&self) -> ProcessUniqueId {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct ArrayViewIndex {
    id: ProcessUniqueId,
    view: ArrayViewType,
    //views: Vec<Some<ArrayView>>,
}

// TODO index
impl ArrayViewIndex {
    pub fn new(x: usize) -> ArrayView {
        ArrayView {
            id: ProcessUniqueId::new(),
            view: ArrayViewType::Array(x),
        }
    }

    pub fn get(&self, y: usize) -> ArrayView {
        match self.view {
            ArrayViewType::Array(x) => ArrayView {
                id: ProcessUniqueId::new(),
                view: ArrayViewType::Variable(x, y),
            },
            _ => panic!("Can't index not array view"),
        }
    }

    pub fn get_view(&self) -> &ArrayViewType {
        &self.view
    }
}

impl VariableView for ArrayViewIndex {
    fn get_id(&self) -> ProcessUniqueId {
        self.id
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
        use $crate::variables::handlers::macros::{VarView, ArrayView, ArrayViewType,ArrayViewIndex};
        use $crate::variables::handlers::{
            VariablesHandlerBuilder,
            SpecificVariablesHandler,
            SpecificVariablesHandlerBuilder};

        #[derive(Debug,Clone)]
        struct SpecificTypeHandler<Var: Variable> {
            variables: Vec<Var>,
            variables_array: Vec<Array<Var>>,
        }

        impl<Var: Variable> SpecificTypeHandler<Var> {
            fn new() -> Self {
                SpecificTypeHandler {
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

        $(
            impl SpecificVariablesHandlerBuilder<$type, VarView, Handler>
            for Builder {
                fn add(&mut self, x: $type) -> VarView {
                    let view = VarView::new(self.$type.variables.len());
                    self.$type.variables.push(x);
                    view
                }
            }

            impl SpecificVariablesHandlerBuilder<Array<$type>, ArrayView, Handler>
            for Builder {
                fn add(&mut self, x: Array<$type>) -> ArrayView {
                    let view = ArrayView::new(self.$type.variables_array.len());
                    self.$type.variables_array.push(x);
                    view
                }
            }

            impl $crate::variables::handlers::VariablesHandler for Handler {
                fn retrieve_all_states(
                    &mut self,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>> {
                    unimplemented!()
                }
            }

            impl SpecificVariablesHandler<$type, VarView> for Handler {
                fn get_mut(&mut self, view: &VarView) -> &mut $type {
                    let idx = view.get_idx();
                    unsafe { self.$type.variables.get_unchecked_mut(idx) }
                }
                fn get(&self, view: &VarView) -> &$type {
                    let idx = view.get_idx();
                    unsafe { self.$type.variables.get_unchecked(idx) }
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
                fn retrieve_all_states(
                    &mut self,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>> {
                    unimplemented!()
                }
            }

            impl SpecificVariablesHandler<Array<$type>, ArrayView> for Handler {
                fn get_mut(&mut self, view: &ArrayView) -> &mut Array<$type> {
                    if let ArrayViewType::Array(x) = *view.get_view() {
                        unsafe { self.$type.variables_array.get_unchecked_mut(x) }
                    } else {
                        panic!()
                    }
                }
                fn get(&self, view: &ArrayView) -> & Array<$type> {
                    if let ArrayViewType::Array(x) = *view.get_view() {
                        unsafe { self.$type.variables_array.get_unchecked(x) }
                    } else {
                        panic!()
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
                fn retrieve_all_states(
                    &mut self,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>> {
                    unimplemented!()
                }
            }

            // TODO index on Array<Var>
            impl SpecificVariablesHandler<$type, ArrayViewIndex> for Handler {
                fn get_mut(&mut self, view: &ArrayViewIndex) -> &mut $type {
                    if let ArrayViewType::Variable(x, y) = *view.get_view() {
                        unsafe {
                            self.$type.variables_array
                                .get_unchecked_mut(x)
                                .variables
                                .get_unchecked_mut(y)
                        }
                    } else {
                        panic!()
                    }
                }
                fn get(&self, view: &ArrayViewIndex) -> &$type {
                    if let ArrayViewType::Variable(x, y) = *view.get_view() {
                        unsafe {
                            self.$type.variables_array
                                .get_unchecked(x)
                                .variables
                                .get_unchecked(y)
                        }
                    } else {
                        panic!()
                    }
                }
                fn retrieve_state(&mut self, view: &ArrayViewIndex) -> VariableState {
                    self.get_mut(view).retrieve_state()
                }

                fn retrieve_states<'a, Views>(
                    &mut self,
                    views: Views,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>>
                    where
                        Views: Iterator<Item = &'a ArrayViewIndex>,
                {
                    let mut states: Vec<(Box<VariableView>, _)> = Vec::new();
                    for view in views {
                        let state = self.get_mut(view).retrieve_state();
                        let view = Box::new(view.clone());
                        states.push((view,state));
                    }
                    Box::new(states.into_iter())
                }
                fn retrieve_all_states(
                    &mut self,
                ) -> Box<Iterator<Item = (Box<VariableView>, VariableState)>> {
                    unimplemented!()
                }
            }
        )+
    };
}
