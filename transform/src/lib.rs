use std::collections::HashMap;

use swc_ecma_ast::*;
use swc_ecma_visit::{VisitMut, VisitMutWith};

use regex::Regex;

mod parser;

// Visitor

pub struct TransformVisitor {
    expr_def_map: HashMap<String, Expr>,
}

impl TransformVisitor {
    pub fn new() -> Self {
        Self {
            expr_def_map: HashMap::new(),
        }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, node: &mut Expr) {
        if let Some(tag_tpl) = node.as_mut_tagged_tpl() {
            if let Some(tag) = tag_tpl.tag.as_mut_ident() {
                if &tag.sym != "gql" {
                    return;
                }
                if tag_tpl.tpl.quasis.len() == 0 {
                    return;
                }

                let template = &mut tag_tpl.tpl;
                let mut data: String = "".into();

                for quasi in &mut template.quasis {
                    data += &quasi.raw;
                }

                let gql_raw_string = data.to_string();
                let no_gql_line_regex = Regex::new(r#"(^\$\{.*\}$)"#).unwrap();

                let gql_text = gql_raw_string
                    .lines()
                    .filter(|line| !no_gql_line_regex.is_match(line.trim()))
                    .map(|line| String::from(line) + "\n")
                    .collect();

                let expressions = template.exprs.clone();
                let gql_swc_ast_result = parser::parse_graphql_tag(
                    gql_text,
                    tag_tpl.span,
                    expressions,
                    &mut self.expr_def_map,
                );

                match gql_swc_ast_result {
                    Ok(swc_ast) => *node = swc_ast,
                    Err(gql_ast) => {
                        for error in gql_ast.errors() {
                            println!(
                                "GraphQL Error: At index {}, {} got \"{}\" instead",
                                error.index(),
                                error.message(),
                                error.data()
                            )
                        }
                    }
                }
            }
        } else {
            node.visit_mut_children_with(self)
        }
    }
}
