use snowflake::ProcessUniqueId;
use std::marker::PhantomData;
use variables::{ArrayOfRefs, ArrayOfVars, ArrayView, Variable, VariableView, ViewIndex};

// move Var and ArrayOfVarsView inside macro => find how to handle extern crate ProcessUniqeId

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VarIndexType {
    FromVar(usize),
    FromArrayOfVars(usize, usize),
}

// CLone and Copy are implemented manually due to phantom data
#[derive(Debug)]
pub struct VarView<Var: Variable> {
    id: ProcessUniqueId,
    pub view: VarIndexType,
    phantom: PhantomData<*const Var>,
}

impl<Var: Variable> Clone for VarView<Var> {
    fn clone(&self) -> VarView<Var> {
        VarView {
            id: self.id,
            view: self.view,
            phantom: PhantomData,
        }
    }
}
impl<Var: Variable> Copy for VarView<Var> {}
impl<Var: Variable> VariableView for VarView<Var> {
    type Variable = Var;
}

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
            view: VarIndexType::FromArrayOfVars(x, y),
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
            VarIndexType::FromArrayOfVars(x, y) => {
                ViewIndex::new_from_array_var(self.id, x, y)
            }
        }
    }
}

// Add len field
#[derive(Debug)]
pub struct ArrayOfVarsView<Var: Variable> {
    pub id: ProcessUniqueId,
    x: usize,
    phantom: PhantomData<Var>,
}

impl<Var: Variable> Clone for ArrayOfVarsView<Var> {
    fn clone(&self) -> ArrayOfVarsView<Var> {
        ArrayOfVarsView {
            id: self.id,
            x: self.x,
            phantom: PhantomData,
        }
    }
}
impl<Var: Variable> Copy for ArrayOfVarsView<Var> {}
impl<Var: Variable> ArrayView for ArrayOfVarsView<Var> {
    type Variable = Var;
    type Array = ArrayOfVars<Var>;
}

impl<Var: Variable> ArrayOfVarsView<Var> {
    pub fn new(id: ProcessUniqueId, x: usize) -> ArrayOfVarsView<Var> {
        ArrayOfVarsView {
            id: id,
            x: x,
            phantom: PhantomData,
        }
    }

    //pub fn len(&self) -> usize {}

    // Change id type to implement partialeq
    //pub fn get(&self, y: usize) -> Option<VarView<Var>> {
    pub fn get(&self, y: usize) -> VarView<Var> {
        VarView {
            id: self.id,
            view: VarIndexType::FromArrayOfVars(self.x, y),
            phantom: PhantomData,
        }
    }

    pub fn get_idx(&self) -> usize {
        self.x
    }
}

//impl<Var: Variable> Index<usize> for ArrayOfVarsView<Var> {
//type Output = VarView<Var>;
//fn index(&self, idx: usize) -> &VarView<Var> {
//&self.get(idx)
//}
//}

impl<Var: Variable> Into<ViewIndex> for ArrayOfVarsView<Var> {
    fn into(self) -> ViewIndex {
        ViewIndex::new_from_array(self.id, self.x)
    }
}

// Remove Into<ViewIndex>
// ViewIndex given by variablehandler
#[derive(Debug)]
pub struct ArrayOfRefsView<Var: Variable> {
    id: ProcessUniqueId,
    x: usize,
    phantom: PhantomData<Var>,
}
impl<Var: Variable> Clone for ArrayOfRefsView<Var> {
    fn clone(&self) -> ArrayOfRefsView<Var> {
        ArrayOfRefsView {
            id: self.id,
            x: self.x,
            phantom: PhantomData,
        }
    }
}
impl<Var: Variable> Copy for ArrayOfRefsView<Var> {}
impl<Var: Variable> ArrayView for ArrayOfRefsView<Var> {
    type Variable = Var;
    type Array = ArrayOfRefs<Var>;
}

impl<Var: Variable> ArrayOfRefsView<Var> {
    pub fn new(id: ProcessUniqueId, x: usize) -> ArrayOfRefsView<Var> {
        ArrayOfRefsView {
            id: id,
            x: x,
            phantom: PhantomData,
        }
    }

    pub fn get_idx(&self) -> usize {
        self.x
    }
}

impl<Var: Variable> Into<ViewIndex> for ArrayOfRefsView<Var> {
    fn into(self) -> ViewIndex {
        ViewIndex::new_from_array(self.id, self.x)
    }
}

