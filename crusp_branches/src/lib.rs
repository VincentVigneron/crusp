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

// TODOivincent):
// - values and states should not depend on variable types because proc macro constraint_old
//   is here to generate the glue between the variables and the solver and keep the Constraint
//   implementor free from all that stuff, which simplifiy unit testing on constraint_old
//   No need to worry about the solver. If value and state relies on variable types
//   then it is not possible. Furthermore, most domain traits that works on 2 variables  are
//   symmetrics so it is possible
//   to make varibale relies on values and states and not the otherway.
// - Add varr array type for arrays or list of variables

// Used to keep tarck of the real span for error reporting
#[derive(Debug, Clone)]
struct ConstraintData {
    ident: syn::Ident,
    vars: Vec<ConstraintVar>,
    vals: Vec<ConstraintVal>,
    states: Vec<ConstraintState>,
    generics: Vec<ConstraintGeneric>,
}

#[derive(Debug, Clone)]
struct ConstraintGenericParam {
    ident: syn::Ident,
}

#[derive(Debug, Clone)]
struct ConstraintLifetime {
    ident: syn::Ident,
}

#[derive(Debug, Clone)]
struct ConstraintConst {
    ident: syn::Ident,
}


#[derive(Debug, Clone)]
struct ConstraintGeneric {
    generics: Vec<syn::TypeParam>,
    const_params: Vec<syn::ConstParam>,
    lifetimes: Vec<syn::LifetimeDef>,
    where_clause: Option<syn::WhereClause>,
    where_predicates: Vec<syn::WherePredicate>,
}

impl ConstraintGeneric {
/*    fn new(generics: Vec<syn::TypeParam>,
        const_params: Vec<syn::ConstParam>,
        lifetimes: Vec<syn::LifetimeDef>,
        where_clause: Option<syn::WhereClause>,
        where_predicates: Vec<syn::WherePredicate>) -> Self
    {

// remove where_clause can be created with where predicates
        // create dependency graph of generics here
        //    => it will help keep required predicate when filtering
    }
*/
    fn to_where_clause(&self) -> Option<syn::WhereClause> {
        None
    }

    fn where_clause_filter<Pred>(&self, mut pred: Pred) -> Option<syn::WhereClause>
    where
        Pred: FnMut(&syn::WherePredicate) -> bool
    {
        // return none if non...
        let mut predicates = syn::punctuated::Punctuated::new();
        let to_keep: Vec<_> = self.where_predicates.iter().filter(|&w| pred(w)).collect();
        for &pred in to_keep.iter() {
            predicates.push_value(pred.clone());
            predicates.push_punct(Token!(,)(span!()));
        }
        Some(syn::WhereClause {
            where_token: Token!(where)(span!()),
            predicates: predicates,
        })
    }
}

#[derive(Debug, Clone)]
struct ConstraintVar {
    ident: syn::Ident,
    ty: syn::TypePath,

    //ToTokens::::::::::::::;
}

#[derive(Debug, Clone)]
struct ConstraintVal {
    ident: syn::Ident,
    ty: syn::Type,
}

#[derive(Debug, Clone)]
struct ConstraintState {
    ident: syn::Ident,
    ty: syn::Type,
}

macro_rules! fields_with_attr {
    ($fields: expr, $name: expr) => ({
        $fields.named.iter()
            .filter(|field| field.attrs.len() == 1usize)
            .filter(|field| !field.attrs[0].path.segments.is_empty())
            .filter(|field| field.attrs[0].path.segments[0].ident == $name)
    })
}


macro_rules! abs_segment_path_of {
    (@IMPL) => ({
        syn::punctuated::Punctuated::new()
    });
    ($p:expr) => ({
        let mut ph_segments = abs_segment_path_of!(@IMPL);
        ph_segments.push_value(
            syn::PathSegment{
                ident: syn::Ident::new($p, span!()),
                arguments: syn::PathArguments::None, }
            );
        ph_segments.push_punct(Token!(::)(span!()));
        ph_segments
    });
    ($($ps:expr),+) => ({
        abs_segment_path_of!(@REV [] [$($ps),+])
    });
    (@REV [$($revs:expr),*] [$p:expr$(,$ps:expr)*]) => ({
        abs_segment_path_of!(@REV [$p $(,$revs)*] [$($ps),*])
    });
    (@REV [$p:expr,$($ps:expr),*] []) => ({
        abs_segment_path_of!(@IMPL $p $(,$ps)*)
    });
    (@IMPL $p:expr $(,$ps:expr)*) => ({
        let mut ph_segments = abs_segment_path_of!(@IMPL $($ps),*);
        ph_segments.push_value(
                syn::PathSegment{
                    ident: syn::Ident::new($p, span!()),
                    arguments: syn::PathArguments::None, }
        );
        ph_segments.push_punct(Token!(::)(span!()));
        ph_segments
    });
}

fn generate_phantomdata_type(ty: syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma>) -> syn::Type {
    let mut ph_segments = abs_segment_path_of!("std", "marker");
    ph_segments.push_value(
            syn::PathSegment{
                ident: syn::Ident::new("PhantomData", span!()),
                arguments: syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Token!(<)([span!()]),
                    args: ty,
                    gt_token: Token!(>)([span!()]),
                }),
            }
    );
    syn::Type::Path(
        syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: Some(Token!(::)(span!())),
                segments: ph_segments,
            }
        }
    )
}

fn ident_to_generic_argument_segments(ident: &syn::Ident)
    -> syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma>
{
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
    ty_as_gen
}

fn phantom_data_value() -> syn::Path {
    let mut ph_segments = abs_segment_path_of!("std", "marker");
    let mut ph_segments = abs_segment_path_of!("std", "marker");
    ph_segments.push_value(
        syn::PathSegment{
            ident: syn::Ident::new("PhantomData", span!()),
            arguments: syn::PathArguments::None,
        }
    );
     syn::Path {
        leading_colon: Some(Token!(::)(span!())),
        segments: ph_segments,
    }
}

