extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use syn::Token;
use quote::quote;

enum AttributType {
    Var, Val, State
}

macro_rules! span {
    () => {{
        syn::export::Span::call_site()
    }}
}

#[derive(Debug,Clone)]
struct GraphElt {
    ident: syn::Ident,
    node: syn::Ident,
    event: syn::Ident,
}

#[derive(Debug,Clone)]
struct GraphStructure {
    ident: syn::Ident,
    out: GraphElt,
    ins: Vec<GraphElt>,
}

fn read_typepath_ident(path: &syn::TypePath) -> syn::Ident {
    path.path.segments.first().expect("One type").value().ident.clone()
}

fn pair_to_idents(pair: &syn::TypeTuple) -> (syn::Ident, syn::Ident) {
    let pair: Vec<_> = pair.elems.iter()
        .map(|tp|
            if let syn::Type::Path(ref tp) = tp {
                tp
            } else {
                unimplemented!()
            }
        )
        .collect();
    if pair.len() != 2 {
        panic!()
    }
    (
        read_typepath_ident(pair[0]),
        read_typepath_ident(pair[1]),
    )
}

fn field_to_graph_elt(field: &syn::Field) -> GraphElt {
    let ident = field.ident.clone().expect("Identifier expected");
    let tuple = if let syn::Type::Tuple(ref tuple) = field.ty {
        tuple
    } else {
        unimplemented!()
    };
    let (node, event) = pair_to_idents(tuple);
    GraphElt {
        ident: ident,
        node: node,
        event: event,
    }
}

