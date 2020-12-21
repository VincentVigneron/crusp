use snowflake::ProcessUniqueId;
use std::marker::PhantomData;
use std::sync::Arc;
use variables::{ArrayOfRefs, ArrayOfVars, Variable};

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

unsafe impl<Var: Variable> Send for VarView<Var> {}

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
//impl<Var: Variable> VariableContainerView for VarView<Var> {
//type Container = Var;
//type Variable = Var;
//}

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

// Add len field
#[derive(Debug)]
pub struct ArrayOfVarsView<Var: Variable> {
    pub id: ProcessUniqueId,
    x: usize,
    phantom: PhantomData<Var>,
}
unsafe impl<Var: Variable> Send for ArrayOfVarsView<Var> {}

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
//impl<Var: Variable> VariableContainerView for ArrayOfVarsView<Var> {
//type Container = ArrayOfVars<Var>;
//type Variable = Var;
//}

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

#[derive(Debug)]
pub struct ArrayOfRefsView<Var: Variable> {
    id: ProcessUniqueId,
    x: usize,
    phantom: PhantomData<Var>,
}
unsafe impl<Var: Variable> Send for ArrayOfRefsView<Var> {}
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
//impl<Var: Variable> VariableContainerView for ArrayOfRefsView<Var> {
//type Container = ArrayOfRefs<Var>;
//type Variable = Var;
//}

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

#[derive(Debug, Clone)]
pub struct VariableHandlerBuilder<Var: Variable> {
    pub id: ProcessUniqueId,
    pub variables: Vec<Var>,
    pub variables_array: Vec<ArrayOfVars<Var>>,
    pub variables_ref_view: Vec<Arc<Vec<VarView<Var>>>>,
}

impl<Var: Variable> VariableHandlerBuilder<Var> {
    pub fn new() -> Self {
        VariableHandlerBuilder {
            id: ProcessUniqueId::new(),
            variables: Vec::new(),
            variables_array: Vec::new(),
            variables_ref_view: Vec::new(),
        }
    }
    pub fn finalize(self) -> VariableHandler<Var> {
        let id = self.id;
        let mut variables = self.variables;
        let mut variables_array = self.variables_array;
        let variables_ref_view = self.variables_ref_view;

        let variables_ref: Vec<ArrayOfRefs<Var>> = variables_ref_view
            .iter()
            .map(|ref views| {
                let ref_array = views
                    .iter()
                    .map(|view| unsafe {
                        match view.view {
                            VarIndexType::FromVar(x) => {
                                variables.get_unchecked_mut(x) as *mut _
                            }
                            VarIndexType::FromArrayOfVars(x, y) => variables_array
                                .get_unchecked_mut(x)
                                .variables
                                .get_unchecked_mut(y)
                                as *mut _,
                        }
                    }).collect::<Vec<_>>();
                ArrayOfRefs::new(ref_array).unwrap()
            }).collect();
        VariableHandler {
            id: id,
            variables: variables,
            variables_array: variables_array,
            variables_ref: variables_ref,
            variables_ref_view: variables_ref_view,
        }
    }
}

#[derive(Debug)]
pub struct VariableHandler<Var: Variable> {
    pub id: ProcessUniqueId,
    pub variables: Vec<Var>,
    pub variables_array: Vec<ArrayOfVars<Var>>,
    pub variables_ref: Vec<ArrayOfRefs<Var>>,
    pub variables_ref_view: Vec<Arc<Vec<VarView<Var>>>>,
}
unsafe impl<Var: Variable> Send for VariableHandler<Var> {}
unsafe impl<Var: Variable> Sync for VariableHandler<Var> {}
impl<Var: Variable> VariableHandler<Var> {}
impl<Var: Variable> Clone for VariableHandler<Var> {
    fn clone(&self) -> VariableHandler<Var> {
        let builder = VariableHandlerBuilder {
            id: self.id,
            variables: self.variables.clone(),
            variables_array: self.variables_array.clone(),
            variables_ref_view: self.variables_ref_view.clone(),
        };
        builder.finalize()
    }
}