// maybe return expression foir initialization too??
fn phantomise_generic(type_param: &syn::TypeParam) -> (syn::Field,syn::FieldValue) {
    let ident_name = format!("__CRUSP__PHANTOM__{}", &type_param.ident).to_ascii_lowercase();


    let ty = ident_to_generic_argument_segments(&type_param.ident);
    let ty = generate_phantomdata_type(ty);

    // used for struct initialization
    // create __crup__phantom__$varname: ::std::marker::PhantomData
    let field_value = syn::FieldValue {
            attrs: Vec::new(),
            member: syn::Member::Named(syn::Ident::new(&ident_name, span!())),
            colon_token: Some(Token!(:)(span!())),
            expr: syn::Expr::Path(syn::ExprPath{
                attrs:Vec::new(),
                qself:None,
                path:phantom_data_value(),
            }),
    };
    // create field: pub __crup__phantom__$varname: ::std::marker::PhantomData<$vartype>
    let field = syn::Field {
        attrs: Vec::new(),
        vis: syn::Visibility::Public(syn::VisPublic{
            pub_token: Token!(pub)(span!())
        }),
        ident: Some(syn::Ident::new(&ident_name, span!())),
        colon_token: None,
        ty: ty,
    };
    (field, field_value)
}

fn read_generics(generics: &syn::Generics) -> ConstraintGeneric {
    let mut generics_p = Vec::new();
    let mut const_params = Vec::new();
    let mut lifetimes = Vec::new();
    let mut where_predicates = Vec::new();
    let where_clause = generics.where_clause.clone();
    for param in generics.params.iter() {
        match param {
            syn::GenericParam::Type(ref ty) => {
                generics_p.push(ty.clone());
            },
            syn::GenericParam::Lifetime(ref lifetime) => {
                lifetimes.push(lifetime.clone());
            },
            syn::GenericParam::Const(ref const_param) => {
                const_params.push(const_param.clone());
            },
        }
    }
    if let Some(ref where_clause) = where_clause {
        for pred in where_clause.predicates.iter() {
            where_predicates.push(pred.clone());
        }
    }
    ConstraintGeneric {
        generics: generics_p,
        const_params: const_params,
        lifetimes: lifetimes,
        where_clause: where_clause,
        where_predicates: where_predicates,
    }
}

fn read_var_fields(data: &syn::DataStruct) -> Vec<ConstraintVar> {
    let fields = match data.fields {
        syn::Fields::Named(ref fields) => fields,
        _ => unimplemented!(),
    };
    fields_with_attr!(fields, "var")
        .map(|field| {
            let ident = field.ident.as_ref().expect("variable name").clone();
            let ty = match  field.ty {
                syn::Type::Path(ref path) => path.clone(),
                _ => unimplemented!(),
            };
            ConstraintVar {
                ident: ident,
                ty: ty
            }
        })
        .collect()
}

fn assert_vars(vars: &[ConstraintVar], ast: &DeriveInput) -> Vec<Box<dyn quote::ToTokens>> {
    let mut generics = ast.generics.clone();
    let mut where_clause = generics.where_clause.clone();
    let mut asserts = Vec::new();
    for var in vars {
        let var_ident = &var.ident;
        let var_ty = &var.ty;
        let var_name = format!("__CRUSP__ASSERT_TRAIT_VARIABLE_{}", var_ident);
        let var_fn = syn::Ident::new(&var_name, span!());
        let assert_sync = quote_spanned! {var_ty.path.segments[0].ident.span() =>
        //    struct __CRUSP__AssertVariable where i32: Variable;
            fn #var_fn #generics (#var_ident: #var_ty) -> ()
                #where_clause
            {
                let _x: Box<dyn Variable> = Box::new(#var_ident);
            }
        };
        let assert_sync: Box<dyn quote::ToTokens> = Box::new(assert_sync);
        asserts.push(assert_sync);
    }
    asserts
}

// TODO(vincent): check if item is a DataStruct
#[proc_macro_attribute]
pub fn constraint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let  ast = parse_macro_input!(item as DeriveInput);
    eprintln!("{:#?}", ast);
    let data = if let syn::Data::Struct(ref data) =  ast.data {
        data
    } else {
        unimplemented!()
    };
    let generics = &ast.generics;
    let ident = ast.ident.clone();
    let _attributes = &ast.attrs;
    let visibility = &ast.vis;

    let vars = read_var_fields(data);
    // read var array fields
    // read value fields
    // read state fields
    let generics = read_generics(&ast.generics);
    let phantoms: Vec<_> = generics.generics.iter()
        .map(phantomise_generic)
        .map(|(field,_)| field)
        .collect();
    let phantom_values: Vec<_> = generics.generics.iter()
        .map(phantomise_generic)
        .map(|(_,field_value)| field_value)
        .collect();

    //eprintln!("{:#?}", phantoms);

    let asserts = assert_vars(&vars[..], &ast);

    let mut glob_generics = ast.generics.clone();
    let mut glob_where_clause = generics.where_clause.clone();

    let expanded = quote!(
        #(#asserts)*

        struct #ident #glob_generics #glob_where_clause {
            #(#phantoms,)*
        }

        impl #glob_generics #ident #glob_generics #glob_where_clause {
            pub fn new() -> Self {
                Self  {
                    #(#phantom_values,)*
                }
            }
        }
    );
    expanded.into()
}

#[proc_macro_attribute]
pub fn test_fn(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::ItemFn = syn::parse(item).unwrap();
    eprintln!("{:#?}", ast);
    let e = quote!();

    e.into()
}


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