// TODO(vincent): check if item is a DataStruct
#[proc_macro_attribute]
pub fn crusp_lazy_graph(attr: TokenStream, item: TokenStream) -> TokenStream {
    let  ast = parse_macro_input!(item as DeriveInput);
    //eprintln!("{:#?}", ast);
    let data = if let syn::Data::Struct(ref data) =  ast.data {
        data
    } else {
        unimplemented!()
    };
    // get visibility
    let ident = ast.ident.clone();
    let ident_builder = format!("{}Builder", ast.ident);
    let graph_ident_builder = syn::Ident::new(&ident_builder, span!());
    let fields = if let syn::Fields::Named(ref fields) = data.fields {
        &fields.named
    } else {
        unimplemented!()
    };
    let (out, ins): (Vec<_>, Vec<_>) = fields.iter()
        .partition(|field| field.attrs[0].path.segments[0].ident == "output");
    if out.len() != 1 {
        panic!()
    }
    if ins.len() < 1 {
        panic!()
    }
    let out = out.into_iter()
        .map(|field| field_to_graph_elt(field))
        .next()
        .expect("Exactly one element");
    let ins: Vec<_> = ins.into_iter()
        .map(|field| field_to_graph_elt(field))
        .collect();
    let graph = GraphStructure {
        ident: ident,
        out: out,
        ins: ins,
    };
    //eprintln!("{:#?}", graph);

    let graph_ident = graph.ident;
    let (out_ident, out_node, out_event) =
        (graph.out.ident, graph.out.node, graph.out.event);

    let out_field =  quote!(
        #out_ident: ::crusp_graph::HandlerOutput<#out_node, #out_event>
    );
    let out_builder_field = quote!(
        #out_ident: ::crusp_graph::HandlerOutputBuilder<#out_node, #out_event>
    );
    let out_where = quote!(
            #out_node: ::crusp_graph::GraphNode,
            #out_event: ::crusp_graph::GraphEvent
    );
    let in_builder_fields: Vec<_> = graph.ins.iter()
        .map(|field| {
            let (ident, node, event) =
                (field.ident.clone(), field.node.clone(), field.event.clone());
            quote!(
                #ident: ::crusp_graph::LazyInputEventGraphBuilder<
                    #node,
                    #event,
                    ::crusp_graph::OutCostEventLink<#out_event>
                >
            )
        })
        .collect();
    let in_rev_builder_fields: Vec<_> = graph.ins.iter()
        .map(|field| {
            let (ident, in_node, event) =
                (field.ident.clone(), field.node.clone(), field.event.clone());
            let ident_name = format!("__crusp__rev_{}", ident);
            let ident = syn::Ident::new(&ident_name, span!());
            let out_node = out_node.clone();
            quote!(
                #ident: ::crusp_graph::AdjacentListGraphBuilder<
                    #out_node,
                    #in_node,
                >
            )
        })
        .collect();
    let in_fields: Vec<_> = graph.ins.iter()
        .map(|field| {
            let (ident, node, event) =
                (field.ident.clone(), field.node.clone(), field.event.clone());
            quote!(
                #ident: ::crusp_graph::LazyInputEventHandler<
                    #node,
                    #event,
                    ::crusp_graph::OutCostEventLink<#out_event>
                >
            )
        })
        .collect();
    let in_rev_fields: Vec<_> = graph.ins.iter()
        .map(|field| {
            let (ident, in_node, event) =
                (field.ident.clone(), field.node.clone(), field.event.clone());
            let ident_name = format!("__crusp__rev_{}", ident);
            let ident = syn::Ident::new(&ident_name, span!());
            let out_node = out_node.clone();
            quote!(
                #ident: ::std::rc::Rc<::crusp_graph::AdjacentListGraph<
                    #out_node,
                    #in_node,
                >>
            )
        })
        .collect();
    let in_events_handler: Vec<_> = graph.ins.iter()
        .map(|field| {
        let (ident, node, event) =
            (field.ident.clone(), field.node.clone(), field.event.clone());
            let out_node = out_node.clone();
            let out_event = out_event.clone();
            let in_node = node.clone();
            let in_event = event.clone();
            quote!(
                impl ::crusp_graph::InputEventHandler<#in_node, #in_event> for #graph_ident
                {
                    #[allow(clippy::inline_always)]
                    #[inline(always)]
                     fn notify(&mut self, in_node: &#in_node, in_event: &#in_event) -> bool {
                         if self.#ident.notify(in_node, in_event) {
                            true
                         } else {
                             false
                         }
                    }
                }
            )
        })
        .collect();
    let inout_events_handler: Vec<_> = graph.ins.iter()
        .map(|field| {
            let graph_ident_builder = graph_ident_builder.clone();
            let (in_ident, node, event) =
                (field.ident.clone(), field.node.clone(), field.event.clone());
            let rev_ident = format!("__crusp__rev_{}", in_ident);
            let rev_ident = syn::Ident::new(&rev_ident, span!());
            let out_node = out_node.clone();
            let out_event = out_event.clone();
            let out_ident = out_ident.clone();
            let in_node = node.clone();
            let in_event = event.clone();
            quote!(
                impl ::crusp_graph::InOutEventHandlerBuilder<#out_node, #out_event, #in_node, #in_event>
                    for #graph_ident_builder
                {
                    fn add_event(&mut self, out_node: &#out_node, out_event: &#out_event, in_node: &#in_node, in_event: &#in_event, cost: i64) {
                        let idx = self.#out_ident.add_node(*out_node);
                        let out = <::crusp_graph::OutCostEventLink<#out_event>>::new(
                            idx,
                            *out_event,
                            cost
                        );
                        self.#in_ident.add_event(*in_node, *in_event, out);
                        self.#rev_ident.add_node(out_node, in_node);
                    }
                }
            )
        })
        .collect();
    let graph_builder_impl = {
        let graph_ident_builder = graph_ident_builder.clone();
        let graph_ident = graph_ident.clone();
        let out_ident = out_ident.clone();
        let out_node = out_node.clone();
        let out_event = out_event.clone();
        let in_idents: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.ident.clone();
                quote!(#field)
            })
            .collect();
        let in_rev_idents: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.ident.clone();
                let rev_field = format!("__crusp__rev_{}", field);
                let rev_field = syn::Ident::new(&rev_field, span!());
                quote!(#rev_field)
            })
            .collect();
        let in_idents2 = in_idents.clone();
        let in_idents3 = in_idents.clone();
        let out_events: Vec<_> = std::iter::repeat(&out_event)
            .clone()
            .take(in_rev_idents.len())
            .collect();
        let out_nodes: Vec<_> = std::iter::repeat(&out_node)
            .clone()
            .take(in_idents.len())
            .collect();
        let in_nodes: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.node.clone();
                quote!(#field)
            })
            .collect();
        let in_events: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.event.clone();
                quote!(#field)
            })
            .collect();
        let out_events2 = out_events.clone();
        let in_nodes2 = in_nodes.clone();
        let in_events2 = in_events.clone();
        let out_events2 = out_events.clone();
        let in_rev_idents2 = in_rev_idents.clone();
        let in_rev_idents3 = in_rev_idents.clone();
        let in_rev_nodes = in_nodes.clone();

        quote!(
            impl #graph_ident_builder
            {
                pub fn new() -> Self {
                    #graph_ident_builder {
                        #(#in_idents: <::crusp_graph::LazyInputEventHandler<#in_nodes, #in_events, ::crusp_graph::OutCostEventLink<#out_events>>>::builder()),*,
                        #(#in_rev_idents: <::crusp_graph::AdjacentListGraph<#out_nodes,#in_rev_nodes>>::builder()),*,
                        #out_ident: <::crusp_graph::HandlerOutput<#out_node, #out_event>>::builder(),
                    }
                }

                pub fn finalize(self) -> #graph_ident {
                    #graph_ident {
                        #(#in_idents2: <::crusp_graph::LazyInputEventHandler<#in_nodes2, #in_events2, ::crusp_graph::OutCostEventLink<#out_events2>>>::new(self.#in_idents3.finalize())),*,
                        #(#in_rev_idents2: ::std::rc::Rc::new(self.#in_rev_idents3.finalize())),*,
                        #out_ident: self.#out_ident.finalize(),
                    }
                }
            }
        )
    };
    let graph_impl = {
        let graph_ident_builder = graph_ident_builder.clone();
        let graph_ident = graph_ident.clone();
        let out_ident = out_ident.clone();
        let out_node = out_node.clone();
        let out_event = out_event.clone();
        let in_idents: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.ident.clone();
                quote!(#field)
            })
            .collect();
        let out_events: Vec<_> = std::iter::repeat(&out_event)
            .clone()
            .take(in_idents.len())
            .collect();
        let in_nodes: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.node.clone();
                quote!(#field)
            })
            .collect();
        let in_events: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.event.clone();
                quote!(#field)
            })
            .collect();

        quote!(
            impl  #graph_ident
            {
                pub fn builder() -> #graph_ident_builder {
                    <#graph_ident_builder>::new()
                }

                #[allow(clippy::type_complexity)]
                #[inline]
                pub fn split_in_out(&mut self) -> (
                        &mut ::crusp_graph::HandlerOutput<#out_node, #out_event>,
                        #(&mut ::crusp_graph::LazyInputEventHandler<
                            #in_nodes, #in_events,
                            ::crusp_graph::OutCostEventLink<#out_events>>
                        ),*
                    )
                {
                    (
                        unsafe{ &mut *((&mut self.#out_ident) as *mut _)},
                        #(unsafe{ &mut *((&mut self.#in_idents) as *mut _)}),*
                    )
                }
            }
        )
    };
    let impl_pop = {
        let out_ident = out_ident.clone();
        let out_node = out_node.clone();
        let out_event = out_event.clone();
        let in_idents: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.ident.clone();
                quote!(#field)
            })
            .collect();
        let in_idents2: Vec<_> = in_idents.clone();

        quote!(
            impl ::crusp_graph::OutputEventHandler<#out_node, #out_event> for #graph_ident
            {
                fn collect_and_pop(&mut self) -> Option<(#out_node, #out_event)> {
                    let (__crusp__outs, #(#in_idents),*) = self.split_in_out();
                    #(#in_idents2.trigger_events(|__crusp__out| __crusp__outs.collect_out_event(__crusp__out)));*;
                    self.#out_ident.pop()
                }
        })
    };
    let impl_visit_all = {
        let graph_ident = graph_ident.clone();
        let out_ident = out_ident.clone();
        let out_node = out_node.clone();
        let out_event = out_event.clone();
        let in_idents: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.ident.clone();
                quote!(#field)
            })
            .collect();
        let in_idents2: Vec<_> = in_idents.clone();
        let in_nodes: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.node.clone();
                quote!(#field)
            })
            .collect();
        let in_rev_idents: Vec<_> = graph.ins.iter()
            .map(|field| {
                let field = field.ident.clone();
                let rev_field = format!("__crusp__rev_{}", field);
                let rev_field = syn::Ident::new(&rev_field, span!());
                quote!(#rev_field)
            })
            .collect();

        quote!(
            impl <Visitor> ::crusp_graph::VisitAllOutputsNode<#out_node, Visitor> for #graph_ident
               where
               #(Visitor: VisitMut<#in_nodes>),*,
            {
                fn visit_all_in_nodes(&self, out_node: &#out_node, visitor: &mut Visitor)
                {
                    #(self.#in_rev_idents.visit_in_nodes(out_node, visitor));*;
                }
            }
        )
    };
    let impl_visitors: Vec<_> = graph.ins.iter()
        .map(|field| {
            let graph_ident_builder = graph_ident_builder.clone();
            let (in_ident, node, event) =
                (field.ident.clone(), field.node.clone(), field.event.clone());
            let rev_ident = format!("__crusp__rev_{}", in_ident);
            let rev_ident = syn::Ident::new(&rev_ident, span!());
            let out_node = out_node.clone();
            let out_event = out_event.clone();
            let out_ident = out_ident.clone();
            let in_node = node.clone();
            let in_event = event.clone();
            quote!(
                impl ::crusp_graph::VisitOutputsNode<#out_node, #in_node>
                    for #graph_ident
                {
                    fn visit_in_nodes<Visitor>(&self, out_node: &#out_node, visitor: &mut Visitor)
                        where Visitor: VisitMut<#in_node>
                    {
                        self.#rev_ident.visit_in_nodes(out_node, visitor);
                    }
                }
            )
    })
    .collect();
    let in_wheres: Vec<_> = graph.ins.iter()
        .map(|field| {
            let (_ident, node, event) =
                (field.ident.clone(), field.node.clone(), field.event.clone());
            quote!(
                #node: ::crusp_graph::GraphNode,
                #event: ::crusp_graph::GraphEvent
            )
        })
        .collect();
    let out_builder_where = out_where.clone();
    let in_builder_wheres = in_wheres.clone();
    let expanded = quote!(
        struct #graph_ident_builder
        {
            #out_builder_field,
            #(#in_builder_fields),*,
            #(#in_rev_builder_fields),*,
        }

        struct #graph_ident
        {
            #out_field,
            #(#in_fields),*,
            #(#in_rev_fields),*
        }

        #(#impl_visitors)*

        #(#impl_visit_all)*

        #(#in_events_handler)*

        #(#inout_events_handler)*

        #graph_builder_impl

        #graph_impl

        #impl_pop
    );
    expanded.into()
}


/*

#[proc_macro_attribute]
pub fn constraint_old(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());

    let  ast = parse_macro_input!(item as DeriveInput);
    //eprintln!("{:#?}", ast);

    let name = &ast.ident;
    let curr_name = format!("{}", name);
    let curr_ident = syn::Ident::new(&curr_name, name.span());
    let var_name = format!("__CRUSP__Vars{}", name);
    let var_ident = syn::Ident::new(&var_name, name.span());

    let lifetime_vars = "__crup__vars";

    let remove_attrs = |field: &syn::Field| {
        syn::Field{
            attrs: Vec::new(),
            vis: field.vis.clone(),
            ident: field.ident.clone(),
            ty: field.ty.clone(),
            colon_token: field.colon_token.clone(),
        }
    };

// None if absent
    let mut new_type_names: Vec<(String, syn::Path)>;
    let vars = if let syn::Data::Struct(
        syn::DataStruct{
            fields: syn::Fields::Named(
                syn::FieldsNamed{ref named, ..}
            ),
            ..
        }
    ) = ast.data  {
        /*new_type_names = named.iter()
            .filter(|field|  field.attrs[0].path.segments[0].ident == "var")
            .map(|field| {
                (format!("__CRUSP__{}", field.ident.as_ref().unwrap().to_string().to_ascii_uppercase()),
                match field.ty {
                    syn::Type::
                    Path(ref p) => p.path.clone(),
                    _ => { unimplemented!() },
                }
                )
            })
            .collect();*/
        // create generics named with bound
        named.iter().filter(|field|  field.attrs[0].path.segments[0].ident == "var")
        .map(|field| {


            let mut types = syn::punctuated::Punctuated::new();
            let t_name = format!("__CRUSP__{}", field.ident.as_ref().unwrap().to_string().to_ascii_uppercase());
            types.push_value(syn::PathSegment {
                ident: syn::Ident::new(
                    &t_name,
                    syn::export::Span::call_site()) ,
                arguments: syn::PathArguments::None,
            });
            syn::Field{
                attrs: Vec::new(),
                vis: field.vis.clone(),
                ident: field.ident.clone(),
                ty: syn::Type::Path(
                    syn::TypePath{
                        qself:None,
                        path: syn::Path {
                            leading_colon: None,
                            segments: types,
                        }
                    }
                ),
                colon_token: field.colon_token.clone(),
        }})
    } else {
        unimplemented!()
    };
