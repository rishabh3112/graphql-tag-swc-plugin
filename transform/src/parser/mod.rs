// built-ins
use std::collections::HashMap;

// libs
use apollo_parser::SyntaxTree;
use swc_common::{comments::Comments, Span};
use swc_ecma_ast::*;

// modules
mod nodes;
pub mod utils;

// helpers
use nodes::document::create_document;

pub fn parse_graphql_tag<C: Comments>(
    body: String,
    span: Span,
    expressions: Vec<Box<Expr>>,
    expr_def_map: &mut HashMap<String, Expr>,
    unique_fn_name: String,
    unique_fn_used: &mut bool,
    comments: &mut C,
) -> Result<Expr, SyntaxTree> {
    let parser = apollo_parser::Parser::new(&body);
    let ast = parser.parse();

    if ast.errors().len() != 0 {
        Err(ast)
    } else {
        let doc = ast.document();
        Ok(create_document(
            doc,
            span,
            body,
            expressions,
            expr_def_map,
            unique_fn_name,
            unique_fn_used,
            comments,
        ))
    }
}
