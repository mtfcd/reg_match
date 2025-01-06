use proc_macro::TokenStream;
use quote::{format_ident, quote};
use regex_syntax::ast::Ast as ReAst;
use core::panic;
use std::collections::HashMap;
use syn::{braced, parse::Parse, parse_macro_input, Arm, Expr, Ident};

fn gen_compile_re(arm_count: usize, pat_lit: &str) -> proc_macro2::TokenStream {
    let static_re_name = format_ident!("RE_{}", arm_count);
    let cap_re_name = format_ident!("cap_{}", arm_count);
    quote! {
        static #static_re_name: std::sync::OnceLock<reg_match::regex::Regex> = std::sync::OnceLock::new();
        let #cap_re_name = #static_re_name.get_or_init(
            || reg_match::regex::Regex::new(#pat_lit).unwrap()
        );
    }
}

fn get_local_names(pat_lit: &str) -> Vec<proc_macro2::TokenStream> {
    let re = parse_re(pat_lit)
        .map_err(|e| panic!("{}", e.to_string()))
        .unwrap();
    let mut re_flat = HashMap::new();
    flatten_re(&mut re_flat, &re);
    let mut names = vec![];
    for k in re_flat.keys() {
        let captured_name = format_ident!("{}", k);
        names.push(quote! {
            let #captured_name = &caps[#k];
        })
    }
    names
}

fn get_pattern_literal(arm: &syn::Arm) -> String {
    let syn::Pat::Lit(lit) = &arm.pat else {
        panic!("not literal")
    };
    let syn::Lit::Str(lit) = &lit.lit else {
        panic!("not str literal")
    };
    lit.value()
}

fn gen_arm_code(
    arm_count: usize,
    names: Vec<proc_macro2::TokenStream>,
    body: &Expr,
    cap_re_name: proc_macro2::Ident,
    expr: &PatExpr,
) -> proc_macro2::TokenStream {
    if arm_count == 0 {
        return quote! {
            if let Some(caps) = #cap_re_name.captures(#expr) {
                #(#names)*
                #body
            }
        };
    }
    quote! {
        else if let Some(caps) = #cap_re_name.captures(#expr) {
            #(#names)*
            #body
        }
    }
}

#[proc_macro]
pub fn reg_match(input: TokenStream) -> TokenStream {
    // Parse the input as a list of match arms
    let reg_match = parse_macro_input!(input as RegMatch);
    let expr = reg_match.expr;

    // Generate code for each arm
    let mut static_re = Vec::new();
    let mut generated_code = Vec::new();

    let mut has_default = false;
    for (arm_count, arm) in reg_match.arms.iter().enumerate() {
        let pattern = &arm.pat; // Pattern in the arm
        let body = &arm.body; // Body of the arm

        if let syn::Pat::Wild(_) = pattern {
            generated_code.push(quote! {
                else  {
                    #body
                }
            });
            has_default = true;
            break;
        }

        let pat_lit = get_pattern_literal(arm);
        static_re.push(gen_compile_re(arm_count, &pat_lit));

        let cap_re_name = format_ident!("cap_{}", arm_count);
        let names = get_local_names(&pat_lit);


        generated_code.push(gen_arm_code(arm_count, names, body, cap_re_name, &expr));
    }
    if !has_default {
        panic!("should always has a default branch.")
    }

    // Combine the generated code
    let expanded = quote! {
        {
            #(#static_re)*
            #(#generated_code)*
        }
    };

    TokenStream::from(expanded)
}

enum PatExpr {
    E(Expr),
    I(Ident),
}

impl quote::ToTokens for PatExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            PatExpr::E(e) => e.to_tokens(tokens),
            PatExpr::I(i) => i.to_tokens(tokens),
        }
    }
}

struct RegMatch {
    pub expr: PatExpr,
    pub arms: Vec<Arm>,
}

impl Parse for RegMatch {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = if input.peek(syn::Ident) {
            PatExpr::I(input.parse()?)
        } else {
            PatExpr::E(input.parse()?)
        };

        let content;
        braced!(content in input);

        let mut arms = vec![];
        while let Ok(a) = content.parse() {
            arms.push(a);
        }
        Ok(RegMatch { expr, arms })
    }
}

fn flatten_re(out: &mut HashMap<String, ()>, re: &ReAst) {
    match re {
        ReAst::Flags(_) => (),
        ReAst::Dot(_) => (),
        ReAst::Assertion(_) => (),
        ReAst::Empty(_) => (),
        ReAst::Literal(_) => (),
        ReAst::ClassUnicode(_) => (),
        ReAst::ClassPerl(_) => (),
        ReAst::ClassBracketed(_) => (),
        ReAst::Repetition(e) => flatten_re(out, &e.ast),
        ReAst::Group(g) => match &g.kind {
            regex_syntax::ast::GroupKind::CaptureIndex(_) => flatten_re(out, &g.ast),
            regex_syntax::ast::GroupKind::CaptureName { name, .. } => {
                out.insert(name.name.clone(), ());
                flatten_re(out, &g.ast)
            }
            regex_syntax::ast::GroupKind::NonCapturing(_) => flatten_re(out, &g.ast),
        },
        ReAst::Concat(c) => {
            for c in &c.asts {
                flatten_re(out, c);
            }
        }
        ReAst::Alternation(a) => {
            for child in &a.asts {
                flatten_re(out, child);
            }
        }
    }
}

fn parse_re(regex_raw: &str) -> Result<ReAst, Box<regex_syntax::ast::Error>> {
    regex_syntax::ast::parse::Parser::new()
        .parse(regex_raw)
        .map_err(Box::new)
}
