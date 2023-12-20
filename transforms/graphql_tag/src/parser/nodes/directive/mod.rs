// libs
use apollo_parser::cst::{Directive, Directives};
use swc_common::Span;
use swc_ecma_ast::*;

// helpers
use crate::parser::{
    nodes::{arguments::create_arguments, name::create_name},
    utils::get_key_value_node,
};

fn create_directive(directive: Directive, span: Span) -> Option<ExprOrSpread> {
    let kind = get_key_value_node("kind".into(), "Directive".into());
    let name = get_key_value_node(
        "name".into(),
        create_name(directive.name().unwrap().text().as_str().into(), span),
    );

    let mut directive_object = ObjectLit {
        span,
        props: vec![kind, name],
    };

    if directive.arguments().is_some() {
        let arguments_prop = get_key_value_node(
            "arguments".into(),
            create_arguments(directive.arguments(), span),
        );

        directive_object.props.push(arguments_prop)
    }

    Some(ExprOrSpread {
        spread: None,
        expr: Box::new(Expr::Object(directive_object)),
    })
}

pub fn create_directives(directives: Option<Directives>, span: Span) -> Expr {
    if directives.is_none() {
        return Expr::Array(ArrayLit {
            span,
            elems: vec![],
        });
    }

    Expr::Array(ArrayLit {
        span,
        elems: directives
            .unwrap()
            .directives()
            .into_iter()
            .map(|directive| create_directive(directive, span))
            .collect(),
    })
}