// OTHER SYNTAX
// variables_handler_build!(
//      IntVar,
//      ArrayOfVars of IntVar,
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
        //use $crate::variables::VariableState;
        use $crate::variables::ArrayOfVars;
        use $crate::variables::ArrayOfRefs;
        use $crate::variables::Array;
        use $crate::variables::handlers::macros::{
            VarView,
            ArrayOfVarsView,
            VarIndexType,
            ArrayOfRefsView};
        use $crate::variables::handlers::{
            VariablesHandlerBuilder,
            SpecificVariablesHandler,
            SpecificVariablesHandlerBuilder,
            SpecificArraysHandler,
            SpecificArraysHandlerBuilder
            };
        use snowflake::ProcessUniqueId;

        #[derive(Debug)]
        struct SpecificTypeHandler<Var: Variable> {
            id: ProcessUniqueId,
            variables: Vec<Var>,
            variables_array: Vec<ArrayOfVars<Var>>,
            variables_ref: Vec<ArrayOfRefs<Var>>,
            variables_ref_view: Vec<Vec<VarView<Var>>>,
        }

        impl<Var: Variable> SpecificTypeHandler<Var> {
            //fn new() -> Self {
                //SpecificTypeHandler {
                    //id: ProcessUniqueId::new(),
                    //variables: Vec::new(),
                    //variables_array: Vec::new(),
                    //variables_ref: Vec::new(),
                    //variables_ref_view: Vec::new(),
                //}
            //}
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
            variables_array: Vec<ArrayOfVars<Var>>,
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

                let variables_ref: Vec<ArrayOfRefs<Var>> = variables_ref_view.iter()
                    .map(|ref views| {
                        let ref_array = views.iter().map(|view| {
                            unsafe{match view.view {
                                VarIndexType::FromVar(x) =>
                                    variables.get_unchecked_mut(x) as *mut _,
                                VarIndexType::FromArrayOfVars(x,y) =>
                                    variables_array.get_unchecked_mut(x).variables.get_unchecked_mut(y) as *mut _,
                            }}
                        }).collect::<Vec<_>>();
                        ArrayOfRefs::new(ref_array).unwrap()
                    })
                    .collect();
                //let variables_ref = ArrayOfRefs::new(variables_ref);
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
            //fn retrieve_all_changed_states(
                //&mut self,
                //) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                //let changed_states = vec![$({
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
                            //let view: ViewIndex =
                                //VarView::<$type>::new(id, x).into();
                            //(view, state)
                        //})
                    //.collect();
                    //let id = self.$type.id.clone();
                    //let views: Vec<(ViewIndex, _)> = self.$type.variables
                        //.iter_mut()
                        //.enumerate()
                        //.map(|(x,val)| (x, val.retrieve_state()))
                        //.filter(|&(_,ref state)| *state != VariableState::NoChange)
                        //.map(|(x,state)| {
                            //let view: ViewIndex =
                                //VarView::<$type>::new(id, x).into();
                            //(view, state)
                        //})
                    //.collect();
                    //Box::new(
                        //var_states.into_iter()
                        //.chain(array_states.into_iter())
                        //.chain(views.into_iter()))
                //}),+];
                //Box::new(
                    //changed_states.into_iter().flat_map(|changes| changes)
                    //)
            //}
            //fn retrieve_changed_states<Views>(
                //&mut self,
                //views: Views,
                //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
                //where Views: Iterator<Item = ViewIndex> {
                    //use $crate::variables::IndexType;
                    //let states = views
                        //.map(|idx| {
                            //// maybe using get_id and get_type?
                            //let state = match idx.id {
                                //$(
                                    //id if id == self.$type.id => {
                                        //match idx.index_type {
                                            //IndexType::FromVar(x) => {
                                                //unsafe {
                                                    //self.$type.variables
                                                        //.get_unchecked_mut(x)
                                                        //.retrieve_state()
                                                //}
                                            //}
                                            //IndexType::FromArrayOfVars(x) => {
                                                //unsafe {
                                                    //self.$type.variables
                                                        //.get_unchecked_mut(x)
                                                        //.retrieve_state()
                                                //}
                                            //}
                                            //IndexType::FromArrayOfVarsVar(x,y) => {
                                                //unsafe {
                                                    //self.$type.variables_array
                                                        //.get_unchecked_mut(x)
                                                        //.variables
                                                        //.get_unchecked_mut(y)
                                                        //.retrieve_state()
                                                //}
                                            //}
                                        //}
                                    //}
                                //)+
                                    //_ => {unreachable!()}
                            //};
                            //(idx, state)
                        //})
                    //.filter(|&(_,ref state)| *state != VariableState::NoChange)
                        //.collect::<Vec<_>>();
                    //Box::new(states.into_iter())
                //}
        }

        $(
            impl SpecificVariablesHandlerBuilder<VarView<$type>, Handler, $type>
            for Builder {
                fn add(&mut self, x: $type) -> VarView<$type> {
                    let view = VarView::new(self.$type.id, self.$type.variables.len());
                    self.$type.variables.push(x);
                    view
                }
            }

            impl SpecificArraysHandlerBuilder<ArrayOfVarsView<$type>, Handler, ArrayOfVars<$type>>
            for Builder {
                fn add_array(&mut self, x: ArrayOfVars<$type>) -> ArrayOfVarsView<$type> {
                    let view = ArrayOfVarsView::new(self.$type.id, self.$type.variables_array.len());
                    self.$type.variables_array.push(x);
                    view
                }
            }

            impl SpecificArraysHandlerBuilder<ArrayOfRefsView<$type>, Handler, Vec<VarView<$type>>>
            for Builder {
                fn add_array(&mut self, x: Vec<VarView<$type>>) -> ArrayOfRefsView<$type> {
                    let view = ArrayOfRefsView::new(self.$type.id, self.$type.variables_ref_view.len());
                    self.$type.variables_ref_view.push(x);
                    view
                }
            }


            impl SpecificVariablesHandler<VarView<$type>> for Handler {
                fn get_mut(&mut self, view: &VarView<$type>) -> &mut $type {
                    match *view.get_idx() {
                        VarIndexType::FromVar(x) => {
                            unsafe { self.$type.variables.get_unchecked_mut(x) }
                        }
                        VarIndexType::FromArrayOfVars(x,y) => {
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
                        VarIndexType::FromArrayOfVars(x,y) => {
                            unsafe {
                                self.$type.variables_array
                                    .get_unchecked(x)
                                    .variables
                                    .get_unchecked(y)
                            }
                        }
                    }
                }

                fn get_variable_id(&self, view: &VarView<$type>) -> ViewIndex {
                     view.clone().into()
                }

                //fn into_indexes(&self, view: &VarView<$type>) -> Box<Iterator<Item = ViewIndex>> {
                    //let view_index: ViewIndex = view.clone().into();
                    //Box::new(vec![view_index].into_iter())
                //}

                //fn retrieve_state(&mut self, view: &VarView<$type>) -> VariableState {
                    //self.get_mut(view).retrieve_state()
                //}

                //fn retrieve_states<'a, Views>(
                    //&mut self,
                    //views: Views,
                    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
                    //where
                        //Views: Iterator<Item = &'a VarView<$type>>,
                    //{
                        //let mut states: Vec<(ViewIndex, _)> = Vec::new();
                        //for view in views {
                            //let state = self.get_mut(view).retrieve_state();
                            //let view = view.clone().into();
                            //states.push((view,state));
                        //}
                        //Box::new(states.into_iter())
                    //}
                //fn retrieve_all_changed_states(
                    //&mut self,
                    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                    //let id = self.$type.id.clone();
                    //let views: Vec<(ViewIndex, _)> = self.$type.variables
                        //.iter_mut()
                        //.enumerate()
                        //.map(|(x,val)| (x, val.retrieve_state()))
                        //.filter(|&(_,ref state)| *state != VariableState::NoChange)
                        //.map(|(x,state)| {
                            //let view: ViewIndex = VarView::<$type>::new(id, x).into();
                            //(view, state)
                        //})
                    //.collect();
                    //Box::new(views.into_iter())
                //}
            }

            impl SpecificArraysHandler<ArrayOfVarsView<$type>> for Handler {
                fn get_array_mut(&mut self, view: &ArrayOfVarsView<$type>) -> &mut ArrayOfVars<$type> {
                    unsafe {
                        self.$type.variables_array.get_unchecked_mut(view.get_idx())
                    }
                }
                fn get_array(&self, view: &ArrayOfVarsView<$type>) -> & ArrayOfVars<$type> {
                    unsafe {
                        self.$type.variables_array.get_unchecked(view.get_idx())
                    }
                }

                fn get_array_id(&self, view: &ArrayOfVarsView<$type>, position: usize) ->  ViewIndex {
                    VarView::<$type>::new_from_array(view.id, view.get_idx(), position).into()
                }

                fn get_array_ids(&self, view: &ArrayOfVarsView<$type>) -> Box<Iterator<Item = ViewIndex>> {
                    let range = 0..self.$type.variables_array[view.get_idx()].len();
                    Box::new(range
                        .map(|y| VarView::<$type>::new_from_array(view.id, view.get_idx(), y))
                        .map(Into::<ViewIndex>::into)
                        .collect::<Vec<_>>()
                        .into_iter()
                    )
                }

                //fn retrieve_state(&mut self, view: &ArrayOfVarsView<$type>) -> VariableState {
                    //self.get_mut(view).retrieve_state()
                //}

                //fn into_indexes(&self, view: &ArrayOfVarsView<$type>) -> Box<Iterator<Item = ViewIndex>> {
                    //let range = 0..self.$type.variables_array[view.get_idx()].len();
                    //Box::new(range
                        //.map(|y| VarView::<$type>::new_from_array(view.id, view.get_idx(), y))
                        //.map(Into::<ViewIndex>::into)
                        //.collect::<Vec<_>>()
                        //.into_iter()
                    //)
                //}

                //// Optimize by accessing variabl directly in the data structure
                //fn retrieve_states<'a, Views>(
                    //&mut self,
                    //views: Views,
                    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
                    //where
                        //Views: Iterator<Item = &'a ArrayOfVarsView<$type>>,
                    //{
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
                    //}
                //fn retrieve_all_changed_states(
                    //&mut self,
                    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
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
                //}
            }

            impl SpecificArraysHandler<ArrayOfRefsView<$type>> for Handler {
                fn get_array_mut(&mut self, view: &ArrayOfRefsView<$type>) -> &mut ArrayOfRefs<$type> {
                    unsafe {
                        self.$type.variables_ref.get_unchecked_mut(view.get_idx())
                    }
                }
                fn get_array(&self, view: &ArrayOfRefsView<$type>) -> &ArrayOfRefs<$type> {
                    unsafe {
                        self.$type.variables_ref.get_unchecked(view.get_idx())
                    }
                }

                fn get_array_id(&self, view: &ArrayOfRefsView<$type>, position: usize) ->  ViewIndex {
                    self.$type.variables_ref_view[view.get_idx()][position].into()
                }

                fn get_array_ids(&self, view: &ArrayOfRefsView<$type>) -> Box<Iterator<Item = ViewIndex>> {
                    Box::new(self.$type.variables_ref_view[view.get_idx()].iter()
                        .cloned()
                        .map(Into::<ViewIndex>::into)
                        .collect::<Vec<_>>()
                        .into_iter()
                    )
                }

                //fn retrieve_state(&mut self, view: &ArrayOfRefsView<$type>) -> VariableState {
                    //self.get_mut(view).retrieve_state()
                //}

                //fn into_indexes(&self, view: &ArrayOfRefsView<$type>) -> Box<Iterator<Item = ViewIndex>> {
                    //Box::new(self.$type.variables_ref_view[view.get_idx()].iter()
                        //.cloned()
                        //.map(Into::<ViewIndex>::into)
                        //.collect::<Vec<_>>()
                        //.into_iter()
                    //)
                //}

                //// Optimize by accessing variabl directly in the data structure
                //fn retrieve_states<'a, Views>(
                    //&mut self,
                    //_views: Views,
                    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
                    //where
                        //Views: Iterator<Item = &'a ArrayOfRefsView<$type>>,
                    //{
                        //unimplemented!()
                        ////let mut states: Vec<(ViewIndex, _)> = Vec::new();
                        ////for view in views {
                            ////{
                                ////let state = self.get_mut(view).retrieve_state();
                                ////let view = view.clone().into();
                                ////states.push((view,state));
                            ////}
                            ////for i in 0..(self.get(view).variables.len()) {
                                ////let view = view.get(i);
                                ////let state = self.get_mut(&view).retrieve_state();
                                ////let view = view.clone().into();
                                ////states.push((view,state));
                            ////}
                        ////}
                        ////Box::new(states.into_iter())
                    //}
                //fn retrieve_all_changed_states(
                    //&mut self,
                    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                    //unimplemented!()
                    ////let id = self.$type.id.clone();
                    ////let var_states: Vec<(ViewIndex, _)> = self.$type
                        ////.variables_array
                        ////.iter_mut()
                        ////.enumerate()
                        ////.flat_map(|(x,val)| {
                            ////val.iter_mut()
                                ////.enumerate()
                                ////.map(move |(y,val)| (x,y,val.retrieve_state()))
                        ////})
                    ////.filter(|&(_,_,ref state)| *state != VariableState::NoChange)
                        ////.map(|(x,y,state)| {
                            ////let view: ViewIndex =
                                ////VarView::<$type>::new_from_array(id, x, y).into();
                            ////(view, state)
                        ////})
                    ////.collect();
                    ////let array_states: Vec<(ViewIndex, _)> = self.$type
                        ////.variables_array
                        ////.iter_mut()
                        ////.enumerate()
                        ////.map(|(x,val)| (x, val.retrieve_state()))
                        ////.filter(|&(_,ref state)| *state != VariableState::NoChange)
                        ////.map(|(x,state)| {
                            ////let view: ViewIndex = VarView::<$type>::new(id, x).into();
                            ////(view, state)
                        ////})
                    ////.collect();
                    ////Box::new(var_states.into_iter().chain(array_states.into_iter()))
                //}
            }
            )+
    };
}