#[macro_export]
macro_rules! variables_handler_build {
    ($($builder: ident),+) => {
        use $crate::variables::{
            VariableBuilder,  ArrayBuilder,
            ArrayOfVars,ArrayOfVarsBuilder,
            ArrayOfRefs
        };
        use $crate::variables::handlers::macros::{
            ArrayOfVarsView, ArrayOfRefsView, VarView,
            VariableHandler, VariableHandlerBuilder, VarIndexType
        };
        use $crate::variables::handlers::{
            VariablesHandlerBuilder,
            VariableContainerHandler,
            VariableContainerHandlerBuilder,
        };
        use std::sync::Arc;

        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub struct Builder {
            $(
                $builder: VariableHandlerBuilder<<$builder as VariableBuilder>::Variable>
             ),+,
             var_id: usize,
        }

        #[derive(Debug,Clone)]
        #[allow(non_snake_case)]
        pub struct Handler {
            $(
                $builder: VariableHandler<<$builder as VariableBuilder>::Variable>
             ),+
        }

        impl Builder {
            pub fn new() -> Builder {
                Builder {
                    $(
                        $builder: VariableHandlerBuilder::new()
                     ),+,
                     var_id: 0,
                }
            }

            pub fn new_id(&mut self) -> usize {
                let id = self.var_id;
                self.var_id += 1;
                id
            }
        }

        impl VariablesHandlerBuilder<Handler> for Builder {
            fn new_builder() -> Self {
                Self::new()
            }
            fn finalize(self) -> Handler {
                Handler {
                    $(
                        $builder: self.$builder.finalize()
                     ),+
                }
            }
        }

        impl $crate::variables::handlers::VariablesHandler for Handler {}
        unsafe impl Sync for Handler {}
        unsafe impl Send for Handler {}

        $(
            impl VariableContainerHandlerBuilder<
                <$builder as VariableBuilder>::Variable,
                VarView<<$builder as VariableBuilder>::Variable>,
                Handler,
                $builder
            > for Builder {
                fn add(&mut self, x: $builder) -> VarView<<$builder as VariableBuilder>::Variable> {
                    let view = VarView::new(self.$builder.id, self.$builder.variables.len());
                    let id = self.new_id();
                    self.$builder.variables.push(x.finalize(id));
                    view
                }
            }

            impl VariableContainerHandlerBuilder<
                ArrayOfVars<<$builder as VariableBuilder>::Variable>,
                ArrayOfVarsView<<$builder as VariableBuilder>::Variable>,
                Handler,
                ArrayOfVarsBuilder<$builder>
            > for Builder {
                fn add(&mut self, x: ArrayOfVarsBuilder<$builder>) -> ArrayOfVarsView<<$builder as VariableBuilder>::Variable> {
                    let view = ArrayOfVarsView::new(self.$builder.id, self.$builder.variables_array.len());
                    let x = x.into_iter().map(|val| val.finalize(self.new_id())).collect::<Vec<_>>();
                    let x: ArrayOfVars<<$builder as VariableBuilder>::Variable> = ArrayOfVars::new_from_iter(x.into_iter()).unwrap();
                    self.$builder.variables_array.push(x);
                    view
                }
            }

            impl VariableContainerHandlerBuilder<
                ArrayOfRefs<<$builder as VariableBuilder>::Variable>,
                ArrayOfRefsView<<$builder as VariableBuilder>::Variable>,
                Handler,
                Vec<VarView<<$builder as VariableBuilder>::Variable>>
            > for Builder {
                fn add(&mut self, x: Vec<VarView<<$builder as VariableBuilder>::Variable>>)
                    -> ArrayOfRefsView<<$builder as VariableBuilder>::Variable>
                {
                    let view = ArrayOfRefsView::new(self.$builder.id, self.$builder.variables_ref_view.len());
                    self.$builder.variables_ref_view.push(Arc::new(x));
                    view
                }
            }


            impl VariableContainerHandler<<$builder as VariableBuilder>::Variable> for Handler {
                type View = VarView<<$builder as VariableBuilder>::Variable>;

                fn get_mut(&mut self, view: &VarView<<$builder as VariableBuilder>::Variable>) -> &mut <$builder as VariableBuilder>::Variable {
                    match *view.get_idx() {
                        VarIndexType::FromVar(x) => {
                            unsafe { self.$builder.variables.get_unchecked_mut(x) }
                        }
                        VarIndexType::FromArrayOfVars(x,y) => {
                            unsafe {
                                self.$builder.variables_array
                                    .get_unchecked_mut(x)
                                    .variables
                                    .get_unchecked_mut(y)
                            }
                        }
                    }
                }
                fn get(&self, view: &VarView<<$builder as VariableBuilder>::Variable>) -> &<$builder as VariableBuilder>::Variable {
                    match *view.get_idx() {
                        VarIndexType::FromVar(x) => {
                            unsafe { self.$builder.variables.get_unchecked(x) }
                        }
                        VarIndexType::FromArrayOfVars(x,y) => {
                            unsafe {
                                self.$builder.variables_array
                                    .get_unchecked(x)
                                    .variables
                                    .get_unchecked(y)
                            }
                        }
                    }
                }
            }

            impl VariableContainerHandler<ArrayOfVars<<$builder as VariableBuilder>::Variable>> for Handler {
                type View = ArrayOfVarsView<<$builder as VariableBuilder>::Variable>;
                fn get_mut(&mut self, view: &ArrayOfVarsView<<$builder as VariableBuilder>::Variable>)
                    -> &mut ArrayOfVars<<$builder as VariableBuilder>::Variable>
                {
                    unsafe {
                        self.$builder.variables_array.get_unchecked_mut(view.get_idx())
                    }
                }
                fn get(&self, view: &ArrayOfVarsView<<$builder as VariableBuilder>::Variable>)
                    -> &ArrayOfVars<<$builder as VariableBuilder>::Variable>
                {
                    unsafe {
                        self.$builder.variables_array.get_unchecked(view.get_idx())
                    }
                }
            }

            impl VariableContainerHandler<ArrayOfRefs<<$builder as VariableBuilder>::Variable>> for Handler {
                type View = ArrayOfRefsView<<$builder as VariableBuilder>::Variable>;
                fn get_mut(&mut self, view: &ArrayOfRefsView<<$builder as VariableBuilder>::Variable>)
                    -> &mut ArrayOfRefs<<$builder as VariableBuilder>::Variable>
                {
                    unsafe {
                        self.$builder.variables_ref.get_unchecked_mut(view.get_idx())
                    }
                }
                fn get(&self, view: &ArrayOfRefsView<<$builder as VariableBuilder>::Variable>)
                    -> &ArrayOfRefs<<$builder as VariableBuilder>::Variable>
                {
                    unsafe {
                        self.$builder.variables_ref.get_unchecked(view.get_idx())
                    }
                }
            }
        )+
    };
}