//    eprintln!("{:#?}", new_type_names);
    let mut generics = ast.generics.clone();
    //let mut generics_name: HashMap<String, bool> = HashMap::new();
    let phantom_generics_type: Vec<_> = ast.generics.params.iter()
        .filter(|&g| if let syn::GenericParam::Type(_) = g { true } else { false})
        .map(|g| if let syn::GenericParam::Type(ref p) = g { p.ident.clone() } else { unreachable!() })
        .map(|ident| {
            let ident_name = format!("__CRUSP__PHANTOM__{}", &ident).to_ascii_lowercase();
            let mut types = syn::punctuated::Punctuated::new();
            types.push_value(
                    syn::PathSegment{
                        ident: ident.clone(),
                        arguments: syn::PathArguments::None, }
            );
            let ty = syn::Type::Path (
                syn::TypePath {
                    qself:None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: types,
                    }
                }
            );
            let mut ph_segments = syn::punctuated::Punctuated::new();
            ph_segments.push_value(
                    syn::PathSegment{
                        ident: syn::Ident::new("std", span!()),
                        arguments: syn::PathArguments::None, }
            );
            ph_segments.push_punct(Token!(::)(span!()));
            ph_segments.push_value(
                    syn::PathSegment{
                        ident: syn::Ident::new("marker", span!()),
                        arguments: syn::PathArguments::None, }
            );
            ph_segments.push_punct(Token!(::)(span!()));
            let mut ty_as_gen_seg = syn::punctuated::Punctuated::new();
            ty_as_gen_seg.push_value(syn::PathSegment {
                ident: ident.clone(),
                arguments: syn::PathArguments::None,
            });
            let mut ty_as_gen = syn::punctuated::Punctuated::new();
            ty_as_gen.push_value(syn::GenericArgument::Type(
                syn::Type::Path(syn::TypePath{
                    qself: None,
                    path: syn::Path {
                        leading_colon: None,
                        segments: ty_as_gen_seg,
                    }
                })
            ));
            ph_segments.push_value(
                    syn::PathSegment{
                        ident: syn::Ident::new("PhantomData", span!()),
                        arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Token!(<)([span!()]),
                            args: ty_as_gen,
                            gt_token: Token!(>)([span!()]),
                        }),
                    }
            );
            let ph_ty = syn::Type::Path(
                syn::TypePath {
                    qself: None,
                    path: syn::Path {
                        leading_colon: Some(Token!(::)(span!())),
                        segments: ph_segments,
                    }
                }
            );
            syn::Field {
                attrs: Vec::new(),
                vis: syn::Visibility::Public(syn::VisPublic{
                    pub_token: Token!(pub)(span!())
                }),
                ident: Some(syn::Ident::new(&ident_name, span!())),
                colon_token: None,
                ty: ph_ty,
            }
        })
        .collect();

    // collect all generics named
    //  check when one is used in struct field
    // add phantom data for other
    // get all fields for creating new with phantomData...?

    /*for (gen, _) in new_type_names.iter() {
        generics.params.push(
                syn::GenericParam::Type(
                    syn::TypeParam {
                        attrs: Vec::new(),
                        ident: syn::Ident::new(&gen, syn::export::Span::call_site()),
                        colon_token: None,
                        bounds: syn::punctuated::Punctuated::new(),
                        eq_token: None,
                        default: None,
                    }
                )
        );
    }*/
    let mut where_clause = generics.where_clause.clone();
    /*for (gen, bound) in new_type_names.iter() {
        let mut ty = syn::punctuated::Punctuated::new();
        ty.push_value(syn::PathSegment {
            ident: syn::Ident::new(
                &gen,
                syn::export::Span::call_site()) ,
            arguments: syn::PathArguments::None,
        });
        let mut bounds = syn::punctuated::Punctuated::new();
        bounds.push_value(
            syn::TypeParamBound::Trait(syn::TraitBound{
                paren_token: None,
                modifier: syn::TraitBoundModifier::None,
                lifetimes: None,
                path: bound.clone(),
            })
        );
        where_clause.as_mut().unwrap().predicates.push(
                syn::WherePredicate::Type(
                    syn::PredicateType  {
                        lifetimes: None,
                        bounded_ty: syn::Type::Path(
                                syn::TypePath {
                                    qself: None,
                                    path: syn::Path {
                                        leading_colon: None,
                                        segments: ty
                                    },
                                }
                        ),
                        colon_token: syn::token::Colon{
                            spans: [syn::export::Span::call_site()]
                        },
                        bounds: bounds,
                    }
                )
        );
    }*/
    let vals = if let syn::Data::Struct(
        syn::DataStruct{
            fields: syn::Fields::Named(
                syn::FieldsNamed{ref named, ..}
            ),
            ..
        }
    ) = ast.data  {
        named.iter()
            .filter(|field|  field.attrs[0].path.segments[0].ident == "val")
            .map(remove_attrs)
    } else {
        unimplemented!()
    };
    let states = if let syn::Data::Struct(
        syn::DataStruct{
            fields: syn::Fields::Named(
                syn::FieldsNamed{ref named, ..}
            ),
            ..
        }
    ) = ast.data  {
        named.iter().filter(|field|  field.attrs[0].path.segments[0].ident == "state")
        .map(remove_attrs)
    } else {
        unimplemented!()
    };
    // other

    let mut generics_vars = ast.generics.clone();
    let vars_lt = syn::Lifetime {
            apostrophe: syn::export::Span::call_site(),
            ident: syn::Ident::new(&lifetime_vars, syn::export::Span::call_site()),
    };
    let vars_lt_gen = syn::GenericParam::Lifetime(syn::LifetimeDef {
        attrs: Vec::new(),
        lifetime: vars_lt.clone(),
        colon_token: None,
        bounds: syn::punctuated::Punctuated::new(),
    });
    generics_vars.params.insert(0usize, vars_lt_gen.clone());
    let vars = if let syn::Data::Struct(
        syn::DataStruct{
            fields: syn::Fields::Named(
                syn::FieldsNamed{ref named, ..}
            ),
            ..
        }
    ) = ast.data  {
        // create generics named with bound
        named.iter().filter(|field|  field.attrs[0].path.segments[0].ident == "var")
        .map(|field| {
            /*let mut types = syn::punctuated::Punctuated::new();
            let t_name = format!("__CRUSP__{}", field.ident.as_ref().unwrap().to_string().to_ascii_uppercase());
            types.push_value(syn::PathSegment {
                ident: field.ident.as_ref().unwrap().clone(),
                /*syn::Ident::new(
                    field.ident.as_ref().unwrap(),
                    //&t_name,
                    syn::export::Span::call_site()) ,*/
                arguments: syn::PathArguments::None,
            });*/
            syn::Field{
                attrs: Vec::new(),
                vis: field.vis.clone(),
                ident: field.ident.clone(),
                ty: syn::Type::Reference(
                    syn::TypeReference {
                        and_token: syn::token::And {spans: [syn::export::Span::call_site()]},
                        lifetime: Some(vars_lt.clone()),
                        mutability: Some(syn::token::Mut {span: syn::export::Span::call_site()}),
                        elem: Box::new(field.ty.clone()),
                    }
                ),
                colon_token: field.colon_token.clone(),
        }})
    } else {
        unimplemented!()
    };

    let assert_sync = quote_spanned! {name.span() =>
        /*trait V {}
        trait VV : V {}

        struct T {}
        impl V for T{}
        impl VV for T {}
        trait _AssertTRAIT {
            type Associated: V;
        }
        struct FAKE_T<U, /* auto added*/Z>
        where
        //U: VV,
        // auto added to check trait simple:
        Z: _AssertTRAIT<Associated = U>
        {
            _u: ::std::marker::PhantomData<U>,
            _z: ::std::marker::PhantomData<Z>,
        }*/

        fn _ASSERT_LHS_VAR<T,LHS,RHS>(_lhs: LHS) -> ()
            where T: Ord+Eq,
            LHS: BoundedDomain<T, RHS>,
            RHS: BoundedDomain<T, LHS>,
        {
            let vars: Box<Variable> = Box::new(_lhs);
        }


    //    struct __CRUSP__AssertVariable where i32: Variable;
        fn _t() -> () {
            let _x = #curr_ident::new(0i32, 1i32);
        }
    };

let gen_ = phantom_generics_type.clone();
    let expanded = quote!(
        #assert_sync
        struct #curr_ident #generics #where_clause
        {
            #(#vals,)*
            #(#states,)*
            #(#gen_,)*
        }

        struct #var_ident #generics_vars #where_clause
         {
            #(#vars,)*
            #(#phantom_generics_type,)*
        }

        /*impl #name  {
            fn builder() -> #view_ident {
                #view_ident {

                }
            }
        }*/
    );
    expanded.into()
    //    assert_sync.into()
}
*/
