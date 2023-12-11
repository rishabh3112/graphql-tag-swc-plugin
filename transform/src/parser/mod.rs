// built-ins
use std::collections::HashMap;

// libs
use apollo_parser::SyntaxTree;
use swc_common::Span;
use swc_ecma_ast::*;

// modules
mod nodes;
mod utils;

// helpers
use nodes::document::create_document;

pub fn parse_graphql_tag(
    body: String,
    span: Span,
    expressions: Vec<Box<Expr>>,
    expr_def_map: &mut HashMap<String, Expr>,
) -> Result<Expr, SyntaxTree> {
    let parser = apollo_parser::Parser::new(&body);
    let ast = parser.parse();

    if ast.errors().len() != 0 {
        Err(ast)
    } else {
        let doc = ast.document();
        Ok(create_document(doc, span, body, expressions, expr_def_map))
    }
}
