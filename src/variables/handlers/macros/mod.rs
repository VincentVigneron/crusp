use snowflake::ProcessUniqueId;
use std::marker::PhantomData;
use variables::{Variable, VariableView, ViewIndex};

// move Var and ArrayView inside macro => find how to handle extern crate ProcessUniqeId

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VarIndexType {
    FromVar(usize),
    FromArray(usize, usize),
}

#[derive(Copy, Clone, Debug)]
pub struct VarView<Var: Variable> {
    id: ProcessUniqueId,
    pub view: VarIndexType,
    phantom: PhantomData<Var>,
}

impl<Var: Variable> VariableView for VarView<Var> {}

impl<Var: Variable> VarView<Var> {
    pub fn new(id: ProcessUniqueId, x: usize) -> VarView<Var> {
        VarView {
            id: id,
            view: VarIndexType::FromVar(x),
            phantom: PhantomData,
        }
    }

    pub fn new_from_array(id: ProcessUniqueId, x: usize, y: usize) -> VarView<Var> {
        VarView {
            id: id,
            view: VarIndexType::FromArray(x, y),
            phantom: PhantomData,
        }
    }

    pub fn get_idx(&self) -> &VarIndexType {
        &self.view
    }
}

impl<Var: Variable> Into<ViewIndex> for VarView<Var> {
    fn into(self) -> ViewIndex {
        match self.view {
            VarIndexType::FromVar(x) => ViewIndex::new_from_var(self.id, x),
            VarIndexType::FromArray(x, y) => ViewIndex::new_from_array_var(self.id, x, y),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ArrayView<Var: Variable> {
    pub id: ProcessUniqueId,
    x: usize,
    phantom: PhantomData<Var>,
}
impl<Var: Variable> VariableView for ArrayView<Var> {}

impl<Var: Variable> ArrayView<Var> {
    pub fn new(id: ProcessUniqueId, x: usize) -> ArrayView<Var> {
        ArrayView {
            id: id,
            x: x,
            phantom: PhantomData,
        }
    }

    // Change id type to implement partialeq
    pub fn get(&self, y: usize) -> VarView<Var> {
        VarView {
            id: self.id,
            view: VarIndexType::FromArray(self.x, y),
            phantom: PhantomData,
        }
    }

    pub fn get_idx(&self) -> usize {
        self.x
    }
}

impl<Var: Variable> Into<ViewIndex> for ArrayView<Var> {
    fn into(self) -> ViewIndex {
        ViewIndex::new_from_array(self.id, self.x)
    }
}

// Remove Into<ViewIndex>
// ViewIndex given by variablehandler
#[derive(Copy, Clone, Debug)]
pub struct RefArrayView<Var: Variable> {
    id: ProcessUniqueId,
    x: usize,
    phantom: PhantomData<Var>,
}
impl<Var: Variable> VariableView for RefArrayView<Var> {}

impl<Var: Variable> RefArrayView<Var> {
    pub fn new(id: ProcessUniqueId, x: usize) -> RefArrayView<Var> {
        RefArrayView {
            id: id,
            x: x,
            phantom: PhantomData,
        }
    }

    pub fn get_idx(&self) -> usize {
        self.x
    }
}

impl<Var: Variable> Into<ViewIndex> for RefArrayView<Var> {
    fn into(self) -> ViewIndex {
        ViewIndex::new_from_array(self.id, self.x)
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
        use $crate::variables::ViewIndex;
        use $crate::variables::VariableState;
        use $crate::variables::Array;
        use $crate::variables::RefArray;
        use $crate::variables::List;
        use $crate::variables::handlers::macros::{
            VarView,
            ArrayView,
            VarIndexType,
            RefArrayView};
        use $crate::variables::handlers::{
            VariablesHandlerBuilder,
            SpecificVariablesHandler,
            SpecificVariablesHandlerBuilder,
            };
        use snowflake::ProcessUniqueId;

        #[derive(Debug)]
        struct SpecificTypeHandler<Var: Variable> {
            id: ProcessUniqueId,
            variables: Vec<Var>,
            variables_array: Vec<Array<Var>>,
            variables_ref: Vec<RefArray<Var>>,
            variables_ref_view: Vec<Vec<VarView<Var>>>,
        }

        impl<Var: Variable> SpecificTypeHandler<Var> {
            fn new() -> Self {
                SpecificTypeHandler {
                    id: ProcessUniqueId::new(),
                    variables: Vec::new(),
                    variables_array: Vec::new(),
                    variables_ref: Vec::new(),
                    variables_ref_view: Vec::new(),
                }
            }
        }

        impl<Var: Variable> Clone for SpecificTypeHandler<Var> {
            fn clone(&self) -> SpecificTypeHandler<Var> {
                let builder = SpecificTypeHandlerBuilder {
                    id: self.id,
                    variables: self.variables.clone(),
                    variables_array: self.variables_array.clone(),
                    variables_ref_view: self.variables_ref_view.clone(),
                };
                builder.finalize()
            }
        }

        #[derive(Debug,Clone)]
        struct SpecificTypeHandlerBuilder<Var: Variable> {
            id: ProcessUniqueId,
            variables: Vec<Var>,
            variables_array: Vec<Array<Var>>,
            variables_ref_view: Vec<Vec<VarView<Var>>>,
        }

        impl<Var: Variable> SpecificTypeHandlerBuilder<Var> {
            fn new() -> Self {
                SpecificTypeHandlerBuilder {
                    id: ProcessUniqueId::new(),
                    variables: Vec::new(),
                    variables_array: Vec::new(),
                    variables_ref_view: Vec::new(),
                }
            }
            fn finalize(self) -> SpecificTypeHandler<Var> {
                let id = self.id;
                let mut variables = self.variables;
                let mut variables_array = self.variables_array;
                let variables_ref_view = self.variables_ref_view;

                let variables_ref: Vec<RefArray<Var>> = variables_ref_view.iter()
                    .map(|ref views| {
                        let ref_array = views.iter().map(|view| {
                            unsafe{match view.view {
                                VarIndexType::FromVar(x) =>
                                    variables.get_unchecked_mut(x) as *mut _,
                                VarIndexType::FromArray(x,y) =>
                                    variables_array.get_unchecked_mut(x).variables.get_unchecked_mut(y) as *mut _,
                            }}
                        }).collect::<Vec<_>>();
                        RefArray::new(ref_array).unwrap()
                    })
                    .collect();
                //let variables_ref = RefArray::new(variables_ref);
                SpecificTypeHandler {
                    id: id,
                    variables: variables,
                    variables_array: variables_array,
                    variables_ref: variables_ref,
                    variables_ref_view: variables_ref_view,
                }
            }
        }

        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub struct Builder {
            $(
                $type: SpecificTypeHandlerBuilder<$type>
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
                        $type: SpecificTypeHandlerBuilder::new()
                     ),+
                }
            }
        }

        impl VariablesHandlerBuilder<Handler> for Builder {
            fn finalize(self) -> Handler {
                Handler {
                    $(
                        $type: self.$type.finalize()
                     ),+
                }
            }
        }

        impl $crate::variables::handlers::VariablesHandler for Handler {
            fn retrieve_all_changed_states(
                &mut self,
                ) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                let changed_states = vec![$({
                    let id = self.$type.id.clone();
                    let var_states: Vec<(ViewIndex, _)> = self.$type
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
                            let view: ViewIndex =
                                VarView::<$type>::new_from_array(id, x, y).into();
                            (view, state)
                        })
                    .collect();
                    let array_states: Vec<(ViewIndex, _)> = self.$type
                        .variables_array
                        .iter_mut()
                        .enumerate()
                        .map(|(x,val)| (x, val.retrieve_state()))
                        .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,state)| {
                            let view: ViewIndex =
                                VarView::<$type>::new(id, x).into();
                            (view, state)
                        })
                    .collect();
                    let id = self.$type.id.clone();
                    let views: Vec<(ViewIndex, _)> = self.$type.variables
                        .iter_mut()
                        .enumerate()
                        .map(|(x,val)| (x, val.retrieve_state()))
                        .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,state)| {
                            let view: ViewIndex =
                                VarView::<$type>::new(id, x).into();
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
            fn retrieve_changed_states<Views>(
                &mut self,
                views: Views,
                ) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
                where Views: Iterator<Item = ViewIndex> {
                    use $crate::variables::IndexType;
                    let states = views
                        .map(|idx| {
                            // maybe using get_id and get_type?
                            let state = match idx.id {
                                $(
                                    id if id == self.$type.id => {
                                        match idx.index_type {
                                            IndexType::FromVar(x) => {
                                                unsafe {
                                                    self.$type.variables
                                                        .get_unchecked_mut(x)
                                                        .retrieve_state()
                                                }
                                            }
                                            IndexType::FromArray(x) => {
                                                unsafe {
                                                    self.$type.variables
                                                        .get_unchecked_mut(x)
                                                        .retrieve_state()
                                                }
                                            }
                                            IndexType::FromArrayVar(x,y) => {
                                                unsafe {
                                                    self.$type.variables_array
                                                        .get_unchecked_mut(x)
                                                        .variables
                                                        .get_unchecked_mut(y)
                                                        .retrieve_state()
                                                }
                                            }
                                        }
                                    }
                                )+
                                    _ => {unreachable!()}
                            };
                            (idx, state)
                        })
                    .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .collect::<Vec<_>>();
                    Box::new(states.into_iter())
                }
        }

        $(
            impl SpecificVariablesHandlerBuilder<$type, VarView<$type>, Handler, $type>
            for Builder {
                fn add(&mut self, x: $type) -> VarView<$type> {
                    let view = VarView::new(self.$type.id, self.$type.variables.len());
                    self.$type.variables.push(x);
                    view
                }
            }

            impl SpecificVariablesHandlerBuilder<Array<$type>, ArrayView<$type>, Handler, Array<$type>>
            for Builder {
                fn add(&mut self, x: Array<$type>) -> ArrayView<$type> {
                    let view = ArrayView::new(self.$type.id, self.$type.variables_array.len());
                    self.$type.variables_array.push(x);
                    view
                }
            }

            impl SpecificVariablesHandlerBuilder<RefArray<$type>, RefArrayView<$type>, Handler, Vec<VarView<$type>>>
            for Builder {
                fn add(&mut self, x: Vec<VarView<$type>>) -> RefArrayView<$type> {
                    let view = RefArrayView::new(self.$type.id, self.$type.variables_ref_view.len());
                    self.$type.variables_ref_view.push(x);
                    view
                }
            }


            impl SpecificVariablesHandler<$type, VarView<$type>> for Handler {
                fn get_mut(&mut self, view: &VarView<$type>) -> &mut $type {
                    match *view.get_idx() {
                        VarIndexType::FromVar(x) => {
                            unsafe { self.$type.variables.get_unchecked_mut(x) }
                        }
                        VarIndexType::FromArray(x,y) => {
                            unsafe {
                                self.$type.variables_array
                                    .get_unchecked_mut(x)
                                    .variables
                                    .get_unchecked_mut(y)
                            }
                        }
                    }
                }
                fn get(&self, view: &VarView<$type>) -> &$type {
                    match *view.get_idx() {
                        VarIndexType::FromVar(x) => {
                            unsafe { self.$type.variables.get_unchecked(x) }
                        }
                        VarIndexType::FromArray(x,y) => {
                            unsafe {
                                self.$type.variables_array
                                    .get_unchecked(x)
                                    .variables
                                    .get_unchecked(y)
                            }
                        }
                    }
                }

                fn into_indexes(&self, view: &VarView<$type>) -> Box<Iterator<Item = ViewIndex>> {
                    let view_index: ViewIndex = view.clone().into();
                    Box::new(vec![view_index].into_iter())
                }

                fn retrieve_state(&mut self, view: &VarView<$type>) -> VariableState {
                    self.get_mut(view).retrieve_state()
                }

                fn retrieve_states<'a, Views>(
                    &mut self,
                    views: Views,
                    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
                    where
                        Views: Iterator<Item = &'a VarView<$type>>,
                    {
                        let mut states: Vec<(ViewIndex, _)> = Vec::new();
                        for view in views {
                            let state = self.get_mut(view).retrieve_state();
                            let view = view.clone().into();
                            states.push((view,state));
                        }
                        Box::new(states.into_iter())
                    }
                fn retrieve_all_changed_states(
                    &mut self,
                    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                    let id = self.$type.id.clone();
                    let views: Vec<(ViewIndex, _)> = self.$type.variables
                        .iter_mut()
                        .enumerate()
                        .map(|(x,val)| (x, val.retrieve_state()))
                        .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,state)| {
                            let view: ViewIndex = VarView::<$type>::new(id, x).into();
                            (view, state)
                        })
                    .collect();
                    Box::new(views.into_iter())
                }
            }

            impl SpecificVariablesHandler<Array<$type>, ArrayView<$type>> for Handler {
                fn get_mut(&mut self, view: &ArrayView<$type>) -> &mut Array<$type> {
                    unsafe {
                        self.$type.variables_array.get_unchecked_mut(view.get_idx())
                    }
                }
                fn get(&self, view: &ArrayView<$type>) -> & Array<$type> {
                    unsafe {
                        self.$type.variables_array.get_unchecked(view.get_idx())
                    }
                }

                fn retrieve_state(&mut self, view: &ArrayView<$type>) -> VariableState {
                    self.get_mut(view).retrieve_state()
                }

                fn into_indexes(&self, view: &ArrayView<$type>) -> Box<Iterator<Item = ViewIndex>> {
                    let range = 0..self.$type.variables_array[view.get_idx()].len();
                    Box::new(range
                        .map(|y| VarView::<$type>::new_from_array(view.id, view.get_idx(), y))
                        .map(Into::<ViewIndex>::into)
                        .collect::<Vec<_>>()
                        .into_iter()
                    )
                }

                // Optimize by accessing variabl directly in the data structure
                fn retrieve_states<'a, Views>(
                    &mut self,
                    views: Views,
                    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
                    where
                        Views: Iterator<Item = &'a ArrayView<$type>>,
                    {
                        let mut states: Vec<(ViewIndex, _)> = Vec::new();
                        for view in views {
                            {
                                let state = self.get_mut(view).retrieve_state();
                                let view = view.clone().into();
                                states.push((view,state));
                            }
                            for i in 0..(self.get(view).variables.len()) {
                                let view = view.get(i);
                                let state = self.get_mut(&view).retrieve_state();
                                let view = view.clone().into();
                                states.push((view,state));
                            }
                        }
                        Box::new(states.into_iter())
                    }
                fn retrieve_all_changed_states(
                    &mut self,
                    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                    let id = self.$type.id.clone();
                    let var_states: Vec<(ViewIndex, _)> = self.$type
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
                            let view: ViewIndex =
                                VarView::<$type>::new_from_array(id, x, y).into();
                            (view, state)
                        })
                    .collect();
                    let array_states: Vec<(ViewIndex, _)> = self.$type
                        .variables_array
                        .iter_mut()
                        .enumerate()
                        .map(|(x,val)| (x, val.retrieve_state()))
                        .filter(|&(_,ref state)| *state != VariableState::NoChange)
                        .map(|(x,state)| {
                            let view: ViewIndex = VarView::<$type>::new(id, x).into();
                            (view, state)
                        })
                    .collect();
                    Box::new(var_states.into_iter().chain(array_states.into_iter()))
                }
            }

            impl SpecificVariablesHandler<RefArray<$type>, RefArrayView<$type>> for Handler {
                fn get_mut(&mut self, view: &RefArrayView<$type>) -> &mut RefArray<$type> {
                    unsafe {
                        self.$type.variables_ref.get_unchecked_mut(view.get_idx())
                    }
                }
                fn get(&self, view: &RefArrayView<$type>) -> & RefArray<$type> {
                    unsafe {
                        self.$type.variables_ref.get_unchecked(view.get_idx())
                    }
                }

                fn retrieve_state(&mut self, view: &RefArrayView<$type>) -> VariableState {
                    self.get_mut(view).retrieve_state()
                }

                fn into_indexes(&self, view: &RefArrayView<$type>) -> Box<Iterator<Item = ViewIndex>> {
                    Box::new(self.$type.variables_ref_view[view.get_idx()].iter()
                        .cloned()
                        .map(Into::<ViewIndex>::into)
                        .collect::<Vec<_>>()
                        .into_iter()
                    )
                }

                // Optimize by accessing variabl directly in the data structure
                fn retrieve_states<'a, Views>(
                    &mut self,
                    views: Views,
                    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
                    where
                        Views: Iterator<Item = &'a RefArrayView<$type>>,
                    {
                        unimplemented!()
                        //let mut states: Vec<(ViewIndex, _)> = Vec::new();
                        //for view in views {
                            //{
                                //let state = self.get_mut(view).retrieve_state();
                                //let view = view.clone().into();
                                //states.push((view,state));
                            //}
                            //for i in 0..(self.get(view).variables.len()) {
                                //let view = view.get(i);
                                //let state = self.get_mut(&view).retrieve_state();
                                //let view = view.clone().into();
                                //states.push((view,state));
                            //}
                        //}
                        //Box::new(states.into_iter())
                    }
                fn retrieve_all_changed_states(
                    &mut self,
                    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                    unimplemented!()
                    //let id = self.$type.id.clone();
                    //let var_states: Vec<(ViewIndex, _)> = self.$type
                        //.variables_array
                        //.iter_mut()
                        //.enumerate()
                        //.flat_map(|(x,val)| {
                            //val.iter_mut()
                                //.enumerate()
                                //.map(move |(y,val)| (x,y,val.retrieve_state()))
                        //})
                    //.filter(|&(_,_,ref state)| *state != VariableState::NoChange)
                        //.map(|(x,y,state)| {
                            //let view: ViewIndex =
                                //VarView::<$type>::new_from_array(id, x, y).into();
                            //(view, state)
                        //})
                    //.collect();
                    //let array_states: Vec<(ViewIndex, _)> = self.$type
                        //.variables_array
                        //.iter_mut()
                        //.enumerate()
                        //.map(|(x,val)| (x, val.retrieve_state()))
                        //.filter(|&(_,ref state)| *state != VariableState::NoChange)
                        //.map(|(x,state)| {
                            //let view: ViewIndex = VarView::<$type>::new(id, x).into();
                            //(view, state)
                        //})
                    //.collect();
                    //Box::new(var_states.into_iter().chain(array_states.into_iter()))
                }
            }
            )+
    };
}
